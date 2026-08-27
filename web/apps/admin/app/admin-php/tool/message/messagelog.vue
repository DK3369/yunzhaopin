<template>
<div id="daohaapp" class="moduleElenAl">
			<div class="moduleSeachmore">
				<div class="tableSeachInpt  tableSeachInptsmall">
               
                    <el-select v-model="time" clearable @change="dateSelectChange" size="small" :placeholder="lc('wap_js_00088')" >
                        <el-option :label="lc('common_01940')" value="1"></el-option>
                        <el-option :label="lc('admin_user_00179')" value="3"></el-option>
                        <el-option :label="lc('admin_user_00178')" value="7"></el-option>
                        <el-option :label="lc('admin_user_00180')" value="15"></el-option>
                        <el-option :label="lc('admin_tool_00499')" value="30"></el-option>
                    </el-select>
                </div>
                <div class="tableSeachInpt tableSeachInptsmalltwo"> 
                    <el-date-picker v-model="daterange" type="daterange" @change="changedate" :range-separator="lc('admin_company_00019')"
                            :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" size="samll">
                    </el-date-picker> 
                </div>
				<div class="tableSeachInpt tableSeachInptsmall">
					<el-select v-model="state" clearable size="small" :placeholder="lc('admin_tool_00496')" @change="search">
						<el-option :label="lc('admin_tool_00495')" value="1"></el-option>
						<el-option :label="lc('admin_tool_00494')" value="2"></el-option>
					</el-select>
				</div>
                <div class="tableSeachInpt tableSeachInptsmall">
                    <el-select v-model="port" clearable size="small" :placeholder="lc('admin_tool_00497')" @change="search">
                        <el-option :label="name" :value="val" v-for="(name,val) in ports" :key="val"></el-option>
                    </el-select>
                </div>
				<div class="tableSeackellsd" style="padding: 2px 0; margin-bottom: 8px;">
					<el-input v-model="keyword" :placeholder="lc('admin_system_00198')" size="small" prefix-icon="el-icon-search" clearable>
                        <template #prepend><el-select  v-model="type" size="small" :placeholder="lc('admin_tool_00503')">
                            <el-option :label="lc('wap_01619')" value="1"></el-option>
                            <el-option :label="lc('admin_tool_00500')" value="2"></el-option>
                            <el-option :label="lc('member_user_00281')" value="3"></el-option>
                            <el-option :label="lc('wap_user_00102')"   value="4"></el-option>
                        </el-select></template>
					</el-input>
				</div>
				<div class="tableSeachInpt">
					<el-button type="primary" size="small" icon="el-icon-search" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
				</div>
			</div>
			<div class="moduleElTable">
				<el-table  ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange"
                    :default-sort = "{prop: 'id', order: 'descending'}" @sort-change="sortChange" border style="width: 100%"
					:header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" :empty-text="emptytext">
					<el-table-column type="selection" width="55">
					</el-table-column>
					<el-table-column prop="id" :label="lc('common_02108')" width="80" sortable="custom">
					</el-table-column>
					<el-table-column prop="moblie" :label="lc('wap_01619')"  width="110">
					</el-table-column>

					<el-table-column :label="lc('admin_tool_00493')"  width="200">
                        <template #default="scope">
                            <div style="color:#009688">{{scope.row.fname}}</div>
                            <div>{{scope.row.sname}}</div>
                        </template>
					</el-table-column>
					<el-table-column prop="content" :label="lc('wap_user_00102')" min-width="300">
					</el-table-column>
					<el-table-column prop="ctime_n" :label="lc('admin_tool_00504')" sortable="custom" width="200">
						<template #default="scope">
							<div>{{scope.row.ip}}</div>
							<div>{{scope.row.ctime_n}}</div>
						</template>
					</el-table-column>
					<el-table-column :label="lc('member_user_00181')" width="110">
						<template #default="scope">
							<span v-if="scope.row.state==0" class="admin_state1">{{ lc('admin_tool_00502') }}</span>
                            <el-tooltip placement="top" v-else>
                               <div>{{scope.row.result}}</div>
                               <el-button  size="small" type="danger" plain>{{ lc('admin_tool_00501') }}<i class="el-icon-info el-icon--right"></i></el-button>
                            </el-tooltip>
                            
						</template>
					</el-table-column>
                    <el-table-column :label="lc('admin_tool_00505')"  width="110">
                        <template #default="scope">
                            {{scope.row.port_n}}<br/>{{scope.row.location}}
                        </template>
                        
                    </el-table-column>
					<el-table-column :label="lc('member_user_00048')" width="80" fixed="right" header-align="center">
						<template #default="scope">
							<div class="cz_button">

								<el-button type="danger" size=" "  @click="deleteinfo(scope.row.id)">{{ lc('wap_js_00077') }}</el-button>
							</div>

						</template>
					</el-table-column>

				</el-table>
			</div>
			<div class="modulePaging">
               	<div class="modulecz">
                    <el-checkbox v-model="allchecked" @change="allcheckChange">{{ lc('wap_js_00074') }}</el-checkbox>
                    <el-button  size="small" @click="deleteAll">{{ lc('member_com_00055') }}</el-button>
                    <el-button   size="small" @click="repeatSend">{{ lc('admin_tool_00498') }}</el-button>
                </div>
                <div class="modulePagNum">
                    <el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
                        :current-page="currentPage" :page-size="page_size" :page-sizes="page_sizes" :total="total"
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
				data: function() {
					return {
                        emptytext: window.lc('wap_js_00113'),
                        type:'1',
                        keyword:'',
                        time:'',
                        state:'',

						tableData: [],
                        total: 0,
                        limit: '',
                        page_size: 0,
                        currentPage: 1,
						prevPage:0,
                        page_sizes: [],
                        sort_t:'',
                        order:'',
                        list_loading: false,
                        daterange: [],

                        allchecked: false,
                        choosedata: [],
                        ports:[],
                        port:'',
					}
				},
                created: function () {
                    this.getList();
                    this.getBaseData();
                },
				methods: {
					async getList() {
                        let that = this;
                        let params = {
                            page: that.currentPage,
                            limit: that.limit,
                            time:that.time,
                            state:that.state,
                            port:that.port,
                            t:that.sort_t,
                            order:that.order,
                            type:that.type,
                            keyword:that.keyword
                        }
                        if (this.daterange && this.daterange.length > 0) {

                            params['date1'] = this.daterange[0].getTime()/1000;
                            params['date2'] = this.daterange[1].getTime()/1000;
                        }
                        this.list_loading = true;
                        that.emptytext = window.lc('admin_user_weipin_00026');
                        httpPost('m=tool&c=messagelog&a=index', params, {hideloading: true}).then((result) => {
                            this.list_loading = false;
                            var res = result.data
                            if (res.error == 0) {
                                that.tableData = res.data.list;
                                that.total = parseInt(res.data.total);
                                that.page_sizes = res.data.page_sizes;
                                that.page_size = res.data.page_size;
								
								if(that.prevPage != that.currentPage){
									that.prevPage = that.currentPage;
									that.$refs.table.bodyWrapper.scrollTop = 0;
								}
                                if (that.tableData.length === 0){
                                    that.emptytext = window.lc('wap_js_00113');
                                }
                            }
                        }).catch(function (e) {
                            console.log(e)
                        })
                    },
                    getBaseData() {
                        let _this = this;
                        httpPost('m=tool&c=messagelog&a=index_base_data', {}, {hideloading: true}).then(function (response) {
                            let res = response.data;
                            _this.ports = Object.freeze(res.data.ports);
                        }).catch(function (error) {
                            console.log(error);
                        });
                    },
                    search: function () {
                        this.currentPage = 1;
                        this.getList();
                    },
                    sortChange:function(e){
                        this.sort_t = e.prop;
                        if(e.prop=='ctime_n'){
                          this.sort_t = 'ctime';
                        }
                        this.order = e.order=='ascending'?'asc':'desc';
                        this.search();
                    },
                    changedate: function (e) {
                        if(e!==null){
                            this.time = '';
                        }
						this.search();
                    },
                    dateSelectChange:function(e){
                        if(e!=''){
                            this.daterange = [];
                        }
                        this.search();
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
                    allcheckChange: function () {

                        this.$refs.table.toggleAllSelection();

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
                            message.error(window.lc('admin_user_weipin_00005')); return;
                        }
                        var params = {
                            del: idarr
                        };

                        delConfirm(_this, params, this.deletePost)
                    },
                    repeatSend:function(){
                        var _this = this;
                        var idarr = [];
                        if (this.choosedata.length > 0) {
                            for (let i in this.choosedata) {
                                idarr.push(this.choosedata[i].id);
                            }
                        } else {
                            message.error(window.lc('admin_tool_00506')); return;
                        }
                        var params = {
                            id: idarr
                        };
                        _this.$confirm(window.lc('admin_tool_00507'), window.lc('wap_user_00205'), {
                            confirmButtonText: window.lc('common_02016'),
                            cancelButtonText: window.lc('wap_js_00080'),
                            type: 'warning'
                        }).then(() => {
                            _this.repeatSendPost(params);
                        });
                    },
                    async repeatSendPost(params) {

                        let that = this;

                        httpPost('m=tool&c=messagelog&a=repeat', params).then(function (result) {

                            var res = result.data;
                            if (res.error == 0) {
                                message.success(res.msg, function () { that.getList();}); return;
                            } else {
                                message.error(res.msg); return;
                            }
                        }).catch(function (e) {
                            console.log(e)
                        })
                    },
                    async deletePost(params) {

                        let that = this;

                        httpPost('m=tool&c=messagelog&a=del', params).then(function (result) {

                            var res = result.data;
                            if (res.error == 0) {
                                message.success(res.msg, function () { that.getList(); }); return;
                            } else {
                                message.error(res.msg); return;
                            }
                        }).catch(function (e) {
                            console.log(e)
                        })
                    },
				}
			}
</script>
