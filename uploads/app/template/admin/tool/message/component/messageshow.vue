<template>
	<div class="moduleElHight">
		<div class="moduleTable" v-if="islook">
			<table class="tableVue">
				<thead>
					<tr align="left">
						<th width="200">{yun:}t key='member_com_00021'{/yun}</th>
						<th width="500">{yun:}t key='member_user_00181'{/yun}</th>
						<th>{yun:}t key='member_com_00207'{/yun}</th>
					</tr>
				</thead>
				<tbody>
					<tr>
						<td colspan="3" style="padding:0px;">
							<el-divider content-position="left">{yun:}t key='admin_tool_00474'{/yun}</el-divider>
						</td>

					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00492'{/yun}</div>
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
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_msg_appkey"></el-input>
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
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_msg_appsecret"></el-input>
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
							<div class="TableTite">{yun:}t key='admin_tool_00481'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ip_msgnum" @input="inputIntNumber($event, 'ip_msgnum')">
									<template slot="append">{yun:}t key='common_02088'{/yun}</template>
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
							<div class="TableTite">{yun:}t key='admin_tool_00480'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="moblie_msgnum" @input="inputIntNumber($event, 'moblie_msgnum')">
									<template slot="append">{yun:}t key='common_02088'{/yun}</template>
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
							<div class="TableTite">{yun:}t key='admin_tool_00478'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="cert_msgtime" @input="inputIntNumber($event, 'cert_msgtime')">
									<template slot="append">{yun:}t key='wap_com_00247'{/yun}</template>
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
							<div class="TableTite">{yun:}t key='admin_tool_00483'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="moblie_codetime" @input="inputIntNumber($event, 'moblie_codetime')">
									<template slot="append">{yun:}t key='wap_com_00247'{/yun}</template>
								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span> {yun:}t key='admin_tool_00475'{/yun}</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_00096'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="rest_msgnum" :disabled="true">
									<template slot="append">{yun:}t key='common_02088'{/yun}</template>
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
							<div class="TableTite">{yun:}t key='admin_tool_00490'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-link type="primary" href="https://u.ov6.com/" target="_blank">{yun:}t key='admin_tool_00491'{/yun}</el-link>
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
							<el-divider content-position="left">{yun:}t key='admin_tool_00487'{/yun}</el-divider>
						</td>

					</tr>


					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00487'{/yun}</div>
						</td>
						<td>
							<div class="TableButn">
								<el-switch v-model="sy_kh_isopen" active-text="开启" inactive-text="关闭" active-value="1"></el-switch>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00476'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">appKey</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_kh_appkey">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00479'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">appSecret</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_kh_appsecret">

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
							<div class="TableTite">{yun:}t key='admin_tool_00482'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_kh_city">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00473'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00484'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="rest_khnum" :disabled="true">
									<template slot="append">{yun:}t key='common_02088'{/yun}</template>
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
							<el-divider content-position="left">{yun:}t key='admin_tool_00488'{/yun}</el-divider>
						</td>

					</tr>
					<tr>
						<td>
							<div class="TableTite">appKey</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_tyc_appkey">

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
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="sy_tyc_appsecret">

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
							<div class="TableTite">{yun:}t key='admin_tool_00485'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="rest_businessnum" :disabled="true">
									<template slot="append">{yun:}t key='common_02088'{/yun}</template>
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
							<div class="TableTite">{yun:}t key='admin_tool_00486'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-link type="primary" href="https://u.ov6.com/" target="_blank">{yun:}t key='admin_tool_00491'{/yun}</el-link>
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
			<el-button type="primary" :loading='post_loading' size="medium" @click="postSet">{yun:}t key='common.submit'{/yun}</el-button>
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

                    message.warning(window.yunAdminT("{yun:}t key='admin_tool_00477'{/yun}"));
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