<template>
    <!-- Member > Company > Verification & Review: Company verification review -->
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="tableSeachInpt">
                    <el-input v-model="searchForm.keyword" @keyup.enter.native="handleSearch" :placeholder="lc('admin_00340')" size="small" prefix-icon="el-icon-search"
                        clearable>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.status" size="small" slot="prepend" :placeholder="lc('wap_com_00406')" clearable @change="handleSearch">
                        <el-option :label="lc('wap_user_00166')" value="3"></el-option>
                        <el-option :label="lc('wap_user_00165')" value="1"></el-option>
                        <el-option :label="lc('wap_user_00167')" value="2"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.end" size="small" slot="prepend" :placeholder="lc('wap_com_00342')" clearable @change="handleSearch">
                        <el-option :label="lc('common_01940')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00179')" value="3"></el-option>
                        <el-option :label="lc('admin_user_00178')" value="7"></el-option>
                        <el-option :label="lc('admin_user_00180')" value="15"></el-option>
                        <el-option :label="lc('admin_user_00175')" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
        </div>
        <div class="admin_datatip"><i class="el-icon-document"></i> {{ lc("admin_data_stats") }} {{ lc("admin_total_count", [comCertAll]) }}
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [comCert1]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_failed_count", [comCert2]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }"
            style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 135px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable"
                @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" sortable="custom" width="90"></el-table-column>
                <el-table-column prop="name" :label="lc('wap_user_00080')"></el-table-column>
                <el-table-column :label="lc('admin_user_00188')">
                    <template slot-scope="scope">
                        <template v-if="scope.row.check">
                            <el-button type="primary" size="mini" plain @click="handleStatus(scope)">{{ lc('member_com_00325') }}</el-button>
                        </template>
                        <template v-else>
                            {{ lc('common_02082') }}
                        </template>
                    </template>
                </el-table-column>
                <el-table-column prop="ctime" :label="lc('wap_com_00342')" sortable="custom">
                    <template slot-scope="scope">{{ scope.row.ctime_n }}</template>
                </el-table-column>
                <el-table-column prop="status" :label="lc('member_user_00181')" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == 1" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                            <span v-else-if="scope.row.status == 0" class="admin_state4">{{ lc('wap_user_00166') }}</span>
                            <span v-else-if="scope.row.status == 2" class="admin_state2">{{ lc('wap_user_00167') }}</span>
                            <template v-else>--</template>
                            <!--<span class="admin_state1">Reviewed</span>-->
                            <!--<span class="admin_state2">Rejected</span>-->
                            <!--<span class="admin_state3">Locked</span>-->
                            <!--<span class="admin_state4">Pending review</span>-->
                            <!--<span class="admin_state5">Suspended</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="140">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="handleStatus(scope)">{{ lc('member_user_00152') }}</el-button>
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="handleStatus(null, true)" size="mini">{{ lc('admin_user_weipin_00037') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!-- Single review -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_company_00064')" :visible.sync="statusVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small">{{ lc('wap_com_00157') }}</div>
                    <el-input placeholder="" v-model="ruleFormStatus.name"></el-input>
                    <template v-if="com_social_credit">
                        <div class="wxsettip_small">{{ lc('admin_user_company_00063') }}</div>
                        <el-input placeholder="" v-model="info.social_credit" :disabled="true"></el-input>
                    </template>
                    <div class="wxsettip_small">{{ lc('admin_00460') }}</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image style="width: 80px; height: 80px" :src="info.check"
                                :preview-src-list="[info.check]"></el-image>
                            <div>{{ lc('admin_user_company_00065') }}</div>
                        </div>
                        <template v-if="com_cert_owner">
                            <div class="zzrz_imgpreview">
                                <el-image style="width: 80px; height: 80px" :src="info.owner_cert"
                                    :preview-src-list="[info.owner_cert]"></el-image>
                                <div>{{ lc('member_com_00067') }}</div>
                            </div>
                        </template>
                        <template v-if="com_cert_wt">
                            <div class="zzrz_imgpreview">
                                <el-image style="width: 80px; height: 80px" :src="info.wt_cert"
                                    :preview-src-list="[info.wt_cert]"></el-image>
                                <div>{{ lc('member_com_00062') }}</div>
                            </div>
                        </template>
                        <template v-if="com_cert_other">
                            <div class="zzrz_imgpreview">
                                <el-image style="width: 80px; height: 80px" :src="info.other_cert"
                                    :preview-src-list="[info.other_cert]"></el-image>
                                <div>{{ lc('member_com_00069') }}</div>
                            </div>
                        </template>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormStatus.status" label="1">{{ lc('admin_user_00149') }}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('admin_00633') }}</div>
                    <el-checkbox v-model="ruleFormStatus.job_status">{{ lc('admin_user_company_00062') }}</el-checkbox>
                    <div class="wxsettip_small ">{{ lc('member_user_00062') }}</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!-- Batch review -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_weipin_00037')" :visible.sync="statusAllVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormStatus.status" label="1">{{ lc('admin_user_00149') }}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('admin_00633') }}</div>
                    <el-checkbox v-model="ruleFormStatus.job_status">{{ lc('admin_user_company_00062') }}</el-checkbox>
                    <div class="wxsettip_small ">{{ lc('member_user_00062') }}</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        status: {type: String, default: ''}
    },
    data: function () {
        return {
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            searchForm: {
                page: 1,
                limit: null,
                keyword: null,
                status: this.status,
                end: null,
            },
            com_social_credit: null,
            com_cert_owner: null,
            com_cert_wt: null,
            com_cert_other: null,
            comCertAll: 0,
            comCert1: 0,// Unreviewed
            comCert2: 0,// Rejected
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,// Select all
            isIndeterminate: false,// Checkbox indeterminate state
            selectedItem: [],
            info: {
                name: '',
                social_credit: '',
                check: '',
                owner_cert: '',
                wt_cert: '',
                other_cert: '',
            },
            // Audit
            statusVisible: false,
            ruleFormStatus: {
                uid: null,
                name:'',
                status: null,// Review operation
                job_status: false,//{{ lc('admin_user_company_00325') }}
                statusbody: null,//{{ lc('member_user_00062') }}
            },
            // BatchAudit
            statusAllVisible: false,
            submitLoading: false,

            prevPage: 0
        }
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getConfigFun();
            that.getCertStatistFun();
        }, 200)
    },
    created() {
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
        getCertStatistFun:function(){
            let that = this;
            httpPost('m=user&c=company_cert&a=getCertStatist', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    
                    that.comCertAll = res.data.comCertAll;
                    that.comCert1 = res.data.comCert1;
                    that.comCert2 = res.data.comCert2;
                }
            })
        },
        getConfigFun:function(){
            let that = this;
            httpPost('m=user&c=company_cert&a=getConfigData', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.com_social_credit = res.data.com_social_credit;
                    that.com_cert_owner = res.data.com_cert_owner;
                    that.com_cert_wt = res.data.com_cert_wt;
                    that.com_cert_other = res.data.com_cert_other;
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
            httpPost('m=user&c=company_cert&a=index', params, {hideloading: true}).then(function (response) {
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
                    list.push(item.uid);
                }
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.uid = scope.row.uid;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_cert&a=del', params).then(function (response) {
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
                    message.error(lc('admin_user_weipin_00001'));
                    return false;
                }
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.uid);
                }
                this.ruleFormStatus.uid = list.join(',');
                this.ruleFormStatus.status = null;
                this.ruleFormStatus.statusbody = null;
                this.statusAllVisible = true;
            } else {
                this.info = scope.row
                this.ruleFormStatus.uid = scope.row.uid;
                this.ruleFormStatus.status = (scope.row.status == 1) ? scope.row.status : null;
                this.ruleFormStatus.name = scope.row.name;
                let _this = this;
                let params = {uid: scope.row.uid};
                httpPost('m=user&c=company_cert&a=sbody', params).then(function (response) {
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
            params.job_status = params.job_status ? 1 : 0;
            if (params.status == null) {
                message.error(lc('admin_user_weipin_00001'));
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_cert&a=status', params).then(function (response) {
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
            this.ruleFormStatus.uid = null;
            this.ruleFormStatus.status = null;
            this.ruleFormStatus.job_status = null;
            this.ruleFormStatus.statusbody = '';
            this.statusVisible = false;
            this.statusAllVisible = false;
        },
    },
};
</script>
<style scoped></style> 