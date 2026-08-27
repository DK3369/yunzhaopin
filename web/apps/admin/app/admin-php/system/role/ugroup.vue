<template>
<div id="daohaapp" class="moduleElenAl">
        <div class="moduleSeachs">
            <div class="">{{ lc('admin_system_00232') }}</div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="addgroup">{{ lc('admin_system_00234') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <div class="tableDome_tip">
                <el-alert :title="lc('admin_system_00231')" type="success" :closable="false">
                </el-alert>
            </div>
            <el-table :data="tableData" border style="width: 100%"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="calc(100% - 48px)" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="id" :label="lc('member_com_00345')" width="80">
                </el-table-column>
                <el-table-column prop="group_name" :label="lc('admin_system_00236')"> </el-table-column>
                <el-table-column prop="group_type_n" :label="lc('admin_system_00233')"> </el-table-column>
                <el-table-column prop="num" :label="lc('admin_system_00235')"> </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="addgroup(scope.row.id)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="small" @click="rowdel(scope.row)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <el-drawer :title="lc('admin_system_00232')" v-model="group_drawer" :modal-append-to-body="false" :show-close="true" :with-header="true" size="85%">
				<groupadd :id_v="id" @child-event="closeGroupAdd"></groupadd>
			</el-drawer>
        </div>
    </div>
</template>

<script>
import Groupadd from './component/groupadd.vue'

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
                    emptytext: window.lc('wap_js_00113'),
                    loading: false,
                    currentPage: 1,
                    perPage: 0,
                    pageSizes: [],
                    total: 0,
                    tableData: [],
					group_drawer: false,
                    
					id: ''
                }
            },
            components: {
				'groupadd': Groupadd,
            },
            created: function () {
                this.getList();


            },
            methods: {
				addgroup(id){
					if (id > 0){
						this.id = id;
					}else{
						this.id = ''
					}
					this.group_drawer = true;
				},
                handleSizeChange(val) {
                    this.perPage = val;
                    this.getList()
                },
                handleCurrentChange(val) {
                    this.currentPage = val;
                    this.getList()
                },
                async getList() {
                    let that = this;
                    that.loading = true;
                    that.emptytext = window.lc('admin_user_weipin_00026');
                    httpPost('m=system&c=role_ugroup&a=index', { page: that.currentPage, pageSize: that.perPage }).then(function (result) {
                        var res = result.data
                        if (res.error == 0) {
                            that.tableData = res.data.list
                            that.perPage = parseInt(res.data.perPage)
                            that.pageSizes = res.data.pageSizes
                            that.total = parseInt(res.data.total);
                            that.loading = false;
                            if (that.tableData.length === 0){
                                that.emptytext = window.lc('wap_js_00113');
                            }
                        }
                    }).catch(function (e) {
                        console.log(e)
                    })
                },
                rowdel(row) {
                    if (!row.id) {
                        this.$message({ showClose: true, message: window.lc('admin_user_weipin_00005'), type: 'warning' });
                        return false;
                    }
                    delConfirm(this, row.id, this.delete);
                },
                delete(id) {
                    let _this = this;
                    let params = {
                        id: id
                    };
                    httpPost('m=system&c=role_ugroup&a=del', params).then(function (response) {
                        if (response.data.error == 0) {
                            _this.$message.success(window.lc('wap_user_00264'));
                            _this.getList();
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                },
                closeGroupAdd(){
                    this.group_drawer = false;
                    this.getList();
                }
            }
        }
</script>
