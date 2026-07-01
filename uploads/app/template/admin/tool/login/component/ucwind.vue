<template>
	<div class="moduleElHight">

		<div class="moduleTable" style="max-height: calc(100% - (60px + 25px));">
			<div class="tableDome_tip">
				<el-alert
					:title="lc('admin_tool_00442')"
					:closable="false">
				</el-alert>
			</div>
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
							<div class="TableTite">{{ lc('admin_tool_00441') }} </div>
						</td>
						<td>
							<div class="TableButn">
								<el-switch v-model="config.sy_pw_type" :active-text="lc('member_com_00287')" :inactive-text="lc('resume_00030')" active-value="pw_center" inactive-value=""></el-switch>
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
							<div class="TableTite">{{ lc('admin_tool_00445') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_DBHOST">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00457') }}</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">{{ lc('admin_tool_00446') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_DBUSER">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00447') }}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{{ lc('admin_tool_00448') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_DBPW">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00449') }}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{{ lc('admin_tool_00450') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_DBNAME">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00458') }}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{{ lc('admin_tool_00451') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_DBTABLEPRE">

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
							<div class="TableTite">{{ lc('admin_tool_00452') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_KEY">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00459') }}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">{{ lc('admin_tool_00463') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_API">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00460') }}</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">{{ lc('admin_tool_00464') }}</div>
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
							<div class="TableTite">{{ lc('admin_tool_00455') }}</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_APPID">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00461') }}</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">UC APP UC_APP</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input :placeholder="lc('wap_user_00076')" v-model="ucinfo.UC_APP">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>{{ lc('admin_tool_00462') }}</span>
							</div>
						</td>
					</tr>


				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="post">{{ lc('common.submit') }}</el-button>
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