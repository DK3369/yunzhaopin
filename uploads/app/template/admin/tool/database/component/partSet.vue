<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00059'{/yun}" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00257'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="locoy_config.locoy_partjob_status">
                                <el-radio label="1">{yun:}t key='member_user_00042'{/yun}</el-radio>
                                <el-radio label="0">{yun:}t key='wap_user_00166'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00258'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_part_type" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_tool_00259'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_part_salary" @input="inputIntNumber($event, 'locoy_config', 'locoy_part_salary')" placeholder=" ">
                                <template slot="append">{yun:}t key='admin_user_00350'{/yun}</template>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00260'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="locoy_config.locoy_part_billing" placeholder="{yun:}t key='wap_user_00100'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_tool_00252'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-input v-model="locoy_config.locoy_part_hits" @input="inputIntNumber($event, 'locoy_config', 'locoy_part_hits')" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00255'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
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