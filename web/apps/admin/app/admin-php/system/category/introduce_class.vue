<template>
<div id="hyfl" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="">{{ lc('wap_00527') }}</div>
            <div class="nrtopbtn">
                <el-button type="primary" icon="el-icon-document-add" @click="addVisible = true"
                    size="small">{{ lc('admin_00222') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" stripe border style="width: 100%;" height="100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" ref="multipleTable"
                @selection-change="handleSelectionChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="180"></el-table-column>
                <el-table-column :label="lc('admin_system_00128')" property="name">
                    <template #default="scope">
                        <el-input v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.name" @blur="alterData(scope)"></el-input>
                        <span v-else>
                            {{ scope.row.name }}<img @click="editData(scope)" class="editIcon"
                                src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_system_00100')" property="sort">
                    <template #default="scope">
                        <el-input type="number" v-if="scope.row[scope.column.property + 'isShow']"
                            :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                            v-model="scope.row.sort" @blur="alterData(scope)"
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                        <span v-else>
                            {{ scope.row.sort }}<img @click="editData(scope)" class="editIcon"
                                src="/admin/php-admin/images/bine.png" alt="" style="margin-left: 4px;" width="14" height="14">
                        </span>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <!-- <div class="TableLink">
                            <el-link :underline="false" type="primary" @click="editInfo(scope)">修改</el-link>
                            <el-link :underline="false" type="primary" @click="deleteRow(scope)">删除</el-link>
                        </div> -->
                        <div class="cz_button">
                            <el-button size="small" @click="editInfo(scope)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="small" @click="deleteRow(scope)" type="danger">{{ lc('wap_js_00077') }}</el-button>
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
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_00219')" v-model="addVisible" :destroy-on-close="true" :modal-append-to-body="false"
                size="50%">
                <introduce_class_add :id="id" @child-event-getlist="closeAddBox"></introduce_class_add>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import IntroduceClassAdd from './component/introduce_class_add.vue'

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
                emptytext: window.lc('wap_js_00113'),
                loading: false,
                id: 0,
                tableData: [], //表格数据
                checked: false,
                isIndeterminate: false,// checkbox 的不确定状态
                selectedItem: [],
                addVisible: false,
                oldData: null,
                
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
                httpPost('m=system&c=category_introduce_class&a=index').then(function (response) {
                    let res = response.data;
                    _this.tableData = res.data;
                    _this.loading = false;
                    if (_this.tableData.length === 0){
                        _this.emptytext = window.lc('wap_js_00113');
                    }
                }).catch(function (error) {
                    console.log(error);
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
                    // let index = scope.$index;
                    // this.tableData.splice(index, 1);
                    params.delType = 'single';
                    params.delid = scope.row.id;
                }

                delConfirm(this, params, this.delete);
            },
            delete(params) {
                let _this = this;
                httpPost('m=system&c=category_introduce_class&a=del', params).then(function (response) {
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
                httpPost('m=system&c=category_introduce_class&a=ajax', sendData, { hideloading: true }).then(function (response) {
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
            editInfo(scope) {
                this.id = parseInt(scope.row.id);
                this.addVisible = true;
            },
            closeAddBox() {
                this.addVisible = false;
                this.getList();
            }
        },
        components: {
            'introduce_class_add': IntroduceClassAdd,
        },
    }
</script>
