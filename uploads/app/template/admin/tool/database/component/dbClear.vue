<template>
    <div class="moduleElHight">
        <div class="tableDome_tip">
            <el-alert title="定期清理数据表碎片可以提升数据库的查询速度。从而提升用户体验度，站长可自行选择老数据清理。" type="success" :closable="false"></el-alert>
        </div>
        <div class=" moduleTable">

            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="100">类目</th>
                    <th>状态</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">清理内容</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-checkbox-group v-model="clearTable">
                                <el-checkbox label="userid_job">职位申请记录</el-checkbox>
                                <el-checkbox label="userid_msg">面试邀请记录</el-checkbox>
                                <el-checkbox label="down_resume">下载简历记录</el-checkbox>
                                <el-checkbox label="talent_pool">收藏简历记录</el-checkbox>
                                <el-checkbox label="look_resume">浏览简历记录</el-checkbox>
                                <el-checkbox label="look_job">浏览职位记录</el-checkbox>
                                <el-checkbox label="email_msg">邮件发送记录</el-checkbox>
                                <el-checkbox label="moblie_msg">短信发送记录</el-checkbox>
                                <el-checkbox label="member_log">会员日志记录</el-checkbox>
                                <el-checkbox label="sysmsg">系统消息</el-checkbox>
                                <el-checkbox label="recycle">回收站</el-checkbox>
                            </el-checkbox-group>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">清理时间</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-radio-group v-model="clearTime">
                                <el-radio label="730">两年前</el-radio>
                                <el-radio label="360">一年前</el-radio>
                                <el-radio label="180">半年前</el-radio>
                                <el-radio label="90">三个月前</el-radio>
                                <el-radio label="60">两个月前</el-radio>
                                <el-radio label="30">一个月前</el-radio>
                            </el-radio-group>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="checkData">提交</el-button>
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
                    message.error('请选择需要清理的数据表！');
                    return false;
                }
                if (that.clearTime == 0){
                    message.error('请选择需要清理时间！');
                    return false;
                }

                params.clearTime = that.clearTime;

                setTimeout(function () {

                    that.clearTable.forEach(function (item) {

                        console.log(item+'数据清理中');

                        params.clearTable = item;
                        that.clearData(params);

                        console.log(item+'数据清除完成');
                    })
                    message.success('数据清除完成', function () {

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