<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="" type="success" :closable="false"></el-alert>
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
                        <div class="TableTite">{{ lc('admin_tool_00233') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="gsd_config.sy_ip">
                                <el-radio :label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio :label="2">{{ lc('common.close') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_tool_00238') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00234') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="gsd_config.sy_ip_appkey" placeholder=" "></el-input>
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
                        <div class="TableTite">{{ lc('admin_tool_00235') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="gsd_config.sy_ip_appsecret" placeholder=" "></el-input>
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
                        <div class="TableTite">{{ lc('admin_tool_00239') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="rest_num" placeholder=" " :disabled="true"></el-input>
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
                <el-button type="primary" size="medium" @click="setIpAddressConfig" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
            </div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            gsd_config: Object,
            ip_num: Number
        },
        watch: {
            gsd_config: {
                handler(n, v) {
                },
                deep: true
            },
            ip_num: {
                handler(newValue, oldValue) {
                    if (newValue != oldValue) {
                        this.getRestNum();
                    }
                },
                deep: true,
                immediate: true
            }
        },
        data: function () {
            return {
                rest_num: 0,
                saveLoading: false,
            }
        },
        methods: {
            async getRestNum() {
                let that = this;
                let res = await httpPost('m=tool&c=gsdConfig&a=getRestNum', {type: 'ip'});
                if (res.data.error == 0) {
                    let data = res.data.data;
                    that.rest_num = data.rest_num;
                }
            },
            setIpAddressConfig: function () {
                let that = this;
                let params = {
                    gsdConfig: 1,
                    sy_ip: that.gsd_config.sy_ip,
                    sy_ip_appkey: that.gsd_config.sy_ip_appkey,
                    sy_ip_appsecret: that.gsd_config.sy_ip_appsecret
                };
                that.saveLoading = true;
                httpPost('m=tool&c=gsdConfig&a=setIpAddressConfig', params).then(function (res) {
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