<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input placeholder="请输入搜索内容" size="small" @keyup.enter.native="doUserQuery" v-model="search.keyword" class="input-with-select" clearable>
                    <el-select v-model="search.type" slot="prepend" placeholder="用户名">
                        <el-option label="用户名" value="1"></el-option>
                        <el-option label="手机号" value="2"></el-option>
                        <el-option label="用户ID" value="3"></el-option>
                        <el-option label="IP" value="4"></el-option>
                    </el-select>
                </el-input>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.source" size="small" slot="prepend" placeholder="用户来源" clearable @change="doUserQuery">
                    <el-option v-for="(value,key) in sourceArr" :key="key" :label="value" :value="key"></el-option>
                </el-select>
            </div>
            <!--收起部分-->
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search.utype" size="small" slot="prepend" placeholder="身份类型" clearable @change="doUserQuery">
                    <el-option v-for="(item,key) in userType" :key="key" :label="item.label" :value="item.value"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search.status" size="small" slot="prepend" placeholder="用户状态" clearable @change="doUserQuery">
                    <el-option label="正常" value="1"></el-option>
                    <el-option label="锁定" value="2"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall" :class="{ 'searchbutnOnff': seachbutn }">
                <el-select v-model="search.time_type" size="small" slot="prepend" placeholder="筛选日期" clearable @change="handleTimeChange">
                    <el-option label="注册时间" value="adtime"></el-option>
                    <el-option label="登录时间" value="lotime"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo">
                <el-date-picker v-model="search.times" type="daterange" align="right" unlink-panels range-separator="至" start-placeholder="开始日期" end-placeholder="结束日期" :picker-options="timeOptions" value-format="yyyy-MM-dd" size="small" @change="handleTimeChange"></el-date-picker>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="doUserQuery">查询</el-button>
            </div>
            <div class="tableSeachInpt tableSeachzk" :class="{ 'searchbutnKai': seachbutn }">
                <el-button type="info" class="zhankai" @click="seachbutn = !seachbutn, tableHig = !tableHig" aria-disabled="false" size="mini" plain>展开<i class="el-icon-arrow-down el-icon--right"></i></el-button>
                <el-button type="info" class="shouqi" @click="seachbutn = !seachbutn, tableHig = !tableHig" aria-disabled="false" size="mini" plain>合并<i class="el-icon-arrow-up el-icon--right"></i></el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> 数据统计：共 {{memNum.memAllNum}} 条
            <span class="admin_datatip_n cp_n" @click="lockList">已锁定：{{memNum.memStatusNum3}} 条</span>
            <span class="admin_datatip_n">搜索结果： {{total}} 条</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTabAllyue': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @selection-change="selectChange" ref="multipleTable" @sort-change="shortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="uid" label="用户ID" width="90" sortable="custom"> </el-table-column>
                <el-table-column label="名称/用户名" min-width="180" show-overflow-tooltip>
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <div>{{props.row.countname }}</div>
                            <el-link @click="getMemberUrl(props.row.uid,props.row.usertype)" target="_blank" type="primary">{{props.row.username }}</el-link>
                            <el-tooltip v-if="props.row.status == 2" class="item" effect="dark" content="已锁定" placement="top-start">
                                <i class="el-icon-lock" style="color: orange"></i>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="usersf" label="当前身份" min-width="120">
                    <template slot-scope="props">
                        <div class="user_sf">
                            <span class="user_sf1" v-if="props.row.usertype == 2">企业用户</span>
                            <span class="user_sf2" v-if="props.row.usertype == 1">个人用户</span>
                            <span class="user_sf_no" v-if="props.row.usertype == 5">暂无身份</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="手机号/归属地" min-width="130">
                    <template slot-scope="props">
                        <div class="moduleProps">
                            <span>{{ props.row.moblie }}</span>
                            <template v-if="props.row.moblie_address">
                                <span class="gsd"> {{ props.row.moblie_address }}</span>
                            </template>
                            <template v-else>
                                <el-link type="primary" @click="getmobileaddress(props.row)">查询归属地</el-link>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="login_date" label="注册/登录" min-width="170" sortable="custom">
                    <template slot-scope="props">
                        <div class=""> <span class="gsd">{{ props.row.reg_date_n }}</span></div>
                        <div class=""> <span>{{ props.row.login_date > 0 ? props.row.login_date_n : '未登录' }}</span></div>
                    </template>
                </el-table-column>
                <el-table-column prop="userly" label="来源" min-width="180">
                    <template slot-scope="props">
                        {{sourceArr[props.row.source]}}
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="IP/归属地" min-width="180">
                    <template slot-scope="props">
                        <div class="moduleProps" v-if="props.row.login_ip">
                            <span>{{ props.row.login_ip }}</span>
                            <template v-if="props.row.login_address">
                                <span class="gsd"> {{ props.row.login_address }}</span>
                            </template>
                            <template v-else>
                                <el-link type="primary" @click="getipaddress(props.row)">查询归属地</el-link>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="zt" label="状态" width="60" fixed="right">
                    <template slot-scope="props">
                        <div class="admin_state">
                            <span v-if="props.row.status == 1">正常</span>
                            <el-tooltip class="item" effect="dark" :content="props.row.lock_info" placement="right" v-if="props.row.status == 2">
                                <span class="admin_state3">锁定</span>
                            </el-tooltip>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="detailFun(scope.row)">修改</el-button>
                            <el-popover placement="bottom" width="90" trigger="hover">
                                <div class="moduleMores">
                                    <el-button type="text" @click="lockUser(scope.row)">锁定会员</el-button>
                                    <el-button type="text" @click="resetPassword(scope.row)">重置密码</el-button>
                                    <el-button type="text" @click="shareZhan(scope.row)">分配站点</el-button>
                                    <el-button type="text" @click="del(scope.row)">删除会员</el-button>
                                </div>
                                <el-button size="small" plain slot="reference" @click="visible = !visible">更多</el-button>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="batchDel" size="mini">批量删除</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination :total="total" @current-change="userPageChange" :page-size="pageSize" :page-sizes="pageSizes" @size-change="handleSizeChange" :current-page.sync="page" layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
        <!--修改用户弹窗-->
        <div class="modluDrawer">
            <el-drawer title=" 用户信息" :visible.sync="userdrawer" :append-to-body="true" size="40%">
                <div class="drawerModInfo drawerModInfoOne" style="height: calc(100% - 80px); overflow-y: auto;">
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>当前身份</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-select v-model="detail.usertype" size="small" slot="prepend" placeholder="身份类型" :disabled="true">
                                <el-option v-for="(item,key) in userType" :key="key" :label="item.label" :value="item.value"></el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>用户名</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input placeholder="用户名" v-model="detail.username"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>登录密码</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input type="password" placeholder="登录密码" v-model="edit_password"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>手机号码</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input placeholder="输入手机号码" v-model="detail.moblie"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>E-mail</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input placeholder="输入邮箱" v-model="detail.email"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>注册IP</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input placeholder="ip地址" v-model="detail.login_ip"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>使用范围</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-select v-model="search.did" size="small" slot="prepend" placeholder="使用范围" filterable>
                                <el-option v-for="(value,key) in dnameArr" :key="key" :label="value" :value="key"></el-option>
                            </el-select>
                        </div>
                    </div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>状态</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-radio-group v-model="detail.status">
                                <el-radio label="1">正常</el-radio>
                                <el-radio label="2">锁定</el-radio>
                            </el-radio-group>
                        </div>
                    </div>
                </div>
                <div class="setBasicButn" style="border: none;">
                    <el-button @click="userdrawer = false">取 消</el-button>
                    <el-button type="primary" @click="memberSave" :disabled="saveLoading">修 改</el-button>
                </div>
            </el-drawer>
        </div>
        <!--锁定用户弹窗-->
        <div class="modluDrawer">
            <el-dialog title="锁定用户" :visible.sync="usersddrawer" :append-to-body="true" width="450px">
                <div>
                    <div class="wxsettip_small ">用户状态</div>
                    <template>
                        <el-radio-group v-model="lockUserArr.status">
                            <el-radio label="1">正常</el-radio>
                            <el-radio label="2">锁定</el-radio>
                        </el-radio-group>
                    </template>
                    <div class="wxsettip_small ">锁定说明</div>
                    <el-input placeholder="请输入内容" type="textarea" :rows="2" v-model="lockUserArr.lock_info"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="usersddrawer = false">取 消</el-button>
                    <el-button type="primary" @click="lockUserSave" :disabled="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--分配站点弹窗-->
        <div class="modluDrawer">
            <el-dialog title="分配站点" :visible.sync="usercitydrawer" :append-to-body="true" width="450px">
                <div class="wxsettip_small ">当前用户</div>
                <el-input placeholder="企业用户" v-model="shareZhanArr.username" :disabled="true"></el-input>
                <div>
                    <div class="wxsettip_small ">分配站点</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="shareZhanArr.did" size="small" slot="prepend" placeholder="使用范围" filterable>
                            <el-option v-for="(value,key) in dnameArr" :key="key" :label="value" :value="key"></el-option>
                        </el-select>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="usercitydrawer = false">取 消</el-button>
                    <el-button type="primary" @click="shareSave" :disabled="saveLoading">确 定</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
