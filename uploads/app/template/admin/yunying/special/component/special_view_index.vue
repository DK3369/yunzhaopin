<template>
    <div class="jobspecial_box">
        <div class="moduleSeachbig">
            <div class="tableSeachInpt">
                <el-input v-model="searchForm.keyword" placeholder="企业名称" size="small" prefix-icon="el-icon-search" clearable>
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch">查询</el-button>
            </div>
            <div v-if="showAdd" class="tableSeachInpt">
                <el-button type="primary" plain icon="el-icon-plus" size="mini" @click="companyVisible = true">添加参会企业</el-button>
            </div>
        </div>

        <div class="admin_datatip"><i class="el-icon-document"></i> 参会企业共 {{ applyNum }}家 参会职位 {{ jobsNum }}</div>

        <div class="moduleElenAlRight">
            <div class="moduleElTable" style="margin-top: 0; padding: 0; height: calc(100% - 55px); width: 100%;">
                <el-table :data="tableData" border style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                    height="100%" ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="shortChange" v-loading="loading">
                    <template slot="empty">
                        <p>{{dataText}}</p>
                    </template>
                    <el-table-column type="selection" width="55">
                    </el-table-column>
                    <el-table-column prop="uid" label="编号" sortable="custom" width="100">
                    </el-table-column>
                    <el-table-column prop="name" label="企业名称" min-width="220">
                        <template slot-scope="scope">
                            <el-link :href="scope.row.comUrl" target="_blank" type="primary">{{ scope.row.name }}</el-link>
                        </template>
                    </el-table-column>
                    <el-table-column prop="sort" label="排序" sortable="custom" width="120">
                        <template slot-scope="scope">
                            <el-input v-if="scope.row[scope.column.property + 'isShow']" :ref="scope.column.property + scope.$index"
                                :id="scope.column.property + scope.$index" v-model="scope.row.sort" @blur="alterData(scope, 'int')"
                                onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                            <span v-else>
                {{ scope.row.sort }}<img @click="editData(scope)" class="editIcon" src="../../../admin/images/bine.png" alt=""
                                style="margin-left: 4px;" width="14" height="14">
              </span>
                        </template>
                    </el-table-column>
                    <el-table-column prop="tpl" label="状态" width="140">
                        <template slot-scope="scope">
                            <template v-if="scope.row.status == 1"><span style="color:#61687C;">已参加</span></template>
                            <template v-else-if="scope.row.status == 2"><span style="color:red;">未通过</span></template>
                            <template v-else>申请加入</template>
                        </template>
                    </el-table-column>
                    <el-table-column v-if="special.tpl == 'gl.htm'" prop="limit" label="名企展示" width="150">
                        <template slot-scope="scope">
                            <div class="cz_button">
                                <el-button v-if="scope.row.famous == 1" size="mini" @click="handleSpecialFamous(scope)">取消</el-button>
                                <el-button v-else size="mini" @click="handleSpecialFamous(scope)">设为名企</el-button>
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column label="操作" width="140" header-align="center" align="right">
                        <template slot-scope="scope">
                            <div class="cz_button">
                                <el-button size="mini" @click="handleAudit(scope)">加入</el-button>
                                <el-button type="danger" size="mini" @click="deleteRow(scope)">取消</el-button>
                            </div>
                        </template>
                    </el-table-column>
                </el-table>
            </div>
            <div class="modulePaging">
                <div>
                    <el-checkbox :indeterminate="isIndeterminate" v-model="checked" @change="selectAllBottom">全选</el-checkbox>
                    <el-button @click="deleteRow(null, true)" size="mini">批量删除</el-button>
                    <el-button @click="handleStatuscom" size="mini">批量审核</el-button>
                    <el-button plain icon="el-icon-download" @click.native.prevent="handleExport" size="mini">导出参会企业</el-button>
                </div>
                <div class="modulePagNum">
                    <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                        :current-page.sync="searchForm.page" :page-size="searchForm.limit" :page-sizes="pageSizes"
                        layout="total, sizes, prev, pager, next, jumper" :total="total" :pager-count="pagerCount">
                    </el-pagination>
                </div>
            </div>
        </div>

        <!-- 批量审核 弹窗 -->
        <div class="modluDrawer">
            <el-dialog :visible.sync="statuscomVisible" title="申请审核" width="400px" :modal-append-to-body="false"
                :append-to-body="true" :show-close="true" :destroy-on-close="true">
                <special_view_status :pid="statuscomPids" @child-event="statuscomVisible = false; getList();"
                    @child-event-close="statuscomVisible = false"></special_view_status>
            </el-dialog>
        </div>

        <!--企业详情审核 -->
        <el-drawer :visible.sync="auditVisible" title="企业详情" :append-to-body="true" :destroy-on-close="true"
            :modal-append-to-body="false" size="80%">
            <special_view_audit :id="info.id" :uid="info.uid" @child-event="auditVisible = false; getList();"></special_view_audit>
        </el-drawer>

        <!--添加参会企业-->
        <el-drawer :visible.sync="companyVisible" title="添加参会企业" :append-to-body="true" :destroy-on-close="true"
            :modal-append-to-body="false" size="90%">
            <special_view_company :id="id" @child-event="companyVisible = false; getList();"></special_view_company>
        </el-drawer>
    </div>
