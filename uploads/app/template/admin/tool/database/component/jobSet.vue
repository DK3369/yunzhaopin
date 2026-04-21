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
                    <th width="600">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">职位状态</div>
                    </td>
                    <td>
                        <div class="TableButn">
							<el-radio-group v-model="locoy_config.locoy_job_status">
								<el-radio label="1">已通过</el-radio>
								<el-radio label="0">未审核</el-radio>
							</el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>采集来的职位状态</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">职位发布日期</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-date-picker v-model="locoy_config.locoy_job_sdate" type="date" placeholder="选择日期" style="width: 187px;"></el-date-picker>
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
                        <div class="TableTite">从事行业无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_hy" placeholder="请选择" style="width: 187px;">
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
                        <div class="TableTite">职位类别无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_job1" placeholder="请选择" @change="handelJobOneOption">
                                <el-option v-for="job1 in jobOne" :key="job1.value" :label="job1.label" :value="job1.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_job1_son" placeholder="请选择" style="margin-left: 20px;" @change="handelJobTwoOption">
                                <el-option v-for="job2 in jobTwo" :key="job2.value" :label="job2.label" :value="job2.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_job_post" placeholder="请选择" style="margin-left: 20px;" @change="handelJobThreeOption">
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
                        <div class="TableTite">城市地区无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_province" placeholder="请选择" @change="handelCityOneOption">
                                <el-option v-for="city1 in cityOne" :key="city1.value" :label="city1.label" :value="city1.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_job_city" placeholder="请选择" style="margin-left: 20px;" @change="handelCityTwoOption">
                                <el-option v-for="city2 in cityTwo" :key="city2.value" :label="city2.label" :value="city2.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_job_three" placeholder="请选择" style="margin-left: 20px;" @change="handelCityThreeOption">
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
                        <div class="TableTite">薪水待遇无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-input v-model="locoy_config.locoy_minsalary" @input="inputIntNumber($event, 'locoy_config', 'locoy_minsalary')" placeholder="最低薪资" style="width: 187px;">
                                <template slot="append">元</template>
                            </el-input>
                            <el-input v-model="locoy_config.locoy_maxsalary" @input="inputIntNumber($event, 'locoy_config', 'locoy_maxsalary')" placeholder="最高薪资" style="width: 187px; margin-left: 20px;">
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
                        <div class="TableTite">招聘人数无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-input placeholder="招聘人数" v-model="locoy_config.locoy_com_number" @input="inputIntNumber($event, 'locoy_config', 'locoy_com_number')" style="width: 187px;">
                                <template slot="append">人</template>
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
                        <div class="TableTite">教育程度无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_edu" placeholder="请选择" style="width: 187px;">
                                <el-option v-for="je in eduOptions" :key="je.value" :label="je.label" :value="je.value"></el-option>
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
                        <div class="TableTite">工作经验无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_exp" placeholder="请选择" style="width: 187px;">
                                <el-option v-for="je in expOptions" :key="je.value" :label="je.label" :value="je.value"></el-option>
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
                        <div class="TableTite">年龄要求无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <div class="TableSelect" style="display: flex;align-items: center;">
                                <el-input v-model="locoy_config.locoy_min_age" @input="inputIntNumber($event, 'locoy_config', 'locoy_min_age')" placeholder="最小年龄" style="width: 187px;">
                                    <template slot="append">岁</template>
                                </el-input>
                                <el-input v-model="locoy_config.locoy_max_age" @input="inputIntNumber($event, 'locoy_config', 'locoy_max_age')" placeholder="最大年龄" style="width: 187px; margin-left: 20px;">
                                    <template slot="append">岁</template>
                                </el-input>
                            </div>
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
                        <div class="TableTite">性别要求无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_sexs" placeholder="请选择" style="width: 187px;">
                                <el-option v-for="cs in sexOptions" :key="cs.value" :label="cs.label" :value="cs.value"></el-option>
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
                        <div class="TableTite">婚姻状况无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_marriage" placeholder="请选择" style="width: 187px;">
                                <el-option v-for="jm in marriageOptions" :key="jm.value" :label="jm.label" :value="jm.value"></el-option>
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
                        <div class="TableTite">到岗时间无法匹配为：</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_report" placeholder="请选择" style="width: 187px;">
                                <el-option v-for="jr in reportOptions" :key="jr.value" :label="jr.label" :value="jr.value"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;height: 80px;">
            <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>

