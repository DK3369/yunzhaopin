<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="采集前务必设置自己的接口密码，以免被其他人利用。注意：这里所设置的参数，只作为没有值的情况下使用，若采集软件有值传输，会优先使用传输值。" type="success" :closable="false"></el-alert>
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
                        <div class="TableTite">从事行业无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_com_hy" placeholder="请选择">
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
                        <div class="TableTite">企业性质无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_pr" placeholder="请选择">
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
                        <div class="TableTite">企业地址无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_com_province" placeholder="请选择" @change="handelCityOneOption">
                                <el-option v-for="city1 in cityOne" :key="city1.value" :label="city1.label" :value="city1.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_com_city" placeholder="请选择" style="margin-left: 20px;" @change="handelCityTwoOption">
                                <el-option v-for="city2 in cityTwo" :key="city2.value" :label="city2.label" :value="city2.value"></el-option>
                            </el-select>
                            <el-select v-model="locoy_config.locoy_com_town" placeholder="请选择" style="margin-left: 20px;" @change="handelCityThreeOption">
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
                        <div class="TableTite">企业规模无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_job_mun" placeholder="请选择">
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
                        <div class="TableTite">注册资金无值为</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_com_money" placeholder="" @input="inputIntNumber($event, 'locoy_config', 'locoy_com_money')">
                                <template slot="append">万元</template>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>如：0-10000，默认为0</span>
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