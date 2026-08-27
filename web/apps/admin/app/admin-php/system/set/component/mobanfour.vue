<template>
	<div style="position: relative; overflow: hidden; height: 100%;">
		<div class="moduleElHight">
			<div class="tableDome_tip">
				<el-alert :title="lc('admin_system_00335')" type="success" :closable="false">
				</el-alert>
			</div>
			<div class="moduleHeadrButn" style=" margin-bottom: 12px;;">
				<el-button type="primary" icon="el-icon-document-add" @click="addTplBox">{{ lc('admin_system_00341') }}</el-button>
			</div>
			<div class="moduleElTable" style="height: calc(100% - 105px);">
				<el-table :data="tableData" border style="width: 100%"
					:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
					<el-table-column prop="wenjian" :label="lc('wap_js_00081')" width="150">
						<template #default="scope">
							<div class="demo-image__preview">
								<el-image style="width: 100px; height: 60px" :src="scope.row.pic_n"
									:preview-src-list="srcList">
								</el-image>
							</div>
						</template>
					</el-table-column>
					<el-table-column prop="name" :label="lc('wap_com_00413')">
					</el-table-column>
					<el-table-column prop="status_n" :label="lc('member_user_00181')">
					</el-table-column>
					<el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
						<template #default="scope">
							<div class="moduleElTaCaoz">
								<el-button size="small" @click="editTpl(scope.row)">{{ lc('wap_js_00073') }}</el-button>
								<el-button size="small" @click="previewTpl(scope.row.id)">{{ lc('wap_00071') }}</el-button>
								<el-button size="small" @click="delTpl(scope.row)" type="danger">{{ lc('common.delete') }}</el-button>

							</div>
						</template>
					</el-table-column>
				</el-table>
			</div>
			<div class="modluDrawer">
				<el-drawer :title="lc('admin_system_00338')" v-model="editTplBox" :modal-append-to-body="false" :show-close="true"
					:with-header="true" size="50%">
					<div class="drawerModlue">
						<div class="drawerModInfo">

							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{{ lc('admin_system_00342') }}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="indexTplInfo.name"></el-input>
								</div>
								<div class="drawerModTips">
									<el-alert :title="lc('admin_system_00340')" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>

							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{{ lc('admin_system_00344') }}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="indexTplInfo.top">
										<template #suffix><span style="line-height: 35px;">px</span></template>
									</el-input>
								</div>
								<div class="drawerModTips">
									<el-alert :title="lc('admin_system_00336')" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{{ lc('admin_system_00345') }}</span>
								</div>
								<div class="drawerModInpt">
									<el-switch v-model="indexTplInfo.hse" active-color="#13ce66" inactive-color="#ccc">
									</el-switch>
								</div>
								<div class="drawerModTips">
									<el-alert :title="lc('admin_system_00337')" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{{ lc('member_user_00181') }}</span>
								</div>
								<div class="drawerModInpt">
									<el-switch v-model="indexTplInfo.status" active-color="#13ce66" inactive-color="#ccc">
									</el-switch>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{{ lc('admin_system_00343') }}</span>
								</div>
								<div class="drawerModInpt">
									<el-date-picker v-model="indexTplInfo.strtimes" type="daterange" :range-separator="lc('admin_company_00019')"
										:start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" value-format="YYYY-MM-dd">
									</el-date-picker>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{{ lc('admin_00119') }}</span>
								</div>
								<div class="drawerModInpt">
									<el-upload class="avatar-uploader" :accept="pic_accept" :action="uploadAction" :show-file-list="false"
										:on-change="uploadChange">
										<img v-if="indexTplInfo.picurl" :src="indexTplInfo.picurl" class="avatar">
										<i v-else class="el-icon-plus avatar-uploader-icon"></i>
									</el-upload>
								</div>
							</div>
						</div>
						<div class="setBasicButn" style="border: none;">
							<el-button type="primary" size="medium" @click="tplSave" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
						</div>
					</div>
				</el-drawer>
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
	data: function () {
		return {
			pic_accept: localStorage.getItem("pic_accept"),
			emptytext: lc('wap_js_00113'),
			loading: false,
			tableData: [],
			srcList: [],
			indexTplInfo: {
				name: '',
				status: 0,
				top: 0,
				hse: 0,
				picurl: '',
				strtimes: [],
				pic: '',
				id: ''
			},
			sy_weburl: localStorage.getItem("sy_weburl"),
			files: [],
			editTplBox: false,
			tplid: '',
			saveLoading: false,
			uploadAction: baseUrl + 'm=common&c=common_upload'
		}
	},
	created() {
		this.getList();
	},
	methods: {
		previewTpl(id){
			window.open(this.sy_weburl + '/index.php?tpltype=' + id);
		},
		addTplBox() {
			this.indexTplInfo.name = '';
			this.indexTplInfo.status = 0;
			this.indexTplInfo.hse = 0;
			this.indexTplInfo.top = 0;
			this.indexTplInfo.picurl = '';
			this.indexTplInfo.pic = '';
			this.indexTplInfo.strtimes = [];
			this.indexTplInfo.id = '';

			this.editTplBox = true;
		},
		delTpl(row) {
			this.tplid = row.id;
			delConfirm(this, {}, this.delTplSubmit, lc('wap_user_00001'));
		},
		async delTplSubmit() {
			let that = this;
			if (that.tplid == '') {
				message.error(lc('admin_00307'));
				return false;
			}
			httpPost('m=system&c=set_tplset&a=indextpldel', { id: that.tplid }).then(function (response) {
				let res = response.data;
				if (res.error == 0) {
					that.delTplBox = false;
					message.success(res.msg, function () {
						that.getList();
					});
				} else {
					message.error(res.msg);
				}
			}).catch(function (error) {
				console.log(error)
			})
		},
		async tplSave() {
			let that = this;
			let formData = new FormData();
			if (that.indexTplInfo.name == '') {
				message.error(lc('admin_system_00339'));
				return false;
			}
			if (that.indexTplInfo.picurl == '' && that.indexTplInfo.pic == '') {
				message.error(lc('admin_00258'));
				return false;
			}
			if (that.indexTplInfo.status) {
				that.indexTplInfo.status = 1;
			} else {
				that.indexTplInfo.status = 0;
			}
			if (that.indexTplInfo.hse) {
				that.indexTplInfo.hse = 1;
			} else {
				that.indexTplInfo.hse = 0;
			}
			formData.append('se', that.indexTplInfo.hse);
			formData.append('name', that.indexTplInfo.name);
			formData.append('status', that.indexTplInfo.status);
			formData.append('height', that.indexTplInfo.top);
			formData.append('time', that.indexTplInfo.strtimes);
			if (that.files.length !== 0) {
				formData.append('file', that.files);
			}
			if (that.indexTplInfo.id > 0) {
				formData.append('id', that.indexTplInfo.id);
			}
			that.saveLoading = true;
			httpPost('m=system&c=set_tplset&a=indextplsave', formData).then(function (response) {
				let res = response.data;
				if (res.error == 0) {
					message.success(res.msg, function () {
						that.editTplBox = false;
						that.getList();
					});
				} else {
					message.error(res.msg);
				}
			}).finally(function () {
				setTimeout(function () {
				    that.saveLoading = false;
				}, 2000);
			});
		},
		editTpl(row) {
			this.indexTplInfo.name = row.name;
			this.indexTplInfo.status = row.status == 1 ? true : false;
			this.indexTplInfo.top = row.height;
			this.indexTplInfo.hse = row.se == 1 ? true : false;
			this.indexTplInfo.picurl = row.pic_n;
			this.indexTplInfo.pic = row.pic;
			this.indexTplInfo.strtimes = row.strtimes;
			this.indexTplInfo.id = row.id;

			this.editTplBox = true;
		},
		async getList() {
			let that = this;
			let param = {};
			that.loading = true;
			that.emptytext = lc('admin_user_weipin_00026');
			httpPost('m=system&c=set_tplset&a=pcindextpl', param).then(function (response) {
				let res = response.data;
				if (res.error == 0) {
					that.tableData = res.data.list;
					that.srcList = res.data.imgarr;
					that.loading = false;
					if (that.tableData.length === 0){
                        that.emptytext = lc('wap_js_00113');
                    }
				}
			}).catch(function (error) {
				console.log(error)
			})
		},
		uploadChange(file) {
			this.indexTplInfo.picurl = URL.createObjectURL(file.raw);
			// 复刻文件信息
			this.files = file.raw;
		},
	},
};
</script>
<style scoped>
.avatar-uploader .el-upload {
	border: 1px dashed #d9d9d9;
	border-radius: 6px;
	cursor: pointer;
	position: relative;
	overflow: hidden;
}

.avatar-uploader .el-upload:hover {
	border-color: #409EFF;
}

.avatar-uploader-icon {
	font-size: 28px;
	color: #8c939d;
	width: 148px;
	height: 148px;
	line-height: 148px;
	text-align: center;
}

.avatar {
	width: 148px;
	height: 148px;
	display: block;
}
</style>
