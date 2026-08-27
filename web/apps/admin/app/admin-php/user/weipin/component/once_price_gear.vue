<template>
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt">
            </div>
            <div class="moduleSeachButn moduleSeachMart">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="openAdd">{{ lc('admin_user_weipin_00051') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable modulElTableOnes" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="60">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="120">
                </el-table-column>
                <el-table-column prop="days" :label="lc('admin_user_weipin_00053')">
                    <template #default="scope">
                        <div class="moduleProps moduleTrButn" v-if="scope.row[scope.column.property + 'isShow']">
                            <el-input :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                                      v-model="scope.row.days" type="number" @blur="editTable(scope)"></el-input>
                        </div>
                        <div class="moduleProps moduleTrButn" v-else>
                            <span>{{ scope.row.days }}</span>
                            <el-button type="text" icon="el-icon-edit" @click="showTableEdit(scope)"></el-button>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="price" :label="lc('wap_00563')">
                    <template #default="scope">
                        <div class="moduleProps moduleTrButn" v-if="scope.row[scope.column.property + 'isShow']">
                            <el-input :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                                      v-model="scope.row.price" type="number" @blur="editTable(scope)"></el-input>
                        </div>
                        <div class="moduleProps moduleTrButn" v-else>
                            <span>{{ scope.row.price }}</span>
                            <el-button type="text" icon="el-icon-edit" @click="showTableEdit(scope)"></el-button>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="80">
                    <template #default="scope">
                        <div class="moduleElTaCaoz">
                            <el-button type="danger" size="small" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                             @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_weipin_00051')" v-model="dialogAdd" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="350px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00413') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleFormAdd.days" placeholder=""
                                      @input="inputIntNumber($event, 'ruleFormAdd', 'days')">
                                <template #append>{{ lc('common_02067') }}</template>
                            </el-input>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_user_weipin_00052') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleFormAdd.price" placeholder=""
                                      @input="inputIntNumber($event, 'ruleFormAdd', 'price')">
                                <template #append>{{ lc('common_02056') }}</template>
                            </el-input>
                        </div>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogAdd = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveAdd" :disabled="saveLoading">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
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
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            // list
            list: [],

            checkedAll: false, // 全选
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},

            saveLoading: false,

            // Add
            dialogAdd: false,
            ruleFormAdd: {},

            // 列表编辑
            oldData: null,
        }
    },

    mounted() {

    },
    created() {
        this.init();
    },
    methods: {
        init() {
            this.getList();
        },

        getList() {
            let that = this;
            that.loading = true;
            httpPost('m=user&c=weipin_once&a=price_gear',{}, {hideloading: true}).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.list = data.list;
                that.loading = false;
                if (that.list.length === 0) {
                    that.dataText = lc('wap_js_00113');
                }
            })
        },

        // 批量操作
        handleSelectionChange(val) {
            if (val.length == 0) {
                this.checkedAll = false;
                this.checkedAllIndeterminate = false;
            } else {
                if (val.length === this.list.length) {
                    this.checkedAll = true;
                    this.checkedAllIndeterminate = false;
                } else {
                    this.checkedAll = false;
                    this.checkedAllIndeterminate = true;
                }
            }
            this.multipleSelection = val;
        },
        batch(type) {
            let that = this;
            if (this.multipleSelection.length == 0 && type == 'del') {
                message.error(lc('admin_user_weipin_00005'));
                return false;
            }else if (this.multipleSelection.length == 0){
                message.error(lc('admin_user_weipin_00001'));
                return false;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.id);
            })
            this.idArr = idArr;

            if (type == 'del') {
                this.del();
            }
        },
        checkAll(val) {
            val ? this.checkedAllIndeterminate = false : '';
            this.$refs.multipleTable.toggleAllSelection();
        },

        del(idx) {
            let that = this,
                params = {},
                msg = '';

            if (typeof idx == 'undefined') { // {{ lc('member_com_00055') }}
                params.del = this.idArr;
                msg = lc('common_00853');
            } else {// {{ lc('common_01711') }}
                params.del = that.list[idx].id;
                msg = lc('admin_00333');
            }

            delConfirm(this, params, function (params) {
                httpPost('m=user&c=weipin_once&a=price_gear_del', params).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        that.getList();
                        that.$refs.multipleTable.clearSelection();
                        message.success(res.data.msg);
                    }
                })
            }, msg)
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },

        openAdd(row) {
            this.ruleFormAdd = {};
            this.dialogAdd = true;
        },

        saveAdd() {
            let that = this,
                ruleForm = that.ruleFormAdd;

            if (typeof ruleForm.days === 'undefined' || $.trim(ruleForm.days) == "" || $.trim(ruleForm.days) == 0) {
                message.error(lc('admin_00614'));
                return false;
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_once&a=price_gear_add', ruleForm).then(function (response) {
                let res = response.data;

                if (res.error > 0) {
                    that.saveLoading = false;
                    message.error(res.msg);
                } else {
                    that.dialogAdd = false;
                    that.getList();
                    message.success(res.msg, function () {
                        that.saveLoading = false;
                    });
                }
            })
        },

        showTableEdit(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.list, index, copyRow);
            this.$nextTick(() => {
                let ref = column.property + index;
                $("#" + ref).focus();
            });
        },

        editTable(scope) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.list, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let sendData = {id: row.id};
            sendData[column.property] = row[column.property];
            httpPost('m=user&c=weipin_once&a=price_gear_ajax', sendData).then(function (response) {
                let res = response.data;
                if (res.error > 0) {
                    message.error(res.msg);
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            });
        },
    },
};
</script>
<style scoped></style>