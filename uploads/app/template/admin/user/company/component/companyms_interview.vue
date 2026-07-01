<template>
<div class="moduleElHight">
    <div class="moduleElSearchInf">
        <div class="moduleElTabInpt" style="flex-wrap: wrap;">
            <div class="moduleInptList">
                <el-input :placeholder="lc('admin_user_weipin_00003')" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                    <el-select v-model="searchForm.type" slot="prepend" :placeholder="lc('wap_user_00100')">
                        <el-option :label="lc('wap_user_00080')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00130')" value="2"></el-option>
                    </el-select>
                </el-input>
            </div>
            <div class="moduleInptList">
                <el-select v-model="searchForm.status" size="small" slot="prepend" :placeholder="lc('wap_com_00406')" clearable @change="handleSearch">
                    <el-option :label="lc('wap_user_00166')" value="0"></el-option>
                    <el-option :label="lc('wap_user_00165')" value="1"></el-option>
                    <el-option :label="lc('wap_user_00167')" value="2"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
    </div>
    <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
        <el-table :data="tableData" style="width: 100%" stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable" @selection-change="handleSelectionChange" v-loading="loading">
            <template slot="empty">
                <p>{{dataText}}</p>
            </template>
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="80"></el-table-column>
            <el-table-column prop="comname" :label="lc('wap_user_00080')" width="200" show-overflow-tooltip></el-table-column>
            <el-table-column :label="lc('admin_user_company_00056')" width="200">
                <template slot-scope="scope">
                    <span>{{scope.row.linkman}} - {{scope.row.linktel}}</span>
                </template>
            </el-table-column>
            <el-table-column prop="address" :label="lc('wap_01055')" min-width="180" show-overflow-tooltip></el-table-column>
            <el-table-column prop="intertime" :label="lc('wap_user_00255')" width="150"></el-table-column>
            <el-table-column prop="content" :label="lc('admin_user_00290')" min-width="200" show-overflow-tooltip></el-table-column>
            <el-table-column prop="addtime_n" :label="lc('member_com_00300')" width="150"></el-table-column>
            <el-table-column prop="status" :label="lc('member_user_00181')" fixed="right">
                <template slot-scope="scope">
                    <div class="admin_state">
                        <el-tag size="small" v-if="scope.row.status == '0'">{{ lc('wap_user_00166') }}</el-tag>
                        <el-tag size="small" type="success" v-else-if="scope.row.status == '1'">{{ lc('wap_user_00165') }}</el-tag>
                        <template v-else-if="scope.row.status == '2'">
                            <el-tooltip class="item" effect="dark" :content="scope.row.statusbody" placement="left">
                                <el-tag size="small" type="danger">{{ lc('wap_user_00167') }}</el-tag>
                            </el-tooltip>
                        </template>
                    </div>
                </template>
            </el-table-column>
            <el-table-column :label="lc('member_user_00048')" header-align="center" width="190" fixed="right">
                <template slot-scope="scope">
                    <div class="cz_button">
                        <el-button size="mini" @click="handleStatus(scope)">{{ lc('member_user_00152') }}</el-button>
                        <el-button size="mini" @click="editRow(scope)">{{ lc('wap_js_00073') }}</el-button>
                        <el-button type="danger" size="mini" @click="deleteRow(scope)">{{ lc('common.delete') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div>
            <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button @click="handleStatus(null, true)" size="mini">{{ lc('admin_user_weipin_00037') }}</el-button>
            <el-button @click="deleteRow(null, true)" size="mini">{{ lc('member_com_00055') }}</el-button>
        </div>
        <div class="modulePagNum">
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
        </div>
    </div>
    <!-- 审核 -->
    <div class="modluDrawer">
        <el-dialog :title="titleStatus" :visible.sync="statusVisible" :modal-append-to-body="false" width="400px">
            <div class="toolClasDia fenpeizhand">
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>{{ lc('admin_user_weipin_00065') }}</span>
                    </div>
                    <div class="toolClasCont">
                        <el-radio-group v-model="ruleFormStatus.status">
                            <el-radio label="1">{{ lc('admin_user_00149') }}</el-radio>
                            <el-radio label="2">{{ lc('wap_user_00167') }}</el-radio>
                        </el-radio-group>
                    </div>
                </div>
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>{{ lc('member_user_00450') }}</span>
                    </div>
                    <div class="toolClasCont">
                        <el-input type="textarea" :rows="2" v-model="ruleFormStatus.statusbody"></el-input>
                    </div>
                </div>
            </div>
            <span slot="footer" class="dialog-footer">
                <el-button @click="resetFormStatus('ruleFormStatus')">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
            </span>
        </el-dialog>
    </div>
    <!--修改-->
    <div class="modluDrawer">
        <el-dialog :title="lc('admin_00626')" :visible.sync="editVisible" :modal-append-to-body="false" width="500px">
            <div class="yunyinDialog">
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('wap_com_00413') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.name" :placeholder="lc('wap_user_00076')"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('admin_user_weipin_00027') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.linkman" :placeholder="lc('wap_user_00076')"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('wap_user_00265') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.linktel" :placeholder="lc('wap_user_00076')"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('wap_01055') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.address" :placeholder="lc('wap_user_00076')"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('wap_01426') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-date-picker v-model="ruleForm.intertime" type="datetime" :placeholder="lc('admin_user_company_00055')" value-format="yyyy-MM-dd HH:mm:ss"></el-date-picker>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>{{ lc('member_com_00353') }}</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input type="textarea" :rows="2" placeholder="" v-model="ruleForm.content"></el-input>
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
            loading: false,
            dataText: lc('admin_user_weipin_00026'),
            searchForm: {
                page: 1,
                limit: null,
                keyword: null,
                type: '1',
                status: this.status,
            },
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
                pid: null,
                status: null,
                statusbody: '',
            },
            titleStatus: lc('admin_company_00047'),
            // edit
            editVisible: false,//编辑
            ruleForm: {
                id: null,
                uid: null,
                name: '',
                linkman: '',
                linktel: '',
                address: '',
                intertime: '',
                content: '',
            },
            submitLoading: false,

            prevPage: 0
        }
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
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            _this.loading = true;
            httpPost('m=user&c=company_interview&a=index', params, {hideloading: true}).then(function (response) {
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
        editRow(scope) {
            this.ruleForm.id = scope.row.id;
            this.ruleForm.uid = scope.row.uid;
            this.ruleForm.name = scope.row.name;
            this.ruleForm.linkman = scope.row.linkman;
            this.ruleForm.linktel = scope.row.linktel;
            this.ruleForm.address = scope.row.address;
            this.ruleForm.intertime = scope.row.intertime;
            this.ruleForm.content = scope.row.content;
            this.editVisible = true;
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) { }});
            if (!this.ruleForm.name.length) {
                message.error(lc('wap_01221'));
                return false;
            }
            if (!this.ruleForm.linkman.length) {
                message.error(lc('member_com_00677'));
                return false;
            }
            if (!this.ruleForm.linktel.length) {
                message.error(lc('member_com_00678'));
                return false;
            }
            //TODO 验证手机号
            if (!this.ruleForm.address.length) {
                message.error(lc('member_com_00680'));
                return false;
            }
            if (!this.ruleForm.intertime.length) {
                message.error(lc('member_com_00681'));
                return false;
            }
            let _this = this;
            let params = this.ruleForm;
            _this.submitLoading = true;
            httpPost('m=user&c=company_interview&a=save', params).then(function (response) {
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
            this.ruleForm.uid = null;
            this.ruleForm.name = '';
            this.ruleForm.linkman = '';
            this.ruleForm.linktel = '';
            this.ruleForm.address = '';
            this.ruleForm.intertime = '';
            this.ruleForm.content = '';
            this.editVisible = false;
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
                params.del = list;
            } else {
                // let index = scope.$index;
                // this.tableData.splice(index, 1);
                params.del = scope.row.id;
            }
            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=user&c=company_interview&a=delYqmb', params).then(function (response) {
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
                this.titleStatus = lc('admin_user_weipin_00037');
            } else {
                this.ruleFormStatus.pid = scope.row.id;
                this.titleStatus = lc('admin_company_00047');
                let _this = this;
                if (parseInt(scope.row.status) > 0) {
                    _this.ruleFormStatus.status = scope.row.status;
                }
                _this.ruleFormStatus.statusbody = scope.row.statusbody;
            }
            this.statusVisible = true;
        },
        submitFormStatus(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = this.ruleFormStatus;
            if (params.status == null) {
                message.error(lc('admin_user_weipin_00015'));
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_interview&a=status', params).then(function (response) {
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
        }
    },
};
</script>
<style scoped>
    .moduleElHight .moduleElTable{padding:0;margin:0;height:calc(100% - 110px);width:100%}
    .moduleElTableHig{height:calc(100% - 90px)!important}
</style> 