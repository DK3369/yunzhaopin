<template>
    <div class="moduleElHight">

        <div class="moduleSeachs">
            <div class="moduleSeachleft">
                <div class="moduleInptList">
                    <el-input placeholder="请输入绑定用户" v-model="keyword" class="input-with-select" size="small" clearable>
                        <el-select v-model="wtype" slot="prepend" placeholder="搜索类型">
                            <el-option label="微信昵称" value="1"></el-option>
                            <el-option label="已绑定用户" value="2"></el-option>
                        </el-select>
                    </el-input>
                </div>

                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="status" clearable size="small" slot="prepend" placeholder="状态" @change="search">
                        <el-option label="已登录" value="1"></el-option>
                        <el-option label="未登录" value="2"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmall">

                    <el-select v-model="time" clearable size="small" slot="prepend" placeholder="登录时间" @change="search">
                        <el-option label="今天" value="1"></el-option>
                        <el-option label="最近三天" value="3"></el-option>
                        <el-option label="最近七天" value="7"></el-option>
                        <el-option label="最近半月" value="15"></el-option>
                        <el-option label="最近一月" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt">
                    <el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
                </div>
            </div>
            <div class="moduleSeachButn">
                <el-button type="danger" icon="el-icon-document-add" size="mini" @click="clearwx">清理前三天数据</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table ref="table" :data="tableData" v-loading="list_loading" border style="width: 100%"
                      :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%"
                      :empty-text="emptytext">

                <el-table-column prop="wxloginid" label="编号" width="200">
                </el-table-column>
                <el-table-column prop="username" label="用户名称">
                </el-table-column>
                <el-table-column label="用户类型">
                    <template slot-scope="scope">
                        <span v-if="scope.row.usertype==1">个人</span>
                        <span v-else-if="scope.row.usertype==2">企业</span>
                    </template>
                </el-table-column>
                <el-table-column prop="wxid" label="绑定ID（OpenId）">
                </el-table-column>
                <el-table-column prop="time_n" label="生成时间">
                </el-table-column>

                <el-table-column prop="zt" label="扫码状态">
                    <template slot-scope="scope">
                        <span v-if="scope.row.status==1" class="admin_state1">已登陆</span>
                        <span v-else class="admin_state2">未登录</span>
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
            emptytext: '暂无数据',
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
            that.emptytext = "数据加载中";
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
                        that.emptytext = "暂无数据";
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
            this.$confirm('确定要清除三天前的数据？', '温馨提示', {
                confirmButtonText: '确定',
                cancelButtonText: '取消',
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