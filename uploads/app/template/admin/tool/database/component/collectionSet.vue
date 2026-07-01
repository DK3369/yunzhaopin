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
                        <div class="TableTite">{{ lc('admin_tool_00173') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="locoy_config.locoy_online">
                                <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio label="2">{{ lc('common.close') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00172') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00174') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_key" placeholder=" "></el-input>
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
                        <div class="TableTite">{{ lc('admin_tool_00175') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="locoy_config.locoy_rate" placeholder=""></el-input>
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
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="submitLocoyConfig" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
            </div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            locoy_config: Object,
        },
        data: function () {
            return {
                saveLoading: false
            }
        },
        watch: {
            locoy_config: {
                handler (n, v){
                },
                deep: true
            }
        },
        methods: {
            submitLocoyConfig: function () {
                let that = this;
                let params = {
                    locoyConfig: 1,
                    locoy_online: that.locoy_config.locoy_online,
                    locoy_key: that.locoy_config.locoy_key,
                    locoy_rate: that.locoy_config.locoy_rate
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