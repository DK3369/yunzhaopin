<template>
	<div class="tableDome" style="position: relative;">
		<div class="tableDome_tip">
			<el-alert
					title="计划任务是一项使系统在规定时间自动执行某些特定任务的功能可通过“计划任务”实现自动刷新职位、简历刷新和生成XML操作。计划任务可以设置计划类型按“天、周、月”等方式去执行！"
					type="success"
					:closable="false">
			</el-alert>
		</div>
		<div class="moduleTable">
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
						<div class="TableTite">任务名称</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="请输入内容" v-model="info.name">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>如：定期修改职位</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">执行文件</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="请输入内容" v-model="info.dir">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>执行文件名 如：index.php,文件存放路径/app/include/cron/</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">类型</div>
					</td>
					<td>
						<div class="TableButn">
							<el-radio v-model="info.type" label="1">每周</el-radio>
							<el-radio v-model="info.type" label="2">每月</el-radio>
							<el-radio v-model="info.type" label="3">每天</el-radio>
							<el-radio v-model="info.type" label="5">每隔N分钟</el-radio>
							<el-radio v-model="info.type" label="4">每隔N秒</el-radio>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>类型</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 1">
					<td>
						<div class="TableTite">每周</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.week" placeholder="请选择">
								<el-option v-for="item in weekday" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>每周</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 2">
					<td>
						<div class="TableTite">每月</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.month" placeholder="请选择">
								<el-option v-for="item in monthday" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>每月</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type < 4">
					<td>
						<div class="TableTite">小时</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.hour" placeholder="请选择">
								<el-option v-for="item in hour" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>小时</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type <= 3 || info.type == 5">
					<td>
						<div class="TableTite">
							<span v-if="info.type <= 3">分钟</span>
							<span v-else>每隔N分钟执行一次</span>
						</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="请输入内容" v-model="info.minute" @input="info.minute=info.minute.replace(/[^0-9]/g,'')">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span v-if="info.type <= 3">不填请留空，默认:00</span>
							<span v-else>必填</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 4">
					<td>
						<div class="TableTite">
							<span>每隔N秒执行一次</span>
						</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="请输入内容" v-model="info.minute">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>必填</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">是否启用</div>
					</td>
					<td>
						<div class="setBasicIput">
							<el-switch v-model="info.display" active-text="开启">
							</el-switch>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>是否启用</span>
						</div>
					</td>
				</tr>
				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
		</div>
	</div>
</template>
<!-- script -->
<script>
    let info = {type: 1, week: '0', hour: '0', minute: '', name: '', dir: '', display: false};
    module.exports = {
        props: {
            id_v: {
                type: String,
                default: ''
            }
        },
        data: function () {
            return {
                id: '',
                info: deepClone(info),
                monthday: [],
                weekday: [],
                hour: [],
                options: [],
				saveLoading: false
            }
        },
        watch: {
			id_v: {
				handler(val) {
					this.id = val;
					this.getInfo();
				},
				immediate: true,
				deep: true,
			},
		},
        methods: {
            handleClick(tab, event) {
                console.log(tab, event);
            },
            async getInfo() {
                var that = this
                let res = await httpPost('m=system&c=set_cron&a=info', { id: this.id });
                if (res.data.error == 0) {
                    let data = res.data.data;
                    if (that.id) {
                        that.info = data.row
                    }
                    that.monthday = data.montharr
                    that.weekday = data.arrweek
                    that.hour = data.hourarr
                }
            },
            save() {
                let that = this;
                let params = that.info
                if (that.id) {
                    params.id = that.id;
                }
                if (params.type == 5) {
                    params.minu = params.minute
                } else if (params.type == 4) {
                    params.second = params.minute
                }
				that.saveLoading = true;
                httpPost('m=system&c=set_cron&a=save', params).then(function (res) {
                    if (res.data.error == 0) {
                        message.success(res.data.msg,function(){
							that.$emit("child-event");
						});
                        
                    } else {
                        message.error(res.data.msg);
                    }
                }).finally(function () {
					setTimeout(function () {
						that.saveLoading = false;
					}, 2000);
				});
            },
        }
    };
</script>