<script>
module.exports = {
    data: function() {
        return {
            loading: false,
            dataText: '数据加载中',
            checkedAll: false,
            visible: true,
            search: {
                utype: "",
                status: '',
                time_type: 'lotime',
                times: [],
                source: '',
                keyword: '',
                type: '1'
            },
            timeOptions: {
                shortcuts: [{
                    text: '昨天',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        start.setTime(start.getTime() - 3600 * 1000 * 24);
                        end.setTime(end.getTime() - 3600 * 1000 * 24);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '今天',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '本周',
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() - 1) * 24 * 60 * 60 * 1000);
                        const end = new Date();
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '上周',
                    onClick(picker) {
                        const start = new Date(new Date().setHours(0, 0, 0) - (new Date().getDay() + 6) * 24 * 60 * 60 * 1000);
                        const end = new Date(new Date().setHours(0, 0, 0) + (0 - new Date().getDay()) *24 * 60 * 60 *1000);
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '本月',
                    onClick(picker) {
                        const end = new Date();
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }, {
                    text: '上月',
                    onClick(picker) {
                        const end = new Date(new Date(new Date().getFullYear(), new Date().getMonth(), 0).setHours(23, 59, 59, 59));
                        const start = new Date(new Date(new Date().getFullYear(), new Date().getMonth() - 1, 1).setHours(0, 0, 0));
                        picker.$emit('pick', [start, end]);
                    }
                }]
            },
            isSearchTime: false,
            select: '',
            usersddrawer: false,
            seachbutn: false,
            userdrawer: false,
            usercitydrawer: false,
            tableHig: true,
            tableData: [],
            items: [
                { type: '', label: '正常' },
            ],
            userType: [{
                value: 1,
                label: "个人身份"
            }, {
                value: 2,
                label: "企业身份"
            }, {
                value: 5,
                label: "无身份类型"
            }],
            sourceArr: [],
            dnameArr: {},
            //
            uri: "m=user&c=",
            total: 0,
            page: 1,
            idsArr: [],
            pageSize: 0,
            pageSizes: [],
            detail: {},
            edit_password: '',
            memNum: {},
            lockUserArr: {
                status: '',
                lock_info: ''
            }, // 锁定
            shareZhanArr: {
                did: ''
            },

            weburl: '',

            saveLoading: false,

            prevPage: 0
        }
    },
    created() {
        var that = this;
        let params = window.parent.homeapp.$route.params;
        let query = window.parent.homeapp.$route.query;
        
        if (!$.isEmptyObject(query.params)) {
            params = {...params,...query.params};
        }
        
        if (!$.isEmptyObject(params)) {
            delete params.activeName;
            this.getParams(params);
        }
        this.getList();
        this.shenheNumber();
    },
    mounted() {
        var that = this
        setTimeout(function() {
            that.getCacheFun();
        }, 200)
    },
    methods: {
        getCacheFun() {
            let _this = this;
            let url = _this.uri + 'admin_member&a=getCache';

            httpPost(url, {}, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.sourceArr = res.data.source;
                    _this.dnameArr = res.data.dname;
                }
            })
        },
        shortChange(e) {
            let orderMap = { ascending: 'asc', descending: 'desc' }
            this.search.t = e.order ? e.prop : null;
            this.search.order = orderMap[e.order];
            this.getList();
        },
        getParams: function(params = {}, search = false) {
            var that = this;
            for (let i in params) {
                if (typeof that.search[i] != 'undefined') {
                    that.search[i] = params[i];
                }
            }
            if (search) {
                this.getList();
            }
        },
        async getMemberUrl(uid, utype) {
            let response = await httpPost('m=user&c=admin_member&a=Imitate', { uid: uid, utype: utype });

            let res = response.data;
            if (res.error === 0) {
                window.open(res.data.url);
            } else {
                message.error(res.msg);
            }
        },
        lockList: function() {
            this.search.status = '2';
            this.page = 1
            this.getList()
        },
        selectChange: function(val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.uid);
                });
            }
            if (_this.idsArr.length == 0) {
                _this.checkedAll = false;
            } else {
                if (_this.idsArr.length == _this.tableData.length) {
                    _this.checkedAll = true;
                } else {
                    _this.checkedAll = false;
                }
            }
        },
        doUserQuery() {
            this.page = 1
            this.getList()
        },
        userPageChange(val) {
            this.page = val
            this.getList()
        },
        userPageSizeChange(val) {
            this.pageSize = val
            this.getList()
        },
        handleTimeChange() {
            if (this.search.time_type != '' && Array.isArray(this.search.times) && this.search.times.length) {

                this.isSearchTime = true;
                this.doUserQuery();
            }
            if (this.isSearchTime && this.search.time_type == '' && this.search.times == null){

                this.isSearchTime = false;
                this.doUserQuery();
            }
        },
        getList: function() {
            let _this = this;
            let url = _this.uri + 'admin_member&a=index';

            _this.search.page = this.page;
            _this.search.pageSize = this.pageSize;
            _this.loading = true;
            httpPost(url, _this.search, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data.data;
                    _this.total = res.data.total;
                    _this.pageSizes = res.data.pageSizes;
                    _this.loading = false;
                    if (_this.prevPage != _this.page) {
                        _this.prevPage = _this.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0) {
                        _this.dataText = "暂无数据";
                    }
                }
            })
        },
        handleSizeChange(val) {
            this.pageSize = val;
            this.getList();
        },
        handleCurrentChange(val) {
            this.page = val;
            this.getList();
        },
        shenheNumber: function() {
            let _this = this;
            let url = this.uri + 'admin_member&a=memNum'
            httpPost(url, {}, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.memNum = res.data;
                } else {
                    message.error(res.msg);
                }
            })
        },
        detailFun: function(row) {
            this.detail = deepClone(row);

            this.detail.usertype = parseInt(this.detail.usertype);

            this.userdrawer = true;
        },
        memberSave: function() {
            let _this = this;
            let url = this.uri + 'admin_member&a=editSave';
            this.detail.password = this.edit_password;
            _this.saveLoading = true;
            httpPost(url, _this.detail).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function() {
                        _this.getList();
                    })
                    _this.userdrawer = false;
                } else {
                    message.error(res.msg)
                }
            }).finally(function() {
                setTimeout(function() {
                    _this.saveLoading = false;
                }, 2000);
            });
        },
        // 重置密码
        resetPassword: function(params) {
            let username = params.username;
            let _this = this;
            let url = this.uri + 'admin_member&a=reset_pw';
            let msg = '确定要重置密码吗?';
            delConfirm(_this, params, function(params) {
                httpPost(url, { uid: params.uid }).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success("用户：" + username + " 密码已经重置为123456！", function() {
                            _this.getList();
                        });
                    }
                })
            }, msg)
        },
        lockUser: function(detail) {
            this.lockUserArr.status = detail.status;
            this.lockUserArr.uid = detail.uid;
            this.lockUserArr.lock_info = detail.lock_info;
            this.usersddrawer = true
        },
        lockUserSave: function() {
            let _this = this;
            let url = this.uri + 'admin_member&a=lock';
            _this.saveLoading = true;
            httpPost(url, _this.lockUserArr).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function() {
                        _this.getList();
                    })
                } else {
                    message.error(res.msg);
                }
            }).finally(function() {
                setTimeout(function() {
                    _this.saveLoading = false;
                }, 2000);
            });
            this.usersddrawer = false;
        },
        shareZhan: function(detail) {
            this.shareZhanArr.uid = detail.uid;
            this.shareZhanArr.username = detail.username
            this.shareZhanArr.did = '' + detail.did;
            this.usercitydrawer = true
        },
        shareSave: function() {
            let _this = this;
            let url = this.uri + 'admin_member&a=checksitedid';
            _this.saveLoading = true;
            httpPost(url, _this.shareZhanArr).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function() {
                        _this.getList();
                    })
                } else {
                    message.error(res.msg);
                }
            }).finally(function() {
                setTimeout(function() {
                    _this.saveLoading = false;
                }, 2000);
            });
            this.usercitydrawer = false;
        },
        del: function(detail) {
            let _this = this;
            let url = this.uri + 'admin_member&a=del';
            let msg = '确定要删除?';
            delConfirm(_this, detail, function(params) {
                httpPost(url, { del: detail.uid }).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function() {
                            _this.getList();
                        });
                    }
                })
            }, msg);
        },
        batchDel: function() {
            let ids = this.idsArr;
            if (!ids.length) {
                message.error('请选择要删除的数据');
                return
            }
            let _this = this,
                params = {};
            params.del = ids;
            let url = this.uri + 'admin_member&a=del';
            let msg = '确定要删除?';
            delConfirm(_this, params, function(params) {
                httpPost(url, params).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function() {
                            _this.getList();
                        });
                    }
                })
            }, msg);
        },
        getipaddress: function(detail) {
            let ip = detail.login_ip;
            if (!ip) {
                message.error('IP地址为空!');
                return
            }
            let url = this.uri + 'admin_member&a=getIpAddress';
            let _this = this;
            httpPost(url, { uid: detail.uid, ip: ip }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function() {
                        _this.getList();
                    });
                } else {
                    message.error(res.msg);
                }
            })
        },
        getmobileaddress: function(detail) {
            let moblie = detail.moblie;
            if (!moblie) {
                message.error('电话不能为空!');
                return
            }
            let url = this.uri + 'admin_member&a=getMobileAddress';
            let _this = this;
            httpPost(url, { uid: detail.uid, moblie: moblie }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function() {
                        _this.getList();
                    });
                } else {
                    message.error(res.msg);
                }
            })
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
    },
};
</script>
<style>
.drawerModTite {
    width: 110px !important;
}

.drawerModInfoOne .drawerModInpt {
    width: calc(100% - 120px);
    display: flex;
    align-items: center;
}
.moduleElTabAllyue{
	height: calc(100% - 136px) !important;
}
@media (max-width: 1440px) {
	.moduleElTabAllyue {
	    height: calc(100% - 176px) !important;
	}
}
</style>
<style scoped>

.el-dialog__body {
    padding: 0px 20px;
}

.tableSeachInptsmall .el-input {
    width: initial;
}

.tableSeachInptsmall .el-select {
    margin-right: 0px !important;
}

.el-input-group__prepend {
    background-color: #ffffff;
    padding: 0 0 0 20px;
}

.moduleElHight .moduleElTable {
    height: calc(100% - 136px);
}

</style>