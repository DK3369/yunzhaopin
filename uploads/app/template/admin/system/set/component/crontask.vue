<template>
    <div class="moduleElHight">
        <div class="moduleSeachs">
            <div class="moduleSeachleft"></div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="mini" @click="addcron">添加任务</el-button>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 81px);">
            <el-table :data="tableData" border style="width: 100%" ref="multipleTable"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="id" label="ID" width="80">
                </el-table-column>
                <el-table-column prop="name" label="任务名称" min-width="220">
                </el-table-column>
                <el-table-column prop="dir" label="执行文件" width="220">
                </el-table-column>
                <el-table-column prop="type_n" label="执行类型" width="160">
                </el-table-column>
                <el-table-column prop="display_n" label="是否启用" width="100">
                </el-table-column>
                <el-table-column prop="nowtime_n" label="上次执行时间" width="160">
                </el-table-column>
                <el-table-column prop="nexttime_n" label="下次执行时间" width="160">
                </el-table-column>
                <el-table-column prop="waibu" label="外部调用" width="100">
                    <template slot-scope="scope">
                        <el-link type="primary" @click="copyurl(scope.row.src)">调用</el-link>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" label="操作" width="200">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button size="mini"
                                @click="exec_ctl(scope.row.display, scope.row.id)">执行</el-button>
                            <el-button size="mini" @click="addcron(scope.row.id)">修改</el-button>
                            <el-button size="mini" type="danger" @click="delrow(scope.row)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-dialog title="信息" :visible.sync="drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="300px">
                <div style="overflow: hidden; position: relative; padding-bottom: 15px;">
                    <span>该任务尚未启用！</span>
                </div>
                
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-dialog title="调用地址" :visible.sync="dy_drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="30%">
                <span>复制(CTRL+C)以下內容并添加到服务器配置中</span>
                <div style="margin:20px 5px; overflow: hidden; position: relative; padding-bottom: 20px;">
                    <el-input v-model="curr_url" size="small">
                    </el-input>
                </div>
            </el-dialog>
			<el-drawer title="计划任务" :visible.sync="cron_drawer" :modal-append-to-body="false" :show-close="true" :with-header="true" size="45%">
				<cronadd :id_v="id" @child-event="getList"></cronadd>
			</el-drawer>
        </div>
    </div>
</template>
<script>
module.exports = {
    data: function () {
        return {
            emptytext: '暂无数据',
            loading: false,
            currentPage: 1,
            prevPage: 0,
            perPage: 0,
            pageSizes: [],
            total: 0,
            tableData: [],
            drawer: false,
            dy_drawer: false,
            curr_url: '',
			cron_drawer: false,
			id: '',
        }
    },
	components: {
		'cronadd': httpVueLoader('./cronadd.vue'),
	},
    mounted() {

    },
    methods: {
		addcron(id){
			if (id > 0 ){
				this.id = id;
			}else{
				this.id = '';
			}
			this.cron_drawer = true;
		},
        // 执行计划任务
        exec_cron(id) {
            var that = this
            httpPost('m=system&c=set_cron&a=run', { id: id }).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    message.success(res.msg, function () {
                        that.getList()
                    });
                } else {
                    message.error(res.msg);
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        // 点击执行
        exec_ctl(display, id) {
            var that = this
            delConfirm(that, id, that.exec_cron, '确定现在执行该任务？');
        },
        copyurl(url) {
            this.curr_url = url
            this.dy_drawer = true
        },
        handleSizeChange(val) {
            this.perPage = val;
            this.getList()
        },
        handleCurrentChange(val) {
            this.currentPage = val;
            this.getList();
        },
        async getList() {
            let that = this;
			that.cron_drawer = false;
            let params = {
                page: that.currentPage,
                pageSize: that.perPage
            }
            that.loading = true;
            that.emptytext = "数据加载中";
            httpPost('m=system&c=set_cron&a=index', params).then(function (result) {
                var res = result.data
                if (res.error == 0) {
                    that.tableData = res.data.list
                    that.perPage = parseInt(res.data.perPage)
                    that.pageSizes = res.data.pageSizes
                    that.total = parseInt(res.data.total);
                    if (that.prevPage != that.currentPage) {
                        that.prevPage = that.currentPage;
                        that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                    }
                    that.loading = false;
                    if (that.tableData.length === 0){
                        that.emptytext = "暂无数据";
                    }
                }
            }).catch(function (e) {
                console.log(e)
            })
        },
        delrow(row) {
            delConfirm(this, row.id, this.delete);
        },
        async delete(Ids) {
            let that = this;
            let params = {
                del: Ids
            };
            httpPost('m=system&c=set_cron&a=del', params).then(function (response) {
                if (response.data.error == 0) {
                    message.success('操作成功', that.getList());
                } else {
                    message.error(response.data.msg);
                }
            }).catch(function (error) {
                console.log(error);
            })
        },
    },
};
</script>
<style>
	.el-table .el-table__cell {
    padding: 12px 0;
}
</style>