<template>
    <!--会员-企业-套餐服务：套餐设置-->
    <div class="moduleElHight">
        <div class="tableSeachInpt">
            <el-button type="primary" icon="el-icon-plus" size="mini" @click="handleAdd">设置会员套餐</el-button>
        </div>
        <div class="admin_datatip" style="margin-bottom: 12px;">
            <i class="el-icon-document"></i>
            <span>可设置的等级包括：普通会员，高级会员，钻石会员等等，按照实际情况去设置等级的区分，会员等级需满足的条件和享受的优惠</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 132px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="编号" sortable="custom" width="80"></el-table-column>
                <el-table-column label="套餐名称/模式" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class=" ">{{ scope.row.name }}</div>
                            <span class="gsd">{{ scope.row.type_n }} </span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="服务金额/时间" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <div class="tcjiage ">{{ scope.row.service_price }}元</div>
                            <span class="tctime">
                                 <template v-if="scope.row.service_time != ''">
                                    {{ scope.row.service_time }}天
                                 </template>
                                <template v-else>
                                    不限
                                </template>
                            </span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="职位数量" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <template v-if="scope.row.type == 1">
                                <div>刷新：{{ scope.row.breakjob_num }}份</div>
                                <div>上架：{{ scope.row.job_num }}份</div>
                            </template>
                            <template v-else-if="scope.row.type == 2">
                                <div>刷新：{{ scope.row.breakjob_num == 0 ? '-' : '每日' + scope.row.breakjob_num + '份' }}</div>
                                <div>上架：{{ scope.row.job_num }}份</div>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="简历数量" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <template v-if="scope.row.type == 1">
                                <div>面试：{{ scope.row.interview }}次</div>
                                <div>下载：{{ scope.row.resume }}份</div>
                            </template>
                            <template v-else-if="scope.row.type == 2">
                                <div>面试：{{ scope.row.interview == 0 ? '-' : '每日' + scope.row.interview + '份' }}次</div>
                                <div>下载：{{ scope.row.resume == 0 ? '-' : '每日' + scope.row.resume + '份' }}</div>
                            </template>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="推广数量" width="140">
                    <template slot-scope="scope">
                        <div class="moduleProps">
                            <span class=" ">置顶：{{ scope.row.top_num }}天</span>
                            <span class=" ">紧急：{{ scope.row.urgent_num }}天</span>
                            <span class=" ">推荐：{{ scope.row.rec_num }}天</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="comd" label="其他数量" min-width="300">
                    <template slot-scope="scope">
                        <div class="modulePropsbox">
                            <div class="modulePropsboxsmall">
                                <template v-if="scope.row.type == 1">
                                    <div>招聘会报名：{{ scope.row.zph_num }}场</div>
                                </template>
                                <template v-else-if="scope.row.type == 2">
                                    <div>招聘会报名：{{ scope.row.zph_num == 0 ? '-' : '每日' + scope.row.zph_num + '场' }}</div>
                                </template>
                            </div>
                            <div class="modulePropsboxsmall">
                                
                                <div></div>
                            </div>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="sort" label="排序" sortable="custom" width="80"></el-table-column>
                <el-table-column prop="zt" label="服务状态" width="80">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.display == 1" class="admin_state1">已启用</span>
                            <span v-else class="admin_state2">未启用</span>
                            <!--<span class="admin_state1"> 已开启</span>-->
                            <!--<span class="admin_state2">未通过</span>-->
                            <!--<span class="admin_state3">已锁定</span>-->
                            <!--<span class="admin_state4">待审核</span>-->
                            <!--<span class="admin_state5">已暂停</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="editRow(scope)">修改</el-button>
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">批量删除</el-button>
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
            emptytext: '暂无数据',
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
            titleAddEdit: '设置会员套餐',
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
            _this.emptytext = "数据加载中";
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
                        _this.emptytext = "暂无数据";
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
            this.titleAddEdit = '设置会员套餐';
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
            this.titleAddEdit = '修改会员套餐';
            this.info = scope.row;
            this.addVisible = true;
        },
        deleteRow(scope, isMore) {
            let params = {};
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error('请选择要删除的数据');
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
                    message.success('删除成功！');
                    _this.getList();
                } else {
                    message.error('删除失败！');
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