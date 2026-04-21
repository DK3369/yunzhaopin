<template>
	<div class="moduleElHight">
		<div class="moduleSeachbig">
            <div class="tableSeachInpt tableSeachInptsmall" >
                <el-select v-model="search.status" size="small" slot="prepend" placeholder="处理状态" clearable @change="doUserQuery">
                    <el-option label="未处理" value="1"></el-option>
                    <el-option label="已处理" value="2"></el-option>
                </el-select>
            </div>
			<div class="tableSeachInpt tableSeachInptsmall">
				<el-input placeholder="请输入搜索内容" size="small" @keyup.enter.native="doUserQuery" v-model="search.keyword" class="input-with-select" clearable>
					<el-select v-model="search.type" size="small" slot="prepend" placeholder="用户名" >
						<el-option label="用户名" value="1"></el-option>
						<el-option label="手机号" value="2"></el-option>
						<el-option label="用户ID" value="3"></el-option>
					</el-select>
				</el-input>
			</div>
			<div class="tableSeachInpt">
				<el-button type="primary" icon="el-icon-search" size="mini" @click="doUserQuery">查询</el-button>
			</div>
					 
		</div>
		<div class="admin_datatip"><i class="el-icon-document"></i>  数据统计：共 {{ memNum.count }} 条
            <span class="admin_datatip_n cp_n" @click="lockList">未处理：{{ memNum.weishenhe }} 条</span>
            <span class="admin_datatip_n">搜索结果： {{ total }} 条</span>
		</div>
		<div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
			<el-table :data="tableData" style="width: 100%" stripe  @selection-change="selectChange" ref="multipleTable"
							:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" v-loading="loading">
				<template slot="empty">
					<p>{{dataText}}</p>
				</template>
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="uid" label="用户ID" width="80"> </el-table-column>
                <el-table-column prop="username" label="用户名" > </el-table-column>
                <el-table-column prop="usertype_name" label="用户类型"  width="130">
                </el-table-column>
                <el-table-column prop="tel" label="手机号" > </el-table-column>
                <el-table-column prop="ctime_ymd" label="申请时间" width="180" > </el-table-column>
                <el-table-column prop="zt" label="状态">
                    <template slot-scope="scope">
                        <div class="admin_state">
                            <span v-if="scope.row.status == '2'" class="admin_state1">已处理</span>
                            <span v-else class="admin_state2">未处理</span>
                        </div>
                    </template>
                </el-table-column>
			    <el-table-column label="操作" width="140" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button  v-if="scope.row.status == '1'"  size="small " plain @click="handle(scope.row)">处理</el-button>
                            <el-button   type="danger" size="small "  @click="del(scope.row)">删除</el-button>
                        </div>
                    </template>
                </el-table-column>
            </el-table>
		</div>

		<div class="modulePaging">
            <div>
                <el-checkbox v-model="checkedAll" @change="selectAllBottom">全选</el-checkbox>
                <el-button @click="batchDel" size="mini">批量删除</el-button></div>
            <div class="modulePagNum">
                <el-pagination :total="total" @current-change="userPageChange"
                               @size-change="handleSizeChange"
                               :page-size="pageSize" :page-sizes="pageSizes"
                               :current-page.sync="page" layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
            </div>
		</div>

		 <div class="modluDrawer">
		 	<el-dialog title="注销提示" :visible.sync="cldrawer" :with-header="true" :modal-append-to-body="false"
		 		:show-close="true" width="300px">
		 		<div>
		 			<i class="el-icon-warning"></i> 注销账号？, 是否继续
		 		</div>
		 		<span slot="footer" class="dialog-footer">
		 			<el-button @click="cldrawer = false">取 消</el-button>
		 			<el-button type="primary" @click="cldrawer = false">确 定</el-button>
		 		</span>
		 	</el-dialog>
		 </div>
	</div>
</template>
    
