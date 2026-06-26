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
                        <div class="TableTite">{yun:}t key='admin_tool_00060'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_length" @input="inputIntNumber($event, 'locoy_config', 'locoy_length')" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00065'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00061'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_name" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00066'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00062'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_pwd" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00064'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00133'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="locoy_config.locoy_user_status">
                                <el-radio label="1">{yun:}t key='wap_user_00165'{/yun}</el-radio>
                                <el-radio label="0">{yun:}t key='wap_user_00166'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00063'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-select v-model="locoy_config.locoy_rating" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="item in ratingOptions" :key="item.value" :label="item.label" :value="item.value"></el-option>
                            </el-select>

                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
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
            account_set: Number
        },
        watch: {
            locoy_config: {
                handler (n, v){
                },
                deep: true
            },
            account_set: {
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

                ratingOptions: [],
                saveLoading: false
            }
        },
        methods: {
            inputIntNumber(val, form, key) {
                this.$props[form][key] = val.replace(/[^0-9]/g,'');
            },
            async getCache() {
                let res = await httpPost('m=tool&c=dataCollection&a=getRating');
                if (res.data.error == 0) {
                    let data = res.data.data;

                    var ratingArr = data.ratingArr;
                    ratingArr.forEach((item) => {
                        this.ratingOptions.push({value: item.id, label: item.name})
                    });
                }
            },
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,

                    locoy_length: that.locoy_config.locoy_length,
                    locoy_name: that.locoy_config.locoy_name,
                    locoy_pwd: that.locoy_config.locoy_pwd,
                    locoy_user_status: that.locoy_config.locoy_user_status,
                    locoy_rating: that.locoy_config.locoy_rating
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