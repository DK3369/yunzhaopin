<template>
    <div class="moduleElHight">
        <div class="moduleSeachbig" v-if="cansearch">
            <div class="tableSeachInpt">
                <el-select v-model="type" size="small" slot="prepend" placeholder="操作类型">
                    <el-option label="消费单号" value="1"></el-option>
                    <el-option label="用户名" value="2"></el-option>
                    <el-option label="企业名称/姓名" value="3"></el-option>
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
                      :default-sort="{ prop: 'id', order: 'descending' }" @selection-change="handleSelectionChange"  @sort-change='sortChange' height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="order_id" label="消费单号" width="150"></el-table-column>
                <el-table-column prop="order_type_n" label="支付类型" width="150"></el-table-column>
                <el-table-column prop="order_type_n" label="订单类型" width="150">
                    <template slot-scope="scope">
                        {{scope.row.type_n}}{{scope.row.rating_name}}
                    </template>
                </el-table-column>
                <el-table-column prop="order_price" label="付款金额" width="150" sortable="custom">
                    <template slot-scope="scope">
                        {{scope.row.order_price}}
                    </template>
                </el-table-column>
                <el-table-column prop="order_remark" label="备注说明">
                </el-table-column>
                <el-table-column prop="order_time_n" label="充值时间" width="150" sortable="custom"></el-table-column>
                <el-table-column prop="order_state_n" label="状态" width="150">
                    <template slot-scope="scope">
                        <span v-html="scope.row.order_state_n"></span>
                    </template>
                </el-table-column>
                <el-table-column prop="order_state_n" label="业务员" width="150">
                    <template slot-scope="scope">
                        <span v-if="scope.row.crm_name">{{scope.row.crm_name}}</span>
                        <span v-else style="color: red;">未绑定</span>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="200" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="small " @click="showdtl(scope.row.id)">查看</el-button>
                            <el-button size="small " @click="ht(scope.row.id)">合同</el-button>
                            <el-button size="small " type="danger" @click="deleteRow(scope)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="deleteRow(null, true)" size="mini">批量删除</el-button>
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
        <div class="modluDrawer">
            <el-drawer title="合同图片" :append-to-body="true" :visible.sync="showhtpic" :destroy-on-close="true" size="85%">
                <com_htimg :oid="curr_id" style="margin-left:10px;"></com_htimg>
            </el-drawer>
        </div>
        <div class="modluDrawer" v-if="curr_dtl">
            <el-drawer title="订单详情" :append-to-body="true" :visible.sync="dtlVisible" :destroy-on-close="true"
                       size="530px">
                <div>
                    <div class="uploadTable" style="padding:0 20px;">
                        <div class="jiliTanJinli">
                            <div class="jiliTanJinTite">
                                <span>订单号</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.order_id}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli">
                            <div class="jiliTanJinTite">
                                <span>申请交易时间</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.order_time_n}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli">
                            <div class="jiliTanJinTite">
                                <span>用户名</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span v-if="curr_dtl.type == 3 || curr_dtl.order_type == 'bank'">
                                    {{curr_dtl.comname}}+
                                </span>
                                <span>{{curr_dtl.username}}</span>
                            </div>
                        </div>

                        <div class="jiliTanJinli">
                            <div class="jiliTanJinTite">
                                <span>{{curr_dtl.order_state == 1 ? '待付' : '付款'}}金额</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <el-input v-if="curr_dtl.order_state == 1 || curr_dtl.order_state == 3" placeholder="请输入金额" size="small" v-model="curr_dtl.order_price" type="number">
                                    <template slot="append">元</template>
                                </el-input>
                                <span v-else>{{curr_dtl.order_price}}元</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="curr_dtl.type == 3||curr_dtl.order_type=='bank'">
                            <div class="jiliTanJinTite">
                                <span>汇款银行</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.bankname}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="curr_dtl.type == 3||curr_dtl.order_type=='bank'">
                            <div class="jiliTanJinTite">
                                <span>汇入账号</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.bankid}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="curr_dtl.type == 3||curr_dtl.order_type=='bank'">
                            <div class="jiliTanJinTite">
                                <span>汇款金额</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.order_price}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli">
                            <div class="jiliTanJinTite">
                                <span>备注</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <el-input v-if="curr_dtl.order_state == 1 || curr_dtl.order_state == 3" placeholder="请输入备注" size="small" v-model="curr_dtl.order_remark" type="textarea" rows="3">
                                </el-input>
                                <span v-else>{{curr_dtl.order_remark}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli">
                            <div class="jiliTanJinTite">
                                <span>订单类型</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.type_n}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="curr_dtl.type == 2">
                            <div class="jiliTanJinTite">
                                <span>所得{{integral_pricename}}</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span>{{curr_dtl.integral}}</span>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="htpics.length > 0">
                            <div class="jiliTanJinTite">
                                <span>合同图片</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <div style="display: flex;">
                                    <el-image v-for="(item,index) in htpics" :src="item.pic_n" :key="index" :preview-src-list="previewPics" style="width: 140px; height: 140px"></el-image>
                                </div>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="curr_dtl.type=='3'||curr_dtl.order_type=='bank'">
                            <div class="jiliTanJinTite">
                                <span>上传汇款单</span>
                            </div>
                            <div class="jiliTanJinCont">
                                <span v-if="curr_dtl.order_state == 1 || !curr_dtl.order_pic"></span>
                                <div v-else>
                                    <el-image :src="curr_dtl.order_pic" :preview-src-list="[curr_dtl.order_pic]" style="width: 160px; height: 160px"></el-image>
                                    <el-alert title="提示：点击图片可以查看大图,如图片仍看不清，请在图片上右击鼠标，选择“在新标签页中打开图片”或“图片另存为”" type="info" :closable="false">
                                    </el-alert>
                                </div>
                            </div>
                        </div>
                        <div class="jiliTanJinli" v-if="curr_dtl.order_state==1||curr_dtl.order_state==3">
                            <div class="jiliTanJinTite">
                                <span></span>
                            </div>
                            <div class="jiliTanJinCont">
                                <el-button size="small " @click="del(scope.row.id)">修改</el-button>
                                <el-button size="small " @click="del(scope.row.id)">确认</el-button>
                            </div>
                        </div>
                    </div>
                </div>
            </el-drawer>
        </div>
    </div>
