<template>
    <div class="moduleElenAlCategorySub">
        <div class="moduleSeachs categorySub">
            <div></div>
            <div class="categoryTopBtn">
                <el-button class="" type="primary" icon="el-icon-document-add" size="mini" @click="openAdd('')">添加类别</el-button>
            </div>
        </div>
        <div class="moduleElTable moduleElTableCategoreSub">
            <el-table :data="list" :default-sort="{prop: 'date', order: 'descending'}" stripe border
                      ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="sortChange" :empty-text="emptytext"
                      style="width: 100%;height: 100%;" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" height="100%">
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="id" label="编号" width="90" sortable="custom">
                </el-table-column>
                <el-table-column prop="name" label="类别">
                </el-table-column>
                <el-table-column prop="add_time_n" label="发布时间">
                </el-table-column>
                <el-table-column label="操作" width="200" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="openAdd(scope.row)">修改</el-button>
                            <el-button type="danger" size="mini" @click="del(scope.$index)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">全选</el-checkbox>
                <el-button @click="batch('del')" size="mini">批量删除</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                               :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                               layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>

        <div class="modluDrawer">
            <el-drawer :title="detail.id ? '修改类别' : '添加类别'" :visible.sync="drawerAdd" append-to-body :show-close="true"
                       :with-header="true" size="45%">
                <add :refresh="random" :id="detail.id ? detail.id : ''" source="manage" @child-event="closeAdd"></add>
            </el-drawer>
        </div>
    </div>
</template>

<script setup>
module.exports = {
    props: {
        pid: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            emptytext: '暂无数据',
            loading: false,
            // 列表
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            // 列表排序
            t: '',
            order: '',

            checkedAll: false, // 全选
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},

            // 添加
            drawerAdd: false,
            random: 0,
            prevPage:0
        }
    },
    components: {
        'add': httpVueLoader('./class_add.vue'),
    },
    created() {
        this.getList();
    },
    methods: {
        handleSizeChange(val) {
            this.limit = val;
            this.getList();
        },
        handleCurrentChange(val) {
            this.page = val;
            this.getList();
        },
        sortChange(event) {
            this.t = event.order ? event.prop : '';
            this.order = event.order ? event.order == 'descending' ? 'desc' : 'asc' : '';
            this.search();
        },
        search() {
            this.page = 1;
            this.getList();
        },
        getList() {
            let that = this,
                params = {
                    page: that.page,
                    limit: that.limit,
                    t: that.t,
                    order: that.order,
                    pid: that.pid,
                };
                that.loading = true;
                that.emptytext = "数据加载中";
            httpPost('m=neirong&c=question_class', params).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.list = data.list;
                that.total = parseInt(data.total);
                that.pageSizes = data.page_sizes;
                if (that.limit === 0) {
                    that.limit = parseInt(data.limit); // 取系统配置默认数量
                }
                if (that.page > data.page) {
                    that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                }
                if(that.prevPage != that.page){
                    that.prevPage = that.page;
                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                }
                that.loading = false;
                if (that.list.length === 0){
                    that.emptytext = "暂无数据";
                }
            })
        },

        // 批量操作
        handleSelectionChange(val) {
            if (val.length == 0) {
                this.checkedAll = false;
                this.checkedAllIndeterminate = false;
            } else {
                if (val.length === this.list.length) {
                    this.checkedAll = true;
                    this.checkedAllIndeterminate = false;
                } else {
                    this.checkedAll = false;
                    this.checkedAllIndeterminate = true;
                }
            }
            this.multipleSelection = val;
        },
        batch(type) {
            if (this.multipleSelection.length == 0) {
                let msg = '请选择要操作的数据项'
                if (type == 'del') {
                    msg = '请选择要删除的数据项'
                }
                message.error(msg);
                return false;
            }

            let idArr = [];
            this.multipleSelection.forEach(function (item) {
                idArr.push(item.id);
            })
            this.idArr = idArr;

            if (type == 'del') {
                this.del();
            } else if (type == 'audit') {
                this.openAudit();
            }
        },
        checkAll(val) {
            val ? this.checkedAllIndeterminate = false : '';
            this.$refs.multipleTable.toggleAllSelection();
        },

        del(idx) {
            let that = this,
                params = {},
                msg = '';

            if (typeof idx == 'undefined') { // 批量删除
                params.del = this.idArr;
                msg = '你确定要删除选中项吗？';
            } else {// 单个删除
                params.id = that.list[idx].id;
                msg = '你确定要删除当前项吗？';
            }
            params.qid = that.id;

            delConfirm(this, params, function (params) {
                httpPost('m=neirong&c=question_class&a=del', params).then(function (res) {
                    if (res.data.error > 0) {
                        message.error(res.data.msg);
                    } else {
                        message.success(res.data.msg, function () {
                            that.$refs.multipleTable.clearSelection();
                            that.getList();
                        });
                    }
                })
            }, msg)
        },

        openAdd(row) {
            this.detail = row == '' ? {} : row;
            this.random = Math.floor(Math.random() * 1000);
            this.drawerAdd = true;
        },

        closeAdd() {
            this.drawerAdd = false;
            this.getList();
        },
    },
    watch: {
        pid: function (val, oldVal) {
            this.search();
        },
    }
}
</script>

<style scoped>
</style>