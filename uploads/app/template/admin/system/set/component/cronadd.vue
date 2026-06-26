<template>
	<div class="tableDome" style="position: relative;">
		<div class="tableDome_tip">
			<el-alert
					title="{yun:}t key='admin_system_00254'{/yun}"
					type="success"
					:closable="false">
			</el-alert>
		</div>
		<div class="moduleTable">
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
						<div class="TableTite">{yun:}t key='admin_00892'{/yun}</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.name">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='admin_00893'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">{yun:}t key='admin_system_00262'{/yun}</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.dir">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='admin_system_00255'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">{yun:}t key='admin_system_00689'{/yun}</div>
					</td>
					<td>
						<div class="TableButn">
							<el-radio v-model="info.type" label="1">{yun:}t key='admin_system_00268'{/yun}</el-radio>
							<el-radio v-model="info.type" label="2">{yun:}t key='admin_system_00270'{/yun}</el-radio>
							<el-radio v-model="info.type" label="3">{yun:}t key='admin_system_00269'{/yun}</el-radio>
							<el-radio v-model="info.type" label="5">{yun:}t key='admin_system_00261'{/yun}</el-radio>
							<el-radio v-model="info.type" label="4">{yun:}t key='admin_00894'{/yun}</el-radio>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='admin_system_00689'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 1">
					<td>
						<div class="TableTite">{yun:}t key='admin_system_00268'{/yun}</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.week" placeholder="{yun:}t key='wap_user_00100'{/yun}">
								<el-option v-for="item in weekday" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='admin_system_00268'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 2">
					<td>
						<div class="TableTite">{yun:}t key='admin_system_00270'{/yun}</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.month" placeholder="{yun:}t key='wap_user_00100'{/yun}">
								<el-option v-for="item in monthday" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='admin_system_00270'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type < 4">
					<td>
						<div class="TableTite">{yun:}t key='wap_js_00128'{/yun}</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.hour" placeholder="{yun:}t key='wap_user_00100'{/yun}">
								<el-option v-for="item in hour" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='wap_js_00128'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type <= 3 || info.type == 5">
					<td>
						<div class="TableTite">
							<span v-if="info.type <= 3">{yun:}t key='wap_com_00247'{/yun}</span>
							<span v-else>{yun:}t key='admin_00895'{/yun}</span>
						</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.minute" @input="info.minute=info.minute.replace(/[^0-9]/g,'')">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span v-if="info.type <= 3">{yun:}t key='admin_00896'{/yun}</span>
							<span v-else>{yun:}t key='member_user_00011'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 4">
					<td>
						<div class="TableTite">
							<span>{yun:}t key='admin_00897'{/yun}</span>
						</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.minute">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='member_user_00011'{/yun}</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">{yun:}t key='admin_system_00263'{/yun}</div>
					</td>
					<td>
						<div class="setBasicIput">
							<el-switch v-model="info.display" active-text="开启">
							</el-switch>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{yun:}t key='admin_system_00263'{/yun}</span>
						</div>
					</td>
				</tr>
				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
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
