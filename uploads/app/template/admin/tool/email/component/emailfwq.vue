<template>
	<div class="moduleElHight" v-if="islook">
		<div class="tableDome_tip">
			<el-alert title="可以使用QQ/163等邮箱，465端口需要开启SSL服务，注意邮箱密码不是账户密码一般是独立授权密码" type="success" :closable="false">
			</el-alert>
		</div>
		<div class="moduleTable">
			<div>
				<div class="TableTite">邮件发送方式</div>
				<div class="TableButn">
					<el-radio v-model="sy_email_online" :label="1">SMTP服务器发送邮件</el-radio>
					<el-radio v-model="sy_email_online" :label="2">阿里云DirectMail</el-radio>
				</div>
			</div>

			<table class="tableVue" v-if="sy_email_online==2">
				<thead>
					<tr align="left">
						<th width="200">名称</th>
						<th width="500">状态</th>
						<th>说明</th>
					</tr>
				</thead>
				<tbody>
					
						<tr>
							<td>
								<div class="TableTite">AccessKey ID</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input placeholder="请输入内容" v-model="accesskey">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									<span>如：js4Blub11sd14BwQEe</span>
								</div>
							</td>
						</tr>
						<tr>
							<td>
								<div class="TableTite">Access Key Secret</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input placeholder="请输入内容" v-model="accesssecret">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									
								</div>
							</td>
						</tr>
						<tr>
							<td>
								<div class="TableTite">发信地址</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input placeholder="请输入内容" v-model="ali_email">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									<span>如：phpyun@qq.com</span>
								</div>
							</td>
						</tr>
						<tr>
							<td>
								<div class="TableTite">标签</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input placeholder="请输入内容" v-model="ali_tag">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									
								</div>
							</td>
						</tr>
						<tr>
							<td>
								<div class="TableTite">发件人昵称</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input placeholder="请输入内容" v-model="ali_name">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									
								</div>
							</td>
						</tr>
				</tbody>	
			</table>
			<div   v-if="sy_email_online==1">
				<div v-for="(eclist,index) in SMTPlist" :key="index">
					<table class="tableVue">
						<thead>
							<tr align="left">
								<th width="200">名称</th>
								<th width="500">状态</th>
								<th>说明</th>
							</tr>
						</thead>
						<tbody>
							
								
							<tr>
								<td>
									<div class="TableTite">SMTP服务器</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input placeholder="请输入内容" v-model="eclist.smtpserver">

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>如：smtp.qq.com</span>
									</div>
								</td>
							</tr>

							<tr>
								<td>
									<div class="TableTite">SMTP服务器的用户邮箱</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input placeholder="请输入内容" v-model="eclist.smtpuser">

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>如：phpyun@qq.com</span>
									</div>
								</td>
							</tr>

							<tr>
								<td>
									<div class="TableTite">邮箱密码</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input placeholder="请输入内容" v-model="eclist.smtppass" show-password>

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>此处密码一般为邮箱独立的授权密码而并非原始邮箱密码，具体可查看各邮箱设置详细信息</span>
									</div>
								</td>
							</tr>
							<tr>
								<td>
									<div class="TableTite">SMTP服务器端口</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input placeholder="请输入内容" v-model="eclist.smtpport"></el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span> </span>
									</div>
								</td>
							</tr>
							<tr>
								<td>
									<div class="TableTite">发件人昵称</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input placeholder="请输入内容" v-model="eclist.smtpnick">

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span> </span>
									</div>
								</td>
							</tr>

							<tr>
								<td>
									<div class="TableTite">启用邮箱</div>
								</td>
								<td>
									<div class="TableButn">
										<el-switch v-model="eclist.default"   active-value="1" inactive-value="0"></el-switch>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>QQ互联平台移动应用和网站应用是否已打通，未打通的请不要打开</span>
									</div>
								</td>
							</tr>
							<tr v-if="eclist.isadd!=1">
								<td>
									<div class="TableTite">操作</div>
								</td>
								<td>
									<div class="TableButn">
										<el-button type="text" size="medium" @click="delEmail(eclist.id)"><i class="el-icon-delete "></i> 删除</el-button>
										<el-button type="text" size="medium"  @click="testEamil(eclist.id)"><i class="el-icon-timer "></i> 测试</el-button>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										
									</div>
								</td>
							</tr>	
							
						</tbody>
					</table>
				 
				</div>
			</div>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="postSet" :disabled="saveLoading">提交</el-button>
			<el-button type="primary" size="medium" plain v-if="sy_email_online==1" @click="addSMTP">新增</el-button>
			<el-button type="primary" size="medium" plain v-if="sy_email_online==2" @click="testEamil('aliyun')">测试</el-button>
		</div>

	</div>