</template>
<script>
    module.exports = {
        props: {
            searchtype: String,
            searchword: String,
            searchable: {
                type: Boolean,
                default: true
            },
            cuid: String,
			from: {
			    type: String,
			    default: ''
			},
			param:{
				type: Object,
				default: function(){
					return {};
				}
			}
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
                sort_type: '',
                sort_col: '',
                cansearch: true,
                com_uid: '',
				is_crm:'',
				order_state:'',
				order_type:'',
                integral_pricename: '',
                islook: false,
                dtlVisible: false,
                curr_dtl: null,
                htpics: [],
                previewPics: [],
                checkedAll: false,//全选
                selectedItem: [],
                curr_id: '',
                showhtpic: false,

                prevPage: 0
            }
        },
        mounted() {
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
			param: {
				handler(obj) {
					
					if (!$.isEmptyObject(obj)) {
						
						for(let i in obj){
							this[i] = obj[i];
						}
						
					}
				},
				immediate: true,
				deep: true,
			},
        },
        components: {
            'com_htimg': httpVueLoader('./com_htimg.vue'),
        },
        methods: {
            ht: function(id){
                this.curr_id = id
                this.showhtpic = true
            },
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
            showdtl: function(id){
                var that = this
                httpPost('m=user&c=company_order&a=edit', {id: id}).then(function (result) {
                    var res = result.data
                    if (res.error == 0) {
                        that.curr_dtl = res.data.row
                        that.htpics = res.data.htpics
                        that.previewPics = res.data.preview_pics
                        that.integral_pricename = res.data.integral_pricename
                        that.dtlVisible = true
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            sortChange: function (column) {
                if (column.order == 'descending') {
                    this.sort_type = 'desc';
                } else if (column.order == 'ascending') {
                    this.sort_type = 'asc';
                } else {
                    this.sort_type = '';
                }
                this.sort_col = column.prop
                if (this.sort_col == 'order_time_n') {
                    this.sort_col = 'order_time'
                }
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
				if (that.is_crm) {
				    params.is_crm = that.is_crm
				}
                if (that.type) {
                    params.type = that.type
                }
				if(that.order_state){
					params.order_state = that.order_state
				}
				if(that.order_type){
					params.order_type = that.order_type
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
				var url = '';
				if(that.from=='persona'){
					url='m=crm&c=crm_my_customer&a=company_order';
				}else{
					url='m=user&c=company_order&a=index';
				}
                httpPost(url, params).then(function (result) {
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
                            that.emptytext = "暂无数据";
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
                        message.error('请选择要删除的数据');
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
                httpPost('m=user&c=company_order&a=del', params).then(function (response) {
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