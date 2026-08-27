<template>
<div id="daohaapp" class="moduleElenAl">
       <div class="moduleSeachs">
			<div class="moduleSeachleft">
				<div class="moduleInptList">
				    <el-input :placeholder="lc('admin_user_weipin_00003')" v-model="searchForm.keyword" size="small" class="input-with-select" clearable>
				        <template #prepend><el-select v-model="searchForm.ftype" :placeholder="lc('admin_yunying_00099')">
				            <el-option :label="lc('admin_yunying_00099')" value="1"></el-option>
				            <el-option :label="lc('admin_yunying_00100')" value="2"></el-option>
				        </el-select></template>
				    </el-input>
				</div>
                <div class="moduleInptList">
                    <el-select v-model="searchForm.status" size="small" clearable :placeholder="lc('admin_user_00161')" @change="search">
                        <el-option :label="lc('admin_user_00164')" value="0"></el-option>
                        <el-option :label="lc('admin_user_00163')" value="1"></el-option>
                    </el-select>
                </div>
				<div class="newsbtnbox"  >
                   <el-button type="primary" icon="el-icon-search" size="small"  @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
                </div>
           </div>
        </div>
        <div class="moduleElTable">
            <el-table :data="list" border style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
                :header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @sort-change="shortChange" v-loading="loading">
                <template #empty>
                    <p>{{dataText}}</p>
                </template>
                <el-table-column type="selection" width="55">
                </el-table-column>
                <el-table-column prop="id" :label="lc('member_com_00345')" width="80" sortable="custom">
                </el-table-column>
                <el-table-column prop="r_name" :label="lc('admin_yunying_00099')" width="260">
                </el-table-column>
                <el-table-column prop="username" :label="lc('admin_yunying_00100')" width="160">
                </el-table-column>
                <el-table-column prop="jubwent" :label="lc('admin_01183')" min-width="220">
                    <template #default="scope">
                        <div class="moduleProps" v-if="scope.row.is_del !=''">
                            {{scope.row.is_del}}
                        </div>
                        <div class="moduleProps" v-else>
                            {{scope.row.title}}
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="reason" :label="lc('admin_system_00133')" width="200">
                </el-table-column>
                <el-table-column prop="inputtime" :label="lc('admin_01184')" width="160" sortable="custom">
					<template #default="scope">
						<span>{{scope.row.inputtime_n}}</span>
					</template>
                </el-table-column>
                <el-table-column prop="zt" :label="lc('member_user_00181')" width="100">
                	<template #default="scope">
                        <div class="admin_state">
                            <span class="admin_state1" v-if="scope.row.status==1">{{ lc('admin_user_00163') }}</span>
                            <span class="admin_state2" v-else>{{ lc('admin_user_00164') }}</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column fixed="right" :label="lc('member_user_00048')" width="138">
                    <template #default="scope">
                        <div class="cz_button">
                            <el-button size="small" @click="resultReport(scope.row)" style="margin-right: 10px;">{{ lc('admin_user_00165') }}</el-button>
                            <el-popover placement="bottom" width="90" trigger="hover">
                                <div class="moduleMores">
                                    <el-button type="text" @click="edit(scope.row.eid)">{{ lc('wap_js_00073') }}</el-button>
                                    <el-button type="text" @click="del(scope.$index)">{{ lc('admin_01181') }}</el-button>
                                    <el-button type="text" @click="delAsk(scope.$index)">{{ lc('admin_01182') }}</el-button>
                                </div>
                                <template #reference><el-button size="small">{{ lc('admin_company_00025') }}</el-button></template>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
        <div class="modulePaging">
            <div class="modulecz modulePagButn">
                <el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate" @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
                <el-button @click="batch('del')">{{ lc('member_com_00055') }}</el-button>
            </div>
            <div class="modulePagNum">
                <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
                   layout="total, sizes, prev, pager, next, jumper" :total="total">
                </el-pagination>
            </div>
        </div>
        <div class="modluDrawer">
            <el-dialog :title="lc('admin_01180')" v-model="statusBox" :with-header="true" :modal-append-to-body="false"
              :show-close="true" width="30%">
             <div>
                 <el-input type="textarea" :rows="2" :placeholder="lc('admin_yunying_00098')" v-model="result">
                 </el-input>
             </div>
              <template #footer><span class="dialog-footer">
                  <el-button @click="statusBox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
                  <el-button type="primary" @click="submitStatus" :disabled="submitLoading">{{ lc('wap_com_00019') }}</el-button>
              </span></template>
            </el-dialog>
			<el-drawer :title="lc('admin_00249')" v-model="editBox" :modal-append-to-body="false" :show-close="true"
			    :with-header="true" size="60%">
			    <editask :id_v="id" @child-event="closeInfo"></editask>
			</el-drawer>
        </div>
    </div>
</template>

