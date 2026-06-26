<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig" v-if="cansearch">
            <div class="tableSeachInpt">
                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" size="small" prefix-icon="el-icon-search" v-model="keyword">
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
            </div>
        </div>
        <div class="moduleElTable"
             style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 50px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe ref="multipleTable"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      @selection-change="handleSelectionChange" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="type_n" label="套餐类目" width="150"></el-table-column>
                <el-table-column prop="num" label="消耗数量" width="150"></el-table-column>
                <el-table-column prop="detail" label="描述"></el-table-column>
                <el-table-column prop="ip" label="IP" width="150"></el-table-column>
                <el-table-column prop="time_n" label="时间" width="150"></el-table-column>
                <el-table-column label="操作" width="200" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " type="danger" @click="deleteRow(scope)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">{yun:}t key='member_com_00055'{/yun}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange"
                               @current-change="handleCurrentChange"
                               :current-page="currentPage" :page-sizes="pageSizes"
                               :page-size="perPage"
                               layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
    </div>
</template>
<script>
    module.exports = {
        props: {
            searchword: String,
            searchable: {
                type: Boolean,
                default: true
            },
            cuid: String
        },
        data: function () {
            return {
                loading: false,
                emptytext: "{yun:}t key='wap_js_00113'{/yun}",
                keyword: '',
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                tableHig: true,
                tableData: [],
                cansearch: true,
                com_uid: '',
                integral_pricename: '',
                islook: false,
                checkedAll: false,//全选
                selectedItem: [],

                prevPage: 0
            }
        },
        created() {
            this.search();
        },
        watch: {
            cuid: {
                handler(val) {
                    this.com_uid = val;
                },
                immediate: true,
                deep: true,
            },
            searchword: {
                handler(val) {
                    this.keyword = val;
                },
                immediate: true,
                deep: true,
            },
            searchable: {
                handler(val) {
                    this.cansearch = val;
                },
                immediate: true,
                deep: true,
            },
        },
        methods: {
            handleSelectionChange(val) {
                this.selectedItem = val;
                if (this.selectedItem.length == 0) {
                    this.isIndeterminate = false;
                    this.checkedAll = false;
                } else {
                    if (this.selectedItem.length == this.tableData.length) {
                        this.isIndeterminate = false;
                        this.checkedAll = true;
                    } else {
                        this.isIndeterminate = true;
                        this.checkedAll = false;
                    }
                }
            },
            selectAllBottom(value) {
                value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
            },
            handleSizeChange(val) {
                this.perPage = val;
                this.getList()
            },
            handleCurrentChange(val) {
                this.currentPage = val;
                this.getList()
            },
            search() {
                this.currentPage = 1;
                this.getList();
            },
            getList: function () {
                let that = this;
                let params = {
                    page: that.currentPage,
                    pageSize: that.perPage
                }
                if (that.com_uid) {
                    params.uid = that.com_uid
                }
                if (that.keyword) {
                    params.keyword = that.keyword
                }
                that.loading = true;
                that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
                httpPost('m=user&c=company&a=statisDetail', params,{ hideloading: true }).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list
                        that.perPage = parseInt(res.data.perPage)
                        that.pageSizes = res.data.pageSizes
                        that.total = parseInt(res.data.total)
                        that.integral_pricename = res.data.integral_pricename
                        that.loading = false;
                        if(that.prevPage != that.currentPage){
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        if (that.tableData.length === 0){
                            that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
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
                    params.id = scope.row.id;
                }
                delConfirm(this, params, this.delete);
            },
            delete(params) {
                let _this = this;
                httpPost('m=user&c=company&a=delStatisDetail', params).then(function (response) {
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
        },
    };
</script>
<style scoped>

    .moduleElHight .moduleElTable {
        padding: 0;
        margin: 0;
        height: calc(100% - 110px);
        width: 100%;
    }

    .moduleElTableHig {
        height: calc(100% - 90px) !important;
    }
</style>