<template>
<div id="cityfl" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="moduleSeachleft">


            <div class="tableSeackellsd" style="padding: 2px 0;">
                <el-input :placeholder="lc('admin_00340')" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                    <template #prepend><el-select v-model="searchForm.type" :placeholder="lc('wap_user_00100')">
                        <el-option :label="lc('wap_user_00103')" value="1"></el-option>
                        <el-option :label="lc('admin_00248')" value="2"></el-option>
                    </el-select></template>
                </el-input>
            </div>
            <div class="tableSeachInptsmall newsinput" v-for="(searchItem, searchIndex) in searchList">
                <el-select v-model="searchForm[searchItem.param]" size="small" :clearable="true" :placeholder="searchItem.name" @change="search">
                    <el-option v-for="(searchLabel, searchValue) in searchItem.value" :label="searchLabel" :value="searchValue"></el-option>
                </el-select>
            </div>

            <div class="newsbtnbox" style="margin-bottom: 0px;;">
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
    </div>

    <div class="moduleElTable">
        <el-table :data="list" :default-sort="{prop: 'date', order: 'descending'}" stripe border
                  ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="sortChange" :empty-text="emptytext"
                  style="width: 100%;" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
            <el-table-column type="selection" width="55"> </el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="90" sortable="custom">
            </el-table-column>
            <el-table-column :label="lc('wap_user_00103')" min-width="190">
                <template #default="scope">
                    <el-link :href="scope.row.ask_url" target="_blank" type="primary" :underline="false">{{scope.row.title}}</el-link>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_00252')" width="140">
                <template #default="scope">
                    <div class="admin_state">
                        <span v-if="scope.row.classname">{{scope.row.classname}}</span>
                        <span v-else class="admin_state2">{{ lc('admin_00247') }}</span>
                    </div>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_00248')" min-width="180">
                <template #default="scope">
                    <div>
                        <span v-if="scope.row.nickname">{{scope.row.nickname}}</span>
                        <span v-else>{{scope.row.username}}</span>
                    </div>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_00253')" width="110" prop="answer_num" sortable="custom">
                <template #default="scope">
                    <div style="padding-top: 10px;">
                        <el-badge v-if="scope.row.answer_num > 0" :value="scope.row.answer_num" :max="99" class="item">
                            <a href="javascript:void(0)" @click="openAnswer(scope.row)">{{ lc('wap_com_00427') }}</a>
                        </el-badge>
                        <span v-else>{{ lc('admin_00254') }}</span>
                    </div>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_00231')" width="100">
                <template #default="scope">
                    <el-switch v-model="scope.row.is_recom" active-color="#13ce66" inactive-color="#ccc"
                               active-value="1" inactive-value="0" @change="bindRec($event, scope.row)">
                    </el-switch>
                </template>
            </el-table-column>
            <el-table-column :label="lc('admin_00251')" prop="add_time" sortable="custom" min-width="180">
                <template #default="scope">
                    <div>{{scope.row.add_time_n}}</div>
                </template>
            </el-table-column>
            <el-table-column prop="zt" :label="lc('member_user_00181')" width="100">
                <template #default="scope">
                    <div class="admin_state">
                        <span v-if="scope.row.state == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                        <span v-else-if="scope.row.state == 2" class="admin_state2">{{ lc('wap_user_00167') }}</span>
                        <!--<span class="admin_state3">Locked</span>-->
                        <!--<span class="admin_state4">Pending review</span>-->
                        <span v-else class="admin_state5">{{ lc('wap_user_00166') }}</span>
                    </div>
                </template>
            </el-table-column>
            <el-table-column :label="lc('member_user_00048')" width="200" fixed="right">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small " plain @click="openAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>
                        <el-button size="small " plain @click="openEdit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                        <el-button type="danger" size="small" @click="del(scope.$index)">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="">
            <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
            <el-button @click="batch('audit')" size="small">{{ lc('admin_user_weipin_00037') }}</el-button>
        </div>
        <div class="modulePagNum">
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                           :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                           layout="total, sizes, prev, pager, next, jumper" :total="total">
            </el-pagination>
        </div>
    </div>

    <div class="modluDrawer">
        <el-dialog :title="lc('admin_00793')" width="500px" v-model="dialogAudit" :modal-append-to-body="false">
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
            </div>
            <template #footer><span class="dialog-footer">
                <el-button @click="dialogAudit = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="saveAudit">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
    </div>
    <div class="modluDrawer">
        <el-drawer :title="lc('admin_00249')" v-model="drawerEdit" append-to-body :show-close="true"
                   :with-header="true" size="700px">
            <edit :id="detail.id" @child-event="closeEdit"></edit>
        </el-drawer>
        <el-drawer :title="lc('admin_00250')" v-model="drawerAnswer" append-to-body :show-close="true"
                   :with-header="true" size="80%">
            <answer :id="detail.id" :status="status.answer" @child-event="closeAnswer"></answer>
        </el-drawer>
        <el-drawer :title="lc('admin_00227')" v-model="drawerReview" append-to-body :show-close="true"
                   :with-header="true" size="70%">
            <review :status="status.comment" @child-event="closeReview"></review>
        </el-drawer>
    </div>
