<template>
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt">
            </div>
            <div class="moduleSeachButn moduleSeachMart">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="openAdd">{yun:}t key='admin_user_weipin_00051'{/yun}</el-button>
            </div>
        </div>
        <div class="moduleElTable modulElTableOnes" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="60">
                </el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="120">
                </el-table-column>
                <el-table-column prop="days" label="{yun:}t key='admin_user_weipin_00053'{/yun}">
                    <template slot-scope="scope">
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
                <el-table-column prop="price" label="{yun:}t key='wap_00563'{/yun}">
                    <template slot-scope="scope">
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
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="80">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <el-button type="danger" size="mini" @click="del(scope.$index)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
                             @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="batch('del')" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_weipin_00051'{/yun}" :visible.sync="dialogAdd" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="350px">
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00413'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleFormAdd.days" placeholder=""
                                      @input="inputIntNumber($event, 'ruleFormAdd', 'days')">
                                <template slot="append">{yun:}t key='common_02067'{/yun}</template>
                            </el-input>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_user_weipin_00052'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleFormAdd.price" placeholder=""
                                      @input="inputIntNumber($event, 'ruleFormAdd', 'price')">
                                <template slot="append">{yun:}t key='common_02056'{/yun}</template>
                            </el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAdd = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveAdd" :disabled="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    data: function () {
        return {
            loading: false,
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
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
                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
                message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
                return false;
            }else if (this.multipleSelection.length == 0){
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
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

            if (typeof idx == 'undefined') { // {yun:}t key='member_com_00055'{/yun}
                params.del = this.idArr;
                msg = lc('common_00853');
            } else {// {yun:}t key='common_01711'{/yun}
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
                message.error("{yun:}t key='admin_00614'{/yun}");
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