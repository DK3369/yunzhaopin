<template>
	<div class="tableDome" style="position: relative;">
		<div class="tableDome_tip">
			<el-alert
					:title="lc('admin_system_00254')"
					type="success"
					:closable="false">
			</el-alert>
		</div>
		<div class="moduleTable">
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
						<div class="TableTite">{{ lc('admin_00892') }}</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input :placeholder="lc('wap_user_00076')" v-model="info.name">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('admin_00893') }}</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">{{ lc('admin_system_00262') }}</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input :placeholder="lc('wap_user_00076')" v-model="info.dir">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('admin_system_00255') }}</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">{{ lc('admin_system_00689') }}</div>
					</td>
					<td>
						<div class="TableButn">
							<el-radio v-model="info.type" label="1">{{ lc('admin_system_00268') }}</el-radio>
							<el-radio v-model="info.type" label="2">{{ lc('admin_system_00270') }}</el-radio>
							<el-radio v-model="info.type" label="3">{{ lc('admin_system_00269') }}</el-radio>
							<el-radio v-model="info.type" label="5">{{ lc('admin_system_00261') }}</el-radio>
							<el-radio v-model="info.type" label="4">{{ lc('admin_00894') }}</el-radio>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('admin_system_00689') }}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 1">
					<td>
						<div class="TableTite">{{ lc('admin_system_00268') }}</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.week" :placeholder="lc('wap_user_00100')">
								<el-option v-for="item in weekday" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('admin_system_00268') }}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 2">
					<td>
						<div class="TableTite">{{ lc('admin_system_00270') }}</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.month" :placeholder="lc('wap_user_00100')">
								<el-option v-for="item in monthday" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('admin_system_00270') }}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type < 4">
					<td>
						<div class="TableTite">{{ lc('wap_js_00128') }}</div>
					</td>
					<td>
						<div class="TableSelect">
							<el-select v-model="info.hour" :placeholder="lc('wap_user_00100')">
								<el-option v-for="item in hour" :key="item.value" :label="item.label"
										   :value="item.value">
								</el-option>
							</el-select>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('wap_js_00128') }}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type <= 3 || info.type == 5">
					<td>
						<div class="TableTite">
							<span v-if="info.type <= 3">{{ lc('wap_com_00247') }}</span>
							<span v-else>{{ lc('admin_00895') }}</span>
						</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input :placeholder="lc('wap_user_00076')" v-model="info.minute" @input="info.minute=info.minute.replace(/[^0-9]/g,'')">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span v-if="info.type <= 3">{{ lc('admin_00896') }}</span>
							<span v-else>{{ lc('member_user_00011') }}</span>
						</div>
					</td>
				</tr>
				<tr v-if="info.type == 4">
					<td>
						<div class="TableTite">
							<span>{{ lc('admin_00897') }}</span>
						</div>
					</td>
					<td>
						<div class="TableInpt">
							<el-input :placeholder="lc('wap_user_00076')" v-model="info.minute">
							</el-input>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('member_user_00011') }}</span>
						</div>
					</td>
				</tr>
				<tr>
					<td>
						<div class="TableTite">{{ lc('admin_system_00263') }}</div>
					</td>
					<td>
						<div class="setBasicIput">
							<el-switch v-model="info.display" :active-text="lc('member_com_00287')">
							</el-switch>
						</div>
					</td>
					<td>
						<div class="TableShuom">
							<span>{{ lc('admin_system_00263') }}</span>
						</div>
					</td>
				</tr>
				</tbody>
			</table>
		</div>
		<div class="setBasicButn" style="border: none;">
			<el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
		</div>
	</div>
</template>
<!-- script -->
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

    let info = {type: 1, week: '0', hour: '0', minute: '', name: '', dir: '', display: false};
    export default {
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
