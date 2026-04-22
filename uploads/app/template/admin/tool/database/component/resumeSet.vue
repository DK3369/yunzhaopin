<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="采集前务必设置自己的接口密码，以免被其他人利用，这里所设置的参数，只作为没有值的情况下使用，若采集软件有值传输，会优先使用传输值" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">简历状态</div>
                    </td>
                    <td>
                        <div class="TableButn">
							<el-radio-group v-model="locoy_config.locoy_resume_status">
								<el-radio label="1">已通过</el-radio>
								<el-radio label="0">未审核</el-radio>
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
                        <div class="TableTite">期望从事行业无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_resume_hy" placeholder="请选择">
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
                        <div class="TableTite">期望从事职位无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_resume_job1" placeholder="请选择" @change="handelJobOneOption">
							<el-option v-for="job1 in jobOne" :key="job1.value" :label="job1.label" :value="job1.value"></el-option>
						</el-select>
							<el-select v-model="locoy_config.locoy_resume_son" placeholder="请选择" style="margin-left: 20px;" @change="handelJobTwoOption">
								<el-option v-for="job2 in jobTwo" :key="job2.value" :label="job2.label" :value="job2.value"></el-option>
							</el-select>
							<el-select v-model="locoy_config.locoy_resume_post" placeholder="请选择" style="margin-left: 20px;" @change="handelJobThreeOption">
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
                        <div class="TableTite">期望工作地点无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
							<el-select v-model="locoy_config.locoy_resume_province" placeholder="请选择" @change="handelCityOneOption">
								<el-option v-for="city1 in cityOne" :key="city1.value" :label="city1.label" :value="city1.value"></el-option>
							</el-select>
							<el-select v-model="locoy_config.locoy_resume_city" placeholder="请选择" style="margin-left: 20px;" @change="handelCityTwoOption">
								<el-option v-for="city2 in cityTwo" :key="city2.value" :label="city2.label" :value="city2.value"></el-option>
							</el-select>
							<el-select v-model="locoy_config.locoy_resume_three" placeholder="请选择" style="margin-left: 20px;" @change="handelCityThreeOption">
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
                        <div class="TableTite">期望月薪无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
							<el-input v-model="locoy_config.locoy_user_minsalary" @input="inputIntNumber($event, 'locoy_config', 'locoy_user_minsalary')" placeholder="最低薪资" style="width: 187px;">
								<template slot="append">元</template>
							</el-input>
							<el-input v-model="locoy_config.locoy_user_maxsalary" @input="inputIntNumber($event, 'locoy_config', 'locoy_user_maxsalary')" placeholder="最高薪资" style="width: 187px; margin-left: 20px;">
								<template slot="append">元</template>
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
                        <div class="TableTite">到岗时间无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_user_report" placeholder="请选择">
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
                        <div class="TableTite">浏览数随机范围</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_resume_rand" @input="inputIntNumber($event, 'locoy_config', 'locoy_resume_rand')" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：0-100，默认为0</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">提交</el-button>
            </div>
        </div>
    </div>
</template>

<script>
	module.exports = {
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