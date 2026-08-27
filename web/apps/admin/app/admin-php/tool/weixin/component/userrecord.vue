<template>
    <div class="moduleElHight">

        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                <div class="moduleInptList">
                    <el-input :placeholder="lc('admin_tool_00591')" v-model="keyword" class="input-with-select" size="small" clearable>
                        <template #prepend><el-select v-model="wtype" :placeholder="lc('admin_tool_00503')">
                            <el-option :label="lc('admin_tool_00597')" value="1"></el-option>
                            <el-option :label="lc('admin_tool_00596')" value="2"></el-option>
                        </el-select></template>
                    </el-input>
                </div>

                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="status" clearable size="small" :placeholder="lc('member_user_00181')" @change="search">
                        <el-option :label="lc('wap_user_00126')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00139')" value="2"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">

                    <el-select v-model="time" clearable size="small" :placeholder="lc('admin_user_00134')" @change="search">
                        <el-option :label="lc('common_01940')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00179')" value="3"></el-option>
                        <el-option :label="lc('admin_user_00178')" value="7"></el-option>
                        <el-option :label="lc('admin_user_00180')" value="15"></el-option>
                        <el-option :label="lc('admin_tool_00499')" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="danger" icon="el-icon-document-add" size="small" @click="clearwx">{{ lc('admin_tool_00599') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" border style="width: 100%"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                      :empty-text="emptytext">

                <el-table-column prop="wxloginid" :label="lc('member_com_00345')" width="200">
                </el-table-column>
                <el-table-column prop="username" :label="lc('admin_user_company_00144')">
                </el-table-column>
                <el-table-column :label="lc('admin_user_00162')">
                    <template #default="scope">
                        <span v-if="scope.row.usertype==1">{{ lc('admin_user_00304') }}</span>
                        <span v-else-if="scope.row.usertype==2">{{ lc('common.company') }}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="wxid" :label="lc('admin_tool_00588')">
                </el-table-column>
                <el-table-column prop="time_n" :label="lc('admin_tool_00600')">
                </el-table-column>

                <el-table-column prop="zt" :label="lc('admin_tool_00598')">
                    <template #default="scope">
                        <span v-if="scope.row.status==1" class="admin_state1">{{ lc('admin_tool_00595') }}</span>
                        <span v-else class="admin_state2">{{ lc('admin_user_00139') }}</span>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz">
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                               :current-page="currentPage" :page-size="limit" :page-sizes="page_sizes" :total="total"
                               layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
        </div>
    </div>

</template>
<script>
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
            emptytext: window.yunAdminT(lc('wap_js_00113')),
            tableData: [],
            total: 0,
            limit: 0,
            currentPage: 1,
			prevPage:0,
            page_sizes: [],

            list_loading: false,

            allchecked: false,
            choosedata: [],

            keyword: '',
            wtype: '1',
            status: '',
            time: '',
            daterange: [],
            sort_t: '',
            order: '',
        }
    },

    mounted() {
        this.getList();
    },
    methods: {
        async getList() {
            let that = this;
            let params = {
                page: that.currentPage,
                limit: that.limit,
                keyword: that.keyword,
                wtype: that.wtype,
                status: that.status,
                time: that.time
            }


            this.list_loading = true;
            that.emptytext = window.yunAdminT(lc('admin_user_weipin_00026'));
            httpPost('m=tool&c=weixinrecord&a=index', params, {hideloading: true}).then((result) => {
                this.list_loading = false;
                var res = result.data;

                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.total = parseInt(res.data.total)
                    that.page_sizes = res.data.page_sizes;
                    that.limit = res.data.page_size;
					
					if(that.prevPage != that.currentPage){
						that.prevPage = that.currentPage;
						that.$refs.table.bodyWrapper.scrollTop = 0;
					}
                    if (that.tableData.length === 0) {
                        that.emptytext = window.yunAdminT(lc('wap_js_00113'));
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

        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList()
        },
        handleSizeChange(val) {
            this.currentPage = 1
            this.limit = val
            this.getList()
        },

        async clearwx() {
            var that = this;
            this.$confirm(window.yunAdminT(lc('admin_tool_00601')), window.yunAdminT(lc('wap_user_00205')), {
                confirmButtonText: window.yunAdminT(lc('common.confirm')),
                cancelButtonText: window.yunAdminT(lc('common.cancel')),
                type: 'warning'
            }).then(() => {
                httpPost('m=tool&c=weixinrecord&a=clearwx', {}).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, that.search());
                    } else {
                        message.error(res.msg);
                    }
                })
            })
        },
        doLayout(){
            if (this.$refs.table) {
                this.$nextTick(() => {
                    this.$refs.table.doLayout();
                })
            }
        }
    },
};
</script>
<style scoped>
.tableSeachInpt {
    margin-top: 0px;
    margin-bottom: -2px;
}


</style>