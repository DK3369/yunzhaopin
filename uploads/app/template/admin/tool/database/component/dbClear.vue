<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="{yun:}t key='admin_tool_00191'{/yun}" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="100">{yun:}t key='admin_tool_00192'{/yun}</th>
                    <th>{yun:}t key='member_user_00181'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00193'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-checkbox-group v-model="clearTable">
                                <el-checkbox label="userid_job">{yun:}t key='admin_user_company_00352'{/yun}</el-checkbox>
                                <el-checkbox label="userid_msg">{yun:}t key='admin_tool_00195'{/yun}</el-checkbox>
                                <el-checkbox label="down_resume">{yun:}t key='admin_tool_00196'{/yun}</el-checkbox>
                                <el-checkbox label="talent_pool">{yun:}t key='admin_tool_00197'{/yun}</el-checkbox>
                                <el-checkbox label="look_resume">{yun:}t key='admin_tool_00198'{/yun}</el-checkbox>
                                <el-checkbox label="look_job">{yun:}t key='admin_tool_00199'{/yun}</el-checkbox>
                                <el-checkbox label="email_msg">{yun:}t key='admin_tool_00200'{/yun}</el-checkbox>
                                <el-checkbox label="moblie_msg">{yun:}t key='admin_tool_00201'{/yun}</el-checkbox>
                                <el-checkbox label="member_log">{yun:}t key='admin_tool_00202'{/yun}</el-checkbox>
                                <el-checkbox label="sysmsg">{yun:}t key='wap_user_00363'{/yun}</el-checkbox>
                                <el-checkbox label="recycle">{yun:}t key='admin_tool_00190'{/yun}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00194'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-radio-group v-model="clearTime">
                                <el-radio label="730">{yun:}t key='admin_tool_00203'{/yun}</el-radio>
                                <el-radio label="360">{yun:}t key='admin_tool_00204'{/yun}</el-radio>
                                <el-radio label="180">{yun:}t key='admin_tool_00205'{/yun}</el-radio>
                                <el-radio label="90">{yun:}t key='admin_tool_00206'{/yun}</el-radio>
                                <el-radio label="60">{yun:}t key='admin_tool_00207'{/yun}</el-radio>
                                <el-radio label="30">{yun:}t key='admin_tool_00208'{/yun}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="checkData">{yun:}t key='common.submit'{/yun}</el-button>
            </div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        data: function () {
            return {

                clearTable: [],
                clearTime: 0
            }
        },
        mounted() {
        },
        methods: {
            checkData: function () {
                let that = this;
                let params = {};
                if (that.clearTable.length == 0){
                    message.error(window.yunAdminT("{yun:}t key='admin_tool_00209'{/yun}"));
                    return false;
                }
                if (that.clearTime == 0){
                    message.error(window.yunAdminT("{yun:}t key='admin_tool_00210'{/yun}"));
                    return false;
                }

                params.clearTime = that.clearTime;

                setTimeout(function () {

                    that.clearTable.forEach(function (item) {

                        console.log(item + window.yunAdminT("{yun:}t key='admin_tool_00211'{/yun}"));

                        params.clearTable = item;
                        that.clearData(params);

                        console.log(item + window.yunAdminT("{yun:}t key='admin_tool_00212'{/yun}"));
                    })
                    message.success(window.yunAdminT("{yun:}t key='admin_tool_00212'{/yun}"), function () {

                        that.clearTable = [];
                        that.clearTime = 0;
                    });
                }, 1000)
            },
            clearData(params){
                let that = this;
                httpPost('m=tool&c=database&a=clearData', params).then(function (res) {
                    let data = res.data;
                    if (data.error == 0) {
                        // console.log(data.msg);
                        return false;
                    } else if (data.error == 1){
                        // message.error(data.msg);
                        return false;
                    } else{
                        that.clearData(params);
                    }
                }).catch(function (error) {

                    console.log(error);
                })
            }
        },
    };
</script>
<style scoped>
    .moduleTable {max-height: calc(100% - (60px + 10px));}
</style>