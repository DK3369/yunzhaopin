<template>
    <div class="moduleElHight" style="padding: 0 20px;">
        <div class="moduleElTable">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                      :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="80">
                </el-table-column>
                <el-table-column prop="content_n" :label="lc('wap_user_00102')">
                </el-table-column>
                <el-table-column :label="lc('admin_00781')" width="150">
                    <template #default="scope">
                        <div>{{scope.row.nickname ? scope.row.nickname : scope.row.username}}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="comment" :label="lc('admin_00782')" width="80">
                </el-table-column>
                <el-table-column prop="support" :label="lc('admin_00230')" width="80">
                </el-table-column>
                <el-table-column :label="lc('admin_00783')" width="160">
                    <template #default="scope">
                        <div>{{scope.row.add_time_n}}</div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('wap_00167')" width="100">
                    <template #default="scope">
                        <div>
                            <el-badge v-if="scope.row.comment > 0" class="item">
                                <el-link target="_blank" type="primary" :underline="false" @click="openReview(scope.row)">{{ lc('wap_com_00427') }}</el-link>
                            </el-badge>
                            <span v-else>{{ lc('admin_00226') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00181')" width="100">
                    <template #default="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                            <span v-else-if="scope.row.status == 2" class="admin_state2">{{ lc('wap_user_00167') }}</span>
                            <span v-else class="admin_state5">{{ lc('wap_user_00166') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="200" align="center">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" plain @click="openAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>
                            <el-button size="small" plain @click="openEdit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button type="danger" size="small" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{{ lc('wap_js_00074') }}
                </el-checkbox>
                <el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="batch('audit')" size="small">{{ lc('admin_user_weipin_00037') }}</el-button>
            </div>
            <div class="modulePagNum">
                <!--<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"-->
                <!--               :current-page="page" :page-sizes="pageSizes" :page-size="limit"-->
                <!--               layout="total, sizes, prev, pager, next, jumper" :total="total">-->
                <!--</el-pagination>-->
            </div>
        </div>

        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00784')" width="500px" v-model="dialogAudit" append-to-body>
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00229') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-radio v-model="ruleFormAudit.status" label="1">{{ lc('admin_user_00149') }}</el-radio>
                            <el-radio v-model="ruleFormAudit.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00779') }}</span>
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
                <template #footer><span class="dialog-footer">
                    <el-button @click="dialogAudit = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="saveAudit" :disabled="saveLoading">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>

            <el-dialog :title="lc('admin_00785')" width="500px" v-model="dialogEdit" append-to-body>
                <div class="toolClasDia fenpeizhand">
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00224') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <span>{{detail.title}}</span>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00225') }}</span>
                        </div>
                        <div class="toolClasCont">
                            <el-input v-model="ruleForm.support" :placeholder="lc('admin_00786')"
                                      @input="inputIntNumber($event, 'ruleForm', 'support')"></el-input>
                        </div>
                    </div>
                    <div class="toolClasList">
                        <div class="toolClasTite">
                            <span>{{ lc('admin_00780') }}</span>
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
                <template #footer><div class="dialog-footer">
                    <el-button type="primary" @click="saveEdit" :disabled="saveLoading">{{ lc('wap_js_00094') }}</el-button>
                </div></template>
            </el-dialog>

            <el-drawer :title="lc('admin_00227')" v-model="drawerReview" append-to-body :show-close="true"
                       :with-header="true" size="70%">
                <review :aid="detail.id" @child-event="closeReview"></review>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import QuestionReview from './question_review.vue'

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
        props: {
            id: {type: String, default: ''},
            status: {type: String, default: ''}
        },
        data: function () {
            return {
                emptytext: lc('wap_js_00113'),
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

                // 评论列表
                drawerReview: false
            }
        },
        components: {
            'review': QuestionReview,
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
                        id: that.id
                    };

                if (that.status !== '') {
                    params.status = that.status;
                }

                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=neirong&c=question&a=getanswer', params).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    that.list = data.list;
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = lc('wap_js_00113');
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
                if (this.multipleSelection.length == 0) {
                    message.warning(lc('admin_user_weipin_00001'));
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

                if (typeof idx == 'undefined') { // {{ lc('member_com_00055') }}
                    params.del = this.idArr;
                    msg = lc('common_00853');
                } else {// {{ lc('common_01711') }}
                    params.id = that.list[idx].id;
                    msg = lc('admin_00333');
                }
                params.qid = that.id;

                delConfirm(this, params, function (params) {
                    httpPost('m=neirong&c=question&a=delanswer', params).then(function (res) {
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
                    message.warning(lc('admin_user_weipin_00015'));
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=neirong&c=question&a=statusAnswer', params).then(function(res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg, function() {
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

            inputIntNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9]/g,'');
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
                    message.warning(lc('admin_vue_00063'));
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=neirong&c=question&a=save_answer', params).then(function(res) {
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

            openReview(row) {
                this.drawerReview = true;
                this.detail = row;
            },

            closeReview() {
                this.drawerReview = false;
                this.getList();
            },
        },
        watch: {
            id: function (val, oldVal) {
                this.getList();
            },
        }
    };
</script>
<style scoped>
.moduleElHight .moduleElTable{
    height: calc(100% - 50px);
}
</style>