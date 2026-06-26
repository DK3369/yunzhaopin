<template>
    <!--会员-个人-认证&审核：身份认证审核-->
    <div class="moduleElHight">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt">
                <el-input v-model="searchForm.keyword" @keyup.enter.native="handleSearch" placeholder="{yun:}t key='admin_00461'{/yun}" size="small" prefix-icon="el-icon-search" clearable>
                </el-input>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="searchForm.status" size="small" slot="prepend" placeholder="{yun:}t key='wap_com_00406'{/yun}" clearable @change="handleSearch">
                    <el-option label="{yun:}t key='wap_user_00165'{/yun}" value="1"></el-option>
                    <el-option label="{yun:}t key='wap_user_00166'{/yun}" value="2"></el-option>
                    <el-option label="{yun:}t key='wap_user_00167'{/yun}" value="3"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt tableSeachInptsmall">
                <el-select v-model="searchForm.time" size="small" slot="prepend" placeholder="{yun:}t key='admin_00462'{/yun}" clearable @change="handleSearch">
                    <el-option label="{yun:}t key='common_01940'{/yun}" value="1"></el-option>
                    <el-option label="{yun:}t key='admin_user_00179'{/yun}" value="3"></el-option>
                    <el-option label="{yun:}t key='admin_user_00178'{/yun}" value="7"></el-option>
                    <el-option label="{yun:}t key='admin_user_00180'{/yun}" value="15"></el-option>
                    <el-option label="{yun:}t key='admin_user_00175'{/yun}" value="30"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
            </div>
        </div>
        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }} {{ lc("admin_total_count", [numAll]) }}
            <span class="admin_datatip_n">{{ lc("admin_approved_count", [numAudited]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [numUnaudited]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_failed_count", [numFailed]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable" :class="{ 'modulElTableGai': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable"
                @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
                <template slot="empty">
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" label="UID" width="120" sortable="custom"></el-table-column>
                <el-table-column prop="username_n" label="{yun:}t key='wap_00529'{/yun}"></el-table-column>
                <el-table-column prop="idcard" label="{yun:}t key='wap_user_00173'{/yun}"></el-table-column>
                <el-table-column prop="idcard_pic" label="{yun:}t key='admin_user_00188'{/yun}">
                    <template slot-scope="scope">
                        <template v-if="scope.row.idcard_pic">
                            <el-button type="primary" size="mini" plain @click="handleStatus(scope)">{yun:}t key='member_com_00325'{/yun}</el-button>
                        </template>
                        <template v-else>
                            {yun:}t key='common_02082'{/yun}
                        </template>
                    </template>
                </el-table-column>
                <el-table-column prop="cert_time" label="{yun:}t key='wap_com_00342'{/yun}" sortable="custom">
                    <template slot-scope="scope">{{ scope.row.cert_time_n }}</template>
                </el-table-column>
                <el-table-column prop="idcard_status" label="{yun:}t key='member_user_00181'{/yun}" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.idcard_status == 1" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                            <span v-else-if="scope.row.idcard_status == 0" class="admin_state4">{yun:}t key='wap_user_00166'{/yun}</span>
                            <span v-else-if="scope.row.idcard_status == 2" class="admin_state2">{yun:}t key='wap_user_00167'{/yun}</span>
                            <template v-else>--</template>
                            <!--<span class="admin_state1">已审核</span>-->
                            <!--<span class="admin_state2">未通过</span>-->
                            <!--<span class="admin_state3">已锁定</span>-->
                            <!--<span class="admin_state4">待审核</span>-->
                            <!--<span class="admin_state5">已暂停</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="handleStatus(scope)">{yun:}t key='member_user_00152'{/yun}</el-button>
                            <el-button type="danger" size="mini" @click="deleteRow(scope)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
                <el-button @click="handleStatus(null, true)" size="mini">{yun:}t key='admin_user_weipin_00037'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!--审核弹出框-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00463'{/yun}" :visible.sync="statusVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_00529'{/yun}</div>
                    <el-input placeholder="" v-model="info.username_n" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00173'{/yun}</div>
                    <el-input placeholder="" v-model="info.idcard" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_00460'{/yun}</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image
                                style="width: 80px; height: 80px"
                                :src="info.idcard_pic"
                                :preview-src-list="[info.idcard_pic]">
                            </el-image>
                        </div>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
                    <el-radio v-model="ruleFormStatus.status" label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00062'{/yun}</div>
                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--批量审核-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_weipin_00037'{/yun}" :visible.sync="statusAllVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
                    <el-radio v-model="ruleFormStatus.status" label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00062'{/yun}</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
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
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
            searchForm: {
                page: 1,
                limit: null,
                keyword: null,
                status: this.status,
                time: null,
            },
            numAll: 0,
            numAudited: 0,
            numUnaudited: 0,//未审核
            numFailed: 0,//未通过
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            info: {
                username_n: '',
                idcard: '',
                idcard_pic: '",
            },
            // Audit
            statusVisible: false,
            ruleFormStatus: {
                uid: null,
                status: null,//操作审核
                statusbody: null,//{yun:}t key='member_user_00062'{/yun}
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
            that.getStatistFun();
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
            let orderMap = {ascending: "asc', descending: 'desc'}
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
            httpPost('m=user&c=users_usercert&a=getSfStatist', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.numAll = res.data.numAll;
                    that.numUnaudited = res.data.numUnaudited;
                    that.numAudited = res.data.numAudited;
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
            httpPost('m=user&c=users_usercert&a=index', params, {hideloading: true}).then(function (response) {
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
                        _this.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
                    message.error("{yun:}t key='admin_user_weipin_00005'{/yun}");
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
                params.id = scope.row.uid;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=users_usercert&a=del', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success("{yun:}t key='admin_user_00187'{/yun}");
                    _this.getList();
                } else {
                    message.error("{yun:}t key='admin_user_00186'{/yun}");
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleStatus(scope, isMore) {
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
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
                this.ruleFormStatus.status = (scope.row.idcard_status == 1) ? scope.row.idcard_status : null;
                let _this = this;
                let params = {uid: scope.row.uid};
                httpPost('m=user&c=users_usercert&a=sbody', params).then(function (response) {
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
                message.error("{yun:}t key='admin_user_weipin_00015'{/yun}");
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=users_usercert&a=status', params).then(function (response) {
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
            this.ruleFormStatus.statusbody = '';
            this.statusVisible = false;
            this.statusAllVisible = false;
        },
    },
};
</script>
<style scoped>


</style> 