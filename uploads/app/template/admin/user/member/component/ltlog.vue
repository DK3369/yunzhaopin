<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input placeholder="请输入用户名/ID" size="small" @keyup.enter.native="doUserQuery" v-model="search.keyword" class="input-with-select" clearable>
                    <el-select v-model="search.type" slot="prepend" placeholder="用户名">
                        <el-option label="用户名" value="1"></el-option>
                        <el-option label="用户ID" value="3"></el-option>
                    </el-select>
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-input placeholder="请输入内容" @keyup.enter.native="doUserQuery" size="small" prefix-icon="el-icon-search" v-model="search.content" clearable></el-input>
            </div>
            <div class="tableSeachInpt tableSeachInptsmalltwo">
                <el-date-picker v-model="search.time" type="daterange" range-separator="至" start-placeholder="开始日期" value-format="yyyy-MM-dd" end-placeholder="结束日期" size="mini" @change="doUserQuery"></el-date-picker>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.operas" size="small" slot="prepend" placeholder="操作内容" clearable @change="doUserQuery">
                    <el-option v-for="(value, key) in operasArr" :key="key" :label="value" :value="key"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.parrs" size="small" slot="prepend" placeholder="操作类型" clearable @change="doUserQuery">
                    <el-option label="增加" value="1"></el-option>
                    <el-option label="修改" value="2"></el-option>
                    <el-option label="删除" value="3"></el-option>
                    <el-option label="刷新" value="4"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="search.end" size="small" slot="prepend" placeholder="操作时间" clearable @change="doUserQuery">
                    <el-option v-for="(item, key) in time" :key="key" :label="item.label" :value="item.value"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="doUserQuery">查询</el-button>
            </div>
        </div>


        <div class="moduleElTable moduleElTabUselod"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe @selection-change="selectChange" ref="multipleTable"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @sort-change="shortChange"
                v-loading="loading">
                <template slot="empty">
                    <p>{{ dataText }}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" label="用户ID" width="110" sortable="custom"></el-table-column>
                <el-table-column prop="username" label="用户名" width="150">
                </el-table-column>
                <el-table-column prop="zzh" label="猎头名称" min-width="100" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <el-link :href="weburl + '/lietou/index.php?c=headhunter&look=admin&uid=' + scope.row.uid"
                            target="_blank">{{ scope.row.ltname }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="neirong" label="内容" min-width="180" show-overflow-tooltip>
                    <template slot-scope="scope">
                        {{ scope.row.content }}
                        <template v-if="scope.row.sub_n">
                            ；{{ scope.row.sub_n }}
                        </template>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="IP"  width="150"></el-table-column>
                <el-table-column prop="ctime_ymd" label="时间" width="180" sortable="custom"></el-table-column>
                <el-table-column label="操作" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="del(scope.row)">删除</el-button>
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
                <el-pagination :total="total" @current-change="userPageChange" :page-size="pageSize" :page-sizes="pageSizes"
                    @size-change="handleSizeChange" :current-page.sync="page"
                    layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        typelist: Array,
        time: Array,
    },
    data: function () {
        return {
            loading: false,
            dataText: '数据加载中',
            checkedAll: false,
            search: {
                content: '',
                keyword: '',
                parrs: '',
                end: "",
                time: "",
                type: '1',
                operas: ''
            },
            operasArr: {
                10: '职位',
                88: '财务',
                3: '下载简历',
                5: '收藏关注',
                6: '应聘委托',
                7: '基本信息',
                8: '修改密码',
                11: '修改账号',
                12: '账号认证',
                16: '图片',
                17: '积分兑换',
                18: '消息', 19: '问答', 25: '悬赏推荐', 26: '浏览'
            },
            checked: '',
            seachbutn: false,
            tableHig: true,
            tableData: [],
            items: [
                { type: '', label: '正常' },
            ],
            idsArr: [],
            pageSizes: [],
            total: 0,
            page: 1,
            pageSize: 0,
            uri: "m=user&c=",
            weburl: localStorage.getItem("sy_weburl"),

            prevPage: 0
        }
    },
    created() {
        this.getList();
    },
    methods: {
        shortChange(e) {
            let orderMap = { ascending: 'asc', descending: 'desc' }
            this.search.t = e.prop == 'ctime_ymd' ? 'ctime' : e.prop;
            this.search.order = orderMap[e.order];
            this.page = 1;
            this.getList();
        },
        selectChange: function (val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.id);
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
        handleSizeChange(val) {
            this.pageSize = val
            this.getList()
        },
        getList: function () {
            let _this = this;
            let url = _this.uri + 'admin_memberlog&a=index';
            _this.search.page = this.page;
            _this.search.pageSize = this.pageSize;
            _this.search.utype = 3;
            _this.loading = true;
            httpPost(url, _this.search, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data.data;
                    _this.total = res.data.total;
                    _this.loading = false;
                    _this.pageSizes = res.data.pageSizes;
                    if(_this.prevPage != _this.page){
                        _this.prevPage = _this.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0) {
                        _this.dataText = "暂无数据";
                    }
                }
            })
        },
        del: function (detail) {
            let _this = this,
                params = {};
            params.del = detail.id;
            let url = this.uri + 'admin_memberlog&a=delLog';
            let msg = '确定要删除?';
            delConfirm(_this, params, function (params) {
                httpPost(url, params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            _this.getList();
                        });
                    }
                })
            }, msg);
        },
        batchDel: function () {
            let ids = this.idsArr;
            if (!ids.length) {
                message.error('请选择需要删除的企业日志');
                return
            }
            let _this = this,
                params = {};
            params.del = ids;
            let url = this.uri + 'admin_memberlog&a=delLog'
            let msg = '确定要删除?';
            delConfirm(_this, params, function (params) {
                httpPost(url, params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            _this.getList();
                        });
                    }
                })
            }, msg);
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
    },
};
</script>
<style scoped>
.moduleElHight .moduleElTable {
    padding: 0;
    margin: 0;
    /* height: calc(100% - 110px); */
    width: 100%;
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

</style> 