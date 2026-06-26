<template>
	<div class="moduleElHight">

		<div class="moduleTable" style="max-height: calc(100% - (60px + 25px));">
			<div class="tableDome_tip">
				<el-alert
					title="{yun:}t key='admin_tool_00442'{/yun}"
					:closable="false">
				</el-alert>
			</div>
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
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00441'{/yun} </div>
						</td>
						<td>
							<div class="TableButn">
								<el-switch v-model="config.sy_pw_type" active-text="开启" inactive-text="关闭" active-value="pw_center" inactive-value=""></el-switch>
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
							<div class="TableTite">{yun:}t key='admin_tool_00445'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_DBHOST">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00457'{/yun}</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00446'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_DBUSER">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00447'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00448'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_DBPW">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00449'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00450'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_DBNAME">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00458'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00451'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_DBTABLEPRE">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>pre_ucenter_</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00452'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_KEY">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00459'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00463'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_API">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00460'{/yun}</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00464'{/yun}</div>
						</td>
						<td>
							<div class="TableButn">
								<el-radio v-model="ucinfo.UC_CHARSET" label="utf8"></el-radio>
								<el-radio v-model="ucinfo.UC_CHARSET" label="gbk"></el-radio>
							</div>
						</td>
						<td>
							<div class="TableShuom">

							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{yun:}t key='admin_tool_00455'{/yun}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_APPID">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00461'{/yun}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">UC APP UC_APP</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="ucinfo.UC_APP">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{yun:}t key='admin_tool_00462'{/yun}</span>
							</div>
						</td>
					</tr>


				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="post">{yun:}t key='common.submit'{/yun}</el-button>
		</div>

	</div>
</template>

<script>
	module.exports = {
		props:{
			configdata: {
				default: {}
			},
			ucinfodata: {
				default: {}
			},
		},
		watch:{
			configdata:{
				handler(val, oldVal) {
					this.config = val;
				},
				immediate: true,
				deep:true,
			},
			ucinfodata:{
				handler(val, oldVal) {
					this.ucinfo = val;
				},
				immediate: true,
				deep:true,
			}
		},
		data: function() {
			return {
				config:{},
				ucinfo:{}
			}
		},

		methods: {
			post:function(){
				this.$emit('pw-post',{config:this.config,ucinfo:this.ucinfo});
			}
		},
	};
</script>
<style scoped>

</style>