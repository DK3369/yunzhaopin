<template>
<div id="daohaapp" class="moduleElenAl">
		<div class="moduleSeachmore">
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-select v-model="searchOption.ctime" size="small" clearable @change="search" :placeholder="lc('admin_user_weipin_00030')">
					<el-option :label="lc('common_01940')" value="1"></el-option>
					<el-option :label="lc('admin_user_00179')" value="3"></el-option>
					<el-option :label="lc('admin_user_00178')" value="7"></el-option>
					<el-option :label="lc('admin_user_00180')" value="15"></el-option>
					<el-option :label="lc('admin_user_00175')" value="30"></el-option>
				</el-select>
			</div>
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-select v-model="searchOption.type" size="small" clearable @change="search"
					:placeholder="lc('admin_system_00689')">
					<el-option :label="lc('admin_system_00432')" value="1"></el-option>
					<el-option :label="lc('wap_js_00081')" value="2"></el-option>
				</el-select>
			</div>
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-select v-model="searchOption.did" size="small" clearable @change="search"
					:placeholder="lc('admin_user_weipin_00050')">
					<el-option v-for="item in domainData" :key="item.value" :label="item.label" :value="item.value"></el-option>
				</el-select>
			</div>
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-select v-model="searchOption.state" size="small" clearable @change="search"
					:placeholder="lc('wap_com_00406')">
					<el-option :label="lc('wap_user_00165')" value="1"></el-option>
					<el-option :label="lc('wap_user_00166')" value="2"></el-option>
				</el-select>
			</div>
			<div class="tableSeachInpt">
				<el-input :placeholder="lc('admin_00340')" size="small" v-model="searchOption.keyword" clearable
					prefix-icon="el-icon-search">
				</el-input>
			</div>
			<div class="tableSeachInpt tableSeachbutton">
				<el-button type="primary" size="small" icon="el-icon-search" @click="search">{{ lc('admin_user_weipin_00049') }}</el-button>
			</div>
			<div class="tableSeachInpt tableSeachbutton" style="margin-left: auto;">
				<el-button type="primary" icon="el-icon-document-add" size="small" @click="addlink">{{ lc('admin_system_00430') }}</el-button>
			</div>
		</div>

		<div class="moduleElTable">
			<el-table :data="tableData" border style="width: 100%"
				:header-cell-style="{background:'#f5f7fa',color:'#606266'}" height="100%" @sort-change="shortChange"
				@selection-change="handleSelectionChange" ref="multipleTable" v-loading="loading" :empty-text="emptytext">
				<el-table-column type="selection" width="55">
				</el-table-column>
				<el-table-column prop="id" :label="lc('common_02108')" width="80" sortable="custom">
				</el-table-column>
				<el-table-column prop="link_name" :label="lc('admin_system_00292')" width="180">
				</el-table-column>
				<el-table-column prop="link_url" :label="lc('admin_00101')">
					<template #default="scope">
						<a target="_blank" :href="scope.row.link_url">{{scope.row.link_url}}</a>
					</template>
				</el-table-column>
				<el-table-column prop="did_n" :label="lc('admin_system_00291')" width="110">
				</el-table-column>
				<el-table-column prop="ctime_n" :label="lc('admin_user_weipin_00030')" width="110">
				</el-table-column>
				<el-table-column prop="link_type_n" :label="lc('admin_system_00689')" width="110">
				</el-table-column>
				<el-table-column prop="link_sorting" :label="lc('admin_vue_00044')" width="60">
				</el-table-column>
				<el-table-column :label="lc('member_user_00181')" width="100">
					<template #default="scope">
						<el-tag type="success" size="small" v-if="scope.row.link_state==1">{{ lc('wap_user_00165') }}</el-tag>
						<el-tag type="danger" size="small" v-else>{{ lc('wap_user_00166') }}</el-tag>
					</template>
				</el-table-column>
				<el-table-column fixed="right" :label="lc('member_user_00048')" width="200">
					<template #default="scope">
						<div class="cz_button">
							<el-button size="small" @click="status(scope.row)">{{ lc('member_user_00152') }}</el-button>
							<el-button size="small" @click="eidtlink(scope.row)">{{ lc('wap_js_00073') }}</el-button>
							<el-button size="small" type="danger" @click="delrow(scope.row)">{{ lc('wap_js_00077') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>
		<div class="modulePaging">
			<div class="modulecz modulePagButn">
				<el-checkbox v-model="checkedAll" @change="selectAllBottom">{{ lc('wap_js_00074') }}</el-checkbox>
				<el-button size="small" @click="delAllBottom">{{ lc('member_com_00055') }}</el-button>
				<el-button size="small" @click="setDidAllBottom">{{ lc('admin_user_00279') }}</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
					:current-page="pagination.page" :page-sizes="pagination.pageSizeOption"
					:page-size="pagination.pageSize" layout="total, sizes, prev, pager, next, jumper"
					:total="pagination.total">
				</el-pagination>
			</div>
		</div>
		<!-- 处理弹窗 -->
		<div class="modluDrawer">
			<el-dialog :title="lc('admin_system_00429')" v-model="statusBox" :with-header="true" :modal-append-to-body="false"
				:show-close="true" width="400px">
				<el-form :model="ruleFormStatus" ref="ruleFormStatus" label-width="90px">
					<el-form-item :label="lc('wap_com_00406')">
						<el-radio-group v-model="ruleFormStatus.status">
							<el-radio label="1">{{ lc('wap_user_00165') }}</el-radio>
						</el-radio-group>
					</el-form-item>
					<el-form-item :label="lc('member_user_00062')" prop="content">
						<el-input type="textarea" :rows="2" v-model="ruleFormStatus.content"></el-input>
					</el-form-item>
				</el-form>
				<template #footer><span class="dialog-footer">
					<el-button @click="statusBox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" @click="submitForm">{{ lc('wap_com_00019') }}</el-button>
				</span></template>
			</el-dialog>
		</div>
		<div class="modluDrawer">
			<el-dialog :title="lc('admin_user_weipin_00029')" v-model="setDidBox" :with-header="true" :modal-append-to-body="false"
				:show-close="true" width="400px">
				<el-form ref="ruleFormDid" label-width="90px">
					<el-form-item :label="lc('admin_user_company_00137')" prop="content">
						<el-select v-model="setdid" filterable>
							<el-option v-for="item in domainData" :key="item.value" :label="item.label" :value="item.value"></el-option>
						</el-select>
					</el-form-item>
				</el-form>
				<template #footer><span class="dialog-footer">
					<el-button @click="setDidBox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" @click="submitFormDid">{{ lc('admin_system_00431') }}</el-button>
				</span></template>
			</el-dialog>
		</div>
		<div class="modluDrawer">
			<el-drawer :title="title" v-model="showLinkBox" :modal-append-to-body="false" size="50%">
				<friendlink-add :link_id="link_id" @child-event="closeLinkBox" v-if="showLinkBox"></friendlink-add>
			</el-drawer>
		</div>
		
	</div>
</template>

<script>
import FriendlinkAdd from './component/friendlinkAdd.vue'

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
					emptytext: lc('wap_js_00113'),
					searchForm: [],
					setdid: 0,
					setDidBox: false,
					statusBox: false,
					link_id: 0,
					title: lc('admin_system_00430'),
					showLinkBox: false,
					drawer: false,
					tableData: [],
					searchOption: {
						type: '',
						ctime: '',
						did: '',
						state: '',
						keyword: ''
					},
					pagination: {
						page: 1,
						// pageSize: 10,
						total: 0,
						pageSize: 0,
						pageSizeOption: [10, 20, 50, 100]
					},
					prevPage: 0,
					checkedAll: false,
					selectedItem: [],
					domainData: [],
					ruleFormStatus: {
						status: '',
						content: '',
						id: ''
					},
					loading: false,
				}
			},
			components: {
				'friendlink-add': FriendlinkAdd,
			},
			created() {
				var that = this
				let query = window.parent.homeapp.$route.query;


				if (query.state) {
					that.searchOption.state = query.state;
				}
				this.getList();
			},
			methods: {
				shortChange(e) {
				    let orderMap = {ascending: 'asc', descending: 'desc'}
				    this.searchForm.t = e.order ? e.prop : null;
				    this.searchForm.order = orderMap[e.order];
				    this.search();
				},
				async submitFormDid() {
					let that = this;
					let params = {
						did: that.setdid,
						id: that.selectedItem
					};

					httpPost('m=system&c=set_friendlink&a=sitedid', params).then(function (response) {
						let data = response.data;
						if (data.error == 0) {
							that.setDidBox = false;
							message.success(data.msg, function (e) {
								that.getList();
							});
						} else {
							message.error(data.msg);
						}
					}).catch(function (error) {
						console.log(error);
					})
				},
				setDidAllBottom() {
					if (!this.selectedItem.length) {
						this.$message.error(lc('admin_system_00427'));
						return false;
					}
					this.setdid = 0;
					this.setDidBox = true;
				},
				eidtlink(row) {
					this.link_id = parseInt(row.id);
					this.title = lc('admin_system_00428');
					this.showLinkBox = true;
				},
				addlink() {
					this.title = lc('admin_system_00430');
					this.link_id = 0;
					this.showLinkBox = true;
				},
				closeLinkBox() {
					this.showLinkBox = false;
					this.getList();
				},
				async submitForm() {
					let that = this;
					let params = {
						formdata: that.ruleFormStatus
					};
					that.statusBox = false;
					httpPost('m=system&c=set_friendlink&a=status', params).then(function (response) {
						let data = response.data;
						if (data.error == 0) {
							message.success(data.msg, function (e) {
								that.getList();
							});
						} else {
							message.error(data.msg);
						}
					}).catch(function (error) {
						console.log(error);
					})
				},
				status(row) {
					this.ruleFormStatus.status = row.link_state;
					this.ruleFormStatus.id = row.id;
					this.ruleFormStatus.content = row.statusbody;
					this.statusBox = true;
				},
				search() {
					this.pagination.page = 1;
					this.getList();
				},
				async getList() {
					let that = this;
					let searchForm = that.searchForm;
					let params = {
						searchOption: that.searchOption,
						pagination: that.pagination
					};
					that.loading = true;
					that.emptytext = lc('admin_user_weipin_00026');
					httpPost('m=system&c=set_friendlink&a=index', {...params, ...searchForm},{hideloading: true}).then(function (data) {
						let res = data.data;
						if (res.error == 0) {
							that.tableData = res.data.list;
							that.domainData = res.data.domain;
							that.pagination.total = res.data.total;
							that.pagination.pageSizeOption = res.data.pageSizes;
							if (that.prevPage != that.pagination.page) {
								that.prevPage = that.pagination.page;
								that.$refs.multipleTable.bodyWrapper.scrollTop = 0;
							}
							that.loading = false;
							if (that.tableData.length === 0){
	                            that.emptytext = lc('wap_js_00113');
	                        }
						}
					}).catch(function (error) {
						console.log(error)
					})
				},
				handleSizeChange(val) {
					this.pagination.pageSize = val;
					this.getList();
				},
				handleCurrentChange(val) {
					this.pagination.page = val;
					this.getList();
				},
				selectAllBottom(value) {
					value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
				},
				handleSelectionChange(val) {
					this.selectedItem = [];
					let _this = this;
					if (val.length) {
						val.forEach(item => {
							_this.selectedItem.push(item.id);
						});
					}
					if (_this.selectedItem.length == 0) {
						_this.checkedAll = false;
					} else {
						if (_this.selectedItem.length == _this.tableData.length) {
							_this.checkedAll = true;
						} else {
							_this.checkedAll = false;
						}
					}
				},
				delrow(row) {
					delConfirm(this, row.id, this.delete);
				},
				delAllBottom() {
					if (!this.selectedItem.length) {
						this.$message.error(lc('admin_user_weipin_00005'));
						return false;
					}
					delConfirm(this, this.selectedItem, this.delete);
				},
				async delete(Ids) {
					let _this = this;
					let params = {
						del: Ids
					};
					httpPost('m=system&c=set_friendlink&a=del', params).then(function (response) {
						if (response.data.error == 0) {
							message.success(response.data.msg);
							_this.getList();
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
