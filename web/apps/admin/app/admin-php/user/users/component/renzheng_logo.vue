<template>
    <!-- Member > Individual > Verification & Review: Avatar review -->
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter="handleSearch" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <template #prepend><el-select v-model="searchForm.type" :placeholder="lc('wap_user_00100')">
                            <el-option :label="lc('admin_user_00185')" value="1"></el-option>
                            <el-option :label="lc('admin_user_00130')" value="2"></el-option>
                        </el-select></template>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.status" size="small" :placeholder="lc('wap_com_00406')" clearable @change="handleSearch">
                        <el-option :label="lc('wap_user_00165')" value="0"></el-option>
                        <el-option :label="lc('wap_user_00166')" value="1"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
        </div>

        <div class="admin_datatip">
            <i class="el-icon-document"></i> {{ lc("admin_data_stats") }} {{ lc("admin_total_count", [numAll]) }}
            <span class="admin_datatip_n">{{ lc("admin_approved_count", [numAudited]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [numUnaudited]) }}</span>
            <span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
        </div>
        <div class="moduleElTable" :class="{ 'modulElTableGai': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
            <el-table :data="tableData" style="width: 100%" stripe
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable"
                @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="uid" :label="lc('admin_user_00130')" width="120" sortable="custom"></el-table-column>
                <el-table-column prop="photo" :label="lc('member_user_00161')" width="150">
                    <template #default="scope">
                        <div class="demo-image__preview">
                            <el-image style="width: 60px; height: 60px" :src="scope.row.photo" :preview-src-list="[scope.row.photo]">
                            </el-image>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="username_n" :label="lc('wap_00529')">
                </el-table-column>
                <el-table-column prop="username" :label="lc('admin_user_00140')">
                </el-table-column>
                <el-table-column prop="photo_status" :label="lc('member_user_00181')" width="100">
                    <template #default="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.photo_status == 0" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                            <span v-else-if="scope.row.photo_status == 1" class="admin_state4">{{ lc('wap_user_00166') }}</span>
                            <span v-else-if="scope.row.photo_status == 2" class="admin_state2">{{ lc('wap_user_00167') }}</span>
                            <template v-else>--</template>
                            <!--<span class="admin_state1">Reviewed</span>-->
                            <!--<span class="admin_state2">Rejected</span>-->
                            <!--<span class="admin_state3">Locked</span>-->
                            <!--<span class="admin_state4">Pending review</span>-->
                            <!--<span class="admin_state5">Suspended</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="200" fixed="right">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" plain @click="handleStatus(scope)">{{ lc('member_user_00152') }}</el-button>
                            <el-button size="small" plain @click="editRow(scope)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button type="danger" size="small" @click="deleteRow(scope)">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="small">{{ lc('member_com_00055') }}</el-button>
                <el-button @click="handleStatus(null, true)" size="small">{{ lc('admin_user_weipin_00037') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    v-model:current-page="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!-- Review dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_00183')" v-model="statusVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_00529') }}</div>
                    <el-input placeholder="" v-model="info.username_n" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{{ lc('admin_00457') }}</div>
                    <div class="zzrz_img">
                        <div class="zzrz_imgpreview">
                            <el-image
                                style="width: 80px; height: 80px"
                                :src="info.photo"
                                :preview-src-list="[info.photo]">
                            </el-image>
                        </div>
                    </div>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormStatus.status" label="0">{{ lc('admin_user_00149') }}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('member_user_00062') }}</div>
                    <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="ruleFormStatus.statusbody"></el-input>
                    <el-alert :title="lc('admin_00458')" type="info" show-icon :closable="false"
                        style="background: unset; padding-left: 0;"></el-alert>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Batch review -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_user_weipin_00037')" v-model="statusAllVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormStatus.status" label="0">{{ lc('admin_user_00149') }}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                    <div class="wxsettip_small ">{{ lc('member_user_00062') }}</div>
                    <el-input type="textarea" :rows="2" placeholder="" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span></template>
            </el-dialog>
        </div>
        <!-- Edit dialog -->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00459')" v-model="editVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div class="wxsettip_small ">{{ lc('wap_00529') }}</div>
                <el-input placeholder="" v-model="info.username_n" :disabled="true"></el-input>
                <div class="wxsettip_small ">{{ lc('admin_00457') }}</div>
                <div class="zzrz_img">
                    <div class="zzrz_imgpreview" style="display: flex;align-items: center;">
                        <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChange"
                            :show-file-list="false">
                            <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                        </el-upload>
                        <div class="up_sy_logo_div" style="margin-left: 15px;">
                            <el-image v-if="ruleForm.picurl" style="width:100px;" :src="ruleForm.picurl"
                                :preview-src-list="ruleForm.picurl ? [ruleForm.picurl] : []"></el-image>
                        </div>
                    </div>
                </div>
                <template #footer><span class="dialog-footer">
                    <el-button @click="resetForm('ruleForm')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
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
    props: {
        status: {type: String, default: ''}
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            searchForm: {
                page: 1,
                limit: null,
                type: "1",
                keyword: null,
                status: this.status,
            },
            numAll: 0,
            numAudited: 0,// Reviewed
            numUnaudited: 0,// Unreviewed
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,// Select all
            isIndeterminate: false,// Checkbox indeterminate state
            selectedItem: [],
            info: {
                username_n: '',
                photo: '',
            },
            // Audit
            statusVisible: false,
            ruleFormStatus: {
                uid: null,
                status: null,// Review operation
                statusbody: null,//{{ lc('member_user_00062') }}
            },
            // BatchAudit
            statusAllVisible: false,
            // Update
            editVisible: false,
            ruleForm: {
                id: null,
                picurl: '',
            },
            file: [],// Temporary files
            submitLoading: false,
            uploadAction: baseUrl + 'm=common&c=common_upload',

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
        uploadChange(file) {
            this.ruleForm.picurl = URL.createObjectURL(file.raw);
            // Clone file metadata
            this.file = file.raw;
        },
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
            httpPost('m=user&c=users_pic&a=getStatist', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    that.numAll = res.data.numAll;
                    that.numUnaudited = res.data.numUnaudited;
                    that.numAudited = res.data.numAudited;
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
            httpPost('m=user&c=users_pic&a=index', params, {hideloading: true}).then(function (response) {
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
                params.del = list.join(',');
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.id = scope.row.uid;
            }
            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=users_pic&a=delPhoto', params).then(function (response) {
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
                this.ruleFormStatus.statusbody = '';
                this.statusAllVisible = true;
            } else {
                this.info = scope.row;
                this.ruleFormStatus.uid = scope.row.uid;
                this.ruleFormStatus.status = scope.row.photo_status == 0 ? scope.row.photo_status : null;
                let _this = this;
                let params = {uid: scope.row.uid};
                httpPost('m=user&c=users_pic&a=getStatusBody', params).then(function (response) {
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
            httpPost('m=user&c=users_pic&a=status', params).then(function (response) {
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
        editRow(scope) {
            this.info = scope.row;
            this.ruleForm.id = scope.row.uid;
            this.ruleForm.picurl = scope.row.photo;
            this.file = [];
            this.editVisible = true;
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            if (Array.isArray(this.file) && this.file.length < 1) {
                message.error(lc('wap_01412'));
                return false;
            }
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            delete params.picurl;
            let formData = new FormData();
            Object.keys(params).forEach((key) => {
                if (Array.isArray(params[key])) {
                    params[key].forEach((v) => {
                        formData.append(key + '[]', v);
                    });
                } else {
                    formData.append(key, params[key]);
                }
            });
            if (this.file.length !== 0) {
                formData.append('file', this.file);
            }
            _this.submitLoading = true;
            httpPost('m=user&c=users_pic&a=savePhoto', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.resetForm();
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
        resetForm(formName) {
            //this.$refs[formName].resetFields();
            this.ruleForm.id = null;
            this.ruleForm.picurl = '';
            this.file = [];
            this.editVisible = false;
        },
    },
};
</script>
<style scoped>


</style> 