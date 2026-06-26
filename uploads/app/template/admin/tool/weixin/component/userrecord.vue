<template>
    <div class="moduleElHight">

        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                <div class="moduleInptList">
                    <el-input placeholder="{yun:}t key='admin_tool_00591'{/yun}" v-model="keyword" class="input-with-select" size="small" clearable>
                        <el-select v-model="wtype" slot="prepend" placeholder="{yun:}t key='admin_tool_00503'{/yun}">
                            <el-option label="{yun:}t key='admin_tool_00597'{/yun}" value="1"></el-option>
                            <el-option label="{yun:}t key='admin_tool_00596'{/yun}" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>

                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="status" clearable size="small" slot="prepend" placeholder="{yun:}t key='member_user_00181'{/yun}" @change="search">
                        <el-option label="{yun:}t key='wap_user_00126'{/yun}" value="1"></el-option>
                        <el-option label="{yun:}t key='admin_user_00139'{/yun}" value="2"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">

                    <el-select v-model="time" clearable size="small" slot="prepend" placeholder="{yun:}t key='admin_user_00134'{/yun}" @change="search">
                        <el-option label="{yun:}t key='common_01940'{/yun}" value="1"></el-option>
                        <el-option label="{yun:}t key='admin_user_00179'{/yun}" value="3"></el-option>
                        <el-option label="{yun:}t key='admin_user_00178'{/yun}" value="7"></el-option>
                        <el-option label="{yun:}t key='admin_user_00180'{/yun}" value="15"></el-option>
                        <el-option label="{yun:}t key='admin_tool_00499'{/yun}" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="danger" icon="el-icon-document-add" size="mini" @click="clearwx">{yun:}t key='admin_tool_00599'{/yun}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" border style="width: 100%"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                      :empty-text="emptytext">

                <el-table-column prop="wxloginid" label="{yun:}t key='member_com_00345'{/yun}" width="200">
                </el-table-column>
                <el-table-column prop="username" label="{yun:}t key='admin_user_company_00144'{/yun}">
                </el-table-column>
                <el-table-column label="{yun:}t key='admin_user_00162'{/yun}">
                    <template slot-scope="scope">
                        <span v-if="scope.row.usertype==1">{yun:}t key='admin_user_00304'{/yun}</span>
                        <span v-else-if="scope.row.usertype==2">{yun:}t key='common.company'{/yun}</span>
                    </template>
                </el-table-column>
                <el-table-column prop="wxid" label="{yun:}t key='admin_tool_00588'{/yun}">
                </el-table-column>
                <el-table-column prop="time_n" label="{yun:}t key='admin_tool_00600'{/yun}">
                </el-table-column>

                <el-table-column prop="zt" label="{yun:}t key='admin_tool_00598'{/yun}">
                    <template slot-scope="scope">
                        <span v-if="scope.row.status==1" class="admin_state1">{yun:}t key='admin_tool_00595'{/yun}</span>
                        <span v-else class="admin_state2">{yun:}t key='admin_user_00139'{/yun}</span>
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
module.exports = {
    data: function () {
        return {
            emptytext: window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}"),
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
            that.emptytext = window.yunAdminT("{yun:}t key='admin_user_weipin_00026'{/yun}");
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
                        that.emptytext = window.yunAdminT("{yun:}t key='wap_js_00113'{/yun}");
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
            this.$confirm(window.yunAdminT("{yun:}t key='admin_tool_00601'{/yun}"), window.yunAdminT("{yun:}t key='wap_user_00205'{/yun}"), {
                confirmButtonText: window.yunAdminT("{yun:}t key='common.confirm'{/yun}"),
                cancelButtonText: window.yunAdminT("{yun:}t key='common.cancel'{/yun}"),
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