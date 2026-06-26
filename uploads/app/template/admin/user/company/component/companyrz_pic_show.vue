<template>
    <!--会员-企业-认证&审核：企业环境审核-->
    <div class="moduleElHight">
        <div class="moduleElSearchInf">
            <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                <div class="moduleInptList" style="margin-bottom: 8px;">
                    <el-input placeholder="{yun:}t key='admin_user_weipin_00003'{/yun}" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                        <el-select v-model="searchForm.type" slot="prepend" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option label="{yun:}t key='wap_com_00157'{/yun}" value="1"></el-option>
                            <el-option label="{yun:}t key='admin_user_00130'{/yun}" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="searchForm.status" size="small" slot="prepend" placeholder="{yun:}t key='wap_com_00406'{/yun}" clearable @change="handleSearch">
                        <el-option label="{yun:}t key='wap_user_00165'{/yun}" value="0"></el-option>
                        <el-option label="{yun:}t key='wap_user_00166'{/yun}" value="1"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
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
                <el-table-column prop="uid" label="{yun:}t key='admin_user_00130'{/yun}" sortable="custom" width="90"></el-table-column>
                <el-table-column prop="wenjian" label="{yun:}t key='wap_user_00157'{/yun}" width="150">
                    <template slot-scope="scope">
                        <div class="demo-image__preview">
                            <el-image style="width: 100px; height: 60px" :src="scope.row.picurl"
                                :preview-src-list="[scope.row.picurl]"></el-image>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="name" label="{yun:}t key='wap_user_00080'{/yun}">
                </el-table-column>
                <el-table-column prop="zt" label="{yun:}t key='member_user_00181'{/yun}" width="100">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == '0'" class="admin_state1">{yun:}t key='wap_user_00165'{/yun}</span>
                            <span v-else-if="scope.row.status == '1'" class="admin_state4">{yun:}t key='wap_user_00166'{/yun}</span>
                            <span v-else-if="scope.row.status == '2'" class="admin_state2">{yun:}t key='wap_user_00167'{/yun}</span>
                            <template v-else>--</template>
                            <!--<span class="admin_state1">已审核</span>-->
                            <!--<span class="admin_state2">未通过</span>-->
                            <!--<span class="admin_state3">已锁定</span>-->
                            <!--<span class="admin_state4">待审核</span>-->
                            <!--<span class="admin_state5">已暂停</span>-->
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="200">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini" plain @click="handleStatus(scope)">{yun:}t key='member_user_00152'{/yun}</el-button>
                            <el-button size="mini" plain @click="editRow(scope)">{yun:}t key='wap_js_00073'{/yun}</el-button>
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
            <el-dialog :title="titleStatus" :visible.sync="statusVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small">{yun:}t key='admin_user_weipin_00032'{/yun}</div>
                    <el-radio v-model="ruleFormStatus.status" label="0">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                    <el-radio v-model="ruleFormStatus.status" label="2">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                    <div class="wxsettip_small ">{yun:}t key='member_user_00062'{/yun}</div>
                    <el-input type="textarea" :rows="2" placeholder="{yun:}t key='admin_00627'{/yun}" v-model="ruleFormStatus.statusbody"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetFormStatus('ruleFormStatus')">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!--修改弹出框-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_00632'{/yun}" :visible.sync="editVisible" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="450px">
                <div>
                    <div class="wxsettip_small">{yun:}t key='admin_00630'{/yun}</div>
                    <el-input placeholder="" v-model="ruleForm.name" :disabled="true"></el-input>
                    <div class="wxsettip_small ">{yun:}t key='admin_00631'{/yun}</div>
                    <el-input placeholder="" v-model="ruleForm.title"></el-input>
                    <div class="wxsettip_small">{yun:}t key='member_com_00022'{/yun}</div>
                    <el-input placeholder="" v-model="ruleForm.sort"
                        onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                    <div class="wxsettip_small">{yun:}t key='wap_00540'{/yun}</div>
                    <div class="zzrz_imgpreview" style="display: flex;align-items: center;">
                        <el-upload :action="uploadAction" :on-change="uploadChange" :accept="pic_accept"
                            :show-file-list="false">
                            <el-button size="small" type="primary">{yun:}t key='wap_00540'{/yun}</el-button>
                        </el-upload>
                        <div class="up_sy_logo_div" style="margin-left: 15px;">
                            <el-image v-if="ruleForm.picurl" style="width:100px;" :src="ruleForm.picurl"
                                :preview-src-list="ruleForm.picurl ? [ruleForm.picurl] : []"></el-image>
                        </div>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
                    <el-button @click="resetForm('ruleForm')">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                    <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
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
            dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
            searchForm: {
                page: 1,
                limit: null,
                keyword: null,
                type: '1",
                status: this.status,
            },
            numAll: 0,
            numAudited: 0,
            numUnaudited: 0,
            total: 0,
            tableData: [],
            pageSizes: [],
            tableHig: true,
            checked: false,//{yun:}t key='wap_js_00074'{/yun}
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            // Audit
            statusVisible: false,
            ruleFormStatus: {
                sid: null,
                status: null,
                statusbody: "',
            },
            titleStatus: "{yun:}t key='admin_user_company_00060'{/yun}",
            // Update
            info: {},
            editVisible: false,
            ruleForm: {
                name: null,
                title: null,
                sort: null,
                id: null,
                picurl: '',
                update: "{yun:}t key='wap_js_00073'{/yun}",
                type: 'show',
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
            that.getHjStatistFun();
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
        getHjStatistFun:function(){
            let that = this;
            httpPost('m=user&c=company_pic&a=getHjStatist', {},{hideloading: true}).then(function (response) {
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
            httpPost('m=user&c=company_pic&a=show', params, {hideloading: true}).then(function (response) {
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
                    list.push(item.id);
                }
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.delid = scope.row.id;
            }
            params.type = 'show';
            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_pic&a=del', params).then(function (response) {
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
                    list.push(item.id);
                }
                this.ruleFormStatus.sid = list.join(',');
                this.ruleFormStatus.statusbody = '';
                this.titleStatus = "{yun:}t key='admin_user_weipin_00037'{/yun}";
                this.statusVisible = true;
            } else {
                this.ruleFormStatus.sid = scope.row.id;
                this.titleStatus = "{yun:}t key='admin_user_company_00060'{/yun}";
                let _this = this;
                let params = {id: scope.row.id};
                httpPost('m=user&c=company_pic&a=getShowStatusBody', params).then(function (response) {
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
                message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_pic&a=showStatus', params).then(function (response) {
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
            this.ruleFormStatus.sid = null;
            this.ruleFormStatus.status = null;
            this.ruleFormStatus.statusbody = '';
            this.statusVisible = false;
        },
        editRow(scope) {
            this.ruleForm.id = scope.row.id;
            this.ruleForm.name = scope.row.name;
            this.ruleForm.title = scope.row.title;
            this.ruleForm.sort = scope.row.sort;
            this.ruleForm.picurl = scope.row.picurl;
            this.file = [];
            this.editVisible = true;
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            if (Array.isArray(this.file) && this.file.length < 1) {
                message.error("{yun:}t key='wap_01412'{/yun}");
                return false;
            }
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            delete params.name;
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
            this.ruleForm.id = null;
            this.ruleForm.name = null;
            this.ruleForm.title = null;
            this.ruleForm.sort = null;
            this.ruleForm.picurl = '';
            this.file = [];
            this.editVisible = false;
        },
    },
};
</script>
<style scoped></style> 