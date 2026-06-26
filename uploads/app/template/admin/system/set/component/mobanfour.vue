<template>
	<div style="position: relative; overflow: hidden; height: 100%;">
		<div class="moduleElHight">
			<div class="tableDome_tip">
				<el-alert title="{yun:}t key='admin_system_00335'{/yun}" type="success" :closable="false">
				</el-alert>
			</div>
			<div class="moduleHeadrButn" style=" margin-bottom: 12px;;">
				<el-button type="primary" icon="el-icon-document-add" @click="addTplBox">{yun:}t key='admin_system_00341'{/yun}</el-button>
			</div>
			<div class="moduleElTable" style="height: calc(100% - 105px);">
				<el-table :data="tableData" border style="width: 100%"
					:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading" :empty-text="emptytext">
					<el-table-column prop="wenjian" label="图片" width="150">
						<template slot-scope="scope">
							<div class="demo-image__preview">
								<el-image style="width: 100px; height: 60px" :src="scope.row.pic_n"
									:preview-src-list="srcList">
								</el-image>
							</div>
						</template>
					</el-table-column>
					<el-table-column prop="name" label="模板名称">
					</el-table-column>
					<el-table-column prop="status_n" label="状态">
					</el-table-column>
					<el-table-column fixed="right" label="操作" width="140">
						<template slot-scope="scope">
							<div class="moduleElTaCaoz">
								<el-button size="mini" @click="editTpl(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
								<el-button size="mini" @click="previewTpl(scope.row.id)">{yun:}t key='wap_00071'{/yun}</el-button>
								<el-button size="mini" @click="delTpl(scope.row)" type="danger">{yun:}t key='common.delete'{/yun}</el-button>

							</div>
						</template>
					</el-table-column>
				</el-table>
			</div>
			<div class="modluDrawer">
				<el-drawer title="{yun:}t key='admin_system_00338'{/yun}" :visible.sync="editTplBox" :modal-append-to-body="false" :show-close="true"
					:with-header="true" size="50%">
					<div class="drawerModlue">
						<div class="drawerModInfo">

							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_system_00342'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="indexTplInfo.name"></el-input>
								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_system_00340'{/yun}" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>

							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_system_00344'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="indexTplInfo.top">
										<span slot="suffix" style="line-height: 35px;">px</span>
									</el-input>
								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_system_00336'{/yun}" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_system_00345'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-switch v-model="indexTplInfo.hse" active-color="#13ce66" inactive-color="#ccc">
									</el-switch>
								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_system_00337'{/yun}" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='member_user_00181'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-switch v-model="indexTplInfo.status" active-color="#13ce66" inactive-color="#ccc">
									</el-switch>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_system_00343'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-date-picker v-model="indexTplInfo.strtimes" type="daterange" range-separator="至"
										start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}" value-format="yyyy-MM-dd">
									</el-date-picker>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_00119'{/yun}</span>
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
							<el-button type="primary" size="medium" @click="tplSave" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
						</div>
					</div>
				</el-drawer>
			</div>
		</div>
	</div>
</template>

<script>
module.exports = {
	data: function () {
		return {
			pic_accept: localStorage.getItem("pic_accept"),
			emptytext: "{yun:}t key='wap_js_00113'{/yun}",
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
			delConfirm(this, {}, this.delTplSubmit, "{yun:}t key='wap_user_00001'{/yun}");
		},
		async delTplSubmit() {
			let that = this;
			if (that.tplid == '') {
				message.error("{yun:}t key='admin_00307'{/yun}");
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
				message.error("{yun:}t key='admin_system_00339'{/yun}");
				return false;
			}
			if (that.indexTplInfo.picurl == '' && that.indexTplInfo.pic == '') {
				message.error("{yun:}t key='admin_00258'{/yun}");
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
			that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
			httpPost('m=system&c=set_tplset&a=pcindextpl', param).then(function (response) {
				let res = response.data;
				if (res.error == 0) {
					that.tableData = res.data.list;
					that.srcList = res.data.imgarr;
					that.loading = false;
					if (that.tableData.length === 0){
                        that.emptytext = "{yun:}t key='wap_js_00113'{/yun}";
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
