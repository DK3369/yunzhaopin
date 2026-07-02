<template>
	<div class="moduleElHight">
		<div class="moduleSeachbig">
			<!-- Keyword search and filters -->
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-input v-model="searchForm.keyword" :placeholder="lc('admin_user_weipin_00003')" size="small"
						  prefix-icon="el-icon-search" clearable>
					<el-select v-model="searchForm.type" size="small" slot="prepend" :placeholder="lc('admin_00490')">
						<el-option :label="lc('admin_00490')" :value="1"></el-option>
						<el-option :label="lc('admin_00491')" :value="2"></el-option>
					</el-select>
				</el-input>
			</div>
			<div class="tableSeachInpt">
				<el-button type="primary" icon="el-icon-search" size="mini" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
			</div>
			 
		</div>
		<div class="moduleElTable"
			style="border: 1px solid #ebeef5; width: calc(100% - 2px); height: calc(100% - 105px);">
			<el-table :data="list" style="width: 100%" stripe ref="multipleTable" @selection-change="handleSelectionChange"
					  @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
				<template slot="empty">
					<p>{{dataText}}</p>
				</template>
				<el-table-column type="selection" width="55"> </el-table-column>
				<el-table-column prop="id" :label="lc('admin_00492')" width="100" sortable="custom"> </el-table-column>
				<el-table-column prop="com_name" :label="lc('wap_01403')"> </el-table-column>
				<el-table-column :label="lc('wap_com_00288')">
					<template slot-scope="scope">
						<div>
							<el-link type="primary" :underline="false" @click="openPage(scope.row.job_comapply)">{{scope.row.name}}</el-link>
						</div>
					</template>
				</el-table-column>
				<el-table-column :label="lc('wap_00349')">
					<template slot-scope="scope">
						<div>
							{{scope.row.job_city_one}} - {{scope.row.job_city_two}}
						</div>
					</template>
				</el-table-column>
				<el-table-column :label="lc('wap_user_00018')">
					<template slot-scope="scope">
						<div>
							{{scope.row.job_three_n}}
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="job_salary" :label="lc('admin_00493')"></el-table-column>
				<el-table-column :label="lc('member_user_00048')" width="80" align="center">
					<template slot-scope="scope">
						<div class="cz_button">
							<el-button plain @click="rec(scope.$index)">{{ lc('common.recommended') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
       	<div class="modulePaging">
			<div>
				<el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
							 @change="checkAll">{{ lc('wap_js_00074') }}</el-checkbox>
				<el-button @click="batch('rec')" size="mini">{{ lc('admin_user_00237') }}</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
							   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
							   layout="total, sizes, prev, pager, next, jumper" :total="total">
				</el-pagination>
			</div>
		</div>

		<div class="modluDrawer">
			<!-- Recommendation send progress -->
			<el-dialog :title="lc('admin_00494')" :visible.sync="dialogSend" center append-to-body width="12%">
				<div style="text-align:center;">
					<el-progress type="circle" :percentage="sendPercentage" :format="formatSend"></el-progress>
				</div>
			</el-dialog>
		</div>
	</div>
</template>

<script>
	module.exports = {
		props: {
			id: String,
			eid: String
		},
		data: function() {
			return {
				loading: false,
				dataText: lc('admin_user_weipin_00026'),
				tableHig: true,
				saveLoading: false,

				// Search filters
				searchForm: {
					type: 1
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

				// Send progress dialog
				dialogSend: false,
				sendNum: 0,
				sendPercentage: 0,

				prevPage: 0
			}
		},

		mounted() {

		},
		created() {
			this.init();
		},
		methods: {
			init() {
				this.resetSearch();
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

			search() {
				this.page = 1;
				this.getList();
			},
			getList() {
				let that = this,
					searchForm = that.searchForm,
					params = {
						id: that.id,
						eid: that.eid,
						page: that.page,
						limit: that.limit,
						t: that.t,
						order: that.order,
					};
					that.loading = true;

				httpPost('m=user&c=users_trust&a=recom', { ...params, ...searchForm }).then(function (response) {
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
					message.error(lc('admin_user_weipin_00001'));
					return false;
				}

				let idArr = [];
				this.multipleSelection.forEach(function (item) {
					idArr.push(item.id);
				})
				this.idArr = idArr;

				if (type == 'rec') {
					this.rec();
				}
			},
			checkAll(val) {
				val ? this.checkedAllIndeterminate = false : '';
				this.$refs.multipleTable.toggleAllSelection();
			},

			// recommend
			rec(idx) {
				let that = this;

				if (that.saveLoading) {
					return false;
				}

				delConfirm(this, {}, function (params) {
				    that.saveLoading = true;
					that.sendHandle(idx)
				}, lc('admin_vue_00018'))
			},
			async sendHandle(idx) {
				let that = this,
					params = {
						eid: that.eid
					};
				if (typeof idx == 'undefined') { // Batch send
					that.dialogSend = true;
					let list = that.multipleSelection;
					for(let i = 0; i < list.length; i++) { // Send in loop
						params.comid = list[i].uid;
						params.jobid = list[i].id;
						await that.sendEmail(2, params);
					}
					that.$refs.multipleTable.clearSelection();
				} else {// Single send
					let row = that.list[idx];
					params.comid = row.uid;
					params.jobid = row.id;
					await that.sendEmail(1, params);
				}
				that.getList();
				that.saveLoading = false;
			},
			// Progress formatting method
			formatSend(percentage) {
				let that = this;
				if (percentage === 100) {
					setTimeout(function() {
						that.dialogSend = false;
						that.sendNum = 0;
						that.sendPercentage = 0;
					}, 1500);
					return lc('admin_system_00018');
				} else {
					return lc('admin_00872') + that.sendNum + '/' + that.idArr.length;
				}
			},
			async sendEmail(type, params) {
				let that = this;
				let res = await httpPost('m=user&c=users_trust&a=directrecom', params, {hideloading: type == 2 ? true : false});
				if (res.data.error > 0) {
					message.error(res.data.msg);
				} else {
					if (type == 1) {
						message.success(res.data.msg);
					} else {
						that.sendNum++;
						that.sendPercentage = accMul(accDiv(that.sendNum, that.idArr.length), 100); // Calculate percentage progress
					}
				}
			},

			openPage(url) {
				window.open(url);
			},
		},
		watch: {
			id: function (val, oldVal) {
				this.ruleForm = {};
				this.init();
			}
		}
	};
</script>
<style>

.el-drawer__body {
	padding: 0px 20px;
}
</style>
