<template>
	<div class="moduleElHight">
		<div class="tableDome_tip">
			<el-alert type="success" :closable="false">
				<div slot="title">{{ lc('admin_00940') }}<el-link type="primary" :underline="false" href='http://www.ov6.com/tpl.php' target="_blank">{{ lc('admin_system_00348') }}</el-link>】</div>
			</el-alert>
		</div>
		<div class="moduleElTable" style="height: calc(100% - 105px);">
			<el-table :data="tableData" border style="width: 100%"
				:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" :empty-text="emptytext" height="100%">
				<el-table-column prop="wenjian" :label="lc('wap_js_00081')" width="150">
					<template slot-scope="scope">
						<div class="demo-image__preview">
							<el-image style="width: 100px; height: 60px" :src="scope.row.img"
								:preview-src-list="srcList">
							</el-image>
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="name" :label="lc('wap_com_00413')">
				</el-table-column>
				<el-table-column prop="dir" :label="lc('admin_00944')">
				</el-table-column>
				<el-table-column prop="dir" :label="lc('member_user_00181')">
					<template slot-scope="scope">
						<span v-if="scope.row.dir == sy_style">{{ lc('admin_system_00347') }}</span>
						<span v-else></span>
					</template>
				</el-table-column>
				<el-table-column fixed="right" :label="lc('member_user_00048')" width="140">
					<template slot-scope="scope">
						<div class="moduleElTaCaoz">
							<el-button size="mini" @click="editTpl(scope.row)">{{ lc('wap_js_00073') }}</el-button>
							<el-button size="mini" @click="tplChange(scope.row.dir)">{{ lc('member_user_00284') }}</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>

		<div class="tck_textbox">
			<el-dialog :title="lc('admin_system_00349')" :visible.sync="tplbox" :with-header="true" :modal-append-to-body="false"
				:show-close="true" width="30%">
				<div class="wxsettip_small ">{{ lc('admin_00941') }}</div>
				<el-image :src="tplInfo.img"></el-image>
				<div class="wxsettip_small ">{{ lc('admin_00942') }}</div>
				<el-input v-model="tplInfo.name" :placeholder="lc('member_user_00283')"></el-input>
				<div class="wxsettip_small">{{ lc('admin_system_00350') }}</div>
				<el-input v-model="tplInfo.dir" placeholder="default"></el-input>
				<div class="wxsettip_small">{{ lc('admin_00943') }}</div>
				<el-input v-model="tplInfo.author" :placeholder="lc('admin_00945')"></el-input>
				<span slot="footer" class="dialog-footer">
					<el-button @click="tplbox = false">{{ lc('admin_user_weipin_00043') }}</el-button>
					<el-button type="primary" :loading="save_load" @click="tplSave">{{ lc('wap_com_00019') }}</el-button>
				</span>
			</el-dialog>
		</div>
	</div>
</template>

<script>
module.exports = {
	data: function () {
		return {
			emptytext: lc('wap_js_00113'),
			loading: false,
			tplbox: false,
			sy_style: '',
			tableData: [],
			srcList: [],
			tplInfo: {
				name: '',
				dir: '',
				author: '',
				img:''
			},

			changedir: '',
			save_load:false,
		}
	},
	created() {
		this.getList();
	},
	methods: {
		async changeSave() {
			let that = this;
			that.save_load = true;
			httpPost('m=system&c=set_tplset&a=check_style', { dir: that.changedir }).then(function (response) {
				that.save_load = false;
				let res = response.data;
				if (res.error == 0) {
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
		tplChange(dir) {
			this.changedir = dir;
			delConfirm(this, {}, this.changeSave, lc('resume_00057'));
		},
		editTpl(row) {
			this.tplInfo.name = row.name;
			this.tplInfo.dir = row.dir;
			this.tplInfo.author = row.author;
			this.tplInfo.img = row.img;
			this.tplbox = true;
		},
		async tplSave() {
			let that = this;
			let params = {
				name: that.tplInfo.name,
				author: that.tplInfo.author,
				dir: that.tplInfo.dir
			};
			that.save_load = true;
			httpPost('m=system&c=set_tplset&a=stylesave', params).then(function (response) {
				that.save_load = false;
				let res = response.data;
				if (res.error == 0) {
					that.tplbox = false;
					message.success(res.msg, function () {
						that.getList();
					});
				} else {
					message.error(res.msg);
				}
			});
		},
		async getList() {
			let that = this;
			let param = {};
			that.loading = true;
			that.emptytext = lc('admin_user_weipin_00026');
			httpPost('m=system&c=set_tplset&a=index', param).then(function (response) {
				let res = response.data;
				if (res.error == 0) {
					that.tableData = res.data.list;
					that.sy_style = res.data.sy_style;
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
	},
};
</script>
<style scoped></style>
