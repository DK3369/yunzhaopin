<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_tool_00191')" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="100">{{ lc('admin_tool_00192') }}</th>
                    <th>{{ lc('member_user_00181') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00193') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-checkbox-group v-model="clearTable">
                                <el-checkbox label="userid_job">{{ lc('admin_user_company_00352') }}</el-checkbox>
                                <el-checkbox label="userid_msg">{{ lc('admin_tool_00195') }}</el-checkbox>
                                <el-checkbox label="down_resume">{{ lc('admin_tool_00196') }}</el-checkbox>
                                <el-checkbox label="talent_pool">{{ lc('admin_tool_00197') }}</el-checkbox>
                                <el-checkbox label="look_resume">{{ lc('admin_tool_00198') }}</el-checkbox>
                                <el-checkbox label="look_job">{{ lc('admin_tool_00199') }}</el-checkbox>
                                <el-checkbox label="email_msg">{{ lc('admin_tool_00200') }}</el-checkbox>
                                <el-checkbox label="moblie_msg">{{ lc('admin_tool_00201') }}</el-checkbox>
                                <el-checkbox label="member_log">{{ lc('admin_tool_00202') }}</el-checkbox>
                                <el-checkbox label="sysmsg">{{ lc('wap_user_00363') }}</el-checkbox>
                                <el-checkbox label="recycle">{{ lc('admin_tool_00190') }}</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_tool_00194') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-radio-group v-model="clearTime">
                                <el-radio label="730">{{ lc('admin_tool_00203') }}</el-radio>
                                <el-radio label="360">{{ lc('admin_tool_00204') }}</el-radio>
                                <el-radio label="180">{{ lc('admin_tool_00205') }}</el-radio>
                                <el-radio label="90">{{ lc('admin_tool_00206') }}</el-radio>
                                <el-radio label="60">{{ lc('admin_tool_00207') }}</el-radio>
                                <el-radio label="30">{{ lc('admin_tool_00208') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="checkData">{{ lc('common.submit') }}</el-button>
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
                    message.error(window.yunAdminT(lc('admin_tool_00209')));
                    return false;
                }
                if (that.clearTime == 0){
                    message.error(window.yunAdminT(lc('admin_tool_00210')));
                    return false;
                }

                params.clearTime = that.clearTime;

                setTimeout(function () {

                    that.clearTable.forEach(function (item) {

                        console.log(item + window.yunAdminT(lc('admin_tool_00211')));

                        params.clearTable = item;
                        that.clearData(params);

                        console.log(item + window.yunAdminT(lc('admin_tool_00212')));
                    })
                    message.success(window.yunAdminT(lc('admin_tool_00212')), function () {

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