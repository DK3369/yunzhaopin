<template>
	<div class="moduleElHight">
		<div class="moduleSeachbig">
			<!--关键字搜索和查询在一起-->
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-input v-model="searchForm.keyword" @keyup.enter.native="search" :placeholder="lc('admin_user_weipin_00003')" size="small"
						  prefix-icon="el-icon-search" clearable>
					<el-select v-model="searchForm.type" size="small" slot="prepend" :placeholder="lc('wap_00529')">
						<el-option :label="lc('wap_00529')" :value="1"></el-option>
						<el-option :label="lc('wap_user_00015')" :value="2"></el-option>
					</el-select>
				</el-input>
			</div>
			<div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="tableSeachInpt tableSeachInptsmall">
				<el-select v-model="searchForm[searchItem.param]" slot="prepend" :clearable="true" :placeholder="searchItem.name" size="small" @change="search">
					<el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel" :value="searchValue"></el-option>
				</el-select>
			</div>
			<div class="tableSeachInpt">
				<el-button type="primary" icon="el-icon-search" size="mini" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
			</div>

		</div>

		<div class="admin_datatip">
			<i class="el-icon-document"></i> {{ lc("admin_data_stats") }} <span @click="init">{{ lc("admin_total_count", [resumeAllNum]) }}</span>
			<span class="admin_datatip_n"><span @click="statusSearch('3')">{{ lc("admin_pending_review_count", [resumeStatusNum1 ? resumeStatusNum1 : 0]) }}</span></span>
			<span class="admin_datatip_n">{{ lc('admin_user_00233') }}<span @click="statusSearch('2')">{{resumeStatusNum2 ? resumeStatusNum2 : 0}}</span> {{ lc('common_02088') }}</span>
			<span class="admin_datatip_n">{{ lc("admin_search_results_count", [total]) }}</span>
		</div>
		<div class="moduleElTable" :class="{ 'modulElTableGai': tableHig }"
			style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
			<el-table :data="list" style="width: 100%" stripe ref="multipleTable" @selection-change="handleSelectionChange"
					  @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
				<template slot="empty">
					<p>{{dataText}}</p>
				</template>
				<el-table-column type="selection" width="55"> </el-table-column>
				<el-table-column prop="uid" :label="lc('member_com_00345')" width="80" sortable="custom"> </el-table-column>
				<el-table-column prop="uname" :label="lc('wap_00529')" width="100"> </el-table-column>
				<el-table-column :label="lc('admin_vue_00017')" width=" ">
					<template slot-scope="scope">
						<div>
							<span v-if="scope.row.name">{{scope.row.name}}</span>
							<span v-else style="color: #FF0000;">{{ lc('member_com_00211') }}</span>
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="price" :label="lc('wap_00563')"> </el-table-column>
				<el-table-column prop="add_time" :label="lc('wap_com_00342')" width="150" sortable="custom">
					<template slot-scope="scope">
						<div>{{scope.row.add_time_n}}</div>
					</template>
				</el-table-column>
				<el-table-column prop="" :label="lc('admin_00488')" width="150" align="center">
					<template slot-scope="scope">
						<el-button v-if="scope.row.name && scope.row.status == 1" type="text" size="small" plain
								   @click="openRecom(scope.row)">
							<i class="el-icon-search"></i> {{ lc('admin_00488') }}
						</el-button>
						<div v-else>-</div>
					</template>
				</el-table-column>
				<el-table-column :label="lc('member_user_00181')" width="60" fixed="right">
					<template slot-scope="scope">
						<div class="admin_state">
							<span v-if="scope.row.status == 1" class="admin_state1">{{ lc('wap_com_00191') }}</span>
					        <span v-else-if="scope.row.status == 2" class="admin_state2">{{ lc('admin_user_00234') }}</span>
							<span v-else class="admin_state5">{{ lc('wap_user_00166') }}</span>
						</div>
					</template>
				</el-table-column>
				<el-table-column :label="lc('member_user_00048')" width="190" fixed="right" align="center">
					<template slot-scope="scope">
						<div class="cz_button">
							<template v-if="scope.row.name">
								<el-button plain @click="openPreview(scope.row)">{{ lc('wap_00071') }}</el-button>
								<el-button v-if="scope.row.status == 0" size="mini" plain @click="openAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>
							</template>
							<el-button v-if="scope.row.status != 0" type="danger" size="mini" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
       	<div class="modulePaging">
			<div>
				<el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
							 @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
				<el-button @click="batch('del')" size="mini">{{ lc('member_com_00055') }}</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
							   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
							   layout="total, sizes, prev, pager, next, jumper" :total="total">
				</el-pagination>
			</div>
		</div>
		 <!--审核-->
		<div class="modluDrawer">
			<el-dialog :title="lc('admin_00489')" :visible.sync="dialogAudit" :with-header="true" :modal-append-to-body="false"
				:show-close="true" width="450px">
				<div>
					<div class="wxsettip_small ">{{ lc('admin_user_weipin_00032') }} </div>
					<template>
						<el-radio v-model="ruleFormAudit.status" label="1">{{ lc('admin_user_00236') }}</el-radio>
						<el-radio v-model="ruleFormAudit.status" label="2">{{ lc('admin_user_00234') }}</el-radio>
					</template>
					<div class="wxsettip_small ">{{ lc('member_user_00062') }} </div>
					<div class="wxsettip">{{ lc('admin_user_00232') }} </div>
				</div>
				<span slot="footer" class="dialog-footer">
					<el-button @click="dialogAudit = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" @click="submitAudit">{{ lc('wap_com_00019') }}</el-button>
				</span>
			</el-dialog>
		</div>

		<div class="modluDrawer">
			<!--预览简历-->
			<el-drawer :title="lc('wap_user_00217')" :visible.sync="drawerPreview" append-to-body size="60%">
				<preview :id="detail.eid"></preview>
			</el-drawer>
			<!--匹配岗位-->
			<el-drawer :title="lc('admin_00488')" :append-to-body="true" :visible.sync="drawerRecom" :show-close="true"
					   :with-header="true" size="80%">
				<recom :id="detail.id" :eid="detail.eid"></recom>
			</el-drawer>
		</div>
	</div>
