<template>
	<div class="moduleElHight">
		<div class="moduleSeachbig">

			<div class="tableSeachInpt">
				<el-input placeholder="请输入搜索内容" size="small" @keyup.enter.native="doUserQuery" prefix-icon="el-icon-search" v-model="search.keyword" clearable>
				</el-input>
			</div>
			<div class="tableSeachInpt">
				<el-select v-model="search.appealstate" size="small" clearable placeholder="处理状态" @change="doUserQuery">
                    <el-option label="未处理" value="1"></el-option>
                    <el-option label="已处理" value="2"></el-option>
                </el-select>
			</div>
			<div class="tableSeachInpt">
				<el-button type="primary" icon="el-icon-search" size="mini" @click="doUserQuery">查询</el-button>
			</div>
		</div>
		<div class="moduleElTable" :class="{ 'moduleElTableHig': tableHig }" style="border: 1px solid #ebeef5; width: calc(100% - 2px);">
			<el-table :data="tableData" style="width: 100%" stripe @selection-change="selectChange" ref="multipleTable"
							:header-cell-style="{ background: '#f5f7fa', color: '#606266' }" height="100%" @sort-change="shortChange" v-loading="loading">
				<template slot="empty">
					<p>{{dataText}}</p>
				</template>
                <el-table-column type="selection" width="55"> </el-table-column>
                <el-table-column prop="uid" label="用户ID" width="120" sortable="custom"> </el-table-column>
                <el-table-column prop="username" label="申诉账号" > </el-table-column>
                <el-table-column  label="联系方式" >
                    <template slot-scope="scope">
                        {{scope.row.moblie}}
                        <template v-if="scope.row.moblie && promiss.moblie">
                            <a href="javascript:void(0);" style="color: #399c1d" @click="send_moblie(scope.row);">发信息</a>
                        </template>
                        <br>
                        {{scope.row.email}}
                        <template v-if="scope.row.email  && promiss.email">
                            <a href="javascript:void(0);" style="color: #399c1d" @click="send_email(scope.row);">发邮件</a>
                        </template>
                    </template>
                </el-table-column>
                <el-table-column prop="appeal" label="申诉信息" > </el-table-column>
                <el-table-column prop="appealtime_ymd" label="申请时间" width="160" sortable="custom"> </el-table-column>
                <el-table-column prop="zt" label="状态" width="80">
                    <template slot-scope="props">
                        <div class="admin_state">
                            <span v-if="props.row.appealstate == 2" class="admin_state1">已处理</span>
                            <span v-else class="admin_state2">未处理</span>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column label="操作" width="240" fixed="right">
                    <template slot-scope="scope">
                        <div class="cz_button">
                            <el-button  v-if="scope.row.appealstate == 1"   size="mini" plain @click="set(scope.row)">确认</el-button>
                            <el-button   size="mini" plain @click="resetPassword(scope.row)">重置密码</el-button>
                            <el-button   size="mini" plain @click="detailFun(scope.row)">详情</el-button>
                            <el-button   type="danger" size="mini"  @click="del(scope.row)">删除</el-button>
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
                <el-pagination :total="total" @current-change="userPageChange" :page-sizes="pageSizes"
                               :page-size="pageSize" @size-change="handleSizeChange"
                               :current-page.sync="page" layout="total, sizes, prev, pager, next, jumper">
                </el-pagination>
			</div>
		</div>

		<!--账户详情-->
		<div class="modluDrawer">
			<el-drawer title="账户详情" :visible.sync="detaildrawer" :append-to-body="true" size="620px">
				<div class="drawerModInfo drawerModInfoOne" style="padding: 0 20px;">
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>用户名</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.username" :disabled="true"></el-input>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>姓名</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="user.name" :disabled="true"></el-input>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>电话</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.moblie"  :disabled="true" style="padding-right: 8px;"></el-input>
							<el-tag type="success" v-if="user.moblie_status=='1'">已认证</el-tag>
							<el-tag type="danger" v-else>未认证</el-tag>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>邮箱</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.email" :disabled="true" style="padding-right: 8px;"></el-input>
							<el-tag type="success" v-if="user.email_status=='1'">已认证</el-tag>
							<el-tag type="danger" v-else>未认证</el-tag>
					    </div>
					</div>
					<div class="drawerModLis" v-if="info.usertype=='1'">
					    <div class="drawerModTite">
					        <span>身份证</span>
					    </div>
					    <div class="drawerModInpt">
							<el-tag type="success" v-if="user.idcard_status=='1'">已认证</el-tag>
							<el-tag type="danger" v-else>未认证</el-tag>
					    </div>
					</div>
					<div class="drawerModLis" v-else>
					    <div class="drawerModTite">
					        <span>执照</span>
					    </div>
					    <div class="drawerModInpt">
							<el-tag type="success" v-if="user.yyzz_status=='1'">已认证</el-tag>
							<el-tag type="danger" v-else>未认证</el-tag>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>登录次数</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.login_hits" :disabled="true"></el-input>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>注册时间</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.reg_date_ymd" :disabled="true"></el-input>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>最近登录时间</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.login_date_ymd" :disabled="true"></el-input>
					    </div>
					</div>
					<div class="drawerModLis">
					    <div class="drawerModTite">
					        <span>现居住地</span>
					    </div>
					    <div class="drawerModInpt">
					        <el-input v-model="info.address" :disabled="true"></el-input>
					    </div>
					</div>
					<div class="setBasicButn" style="border: none;">
					    <el-button type="primary" @click="set(info)">确 定</el-button>
					    <el-button type="primary" @click="resetPassword(info)">重置密码</el-button>
					    <el-button type="primary" @click="del(info)">删 除</el-button>
					</div>
				</div>
			</el-drawer>
		</div>
		<!--发短信-->
		<div class="modluDrawer">
			<el-dialog :title="title" :visible.sync="sendInfodrawer" :append-to-body="true" width="450px">
                <template v-if="isEmail">
                    <div class="wxsettip_small ">邮件标题：</div>
                    <el-input v-model="emailTitle" placeholder=""></el-input>
                </template>
                <div class="wxsettip_small ">{{userTitle}}</div>
                <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="textarea">
                </el-input>
				<span slot="footer" class="dialog-footer">
					<el-button @click="sendInfodrawer = false">取 消</el-button>
					<el-button type="primary" @click="saveSendInfo" :disabled="saveLoading">确 定</el-button>
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
            checkedAll:false,
            search:{
				status: this.status,
                keyword:'',
                appealstate:'',
            },
            title:'发送短信',
			input3: '',
			input: '',
			select: '',
			value: true,
			value1: '',
			checked: '',
			drawer: false,
            detaildrawer: false,
			tableHig: true,
			currentPage4: 4,
			tableData: [],
            promiss:{},
			items: [
				{ type: '', label: '正常' }, 
			],
            idsArr:[],
            total:0,
            uri: "m=user&c=",
            page: 1,
            pageSize: 0,
            detail:{},
            memNum:{},
            //  弹窗
            sendInfodrawer:false,
            isEmail: false,
            textarea:'',
            emailTitle:'',
            userTitle:'短信内容：',
            userInfoDetail:{},
            info:{},
            user:{},
            pageSizes:[],
            saveLoading: false,

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
		shortChange(e) {
		    let orderMap = {ascending: 'asc', descending: 'desc'}
		    this.search.t = e.order ? e.prop == "appealtime_ymd"? 'appealtime': e.prop : null;
		    this.search.order = orderMap[e.order];
		    this.search.page = 1;
		    this.getList();
		},
        selectAllBottom(value) {
            value ? this.$refs.multipleTable.toggleAllSelection() : this.$refs.multipleTable.clearSelection();
        },
        lockList:function(){
            this.search.status = 2;
            this.page = 1
            this.getList()
        },
        selectChange:function (val) {
            this.idsArr = [];
            let _this = this;
            if (val.length) {
                val.forEach(item => {
                    _this.idsArr.push(item.uid);
                });
            }
        },
        doUserQuery() {
            this.page = 1
            this.getList()
        },
        userPageChange(val) {
            this.page = val
            this.getList()
        },
        handleSizeChange(val) {
            this.pageSize = val;
            this.getList();
        },
        getList:function(){
            let _this = this;
            let url = _this.uri + 'admin_appeal&a=index';
            _this.search.page = this.page;
            _this.search.pageSize = this.pageSize;
            _this.loading = true;
            httpPost(url, _this.search, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.tableData = res.data.data;
                    _this.total = res.data.total;
                    _this.promiss = res.data.promiss;
                    _this.pageSizes =res.data.pageSizes;
                    _this.loading = false;
                    if(_this.prevPage != _this.page){
						_this.prevPage = _this.page;
						_this.$refs.multipleTable.bodyWrapper.scrollTop = 0;
					}
                    if (_this.tableData.length === 0) {
	                    _this.dataText = "暂无数据";
	                }
                }
            })
        },
		handleCurrentChange(val) {
			console.log(`当前页: ${val}`);
		},
        set:function (detail) {
            let _this = this,
				params = {};
			params.id = detail.uid;
            let url = this.uri + 'admin_appeal&a=success';
			let msg = '确定申诉已完成?';
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						message.success(res.data.msg, function () {
							_this.detaildrawer = false;
							_this.getList();
						});
					}
				})
			}, msg);
        },
        resetPassword:function (detail){
            let username = detail.username;
            let _this = this,
            	params = {};
            params.uid = detail.uid;
            let url = this.uri + 'admin_member&a=reset_pw';
			let msg = '确定要重置密码吗?';
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						message.success("用户："+username+" 密码已经重置为123456！", function () {
							_this.detaildrawer = false;
							_this.getList();
						});
					}
				})
			}, msg);
        },
        detailFun:function (detail){
            let _this = this;
            let url = this.uri + 'admin_appeal&a=info';
            httpPost(url, { id: detail.uid }).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    _this.detaildrawer = true;
                    _this.info = res.data.info;
                    _this.user = res.data.info;
                }else {
                    message.error(res.msg);
                }
            })
        },
        del:function (detail) {
            let _this = this,
            	params = {};
            params.id = detail.uid;
            let url = this.uri + 'admin_appeal&a=del';
			let msg = '确定要删除?';
			delConfirm(_this, params, function (params) {
				httpPost(url, params).then(function(res) {
					if (res.data.error > 0) {
						message.error(res.data.msg);
					} else {
						message.success(res.data.msg, function () {
							_this.detaildrawer = false;
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
            let url = this.uri + 'admin_member&a=del'
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
        send_moblie:function($detail){
            this.detail = $detail
            this.title = '发送消息';
            this.sendInfodrawer = true;
            this.isEmail = false;
            this.userTitle = '短信内容：';
            this.textarea = '';
        },
        send_email:function ($detail) {
            this.detail = $detail
            this.title = '发送邮件';
            this.sendInfodrawer = true;
            this.isEmail = true;
            this.userTitle ='邮件标题：';
            this.textarea  = '';
            this.emailTitle  = '';
        },
        saveSendInfo:function (){
            let _this = this;
            let sendData = {};
            let url  = '';
            if (this.isEmail){
                sendData = {
                    utype :5,
                    email_title : this.emailTitle,
                    content : this.textarea,
                    email_user : this.detail.email,
                    pagelimit : 20,
                    value : 0,
                    sendok : 0,
                    sendno : 0,
                }
                url = this.uri + 'admin_member&a=send';
            }else{
                sendData = {
                    utype :5,
                    content : this.textarea,
                    userarr : this.detail.moblie,
                    pagelimit : 50,
                    value : 0,
                    sendok : 0,
                    sendno : 0,
                }
                url = this.uri + 'admin_member&a=msgsave';
            }
			_this.saveLoading = true;
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg, function(){
						_this.getList();
					});
                }else {
                    message.error(res.msg);
                }
            }).finally(function () {
				_this.saveLoading = false;
			});
        }
	},
};
</script>
<style scoped>
	
.moduleElHight .moduleElTable {
	padding: 0;
	margin: 0;
	height: calc(100% - 100px);
	width: 100%;
}
   .moduleElTableHig {
       height: calc(100% - 95px) !important;
   }
</style> 