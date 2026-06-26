<template>
    <div class="moduleElHight" style="padding: 0 20px;">
        <div class="moduleElTable" style="height: calc(100% - 55px);">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                      :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="80">
                </el-table-column>
                <el-table-column prop="content_n" label="{yun:}t key='wap_user_00102'{/yun}">
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00790'{/yun}" width="150">
                    <template slot-scope="scope">
                        <div>{{scope.row.nickname ? scope.row.nickname : scope.row.username}}</div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_00791'{/yun}" width="160">
                    <template slot-scope="scope">
                        <div>{{scope.row.add_time_n}}</div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00181'{/yun}" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                            <span v-else-if="scope.row.status == 2" class="admin_state2">{yun:}t key='wap_user_00167'{/yun}</span>
                            <span v-else class="admin_state5">{yun:}t key='wap_user_00166'{/yun}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="{yun:}t key='member_user_00048'{/yun}" width="200" align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="openAudit(scope.row)">{yun:}t key='member_user_00152'{/yun}</el-button>
                            <el-button size="small " plain @click="openEdit(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button type="danger" size="small " @click="del(scope.$index)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{yun:}t key='wap_js_00074'{/yun}
                </el-checkbox>
                <el-button @click="batch('del')" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                <el-button @click="batch('audit')" size="mini">{yun:}t key='admin_user_weipin_00037'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <!--<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"-->
                <!--               :current-page="page" :page-sizes="pageSizes" :page-size="limit"-->
                <!--               layout="total, sizes, prev, pager, next, jumper" :total="total">-->
                <!--</el-pagination>-->
            </div>
        </div>

        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00234'{/yun}" width="500px" :visible.sync="dialogAudit" append-to-body>
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00229'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAudit.status" label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                            <el-radio v-model="ruleFormAudit.status" label="2">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00779'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input
                                    type="textarea"
                                    :rows="2"
                                    placeholder=""
                                    v-model="ruleFormAudit.statusbody">
                            </el-input>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="dialogAudit = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="saveAudit" :disabled="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>

            <el-dialog title="{yun:}t key='admin_00792'{/yun}" width="500px" :visible.sync="dialogEdit" append-to-body>
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{yun:}t key='admin_00235'{/yun}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input
                                    type="textarea"
                                    :rows="2"
                                    placeholder=""
                                    v-model="ruleForm.content">
                            </el-input>
                        </div>
                    </div>

                </div>
                <div slot="footer" class="dialog-footer">
                    <el-button type="primary" @click="saveEdit" :disabled="saveLoading">{yun:}t key='wap_js_00094'{/yun}</el-button>
                </div>
            </el-dialog>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            aid: {type: String, default: ''},
            id: {type: String, default: ''},
            status: {type: Number, default: ''}
        },
        data: function () {
            return {
                emptytext: "{yun:}t key='wap_js_00113'{/yun}",
                loading: false,
                // list
                page: 1,
                limit: 0,
                list: [],
                total: 0,
                pageSizes: [],

                checkedAll: false, // 全选
                checkedAllIndeterminate: false,
                multipleSelection: [], // 多选值存储
                idArr: [],

                detail: {},

                saveLoading: false,

                // Audit
                dialogAudit: false,
                ruleFormAudit: {},

                // Update
                dialogEdit: false,
                ruleForm: {},
            }
        },
        created: function () {
            this.getList();
        },
        methods: {
            // handleSizeChange(val) {
            //     this.limit = val;
            //     this.getList();
            // },
            // handleCurrentChange(val) {
            //     this.page = val;
            //     this.getList();
            // },
            getList() {
                let that = this,
                    params = {
                        // page: that.page,
                        // limit: that.limit,
                    };

                if (typeof that.aid !== 'undefined') {
                    params.aid = that.aid;
                }
                if (typeof that.id !== 'undefined') {
                    params.id = that.id;
                }
                if (that.status !== '') {
                    params.status = that.status;
                }
                that.loading = true;
                that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
                httpPost('m=neirong&c=question&a=getcomment', params).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    that.list = data.list;
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                    }
                    // that.total = parseInt(data.total);
                    // that.pageSizes = data.page_sizes;
                    // if (that.limit === 0) {
                    //     that.limit = parseInt(data.limit); // 取系统配置默认数量
                    // }
                    // if (that.page > data.page) {
                    //     that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                    // }
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
                let msg = '';
                if (type == 'del') {
                    msg = "{yun:}t key='admin_user_weipin_00005'{/yun}";
                } else if (type == 'audit') {
                    msg = "{yun:}t key='admin_user_weipin_00001'{/yun}";
                }
                if (this.multipleSelection.length == 0) {
                    message.error(msg);

                    return false;
                }

                let idArr = [];
                this.multipleSelection.forEach(function (item) {
                    idArr.push(item.id);
                })
                this.idArr = idArr;

                if (type == 'del') {
                    this.del();
                } else if (type == 'audit') {
                    this.openAudit();
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

                if (typeof idx == 'undefined") { // {yun:}t key='member_com_00055'{/yun}
                    params.del = this.idArr;
                    msg = "{yun:}t key='common_00853'{/yun}";
                } else {// {yun:}t key='common_01711'{/yun}
                    params.id = that.list[idx].id;
                    msg = lc('admin_00333');
                }

                delConfirm(this, params, function (params) {
                    httpPost('m=neirong&c=question&a=delreview', params).then(function (res) {
                        if (res.data.error > 0) {
                            message.error(res.data.msg);
                        } else {
                            message.success(res.data.msg, function () {
                                that.$refs.multipleTable.clearSelection();
                                that.getList();
                            });
                        }
                    })
                }, msg)
            },

            openAudit(row) {
                this.dialogAudit = true;
                this.ruleFormAudit = {
                    id: typeof row == 'undefined' ? this.idArr : row.id,
                    status: typeof row == 'undefined' ? '' : row.status,
                    statusbody: typeof row == 'undefined' ? '' : row.statusbody,
                };
            },

            saveAudit() {
                let that = this,
                    params = that.ruleFormAudit;

                if (typeof params.status == 'undefined' || params.status === '') {
                    message.warning("{yun:}t key='admin_user_weipin_00015'{/yun}");
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=neirong&c=question&a=statusAnswerReview', params).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg, function () {
                            that.saveLoading = false;
                        });
                    } else {
                        that.dialogAudit = false;
                        that.$refs.multipleTable.clearSelection();
                        that.getList();
                        message.success(res.data.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            openEdit(row) {
                this.dialogEdit = true;
                this.detail = row;
                this.ruleForm = {
                    id: row.id,
                    support: row.support,
                    content: row.content_n
                };
            },

            saveEdit() {
                let that = this,
                    params = that.ruleForm;

                if (params.content === '') {
                    message.warning(lc('admin_vue_00065'));
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=neirong&c=question&a=save_review', params).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg, function () {
                            that.saveLoading = false;
                        });
                    } else {
                        that.dialogEdit = false;
                        that.$refs.multipleTable.clearSelection();
                        that.getList();
                        message.success(res.data.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
        },
        watch: {
            id: function (val, oldVal) {
                this.getList();
            },
            aid: function (val, oldVal) {
                this.getList();
            },
        }
    };
</script>
<style scoped></style>