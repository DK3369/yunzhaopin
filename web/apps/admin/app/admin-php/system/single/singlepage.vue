<template>
<div id="daohaapp" class="moduleElenAl">
        <div class="moduleSeachs" style="padding-bottom: 0px;">
            <div class="moduleSeachleft">
                <div class="moduleElSearchInf">
                    <div class="moduleElTabInpt" style="flex-wrap: wrap;">
                        <div class="moduleInptList" style="flex-wrap: wrap;">
                            <el-input v-model="keyword" :placeholder="lc('admin_00340')">
                                <template #prepend><el-select v-model="is_type" size="small" :placeholder="lc('admin_system_00688')">
                                    <el-option :label="lc('admin_system_00663')" value="0"></el-option>
                                    <el-option :label="lc('wap_user_00309')" value="1"></el-option>
                                    <el-option :label="lc('admin_00198')" value="2"></el-option>
                                </el-select></template>
                            </el-input>
                        </div>
                        <div class="tableSeachInpt">
                            <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                        </div>
                    </div>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" plain size="small" @click="make('')">{{ lc('admin_system_00686') }}</el-button>
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="addinfo('')">{{ lc('admin_system_00685') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table ref="multipleTable" :data="tableData" border style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @selection-change="selectionChange" v-loading="loading" :empty-text="emptytext">
                <el-table-column type="selection" width="55"></el-table-column>
                <el-table-column prop="id" :label="lc('admin_system_00682')" width="80"></el-table-column>
                <el-table-column prop="name" :label="lc('admin_system_00684')"></el-table-column>
                <el-table-column prop="is_type_n" :label="lc('admin_system_00689')"></el-table-column>
                <el-table-column prop="ctime_n" :label="lc('member_com_00300')"></el-table-column>
                <el-table-column :label="lc('admin_system_00687')">
                    <template #default="scope">
                        <el-tag>{{scope.row.is_nav==1?lc('member_com_00023'):lc('admin_user_00340')}}</el-tag>
                    </template>
                </el-table-column>
                <el-table-column :label="lc('admin_vue_00044')" width="80">
                    <template #default="scope">
                        <div class="moduleElTaPax" v-if="editsort_id==scope.row.id">
                            <el-input id="inputref" :placeholder="lc('wap_user_00076')" v-model="editsort" @input="inputIntNumber($event, 'editsort', '')" :data-preval="scope.row.sort" @blur="editChange"></el-input>
                        </div>
                        <div class="moduleElTaPax" v-else>
                            <span>{{scope.row.sort}}</span>
                            <img src="/admin/php-admin/images/bine.png" @click="editcolumn(scope.row.sort,scope.row.id)" alt="">
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="moduleElTaCaoz">
                            <el-button size="small">
                                <el-link style="color: #606266;" type="primary" :underline="false" :href="scope.row.url_pc" target="_blank">{{ lc('wap_00071') }}</el-link>
                            </el-button>
                            <el-button size="small" @click="make(scope.row.id)" v-if="scope.row.is_type==1">{{ lc('wap_00225') }}</el-button>
                            <el-button size="small" @click="addinfo(scope.row.id)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="small" @click="deleteinfo(scope.row.id)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="allchecked" @change="allcheckChange">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button size="small" @click="deleteAll">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange" :current-page="currentPage" :page-size="limit" :page-sizes="page_sizes" :total="total" layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
        <!-- 弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="lc('admin_system_00685')" v-model="addshow" :modal-append-to-body="false" :show-close="true" :with-header="true" size="60%">
                <addsingle :sid="sid" @close-update="closeUpdate"></addsingle>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import Addsingle from './component/addsingle.vue'

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
        data: function() {
            return {
                emptytext: window.lc('wap_js_00113'),
                is_type: '',
                keyword: '',

                tableData: [],
                total: 0,
                limit: 0,
                currentPage: 1,
                prevPage: 0,
                page_sizes: [],

                editsort_id: '',
                editsort: '',

                allchecked: false,
                choosedata: [],

                sid: '',
                addshow: false,
                addloading: false,

                timer: '',
                loading: false,
            }
        },
        components: {
            'addsingle': Addsingle,
        },
        created: function() {
            this.getList();


        },
        methods: {
            inputIntNumber(val, form, key) {
                this.$data[form] = val.replace(/[^0-9]/g,'');
            },
            search() {
                this.currentPage = 1
                this.getList()
            },
            async getList() {
                let that = this;
                let params = {
                    is_type: that.is_type,
                    keyword: that.keyword,
                    page: that.currentPage,
                    limit: that.limit
                }
                that.loading = true;
                that.emptytext = window.lc('admin_user_weipin_00026');
                httpPost('m=system&c=singlepage&a=index', params).then(function(result) {
                    endLoading();
                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = (res.data && res.data.list) ? res.data.list : []
                        that.total = parseInt(res.data.total || 0)
                        that.page_sizes = res.data.page_sizes || [10, 20, 50, 100];
						that.limit = res.data.page_size || that.limit || 20;
                        if (that.prevPage != that.currentPage) {
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        that.loading = false;
                        if (that.tableData.length === 0){
                            that.emptytext = window.lc('wap_js_00113');
                        }
                    }
                }).catch(function(e) {
                    console.log(e)
                })
            },
            handleCurrentChange(val) {
                this.currentPage = val;
                this.getList();
            },
            handleSizeChange(val) {
                this.currentPage = 1
                this.limit = val
                this.getList()
            },
            editcolumn: function(def, id) {

                this.editsort_id = id;
                this.editsort = def;

                this.$nextTick(() => {
                    if (timer) {
                        clearTimeout(timer);
                    }
                    timer = setTimeout(() => {
                        document.getElementById('inputref').focus();
                    }, 100);
                })

            },
            async editChange(e) {

                var that = this;

                var preval = e.target.dataset.preval;

                var val = this.editsort;
                var id = this.editsort_id;

                if (val == preval) {

                    this.editsort_id = '';
                    this.editsort = '';

                } else {

                    var param = { id: id, sort: val };

                    httpPost('m=system&c=singlepage&a=ajax', param).then(function(result) {

                        for (let i in that.tableData) {
                            if (that.tableData[i].id == id) {
                                that.tableData[i].sort = val;
                                break;
                            }
                        }

                        that.editsort_id = '';
                        that.editsort = '';
                        message.success(window.lc('admin_user_company_00208'),function(){
                            that.getList()
                        });

                    }).catch(function(e) {
                        console.log(e)
                    })
                }

            },

            allcheckChange: function() {

                this.$refs.multipleTable.toggleAllSelection();

            },
            selectionChange: function(e) {
                if (this.tableData.length != e.length) {
                    this.allchecked = false;
                } else {
                    this.allchecked = true;
                }
                this.choosedata = e;
            },
            deleteinfo: function(id) {
                var _this = this;

                var params = {
                    del: id
                };
                delConfirm(_this, params, this.deletePost)
            },
            deleteAll: function() {
                var _this = this;
                var idarr = [];
                if (this.choosedata.length > 0) {
                    for (let i in this.choosedata) {
                        idarr.push(this.choosedata[i].id);
                    }
                } else {
                    message.error(window.lc('member_com_00084'));
                    return;
                }
                var params = {
                    del: idarr
                };

                delConfirm(_this, params, this.deletePost)
            },
            async deletePost(params) {

                let that = this;

                httpPost('m=system&c=singlepage&a=del', params).then(function(result) {

                    var res = result.data;
                    if (res.error == 9) {
                        message.success(res.msg, function() { that.getList() });
                        return;
                    } else {
                        message.error(res.msg);
                        return;
                    }
                }).catch(function(e) {
                    console.log(e)
                })
            },

            addinfo: function(id) {
                if (this.sid != id || id == '') {
                    this.sid = id;
                    this.timer = new Date().getTime();
                }
                this.addshow = true;
            },
            closeUpdate: function() {
                this.addshow = false;
                this.getList()
            },
            async make(id) {

                let that = this;
                var params = {
                    id: id
                };
                httpPost('m=system&c=singlepage&a=make', params).then(function(result) {

                    var res = result.data;
                    if (res.error == 0) {
                        message.success(res.msg);
                        return;
                    } else {
                        message.error(res.msg);
                        return;
                    }
                }).catch(function(e) {
                    console.log(e)
                })
            },
        }
    }
</script>
