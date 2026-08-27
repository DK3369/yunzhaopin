<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00183')" type="success" :closable="false"></el-alert>
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
                        <div class="TableTite">{{ lc('admin_tool_00184') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_com_hy" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="hy in hyOptions" :key="hy.value" :label="hy.label" :value="hy.value"></el-option>
                            </el-select>

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
                        <div class="TableTite">{{ lc('admin_tool_00185') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_pr" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="pr in prOptions" :key="pr.value" :label="pr.label" :value="pr.value"></el-option>
                            </el-select>
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
                        <div class="TableTite">{{ lc('admin_tool_00186') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_com_province" :placeholder="lc('wap_user_00100')" @change="handelCityOneOption">
                                <el-option v-for="city1 in cityOne" :key="city1.value" :label="city1.label" :value="city1.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_com_city" :placeholder="lc('wap_user_00100')" style="margin-left: 20px;" @change="handelCityTwoOption">
                                <el-option v-for="city2 in cityTwo" :key="city2.value" :label="city2.label" :value="city2.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_com_town" :placeholder="lc('wap_user_00100')" style="margin-left: 20px;" @change="handelCityThreeOption">
                                <el-option v-for="city3 in cityThree" :key="city3.value" :label="city3.label" :value="city3.value"></el-option>
                            </el-select>
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
                        <div class="TableTite">{{ lc('admin_tool_00187') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_mun" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="mun in munOptions" :key="mun.value" :label="mun.label" :value="mun.value"></el-option>
                            </el-select>
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
                        <div class="TableTite">{{ lc('admin_tool_00188') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_com_money" placeholder="" @input="inputIntNumber($event, 'locoy_config', 'locoy_com_money')">
                                <template #append>{{ lc('wap_js_00004') }}</template>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00189') }}</span>
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
            com_set: Number
        },
        watch: {
            locoy_config: {
                handler (n, v){
                },
                deep: true
            },
            com_set: {
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
                prOptions: [],

                City: [],
                cityOne:[],
                cityTwo:[],
                cityThree:[],

                munOptions: [],
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

                    this.City = data.cityArr;
                    var provinceArr = data.provinceArr;
                    provinceArr.forEach((item) => {
                        this.cityOne.push({value: item.id, label: item.name})
                    });

                    var jobPrArr = data.jobPrArr;
                    jobPrArr.forEach((item) => {
                        this.prOptions.push({value: item.id, label: item.name})
                    });
                    var jobMunArr = data.jobMunArr;
                    jobMunArr.forEach((item) => {
                        this.munOptions.push({value: item.id, label: item.name})
                    });

                    var cityId = this.locoy_config.locoy_com_city,
                        threeCityId = this.locoy_config.locoy_com_town;

                    if (parseInt(this.locoy_config.locoy_com_province) > 0) {
                        this.handelCityOneOption(this.locoy_config.locoy_com_province);
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
            handelCityOneOption: function (val) {
                this.cityTwo = [];
                this.cityThree = [];
                this.locoy_config.locoy_com_city = '';
                this.locoy_config.locoy_com_town = '';
                this.City.forEach((item, index) => {
                    if (item.pid == val) {

                        this.cityTwo.push({value: item.id, label: item.name});
                    }
                });
            },
            handelCityTwoOption: function (val) {
                this.cityThree = [];
                this.locoy_config.locoy_com_city = val;
                this.locoy_config.locoy_com_town = '';
                this.City.forEach((item, index) => {
                    if (item.pid == val) {

                        this.cityThree.push({value: item.id, label: item.name});
                    }
                });
            },
            handelCityThreeOption: function (val) {
                this.locoy_config.locoy_com_town = val;
            },
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,

                    locoy_com_hy: that.locoy_config.locoy_com_hy,
                    locoy_job_pr: that.locoy_config.locoy_job_pr,

                    locoy_com_province: that.locoy_config.locoy_com_province,
                    locoy_com_city: that.locoy_config.locoy_com_city,
                    locoy_com_town: that.locoy_config.locoy_com_town,

                    locoy_job_mun: that.locoy_config.locoy_job_mun,
                    locoy_com_money: that.locoy_config.locoy_com_money
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