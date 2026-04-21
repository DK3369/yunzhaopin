<template>
    <div class="moduleElHight" style="padding: 0 20px;">
        <div class="moduleElTable" style="height: calc(100% - 55px);">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                      :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" label="编号" width="80">
                </el-table-column>
                <el-table-column prop="content_n" label="内容">
                </el-table-column>
                <el-table-column label="评论者昵称" width="150">
                    <template slot-scope="scope">
                        <div>{{scope.row.nickname ? scope.row.nickname : scope.row.username}}</div>
                    </template>
                </el-table-column>
                <el-table-column label="评论时间" width="160">
                    <template slot-scope="scope">
                        <div>{{scope.row.add_time_n}}</div>
                    </template>
                </el-table-column>
                <el-table-column label="状态" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == 1" class="admin_state1">已审核</span>
                            <span v-else-if="scope.row.status == 2" class="admin_state2">未通过</span>
                            <span v-else class="admin_state5">未审核</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="操作" width="200" align="center">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="openAudit(scope.row)">审核</el-button>
                            <el-button size="small " plain @click="openEdit(scope.row)">修改</el-button>
                            <el-button type="danger" size="small " @click="del(scope.$index)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">全选
                </el-checkbox>
                <el-button @click="batch('del')" size="mini">批量删除</el-button>
                <el-button @click="batch('audit')" size="mini">批量审核</el-button>
            </div>
            <div class="modulePagNum">
                <!--<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"-->
                <!--               :current-page="page" :page-sizes="pageSizes" :page-size="limit"-->
                <!--               layout="total, sizes, prev, pager, next, jumper" :total="total">-->
                <!--</el-pagination>-->
            </div>
        </div>

        <div class="modluDrawer">
            <el-dialog title="评论回复审核" width="500px" :visible.sync="dialogAudit" append-to-body>
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>审核：</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAudit.status" label="1">正常</el-radio>
                            <el-radio v-model="ruleFormAudit.status" label="2">未通过</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>说明：</span>
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
                    <el-button @click="dialogAudit = false">取 消</el-button>
                    <el-button type="primary" @click="saveAudit" :disabled="saveLoading">确 定</el-button>
                </span>
            </el-dialog>

            <el-dialog title="修改评论" width="500px" :visible.sync="dialogEdit" append-to-body>
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>评论内容：</span>
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
                    <el-button type="primary" @click="saveEdit" :disabled="saveLoading">确认</el-button>
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
                emptytext: '暂无数据',
                loading: false,
                // 列表
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

                // 审核
                dialogAudit: false,
                ruleFormAudit: {},

                // 修改
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
                that.emptytext = "数据加载中";
                httpPost('m=neirong&c=question&a=getcomment', params).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    that.list = data.list;
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = "暂无数据";
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
                    msg = '请选择要删除的数据';
                } else if (type == 'audit') {
                    msg = '请选择要操作的数据项';
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

                if (typeof idx == 'undefined') { // 批量删除
                    params.del = this.idArr;
                    msg = '你确定要删除选中项吗？';
                } else {// 单个删除
                    params.id = that.list[idx].id;
                    msg = '你确定要删除当前项吗？';
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
                    message.warning('请选择审核状态');
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
                    message.warning('请输入评论内容');
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