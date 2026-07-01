<template>
    <!--会员-企业-职位日志-->
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <el-select v-model="searchForm.ktype" slot="prepend" :placeholder="lc('wap_user_00100')">
                            <el-option :label="lc('admin_00616')" value="1"></el-option>
                            <el-option :label="lc('wap_com_00288')" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" sortable="custom" width="120"></el-table-column>
                <el-table-column prop="job_name" :label="lc('wap_com_00288')" min-width="280">
                    <template slot-scope="scope">
                        <el-link :href="scope.row.joburl" target="_blank" type="primary">{{ scope.row.job_name }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="com_name" :label="lc('admin_00616')" min-width="280" show-overflow-tooltip>
                    <template slot-scope="scope">
                        <el-link :href="scope.row.comurl" target="_blank" type="primary">{{ scope.row.com_name }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column prop="ip" label="IP" width="150"></el-table-column>
                <el-table-column prop="port_n" :label="lc('admin_user_00159')" width="150"></el-table-column>
                <el-table-column prop="r_time_n" :label="lc('admin_00527')" width="150"></el-table-column>
                <el-table-column prop="remark" :label="lc('member_user_00242')" min-width="260" show-overflow-tooltip></el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        type: {type: [String, Number], default: 1},
    },
    data: function () {
        return {
            loading: false,
            searchForm: {
                page: 1,
                limit: null,
                type: this.type,
                keyword: null,
                ktype: '1',
            },
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//{{ lc('wap_js_00074') }}
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            emptytext: lc('wap_js_00113'),

            prevPage: 0
        }
    },
    created() {
        this.getList();
    },
    methods: {
        handleSelectionChange(val) {
            this.selectedItem = val;
            if (this.selectedItem.length == 0) {
                this.isIndeterminate = false;
                this.checked = false;
            } else {
                if (this.selectedItem.length == this.tableData.length) {
                    this.isIndeterminate = false;
                    this.checked = true;
                } else {
                    this.isIndeterminate = true;
                    this.checked = false;
                }
            }
        },
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        shortChange(e) {
            let orderMap = {ascending: 'asc', descending: 'desc'}
            this.searchForm.t = e.order ? e.prop : null;
            this.searchForm.order = orderMap[e.order];
            this.searchForm.page = 1;
            this.getList();
        },
        handleSizeChange(val) {
            this.searchForm.limit = val;
            this.getList();
        },
        handleCurrentChange(val) {
            this.searchForm.page = val;
            this.getList();
        },
        handleSearch() {
            this.searchForm.page = 1
            this.getList()
        },
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            _this.loading = true;
            _this.emptytext = lc('admin_user_weipin_00026');
            httpPost('m=user&c=company_job_refresh_log&a=index', params,{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.total = res.data.total;
                    _this.searchForm.limit = res.data.perPage;
                    _this.pageSizes = res.data.pageSizes;
                    _this.loading = false;
                    if(_this.prevPage != _this.searchForm.page){
                        _this.prevPage = _this.searchForm.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    if (_this.tableData.length === 0){
                        _this.emptytext = lc('wap_js_00113');
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        deleteRow(scope, isMore) {
            let params = {};
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error(lc('admin_user_weipin_00005'));
                    return false;
                }
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.id);
                }
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.id = scope.row.id;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_job_refresh_log&a=delSxLog', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(lc('admin_user_00187'));
                    _this.getList();
                } else {
                    message.error(lc('admin_user_00186'));
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
    },
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
    height: calc(100% - 90px) !important;
}
</style> 