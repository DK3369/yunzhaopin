<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="" type="success" :closable="false"></el-alert>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00233'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="gsd_config.sy_ip">
                                <el-radio :label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                <el-radio :label="2">{yun:}t key='common.close'{/yun}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00238'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00234'{/yun}</div>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00235'{/yun}</div>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00239'{/yun}</div>
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
                <el-button type="primary" size="medium" @click="setIpAddressConfig" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
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