<script>
import Editask from './component/editask.vue'

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
                    loading: false,
                    dataText: lc('admin_user_weipin_00026'),
                    // 搜索筛选项
                    searchForm: {
                        ftype: '1',
                        keyword: '',
                        status:'',
                    },
                    page: 1,
                    limit: 0,
                    list: [],
                    total: 0,
                    pageSizes: [],

                    checkedAll: false, // 全选
                    checkedAllIndeterminate: false,
                    multipleSelection: [], // 多选值存储
                    idArr: [],

                    result:'',
                    pid: '',
                    statusBox:false,

					submitLoading:false,
					editBox:false,
					id: '',
                    prevPage:0
                }
            },
			components: {
			    'editask': Editask,
			},
            created: function () {
                var that = this
                let query = window.parent.homeapp.$route.query;


                if (query.status) {
                    that.searchForm.status = query.status;
                }
                this.getList();
            },
            methods: {
                edit(id){
                    this.id = id;
					this.editBox = true;
                },
				closeInfo() {
				    this.editBox = false;
				    this.getList();
				},
                resultReport(row){
                    this.pid = row.id;
                    this.result = row.result;
                    this.statusBox = true;
                },
                submitStatus(){
                    let that = this;
                    let params = {
                        pid: this.pid,
                        result: this.result
                    }
                    that.statusBox = false;
					that.submitLoading = true;
                    httpPost('m=yunying&c=report_ask&a=saveresult', params).then(function(res) {
                        if (res.data.error > 0) {
                            message.error(res.data.msg);
                        } else {
                            message.success(res.data.msg, function () {
                                that.getList();
                            });
                        }
                    }).finally(function () {
						setTimeout(function () {
						    that.submitLoading = false;
						}, 2000);
					});
                },
				shortChange(e) {
				    let orderMap = {ascending: 'asc', descending: 'desc'}
				    this.searchForm.t = e.order ? e.prop : null;
				    this.searchForm.order = orderMap[e.order];
				    this.page = 1;
				    this.getList();
				},
                handleSizeChange(val) {
                    this.limit = val;
                    this.getList();
                },
                handleCurrentChange(val) {
                    this.page = val;
                    this.getList();
                },
                search() {
                    this.page = 1;
                    this.getList();
                },
                getList() {
                    let that = this,
                        params = {
                            page: that.page,
                            limit: that.limit,
                        };
                    let searchForm = that.searchForm;
                    that.loading = true;
                    httpPost('m=yunying&c=report_ask', {...params, ...searchForm}, {hideloading: true}).then(function (response) {
                        let res = response.data,
                            data = res.data;

                        that.list = data.list;
                        that.total = parseInt(data.total);
                        that.pageSizes = data.page_sizes;
                        if (that.limit === 0) {
                            that.limit = parseInt(data.limit); // 取系统配置默认数量
                        }
                        if (that.page > data.page) {
                            that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
                        }
                        if(that.prevPage != that.page){
                            that.prevPage = that.page;
                            that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
                        }
                        that.loading = false;
                        if (that.list.length === 0) {
                            that.dataText = lc('wap_js_00113');
                        }
                    })
                },
                handleSelectionChange(val) {
                    if (val.length == 0) {
                        this.checkedAll = false;
                        this.checkedAllIndeterminate = false;
                    } else {
                        if (val.length === this.list.length) {
                            this.checkedAll = true;
                            this.checkedAllIndeterminate = false;
                        } else {
                            this.checkedAll = false;
                            this.checkedAllIndeterminate = true;
                        }
                    }
                    this.multipleSelection = val;
                },
                batch(type) {
                    if (this.multipleSelection.length == 0) {
                        message.error(lc('admin_user_weipin_00005'));
                        return false;
                    }

                    let idArr = [];
                    this.multipleSelection.forEach(function(item) {
                        idArr.push(item.id);
                    })
                    this.idArr = idArr;

                    if (type == 'del') {
                        this.del();
                    }
                },
                checkAll(val) {
                    val ? this.checkedAllIndeterminate = false : '';
                    this.$refs.multipleTable.toggleAllSelection();
                },
                del(idx) {
                    let that = this,
                        params = {},
                        msg = '';

                    if (typeof idx == 'undefined') { // 批量删除
                        params.del = this.idArr;
                        msg = lc('common_00853');
                    } else {// 单个删除
                        params.del = that.list[idx].id;
                        msg = lc('admin_01185');
                    }

                    delConfirm(this, params, function (params) {
                        httpPost('m=yunying&c=report_ask&a=del', params).then(function(res) {
                            if (res.data.error > 0) {
                                message.error(res.data.msg);
                            } else {
                                message.success(res.data.msg, function () {
                                    that.$refs.multipleTable.clearSelection();
                                    that.getList();
                                });
                            }
                        })
                    }, msg)
                },
                delAsk(idx) {
                    let that = this,
                        params = {},
                        msg = '';
                        params.del = that.list[idx].eid;
                        msg = lc('admin_01186');
                    delConfirm(this, params, function (params) {
                        httpPost('m=yunying&c=report_ask&a=delquestion', params).then(function(res) {
                            if (res.data.error > 0) {
                                message.error(res.data.msg);
                            } else {
                                message.success(res.data.msg, function () {
                                    that.$refs.multipleTable.clearSelection();
                                    that.getList();
                                });
                            }
                        })
                    }, msg)
                }
            }
        }
</script>
