<template>
    <div class="moduleElHight">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
                <el-input placeholder="请输入搜索内容" size="small" style="margin-right: 8px;" v-model="keyword" clearable class="input-with-select">
                    <el-select v-model="type" slot="prepend" placeholder="请选择">
                        <el-option label="招聘会名称" value="1"></el-option>
                        <el-option label="企业名称" value="2"></el-option>
                    </el-select>
                </el-input>
                <el-select v-model="status" size="small" slot="prepend" style="margin-right: 8px;" placeholder="审核状态" clearable @change="search">
                    <el-option label="已通过" value="1"></el-option>
                    <el-option label="未审核" value="3"></el-option>
                    <el-option label="未通过" value="2"></el-option>
                </el-select>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
            </div>
        </div>
        <div class="admin_datatip"><i class="el-icon-document"></i> 可以实现区域、展位等进行自主设置，企业可在线付费报名参加招聘会等操作
        </div>
        <div class="moduleElTable moduleElMoreLive" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @selection-change="handleSelectionChange" ref="multipleTable" :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='sortChange' v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="编号" sortable="custom"></el-table-column>
                <el-table-column prop="zphname" label="招聘会名称" min-width="180"></el-table-column>
                <el-table-column prop="comname" label="参与企业" min-width="180" show-overflow-tooltip></el-table-column>
                <el-table-column prop="jobname" label="职位" min-width="180" show-overflow-tooltip></el-table-column>
                <el-table-column prop="space_n" label="位置" sortable="custom"></el-table-column>
                <el-table-column prop="sort" label="排序" sortable="custom">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index" v-model="scope.row.sort" @blur="alterData(scope, 1)"></el-input>
                        <span v-else>
                            {{ scope.row.sort }}
                            <img src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14" @click="editData(scope, 1)">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="zt" label="状态">
                    <template slot-scope="props">
                        <div class="admin_state">
                            <span v-if="props.row.status == '1'" class="admin_state1"> 正常</span>
                            <span v-else-if="props.row.status == '0'" class="admin_state2"> 未审核</span>
                            <span v-else-if="props.row.status == '2'" class="admin_state2"> 未通过</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" fixed="right" width="150" align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="cominfo(scope.row)">详情</el-button>
                            <el-button type="danger" size="mini" @click="delrow(scope.row.id)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="delAllBottom" size="mini">批量删除</el-button>
                <el-button @click="multiAudit" size="mini">批量审核</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage" layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--参会企业详情-->
        <el-drawer title="参会企业详情" v-if="dtlislook" :visible.sync="comdrawersh" :modal-append-to-body="false" append-to-body size="80%">
            <div class="shbox">
                <div class="shinfo">
                    <div class="shcomname">{{info.name}}
                        <el-tag type="danger" size="mini">{{info.rating_name}}</el-tag>
                    </div>
                    <div class="sh_zwsz_add">{{curr_comdata.zphname}} 参会展位：{{curr_comdata.space_n}}</div>
                    <div class="sh_zwsz" style="top: 0;">
                        <el-button type="primary" size="mini" plain @click="setZw"><i class="el-icon-edit"></i> 设置展位
                        </el-button>
                        <el-button type="primary" size="mini" @click="showComJob"><i class="el-icon-suitcase-1"></i> 参会职位</el-button>
                    </div>
                    <div class="shcomtel">
                        <span v-if="info.linkman">
                            联系人：{{info.linkman}}<span v-if="info.linkjob">（{{info.linkjob}}）</span>
                        </span>
                        <span class="shcomtel_n" v-if="info.linktel">
                            联系电话：{{info.linktel}}
                        </span>
                        <span v-if="info.crm_uid != '0'">
                            业务员：{{info.crm_name}}
                        </span>
                    </div>
                    <div class="shshowall" style="height: calc(100% - 105px);">
                        <div class="shshow" style="overflow-y: auto; position: relative; height: 100%;">
                            <div class="shshow_tit"><i class="el-icon-office-building"></i> 基本资料</div>
                            <div class="shshow_p">
                                <div class="" v-if="info.welfare">企业福利：
                                    <el-tag style="margin-right: 5px;" v-for="(item,index) in info.welfare_n" :key="index" size="mini">
                                        {{item}}
                                    </el-tag>
                                </div>
                                <div class="" v-if="info.hy">从事行业：{{info.hy_n}}</div>
                                <div class="" v-if="info.pr">企业性质：{{info.pr_n}}</div>
                                <div class="" v-if="info.mun">企业规模：{{info.mun_n}}</div>
                                <div class="" v-if="info.provinceid">企业地址：{{info.job_city_one}} {{info.job_city_two}}
                                    {{info.job_city_three}} {{info.address}}
                                </div>
                                <div class="" v-if="info.content" v-html="info.content"></div>
                            </div>
                            <div class="shshow_tit" v-if="info.job_list.length > 0"><i class="el-icon-suitcase-1"></i>
                                招聘岗位
                            </div>
                            <ul class="shshow_joblist">
                                <li v-for="(item,index) in info.job_list" :key="index">
                                    <el-link type="primary" :underline="false">
                                        <div class="shshow_job">{{item.name}}</div>
                                    </el-link>
                                    <div class="shshow_jobinfo">
                                        <span class="shshow_jobxz">
                                            {{item.job_salary}}
                                        </span>
                                        <span class="shshow_line" v-if="item.job_exp">| {{item.job_exp == '不限' ? '不限经验' : item.job_exp}}</span>
                                        <span class="shshow_line" v-if="item.job_edu">| {{item.job_edu == '不限' ? '不限学历' : item.job_edu}}</span>
                                    </div>
                                    <span class="shshow_zt" v-if="item.ch_n == '已参会'">{{item.ch_n}}</span>
                                    <span class="shshow_zt shshow_ztno" v-else>{{item.ch_n}}</span>
                                </li>
                            </ul>
                        </div>
                        <div class="shcz">
                            <div class="wxsettip_small ">参会企业审核</div>
                            <template>
                                <el-radio v-model="info.zph.status" label="1">正常</el-radio>
                                <el-radio v-model="info.zph.status" label="2">未通过</el-radio>
                            </template>
                            <div class="wxsettip_small ">审核状态说明</div>
                            <el-input type="textarea" v-model="info.zph.statusbody" :rows="2" placeholder="请输入内容">
                            </el-input>
                            <div class=" shczbth">
                                <el-button type="primary" @click="comStatusSave(info.zph.id)" :disabled="submitLoading">提 交</el-button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--批量审核参会企业-->
        <div class="modluDrawer">
            <el-dialog title="参会企业审核" width="300px" :visible.sync="drawercomstatusmultiple" append-to-body :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>审核操作：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="multiComStatus" label="1">正常</el-radio>
                            <el-radio v-model="multiComStatus" label="2">未通过</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>审核说明：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input type="textarea" v-model="multiComStatusBody" :rows="2" placeholder="请输入内容">
                            </el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawercomstatusmultiple = false">取 消</el-button>
                    <el-button type="primary" @click="multipleComStatusSave">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!--参会企业详情 设置展位-->
        <el-drawer title="设置展位" :visible.sync="drawersetzw" :modal-append-to-body="false" append-to-body size="80%">
            <div class="yd_qy">{{curr_zphtitle}}展位选择</div>
            <div class="yd_qylist" v-for="(item,index) in space_list" style="margin-left: 20px;" :key="index">
                <el-divider content-position="center">{{item.name}}</el-divider>
                <div class="yd_ztbox">
                    <div :class="ydCls(item, childit)" @click="changezw(childit, item, $event)" v-for="(childit,index) in item.list" :key="item.id + index + ''">
                        <span v-if="childit.comstatus == '-1'" class="yd_zt_n">可预订</span>
                        <span v-if="childit.comstatus == '1'" class="yd_zt_n">已预定</span>
                        <span v-if="childit.comstatus == '0'" class="yd_zt_n">审核中</span>
                        <span v-if="childit.comstatus == '2' || childit.comstatus == '3'" class="yd_zt_n">不可预订</span>
                        <span class="yd_zt_zw">{{childit.name}}</span>
                    </div>
                </div>
            </div>
            <div class="yd_zt_fot">
                <div class="yd_zt_bth">
                    <div class="yd_zt_bthleft">
                        <div class="yd_zt_bthzwbox">展位：<span class="yd_zt_bthzw">{{sel_zwname}}</span></div>
                    </div>
                    <div class="yd_zt_bthbot">
                        <el-button type="primary" @click="saveChangeZw">添 加</el-button>
                    </div>
                </div>
            </div>
        </el-drawer>
        <!--参会职位-->
        <div class="modluDrawer">
            <el-dialog title="参会职位" width="300px" :visible.sync="drawercomjob" append-to-body :modal-append-to-body="false">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>选择职位：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-select v-model="jobids" filterable remote placeholder="选择职位" multiple>
                                <el-option v-for="item in job_arr" :key="item.value" :label="item.label" :value="item.value">
                                </el-option>
                            </el-select>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="drawercomjob = false">取 消</el-button>
                    <el-button type="primary" @click="saveComJob">确 定</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        shstatus: {type: String, default: ''}
    },
    data: function() {
        return {
            emptytext: '暂无数据',
            loading: false,
            submitLoading: false,
            status: this.shstatus,
            type: '1',
            keyword: '',
            checkedAll: false,
            selectedItem: [],
            tableData: [],
            currentPage: 1,
            perPage: 0,
            pageSizes: [],
            total: 0,
            curr_data: {},
            curr_comdata: {},
            sort_type: '',
            sort_col: '',
            job_arr: [],
            jobids: [],
            tableHig: true,
            comdrawersh: false,
            drawercomjob: false,
            drawersetzw: false,
            comid: '',
            com_arr: [],
            space_list: [],
            sel_comname: '',
            sel_zwid: '',
            sel_cdid: '',
            sel_zwname: '未选择',
            drawercomstatusmultiple: false,
            multiComStatus: '',
            multiComStatusBody: '',
            info: null,
            dtlislook: false,
            curr_zphtitle: '',
            oldData: null,
            islook: false,
            prevPage:0
        }
    },
    mounted() {

    },
    methods: {
        // 设置参会职位
        showComJob() {
            var that = this
            that.job_arr = []
            httpPost('m=neirong&c=zhaopinhui&a=getjoblist', { comid: that.curr_comdata.uid }).then(function(response) {
                if (response.data.error == 0) {
                    if (response.data.data.length > 0) {
                        that.jobids = that.info.jobid_arr
                        that.job_arr = response.data.data
                        that.drawercomjob = true
                    } else {
                        message.error('该企业没有可作展出的职位！');
                    }
                } else {
                    message.error('该企业没有可作展出的职位！');
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 设置参会职位 保存
        saveComJob() {
            var that = this
            var params = { zcomid: that.curr_comdata.id, zphjob: that.jobids.join(',') }
            httpPost('m=neirong&c=zhaopinhui&a=upjob', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.drawercomjob = false
                        that.info.jobid_arr = that.jobids
                        that.getList()
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        ydCls(item, childit) {
            var rt = ['yd_zt']
            if (childit.comstatus == '-1') {
                rt.push('yd_ztkyd')
            } else if (childit.comstatus == '1') {
                rt.push('yd_ztyyd')
            } else if (childit.comstatus == '0') {
                rt.push('yd_ztshz')
            } else if (childit.comstatus == '2' || childit.comstatus == '3') {
                rt.push('yd_ztbkyd')
            }
            if (childit.comstatus == '-1' && this.sel_zwid == childit.id) {
                rt.push('yd_ztkyd_active')
            }
            if (this.sel_zwid == childit.id) {
                this.sel_zwname = childit.name
            }
            return rt
        },
        // 设置展位
        setZw() {
            var that = this
            httpPost('m=neirong&c=zhaopinhui&a=comadd', { id: that.info.zph.zid }).then(function(response) {
                if (response.data.error == 0) {
                    that.space_list = response.data.data.spacelist
                    that.sel_zwid = that.curr_comdata.bid
                    that.sel_cdid = that.curr_comdata.cid
                    that.drawersetzw = true
                } else {
                    message.error('获取参会企业失败');
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 设置展位 保存
        saveChangeZw() {
            var that = this
            var params = { zcomid: that.curr_comdata.id, cid: that.sel_cdid, bid: that.sel_zwid }
            httpPost('m=neirong&c=zhaopinhui&a=upzhanwei', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        that.drawersetzw = false
                        that.curr_comdata.bid = that.sel_zwid
                        that.curr_comdata.cid = that.sel_cdid
                        that.getList()
                    });
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        // 选择展位
        changezw(childit, item, event) {
            if (childit.comstatus != '-1') {
                message.error('请选择可预订的展位！');
                return false
            }
            this.sel_zwid = childit.id
            this.sel_cdid = item.id
            this.sel_zwname = childit.name
            var curr_active = document.getElementsByClassName('yd_ztkyd_active')
            for (let i = 0; i < curr_active.length; i++) {
                curr_active[i].classList.remove('yd_ztkyd_active')
            }
            event.currentTarget.classList.add('yd_ztkyd_active')
        },
        // 添加参会企业选择企业
        comChange(data) {
            var that = this
            var selOption = this.com_arr.find((item) => item.value === data)
            this.sel_comname = selOption.label
            that.job_arr = []
            that.jobids = []
            httpPost('m=neirong&c=zhaopinhui&a=getjoblist', { comid: selOption.value }).then(function(response) {
                if (response.data.error == 0) {
                    if (response.data.data.length > 0) {
                        that.job_arr = response.data.data
                    } else {
                        message.error('该企业没有可作展出的职位！');
                    }
                } else {
                    message.error('该企业没有可作展出的职位！');
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        cominfo(data) {
            var that = this
            that.curr_comdata = data
            httpPost('m=neirong&c=zhaopinhui&a=audit', { id: this.curr_comdata.id, zph_info: 1 }).then(function(response) {
                if (response.data.error == 0) {
                    that.info = response.data.data
                    that.curr_zphtitle = that.info.zph.title
                    that.dtlislook = true
                    that.comdrawersh = true
                } else {
                    message.error('获取参会企业失败');
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        handleSelectionChange(val) {
            this.selectedItem = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.selectedItem.push(item.id);
                });
            }
            if (_this.selectedItem.length == 0) {
                _this.checkedAll = false;
            } else {
                if (_this.selectedItem.length == _this.tableData.length) {
                    _this.checkedAll = true;
                } else {
                    _this.checkedAll = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        handleSizeChange(val) {
            this.perPage = val;
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        delrow(id) {
            delConfirm(this, id, this.delete);
        },
        delAllBottom() {
            if (!this.selectedItem.length) {
                message.error('请选择要删除的数据项');
                return false;
            }
            delConfirm(this, this.selectedItem, this.delete);
        },
        async delete(id) {
            let that = this;
            let params = {
                del: id
            };
            httpPost('m=neirong&c=zhaopinhui&a=delcom', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg);
                    that.getList();
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function(error) {
                console.log(error);
            })
        },
        editData(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.tableData, index, copyRow);
            this.$nextTick(() => {
                let ref = column.property + index;
                $("#" + ref).focus();
            });
        },
        alterData(scope) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.tableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = { id: row.id };
            sendData[column.property] = row[column.property];
            httpPost('m=neirong&c=zhaopinhui&a=ajaxsort', sendData, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success('修改成功');
                } else {
                    message.error('修改失败');
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function(error) {
                console.log(error);
            });
        },
        sortChange: function(column) {
            if (column.order == 'descending') {
                this.sort_type = 'desc';
            } else if (column.order == 'ascending') {
                this.sort_type = 'asc';
            } else {
                this.sort_type = '';
            }
            this.sort_col = column.prop
            if (this.sort_col == 'space_n') {
                this.sort_col = 'sid'
            }
            this.search();
        },
        search() {
            this.currentPage = 1;
            this.getList();
        },
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            if (that.keyword) {
                params.keyword = that.keyword
            }
            if (that.type) {
                params.type = that.type
            }
            if (that.status) {
                params.status = that.status
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=neirong&c=zhaopinhui&a=com', params, {hideloading: true}).then(function(result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total)
                    if(that.prevPage != that.currentPage){
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0){
                        that.emptytext = "暂无数据";
                    }
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        // 参会企业批量审核
        multiAudit: function(){
            if (!this.selectedItem.length) {
                message.error('请选择要审核的数据项');
                return false;
            }
            this.drawercomstatusmultiple = true
        },
        // 参会企业批量审核
        multipleComStatusSave() {
            var that = this
            if (!that.selectedItem.length) {
                message.error('请选择要审核的数据项');
                return false;
            }
            that.comstatus({
                pid: that.selectedItem.join(','),
                status: that.multiComStatus,
                status_body: that.multiComStatusBody
            }, 2);
        },
        comStatusSave(id) {
            this.comstatus({ pid: id, status: this.info.zph.status, statusbody: this.info.zph.statusbody }, 1);
        },
        comstatus(params, tp) {
            var that = this
            this.submitLoading = true;
            httpPost('m=neirong&c=zhaopinhui&a=status', params).then(function(response) {
                if (response.data.error == 0) {
                    message.success(response.data.msg, function() {
                        if (tp == 1) {
                            that.dtlislook = false
                            that.comdrawersh = false
                        } else {
                            that.drawercomstatusmultiple = false
                        }
                        that.getList()
                    });
                } else {
                    message.error('获取参会企业失败');
                }
            }).catch(function(error) {
                console.log(error);
            }).finally(function() {
                that.submitLoading = false;
            });
            if (tp == 1) {
                that.dtlislook = false
                that.comdrawersh = false
            } else {
                that.drawercomstatusmultiple = false
            }
        },
    },
};
</script>
<style scoped></style>