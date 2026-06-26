<template>
    <!--会员-企业-套餐服务：套餐设置-->
    <div class="moduleElHight">
        <div class="tableSeachInpt">
            <el-button type="primary" icon="el-icon-plus" size="mini" @click="handleAdd">{yun:}t key='admin_00689'{/yun}</el-button>
        </div>
        <div class="admin_datatip" style="margin-bottom: 12px;">
            <i class="el-icon-document"></i>
            <span>{yun:}t key='admin_user_company_00164'{/yun}</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 132px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" sortable="custom" width="80"></el-table-column>
                <el-table-column label="{yun:}t key='admin_00690'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class=" ">{{ scope.row.name }}</div>
                            <span class="gsd">{{ scope.row.type_n }} </span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="{yun:}t key='admin_user_company_00165'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="tcjiage ">{{ lc("admin_currency_yuan", [scope.row.service_price]) }}</div>
                            <span class="tctime">
                                 <template v-if="scope.row.service_time != ''">
                                    {{ lc("admin_day_count", [scope.row.service_time]) }}
                                 </template>
                                <template v-else>
                                    {yun:}t key='common_01936'{/yun}
                                </template>
                            </span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="{yun:}t key='admin_user_company_00168'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <template v-if="scope.row.type == 1">
                                <div>{{ lc("admin_refresh_count", [lc("admin_piece_count", [scope.row.breakjob_num])]) }}</div>
                                <div>{{ lc("admin_post_job_count", [lc("admin_piece_count", [scope.row.job_num])]) }}</div>
                            </template>
                            <template v-else-if="scope.row.type == 2">
                                <div>{{ lc("admin_refresh_count", [scope.row.breakjob_num == 0 ? '-' : lc("admin_daily_piece_count", [scope.row.breakjob_num])]) }}</div>
                                <div>{{ lc("admin_post_job_count", [lc("admin_piece_count", [scope.row.job_num])]) }}</div>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="{yun:}t key='admin_user_company_00167'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <template v-if="scope.row.type == 1">
                                <div>{{ lc("admin_interview_count", [lc("admin_times_count", [scope.row.interview])]) }}</div>
                                <div>{{ lc("admin_download_count", [lc("admin_piece_count", [scope.row.resume])]) }}</div>
                            </template>
                            <template v-else-if="scope.row.type == 2">
                                <div>{{ lc("admin_interview_count", [scope.row.interview == 0 ? '-' : lc("admin_daily_piece_count", [scope.row.interview])]) }}</div>
                                <div>{{ lc("admin_download_count", [scope.row.resume == 0 ? '-' : lc("admin_daily_piece_count", [scope.row.resume])]) }}</div>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="{yun:}t key='admin_00691'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class=" ">{{ lc("admin_top_days", [lc("admin_day_count", [scope.row.top_num])]) }}</span>
                            <span class=" ">{{ lc("admin_urgent_days", [lc("admin_day_count", [scope.row.urgent_num])]) }}</span>
                            <span class=" ">{{ lc("admin_recommend_days", [lc("admin_day_count", [scope.row.rec_num])]) }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="{yun:}t key='admin_00692'{/yun}" min-width="300">
                    <template slot-scope="scope">
                        <div class="modulePropsbox">
                            <div class="modulePropsboxsmall">
                                <template v-if="scope.row.type == 1">
                                    <div>{{ lc("admin_job_fair_signup", [lc("admin_session_count", [scope.row.zph_num])]) }}</div>
                                </template>
                                <template v-else-if="scope.row.type == 2">
                                    <div>{{ lc("admin_job_fair_signup", [scope.row.zph_num == 0 ? '-' : lc("admin_daily_session_count", [scope.row.zph_num])]) }}</div>
                                </template>
                            </div>
                            <div class="modulePropsboxsmall">
                                
                                <div></div>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="sort" label="{yun:}t key='member_com_00022'{/yun}" sortable="custom" width="80"></el-table-column>
                <el-table-column prop="zt" label="{yun:}t key='admin_user_company_00166'{/yun}" width="80">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.display == 1" class="admin_state1">{yun:}t key='admin_user_company_00171'{/yun}</span>
                            <span v-else class="admin_state2">{yun:}t key='admin_user_company_00173'{/yun}</span>
                            <!--<span class="admin_state1"> 已开启</span>-->
                            <!--<span class="admin_state2">未通过</span>-->
                            <!--<span class="admin_state3">已锁定</span>-->
                            <!--<span class="admin_state4">待审核</span>-->
                            <!--<span class="admin_state5">已暂停</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="editRow(scope)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--设置会员套餐-->
        <el-drawer :title="titleAddEdit" :visible.sync="addVisible" :destroy-on-close="true" :modal-append-to-body="false" append-to-body  :wrapper-closable="false" size="770px">
            <comrating_index_edit :id="info.id?info.id:0" :config="config" @child-event-list="handleCloseList"></comrating_index_edit>
        </el-drawer>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            loading: false,
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
            searchForm: {
                page: 1,
                limit: null,
            },
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            addVisible: false,
            titleAddEdit: "{yun:}t key='admin_00689'{/yun}",
            config: {},
            info: {},

            prevPage: 0
        }
    },
    created() {
        this.getBaseData();
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
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            _this.loading = true;
            _this.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
            httpPost('m=user&c=company_comrating&a=index', params, {hideloading: true}).then(function (response) {
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
                        _this.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        getBaseData() {
            let _this = this;
            httpPost('m=user&c=company_comrating&a=baseData', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.config = res.data.config;
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        getInfo() {

        },
        handleAdd() {
            this.titleAddEdit = "{yun:}t key='admin_00689'{/yun}";
            this.info = {};
            this.addVisible = true;
        },
        handleCloseList() {
            this.addVisible = false;
            this.getList();
        },
        handleClose(done) {
            done();
            this.addVisible = false;
        },
        editRow(scope) {
            this.titleAddEdit = "{yun:}t key='admin_company_00009'{/yun}";
            this.info = scope.row;
            this.addVisible = true;
        },
        deleteRow(scope, isMore) {
            let params = {};
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
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
            httpPost('m=user&c=company_comrating&a=delrating', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success("{yun:}t key='admin_user_00187'{/yun}");
                    _this.getList();
                } else {
                    message.error("{yun:}t key='admin_user_00186'{/yun}");
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        doLayout(){
            if (this.$refs.multipleTable) {
                this.$nextTick(() => {
                    this.$refs.multipleTable.doLayout();
                })
            }
        }
    },
    components: {
        'comrating_index_edit': httpVueLoader('./comrating_index_edit.vue'),
    }
};
</script>
<style scoped>
.mt-10 {
    margin-top: 10px;
}
.drawerModInfo{
    overflow-y: auto;
    height: calc(100% - 85px);
}
</style>