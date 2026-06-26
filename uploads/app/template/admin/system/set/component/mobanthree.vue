<template>
	<div>
		<div class="moduleElHight">
			<div class="tableDome_tip">
				<el-alert title="{yun:}t key='admin_00948'{/yun}" type="success"
					:closable="false">
				</el-alert>
			</div>
			<div class="moduleHeadrButn" style=" margin-bottom: 12px;;">
				<el-button type="primary" icon="el-icon-document-add" @click="addTplBox">{yun:}t key='member_com_00354'{/yun}</el-button>
			</div>
			<div class="moduleElTable">
				<el-table :data="tableData" border style="width: 100%"
					:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="640" v-loading="loading" :empty-text="emptytext">
					<el-table-column prop="wenjian" label="{yun:}t key='wap_js_00081'{/yun}" width="150">
						<template slot-scope="scope">
							<div class="demo-image__preview">
								<el-image style="width: 80px; height: 100px" :src="scope.row.pic"
									:preview-src-list="srcList">
								</el-image>
							</div>
						</template>
					</el-table-column>
					<el-table-column prop="name" label="{yun:}t key='wap_com_00413'{/yun}">
					</el-table-column>
					<el-table-column prop="url" label="{yun:}t key='admin_00944'{/yun}">
					</el-table-column>
					<el-table-column prop="status_n" label="{yun:}t key='member_user_00181'{/yun}">
					</el-table-column>
					<el-table-column label="{yun:}t key='member_user_00048'{/yun}" width="140">
						<template slot-scope="scope">
							<div class="moduleElTaCaoz">
								<el-button size="mini" @click="editTpl(scope.row)">{yun:}t key='wap_js_00073'{/yun}</el-button>
								<el-button size="mini" @click="delTpl(scope.row)" type="danger">{yun:}t key='common.delete'{/yun}</el-button>
							</div>
						</template>
					</el-table-column>
				</el-table>
			</div>

			<div class="modluDrawer">
				<el-drawer title="{yun:}t key='member_user_00189'{/yun}" :visible.sync="editTplBox" :modal-append-to-body="false" :show-close="true"
					:with-header="true" size="45%">
					<div class="drawerModlue">
						<div class="drawerModInfo" style="max-height: calc(100% - 80px); overflow-y: auto;">

							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='wap_com_00413'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="resumeTplInfo.name"></el-input>
								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_00949'{/yun}" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='member_user_00181'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-switch v-model="resumeTplInfo.status" active-color="#13ce66" inactive-color="#ccc">
									</el-switch>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_00946'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="resumeTplInfo.dir"></el-input>

								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_00950'{/yun}" type="info"
										show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='wap_user_00008'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="resumeTplInfo.price" @input="inputIntNumber($event, 'resumeTplInfo', 'price')"></el-input>

								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_00951'{/yun}" type="info" show-icon :closable="false">
									</el-alert>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_00119'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-upload class="avatar-uploader" :accept="pic_accept" :action="uploadAction" :show-file-list="false"
										:on-change="uploadChange">
										<img v-if="resumeTplInfo.picurl" :src="resumeTplInfo.picurl" class="avatar">
										<i v-else class="el-icon-plus avatar-uploader-icon"></i>
									</el-upload>
								</div>
							</div>
							<div class="drawerModLis">
								<div class="drawerModTite">
									<span>{yun:}t key='admin_00947'{/yun}</span>
								</div>
								<div class="drawerModInpt">
									<el-input v-model="resumeTplInfo.struid"></el-input>
								</div>
								<div class="drawerModTips">
									<el-alert title="{yun:}t key='admin_00952'{/yun}" type="info" show-icon :closable="false">
									</el-alert>
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
			resumeTplInfo: {
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
			delTplBox: false,
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
			this.resumeTplInfo.name = '';
			this.resumeTplInfo.status = 0;
			this.resumeTplInfo.dir = '';
			this.resumeTplInfo.price = '';
			this.resumeTplInfo.picurl = '';
			this.resumeTplInfo.pic = '';
			this.resumeTplInfo.struid = '';
			this.resumeTplInfo.id = '';

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
			httpPost('m=system&c=set_tplset&a=resumetpldel', { id: that.tplid }).then(function (response) {
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
			if (that.resumeTplInfo.dir == '') {
				message.error(lc('admin_vue_00056'));
				return false;
			}
			if (that.resumeTplInfo.picurl == '' && that.resumeTplInfo.pic == '') {
				message.error("{yun:}t key='admin_00258'{/yun}");
				return false;
			}
			if (that.resumeTplInfo.status) {
				that.resumeTplInfo.status = 1;
			} else {
				that.resumeTplInfo.status = 0;
			}
			formData.append('url', that.resumeTplInfo.dir);
			formData.append('name', that.resumeTplInfo.name);
			formData.append('status', that.resumeTplInfo.status);
			formData.append('price', that.resumeTplInfo.price);
			formData.append('service_uid', that.resumeTplInfo.struid);
			if (that.files.length !== 0) {
				formData.append('file', that.files);
			}
			if (that.resumeTplInfo.id > 0) {
				formData.append('id', that.resumeTplInfo.id);
			}
			that.saveLoading = true;
			httpPost('m=system&c=set_tplset&a=resumetplsave', formData).then(function (response) {
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
			this.resumeTplInfo.name = row.name;
			this.resumeTplInfo.status = row.status == 1 ? true : false;
			this.resumeTplInfo.dir = row.url;
			this.resumeTplInfo.price = row.price;
			this.resumeTplInfo.picurl = row.pic_n;
			this.resumeTplInfo.pic = row.pic;
			this.resumeTplInfo.struid = row.service_uid;
			this.resumeTplInfo.id = row.id;

			this.editTplBox = true;
		},
		async getList() {
			let that = this;
			let param = {};
			that.loading = true;
			that.emptytext = "{yun:}t key='admin_user_weipin_00026'{/yun}";
			httpPost('m=system&c=set_tplset&a=resumetpl', param).then(function (response) {
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
			this.resumeTplInfo.picurl = URL.createObjectURL(file.raw);
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
