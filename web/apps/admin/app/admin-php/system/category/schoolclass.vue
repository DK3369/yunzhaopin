<template>
<div id="schollfl" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="">{{ lc('admin_system_00125') }}</div>
            <div class="nrtopbtn">
                <el-button type="primary" icon="el-icon-document-add" size="small"
                    @click="addVisible = true">{{ lc('admin_00197') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" stripe border style="width: 100%;" height="100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" ref="multipleTable"
                @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('admin_system_00098')" width="180">
                </el-table-column>
                <el-table-column :label="lc('admin_system_00097')" property="name">
                    <template #default="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            {{ lc("admin_level1_category_value", [scope.row.name]) }}<img @click="editData(scope)" class="editIcon"
                                src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column prop="variable" :label="lc('admin_system_00123')"></el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="openManage(scope)">{{ lc('wap_com_00304') }}</el-button>
                            <el-button size="small" type="danger" @click="deleteRow(scope)">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <div class="modulecz modulePagButn">
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button @click="deleteRow(null, true)" size="small">{{ lc('member_com_00055') }}</el-button>
                </div>
            </div>
        </div>

        <el-dialog :title="lc('admin_system_00125')" width="40%" v-model="addVisible" :modal-append-to-body="false">
            <partclass_add :position="tableData" mod="category_schoolclass" @child-event-getlist="getList"></partclass_add>
        </el-dialog>
        <div class="modluDrawer">
            <el-drawer :title="lc('wap_com_00304')" v-model="manageVisible" :append-to-body="true"
                :modal-append-to-body="false" :show-close="true" :destroy-on-close="true" size="800px">
                <partclass_manage :id="info.id" mod="category_schoolclass" @child-event-getlist="getList"></partclass_manage>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import PartclassAdd from './component/partclass_add.vue'
import PartclassManage from './component/partclass_manage.vue'

const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})

export default {
        data: function () {
            return {
                emptytext: window.lc('wap_js_00113'),
                loading: false,
                tableData: [],
                checked: false,
                isIndeterminate: false,
                selectedItem: [],
                addVisible: false,
                oldData: null,
                info: {},
                manageVisible: false,
            }
        },
        mounted() {
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
            getList() {
                this.addVisible = false;
                let _this = this;
                _this.loading = true;
                _this.emptytext = window.lc('admin_user_weipin_00026');
                httpPost('m=system&c=category_schoolclass&a=index').then(function (response) {
                    let res = response.data;
                    _this.tableData = Array.isArray(res.data) ? res.data : [];
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = window.lc('wap_js_00113');
                    }
                }).catch(function (error) {
                    console.log(error);
                    _this.loading = false;
                });
            },
            deleteRow(scope, isMore) {
                let params = {};
                if (isMore) {
                    if (!this.selectedItem.length) {
                        message.error(window.lc('admin_user_weipin_00005'));
                        return false;
                    }
                    let list = [];
                    for (let item of this.selectedItem) {
                        list.push(item.id);
                    }
                    params.delType = 'more';
                    params.del = list;
                } else {
                    params.delType = 'single';
                    params.delid = scope.row.id;
                }

                delConfirm(this, params, this.delete);
            },
            delete(params) {
                let _this = this;
                httpPost('m=system&c=category_schoolclass&a=del', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(window.lc('admin_user_00187'));
                        _this.getList();
                    } else {
                        message.error(window.lc('admin_user_00186'));
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
                let sendData = { id: row.id };
                sendData[column.property] = row[column.property];
                httpPost('m=system&c=category_schoolclass&a=ajax', sendData, { hideloading: true }).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(window.lc('admin_user_company_00208'));
                    } else {
                        message.error(window.lc('admin_00187'));
                    }
                    _this.oldData = null;
                    _this.getList();
                }).catch(function (error) {
                    console.log(error);
                });
            },
            openManage(scope) {
                this.info = scope.row;
                this.manageVisible = true;
            }
        },
        components: {
            'partclass_add': PartclassAdd,
            'partclass_manage': PartclassManage,
        }
    }
</script>