</template>

<script>
module.exports = {
    props: {
        //special_com.sid
        id: {type: [String, Number], default: 0},
    },
    data: function () {
        return {
            loading: false,
			pagerCount: 5,
            dataText: '数据加载中',
            pytoken: localStorage.getItem("pytoken"),
            searchForm: {
                page: 1,
                limit: null,
                keyword: null,//关键字
            },
            total: 0,
            tableData: [],
            pageSizes: [],
            checked: false,//全选
            isIndeterminate: false,// checkbox 的不确定状态
            selectedItem: [],
            showAdd: true,//是否显示 添加参会企业
            applyNum: 0,//参会企业的数量
            jobsNum: 0,//参会企业的职位数量
            special: {},//专题的部分信息
            statuscomVisible: false,//批量审核
            statuscomPids: '',
            auditVisible: false,//加入
            info: {},
            companyVisible: false,//添加参会企业
            prevPage:0
        }
    },
    created: function () {
    },
    mounted() {
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
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            params.id = this.id;
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            _this.loading = true;
            httpPost('m=yunying&c=special_special&a=com', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.tableData = res.data.list;
                    _this.total = res.data.total;
                    _this.searchForm.limit = res.data.perPage;
                    _this.pageSizes = res.data.pageSizes;
                    _this.showAdd = res.data.showAdd;
                    _this.applyNum = res.data.applyNum;
                    _this.jobsNum = res.data.jobsNum;
                    _this.special = res.data.special;
                    if(_this.prevPage != _this.searchForm.page){
                        _this.prevPage = _this.searchForm.page;
                        _this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    _this.loading = false;
                    if (_this.tableData.length === 0) {
                        _this.dataText = "暂无数据";
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
                params.id = scope.row.id;
            }

            delConfirm(this, params, this.delete);
        },
        delete(params) {
            let _this = this;
            httpPost('m=yunying&c=special_special&a=delcom', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.getList();
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        editData(scope) {
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            this.oldData = JSON.parse(JSON.stringify(row));
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = true;
            this.$set(this.tableData, index, copyRow);
            this.$nextTick(() => {
                let ref = column.property + index;
                $("#" + ref).focus();
            });
        },
        alterData(scope, type) {
            if (this.oldData == null) {
                return false;
            }
            let index = scope.$index;
            let row = scope.row;
            let column = scope.column;
            if (type === 'int') {
                row[column.property] = row[column.property].replace(/[^0-9]/g, '');
            }
            let copyRow = JSON.parse(JSON.stringify(row));
            copyRow[column.property + "isShow"] = false;
            this.$set(this.tableData, index, copyRow);
            if (row[column.property] === this.oldData[column.property]) {
                return false;
            }
            let _this = this;
            let params = {id: row.id};
            params[column.property] = row[column.property];
            httpPost('m=yunying&c=special_special&a=ajaxsort', params, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success('修改成功');
                } else {
                    message.error('修改失败');
                }
                _this.oldData = null;
                _this.getList();
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleAudit(scope) {
            this.info = scope.row;
            this.auditVisible = true;
        },
        handleStatuscom() {
            //批量审核
            if (!this.selectedItem.length) {
                message.error('您还未选择任何信息！');
                return false;
            }

            let list = [];
            for (let item of this.selectedItem) {
                list.push(item.id);
            }
            this.statuscomPids = list.join(',');
            this.statuscomVisible = true;
        },
        handleSpecialFamous(scope) {
            if (this.special.tpl != 'gl.htm') {
                return false;
            }
            let params = {};
            params.sid = scope.row.sid;
            params.uid = scope.row.uid;
            params.famous = scope.row.famous;
            let msg = '';
            if (scope.row.famous == 1) {
                msg = '确定要取消名企吗？';
            } else {
                msg = '确定要设为名企吗？'
            }
            delConfirm(this, params, this.doSpecialFamous, msg);
        },
        doSpecialFamous(params) {
            let _this = this;
            httpPost('m=yunying&c=special_special&a=setFamous', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.getList();
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleExport() {
            let params = {};
            if (!this.selectedItem.length) {
                delConfirm(this, params, this.export, '确定导出所有参会企业吗？');
                return false;
            } else {
                let list = [];
                for (let item of this.selectedItem) {
                    list.push(item.id);
                }
                params.cid = list.join(',');
                delConfirm(this, params, this.export, '确定导出选择的参会企业吗？');
                return false;
            }
        },
        export(params) {
            let _this = this;

            let formElement = document.createElement('form');
            document.body.appendChild(formElement);
            formElement.id = 'comxls';
            formElement.method = 'post';
            formElement.action = baseUrl + 'm=yunying&c=special_special&a=comxls';
            formElement._target = '_blank';

            let pytokenElement = document.createElement('input');
            pytokenElement.setAttribute('name', 'pytoken');
            pytokenElement.setAttribute('type', 'hidden');
            pytokenElement.setAttribute('value', this.pytoken);
            formElement.appendChild(pytokenElement);

            let zidElement = document.createElement('input');
            zidElement.setAttribute('name', 'zid');
            zidElement.setAttribute('type', 'hidden');
            zidElement.setAttribute('value', this.id);
            formElement.appendChild(zidElement);
            if (params.cid) {
                let cidElement = document.createElement('input');
                cidElement.setAttribute('name', 'cid');
                cidElement.setAttribute('type', 'hidden');
                cidElement.setAttribute('value', params.cid);
                formElement.appendChild(cidElement);
            }
            formElement.submit();
            formElement.remove();
            message.success('处理中...');
        }
    },
    components: {
        'special_view_audit': httpVueLoader('./special_view_audit.vue'),
        'special_view_company': httpVueLoader('./special_view_company.vue'),
        'special_view_status': httpVueLoader('./special_view_status.vue'),
    },
    watch: {
        id: {
            handler: function (newValue, oldValue) {
                if (newValue) {
                    this.getList();
                }
            },
            deep: true,
            immediate: true
        },
    }
};
</script>
<style scoped>
.drawerModInfo::-webkit-scrollbar {
    display: none;
}

.moduleElenAlRight {
    overflow: hidden;
    position: absolute;
    width: calc(100% - 40px);
    height: calc(100% - 86px);
    top: 82px;
    left: 0;
    z-index: 2;
    padding: 0 20px;
}</style>