<template>
	<div class="moduleElHight" v-if="islook">
		<div class="tableDome_tip">
			<el-alert :title="lc('admin_tool_00354')" type="success" :closable="false">
			</el-alert>
		</div>
		<div class="moduleTable">
			<div>
				<div class="TableTite">{{ lc('admin_tool_00367') }}</div>
				<div class="TableButn">
					<el-radio v-model="sy_email_online" :label="1">{{ lc('admin_tool_00363') }}</el-radio>
					<el-radio v-model="sy_email_online" :label="2">{{ lc('admin_tool_00361') }}</el-radio>
				</div>
			</div>

			<table class="tableVue" v-if="sy_email_online==2">
				<thead>
					<tr align="left">
						<th width="200">{{ lc('member_com_00021') }}</th>
						<th width="500">{{ lc('member_user_00181') }}</th>
						<th>{{ lc('member_com_00207') }}</th>
					</tr>
				</thead>
				<tbody>
					
						<tr>
							<td>
								<div class="TableTite">AccessKey ID</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input :placeholder="lc('wap_user_00076')" v-model="accesskey">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									<span>{{ lc('admin_tool_00356') }}</span>
								</div>
							</td>
						</tr>
						<tr>
							<td>
								<div class="TableTite">Access Key Secret</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input :placeholder="lc('wap_user_00076')" v-model="accesssecret">

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
								<div class="TableTite">{{ lc('admin_tool_00369') }}</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input :placeholder="lc('wap_user_00076')" v-model="ali_email">

									</el-input>
								</div>
							</td>
							<td>
								<div class="TableShuom">
									<span>{{ lc('admin_tool_00357') }}</span>
								</div>
							</td>
						</tr>
						<tr>
							<td>
								<div class="TableTite">{{ lc('admin_user_company_00159') }}</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input :placeholder="lc('wap_user_00076')" v-model="ali_tag">

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
								<div class="TableTite">{{ lc('admin_tool_00368') }}</div>
							</td>
							<td>
								<div class="TableInpt">
									<el-input :placeholder="lc('wap_user_00076')" v-model="ali_name">

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
								<th width="200">{{ lc('member_com_00021') }}</th>
								<th width="500">{{ lc('member_user_00181') }}</th>
								<th>{{ lc('member_com_00207') }}</th>
							</tr>
						</thead>
						<tbody>
							
								
							<tr>
								<td>
									<div class="TableTite">{{ lc('admin_tool_00365') }}</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input :placeholder="lc('wap_user_00076')" v-model="eclist.smtpserver">

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>{{ lc('admin_tool_00479') }}</span>
									</div>
								</td>
							</tr>

							<tr>
								<td>
									<div class="TableTite">{{ lc('admin_tool_00362') }}</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input :placeholder="lc('wap_user_00076')" v-model="eclist.smtpuser">

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>{{ lc('admin_tool_00357') }}</span>
									</div>
								</td>
							</tr>

							<tr>
								<td>
									<div class="TableTite">{{ lc('admin_tool_00370') }}</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input :placeholder="lc('wap_user_00076')" v-model="eclist.smtppass" show-password>

										</el-input>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>{{ lc('admin_tool_00355') }}</span>
									</div>
								</td>
							</tr>
							<tr>
								<td>
									<div class="TableTite">{{ lc('admin_tool_00364') }}</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input :placeholder="lc('wap_user_00076')" v-model="eclist.smtpport"></el-input>
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
									<div class="TableTite">{{ lc('admin_tool_00368') }}</div>
								</td>
								<td>
									<div class="TableInpt">
										<el-input :placeholder="lc('wap_user_00076')" v-model="eclist.smtpnick">

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
									<div class="TableTite">{{ lc('admin_tool_00372') }}</div>
								</td>
								<td>
									<div class="TableButn">
										<el-switch v-model="eclist.default"   active-value="1" inactive-value="0"></el-switch>
									</div>
								</td>
								<td>
									<div class="TableShuom">
										<span>{{ lc('admin_tool_00436') }}</span>
									</div>
								</td>
							</tr>
							<tr v-if="eclist.isadd!=1">
								<td>
									<div class="TableTite">{{ lc('member_user_00048') }}</div>
								</td>
								<td>
									<div class="TableButn">
										<el-button type="text" size="medium" @click="delEmail(eclist.id)"><i class="el-icon-delete "></i> {{ lc('common.delete') }}</el-button>
										<el-button type="text" size="medium"  @click="testEamil(eclist.id)"><i class="el-icon-timer "></i> {{ lc('admin_tool_00371') }}</el-button>
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
			<el-button type="primary" size="medium" @click="postSet" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
			<el-button type="primary" size="medium" plain v-if="sy_email_online==1" @click="addSMTP">{{ lc('admin_user_company_00028') }}</el-button>
			<el-button type="primary" size="medium" plain v-if="sy_email_online==2" @click="testEamil('aliyun')">{{ lc('admin_tool_00371') }}</el-button>
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

                    message.warning(window.yunAdminT(lc('admin_tool_00373')));
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
                    message.warning(window.yunAdminT(lc('admin_tool_00359')));
                    return false;
				}else{
					this.$prompt(window.yunAdminT(lc('admin_tool_00366')), window.yunAdminT(lc('wap_js_00125')), {
						confirmButtonText: window.yunAdminT(lc('common.confirm')),
						cancelButtonText: window.yunAdminT(lc('common.cancel')),
						inputPattern: /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,4}$/,
						inputErrorMessage: window.yunAdminT(lc('admin_tool_00360')),
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
					message.warning(window.yunAdminT(lc('admin_tool_00358')));
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