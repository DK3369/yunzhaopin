<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig" v-if="cansearch">
            <div class="tableSeachInpt">
                <el-select v-model="type" size="small" slot="prepend" placeholder="操作类型">
                    <el-option label="消费单号" value="1"></el-option>
                    <el-option label="用户名" value="2"></el-option>
                    <el-option label="备注说明" value="3"></el-option>
                </el-select>
            </div>
            <div class="tableSeachInpt">
                <el-input placeholder="请输入内容" size="small" prefix-icon="el-icon-search" v-model="keyword">
                </el-input>
            </div>
            <div class="tableSeachInpt">
                <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
            </div>
        </div>
        <div class="moduleElTable"
             style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 50px) !important;">
            <el-table :data="tableData" style="width: 100%" stripe ref="multipleTable"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }"
                      :default-sort="{ prop: 'id', order: 'descending' }" @sort-change='sortChange' height="100%" v-loading="loading" @sort-change="shortChange" v-loading="loading" :empty-text="emptytext">>
                <el-table-column prop="order_id" label="消费单号" width="150"></el-table-column>
                <el-table-column prop="uname" label="金额" width="150">
                    <template slot-scope="scope">
                        {{scope.row.order_price}}{{scope.row.type == 1 ? integral_pricename : '元'}}
                    </template>
                </el-table-column>
                <el-table-column prop="pay_remark" label="备注说明">
                </el-table-column>
                <el-table-column prop="pay_time_n" label="消费时间" width="150"></el-table-column>
                <el-table-column prop="pay_state_n" label="状态" width="150">
					<template slot-scope="scope">
						<div v-html="scope.row.pay_state_n"></div>
					</template>
                </el-table-column>
                <el-table-column label="操作" width="80" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button type="danger" size="small " @click="del(scope.row.id)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <!--<el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>-->
                <!--<el-button @click="batchDel">批量删除</el-button>-->
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
        props:{
            searchtype: String,
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
                emptytext: '暂无数据',
                type: '1',
                keyword: '',
                currentPage: 1,
                perPage: 0,
                pageSizes: [],
                total: 0,
                tableHig: true,
                tableData: [],
                weburl: '',
                comuid: '',
                sort_type: '',
                sort_col: '',
                cansearch: true,
                com_uid: '',
                integral_pricename: '',

                prevPage: 0
            }
        },
        created() {
            this.search();
        },
        watch: {
            cuid: {
                handler(val, oldVal) {
                    this.com_uid = val;
                    if (typeof oldVal !== 'undefined' && oldVal != val) {
                        this.search();
                    }
                },
                immediate: true,
                deep: true,
            },
            searchtype: {
                handler(val) {
                    this.type = val;
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
            sortChange: function (column) {
                if (column.order == 'descending') {
                    this.sort_type = 'desc';
                } else if (column.order == 'ascending') {
                    this.sort_type = 'asc';
                } else {
                    this.sort_type = '';
                }
                this.sort_col = column.prop
                this.search();
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
                    params.comid = that.com_uid
                }
                if (that.type) {
                    params.type = that.type
                }
                if (that.keyword) {
                    params.keyword = that.keyword
                }
                if (that.sort_type && that.sort_col) {
                    params.order = that.sort_type
                    params.t = that.sort_col
                }
                that.loading = true;
                that.emptytext = "数据加载中";
                httpPost('m=user&c=company_pay&a=index', params).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list
                        that.perPage = parseInt(res.data.perPage)
                        that.pageSizes = res.data.pageSizes
                        that.total = parseInt(res.data.total)
                        that.integral_pricename = res.data.integral_pricename;
                        that.loading = false;
                        if(that.prevPage != that.currentPage){
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        if (that.tableData.length === 0){
                            that.emptytext = "暂无数据";
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            del: function (id) {
                let _this = this;
                _this.$confirm('确定要删除？', '温馨提示', {
                    confirmButtonText: '确定',
                    cancelButtonText: '取消',
                    type: 'warning'
                }).then(() => {
                    httpPost('m=user&c=company_pay&a=del', {id: id}).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            message.success(res.msg, _this.getList());
                        } else {
                            message.error(res.msg);
                        }
                    })
                }).catch(() => {
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