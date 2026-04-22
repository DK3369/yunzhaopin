<template>
	<div class="moduleElHight">

		<div class="moduleSeachs">
			<div class="moduleSeachButn" style="margin-left: auto;">
				<el-button type="primary" icon="el-icon-document-add" size="mini" @click="addinfo('')">添加模板</el-button>
			</div>
		</div>

		<div class="moduleElTable">
			<el-table ref="table" :data="tableData" v-loading="list_loading" @selection-change="selectionChange" border
				style="width: 100%" :header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" :empty-text="emptytext">
				<el-table-column type="selection" width="55">
				</el-table-column>
				<el-table-column prop="id" label="编号" width="80">
				</el-table-column>
				<el-table-column prop="title" label="模板名称">
				</el-table-column>
				<el-table-column prop="keyword" label="模板类型">
					<template slot-scope="scope">
						<span v-if="scope.row.type == 'onejob' || scope.row.type == 'job'">职位模板</span>
						<span v-else-if="scope.row.type == 'resume'">简历模板</span>
						<span v-else-if="scope.row.type == 'company'">企业模板</span>
					</template>
				</el-table-column>
				<el-table-column label="操作" width="150">
					<template slot-scope="scope">
						<div class="cz_button">
							<el-button size="mini" @click="addinfo(scope.row.id)">编辑</el-button>
							<el-button type="danger" size="small" @click="deleteinfo(scope.row.id)"
								v-if="scope.row.type != 'onejob'">删除</el-button>
						</div>
					</template>
				</el-table-column>
			</el-table>
		</div>


		<div class="modulePaging">
			<div class="modulecz">
				<el-checkbox v-model="allchecked" @change="allcheckChange">全选</el-checkbox>
				<el-button size="mini" @click="deleteAll">批量删除</el-button>
			</div>
			<div class="modulePagNum">
				<el-pagination background @size-change="handleSizeChange" @current-change="handleCurrentChange"
					:current-page="currentPage" :page-size="limit" :page-sizes="page_sizes" :total="total"
					layout="total, sizes, prev, pager, next, jumper">
				</el-pagination>
			</div>
		</div>

	</div>
</template>

<script>

module.exports = {
	props: {
		temptype: {
			type: [String, Number],
			default: '',

		},
        secondpage:Number,
        thirdpage:Number,
	},
	data: function () {
		return {
			emptytext: '暂无数据',
			tableData: [],
			total: 0,
			limit: 0,
			currentPage: 1,
			prevPage:0,
			page_sizes: [],
			list_loading: false,
			allchecked: false,
			choosedata: [],
			temptype_v: '',

			sid: '',
			addshow: false,
			timer: ''
		}
	},
	
	mounted() {
		this.temptype_v = this.temptype;
        if (this.temptype_v == 0){
            this.currentPage=this.secondpage;
        }else if (this.temptype_v == 1){
            this.currentPage=this.thirdpage;
        }
		this.getList();
	},
	methods: {

		async getList() {
			let that = this;
			let params = {
				page: that.currentPage,
				limit: that.limit,
				temptype: this.temptype
			}

			this.list_loading = true;
			that.emptytext = "数据加载中";
			httpPost('m=tool&c=fabutool&a=index', params).then((result) => {
				this.list_loading = false;
				var res = result.data;

				if (res.error == 0) {
					that.tableData = res.data.list
					that.total = parseInt(res.data.total)
					that.page_sizes = res.data.page_sizes;
					that.limit = res.data.page_size;
					
					if(that.prevPage != that.currentPage){
						that.prevPage = that.currentPage;
						that.$refs.table.bodyWrapper.scrollTop = 0;
					}
					if (that.tableData.length === 0){
                        that.emptytext = "暂无数据";
                    }
				}
			}).catch(function (e) {
				console.log(e)
			})
		},
		search: function () {
			this.currentPage = 1;
			this.getList();
		},

		handleCurrentChange(val) {
			this.currentPage = val;
			this.getList()
		},
		handleSizeChange(val) {
			this.currentPage = 1
			this.limit = val
			this.getList()
		},
		allcheckChange: function () {

			this.$refs.table.toggleAllSelection();

		},
		selectionChange: function (e) {
			if (this.tableData.length != e.length) {
				this.allchecked = false;
			} else {
				this.allchecked = true;
			}
			this.choosedata = e;
		},
		deleteinfo: function (id) {
			var _this = this;

			var params = {
				del: id
			};
			delConfirm(_this, params, this.deletePost)
		},
		deleteAll: function () {
			var _this = this;
			var idarr = [];
			if (this.choosedata.length > 0) {
				for (let i in this.choosedata) {
					idarr.push(this.choosedata[i].id);
				}
			} else {
				message.error('请选择要删除的数据'); return;
			}
			var params = {
				del: idarr
			};

			delConfirm(_this, params, this.deletePost)
		},
		async deletePost(params) {

			let that = this;

			httpPost('m=tool&c=fabutool&a=wxPubTempDel', params).then(function (result) {

				var res = result.data;
				if (res.error == 0) {
					message.success(res.msg, function () { that.getList() }); return;
				} else {
					message.error(res.msg); return;
				}
			}).catch(function (e) {
				console.log(e)
			})
		},
		addinfo: function (id) {

			if (this.sid != id || id == '') {
				this.sid = id;
			}

			//this.addshow = true;
            if (this.temptype_v == 0){

            }
			
			window.parent.homeapp.topage({name: 'addpubtemp',path: '/addpubtemp',params:{sid:this.sid,temptype_v:this.temptype_v,page:this.currentPage}});

		},
		closeUpdate: function () {

			this.addshow = false;
		}

	},
};
</script>
<style scoped>
.moduleSeachmore {
	padding: 0px;
}

.moduleSeachs {
	padding: 0px 0px 12px 0px;
	width: 100%;
}

.moduleElTable {
	padding: 0;
	margin: 0;
	height: calc(100% - 84px);
	width: 100%;
}

.tableSeachInptsmalltwo {
	margin-bottom: 0px;
	margin-right: 12px;
}

.tableSeachInptsmalltwo .el-input__inner {
	height: 32px;
	line-height: 32px;
	width: 260px;
	padding: 0px 5px;
	;
}

.el-dialog__body {
	padding: 0px 20px;
}

.el-tag+.el-tag {
	margin-left: 10px;
}

.button-new-tag {
	margin-left: 10px;
	height: 32px;
	line-height: 30px;
	padding-top: 0;
	padding-bottom: 0;
}

.input-new-tag {
	width: 90px;
	margin-left: 10px;
	vertical-align: bottom;
}
</style>