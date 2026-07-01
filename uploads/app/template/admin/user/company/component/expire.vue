<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall tableSeacFromer">
				<el-input v-model="search_params.keyword" @keyup.enter.native="search" :placeholder="lc('admin_00340')" size="small" clearable>
					<el-select v-model="search_params.type" size="small" slot="prepend" :placeholder="lc('wap_com_00157')">
					    <el-option :label="lc('wap_com_00157')" value="1"></el-option>
					    <el-option :label="lc('admin_user_00140')" value="2"></el-option>
					</el-select>
				</el-input>
            </div>
            <!--<div class="tableSeachInpt tableSeachInptsmalltwo">-->
                <!--<el-date-picker v-model="etime" type="daterange" range-separator="至" start-placeholder="开始日期"-->
                                <!--end-placeholder="结束日期" size="mini">-->
                <!--</el-date-picker>-->
            <!--</div>-->
            <div class="tableSeachInpt">

            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }"
             style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                      @sort-change='sortChange' :default-sort="{ prop: 'uid', order: 'descending' }"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" width="120" sortable="custom"></el-table-column>
                <el-table-column prop="name" :label="lc('wap_com_00157')"></el-table-column>
                <el-table-column prop="username" :label="lc('admin_user_00140')"></el-table-column>
                <el-table-column prop="rating_name" :label="lc('admin_user_company_00018')"></el-table-column>
                <el-table-column prop="vip_etime_n" :label="lc('admin_00733')"></el-table-column>
                <el-table-column prop="ywy" :label="lc('admin_user_company_00049')">
                    <template slot-scope="props">
                        {{ lc('admin_00732') }}
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="delrow(scope.row.uid)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="delAllBottom" size="mini">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange"
                               @current-change="handleCurrentChange"
                               :current-page="currentPage" :page-sizes="pageSizes"
                               :page-size="perPage"
                               layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
    </div>
</template>
<script>
    module.exports = {
        data: function () {
            return {
                loading: false,
                emptytext: lc('wap_js_00113'),
                search_params:{
                    type: '1',
                    keyword: '',
                },
                etime: ['', ''],
                checkedAll: false,
                selectedItem: [],
                tableData: [],
                tableHig: true,
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                items: [
                    {type: '', label: lc('admin_user_00149')},
                ],
                islook: false,
                sort_type: '',
                sort_col: '',
                prevPage: 0
            }
        },
        mounted() {

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
        },
        methods: {
			getParams:function(params={},search=false){
				var that = this;
				for(let i in params){
					if(typeof that.search_params[i]!='undefined'){
						that.search_params[i] = params[i];
					}
				}
				if(search){
					this.search();
				}
			},
            sortChange: function (column) {
                if (column.order == 'descending') {
                    this.sort_type = 'desc';
                } else if (column.order == 'ascending') {
                    this.sort_type = 'asc';
                } else {
                    this.sort_type = '';
                }
                this.sort_col = column.prop
                this.search();
            },
            handleSelectionChange(val) {
                this.selectedItem = [];
                let _this = this;
                if (val.length) {
                    val.forEach(item => {
                        _this.selectedItem.push(item.uid);
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
                scrollToTop()
                this.getList()
            },
            handleCurrentChange(val) {
                this.currentPage = val;
                this.getList()
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
                if (that.search_params.type) {
                    params.type = that.search_params.type
                }
                if (that.search_params.keyword) {
                    params.keyword = that.search_params.keyword
                }
                if (that.sort_type && that.sort_col) {
                    params.order = that.sort_type
                    params.t = that.sort_col
                }
                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=user&c=company_expire&a=index', params, {hideloading: true}).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list
                        that.perPage = parseInt(res.data.perPage)
                        that.pageSizes = res.data.pageSizes
                        that.total = parseInt(res.data.total)
                        that.loading = false;
                        if(that.prevPage != that.currentPage){
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            scrollToTop()
                        }
                        if (that.tableData.length === 0){
                            that.emptytext = lc('wap_js_00113');
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            delrow(id) {
                delConfirm(this, id, this.delete);
            },
            delAllBottom() {
                if (!this.selectedItem.length) {
                    message.error(lc('admin_user_weipin_00005'));
                    return false;
                }
                delConfirm(this, this.selectedItem, this.delete);
            },
            async delete(id) {
                let that = this;
                let params = {
                    del: id
                };
                httpPost('m=user&c=company&a=del', params).then(function (response) {
                    if (response.data.error == 0) {
                        message.success(response.data.msg);
                        that.getList();
                    } else {
                        message.error(response.data.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
        },
    };
</script>
<style scoped>
    .tableSeacFromer{
    margin-right: 8px;
}
.tableSeacFromer .el-input-group__prepend{
    padding: 0;
    background: none;
}
.tableSeacFromer .el-select{
    margin-right: 0;
    width: 160px;
}
.tableSeacFromer .el-input{
    margin-right: 0;
}
</style>