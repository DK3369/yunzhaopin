<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmalltwo">
                <el-date-picker v-model="search.times" type="daterange" :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" size="mini" @change="doUserQuery"></el-date-picker>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-input :placeholder="lc('admin_user_00158')" @keyup.enter.native="doUserQuery" size="small" v-model="search.keyword" class="input-with-select" clearable>
                	<el-select v-model="search.type" slot="prepend" :placeholder="lc('admin_user_00136')">
                		<el-option :label="lc('admin_user_00140')" value="1"></el-option>
                		<el-option :label="lc('wap_user_00102')" value="2"></el-option>
                		<el-option :label="lc('admin_user_00130')" value="3"></el-option>
                	</el-select>
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="doUserQuery">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe  @selection-change="selectChange" ref="multipleTable"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @sort-change="shortChange" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" width="110"></el-table-column>
                <el-table-column prop="ltname" :label="lc('admin_00449')" width="150"></el-table-column>
                <el-table-column prop="username" :label="lc('admin_user_00140')" min-width="100" show-overflow-tooltip></el-table-column>
                <el-table-column prop="ip" label="IP"></el-table-column>
                <el-table-column prop="remoteport" :label="lc('admin_user_00159')"></el-table-column>
                <el-table-column prop="ctime_ymd" :label="lc('wap_js_00088')" sortable="custom"></el-table-column>
                <el-table-column prop="content" :label="lc('wap_user_00102')" min-width="180" show-overflow-tooltip></el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="del(scope.row)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batchDel" size="mini">{{ lc('member_com_00055') }}</el-button></div>
            <div class="modulePagNum">
                <el-pagination :total="total" @current-change="userPageChange" :page-size="pageSize"
                               :page-sizes="pageSizes"
                               @size-change="handleSizeChange"
                               :current-page.sync="page" layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
    </div>
</template>

<script>
module.exports = {
    props:{
        typelist: Array
    },
    data: function () {
        return {
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            checkedAll:false,
            input: '',
            select: '',
            value: true,
            tableHig: true,
            tableData: [],
            items: [
                { type: '', label: lc('admin_user_00149') },
            ],
            search:{
                times:'',
                type:'1',
                utype:3,
            },
            uri: "m=user&c=",
            total: 0,
            page: 1,
            idsArr: [],
            pageSize: 0,
            pageSizes:[],
            detail:{},

            prevPage: 0
        }
    },
    created() {
        this.getList();
    },
    methods: {
		shortChange(e) {
		    let orderMap = {ascending: 'asc', descending: 'desc'}
		    this.search.t = e.prop== 'ctime_ymd'? 'ctime' : e.prop ;
		    this.search.order = orderMap[e.order];
		    this.page = 1;
		    this.getList();
		},
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        getList: function () {
            let _this = this;
            let url = _this.uri + 'admin_loginlog&a=index';
            _this.search.page = this.page;
            _this.search.pageSize = this.pageSize;
            _this.loading = true
            httpPost(url, _this.search, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data.data;
                    _this.total = res.data.total;
                    _this.loading = false;
                    _this.pageSizes =res.data.pageSizes;
                    if(_this.prevPage != _this.page){
                        _this.prevPage = _this.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0) {
                        _this.dataText = lc('wap_js_00113');
                    }
                }
            })
        },
        selectChange: function (val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.id);
                });
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
        handleSizeChange(val) {
            this.pageSize = val;
            this.getList();
        },
        handleCurrentChange(val) {
            console.log(`当前页: ${val}`);
        },
        del: function (detail) {
            let _this = this,
				params = {};
			params.del = detail.id;
            let url = this.uri + 'admin_loginlog&a=dellog';
            let msg = lc('admin_vue_00028');
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
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
                message.error(lc('admin_vue_00034'));
                return
            }
            let _this = this,
				params = {};
			params.del = ids;
            let url = this.uri + 'admin_loginlog&a=dellog'
            let msg = lc('admin_vue_00028');
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
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
    }
};
</script>
<style scoped>

.moduleElHight .moduleElTable {
    padding: 0;
    margin: 0;
    height: calc(100% - 110px);
    width: 100%;
}
.moduleElTableHig {
    height: calc(100% - 95px) !important;
}
.tableSeachInptsmall .el-input{
       width: initial;
   }
    .tableSeachInptsmall .el-select{
    	   margin-right: 0px !important;
    }
    .el-input-group__prepend{
    	   background-color: #ffffff;
    	   padding: 0 0 0 20px;
    }
</style>