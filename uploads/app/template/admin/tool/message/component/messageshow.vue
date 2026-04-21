<template>
	<div class="moduleElHight">
		<div class="moduleTable" v-if="islook">
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
						<td colspan="3" style="padding:0px;">
							<el-divider content-position="left">短信内容支持长短信，最多500个字，65个字按一条短信计费</el-divider>
						</td>

					</tr>
					<tr>
						<td>
							<div class="TableTite">短信状态</div>
						</td>
						<td>
							<div class="TableButn">
								<el-switch v-model="sy_msg_isopen" active-text="开启" inactive-text="关闭" active-value="1"></el-switch>
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
							<div class="TableTite">appKey</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_msg_appkey"></el-input>
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
							<div class="TableTite">appSecret</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_msg_appsecret"></el-input>
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
							<div class="TableTite">单IP每日最大发送</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ip_msgnum" @input="inputIntNumber($event, 'ip_msgnum')">
									<template slot="append">条</template>
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
							<div class="TableTite">单手机号每日最大发送</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="moblie_msgnum" @input="inputIntNumber($event, 'moblie_msgnum')">
									<template slot="append">条</template>
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
							<div class="TableTite">单手机号认证类短信发送频率</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="cert_msgtime" @input="inputIntNumber($event, 'cert_msgtime')">
									<template slot="append">分钟</template>
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
							<div class="TableTite">短信验证码时效</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="moblie_codetime" @input="inputIntNumber($event, 'moblie_codetime')">
									<template slot="append">分钟</template>
								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span> 验证码类短信有效时长，建议大于两分钟。</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">剩余短信数量</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="rest_msgnum" :disabled="true">
									<template slot="append">条</template>
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
							<div class="TableTite">购买短信</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-link type="primary" href="https://u.phpyun.com/" target="_blank">购买地址</el-link>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span> </span>
							</div>
						</td>
					</tr>
					<tr>
						<td colspan="3" style="padding:0px;">
							<el-divider content-position="left">空号检测</el-divider>
						</td>

					</tr>


					<tr>
						<td>
							<div class="TableTite">空号检测</div>
						</td>
						<td>
							<div class="TableButn">
								<el-switch v-model="sy_kh_isopen" active-text="开启" inactive-text="关闭" active-value="1"></el-switch>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>无效手机号、空号禁止注册发送验证码</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">appKey</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_kh_appkey">

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
							<div class="TableTite">appSecret</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_kh_appsecret">

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
							<div class="TableTite">手机归属地限制</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_kh_city">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>归属地以外的手机号禁止注册,省市以 / 分隔，多个地区以英文逗号 , 分隔，如：北京,江苏/南京,浙江/杭州 , 留空则不限制</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">剩余检测量</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="rest_khnum" :disabled="true">
									<template slot="append">条</template>
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
						<td colspan="3" style="padding:0px;">
							<el-divider content-position="left">天眼查</el-divider>
						</td>

					</tr>
					<tr>
						<td>
							<div class="TableTite">appKey</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_tyc_appkey">

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
							<div class="TableTite">appSecret</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="sy_tyc_appsecret">

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
							<div class="TableTite">天眼查数量</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="rest_businessnum" :disabled="true">
									<template slot="append">条</template>
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
							<div class="TableTite">购买天眼查</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-link type="primary" href="https://u.phpyun.com/" target="_blank">购买地址</el-link>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span> </span>
							</div>
						</td>
					</tr>


				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" :loading='post_loading' size="medium" @click="postSet">提交</el-button>
		</div>

	</div>
</template>

<script>
	module.exports = {
		data: function() {
			return {
				islook:false,

				sy_msg_isopen:'',
	            sy_msg_appkey:'',
	            sy_msg_appsecret:'',
	            ip_msgnum:'',
	            moblie_msgnum:'',
	            cert_msgtime:'',
	            moblie_codetime:'',
	            sy_kh_isopen:'',
	            sy_kh_appkey:'',
	            sy_kh_appsecret:'',
	            sy_kh_city:'',
	            sy_tyc_appkey:'',
	            sy_tyc_appsecret:'',
	            
	            rest_msgnum:0,
	            rest_businessnum:0,
	            rest_khnum:0,

				post_loading:false,
			}
		},

		mounted() {
			this.getInfo();
		},

		methods: {
			inputIntNumber(val, form) {
                this.$data[form] = val.replace(/[^0-9]/g,'');
            },
			async getInfo() {
                let that = this;
                
                startLoading();

                httpPost('m=tool&c=messageset&a=index',{}).then((result)=>{
                    endLoading();
                    
                    var res = result.data;
                    if (res.error == 0) {
                        that.sy_msg_isopen = res.data.sy_msg_isopen;
                        that.sy_msg_appkey = res.data.sy_msg_appkey;
                        that.sy_msg_appsecret = res.data.sy_msg_appsecret;

                        that.ip_msgnum = res.data.ip_msgnum;
			            that.moblie_msgnum = res.data.moblie_msgnum;
			            that.cert_msgtime = res.data.cert_msgtime;
			            that.moblie_codetime = res.data.moblie_codetime;
			            that.sy_kh_isopen = res.data.sy_kh_isopen;
			            that.sy_kh_appkey = res.data.sy_kh_appkey;
			            that.sy_kh_appsecret = res.data.sy_kh_appsecret;
			            that.sy_kh_city = res.data.sy_kh_city;
			            that.sy_tyc_appkey = res.data.sy_tyc_appkey;
			            that.sy_tyc_appsecret = res.data.sy_tyc_appsecret;
			            
			            that.rest_msgnum = res.data.rest_msgnum;
			            that.rest_businessnum = res.data.rest_businessnum;
			            that.rest_khnum = res.data.rest_khnum;
					}
                    that.islook = true;
                }).catch(function(e){
                    console.log(e)
                })
            },
            async postSet(){

                let that = this;
                
                if (parseInt(that.moblie_codetime) < 2) {

                    message.warning("短信验证时效，建议大于两分钟！");
                    return false;
                    
                }


                var param = {
                    sy_msg_isopen: that.sy_msg_isopen==1?1:2,
                    sy_msg_appkey: that.sy_msg_appkey,
                    sy_msg_appsecret: that.sy_msg_appsecret,


                    sy_kh_isopen: that.sy_kh_isopen==1?1:2,
                    sy_kh_appkey: that.sy_kh_appkey,
                    sy_kh_appsecret: that.sy_kh_appsecret,
                    sy_kh_city: that.sy_kh_city,

                    sy_tyc_appkey: that.sy_tyc_appkey,
                    sy_tyc_appsecret: that.sy_tyc_appsecret,

                    ip_msgnum: that.ip_msgnum,
                    moblie_msgnum: that.moblie_msgnum,
                    cert_msgtime: that.cert_msgtime,
                    moblie_codetime: that.moblie_codetime,
                };
                
                that.post_loading = true;
                
                httpPost('m=tool&c=messageset&a=save',param).then((result)=>{
                    var res = result.data;

                    message.success(res.msg,that.getInfo);

                }).catch(function(e){
                    console.log(e)
                }).finally(function () {
                    setTimeout(function () {
                        that.post_loading = false;
                    }, 2000);
                });
            },
		},
	};
</script>
<style scoped>

</style>