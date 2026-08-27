<template>
	<div class="moduleElHight">
		<div class="moduleSeachbig">
			<!-- Keyword search and filters -->
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-input v-model="searchForm.keyword" @keyup.enter="search" :placeholder="lc('admin_user_weipin_00003')" size="small"
						  prefix-icon="el-icon-search" clearable>
					<template #prepend><el-select v-model="searchForm.type" size="small" :placeholder="lc('wap_00529')">
						<el-option :label="lc('wap_00529')" :value="1"></el-option>
						<el-option :label="lc('wap_user_00015')" :value="2"></el-option>
					</el-select></template>
				</el-input>
			</div>
			<div v-for="(searchItem, searchIndex) in searchList" :key="searchIndex" class="tableSeachInpt tableSeachInptsmall">
				<el-select v-model="searchForm[searchItem.param]" :clearable="true" :placeholder="searchItem.name" size="small" @change="search">
					<el-option v-for="(searchLabel, searchValue) in searchItem.value" :key="searchValue" :label="searchLabel" :value="searchValue"></el-option>
				</el-select>
			</div>
			<div class="tableSeachInpt">
				<el-button type="primary" icon="el-icon-search" size="small" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
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
				<template #empty>
					<p>{{dataText}}</p>
				</template>
				<el-table-column type="selection" width="55"> </el-table-column>
				<el-table-column prop="uid" :label="lc('member_com_00345')" width="80" sortable="custom"> </el-table-column>
				<el-table-column prop="uname" :label="lc('wap_00529')" width="100"> </el-table-column>
				<el-table-column :label="lc('admin_vue_00017')" width=" ">
					<template #default="scope">
						<div>
							<span v-if="scope.row.name">{{scope.row.name}}</span>
							<span v-else style="color: #FF0000;">{{ lc('member_com_00211') }}</span>
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="price" :label="lc('wap_00563')"> </el-table-column>
				<el-table-column prop="add_time" :label="lc('wap_com_00342')" width="150" sortable="custom">
					<template #default="scope">
						<div>{{scope.row.add_time_n}}</div>
					</template>
				</el-table-column>
				<el-table-column prop="" :label="lc('admin_00488')" width="150" align="center">
					<template #default="scope">
						<el-button v-if="scope.row.name && scope.row.status == 1" type="text" size="small" plain
								   @click="openRecom(scope.row)">
							<i class="el-icon-search"></i> {{ lc('admin_00488') }}
						</el-button>
						<div v-else>-</div>
					</template>
				</el-table-column>
				<el-table-column :label="lc('member_user_00181')" width="60" fixed="right">
					<template #default="scope">
						<div class="admin_state">
							<span v-if="scope.row.status == 1" class="admin_state1">{{ lc('wap_com_00191') }}</span>
					        <span v-else-if="scope.row.status == 2" class="admin_state2">{{ lc('admin_user_00234') }}</span>
							<span v-else class="admin_state5">{{ lc('wap_user_00166') }}</span>
						</div>
					</template>
				</el-table-column>
				<el-table-column :label="lc('member_user_00048')" width="190" fixed="right" align="center">
					<template #default="scope">
						<div class="cz_button">
							<template v-if="scope.row.name">
								<el-button plain @click="openPreview(scope.row)">{{ lc('wap_00071') }}</el-button>
								<el-button v-if="scope.row.status == 0" size="small" plain @click="openAudit(scope.row)">{{ lc('member_user_00152') }}</el-button>
							</template>
							<el-button v-if="scope.row.status != 0" type="danger" size="small" @click="del(scope.$index)">{{ lc('common.delete') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
       	<div class="modulePaging">
			<div>
				<el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
							 @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
				<el-button @click="batch('del')" size="small">{{ lc('member_com_00055') }}</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
							   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
							   layout="total, sizes, prev, pager, next, jumper" :total="total">
				</el-pagination>
			</div>
		</div>
		 <!-- Review -->
		<div class="modluDrawer">
			<el-dialog :title="lc('admin_00489')" v-model="dialogAudit" :with-header="true" :modal-append-to-body="false"
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
				<template #footer><span class="dialog-footer">
					<el-button @click="dialogAudit = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" @click="submitAudit">{{ lc('wap_com_00019') }}</el-button>
				</span></template>
			</el-dialog>
		</div>

		<div class="modluDrawer">
			<!-- Resume preview -->
			<el-drawer :title="lc('wap_user_00217')" v-model="drawerPreview" append-to-body size="60%">
				<preview :id="detail.eid"></preview>
			</el-drawer>
			<!-- Matching jobs -->
			<el-drawer :title="lc('admin_00488')" :append-to-body="true" v-model="drawerRecom" :show-close="true"
					   :with-header="true" size="80%">
				<recom :id="detail.id" :eid="detail.eid"></recom>
			</el-drawer>
		</div>
	</div>
</template>

<script>
import ResumePreview from '../../../component/resume_preview.vue'
import ResumeTrustRecom from './resume_trust_recom.vue'

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
		props: {
			status: {type: String, default: ''}
		},
		data: function() {
			return {
				loading: false,
				dataText: lc('admin_user_weipin_00026'),
				tableHig: true,
				saveLoading: false,

				// Search filters
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

				// List sorting
				t: '',
				order: '',

				checkedAll: false, // {{ lc('wap_js_00074') }}
				checkedAllIndeterminate: false,
				multipleSelection: [], // Multi-select value storage
				idArr: [],

				detail: {}, // Single record data

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
			'preview': ResumePreview,
			'recom': ResumeTrustRecom,
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
						that.limit = parseInt(data.limit); // Use default count from system config
					}
					if (that.page > data.page) {
						that.page = parseInt(data.page); // Use latest page after the last page is deleted
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

			// Batch operation
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

			// Preview resume
			openPreview(row) {
				this.detail = row;
				this.drawerPreview = true;
			},

			// Matching jobs
			openRecom(row) {
				this.detail = row;
				this.drawerRecom = true;
			},
		},
	};
</script>
<style scoped>


</style>
