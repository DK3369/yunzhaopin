<template>
    <div class="drawerModlue">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="dialogAdd = true">添加分类</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="list" border style="width: 100%"
                      :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="id" label="分类编号" width="100">
                </el-table-column>
                <el-table-column prop="typename" label="分类名称">
                    <template slot-scope="scope">
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
                <el-table-column fixed="right" label="操作" width="110">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="small" @click="delanv(scope.$index)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>

        <div class="modluDrawer">
            <el-dialog title="添加分类" :visible.sync="dialogAdd" :with-header="true" :append-to-body="false"
                       :show-close="true" width="30%" :modal="false">
                <el-form :model="ruleForm" ref="ruleForm" label-width="100px" class="demo-ruleForm">
                    <el-form-item label="分类名称" prop="name">
                        <el-input v-model="ruleForm.typename"></el-input>
                    </el-form-item>
                </el-form>
                <span slot="footer" class="dialog-footer">
                    <el-button type="primary" @click="submitForm('ruleForm')" :disabled="saveLoading">添加</el-button>
                    <el-button @click="dialogAdd = false">取消</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>
<!-- script -->
<script>
    module.exports = {
        data: function () {
            return {
                emptytext: '暂无数据',
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
                that.emptytext = "数据加载中";
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
                        that.emptytext = "暂无数据";
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
                    message.warning('请填写分类名称');
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
                    message.warning('请填写分类名称');
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
                delConfirm(this, params, this.del, '确定要删除该类别？');
            }
        },
    };
</script>
