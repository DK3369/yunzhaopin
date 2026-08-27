<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00059')" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('member_com_00110') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
							<el-radio-group v-model="locoy_config.locoy_resume_status">
								<el-radio label="1">{{ lc('member_user_00042') }}</el-radio>
								<el-radio label="0">{{ lc('wap_user_00166') }}</el-radio>
							</el-radio-group>
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
                        <div class="TableTite">{{ lc('admin_tool_00264') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_resume_hy" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="hy in hyOptions" :key="hy.value" :label="hy.label" :value="hy.value"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00265') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_resume_job1" :placeholder="lc('wap_user_00100')" @change="handelJobOneOption">
							<el-option v-for="job1 in jobOne" :key="job1.value" :label="job1.label" :value="job1.value"></el-option>
						</el-select>
							<el-select v-model="locoy_config.locoy_resume_son" :placeholder="lc('wap_user_00100')" style="margin-left: 20px;" @change="handelJobTwoOption">
								<el-option v-for="job2 in jobTwo" :key="job2.value" :label="job2.label" :value="job2.value"></el-option>
							</el-select>
							<el-select v-model="locoy_config.locoy_resume_post" :placeholder="lc('wap_user_00100')" style="margin-left: 20px;" @change="handelJobThreeOption">
								<el-option v-for="job3 in jobThree" :key="job3.value" :label="job3.label" :value="job3.value"></el-option>
							</el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00266') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
							<el-select v-model="locoy_config.locoy_resume_province" :placeholder="lc('wap_user_00100')" @change="handelCityOneOption">
								<el-option v-for="city1 in cityOne" :key="city1.value" :label="city1.label" :value="city1.value"></el-option>
							</el-select>
							<el-select v-model="locoy_config.locoy_resume_city" :placeholder="lc('wap_user_00100')" style="margin-left: 20px;" @change="handelCityTwoOption">
								<el-option v-for="city2 in cityTwo" :key="city2.value" :label="city2.label" :value="city2.value"></el-option>
							</el-select>
							<el-select v-model="locoy_config.locoy_resume_three" :placeholder="lc('wap_user_00100')" style="margin-left: 20px;" @change="handelCityThreeOption">
								<el-option v-for="city3 in cityThree" :key="city3.value" :label="city3.label" :value="city3.value"></el-option>
							</el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00267') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
							<el-input v-model="locoy_config.locoy_user_minsalary" @input="inputIntNumber($event, 'locoy_config', 'locoy_user_minsalary')" :placeholder="lc('wap_user_00013')" style="width: 187px;">
								<template #append>{{ lc('common_02056') }}</template>
							</el-input>
							<el-input v-model="locoy_config.locoy_user_maxsalary" @input="inputIntNumber($event, 'locoy_config', 'locoy_user_maxsalary')" :placeholder="lc('wap_user_00014')" style="width: 187px; margin-left: 20px;">
								<template #append>{{ lc('common_02056') }}</template>
							</el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00268') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_report" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="report in reportOptions" :key="report.value" :label="report.label" :value="report.value"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00252') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_resume_rand" @input="inputIntNumber($event, 'locoy_config', 'locoy_resume_rand')" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00255') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
            </div>
        </div>
    </div>
</template>

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

	export default {
		props: {
			locoy_config: Object,
			resume_set: Number
		},
		watch: {
			locoy_config: {
				handler (n, v){
				},
				deep: true
			},
			resume_set: {
				handler(newValue, oldValue) {
					if (newValue == 1) {
						this.getCache();
					}
				},
				deep: true,
				immediate: true
			}
		},
		data: function () {
			return {
				hyOptions: [],

				Job: [],
				jobOne:[],
				jobTwo:[],
				jobThree:[],

				City: [],
				cityOne:[],
				cityTwo:[],
				cityThree:[],

				reportOptions: [],
                saveLoading: false
			}
		},
		methods: {
            inputIntNumber(val, form, key) {
                this.$props[form][key] = val.replace(/[^0-9]/g,'');
            },
			async getCache() {
			    let that = this;
				let res = await httpPost('m=tool&c=dataCollection&a=getCache');
				if (res.data.error == 0) {
					let data = res.data.data;

					var industryArr = data.industryArr;
					industryArr.forEach((item) => {
						this.hyOptions.push({value: item.id, label: item.name})
					});

					this.Job = data.jobArr;
					var jobOneArr = data.jobOneArr;
					jobOneArr.forEach((item) => {
						this.jobOne.push({value: item.id, label: item.name})
					});

					this.City = data.cityArr;
					var provinceArr = data.provinceArr;
					provinceArr.forEach((item) => {
						this.cityOne.push({value: item.id, label: item.name})
					});

					var userReportArr = data.userReportArr;
					userReportArr.forEach((item) => {
						this.reportOptions.push({value: item.id, label: item.name})
					});

                    var job1_son = this.locoy_config.locoy_resume_son,
                        job_post = this.locoy_config.locoy_resume_post,
                        cityId = this.locoy_config.locoy_resume_city,
                        threeCityId = this.locoy_config.locoy_resume_three;

                    if (parseInt(this.locoy_config.locoy_resume_job1) > 0) {
                        this.handelJobOneOption(this.locoy_config.locoy_resume_job1);
                        if (parseInt(job1_son) > 0) {
                            setTimeout(function () {
                                that.handelJobTwoOption(job1_son);
                            }, 100)
                        }
                        if (parseInt(job_post) > 0) {
                            setTimeout(function () {
                                that.handelJobThreeOption(job_post);
                            }, 200)
                        }
                    }

                    if (parseInt(this.locoy_config.locoy_resume_province) > 0) {
                        this.handelCityOneOption(this.locoy_config.locoy_resume_province);
                        if (parseInt(cityId) > 0) {
                            setTimeout(function () {
                                that.handelCityTwoOption(cityId);
                            }, 100)
                        }
                        if (parseInt(threeCityId) > 0) {
                            setTimeout(function () {
                                that.handelCityThreeOption(threeCityId);
                            }, 200)
                        }
                    }
				}
			},
			handelJobOneOption: function (val) {
				this.jobTwo = [];
				this.jobThree = [];
				this.locoy_config.locoy_resume_son = '';
				this.locoy_config.locoy_resume_post = '';
				this.Job.forEach((item, index) => {
					if (item.pid == val) {

						this.jobTwo.push({value: item.id, label: item.name});
					}
				});
			},
			handelJobTwoOption: function (val) {
				this.jobThree = [];
				this.locoy_config.locoy_resume_son = val;
				this.locoy_config.locoy_resume_post = '';
				this.Job.forEach((item, index) => {
					if (item.pid == val) {

						this.jobThree.push({value: item.id, label: item.name});
					}
				});
			},
			handelJobThreeOption: function (val) {
				this.locoy_config.locoy_resume_post = val;
			},
			handelCityOneOption: function (val) {
				this.cityTwo = [];
				this.cityThree = [];
				this.locoy_config.locoy_resume_city = '';
				this.locoy_config.locoy_resume_three = '';
				this.City.forEach((item, index) => {
					if (item.pid == val) {

						this.cityTwo.push({value: item.id, label: item.name});
					}
				});
			},
			handelCityTwoOption: function (val) {
				this.cityThree = [];
				this.locoy_config.locoy_resume_city = val;
				this.locoy_config.locoy_resume_three = '';
				this.City.forEach((item, index) => {
					if (item.pid == val) {

						this.cityThree.push({value: item.id, label: item.name});
					}
				});
			},
			handelCityThreeOption: function (val) {
				this.locoy_config.locoy_resume_three = val;
			},
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,
                    locoy_resume_status: that.locoy_config.locoy_resume_status,
                    locoy_resume_hy: that.locoy_config.locoy_resume_hy,

                    locoy_resume_job1: that.locoy_config.locoy_resume_job1,
                    locoy_resume_son: that.locoy_config.locoy_resume_son,
                    locoy_resume_post: that.locoy_config.locoy_resume_post,

                    locoy_resume_province: that.locoy_config.locoy_resume_province,
                    locoy_resume_city: that.locoy_config.locoy_resume_city,
                    locoy_resume_three: that.locoy_config.locoy_resume_three,

                    locoy_user_minsalary: that.locoy_config.locoy_user_minsalary,
                    locoy_user_maxsalary: that.locoy_config.locoy_user_maxsalary,

                    locoy_user_report: that.locoy_config.locoy_user_report,
                    locoy_resume_rand: that.locoy_config.locoy_resume_rand
                };
                that.saveLoading = true;
                httpPost('m=tool&c=dataCollection&a=setLocoyConfig', params).then(function (res) {
                    if (res.data.error == 0) {

                        message.success(res.data.msg);
                    } else {

                        message.error(res.data.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
		},
	};
</script>
<style scoped>
    .moduleTable {
        max-height: calc(100% - (60px + 10px));
    }
</style>