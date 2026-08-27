<template>
<div id="yujinapp" class="moduleElenAl">

    <div class="moduleSeachs">
        <div class="moduleSeachleft">
            <div class="moduleSeachInpt" style="margin-right: 12px;">
                <el-date-picker size="small" style="width: 260px;" v-model="daterange" type="daterange" @change="changedate" :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')"></el-date-picker>
            </div>
            <div class="tableSeachInptsmall newsinput">
                <el-select v-model="status" size="small" :clearable="true" :placeholder="lc('member_user_00181')" @change="search">
                    <el-option :label="lc('admin_system_00197')" value="1"></el-option>
                    <el-option :label="lc('member_user_00289')" value="2"></el-option>
                </el-select>
            </div>
			<div style="overflow: hidden;position: relative;display: flex;flex-wrap: wrap;align-items: center;">
                <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="moduleSeachButn" style="display: flex;align-items: center;">
            <el-button type="primary" icon="el-icon-document-add" size="small" @click="setshow=true">{{ lc('admin_system_00581') }}</el-button>
        </div>
    </div>

    <div class="moduleElTable">

        <el-table ref="multipleTable" :data="tableData" @selection-change="selectionChange" border
                  style="width: 100%" :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
            <el-table-column type="selection" width="55">
            </el-table-column>
            <el-table-column prop="id" :label="lc('member_com_00345')" width="120">
            </el-table-column>
            <el-table-column :label="lc('admin_user_00119')" width="200">
                <template #default="scope">
                    <div class="moduleProps">
                        <span class=" ">{{scope.row.name_n}}</span>
                        <span class=" ">{{scope.row.username}}</span>
                    </div>
                </template>
            </el-table-column>
            <el-table-column prop="usertype_n" :label="lc('admin_user_00162')" width="120">
            </el-table-column>
            <el-table-column prop="content" :label="lc('admin_system_00580')">
            </el-table-column>
            <el-table-column prop="ctime_n" :label="lc('member_user_00241')" width="220">
            </el-table-column>
            <el-table-column :label="lc('member_user_00181')" width="120">
                <template #default="scope">
                    <span v-if="scope.row.status==1">{{ lc('admin_system_00197') }}</span>
                    <span v-else-if="scope.row.status==2">{{ lc('member_user_00289') }}</span>
                </template>
            </el-table-column>
            <el-table-column fixed="right" :label="lc('member_user_00048')" width="80">
                <template #default="scope">
                    <div class="cz_button">
                        <el-button size="small" type="danger" @click="deleteinfo(scope.row.id)">{{ lc('wap_js_00077') }}</el-button>
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
            <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                           :current-page="currentPage" :page-sizes="page_sizes" :page-size="page_size" :total="total"
                           layout="total, sizes, prev, pager, next, jumper">
            </el-pagination>
        </div>
    </div>

    <div class="modluDrawer">
        <el-drawer :title="lc('admin_system_00581')" v-model="setshow" :modal-append-to-body="false" :show-close="true"
                   :with-header="true" size="92%">
            <warningset v-model:setshow="setshow"></warningset>
        </el-drawer>
    </div>
</div>
</template>

<script>
import Warningset from './component/warningset.vue'

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
                loading: false,
                status: '',
                tableData: [],
                total: 0,
                limit: '',
                currentPage: 1,
                prevPage: 0,
                page_sizes: [],
                page_size:0,

                daterange: [],

                allchecked: false,
                choosedata: [],

                setshow: false,

            }
        },
        components: {
            'warningset': Warningset,
        },
        created: function () {
            var that = this
            let query = window.parent.homeapp.$route.query;


            if (query.status) {
                that.status = query.status;
            }

            this.getList();
        },
        methods: {
            async getList() {
                let that = this;
                let params = {
                    page: that.currentPage,
                    limit: that.limit
                }
                if (this.daterange && this.daterange.length > 0) {

                    params['date1'] = this.daterange[0].getTime() / 1000;
                    params['date2'] = this.daterange[1].getTime() / 1000;
                }
                if (this.status) {
                    params.status = this.status;
                }
                this.loading = true;
                that.emptytext = lc('admin_user_weipin_00026');
                httpPost('m=system&c=warning&a=index', params).then((result) => {

                    var res = result.data
                    if (res.error == 0) {
                        that.tableData = res.data.list;
                        that.total = parseInt(res.data.total);
                        that.page_sizes = res.data.page_sizes;
                        that.page_size = res.data.page_size;
                        if (that.prevPage != that.currentPage) {
                            that.prevPage = that.currentPage;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        that.loading = false;
                        if (that.tableData.length === 0){
                            that.emptytext = lc('wap_js_00113');
                        }
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
            search: function () {
                this.currentPage = 1;
                this.getList();
            },
            changedate: function () {
                if (this.daterange === null) {
                    this.currentPage = 1;
                    this.getList();
                }

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
            allcheckChange: function () {

                this.$refs.multipleTable.toggleAllSelection();

            },
            selectionChange: function (e) {
                if (this.tableData.length != e.length) {
                    this.allchecked = false;
                } else {
                    this.allchecked = true;
                }
                this.choosedata = e;
            },
            deleteinfo: function (id) {
                var _this = this;

                var params = {
                    del: id
                };
                delConfirm(_this, params, this.deletePost)
            },
            deleteAll: function () {
                var _this = this;
                var idarr = [];
                if (this.choosedata.length > 0) {
                    for (let i in this.choosedata) {
                        idarr.push(this.choosedata[i].id);
                    }
                } else {
                    message.error(lc('admin_user_weipin_00005'));
                    return;
                }
                var params = {
                    del: idarr
                };

                delConfirm(_this, params, this.deletePost)
            },
            async deletePost(params) {

                let that = this;

                httpPost('m=system&c=warning&a=del', params).then(function (result) {

                    var res = result.data;
                    if (res.error == 9) {
                        message.success(res.msg, function () {
                            that.getList();
                        });
                        return;
                    } else {
                        message.error(res.msg);
                        return;
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },

        }
    }
</script>
