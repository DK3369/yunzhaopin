<template>
<div id="daohaapp" class="moduleElenAl">
		<div class="moduleSeachs">
			<div class="moduleSeachleft">
				<div class="moduleInptList">
					<el-input :placeholder="lc('admin_user_weipin_00003')" v-model="search.keyword" size="small" class="input-with-select" clearable>
						<template #prepend><el-select v-model="search.type" :placeholder="lc('wap_user_00100')" >
							<el-option :label="lc('admin_user_00295')" value="1"></el-option>
							<el-option :label="lc('admin_user_00140')" value="2"></el-option>
							<el-option :label="lc('admin_user_00290')" value="3"></el-option>
						</el-select></template>
					</el-input>
				</div>
				<div class=" tableSeachInptsmall newsinput">
					<el-select v-model="search.pay_state" size="small" :placeholder="lc('admin_yunying_00097')" clearable
						@change="doUserQuery">
						<el-option v-for="item in payArr" :key="item.value" :label="item.label" :value="item.value"></el-option>
					</el-select>
				</div>
				<div class=" tableSeachInptsmall newsinput">
					<el-select v-model="search.end" size="small" :placeholder="lc('wap_com_00344')" clearable
						@change="doUserQuery">
						<el-option v-for="item in time" :key="item.value" :label="item.label" :value="item.value"></el-option>
					</el-select>
				</div>
				<div class="newsbtnbox">
					<el-button type="primary" icon="el-icon-search" size="small" @click="doUserQuery">{{ lc('admin_user_weipin_00049') }}</el-button>
				</div>
			</div>
		</div>

		<div class="moduleElTable">
			<el-table :data="tableData" border style="width: 100%"
					  @selection-change="selectChange"
					  @sort-change="shortChange"
					  ref="multipleTable"
				:header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" v-loading="loading" :empty-text="emptytext">
				<el-table-column type="selection" width="55">
				</el-table-column>
				<el-table-column prop="id" :label="lc('member_com_00345')" width="80" sortable="custom">
				</el-table-column>
				<el-table-column prop="order_id" :label="lc('admin_user_00295')" width="160">
				</el-table-column>
				<el-table-column prop="username" :label="lc('admin_user_00140')" min-width="160"></el-table-column>
				<el-table-column prop="comname" :label="lc('admin_yunying_00096')" min-width="160"></el-table-column>
				<el-table-column prop="price_str" :label="lc('member_user_00254')" width="150"></el-table-column>
				<el-table-column prop="pay_remark" :label="lc('admin_user_00290')">
				</el-table-column>
				<el-table-column prop="pay_time" :label="lc('wap_com_00344')" width="180" align="center" sortable="custom">
				</el-table-column>
				<el-table-column prop="pay_state_n" :label="lc('member_user_00181')" width="150" align="center">
					<template #default="scope">
						<div v-html="scope.row.pay_state_n"></div>
					</template>
				</el-table-column>
				<el-table-column fixed="right" :label="lc('member_user_00048')" width="80" align="center">
					<template #default="scope">
						<div class="cz_button">
							<el-button type="danger" size="small " @click="del(scope.row)">{{ lc('wap_js_00077') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
		<div class="modulePaging">
			<div class="modulecz modulePagButn">
				<el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
				<el-button @click="batchDel" size="small">{{ lc('member_com_00055') }}</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination :total="total" @current-change="userPageChange"
							   :page-sizes="pageSizes"
							   :page-size="pageSize" @size-change="userPageSizeChange"
					v-model:current-page="page" layout="total, sizes, prev, pager, next, jumper">
				</el-pagination>
			</div>
		</div>
	</div>
</template>

<script>
import Navxiugai from './component/navxiugai.vue'

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
					emptytext: lc('wap_js_00113'),
					checkedAll:false,
					search: {
						pay_state: '',
						type: '1',
						searchVal: '',
						end: ''
					},
					select: '',
					value: true,
					time: [{
						value: 1,
						label: lc('common_01940')
					}, {
						value: 3,
						label: lc('admin_user_00179')
					}, {
						value: 7,
						label: lc('admin_user_00178')
					}, {
						value: 15,
						label: lc('admin_user_00180')
					}, {
						value: 30,
						label: lc('admin_user_00175')
					}],
					payArr: [{
						value: 0,
						label: lc('admin_01264')
					}, {
						value: 1,
						label: lc('admin_yunying_00085')
					}, {
						value: 2,
						label: lc('admin_01265')
					}, {
						value: 3,
						label: lc('admin_yunying_00086')
					}],
					tableData: [],
					total: 0,
					page: 1,
					idsArr: [],
					pageSize: 0,
					pageSizes:[],
					uri: "m=yunying&c=",
					prevPage:0
				}
			},
			components: {
				// 'navxiugai': Navxiugai,
			},
			created() {
				this.getList();


			},
			methods: {
				doUserQuery() {
					this.page = 1
					this.getList()
				},
				userPageChange(val) {
					this.page = val
					this.getList()
				},
				userPageSizeChange(val) {
					this.pageSize = val
					this.getList()
				},
				selectChange: function (val) {
					this.idsArr = [];
					let _this = this;
					if (val.length) {
						val.forEach(item => {
							_this.idsArr.push(item.id);
						});
					}
					if (_this.idsArr.length == 0) {
						_this.checkedAll = false;
					} else {
						if (_this.idsArr.length == _this.tableData.length) {
							_this.checkedAll = true;
						} else {
							_this.checkedAll = false;
						}
					}
				},
				getList() {
					let _this = this;
					let url = _this.uri + 'finance_company_pay&a=index';
					this.search.page = this.page;
					this.search.pageSize = this.pageSize;
					_this.loading = true;
					_this.emptytext = lc('admin_user_weipin_00026');
					httpPost(url, _this.search, {hideloading: true}).then(function (response) {
						let res = response.data;
						if (res.error == 0) {
							_this.tableData = res.data.data;
							_this.total = res.data.total;
							if(_this.prevPage != _this.page){
								_this.prevPage = _this.page;
								_this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
							}
							_this.loading = false;
							_this.pageSizes =res.data.pageSizes;
							if (_this.tableData.length === 0){
	                            _this.emptytext = lc('wap_js_00113');
	                        }
						}
					})
				},
				handleSizeChange(val) {
					console.log(`Page size: ${val}`);
				},
				handleCurrentChange(val) {
					console.log(`Current page: ${val}`);
				},
				del: function ($row) {

					let url = this.uri + 'finance_company_pay&a=del'
					let _this = this;
					delConfirm(this, { id: $row.id }, function (params) {
						httpPost(url, params).then(function(response) {
							let res = response.data;
							if (res.error == 0) {
								message.success(res.msg, _this.getList());
							} else {
								message.error(res.msg);
							}
						})
					}, lc('admin_01269'))
				},
				batchDel: function () {
					let ids = this.idsArr;
					if (!ids.length) {
						message.error(lc('admin_01267'));
						return
					}
					let _this = this;
					let url = this.uri + 'finance_company_pay&a=del'
					delConfirm(this, { del: ids }, function (params) {
						httpPost(url, params).then(function(response) {
							let res = response.data;
							if (res.error == 0) {
								message.success(res.msg, _this.getList());
							} else {
								message.error(res.msg);
							}
						})
					}, lc('admin_yunying_00074'))
				},
				selectAllBottom:function (value) {
					value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
				},
				shortChange(e) {
					let orderMap = {ascending: 'asc', descending: 'desc'}
					this.search.t = e.order ? e.prop : null;
					this.search.order = orderMap[e.order];
					this.page = 1;
					this.getList();
				},
			}
		}
</script>
