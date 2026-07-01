<template>
    <!--会员-企业-认证&审核：企业LOGO审核-->
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList" style="margin-bottom: 8px;">
                    <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <el-select v-model="searchForm.type" slot="prepend" :placeholder="lc('wap_user_00100')">
                            <el-option :label="lc('wap_com_00157')" value="1"></el-option>
                            <el-option :label="lc('admin_user_00130')" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.status" size="small" slot="prepend" :placeholder="lc('wap_com_00406')" clearable @change="handleSearch">
                        <el-option :label="lc('wap_user_00165')" value="0"></el-option>
                        <el-option :label="lc('wap_user_00166')" value="1"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
        </div>
        <div class="admin_datatip"><i class="el-icon-document"></i> {{ lc("admin_data_stats") }} {{ lc("admin_total_count", [numAll]) }}
            <span class="admin_datatip_n">{{ lc("admin_approved_count", [numAudited]) }} </span>
            <span class="admin_datatip_n">{{ lc("admin_pending_review_count", [numUnaudited]) }}</span>
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
                <el-table-column prop="logo" :label="lc('wap_com_00149')" width="150">
                    <template slot-scope="scope">
                        <div class="demo-image__preview">
                            <el-image style="width: 60px; height: 60px" :src="scope.row.logo"
                                :preview-src-list="[scope.row.logo]"></el-image>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="name" :label="lc('wap_user_00080')">
                </el-table-column>
                <el-table-column prop="logo_status" :label="lc('member_user_00181')" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.logo_status == '0'" class="admin_state1">{{ lc('wap_user_00165') }}</span>
                            <span v-else-if="scope.row.logo_status == '1'" class="admin_state4">{{ lc('wap_user_00166') }}</span>
                            <span v-else-if="scope.row.logo_status == '2'" class="admin_state2">{{ lc('wap_user_00167') }}</span>
                            <template v-else>--</template>
                            <!--<span class="admin_state1">已审核</span>-->
                            <!--<span class="admin_state2">未通过</span>-->
                            <!--<span class="admin_state3">已锁定</span>-->
                            <!--<span class="admin_state4">待审核</span>-->
                            <!--<span class="admin_state5">已暂停</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="200">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="handleStatus(scope)">{{ lc('member_user_00152') }}</el-button>
                            <el-button size="mini" plain @click="editRow(scope)">{{ lc('wap_js_00073') }}</el-button>
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
        <!--审核弹出框-->
        <div class="modluDrawer">
            <el-dialog :title="titleStatus" :visible.sync="statusVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small">{{ lc('admin_user_weipin_00032') }}</div>
                    <el-radio v-model="ruleFormStatus.status" label="0">{{ lc('admin_user_00149') }}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{{ lc('wap_user_00167') }}</el-radio>
                    <div class="wxsettip_small">{{ lc('member_user_00062') }}</div>
                    <el-input type="textarea" :rows="2" :placeholder="lc('admin_00627')" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <el-alert :title="lc('admin_00458')" type="info" show-icon :closable="false"
                    style="background: unset; padding-left: 0;"></el-alert>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--修改弹出框-->
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00629')" :visible.sync="editVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small ">{{ lc('wap_com_00157') }}</div>
                    <el-input placeholder="" v-model="info.name" :disabled="true"></el-input>
                    <div class="wxsettip_small">{{ lc('wap_com_00149') }}</div>
                    <div class="zzrz_imgpreview" style="display: flex;align-items: center;">
                        <el-upload :action="uploadAction" :on-change="uploadChange" :accept="pic_accept"
                            :show-file-list="false">
                            <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                        </el-upload>
                        <div class="up_sy_logo_div" style="margin-left: 15px;">
                            <el-image v-if="ruleForm.picurl" style="width:100px;" :src="ruleForm.picurl"
                                :preview-src-list="ruleForm.picurl ? [ruleForm.picurl] : []"></el-image>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetForm('ruleForm')">{{ lc('admin_user_weipin_00043') }}</el-button>
                    <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
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
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            searchForm: {
                page: 1,
                limit: null,
                keyword: null,
                type: '1',
                status: this.status,
            },
            numAll: 0,
            numAudited: 0,
            numUnaudited: 0,
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//{{ lc('wap_js_00074') }}
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            // Audit
            statusVisible: false,
            ruleFormStatus: {
                uid: null,
                status: null,
                statusbody: '',
            },
            titleStatus: lc('admin_user_company_00059'),
            // Update
            info: {},
            editVisible: false,
            ruleForm: {
                id: null,
                picurl: '',
                update: lc('wap_js_00073'),
                type: 'logo',
            },
            file: [],//暂存文件
            submitLoading: false,
            uploadAction: baseUrl + 'm=common&c=common_upload',

            prevPage: 0
        }
    },
    mounted() {
        var that = this
        setTimeout(function () {
            that.getLogoStatistFun();
        }, 200)
    },
    created() {
        this.getList();
    },
    methods: {
        uploadChange(file) {
            this.ruleForm.picurl = URL.createObjectURL(file.raw);
            // 复刻文件信息
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
        getLogoStatistFun:function(){
            let that = this;
            httpPost('m=user&c=company_pic&a=getLogoStatist', {},{hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    
                    that.numAll = res.data.numAll;
                    that.numAudited = res.data.numAudited;
                    that.numUnaudited = res.data.numUnaudited;
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
            httpPost('m=user&c=company_pic&a=index', params, {hideloading: true}).then(function (response) {
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
                params.delid = scope.row.uid;
            }
            params.type = 'logo';
            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_pic&a=del', params).then(function (response) {
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
                this.titleStatus = lc('admin_user_weipin_00037');
                this.statusVisible = true;
            } else {
                this.ruleFormStatus.uid = scope.row.uid;
                this.titleStatus = lc('admin_user_company_00059');
                let _this = this;
                let params = {uid: scope.row.uid};
                httpPost('m=user&c=company_pic&a=getStatusBody', params).then(function (response) {
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
            let params = this.ruleFormStatus;
            if (params.status == null) {
                message.error(lc('admin_user_weipin_00001'));
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_pic&a=status', params).then(function (response) {
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
        },
        editRow(scope) {
            this.info = scope.row;
            this.ruleForm.id = scope.row.uid;
            this.ruleForm.picurl = scope.row.logo;
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
            httpPost('m=user&c=company_pic&a=uploadsave', formData).then(function (response) {
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
            this.info = {};
            this.ruleForm.id = null;
            this.ruleForm.picurl = '';
            this.file = [];
            this.editVisible = false;
        },
    },
};
</script>
<style scoped></style> 