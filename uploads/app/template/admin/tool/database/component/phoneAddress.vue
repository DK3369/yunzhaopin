<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="" type="success" :closable="false"></el-alert>
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
                        <div class="TableTite">手机号归属地</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio-group v-model="gsd_config.sy_mobile">
                                <el-radio :label="1">开启</el-radio>
                                <el-radio :label="2">关闭</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>关闭时不获取手机号归属地，开启后获取手机号归属地</span>
                        </div>
                    </td>
                </tr>


                <tr>
                    <td>
                        <div class="TableTite">APPKEY</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="gsd_config.sy_mobile_appkey" placeholder=" "></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span><el-link type="primary" href="https://u.phpyun.com" target="_blank">前往申请手机号归属地秘钥</el-link></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">APPSECRET</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="gsd_config.sy_mobile_appsecret" placeholder=" "></el-input>
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
                        <div class="TableTite">剩余数量</div>
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
                <el-button type="primary" size="medium" @click="setPhoneAddressConfig" :disabled="saveLoading">提交</el-button>
            </div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            gsd_config: Object,
            phone_num: Number
        },
        watch: {
            gsd_config: {
                handler(n, v) {
                },
                deep: true
            },
            phone_num: {
                handler(newValue, oldValue) {
                    if (newValue!= 0 && newValue != oldValue) {
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
                saveLoading: false
            }
        },
        methods: {
            async getRestNum() {
                let that = this;
                let res = await httpPost('m=tool&c=gsdConfig&a=getRestNum', {type: 'phone'},{hideloading: true});
                if (res.data.error == 0) {
                    let data = res.data.data;
                    that.rest_num = data.rest_num;
                }
            },
            setPhoneAddressConfig: function () {
                let that = this;
                let params = {
                    gsdConfig: 1,

                    sy_mobile: that.gsd_config.sy_mobile,
                    sy_mobile_appkey: that.gsd_config.sy_mobile_appkey,
                    sy_mobile_appsecret: that.gsd_config.sy_mobile_appsecret
                };
                that.saveLoading = true;
                httpPost('m=tool&c=gsdConfig&a=setPhoneAddressConfig', params).then(function (res) {
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