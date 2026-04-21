<template>
    <div class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="moduleSeachInpt">
                <el-input placeholder="请输入用户组名称" v-model="search.keyword" class="input-with-select" size="small"
                    clearable></el-input>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handelSearch">查询</el-button>
            </div>
            <div class="">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addGroup">添加管理员权限</el-button>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 110px); padding: 0 12px; margin-top: 0;">
            <div class="tableDome_tip">
                <el-alert title="分站管理权限列表：主要展示不同分站权限组，以及管理员人数；超级管理员可以根据不通运营需求进行添加、调整" type="info"
                    :closable="false"></el-alert>
            </div>
            <el-table :data="tableData" border style="width: 100%"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="calc(100% - 50px)"
                @selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" label="	编号" width="80"></el-table-column>
                <el-table-column prop="group_name" label="用户组名称"></el-table-column>
                <el-table-column prop="domain_name" label="所属分站"></el-table-column>
                <el-table-column prop="num" label="管理员人数"></el-table-column>
                <el-table-column label="操作" width="140">
                    <template slot-scope="scope">
                        <div class="moduleElTaCaoz">
                            <!-- <a href="javascript:;" @click="editGroup(scope);">
                                <el-button @click="editGroup(scope);" size="small">修改</el-button>
                            </a> -->
                            <el-button @click="editGroup(scope);" size="mini">修改</el-button>
                            <el-button size="mini" @click="delGroup(scope)" type="danger">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz" style="margin-left: 10px;">
                <el-checkbox v-model="checkAll" @change="handleCheckAllChange">全选</el-checkbox>
                <el-button size="mini" @click="delGroupSel">批量删除</el-button>
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
            emptytext: '暂无数据',
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
            that.emptytext = "数据加载中";
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
                    that.emptytext = "暂无数据";
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
            self.addGroupTitle = '添加分站管理员权限';
            self.addGroupShow = true;
        },
        editGroup(scope) {
            var self = this;
            self.groupId = parseInt(scope.row.id);
            self.addGroupTitle = '修改分站管理员权限';
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
            delConfirm(this, params, this.delete, '您确定要删除' + name + '管理员权限组信息吗？');
        },
        delGroupSel() {
            var that = this;
            if (!that.selectedItem.length) {
                message.error('请选择要删除的管理员权限组');
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
