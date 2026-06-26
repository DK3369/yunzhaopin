<template>
	<div class="moduleElHight">
		<div class="moduleSeachbig">
			<!--关键字搜索和查询在一起-->
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-input v-model="searchForm.keyword" placeholder="{yun:}t key='admin_user_weipin_00003'{/yun}" size="small"
						  prefix-icon="el-icon-search" clearable>
					<el-select v-model="searchForm.type" size="small" slot="prepend" placeholder="{yun:}t key='admin_00490'{/yun}">
						<el-option label="公司名" :value="1"></el-option>
						<el-option label="职位名" :value="2"></el-option>
					</el-select>
				</el-input>
			</div>
			<div class="tableSeachInpt">
				<el-button type="primary" icon="el-icon-search" size="mini" @click="search">{yun:}t key='admin_user_weipin_00049'{/yun}</el-button>
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
				<el-table-column prop="id" label="职位编号" width="100" sortable="custom"> </el-table-column>
				<el-table-column prop="com_name" label="公司名称"> </el-table-column>
				<el-table-column label="职位名称">
					<template slot-scope="scope">
						<div>
							<el-link type="primary" :underline="false" @click="openPage(scope.row.job_comapply)">{{scope.row.name}}</el-link>
						</div>
					</template>
				</el-table-column>
				<el-table-column label="工作地区">
					<template slot-scope="scope">
						<div>
							{{scope.row.job_city_one}} - {{scope.row.job_city_two}}
						</div>
					</template>
				</el-table-column>
				<el-table-column label="职位类别">
					<template slot-scope="scope">
						<div>
							{{scope.row.job_three_n}}
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="job_salary" label="待遇"></el-table-column>
				<el-table-column label="操作" width="80" align="center">
					<template slot-scope="scope">
						<div class="cz_button">
							<el-button plain @click="rec(scope.$index)">{yun:}t key='common.recommended'{/yun}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
       	<div class="modulePaging">
			<div>
				<el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
							 @change="checkAll">{yun:}t key='wap_js_00074'{/yun}</el-checkbox>
				<el-button @click="batch('rec')" size="mini">{yun:}t key='admin_user_00237'{/yun}</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
							   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
							   layout="total, sizes, prev, pager, next, jumper" :total="total">
				</el-pagination>
			</div>
		</div>

		<div class="modluDrawer">
			<!--推荐发送进度-->
			<el-dialog title="{yun:}t key='admin_00494'{/yun}" :visible.sync="dialogSend" center append-to-body width="12%">
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
				dataText: "{yun:}t key='admin_user_weipin_00026'{/yun}",
				tableHig: true,
				saveLoading: false,

				// 搜索筛选项
				searchForm: {
					type: 1
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

				checkedAll: false, // {yun:}t key='wap_js_00074'{/yun}
				checkedAllIndeterminate: false,
				multipleSelection: [], // 多选值存储
				idArr: [],

				// 发送进度弹窗
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
				this.t = event.order ? event.prop : "';
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
						that.limit = parseInt(data.limit); // 取系统配置默认数量
					}
					if (that.page > data.page) {
						that.page = parseInt(data.page); // 最后一页被删除后，取最新的页数
					}
					that.loading = false;
					if(that.prevPage != that.page){
	                    that.prevPage = that.page;
	                    that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
	                }
					if (that.list.length === 0) {
	                    that.dataText = "{yun:}t key='wap_js_00113'{/yun}";
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
					message.error("{yun:}t key='admin_user_weipin_00001'{/yun}");
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
				}, '确定要推荐吗？')
			},
			async sendHandle(idx) {
				let that = this,
					params = {
						eid: that.eid
					};
				if (typeof idx == 'undefined') { // 批量发送
					that.dialogSend = true;
					let list = that.multipleSelection;
					for(let i = 0; i < list.length; i++) { // 循环发送
						params.comid = list[i].uid;
						params.jobid = list[i].id;
						await that.sendEmail(2, params);
					}
					that.$refs.multipleTable.clearSelection();
				} else {// 单个发送
					let row = that.list[idx];
					params.comid = row.uid;
					params.jobid = row.id;
					await that.sendEmail(1, params);
				}
				that.getList();
				that.saveLoading = false;
			},
			// 进度格式化方法
			formatSend(percentage) {
				let that = this;
				if (percentage === 100) {
					setTimeout(function() {
						that.dialogSend = false;
						that.sendNum = 0;
						that.sendPercentage = 0;
					}, 1500);
					return '发送完成';
				} else {
					return "{yun:}t key='admin_00872'{/yun}" + that.sendNum + '/' + that.idArr.length;
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
						that.sendPercentage = accMul(accDiv(that.sendNum, that.idArr.length), 100); // 计算百分比进度
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
