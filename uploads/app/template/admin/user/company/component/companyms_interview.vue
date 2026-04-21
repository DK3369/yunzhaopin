<template>
<div class="moduleElHight">
    <div class="moduleElSearchInf">
        <div class="moduleElTabInpt" style="flex-wrap: wrap;">
            <div class="moduleInptList">
                <el-input placeholder="输入你要搜索的关键字" @keyup.enter.native="handleSearch" size="small" v-model="searchForm.keyword" class="input-with-select" clearable>
                    <el-select v-model="searchForm.type" slot="prepend" placeholder="请选择">
                        <el-option label="公司名称" value="1"></el-option>
                        <el-option label="用户ID" value="2"></el-option>
                    </el-select>
                </el-input>
            </div>
            <div class="moduleInptList">
                <el-select v-model="searchForm.status" size="small" slot="prepend" placeholder="审核状态" clearable @change="handleSearch">
                    <el-option label="未审核" value="0"></el-option>
                    <el-option label="已审核" value="1"></el-option>
                    <el-option label="未通过" value="2"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">查询</el-button>
            </div>
        </div>
    </div>
    <div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
        <el-table :data="tableData" style="width: 100%" stripe :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" ref="multipleTable" @selection-change="handleSelectionChange" v-loading="loading">
            <template slot="empty">
                <p>{{dataText}}</p>
            </template>
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="id" label="编号" width="80"></el-table-column>
            <el-table-column prop="comname" label="公司名称" width="200" show-overflow-tooltip></el-table-column>
            <el-table-column label="联系信息" width="200">
                <template slot-scope="scope">
                    <span>{{scope.row.linkman}} - {{scope.row.linktel}}</span>
                </template>
            </el-table-column>
            <el-table-column prop="address" label="面试地址" min-width="180" show-overflow-tooltip></el-table-column>
            <el-table-column prop="intertime" label="面试时间" width="150"></el-table-column>
            <el-table-column prop="content" label="备注说明" min-width="200" show-overflow-tooltip></el-table-column>
            <el-table-column prop="addtime_n" label="添加时间" width="150"></el-table-column>
            <el-table-column prop="status" label="状态" fixed="right">
                <template slot-scope="scope">
                    <div class="admin_state">
                        <el-tag size="small" v-if="scope.row.status == '0'">未审核</el-tag>
                        <el-tag size="small" type="success" v-else-if="scope.row.status == '1'">已审核</el-tag>
                        <template v-else-if="scope.row.status == '2'">
                            <el-tooltip class="item" effect="dark" :content="scope.row.statusbody" placement="left">
                                <el-tag size="small" type="danger">未通过</el-tag>
                            </el-tooltip>
                        </template>
                    </div>
                </template>
            </el-table-column>
            <el-table-column label="操作" header-align="center" width="190" fixed="right">
                <template slot-scope="scope">
                    <div class="cz_button">
                        <el-button size="mini" @click="handleStatus(scope)">审核</el-button>
                        <el-button size="mini" @click="editRow(scope)">修改</el-button>
                        <el-button type="danger" size="mini" @click="deleteRow(scope)">删除</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div>
            <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
            <el-button @click="handleStatus(null, true)" size="mini">批量审核</el-button>
            <el-button @click="deleteRow(null, true)" size="mini">批量删除</el-button>
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
                        <span>审核操作：</span>
                    </div>
                    <div class="toolClasCont">
                        <el-radio-group v-model="ruleFormStatus.status">
                            <el-radio label="1">正常</el-radio>
                            <el-radio label="2">未通过</el-radio>
                        </el-radio-group>
                    </div>
                </div>
                <div class="toolClasList">
                    <div class="toolClasTite">
                        <span>审核说明：</span>
                    </div>
                    <div class="toolClasCont">
                        <el-input type="textarea" :rows="2" v-model="ruleFormStatus.statusbody"></el-input>
                    </div>
                </div>
            </div>
            <span slot="footer" class="dialog-footer">
                <el-button @click="resetFormStatus('ruleFormStatus')">取 消</el-button>
                <el-button type="primary" @click="submitFormStatus('ruleFormStatus')" :disabled="submitLoading">确 定</el-button>
            </span>
        </el-dialog>
    </div>
    <!--修改-->
    <div class="modluDrawer">
        <el-dialog title="面试模板管理" :visible.sync="editVisible" :modal-append-to-body="false" width="500px">
            <div class="yunyinDialog">
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>模板名称</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.name" placeholder="请输入内容"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>联 系 人</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.linkman" placeholder="请输入内容"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>联系电话</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.linktel" placeholder="请输入内容"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>面试地址</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input v-model="ruleForm.address" placeholder="请输入内容"></el-input>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>面试时间</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-date-picker v-model="ruleForm.intertime" type="datetime" placeholder="选择日期时间" value-format="yyyy-MM-dd HH:mm:ss"></el-date-picker>
                    </div>
                </div>
                <div class="yunyinDiaList">
                    <div class="yunyinDiaTite">
                        <span>面试备注</span>
                    </div>
                    <div class="yunyinDiaInpt">
                        <el-input type="textarea" :rows="2" placeholder="" v-model="ruleForm.content"></el-input>
                    </div>
                </div>
            </div>
            <span slot="footer" class="dialog-footer">
                <el-button @click="resetForm('ruleForm')">取 消</el-button>
                <el-button type="primary" @click="submitForm('ruleForm')" :disabled="submitLoading">确 定</el-button>
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
            dataText: '数据加载中',
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
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            //审核
            statusVisible: false,
            ruleFormStatus: {
                pid: null,
                status: null,
                statusbody: '',
            },
            titleStatus: '邀请模板审核',
            //编辑
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
                        _this.dataText = "暂无数据";
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
                message.error('模板名称不能为空！');
                return false;
            }
            if (!this.ruleForm.linkman.length) {
                message.error('联系人不能为空！');
                return false;
            }
            if (!this.ruleForm.linktel.length) {
                message.error('联系电话不能为空！');
                return false;
            }
            //TODO 验证手机号
            if (!this.ruleForm.address.length) {
                message.error('面试地址不能为空！');
                return false;
            }
            if (!this.ruleForm.intertime.length) {
                message.error('面试时间不能为空！');
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
                    message.error('请选择要删除的数据');
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
                    message.success('删除成功！');
                    _this.getList();
                } else {
                    message.error('删除失败！');
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleStatus(scope, isMore) {
            if (isMore) {
                if (!this.selectedItem.length) {
                    message.error('您还未选择任何信息！');
                    return false;
                }
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.id);
                }
                this.ruleFormStatus.pid = list.join(',');
                this.titleStatus = '批量审核';
            } else {
                this.ruleFormStatus.pid = scope.row.id;
                this.titleStatus = '邀请模板审核';
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
                message.error('请选择审核状态');
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