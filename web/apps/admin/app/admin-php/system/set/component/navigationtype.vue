<template>
    <div class="drawerModlue">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="dialogAdd = true">{{ lc('admin_00197') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="list" border style="width: 100%"
                      :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="id" :label="lc('admin_system_00098')" width="100">
                </el-table-column>
                <el-table-column prop="typename" :label="lc('admin_system_00357')">
                    <template #default="scope">
                        <div class="moduleProps moduleTrButn" v-if="scope.row[scope.column.property + 'isShow']">
                            <el-input :ref="scope.column.property + scope.$index" :id="scope.column.property + scope.$index"
                                      v-model="scope.row.typename" @blur="editTypename(scope)"></el-input>
                        </div>
                        <div class="moduleProps moduleTrButn" v-else>
                            <span>{{ scope.row.typename }}</span>
                            <el-button type="text" icon="el-icon-edit" @click="showTypename(scope)"></el-button>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="110">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="small" @click="delanv(scope.$index)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>

        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00197')" v-model="dialogAdd" :with-header="true" :append-to-body="false"
                       :show-close="true" width="30%" :modal="false">
                <el-form :model="ruleForm" ref="ruleForm" label-width="100px" class="demo-ruleForm">
                    <el-form-item :label="lc('admin_system_00357')" prop="name">
                        <el-input v-model="ruleForm.typename"></el-input>
                    </el-form-item>
                </el-form>
                <template #footer><span class="dialog-footer">
                    <el-button type="primary" @click="submitForm('ruleForm')" :disabled="saveLoading">{{ lc('wap_js_00091') }}</el-button>
                    <el-button @click="dialogAdd = false">{{ lc('common.cancel') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
    </div>
</template>
<!-- script -->
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
                list: [],

                dialogAdd: false,
                ruleForm: {
                    typename: '',
                },

                oldData: null,

                saveLoading: false,
                
            }
        },
        created: function () {
            this.getList();
        },
        methods: {
            getList() {
                let that = this,
                    params = {};
                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=system&c=set_navigation&a=type', params).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    let list = data.list;

                    list.forEach(function(item, index) {
                        list[index].typenameEdit = false; // 提前赋值，方便后边排序修改
                    })

                    that.list = list;
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                })
            },
            showTypename(scope) {
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
            editTypename(scope) {
                if (this.oldData == null) {
                    return false;
                }
                let index = scope.$index;
                let row = scope.row;
                let column = scope.column;
                let copyRow = JSON.parse(JSON.stringify(row));
                this.$set(this.list, index, copyRow);
                if (row[column.property] === this.oldData[column.property]) {
                    copyRow[column.property + "isShow"] = false;
                    return false;
                }
                let that = this;
                let params = {id: row.id};
                params[column.property] = row[column.property];

                if (row[column.property] == '') {
                    message.warning(lc('admin_01393'));
                    return false;
                }

                httpPost('m=system&c=set_navigation&a=typename', params).then(function(response) {
                    let res = response.data;
                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        copyRow[column.property + "isShow"] = false;
                        that.oldData = null;
                        message.success(res.msg,function(){
                            that.getList();
                        });
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            submitForm() {
                let that = this,
                    params = that.ruleForm;

                if (typeof params.typename == 'undefined' || params.typename == '') {
                    message.warning(lc('admin_01393'));
                    return;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=system&c=set_navigation&a=typeadd', params).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.dialogAdd = false;
                        message.success(res.msg, function () {
                            that.ruleForm.typename = '';
                            that.$emit("child-event");
                            that.getList();
                        });
                    }
                    that.saveLoading = false;
                })
            },
            del(params) {
                let that = this;
                    
                httpPost('m=system&c=set_navigation&a=typedel', params).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.list.splice(params.id, 1);
                        message.success(res.msg,function(){
                            that.$emit("child-event");
                            that.getList();
                        });
                    }
                })
            },
            delanv(idx){
                let params = {
                    id: this.list[idx].id
                }
                delConfirm(this, params, this.del, lc('admin_vue_00057'));
            }
        },
    };
</script>
