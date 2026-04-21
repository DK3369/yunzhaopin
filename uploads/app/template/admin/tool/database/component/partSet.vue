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
                        <div class="TableTite">兼职状态：</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="locoy_config.locoy_partjob_status">
                                <el-radio label="1">已通过</el-radio>
                                <el-radio label="0">未审核</el-radio>
                            </el-radio-group>
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
                        <div class="TableTite">工作类型无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_part_type" placeholder="请选择">
                                <el-option v-for="type in typeOptions" :key="type.value" :label="type.label" :value="type.value"></el-option>
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
                        <div class="TableTite">薪水无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_part_salary" @input="inputIntNumber($event, 'locoy_config', 'locoy_part_salary')" placeholder=" ">
                                <template slot="append">元/天</template>
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
                        <div class="TableTite">结算周期无法匹配为</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_part_billing" placeholder="请选择">
                                <el-option v-for="item in billingOptions" :key="item.value" :label="item.label" :value="item.value"></el-option>
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
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-input v-model="locoy_config.locoy_part_hits" @input="inputIntNumber($event, 'locoy_config', 'locoy_part_hits')" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>>如：0-100，默认为0</span>
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
            part_set: Number
        },
        watch: {
            locoy_config: {
                handler (n, v){
                },
                deep: true
            },
            part_set: {
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
                typeOptions: [],
                billingOptions: [],
                saveLoading: false
            }
        },
        methods: {
            inputIntNumber(val, form, key) {
                this.$props[form][key] = val.replace(/[^0-9]/g,'');
            },
            async getCache() {
                let res = await httpPost('m=tool&c=dataCollection&a=getPartCache');
                if (res.data.error == 0) {
                    let data = res.data.data;

                    var partTypeArr = data.partTypeArr;
                    partTypeArr.forEach((item) => {
                        this.typeOptions.push({value: item.id, label: item.name})
                    });
                    var billingCycleArr = data.billingCycleArr;
                    billingCycleArr.forEach((item) => {
                        this.billingOptions.push({value: item.id, label: item.name})
                    });
                }
            },
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                     locoyConfig: 1,

                    locoy_partjob_status: that.locoy_config.locoy_partjob_status,
                    locoy_part_type: that.locoy_config.locoy_part_type,
                    locoy_part_salary: that.locoy_config.locoy_part_salary,
                    locoy_part_billing: that.locoy_config.locoy_part_billing,
                    locoy_part_hits: that.locoy_config.locoy_part_hits
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