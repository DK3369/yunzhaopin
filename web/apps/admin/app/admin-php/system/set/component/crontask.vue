<template>
    <div class="moduleElHight">
        <div class="moduleSeachs">
            <div class="moduleSeachleft"></div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="addcron">{{ lc('admin_00898') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable" style="height: calc(100% - 81px);">
            <el-table :data="tableData" border style="width: 100%" ref="multipleTable"
                :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="id" label="ID" width="80">
                </el-table-column>
                <el-table-column prop="name" :label="lc('admin_00892')" min-width="220">
                </el-table-column>
                <el-table-column prop="dir" :label="lc('admin_system_00262')" width="220">
                </el-table-column>
                <el-table-column prop="type_n" :label="lc('admin_system_00272')" width="160">
                </el-table-column>
                <el-table-column prop="display_n" :label="lc('admin_system_00263')" width="100">
                </el-table-column>
                <el-table-column prop="nowtime_n" :label="lc('admin_00901')" width="160">
                </el-table-column>
                <el-table-column prop="nexttime_n" :label="lc('admin_00902')" width="160">
                </el-table-column>
                <el-table-column prop="waibu" :label="lc('admin_system_00271')" width="100">
                    <template #default="scope">
                        <el-link type="primary" @click="copyurl(scope.row.src)">{{ lc('admin_system_00275') }}</el-link>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="200">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small"
                                @click="exec_ctl(scope.row.display, scope.row.id)">{{ lc('admin_system_00274') }}</el-button>
                            <el-button size="small" @click="addcron(scope.row.id)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="small" type="danger" @click="delrow(scope.row)">{{ lc('common.delete') }}</el-button>
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
            <el-dialog :title="lc('admin_00308')" v-model="drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="300px">
                <div style="overflow: hidden; position: relative; padding-bottom: 15px;">
                    <span>{{ lc('admin_00899') }}</span>
                </div>
                
            </el-dialog>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_00903')" v-model="dy_drawer" :with-header="true" :modal-append-to-body="false"
                :show-close="true" width="30%">
                <span>{{ lc('admin_00900') }}</span>
                <div style="margin:20px 5px; overflow: hidden; position: relative; padding-bottom: 20px;">
                    <el-input v-model="curr_url" size="small">
                    </el-input>
                </div>
            </el-dialog>
			<el-drawer :title="lc('admin_system_00273')" v-model="cron_drawer" :modal-append-to-body="false" :show-close="true" :with-header="true" size="45%">
				<cronadd :id_v="id" @child-event="getList"></cronadd>
			</el-drawer>
        </div>
    </div>
</template>
<script>
import Cronadd from './cronadd.vue'

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
		'cronadd': Cronadd,
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
            delConfirm(that, id, that.exec_cron, lc('admin_vue_00049'));
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
            that.emptytext = lc('admin_user_weipin_00026');
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
                        that.emptytext = lc('wap_js_00113');
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
                    message.success(lc('wap_user_00264'), that.getList());
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