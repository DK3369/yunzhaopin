<template>
<div id="daohaapp" class="moduleElenAl">
    <div class="moduleSeachs" style="padding-bottom: 0px;">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList" style="margin-bottom: 8px;">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <template #prepend><el-select v-model="searchForm.type" :placeholder="lc('wap_user_00100')">
                            <el-option :label="lc('member_com_00299')" value="1"></el-option>
                            <el-option :label="lc('wap_user_00163')" value="2"></el-option>
                            <el-option :label="lc('admin_user_00367')" value="3"></el-option>
                            <el-option :label="lc('wap_user_00162')" value="4"></el-option>
                            <el-option :label="lc('wap_com_00313')" value="5"></el-option>
                        </el-select></template>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.status" size="small" :placeholder="lc('wap_com_00406')" clearable @change="handleSearch">
                        <el-option :label="lc('wap_user_00166')" value="0"></el-option>
                        <el-option :label="lc('wap_user_00165')" value="1"></el-option>
                        <el-option :label="lc('wap_user_00167')" value="2"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.job" size="small" :placeholder="lc('wap_00516')" clearable @change="handleSearch">
                        <el-option :label="lc('admin_user_00372')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00373')" value="2"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.zx" size="small" :placeholder="lc('member_user_00061')" clearable @change="handleSearch">
                        <el-option :label="lc('common_01940')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00179')" value="3"></el-option>
                        <el-option :label="lc('admin_user_00178')" value="7"></el-option>
                        <el-option :label="lc('admin_user_00180')" value="15"></el-option>
                        <el-option :label="lc('admin_user_00175')" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.hf" size="small" :placeholder="lc('admin_user_00369')" clearable @change="handleSearch">
                        <el-option :label="lc('common_01940')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00179')" value="3"></el-option>
                        <el-option :label="lc('admin_user_00178')" value="7"></el-option>
                        <el-option :label="lc('admin_user_00180')" value="15"></el-option>
                        <el-option :label="lc('admin_user_00175')" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
        </div>
    </div>
    <div class="moduleElTable">
        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }} {{ lc("admin_total_count", [numAll]) }}
            <span class="admin_datatip_n">{{ lc("admin_approved_count", [numAudited]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [numUnaudited]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_failed_count", [numFailed]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <el-table :data="tableData" border style="width: 100%"
            :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="calc(100% - 43px)" ref="multipleTable"
            @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
            <template #empty>
                <p>{{dataText}}</p>
            </template>
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="80" sortable="custom"></el-table-column>
            <el-table-column prop="username" :label="lc('member_com_00299')" width="100"></el-table-column>
            <el-table-column prop="content" :label="lc('wap_user_00162')" min-width="200" show-overflow-tooltip></el-table-column>
            <el-table-column prop="job_name" :label="lc('wap_user_00163')" min-width="200" show-overflow-tooltip>
            </el-table-column>
            <el-table-column prop="com_name" :label="lc('admin_user_00367')" min-width="220" show-overflow-tooltip>
                <template #default="scope">
                    <el-link :href="scope.row.com_url" target="_blank" type="primary">{{ scope.row.com_name }}</el-link>
                </template>
            </el-table-column>
            <el-table-column prop="datetime" :label="lc('member_user_00061')" width="150">
                <template #default="scope">{{ scope.row.datetime_n }}</template>
            </el-table-column>
            <el-table-column prop="reply" :label="lc('wap_com_00313')" min-width="200" show-overflow-tooltip></el-table-column>
            <el-table-column prop="reply_time" :label="lc('admin_user_00369')" width="150">
                <template #default="scope">
                    <template v-if="scope.row.reply_time">{{ scope.row.reply_time_n }}</template>
                    <template v-else>{{ lc('admin_user_00371') }}</template>
                </template>
            </el-table-column>
            <el-table-column prop="status" :label="lc('member_user_00181')" width="100" fixed="right">
                <template #default="scope">
                    <div class="admin_state">
                        <span v-if="scope.row.status == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                        <span v-else-if="scope.row.status == 0" class="admin_state4">{{ lc('wap_user_00166') }}</span>
                        <span v-else-if="scope.row.status == 2" class="admin_state2">{{ lc('wap_user_00167') }}</span>
                        <template v-else="">--</template>
                        <!--<span class="admin_state1">已审核</span>-->
                        <!--<span class="admin_state2">未通过</span>-->
                        <!--<span class="admin_state3">已锁定</span>-->
                        <!--<span class="admin_state4">待审核</span>-->
                        <!--<span class="admin_state5">已暂停</span>-->
                    </div>
                </template>
            </el-table-column>
            <el-table-column :label="lc('member_user_00048')" width="210" fixed="right">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" plain @click="handleStatus(scope)">{{ lc('member_user_00152') }}</el-button>
                        <el-button size="small" plain @click="handleView(scope)">{{ lc('wap_com_00427') }}</el-button>
                        <el-button type="danger" size="small" @click="deleteRow(scope)">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="modulecz">
            <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button @click="deleteRow(null, true)" size="small">{{ lc('member_com_00055') }}</el-button>
            <el-button @click="handleStatus(null, true)" size="small">{{ lc('admin_user_weipin_00037') }}</el-button>
        </div>
        <div class="modulePagNum">
            <div class="modulePagNum" style="margin: 0 auto;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    v-model:current-page="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
    </div>
    <!-- 审核弹窗 -->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_user_00366')" v-model="statusVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="400px">
            <div class="wxsettip_small ">{{ lc('admin_user_00368') }}</div>
            <template>
                <el-radio v-model="ruleFormStatus.status" label="1">{{ lc('admin_user_00149') }}</el-radio>
                <el-radio v-model="ruleFormStatus.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
            </template>
            <div class="wxsettip_small ">{{ lc('admin_user_00365') }}</div>
            <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleFormStatus.statusbody">
            </el-input>
            <template #footer><span class="dialog-footer">
                <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
            </span></template>
        </el-dialog>
    </div>
    <!-- 查看弹窗 -->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_user_00370')" v-model="viewVisible" :with-header="true" :modal-append-to-body="false"
            :show-close="true" width="400px">
            <div class="wxsettip_small ">{{ lc('wap_user_00162') }}</div>
            <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleFormView.content">
            </el-input>
            <div class="wxsettip_small ">{{ lc('wap_com_00313') }}</div>
            <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleFormView.reply">
            </el-input>
            <template #footer><span class="dialog-footer">
                <el-button @click="resetFormView('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormView('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
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
                searchForm: {
                    page: 1,
                    limit: null,
                    type: "1",
                    keyword: null,
                    status: null,
                    job: null,
                    zx: null,
                    hf: null,
                },
                numAll: 0,
                numAudited: 0,//已审核
                numUnaudited: 0,//未审核
                numFailed: 0,//未通过
                total: 0,
                tableData: [],
                pageSizes: [],
                tableHig: true,
                checked: false,//全选
                isIndeterminate: false,// checkbox 的不确定状态
                selectedItem: [],
                //审核
                statusVisible: false,
                ruleFormStatus: {
                    pid: null,
                    status: null,//操作审核
                    statusbody: null,//审核说明
                },
                //查看
                viewVisible: false,
                ruleFormView: {
                    id: null,
                    content: '',
                    reply: '',
                },
                submitLoading: false,

                prevPage: 0
            }
        },
        mounted() {
            var that = this
            setTimeout(function () {
                that.getStatistFun();
            }, 200)
        },
        created() {
            var that = this
            let query = window.parent.homeapp.$route.query;
            if (query.status) {
                that.searchForm.status = query.status;
            }

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
            shortChange(e) {
                let orderMap = {ascending: 'asc', descending: 'desc'}
                this.searchForm.t = e.order ? e.prop : null;
                this.searchForm.order = orderMap[e.order];
                this.searchForm.page = 1;
                this.getList();
            },
            handleSizeChange(val) {
                this.searchForm.limit = val;
                this.getList();
            },
            handleCurrentChange(val) {
                this.searchForm.page = val;
                this.getList();
            },
            handleSearch() {
                this.searchForm.page = 1
                this.getList()
            },
            getStatistFun:function(){
                let that = this;
                httpPost('m=user&c=users_msg&a=getStatist', {},{hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        that.numAll = res.data.numAll;
                        that.numAudited = res.data.numAudited;
                        that.numUnaudited = res.data.numUnaudited;
                        that.numFailed = res.data.numFailed;
                    }
                })
            },
            getList() {
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.searchForm));
                for (let index in params) {
                    (params[index] === '') && (params[index] = null);
                }
                _this.loading = true;
                httpPost('m=user&c=users_msg&a=index', params, {hideloading: true}).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        _this.tableData = res.data.list;
                        _this.total = res.data.total;
                        _this.searchForm.limit = res.data.perPage;
                        _this.pageSizes = res.data.pageSizes;
                        _this.loading = false;
                        if(_this.prevPage != _this.searchForm.page){
                            _this.prevPage = _this.searchForm.page;
                            _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        if (_this.tableData.length === 0) {
                            _this.dataText = lc('wap_js_00113');
                        }
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            deleteRow(scope, isMore) {
                let params = {};
                if (isMore) {
                    if (!this.selectedItem.length) {
                        message.error(lc('admin_user_weipin_00005'));
                        return false;
                    }
                    let list = [];
                    for (let item of this.selectedItem) {
                        list.push(item.id);
                    }
                    params.del = list.join(',');
                } else {
                    // let index = scope.$index;
                    // this.tableData.splice(index, 1);
                    params.id = scope.row.id;
                }
                delConfirm(this, params, this.delete);
            },
            delete(params) {
                let _this = this;
                httpPost('m=user&c=users_msg&a=del', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(lc('admin_user_00187'));
                        _this.getList();
                    } else {
                        message.error(lc('admin_user_00186'));
                    }
                }).catch(function (error) {
                    console.log(error);
                });
            },
            handleStatus(scope, isMore) {
                if (isMore) {
                    if (!this.selectedItem.length) {
                        message.error(lc('admin_00572'));
                        return false;
                    }
                    let list = [];
                    for (let item of this.selectedItem) {
                        list.push(item.id);
                    }
                    this.ruleFormStatus.pid = list.join(',');
                    this.ruleFormStatus.status = null;
                    this.ruleFormStatus.statusbody = '';
                    this.statusVisible = true;
                } else {
                    this.ruleFormStatus.pid = scope.row.id;
                    this.ruleFormStatus.status = scope.row.status == 1 ? scope.row.status : null;
                    let _this = this;
                    let params = {id: scope.row.id};
                    httpPost('m=user&c=users_msg&a=lockinfo', params).then(function (response) {
                        let res = response.data;
                        if (res.error === 0) {
                            _this.ruleFormStatus.statusbody = res.data;
                        }
                        _this.statusVisible = true;
                    }).catch(function (error) {
                        console.log(error);
                    });
                }
            },
            submitFormStatus(formName) {
                // this.$refs[formName].validate((valid) => {if (valid) {}});
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormStatus));
                if (params.status == null) {
                    message.error(lc('admin_user_weipin_00015'));
                    return false;
                }
                _this.submitLoading = true;
                httpPost('m=user&c=users_msg&a=status', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(res.msg);
                        _this.resetFormStatus();
                        _this.getList();
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    _this.submitLoading = false;
                });
            },
            resetFormStatus(formName) {
                //this.$refs[formName].resetFields();
                this.ruleFormStatus.pid = null;
                this.ruleFormStatus.status = null;
                this.ruleFormStatus.statusbody = '';
                this.statusVisible = false;
            },
            handleView(scope) {
                this.ruleFormView.id = scope.row.id;
                let _this = this;
                let params = {id: scope.row.id};
                httpPost('m=user&c=users_msg&a=msgshow', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        let row = res.data;
                        _this.ruleFormView.content = row.content;
                        _this.ruleFormView.reply = row.reply;
                    }
                    _this.viewVisible = true;
                }).catch(function (error) {
                    console.log(error);
                });
            },
            submitFormView(formName) {
                // this.$refs[formName].validate((valid) => {if (valid) {}});
                let _this = this;
                let params = JSON.parse(JSON.stringify(this.ruleFormView));
                _this.submitLoading = true;
                httpPost('m=user&c=users_msg&a=msgedit', params).then(function (response) {
                    let res = response.data;
                    if (res.error === 0) {
                        message.success(res.msg);
                        _this.resetFormView();
                        _this.getList();
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                }).finally(function () {
                    _this.submitLoading = false;
                });
            },
            resetFormView(formName) {
                //this.$refs[formName].resetFields();
                this.ruleFormView.id = null;
                this.ruleFormView.content = '';
                this.ruleFormView.reply = '';
                this.viewVisible = false;
            },
        },
    }
</script>
