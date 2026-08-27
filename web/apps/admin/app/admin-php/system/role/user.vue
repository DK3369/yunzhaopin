<template>
<div id="daohaapp" class="moduleElenAl">
        <div class="moduleSeachs">
            
			<div class="moduleSeachleft">
				<div class="tableSeachInptsmall tableSeachFomud">
				    <el-select style="width: 220px;" v-model="m_id" size="small" :placeholder="lc('admin_00014')" clearable @change="search">
				        <el-option v-for="item in group" :key="item.id" :label="item.group_name" :value="item.id"></el-option>
				    </el-select>
				</div>
				<div class="moduleInptList">
				    <el-input :placeholder="lc('admin_system_00237')" size="small" v-model="keyword" class="input-with-select" clearable></el-input>
				</div>
			    <div class="newsbtnbox"  >
			        <el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
			    </div>
			 </div>
            <div class="moduleSeachButn">
                <el-button type="primary" icon="el-icon-document-add" size="small" @click="add">{{ lc('admin_system_00238') }}</el-button>
            </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="tableData" border style="width: 100%" ref="multipleTable"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
                <el-table-column prop="uid" :label="lc('member_com_00345')" width="80">
                </el-table-column>
                <el-table-column prop="username" :label="lc('admin_user_00140')"></el-table-column>
                <el-table-column :label="lc('admin_user_company_00372')">
                    <template #default="scope">
                        <el-tag>{{scope.row.group_name}}</el-tag>
                    </template>
                </el-table-column>
                <el-table-column prop="name" :label="lc('wap_00529')"></el-table-column>
                <el-table-column :label="lc('wap_00462')">
                    <template #default="scope">
                        <div v-if="scope.row.mobile">{{ lc("admin_phone_value", [scope.row.mobile]) }}</div>
                        <div v-if="scope.row.weixin">{{ lc("admin_wechat_value", [scope.row.weixin]) }}</div>
                        <div v-if="scope.row.qq">QQ：{{scope.row.qq}}</div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="edit(scope.row)">{{ lc('wap_js_00073') }}</el-button>
                            <el-button size="small" @click="delrow(scope.row)" type="danger">{{ lc('wap_js_00077') }}</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div>
                <!--<el-checkbox v-model="checked">全选</el-checkbox>-->
                <!--<el-button>批量删除</el-button>-->
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                    :current-page="currentPage" :page-sizes="pageSizes" :page-size="perPage"
                    layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <!-- 抽屉弹窗 -->
        <div class="modluDrawer">
            <el-drawer :title="title" v-model="showadd" :modal-append-to-body="false" :show-close="true"
                :with-header="true" size="600px">
                <add :user="info" :week="week" :group="group" source="useradd" @complete="completeAdd"></add>
            </el-drawer>
        </div>
    </div>
</template>

<script>
import AdminAdd from '../../component/admin_add.vue'

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
					keyword:'',
					m_id:'',
					
                    emptytext: window.lc('wap_js_00113'),
                    loading: false,
                    showadd: false,
                    currentPage: 1,
                    prevPage: 0,
                    perPage: 0,
                    pageSizes: [],
                    total: 0,
                    tableData: [],
                    group: [],
                    week: [],
                    info: {},
                    // centerDialogVisible: false,
                    title: window.lc('admin_system_00239'),
                    
					save_load:false,
                }
            },
            components: {
                'add': AdminAdd,
            },
            created: function () {
                let that = this;


                that.getList();
            },
            methods: {
                add() {
                    var that = this
                    that.title = window.lc('admin_system_00239');
                    that.info = {};
                    that.showadd = true;
                },
                edit(row) {
                    var that = this
                    that.title = window.lc('admin_system_00240');
                    that.info = row;
                    that.showadd = true;
                },
                completeAdd() {
                    this.showadd = false;
                    this.getList();
                },
                handleSizeChange(val) {
                    this.perPage = val;
                    this.getList()
                },
                handleCurrentChange(val) {
                    this.currentPage = val;
                    this.getList();
                },
				search:function(){
					this.currentPage = 1;
					this.getList()
				},
                async getList() {
                    let that = this;
                    let params = {
                        page: that.currentPage,
                        pageSize: that.perPage,
						keyword:that.keyword,
						m_id:that.m_id
                    }
                    that.loading = true;
                    that.emptytext = window.lc('admin_user_weipin_00026');
                    httpPost('m=system&c=role_user&a=index', params).then(function (result) {
                        var res = result.data
                        if (res.error == 0) {
                            that.tableData = res.data.list
                            that.group = res.data.group
                            that.week = res.data.week
                            that.perPage = parseInt(res.data.perPage)
                            that.pageSizes = res.data.pageSizes
                            that.total = parseInt(res.data.total);
                            if (that.prevPage != that.currentPage) {
                                that.prevPage = that.currentPage;
                                that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                            }
                            that.loading = false;
                            if (that.tableData.length === 0){
                                that.emptytext = window.lc('wap_js_00113');
                            }
                        }
                    }).catch(function (e) {
                        console.log(e)
                    })
                },
                delrow(row) {
                    delConfirm(this, row.uid, this.delete);
                },
                async delete(id) {
                    let that = this;
                    let params = {
                        del: id
                    };
                    httpPost('m=system&c=role_user&a=del', params).then(function (response) {
                        if (response.data.error == 0) {
                            message.success(window.lc('wap_user_00264'));
                            that.getList();
                        } else {
                            message.error(response.data.msg);
                        }
                    }).catch(function (error) {
                        console.log(error);
                    })
                },
            }
        }
</script>