<script>
module.exports = {
    props: {
        status: {type: String, default: ''}
    },
	data: function () {
		return {
			loading: false,
			dataText: '数据加载中',
            search:{
				type: '1',
				keyword:'',
                status: this.status,
            },
            checkedAll:false,
            input: '',
			select: '',
			value: true,
			value1: '',
			checked: '',
			drawer: false,
			cldrawer: false,
			seachbutn: false,
			tableHig: true,
			currentPage4: 4,
			tableData: [],
			items: [
				{ type: '', label: '正常' }, 
			],
            idsArr:[],
            uri: "m=user&c=",
            total: 0,
            page: 1,
            pageSize: 0,
            pageSizes:[],
            memNum:{},

            prevPage: 0
		}
	},
    created() {
        var that = this;
        let params = window.parent.homeapp.$route.params;
        let query = window.parent.homeapp.$route.query;
        
        if (!$.isEmptyObject(query.params)) {
            params = {...params,...query.params};
        }
        
        if (!$.isEmptyObject(params)) {
            delete params.activeName;
            this.getParams(params);
        }
        this.getList();
        this.shenheNumber();
    },
	mounted() {

	},
	methods: {
		getParams:function(params={},search=false){
			var that = this;
			for(let i in params){
				if(typeof that.search[i]!='undefined'){
					that.search[i] = params[i];
				}
			}
			if(search){
				this.getList();
			}
		},
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        lockList:function(){
            this.search.status = '1';
            this.page = 1
            this.getList()
        },
        getList:function(){
            let _this = this;
            let url = _this.uri + 'admin_member_logout&a=index';
            _this.search.page = this.page;
            _this.search.pageSize = this.pageSize;
            _this.loading = true;
            httpPost(url, _this.search, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data.data;
                    _this.total = res.data.total;
                    _this.loading = false;
                    if(_this.prevPage != _this.page){
						_this.prevPage = _this.page;
						_this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
					}
                    _this.pageSizes =res.data.pageSizes;
                    if (_this.tableData.length === 0) {
	                    _this.dataText = "暂无数据";
	                }
                }
            })
        },
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
        selectChange:function (val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.id);
                });
            }
        },
		handleSizeChange(val) {
            this.pageSize = val;
            this.getList();
		},
		handleCurrentChange(val) {
			console.log(`当前页: ${val}`);
		},
        handle:function (detail) {

            let _this = this,
				params = {};
			params.id = detail.id;
            let url = this.uri + 'admin_member_logout&a=status';
			let msg = '确定同意账号注销?';
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						message.success(res.data.msg, function () {
							_this.getList();
						});
					}
				})
			}, msg);
        },
        del:function (detail) {
            let _this = this,
				params = {};
			params.id = detail.id;
            let url = this.uri + 'admin_member_logout&a=del';
			let msg = '确定要删除?';
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						message.success(res.data.msg, function () {
							_this.getList();
						});
					}
				})
			}, msg);
        },
        batchDel: function () {
            let ids = this.idsArr;
            
            if (!ids.length) {
                message.error('请选择需要删除的用户!');
                return
            }
            let _this = this,
				params = {};
			params.del = ids;
            let url = this.uri + 'admin_member_logout&a=del'
			let msg = '确定要删除?';
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						message.success(res.data.msg, function () {
							_this.getList();
						});
					}
				})
			}, msg);
        },
        shenheNumber:function(){
            let _this = this;
            let url = this.uri + 'admin_member_logout&a=memNum'
            httpPost(url, {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.memNum = res.data;
                }else {
                    message.error(res.msg);
                }
            })
        },
	},
};
</script>
<style scoped>
	
.moduleElHight .moduleElTable {
	padding: 0;
	margin: 0;
	height: calc(100% - 110px);
	width: 100%;
}
   .moduleElTableHig {
       height: calc(100% - 136px) !important;
   }
   .tableSeachInptsmall .el-input{
       width: initial !important;
   }
   .tableSeachInptsmall .el-select{
   	   margin-right: 0px !important;
   }
   .el-input-group__prepend{
   	   background-color: #ffffff;
   	   padding: 0 0 0 20px;
   }
</style>