</template>

<script>
	module.exports = {
		data: function() {
			return {
				islook:false,
				SMTPlist:[],
				sy_email_online:'',
				accesskey:'',
				accesssecret:'',
				ali_email:'',
				ali_tag:'',
				ali_name:'',
				saveLoading: false
			}
		},

		mounted() {
			this.getInfo();
		},
		watch: {
			sy_email_online:{
				handler(val, oldVal) {
					
					if(val==1 && this.SMTPlist.length==0){
						this.addSMTP();
					}
				},
				immediate: true,
			},
		},
		methods: {
			async getInfo() {
                let that = this;
                
                startLoading();

                httpPost('m=tool&c=emailset&a=index',{}).then((result)=>{
                    endLoading();
                    
                    var res = result.data;
                    if (res.error == 0) {
                        that.SMTPlist = res.data.SMTPlist;
                        if(that.SMTPlist.length>0){
                        	for(let i in that.SMTPlist){
                        		that.SMTPlist[i]['isadd'] = 0;
                        	}
                        }
                        that.sy_email_online = res.data.sy_email_online;
                        that.accesskey = res.data.accesskey;
                        that.accesssecret = res.data.accesssecret;
                        that.ali_email = res.data.ali_email;
                        that.ali_tag = res.data.ali_tag;
                        that.ali_name = res.data.ali_name;

					}
                    that.islook = true;
                }).catch(function(e){
                    console.log(e)
                })
            },
            async postSet(){

                let that = this;
                
                if (that.sy_email_online == '') {

                    message.warning("请选择邮件发送方式！");
                    return false;
                    
                }

                if(that.sy_email_online==1){
                	var param = {
	                    sy_email_online:that.sy_email_online,
	                    content:JSON.stringify(that.SMTPlist)
					};
					
                }

                if(that.sy_email_online==2){
                	var param = {
	                    sy_email_online:that.sy_email_online,
						accesskey:that.accesskey,
						accesssecret:that.accesssecret,
						ali_email:that.ali_email,
						ali_tag:that.ali_tag,
						ali_name:that.ali_name
	                };
                }
                
                that.saveLoading = true;
                
                httpPost('m=tool&c=emailset&a=save',param).then((result)=>{
                    var res = result.data;
                    message.success(res.msg,that.getInfo);

                }).catch(function(e){
                    console.log(e)
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
            addSMTP:function(){

            	this.SMTPlist.push({
            		id:new Date().getTime(),
            		smtpserver:'',
            		smtpuser:'',
            		smtppass:'',
            		smtpport:'',
            		smtpnick:'',
            		default:'1',
            		isadd:1,
				});
            },
            testEamil:function(id){
            	var that = this;
            	if(!id){
            		message.warning("请选择需要测试的邮件服务器！");
                    return false;
				}else{
					this.$prompt('填写测试邮箱', '提示', {
						confirmButtonText: '确定',
						cancelButtonText: '取消',
						inputPattern: /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,4}$/,
						inputErrorMessage: '邮箱格式不正确，请重新输入！',
					}).then(({ value }) => {
						var param = {
							ceshi_email:value,
							id:id
						};
						that.emailCeshi(param);
					})
				}
            },
            async emailCeshi(param) {
                let that = this;
                
                startLoading();

                httpPost('m=tool&c=emailset&a=ceshi',param).then((result)=>{
                    endLoading();

                    var res = result.data;
                    if(res.error==0){
                    	message.success(res.msg);
                    }else{
                    	message.error(res.msg);
                    }
                }).catch(function(e){
                    console.log(e)
                })
            },
            delEmail:function(id){
            	var _this = this;
                if(!id){
					message.warning("请选择需要删除的邮件服务器！");
                    return false;
                }        
                var params = {
                    id: id
                };
                delConfirm(_this, params, this.deletePost)
            },
            async deletePost(params) {

                let that = this;

                httpPost('m=tool&c=emailset&a=delconfig', params).then(function (result) {

                    var res = result.data;
                    if (res.error == 0) {
                        message.success(res.msg,that.getInfo); return;
                    } else {
                        message.error(res.msg); return;
                    }
                }).catch(function (e) {
                    console.log(e)
                })
            },
		},
	};
</script>
<style scoped>
.tableVue{ margin-bottom:10px ;}
.TableButn{padding:20px 0}
</style>