</div>
</template>

<script>
import QuestionEdit from './component/question_edit.vue'
import QuestionAnswer from './component/question_answer.vue'
import QuestionReview from './component/question_review.vue'

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
                // Search filters
                searchList: [],
                searchForm: {
                    type: '1'
                },

                // List
                page: 1,
                limit: 0,
                list: [],
                total: 0,
                pageSizes: [],

                // List sorting
                t: '',
                order: '',

                checkedAll: false, // Select all
                checkedAllIndeterminate: false,
                multipleSelection: [], // Multi-select value storage
                idArr: [],

                detail: {},

                saveLoading: false,

                // Review
                dialogAudit: false,
                ruleFormAudit: {},

                // Edit
                drawerEdit: false,

                // Answer list
                drawerAnswer: false,

                // Comment list
                drawerReview: false,
                prevPage:0,
                status: {
                    answer: '',
                    comment: '',
                }, // Used when navigating from pending messages
            }
        },
        components: {
            'edit': QuestionEdit,
            'answer': QuestionAnswer,
            'review': QuestionReview,
        },
        created() {
            var that = this
            let query = window.parent.homeapp.$route.query;



            if (query.drawer) { // Use the specified drawer logic when a drawer is specified
                if (query.drawer == 'answer') {
                    that.drawerAnswer = true;
                } else if (query.drawer == 'comment') {
                    that.drawerReview = true;
                }
                if (query.status) {
                    that.status[query.drawer] = parseInt(query.status);
                }
            } else {
                if (query.status) {
                    that.$set(that.searchForm, 'status', parseInt(query.status));
                }
            }
            this.getGroup();
            this.getList();
        },
        methods: {
            getGroup(){
                let that = this;

                httpPost('m=neirong&c=question&a=getGroup', {}, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;
                    that.searchList = data.search_list;
                })
            },
            handleSizeChange(val) {
                this.limit = val;
                this.getList();
            },
            handleCurrentChange(val) {
                this.page = val;
                this.getList();
            },
            sortChange(event) {
                this.t = event.order ? event.prop : '';
                this.order = event.order ? event.order == 'descending' ? 'desc' : 'asc' : '';
                this.search();
            },
            search() {
                this.page = 1;
                this.getList();
            },
            getList() {
                let that = this,
                    searchForm = that.searchForm,
                    params = {
                        page: that.page,
                        limit: that.limit,
                        t: that.t,
                        order: that.order,
                    };
                    that.loading = true;
                    that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=neirong&c=question', {...params, ...searchForm}, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;
                    that.list = data.list;
                    that.total = parseInt(data.total);
                    that.pageSizes = data.page_sizes;
                    if (that.limit === 0) {
                        that.limit = parseInt(data.limit); // Use default count from system config
                    }
                    if (that.page > data.page) {
                        that.page = parseInt(data.page); // Use latest page after the last page is deleted
                    }
                    if(that.prevPage != that.page){
                        that.prevPage = that.page;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.list.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                })
            },

            // Batch operation
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
                    let msg = lc('admin_user_weipin_00001')
                    if (type == 'del') {
                        msg = lc('admin_00136')
                    } else if (type == 'audit') {
                        msg = lc('admin_00246')
                    }
                    message.error(msg);
                    return false;
                }

                let idArr = [];
                this.multipleSelection.forEach(function(item) {
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

                if (typeof idx == 'undefined') { // Batch delete
                    params.del = this.idArr;
                    msg = lc('common_00853');
                } else {// Single delete
                    params.id = that.list[idx].id;
                    msg = lc('admin_00333');
                }

                delConfirm(this, params, function (params) {
                    httpPost('m=neirong&c=question&a=del', params).then(function(res) {
                        if (res.data.error > 0) {
                            message.error(res.data.msg);
                        } else {
                            that.getList();
                            message.success(res.data.msg, function () {
                                that.$refs.multipleTable.clearSelection();
                            });
                        }
                    })
                }, msg)
            },

            bindRec(val, data) {
                let that = this;

                httpPost('m=neirong&c=question&a=recommend', {id: data.id, rec: val}).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    }
                })
            },

            openAudit(row) {
                this.dialogAudit = true;
                this.ruleFormAudit = {
                    id: typeof row == 'undefined' ? this.idArr : row.id
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

                httpPost('m=neirong&c=question&a=status', params).then(function(res) {
                    that.saveLoading = false;

                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        that.dialogAudit = false;
                        that.getList();
                        message.success(res.data.msg, function () {
                            that.$refs.multipleTable.clearSelection();
                        });
                    }
                })
            },

            openEdit(row) {
                this.drawerEdit = true;
                this.detail = row;
            },

            closeEdit() {
                this.drawerEdit = false;
                this.getList();
            },

            openAnswer(row) {
                this.drawerAnswer = true;
                this.detail = row;
            },

            closeAnswer() {
                this.drawerAnswer = false;
                this.getList();
            },

            closeReview() {
                this.drawerReview = false;
                this.getList();
            },
        }
    }
</script>