</template>

<script>
	module.exports = {
		props: {
			status: {type: String, default: ''}
		},
		data: function() {
			return {
				loading: false,
				dataText: lc('admin_user_weipin_00026'),
				tableHig: true,
				saveLoading: false,

				// 搜索筛选项
				searchList: [],
				searchForm: {
					type: 1,
					status: this.status,
				},

				// list
				page: 1,
				limit: 0,
				list: [],
				total: 0,
				pageSizes: [],

				// 列表排序
				t: '',
				order: '",

				checkedAll: false, // {{ lc('wap_js_00074') }}
				checkedAllIndeterminate: false,
				multipleSelection: [], // 多选值存储
				idArr: [],

				detail: {}, // 单条数据记录

				// Data statistics
				resumeAllNum: 0,
				resumeStatusNum1: 0,
				resumeStatusNum2: 0,

				// Audit
				dialogAudit: false,
				ruleFormAudit: {},

				// {{ lc('wap_user_00217') }}
				drawerPreview: false,

				// {{ lc('admin_00488') }}
				drawerRecom: false,

				prevPage: 0
			}
		},

		mounted() {
			var that = this
	        setTimeout(function () {
	            that.getSearchFun();
	        }, 200)
		},
		components: {
			"preview': httpVueLoader('../../../component/resume_preview.vue'),
			'recom': httpVueLoader('./resume_trust_recom.vue'),
		},
		created() {
			var that = this;
			let params = window.parent.homeapp.$route.params;
			let query = window.parent.homeapp.$route.query;
			
			if (!$.isEmptyObject(query.params)) {
				params = {...params,...query.params};
			}
			
			if (!$.isEmptyObject(params)) {
				delete params.activeName;
				this.getParams(params);
			}
			this.init();
		},
		methods: {
			getParams:function(params={},search=false){
				var that = this;
				for(let i in params){
					if(typeof that.searchForm[i]!='undefined'){
						that.searchForm[i] = params[i];
					}
				}
				if(search){
					this.search();
				}
			},
			init() {
				// this.resetSearch();
				this.getCountData();
				this.search();
			},

			resetSearch() {
				this.searchForm = {
					type: 1
				};
				this.limit = 0;
			},

			handleSizeChange(val) {
				this.limit = val;
				scrollToTop()
				this.getList();
			},
			handleCurrentChange(val) {
				this.page = val;
				this.getList();
			},
			sortChange(event) {
				this.t = event.order ? event.prop : '';
				this.order = event.order ? event.order == 'descending' ? 'desc' : 'asc' : '';
				this.search();
			},

			// Data statistics
			statusSearch(status) {
				this.resetSearch();
				this.searchForm.status = status;
				this.search();
			},
			getCountData() {
				let that = this;

				httpPost('m=user&c=users_trust&a=trustNum', {}, {hideloading: true}).then(function (response) {
					let res = response.data;

					that.resumeAllNum = res.resumeAllNum;
					that.resumeStatusNum1 = res.resumeStatusNum1;
					that.resumeStatusNum2 = res.resumeStatusNum2;
				})
			},
			getSearchFun:function(){
	            let that = this;
	            httpPost('m=user&c=users_trust&a=getSearchData', {},{hideloading: true}).then(function (response) {
	                let res = response.data;
	                if (res.error == 0) {
	                    that.searchList = res.data.search_list;
	                }
	            })
	        },
			search() {
				this.page = 1;
				this.getList();
			},
			getList() {
				let that = this,
					searchForm = that.searchForm,
					params = {
						page: that.page,
						limit: that.limit,
						t: that.t,
						order: that.order,
					};
				that.loading = true;
				httpPost('m=user&c=users_trust', { ...params, ...searchForm }, {hideloading: true}).then(function (response) {
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
					that.loading = false;
					if(that.prevPage != that.page){
	                    that.prevPage = that.page;
	                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
	                    scrollToTop()
	                }
					if (that.list.length === 0) {
	                    that.dataText = lc('wap_js_00113');
	                }
				})
			},

			// 批量操作
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
					message.warning(lc('admin_user_weipin_00001'));
					return false;
				}

				let idArr = [];
				this.multipleSelection.forEach(function (item) {
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

				if (typeof idx == 'undefined') { // {{ lc('member_com_00055') }}
					params.del = this.idArr;
					msg = lc('common_00853');
				}  else {// {{ lc('common_01711') }}
					params.del = that.list[idx].id;
					msg = lc('admin_00333');
				}

				delConfirm(this, params, function (params) {
					httpPost('m=user&c=users_trust&a=del', params).then(function(res) {
						if (res.data.error > 0) {
							message.error(res.data.msg);
						} else {
							that.getList();
							that.$refs.multipleTable.clearSelection();
							message.success(res.data.msg);
						}
					})
				}, msg)
			},

			// Audit
			openAudit(row) {
				this.ruleFormAudit = {
					id: row.id,
					status: row.status,
				};
				this.dialogAudit = true;
			},
			submitAudit() {
				let that = this,
					params = that.ruleFormAudit;

				if (params.status === '0') {
					message.warning(lc('admin_user_weipin_00015'));
					return false;
				}

				if (that.saveLoading) {
					return false;
				}
				that.saveLoading = true;

				httpPost('m=user&c=users_trust&a=status', params).then(function(res) {
					that.saveLoading = false;
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						that.dialogAudit = false;
						that.getList();
						message.success(res.data.msg);
					}
				})
			},

			// 预览简历
			openPreview(row) {
				this.detail = row;
				this.drawerPreview = true;
			},

			// 匹配岗位
			openRecom(row) {
				this.detail = row;
				this.drawerRecom = true;
			},
		},
	};
</script>
<style scoped>


</style>
