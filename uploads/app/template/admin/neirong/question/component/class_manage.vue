<template>
    <div class="moduleElenAlCategorySub">
        <div class="moduleSeachs categorySub">
            <div></div>
            <div class="categoryTopBtn">
                <el-button class="" type="primary" icon="el-icon-document-add" size="mini" @click="openAdd('')">{yun:}t key='admin_00222'{/yun}</el-button>
            </div>
        </div>
        <div class="moduleElTable moduleElTableCategoreSub">
            <el-table :data="list" :default-sort="{prop: 'date', order: 'descending'}" stripe border
                      ref="multipleTable" @selection-change="handleSelectionChange" @sort-change="sortChange" :empty-text="emptytext"
                      style="width: 100%;height: 100%;" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" height="100%">
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="id" label="{yun:}t key='member_com_00345'{/yun}" width="90" sortable="custom">
                </el-table-column>
                <el-table-column prop="name" label="{yun:}t key='admin_00223'{/yun}">
                </el-table-column>
                <el-table-column prop="add_time_n" label="{yun:}t key='admin_user_weipin_00030'{/yun}">
                </el-table-column>
                <el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="200" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " plain @click="openAdd(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
                            <el-button type="danger" size="mini" @click="del(scope.$index)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="batch('del')" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                               :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                               layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>

        <div class="modluDrawer">
            <el-drawer :title="detail.id ? lc('admin_00221') : lc('admin_00222')" :visible.sync="drawerAdd" append-to-body :show-close="true"
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
            emptytext: "{yun:}t key='wap_js_00113'{/yun}",
            loading: false,
            // list
            page: 1,
            limit: 0,
            list: [],
            total: 0,
            pageSizes: [],

            // 列表排序
            t: '',
            order: '",

            checkedAll: false, // {yun:}t key='wap_js_00074'{/yun}
            checkedAllIndeterminate: false,
            multipleSelection: [], // 多选值存储
            idArr: [],

            detail: {},

            // Add
            drawerAdd: false,
            random: 0,
            prevPage:0
        }
    },
    components: {
        "add': httpVueLoader('./class_add.vue'),
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
                that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
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
                    that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
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
                let msg = "{yun:}t key='admin_user_weipin_00001'{/yun}"
                if (type == 'del') {
                    msg = "{yun:}t key='admin_00136'{/yun}"
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

            if (typeof idx == 'undefined") { // {yun:}t key='member_com_00055'{/yun}
                params.del = this.idArr;
                msg = "{yun:}t key='common_00853'{/yun}";
            } else {// {yun:}t key='common_01711'{/yun}
                params.id = that.list[idx].id;
                msg = lc('admin_00333');
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