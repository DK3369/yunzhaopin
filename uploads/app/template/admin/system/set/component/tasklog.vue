<template>
    <div class="moduleElHight">
        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                <div class="tableSeachInpt" style="margin-bottom: 0px;;">
                    <el-input v-model="keyword" placeholder="{yun:}t key='admin_00340'{/yun}" size="small" prefix-icon="el-icon-search" clearable>
                    </el-input>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;">
                    <el-date-picker v-model="time" size="small" type="daterange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                        start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}" value-format="yyyy-MM-dd"
                        style="margin-right: 10px; text-align: left;" @change="search">
                    </el-date-picker>
                </div>
                <div class="tableSeachInpt" style="margin-bottom: 0px;">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 88px);">
            <el-table :data="tableData" border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                @selection-change="handleSelectionChange" ref="multipleTable"
                :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='sortChange' v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" sortable="custom" label="{yun:}t key='member_com_00345'{/yun}" width="140">
                </el-table-column>
                <el-table-column prop="cron_name" label="{yun:}t key='admin_00981'{/yun}">
                </el-table-column>
                <el-table-column prop="ctime_n" label="{yun:}t key='admin_00982'{/yun}" width="220">
                </el-table-column>
                <el-table-column fixed="right" label="{yun:}t key='member_user_00048'{/yun}" width="80">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" type="danger" @click="delrow(scope.row)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>

                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button size="mini" @click="delAllBottom">{yun:}t key='member_com_00055'{/yun}</el-button>
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
module.exports = {
    data: function () {
        return {
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
            loading: false,
            checkedAll: false,
            selectedItem: [],
            time: '',
            keyword: '',
            currentPage: 1,
            prevPage: 0,
            perPage: 0,
            pageSizes: [],
            total: 0,
            tableData: [],
            sort_type: '',
            sort_col: '',
        }
    },
    mounted() {

    },
    methods: {
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
        search() {
            this.currentPage = 1;
            this.getList();
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
            this.getList();
        },
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            if (that.time) {
                params.time = that.time[0] + '~' + that.time[1]
            }
            if (that.keyword) {
                params.keyword = that.keyword
            }
            if (that.sort_type && that.sort_col) {
                params.order = that.sort_type
                params.t = that.sort_col
            }
            that.loading = true;
            that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
            httpPost('m=system&c=set_cron&a=cronLog', params).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total);
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0){
                        that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        delrow(row) {
            delConfirm(this, row.id, this.delete);
        },
        delAllBottom() {
            if (!this.selectedItem.length) {
                this.$message({ showClose: true, message: "{yun:}t key='admin_user_weipin_00005'{/yun}", type: 'warning' });
                return false;
            }
            delConfirm(this, this.selectedItem, this.delete);
        },
        async delete(Ids) {
            let that = this;
            let params = {
                del: Ids
            };
            httpPost('m=system&c=set_cron&a=delLog', params).then(function (response) {
                if (response.data.error == 0) {
                    message.success("{yun:}t key='wap_user_00264'{/yun}", that.getList());
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
<style scoped></style>