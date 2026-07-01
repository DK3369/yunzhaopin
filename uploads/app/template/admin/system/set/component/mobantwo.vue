<template>
	<div class="moduleElHight">
		<div class="tableDome_tip">
			<el-alert :title="lc('admin_system_00351')" type="success" :closable="false">
			</el-alert>
		</div>
		<div class="moduleHeadrButn" style=" margin-bottom: 12px;;">
			<el-button type="primary" icon="el-icon-document-add" @click="addTplBox">{{ lc('member_com_00354') }}</el-button>
		</div>
		<div class="moduleElTable" style="height: calc(100% - 105px);">
			<el-table :data="tableData" border style="width: 100%"
				:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" :empty-text="emptytext" height="100%">
				<el-table-column prop="wenjian" :label="lc('wap_js_00081')" width="150">
					<template slot-scope="scope">
						<div class="demo-image__preview">
							<el-image style="width: 100px; height: 60px" :src="scope.row.pic_n"
								:preview-src-list="srcList">
							</el-image>
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="name" :label="lc('wap_com_00413')">
				</el-table-column>
				<el-table-column prop="url" :label="lc('admin_00944')">
				</el-table-column>
				<el-table-column prop="status_n" :label="lc('member_user_00181')">
				</el-table-column>
				<el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
					<template slot-scope="scope">
						<div class="moduleElTaCaoz">
							<el-button size="mini" @click="editTpl(scope.row)">{{ lc('wap_js_00073') }}</el-button>
							<el-button size="mini" @click="delTpl(scope.row)" type="danger">{{ lc('common.delete') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>

		<div class="modluDrawer">
			<el-drawer :title="lc('admin_user_company_00135')" :visible.sync="editTplBox" :modal-append-to-body="false" :show-close="true"
				:with-header="true" size="45%">
				<div class="drawerModlue">
					<div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto;">

						<div class="drawerModLis">
							<div class="drawerModTite">
								<span>{{ lc('wap_com_00413') }}</span>
							</div>
							<div class="drawerModInpt">
								<el-input v-model="comTplInfo.name"></el-input>
							</div>
							<div class="drawerModTips">
								<el-alert :title="lc('admin_00949')" type="info" show-icon :closable="false">
								</el-alert>
							</div>
						</div>
						<div class="drawerModLis">
							<div class="drawerModTite">
								<span>{{ lc('member_user_00181') }}</span>
							</div>
							<div class="drawerModInpt">
								<el-switch v-model="comTplInfo.status" active-color="#13ce66" inactive-color="#ccc">
								</el-switch>
							</div>
						</div>
						<div class="drawerModLis">
							<div class="drawerModTite">
								<span>{{ lc('admin_00946') }}</span>
							</div>
							<div class="drawerModInpt">
								<el-input v-model="comTplInfo.dir"></el-input>

							</div>
							<div class="drawerModTips">
								<el-alert :title="lc('admin_00953')" type="info"
									show-icon :closable="false">
								</el-alert>
							</div>
						</div>
						<div class="drawerModLis">
							<div class="drawerModTite">
								<span>{{ lc('wap_user_00008') }}</span>
							</div>
							<div class="drawerModInpt">
								<el-input v-model="comTplInfo.price" @input="inputIntNumber($event, 'comTplInfo', 'price')"></el-input>

							</div>
							<div class="drawerModTips">
								<el-alert :title="lc('admin_00951')" type="info" show-icon :closable="false">
								</el-alert>
							</div>
						</div>
						<div class="drawerModLis">
							<div class="drawerModTite">
								<span>{{ lc('admin_00119') }}</span>
							</div>
							<div class="drawerModInpt">
								<el-upload class="avatar-uploader" :accept="pic_accept" :action="uploadAction" :show-file-list="false"
									:on-change="uploadChange">
									<img v-if="comTplInfo.picurl" :src="comTplInfo.picurl" class="avatar">
									<i v-else class="el-icon-plus avatar-uploader-icon"></i>
								</el-upload>
							</div>
						</div>
						<div class="drawerModLis">
							<div class="drawerModTite">
								<span>{{ lc('admin_00947') }}</span>
							</div>
							<div class="drawerModInpt">
								<el-input v-model="comTplInfo.struid"></el-input>
							</div>
							<div class="drawerModTips">
								<el-alert :title="lc('admin_00952')" type="info" show-icon :closable="false">
								</el-alert>
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
</template>

<script>
module.exports = {
	data: function () {
		return {
			pic_accept: localStorage.getItem("pic_accept"),
			emptytext: lc('wap_js_00113'),
			loading: false,
			tableData: [],
			srcList: [],
			comTplInfo: {
				name: '',
				status: 0,
				dir: '',
				price: '',
				picurl: '',
				struid: '',
				pic: '',
				id: ''
			},
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
		inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },
		addTplBox() {
			this.comTplInfo.name = '';
			this.comTplInfo.status = 0;
			this.comTplInfo.dir = '';
			this.comTplInfo.price = '';
			this.comTplInfo.picurl = '';
			this.comTplInfo.pic = '';
			this.comTplInfo.struid = '';
			this.comTplInfo.id = '';

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
			httpPost('m=system&c=set_tplset&a=comtpldel', { id: that.tplid }).then(function (response) {
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
			if (that.comTplInfo.dir == '') {
				message.error(lc('admin_vue_00056'));
				return false;
			}
			if (that.comTplInfo.picurl == '') {
				message.error(lc('admin_00258'));
				return false;
			}
			if (that.comTplInfo.status) {
				that.comTplInfo.status = 1;
			} else {
				that.comTplInfo.status = 0;
			}
			formData.append('url', that.comTplInfo.dir);
			formData.append('name', that.comTplInfo.name);
			formData.append('status', that.comTplInfo.status);
			formData.append('price', that.comTplInfo.price);
			formData.append('service_uid', that.comTplInfo.struid);
			if (that.files.length !== 0) {
				formData.append('file', that.files);
			}
			if (that.comTplInfo.id > 0) {
				formData.append('id', that.comTplInfo.id);
			}
			that.saveLoading = true;
			httpPost('m=system&c=set_tplset&a=comptplsave', formData).then(function (response) {
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
			this.comTplInfo.name = row.name;
			this.comTplInfo.status = row.status == 1 ? true : false;
			this.comTplInfo.dir = row.url;
			this.comTplInfo.price = row.price;
			this.comTplInfo.picurl = row.pic_n;
			this.comTplInfo.pic = row.pic;
			this.comTplInfo.struid = row.service_uid;
			this.comTplInfo.id = row.id;

			this.editTplBox = true;
		},
		async getList() {
			let that = this;
			let param = {};
			that.loading = true;
			that.emptytext = lc('admin_user_weipin_00026');
			httpPost('m=system&c=set_tplset&a=comtpl', param).then(function (response) {
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
			this.comTplInfo.picurl = URL.createObjectURL(file.raw);
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
