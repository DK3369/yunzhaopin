<template>
	<div class="moduleElHight">
		<div class="tableDome_tip">
			<el-alert type="success" :closable="false">
				<div slot="title">更换模板后，如果是静态页面需重新生成！【<el-link type="primary" :underline="false" href='http://www.phpyun.com/tpl.php' target="_blank">获取更多模板</el-link>】</div>
			</el-alert>
		</div>
		<div class="moduleElTable" style="height: calc(100% - 105px);">
			<el-table :data="tableData" border style="width: 100%"
				:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" v-loading="loading" :empty-text="emptytext" height="100%">
				<el-table-column prop="wenjian" label="图片" width="150">
					<template slot-scope="scope">
						<div class="demo-image__preview">
							<el-image style="width: 100px; height: 60px" :src="scope.row.img"
								:preview-src-list="srcList">
							</el-image>
						</div>
					</template>
				</el-table-column>
				<el-table-column prop="name" label="模板名称">
				</el-table-column>
				<el-table-column prop="dir" label="风格目录名称">
				</el-table-column>
				<el-table-column prop="dir" label="状态">
					<template slot-scope="scope">
						<span v-if="scope.row.dir == sy_style">当前使用风格</span>
						<span v-else></span>
					</template>
				</el-table-column>
				<el-table-column fixed="right" label="操作" width="140">
					<template slot-scope="scope">
						<div class="moduleElTaCaoz">
							<el-button size="mini" @click="editTpl(scope.row)">修改</el-button>
							<el-button size="mini" @click="tplChange(scope.row.dir)">使用</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>

		<div class="tck_textbox">
			<el-dialog title="网站模板" :visible.sync="tplbox" :with-header="true" :modal-append-to-body="false"
				:show-close="true" width="30%">
				<div class="wxsettip_small ">预览图</div>
				<el-image :src="tplInfo.img"></el-image>
				<div class="wxsettip_small ">风格名称</div>
				<el-input v-model="tplInfo.name" placeholder="默认模板"></el-input>
				<div class="wxsettip_small">风格目录</div>
				<el-input v-model="tplInfo.dir" placeholder="default"></el-input>
				<div class="wxsettip_small">风格作者</div>
				<el-input v-model="tplInfo.author" placeholder="黄灿灿"></el-input>
				<span slot="footer" class="dialog-footer">
					<el-button @click="tplbox = false">取 消</el-button>
					<el-button type="primary" :loading="save_load" @click="tplSave">确 定</el-button>
				</span>
			</el-dialog>
		</div>
	</div>
</template>

<script>
module.exports = {
	data: function () {
		return {
			emptytext: '暂无数据',
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
			delConfirm(this, {}, this.changeSave, '确定使用该模板？');
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
			that.emptytext = "数据加载中";
			httpPost('m=system&c=set_tplset&a=index', param).then(function (response) {
				let res = response.data;
				if (res.error == 0) {
					that.tableData = res.data.list;
					that.sy_style = res.data.sy_style;
					that.srcList = res.data.imgarr;
					that.loading = false;
					if (that.tableData.length === 0){
                        that.emptytext = "暂无数据";
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
