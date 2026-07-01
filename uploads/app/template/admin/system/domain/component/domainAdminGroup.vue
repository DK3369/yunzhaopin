<template>
    <div class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
                <el-input :placeholder="lc('admin_01036')" v-model="search.keyword" class="input-with-select" size="small"
                    clearable></el-input>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handelSearch">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
            <div class="">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addGroup">{{ lc('admin_01035') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 110px); padding: 0 12px; margin-top: 0;">
            <div class="tableDome_tip">
                <el-alert :title="lc('admin_01037')" type="info"
                    :closable="false"></el-alert>
            </div>
            <el-table :data="tableData" border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="calc(100% - 50px)"
                @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('common_02108')" width="80"></el-table-column>
                <el-table-column prop="group_name" :label="lc('admin_system_00212')"></el-table-column>
                <el-table-column prop="domain_name" :label="lc('admin_system_00174')"></el-table-column>
                <el-table-column prop="num" :label="lc('admin_system_00173')"></el-table-column>
                <el-table-column :label="lc('member_user_00048')" width="140">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <!-- <a href="javascript:;" @click="editGroup(scope);">
                                <el-button @click="editGroup(scope);" size="small">修改</el-button>
                            </a> -->
                            <el-button @click="editGroup(scope);" size="mini">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="mini" @click="delGroup(scope)" type="danger">{{ lc('common.delete') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz" style="margin-left: 10px;">
                <el-checkbox v-model="checkAll" @change="handleCheckAllChange">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button size="mini" @click="delGroupSel">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <div class="modulePagNum" style="margin: 0 auto;">
                    <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page.sync="currentPage" :page-size.sync="pageSize" :page-sizes="pageSizes" layout="total, sizes, prev, pager, next, jumper" :total="total"></el-pagination>
                </div>
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="addGroupTitle" :visible.sync="addGroupShow" :with-header="true" :append-to-body="true" :show-close="true" size="60%;">
                <group-add :group_id="groupId" @child-event="closeGroupAdd"></group-add>
            </el-drawer>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        setshow: Boolean
    },
    data: function () {
        return {
            emptytext: lc('wap_js_00113'),
            search: {
                keyword: null
            },

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

            groupId: 0,
            addGroupTitle: '',
            addGroupShow: false,
            loading: true,
        }
    },
    watch: {
        setshow: {
            handler(val) {
                if (val == true) {
                    this.getAdminGroup();
                }
            },
            immediate: true,
            deep: true,
        },
    },
    components: {
        'group-add': httpVueLoader('./adminGroup.vue'),
    },
    methods: {
        getAdminGroup() {
            var that = this;
            var params = JSON.parse(JSON.stringify(this.search));
            params.pageSize = that.pageSize;
            params.page = that.currentPage;
            that.loading = true;
            that.emptytext = lc('admin_user_weipin_00026');
            httpPost('m=system&c=domain_group&a=groupList', params).then(function (res) {
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
        handelSearch() {
            this.currentPage = 1
            this.getAdminGroup()
        },
        addGroup: function () {
            var self = this;
            self.groupId = 0;
            self.addGroupTitle = lc('admin_01038');
            self.addGroupShow = true;
        },
        editGroup(scope) {
            var self = this;
            self.groupId = parseInt(scope.row.id);
            self.addGroupTitle = lc('admin_01039');
            self.addGroupShow = true;
        },
        closeGroupAdd: function () {
            this.addGroupShow = false;
            this.getAdminGroup();
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
        delGroup(scope, isMore) {
            var that = this;
            let name = '';
            let idArr = [], nameArr = [];
            let params = {};
            if (isMore) {
                this.selectedItem.forEach((item) => {

                    idArr.push(item.id);
                    nameArr.push(item.group_name);
                });
                name = '（' + nameArr.join('，') + '）';
                params.id = idArr;
            } else {

                name = '（' + scope.row.group_name + '）';
                params.id = scope.row.id;
            }
            delConfirm(this, params, this.delete, lc('admin_system_00172') + name + lc('admin_vue_00131'));
        },
        delGroupSel() {
            var that = this;
            if (!that.selectedItem.length) {
                message.error(lc('admin_vue_00048'));
                return;
            }
            this.delAdmin(null, true);
        },
        delete(params) {
            var self = this;
            httpPost('m=system&c=domain_group&a=delGroup', params).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        self.getAdminGroup();
                    });
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
        handleSizeChange(val) {
            console.log(`每页 ${val} 条`);
            this.pageSize = val;
            this.getAdminGroup();
        },
        handleCurrentChange(val) {
            console.log(`当前页: ${val}`);
            this.currentPage = val;
            this.getAdminGroup();
        }
    }
};
</script>
