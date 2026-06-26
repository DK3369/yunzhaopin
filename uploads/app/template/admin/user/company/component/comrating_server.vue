<template>
    <!--会员-企业-套餐服务：增值服务-->
    <div class="moduleElHight">
        <div class="tableSeachInpt">
            <el-button type="primary" icon="el-icon-plus" size="mini" @click="handleAdd">{yun:}t key='admin_user_company_00207'{/yun}</el-button>
            <el-button type="primary" icon="el-icon-plus" plain size="mini" @click="handleDetailAdd">{yun:}t key='admin_00697'{/yun}</el-button>
        </div>
        <div class="admin_datatip" style="margin-bottom: 12px;">
            <i class="el-icon-document"></i>
            <span>{yun:}t key='admin_user_company_00164'{/yun}</span>
        </div>
        <div class="moduleElTable"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 135px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="80" sortable="custom"></el-table-column>
                <el-table-column prop="name" label="{yun:}t key='admin_user_company_00206'{/yun}" width="240">
                    <template slot-scope="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            {{ scope.row.name }}<img @click="editData(scope)" class="editIcon"
                            src="../../../admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="tcsl" label="{yun:}t key='member_user_00181'{/yun}">
                    <template slot-scope="scope">
                        <el-switch v-model="scope.row.display_n" active-text="{yun:}t key='admin_user_company_00171'{/yun}" inactive-text="{yun:}t key='wap_com_00242'{/yun}" @change="handleStatus(scope)"></el-switch>
                    </template>
                </el-table-column>
                <el-table-column prop="sort" label="{yun:}t key='member_com_00022'{/yun}"></el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="210" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="editRow(scope)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button size="mini" plain @click="handleDetail(scope)">{yun:}t key='member_com_00380'{/yun}</el-button>
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
            </div>
        </div>

        <div class="modluDrawer">
            <el-dialog :title="titleAddEdit" :visible.sync="addVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="600px">
                <div class="alogModlue">
                    <div class="alogModlList">
                        <div class="alogModlTite">
                            <span>{yun:}t key='admin_user_company_00206'{/yun}</span>
                        </div>
                        <div class="alogModInpt">
                            <el-input v-model="ruleForm.name" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                        </div>
                        <div class="alogModlTips">
                            <el-alert title="{yun:}t key='admin_00698'{/yun}" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="alogModlList">
                        <div class="alogModlTite">
                            <span>{yun:}t key='member_user_00181'{/yun}</span>
                        </div>
                        <div class="alogModInpt">
                            <el-radio-group v-model="ruleForm.display">
                                <el-radio label="1">{yun:}t key='admin_user_company_00205'{/yun}</el-radio>
                                <el-radio label="2">{yun:}t key='admin_user_company_00203'{/yun}</el-radio>
                            </el-radio-group>
                        </div>
                        <div class="alogModlTips">
                            <el-alert title="{yun:}t key='admin_user_company_00188'{/yun}" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                    <div class="alogModlList">
                        <div class="alogModlTite">
                            <span>{yun:}t key='member_com_00022'{/yun}</span>
                        </div>
                        <div class="alogModInpt">
                            <el-input v-model="ruleForm.sort" placeholder="{yun:}t key='admin_user_00342'{/yun}"></el-input>
                        </div>
                        <div class="alogModlTips">
                            <el-alert title="{yun:}t key='admin_user_company_00197'{/yun}" type="info" show-icon :closable="false">
                            </el-alert>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="cancelFrom">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitForm" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>

        <div class="modluDrawer">
            <el-drawer :title="titleDetail" :visible.sync="detailVisible" :append-to-body="true" :modal-append-to-body="false"
                :show-close="true" :destroy-on-close="true" size="95%">
                <comrating_server_detail :id="info?info.id:0"></comrating_server_detail>
            </el-drawer>
        </div>
        <div class="modluDrawer">
            <el-drawer title="{yun:}t key='admin_00697'{/yun}" :visible.sync="detailAddVisible" :append-to-body="true" :modal-append-to-body="false" :wrapper-closable="false" :destroy-on-close="true" size="770px">
                <comrating_server_detail_edit @child-event-list="detailAddVisible=false;getList();"></comrating_server_detail_edit>
            </el-drawer>
        </div>
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
            tableHig: true,
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            addVisible: false,
            titleAddEdit: "{yun:}t key='admin_user_company_00207'{/yun}",
            oldData: null,
            ruleForm: {
                id: '0',
                name: '',
                display: '2',
                sort: '",
            },
            info: {},
            submitLoading: false,
            detailAddVisible: false,//{yun:}t key='admin_00697'{/yun}
            detailVisible: false,
            titleDetail: "{yun:}t key='member_com_00380'{/yun}",
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
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            _this.loading = true;
            _this.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
            httpPost('m=user&c=company_comrating&a=server', params, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleAdd() {
            this.titleAddEdit = "{yun:}t key='admin_user_company_00207'{/yun}";
            this.info = {};
            this.ruleForm.id = '0';
            this.ruleForm.name = '';
            this.ruleForm.display = '2';
            this.ruleForm.sort = '';
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
            this.titleAddEdit = "{yun:}t key='admin_company_00010'{/yun}";
            this.info = scope.row;
            this.ruleForm.id = scope.row.id;
            this.ruleForm.name = scope.row.name;
            this.ruleForm.display = scope.row.display;
            if (this.ruleForm.display == '0') {
                this.ruleForm.display = '2';
            }
            this.ruleForm.sort = scope.row.sort;
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
            httpPost('m=user&c=company_comrating&a=delserver', params).then(function (response) {
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
            let sendData = {
                id: row.id
            };
            sendData[column.property] = row[column.property];
            httpPost('m=user&c=company_comrating&a=ajax', sendData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success("{yun:}t key='admin_user_company_00208'{/yun}");
                } else {
                    message.error(res.msg);
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleDetailAdd() {
            this.detailAddVisible = true;
        },
        handleDetail(scope) {
            this.info = scope.row;
            this.detailVisible = true;
            this.titleDetail = scope.row.name + " {yun:}t key='member_com_00380'{/yun}"
        },
        handleStatus(scope) {
            let _this = this;
            let id = scope.row.id;
            let display = scope.row.display_n ? 1 : 2;
            let params = {id: id, display: display};
            httpPost('m=user&c=company_comrating&a=opera', params).then(function (response) {
                let res = response.data;
                if (res.error !== 0) {
                    message.error("{yun:}t key='wap_01715'{/yun}");
                    _this.getList();
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            if (params.display != '1' && params.display != '2') {
                delete params.display;
            }
            if (params.name == '') {
                message.error("{yun:}t key='admin_company_00014'{/yun}");
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_comrating&a=save', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                } else {
                    message.error(res.msg);
                }
                _this.addVisible = false;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        cancelFrom() {
            this.addVisible = false;
            this.ruleForm.id = '0';
            this.ruleForm.name = '';
            this.ruleForm.display = '2';
            this.ruleForm.sort = '0';
        },
        doLayout(){
            if (this.$refs.multipleTable) {
                this.$nextTick(() => {
                    this.$refs.multipleTable.doLayout();
                })
            }
        }
    },
    watch: {
        "ruleForm.sort": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.sort = newValue;
                }
            },
            deep: true,
            immediate: true
        }
    },
    components: {
        'comrating_server_detail': httpVueLoader('./comrating_server_detail.vue'),
        'comrating_server_detail_edit': httpVueLoader('./comrating_server_detail_edit.vue'),
    }
};
</script>
<style>
.admin_datatip{
    margin-bottom: 0;
}
</style>
