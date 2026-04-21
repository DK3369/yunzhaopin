<template>
	<div class="moduleElHight">

		<div class="moduleTable" style="max-height: calc(100% - (60px + 25px));">
			<div class="tableDome_tip">
				<el-alert
					title="如果整合失败导致后台无法进入或者出现前台用户混乱，请将根目录下/data/plus/config.php 打开找到 sy_pw_type 将PW_UC置空,再登录后台关闭PW_UC"
					:closable="false">
				</el-alert>
			</div>
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
							<div class="TableTite">整合UCenter </div>
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
							<div class="TableTite">数据库地址 UC_DBHOST</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_DBHOST">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>如：localhost</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">数据库用户名 UC_DBUSER</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_DBUSER">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>ucenter数据库用户名</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">数据库密码 UC_DBPW</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_DBPW">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>ucenter数据库密码</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">数据库名称 UC_DBNAME</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_DBNAME">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>如：ucenter</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">数据表前缀 UC_DBTABLEPRE</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_DBTABLEPRE">

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
							<div class="TableTite">通信密钥 UC_KEY</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_KEY">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>如：phpyun123456</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">Phpwind地址 UC_API</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_API">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>如：http://localhost/ucenter</span>
							</div>
						</td>
					</tr>

					<tr>
						<td>
							<div class="TableTite">Phpwind编码</div>
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
							<div class="TableTite">应用ID UC_APPID</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_APPID">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>如：7</span>
							</div>
						</td>
					</tr>
					<tr>
						<td>
							<div class="TableTite">UC APP UC_APP</div>
						</td>
						<td>
							<div class="TableInpt">
								<el-input placeholder="请输入内容" v-model="ucinfo.UC_APP">

								</el-input>
							</div>
						</td>
						<td>
							<div class="TableShuom">
								<span>如：20</span>
							</div>
						</td>
					</tr>


				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="post">提交</el-button>
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