<script>
    module.exports = {
		props: {
			locoy_config: Object,
            job_set: Number
		},
		watch: {
			locoy_config: {
				handler (n, v){
				},
				deep: true
			},
            job_set: {
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

                eduOptions: [],
                expOptions: [],
                sexOptions: [],
                marriageOptions: [],
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

                    var jobEduArr = data.jobEduArr;
                    jobEduArr.forEach((item) => {
                        this.eduOptions.push({value: item.id, label: item.name})
                    });
                    var jobExpArr = data.jobExpArr;
                    jobExpArr.forEach((item) => {
                        this.expOptions.push({value: item.id, label: item.name})
                    });
                    var comSexArr = data.comSexArr;
                    comSexArr.forEach((item) => {
                        this.sexOptions.push({value: item.id, label: item.name})
                    });
                    var jobMarriageArr = data.jobMarriageArr;
                    jobMarriageArr.forEach((item) => {
                        this.marriageOptions.push({value: item.id, label: item.name})
                    });
                    var jobReportArr = data.jobReportArr;
                    jobReportArr.forEach((item) => {
                        this.reportOptions.push({value: item.id, label: item.name})
                    });

                    var job1_son = this.locoy_config.locoy_job1_son,
                        job_post = this.locoy_config.locoy_job_post,
                        cityId = this.locoy_config.locoy_job_city,
                        threeCityId = this.locoy_config.locoy_job_three;

                    if (parseInt(this.locoy_config.locoy_job_job1) > 0) {
                        this.handelJobOneOption(this.locoy_config.locoy_job_job1);
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

                    if (parseInt(this.locoy_config.locoy_job_province) > 0) {
                        this.handelCityOneOption(this.locoy_config.locoy_job_province);
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
                this.locoy_config.locoy_job1_son = '';
                this.locoy_config.locoy_job_post = '';
                this.Job.forEach((item, index) => {
                    if (item.pid == val) {

                        this.jobTwo.push({value: item.id, label: item.name});
                    }
                });
            },
            handelJobTwoOption: function (val) {
                console.log(val);
                this.jobThree = [];
                this.locoy_config.locoy_job1_son = val;
                this.locoy_config.locoy_job_post = '';
                this.Job.forEach((item, index) => {
                    if (item.pid == val) {

                        this.jobThree.push({value: item.id, label: item.name});
                    }
                });
            },
            handelJobThreeOption: function (val) {
                this.locoy_config.locoy_job_post = val;
            },
            handelCityOneOption: function (val) {
                this.cityTwo = [];
                this.cityThree = [];
                this.locoy_config.locoy_job_city = '';
                this.locoy_config.locoy_job_three = '';
                this.City.forEach((item, index) => {
                    if (item.pid == val) {

                        this.cityTwo.push({value: item.id, label: item.name});
                    }
                });
            },
            handelCityTwoOption: function (val) {
                console.log(val);
                this.cityThree = [];
                this.locoy_config.locoy_job_city = val;
                this.locoy_config.locoy_job_three = '';
                this.City.forEach((item, index) => {
                    if (item.pid == val) {

                        this.cityThree.push({value: item.id, label: item.name});
                    }
                });
            },
            handelCityThreeOption: function (val) {
                this.locoy_config.locoy_job_three = val;
            },
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,
                    locoy_job_status: that.locoy_config.locoy_job_status,
                    locoy_job_sdate: that.locoy_config.locoy_job_sdate,
                    locoy_job_hy: that.locoy_config.locoy_job_hy,

                    locoy_job_job1: that.locoy_config.locoy_job_job1,
                    locoy_job1_son: that.locoy_config.locoy_job1_son,
                    locoy_job_post: that.locoy_config.locoy_job_post,

                    locoy_job_province: that.locoy_config.locoy_job_province,
                    locoy_job_city: that.locoy_config.locoy_job_city,
                    locoy_job_three: that.locoy_config.locoy_job_three,

                    locoy_minsalary: that.locoy_config.locoy_minsalary,
                    locoy_maxsalary: that.locoy_config.locoy_maxsalary,

                    locoy_com_number: that.locoy_config.locoy_com_number,
                    locoy_job_edu: that.locoy_config.locoy_job_edu,
                    locoy_job_exp: that.locoy_config.locoy_job_exp,
                    locoy_min_age: that.locoy_config.locoy_min_age,
                    locoy_max_age: that.locoy_config.locoy_max_age,
                    locoy_job_sexs: that.locoy_config.locoy_job_sexs,
                    locoy_job_marriage: that.locoy_config.locoy_job_marriage,
                    locoy_job_report: that.locoy_config.locoy_job_report
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
        max-height: calc(100% - (120px));
    }
</style>