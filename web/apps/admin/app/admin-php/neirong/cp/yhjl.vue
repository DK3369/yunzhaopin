<template>
<div id="hyfl" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="moduleSeachleft">
			<div class="moduleInptList">
			    <el-input :placeholder="lc('admin_00340')" v-model="keyword" class="input-with-select" clearable>
			        <template #prepend><el-select v-model="type" :placeholder="lc('admin_user_00140')">
			            <el-option :label="lc('admin_user_00140')" value="1"></el-option>
			            <el-option :label="lc('admin_00132')" value="2"></el-option>
			        </el-select></template>
			    </el-input>
			</div>
            <div class="moduleInptList" >
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="nrtopbtn">
        </div>
    </div>
    <div class="moduleElTable">
        <el-table :data="tableData" stripe border style="width: 100%;height: 100%;" @sort-change="sortChange"
                  :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"  @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="80" sortable="custom">
            </el-table-column>
            <el-table-column prop="name" :label="lc('admin_user_00140')" width="150">
            </el-table-column>
            <el-table-column prop="title" :label="lc('admin_00132')" min-width="180" show-overflow-tooltip>
            </el-table-column>
            <el-table-column :label="lc('admin_00126')" width="180">
                <template #default="props">
                    <el-tag type="success" size="small">{{ grouparr[props.row.keyid] }}</el-tag>
                </template>
            </el-table-column>
            <el-table-column prop="grade" :label="lc('admin_00131')" width="150">
            </el-table-column>
            <el-table-column :label="lc('admin_00130')" property="ctime_n" width="150">
            </el-table-column>
            <el-table-column :label="lc('member_user_00048')" width="90" fixed="right">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button type="danger" size="small" @click="delrow(scope.row.id)">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="">
            <el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button @click="delAllBottom" size="small">{{ lc('member_com_00055') }}</el-button>
        </div>
        <div class="modulePagNum">
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                           :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage"
                           layout="total, sizes, prev, pager, next, jumper" :total="total">
            </el-pagination>
        </div>
    </div>
</div>
</template>

<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

export default {
        data: function () {
            return {
                emptytext: lc('wap_js_00113'),
                loading: false,
                tableData: [], //表格数据
                checkedAll: false,
                selectedItem: [],
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                type: '1',
                keyword: '',
                grouparr: [],
				sort_type: '',
				sort_col: '',
                prevPage:0
            }
        },
        created: function () {

            this.getGroup();
            this.getList();
        },
        methods: {
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
            search() {
                this.currentPage = 1;
                this.getList();
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
            getGroup: async function () {
                let that = this;
                httpPost('m=neirong&c=evaluate&a=recordGroup', {}, {hideloading: true}).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.grouparr = res.data.arr
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            async getList() {
                let that = this;
                let params = {
                    page: that.currentPage,
                    pageSize: that.perPage
                }
                if (that.type) {
                    params.type = that.type
                }
                if (that.keyword) {
                    params.keyword = that.keyword
                }
				if (that.sort_type && that.sort_col) {
				    params.order = that.sort_type
				    params.t = that.sort_col
				}
                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=neirong&c=evaluate&a=record', params, {hideloading: true}).then(function (result) {
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
					message.error(lc('admin_00136'));
                    return false;
                }
                delConfirm(this, this.selectedItem, this.delete);
            },
            async delete(id) {
                let that = this;
                let params = {
                    del: id
                };
                httpPost('m=neirong&c=evaluate&a=delevaluatelog', params).then(function (response) {
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
        }
    }
</script>
