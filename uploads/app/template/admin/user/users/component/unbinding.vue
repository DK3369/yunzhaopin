<template>
	<div class="moduleElHight">
		<div class="moduleElSearchInf">
			<div class="moduleElTabInpt" style="flex-wrap: wrap;">
				<div class="moduleInptList moduleInptWidt">
					<el-input placeholder="输入你要搜索的关键字" @keyup.enter.native="search" size="small" v-model="searchForm.keyword" clearable>
						<el-select v-model="searchForm.type" slot="prepend" placeholder="请选择">
							<el-option label="用户名" :value="1"></el-option>
							<el-option label="内容" :value="2"></el-option>
						</el-select>
					</el-input>
				</div>
				<div class="moduleInptList">
					<el-date-picker v-model="daterange" size="small" type="daterange" range-separator="至" start-placeholder="开始日期"
									end-placeholder="结束日期" style="width: 280px;" @change="search">
					</el-date-picker>
				</div>
				<div class="moduleInptList">
					<el-button type="primary" icon="el-icon-search" size="mini" @click="search">查询</el-button>
				</div>
			</div>
		</div>
		<div class="moduleElTable" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
			<el-table :data="list" stripe style="width: 100%" ref="multipleTable" @selection-change="handleSelectionChange"
					  @sort-change="sortChange" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading">
				<template slot="empty">
					<p>{{dataText}}</p>
				</template>
				<el-table-column type="selection" width="55">
				</el-table-column>
				<el-table-column prop="id" label="编号" width="120" sortable="custom">
				</el-table-column>
				<el-table-column prop="username" label="用户名">
				</el-table-column>
				<el-table-column prop="content" label="内容" min-width="220">
				</el-table-column>
				<el-table-column prop="ip" label="IP">
				</el-table-column>
				<el-table-column prop="ctime_n" label="时间">
				</el-table-column>
				<el-table-column fixed="right" label="操作" width="90">
					<template slot-scope="scope">
						<div class="moduleElTaCaoz">
							<el-button type="danger" size="mini" @click="del(scope.$index)">删除</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
		<div class="modulePaging">
			<div>
				<el-checkbox v-model="checkedAll" :indeterminate="checkedAllIndeterminate"
							 @change="checkAll">全选</el-checkbox>
				<el-button @click="batch('del')" size="mini">批量删除</el-button>
				<el-button @click="del('all')" size="mini">一键删除</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
							   :current-page="page" :page-sizes="pageSizes" :page-size="limit"
							   layout="total, sizes, prev, pager, next, jumper" :total="total">
				</el-pagination>
			</div>
		</div>
	</div>
</template>

<script>
	module.exports = {
		data: function () {
			return {
				loading: false,
				dataText: '数据加载中',
				// 日期选择
				daterange: '',

				// 搜索筛选项
				searchForm: {
					type: 1
				},

				// 列表
				page: 1,
				limit: 0,
				list: [],
				total: 0,
				pageSizes: [],

				// 列表排序
				t: '',
				order: '',

				checkedAll: false, // 全选
				checkedAllIndeterminate: false,
				multipleSelection: [], // 多选值存储
				idArr: [],

				prevPage: 0
			}
		},

		mounted() {

		},
		created() {
			var that = this;
			let params = window.parent.homeapp.$route.params;
			let query = window.parent.homeapp.$route.query;
			
			if (!$.isEmptyObject(query)) {
				params = {...query,...params};
			}
			
			if (!$.isEmptyObject(params)) {
				delete params.activeName;
				this.getParams(params);
			}
			this.init();
		},
		methods: {
			init() {
				this.resetSearch();
				this.search();
			},
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
			search() {
				this.page = 1;
				let daterange = this.daterange;
				if (daterange) {
					this.searchForm.time_start = formatDate(daterange[0]);
					this.searchForm.time_end = formatDate(daterange[1]);
				} else {
					this.searchForm.time_start = '';
					this.searchForm.time_end = '';
				}
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
				httpPost('m=user&c=users_member&a=writtenOffLog', { ...params, ...searchForm }, {hideloading: true}).then(function (response) {
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
	                    that.dataText = "暂无数据";
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
				if (this.multipleSelection.length == 0 && type == 'del') {
	                message.error('请选择要删除的数据');
	                return false;
	            }else if(this.multipleSelection.length == 0){
	                message.error('请选择要操作的数据项');
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

				if (typeof idx == 'undefined') { // 批量删除
					params.del = this.idArr;
					msg = '你确定要删除选中项吗？';
				} else if (idx == 'all') { // 一键删除
					params.del = 'all';
					msg = '确定要清空用户解绑日志？';
				} else {// 单个删除
					params.del = that.list[idx].id;
					msg = '你确定要删除当前项吗？';
				}

				delConfirm(this, params, function (params) {
					httpPost('m=user&c=users_member&a=delwflog', params).then(function(res) {
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
		},
	};
</script>
<style scoped>
	.el-input-group__prepend{
		 	   background-color: #ffffff;
		 	   padding: 0 20px;
		 }
	.el-button--primary.is-plain {
		background: #f4f4f5;
		border: none;
	}

	.el-button--primary.is-plain:hover {
		color: #409eff;
	}

	.moduleElTaCaoz {
		justify-content: space-between;
	}

	.moduleElTaCaoz .el-button {
		/* width: 60px; */
		margin: 3px 0;
	}
</style>