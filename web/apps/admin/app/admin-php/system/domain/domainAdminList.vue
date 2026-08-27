<template>
<div id="domainAdminApp" class="moduleElenAl">
    <div class="moduleSeachs">
        <div class="moduleSeachInpt">
            <el-input :placeholder="lc('admin_system_00181')" v-model="search.keyword" class="input-with-select" size="small" clearable></el-input>
            <el-button type="primary" icon="el-icon-search" size="small" @click="handelSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
        </div>
        <div class="">
            <el-button type="primary" icon="el-icon-setting" size="small" plain @click="openGroupList">{{ lc('admin_system_00188') }}</el-button>
            <el-button type="primary" icon="el-icon-document-add" size="small" @click="addAdmin">{{ lc('admin_system_00238') }}</el-button>
        </div>
    </div>
    <div class="moduleElTable">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_system_00179')" type="info" :closable="false"></el-alert>
        </div>
        <el-table :data="tableData" border style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="calc(100% - 48px)" @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55"></el-table-column>
            <el-table-column prop="uid" :label="lc('common_02108')" width="80"></el-table-column>
            <el-table-column prop="username" :label="lc('admin_user_00140')"></el-table-column>
            <el-table-column prop="name" :label="lc('member_user_00230')"></el-table-column>
            <el-table-column prop="domain_name" :label="lc('admin_system_00174')"></el-table-column>
            <el-table-column prop="group_name" :label="lc('admin_system_00187')"></el-table-column>
            <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                <template #default="scope">
                    <div class="cz_button">
                        <a href="javascript:;" @click="editAdmin(scope);">
                            <el-button size="small">{{ lc('wap_js_00073') }}</el-button>
                        </a>
                        <el-button size="small" @click="delAdmin(scope)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                    </div>
                </template>
            </el-table-column>
        </el-table>
    </div>
    <div class="modulePaging">
        <div class="modulecz modulePagButn" style="margin-left: 10px;">
            <el-checkbox v-model="checkAll" @change="handleCheckAllChange">{{ lc('wap_js_00074') }}</el-checkbox>
            <el-button size="small" @click="delAdminSel">{{ lc('member_com_00055') }}</el-button>
        </div>
        <div class="modulePagNum">
            <div class="modulePagNum" style="margin: 0 auto;">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" v-model:current-page="currentPage" v-model:page-size="pageSize" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
            </div>
        </div>
    </div>

    <!-- 弹窗 -->
    <div class="modluDrawer">
        <el-dialog :title="addAdminTitle" v-model="addAdminShow" :with-header="true" :modal-append-to-body="false" :show-close="true" width="550px">
            <admin-add :admin_uid="adminUid" @child-event="closeAdminAdd"></admin-add>
        </el-dialog>
        <el-drawer :title="lc('admin_system_00185')" v-model="setshow" :with-header="true" :append-to-body="true" :show-close="true" size="80%">
            <group-list v-model:setshow="setshow" @child-event="closeGroupList"></group-list>
        </el-drawer>
    </div>
</div>
</template>

<script>
import AdminAdd from './component/adminAdd.vue'
import DomainAdminGroup from './component/domainAdminGroup.vue'

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
                search: {
                    keyword: null
                },

                addAdminShow: false,
                addAdminTitle: lc('admin_system_00186'),

                adminUid: 0,
                tableData: [],

                total: 0,
                currentPage: 1,
                prevPage: 0,
                pageSize: 0,
                pageSizes: [],

                // 批量选择
                checkAll: false,
                isIndeterminate: false,
                selectedItem: [],
                loading: true,
                setshow: false,
            }
        },
        components: {
            'admin-add': AdminAdd,
            'group-list': DomainAdminGroup,
        },
        created: function () {
            this.getDomainAdmin();


        },
        methods: {
            openGroupList() {
                this.setshow = true;
            },
            closeGroupList() {
                this.setshow = false;
                this.getDomainAdmin();
            },
            getDomainAdmin() {
                var that = this;
                var params = JSON.parse(JSON.stringify(this.search));
                params.pageSize = that.pageSize;
                params.page = that.currentPage;
                that.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=system&c=domain_group&a=adminList', params, { hideloading: true }).then(function (res) {
                    let data = res.data.data;
                    that.tableData = data.list;
                    that.total = data.total;
                    that.pageSize = parseInt(data.pageSize);
                    that.pageSizes = data.pageSizes;
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            handelSearch: function () {

                this.currentPage = 1
                this.getDomainAdmin()
            },
            handleSelectionChange(val) {
                this.selectedItem = val;
                if (this.selectedItem.length == 0) {
                    this.isIndeterminate = false;
                    this.checkAll = false;
                } else {
                    if (this.selectedItem.length == this.tableData.length) {
                        this.isIndeterminate = false;
                        this.checkAll = true;
                    } else {
                        this.isIndeterminate = true;
                        this.checkAll = false;
                    }
                }
            },
            handleCheckAllChange(val) {
                val ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
            },
            delAdmin(scope, isMore) {
                var that = this;
                let name = '';
                let idArr = [], nameArr = [];
                let params = {};
                if (isMore) {
                    this.selectedItem.forEach((item) => {

                        idArr.push(item.uid);
                        nameArr.push(item.name);
                    });
                    name = '（' + nameArr.join('，') + '）';
                    params.uid = idArr;
                } else {

                    name = '（' + scope.row.name + '）';
                    params.uid = scope.row.uid;
                }
                delConfirm(this, params, this.delete, lc('admin_system_00182') + name + lc('admin_01052'));
            },
            delAdminSel() {
                var that = this;
                if (!that.selectedItem.length) {
                    message.error(lc('admin_system_00180'));
                    return;
                }
                this.delAdmin(null, true);
            },
            delete(params) {
                var self = this;
                httpPost('m=system&c=domain_group&a=delAdmin', params).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            self.getDomainAdmin();
                        });
                    } else {
                        message.error(res.msg);
                    }
                }).catch(function (error) {
                    console.log(error);
                })
            },
            handleSizeChange(val) {

                console.log(`Page size: ${val}`);
                this.pageSize = val;
                this.getDomainAdmin();
            },
            handleCurrentChange(val) {

                console.log(`Current page: ${val}`);
                this.currentPage = val;
                this.getDomainAdmin();
            },
            addAdmin() {
                var self = this;
                self.adminUid = 0;
                self.addAdminTitle = lc('admin_system_00186');
                self.addAdminShow = true;
            },
            editAdmin(scope) {
                var self = this;
                self.adminUid = parseInt(scope.row.uid);
                self.addAdminTitle = lc('admin_system_00183');
                self.addAdminShow = true;
            },
            closeAdminAdd: function () {
                this.addAdminShow = false;
                this.handelSearch();
            }
        }
    }
</script>
