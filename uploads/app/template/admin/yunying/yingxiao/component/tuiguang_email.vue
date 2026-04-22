<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="250">名称</th>
                    <th>状态</th>
                    <th width="300">操作</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">周年提醒（所有用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{anniversary_e}} 位 <span v-if="anniversary.ctime">，上次发送时间：{{anniversary.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="anniversary.disabled==1 || anniversary_e == 0" plain size="mini" @click="sendemail('2','2','1','','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="anniversary.disabled==1 || anniversary_e == 0" plain size="mini" @click="outemail('anniversary')">导出邮箱</el-button>
                            <el-button :disabled="anniversary.disabled==1 || anniversary_e == 0" plain size="mini" @click="finishemail('anniversary')" >已发送</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">今日会员到期提醒（企业用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{todaydue_e}} 位 <span v-if="todaydue.ctime">，上次发送时间：{{todaydue.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="todaydue.disabled==1 || todaydue_e == 0" plain size="mini" @click="sendemail('1','3','1','1','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="todaydue.disabled==1 || todaydue_e == 0" plain size="mini" @click="outemail('todaydue')">导出邮箱</el-button>
                            <el-button :disabled="todaydue.disabled==1 || todaydue_e == 0" plain size="mini" @click="finishemail('todaydue')">已发送</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">7日会员到期提醒（企业用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{sevendue_e}} 位 <span v-if="sevendue.ctime">，上次发送时间：{{sevendue.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="sevendue.disabled==1 || sevendue_e == 0" plain size="mini" @click="sendemail('1','3','1','7','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="sevendue.disabled==1 || sevendue_e == 0" plain size="mini" @click="outemail('sevendue')">导出邮箱</el-button>
                            <el-button :disabled="sevendue.disabled==1 || sevendue_e == 0" plain size="mini" @click="finishemail('sevendue')">已发送</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">注册后7日内未发布简历（个人用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{useradd_e}} 位 <span v-if="useradd.ctime">，上次发送时间：{{useradd.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="useradd.disabled==1 || useradd_e == 0" plain size="mini" @click="sendemail('1','4','1','7','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="useradd.disabled==1 || useradd_e == 0" plain size="mini" @click="outemail('useradd')">导出邮箱</el-button>
                            <el-button :disabled="useradd.disabled==1 || useradd_e == 0" plain size="mini" @click="finishemail('useradd')">已发送</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">7日未刷新简历（个人用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{userup_e}} 位 <span v-if="userup.ctime">，上次发送时间：{{userup.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="userup.disabled==1 || userup_e == 0" plain size="mini" @click="sendemail('1','5','1','7','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="userup.disabled==1 || userup_e == 0" plain size="mini" @click="outemail('userup')">导出邮箱</el-button>
                            <el-button :disabled="userup.disabled==1 || userup_e == 0" plain size="mini" @click="finishemail('userup')">已发送</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">注册后7日内未发布职位（企业用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{addjob_e}} 位 <span v-if="addjob.ctime">，上次发送时间：{{addjob.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="addjob.disabled==1 || addjob_e == 0" plain size="mini" @click="sendemail('2','6','1','7','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="addjob.disabled==1 || addjob_e == 0" plain size="mini" @click="outemail('addjob')">导出邮箱</el-button>
                            <el-button :disabled="addjob.disabled==1 || addjob_e == 0" plain size="mini" @click="finishemail('addjob')">已发送</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">7日未刷新职位（企业用户）</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>符合用户数：{{upjob_e}} 位 <span v-if="upjob.ctime">，上次发送时间：{{upjob.ctime_n}}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="upjob.disabled==1 || upjob_e == 0" plain size="mini" @click="sendemail('2','7','1','7','3','0','正在发送，请稍候。。。','0','0')">发送邮件</el-button>
                            <el-button :disabled="upjob.disabled==1 || upjob_e == 0" plain size="mini" @click="outemail('upjob')">导出邮箱</el-button>
                            <el-button :disabled="upjob.disabled==1 || upjob_e == 0" plain size="mini" @click="finishemail('upjob')">已发送</el-button>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {},
        data: function () {
            return {
                addjob: {},
                anniversary: {},
                sevendue: {},
                todaydue: {},
                upjob: {},
                useradd: {},
                userup: {},

                addjob_e: 0,
                anniversary_e: 0,
                sevendue_e: 0,
                todaydue_e: 0,
                upjob_e: 0,
                useradd_e: 0,
                userup_e: 0,

                sendLoading: null,
            }
        },
        created: function () {
            this.init();
        },
        methods: {
            init() {
                this.getData();
                this.getNums();
            },
            getData() {
                let that = this;
                httpPost('m=yunying&c=yingxiao_tuiguang', {}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    for (let key in data) {
                        that.$set(that.$data, key, data[key] ? data[key] : {}); // 批量赋值
                    }
                })
            },
            getNums() {
                let that = this;
                httpPost('m=yunying&c=yingxiao_tuiguang&a=getBirthday', {type: 'email'}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    for (let key in data) {
                        that.$set(that.$data, key, data[key] ? data[key] : 0); // 批量赋值
                    }
                })
            },
            sendemail(type, emailtype, emailtpl, dayslimit, status, value, msg, sendok, sendno) {
                let that = this;
                if (status == "3") {
                    var sort = 1;
                    var pagelimit = 20;
                    if (!this.sendLoading) {
                        this.sendLoading = this.$loading({
                            lock: true,
                            text: msg,
                            spinner: 'el-icon-loading',
                            background: 'rgba(0, 0, 0, 0.6)'
                        })
                    }

                    httpPost('m=yunying&c=yingxiao_tuiguang&a=sendPromotion', {
                        action: "sendEmailMsg",
                        type: type,
                        emailtype: emailtype,
                        emailtpl: emailtpl,
                        dayslimit: dayslimit,
                        sort: sort,
                        value: value,
                        sendok: sendok,
                        sendno: sendno,
                        pagelimit: pagelimit
                    }, {hideloading: true}).then(function (response) {
                        let res = response.data,
                            data = res.data;

                        if (res.error == 3) {
                            that.sendemail(type, emailtype, emailtpl, dayslimit, res.error, data.value, res.msg, data.sendok, data.sendno);
                        } else if (res.error > 0) {
                            that.sendLoading.close();
                            message.error(res.msg, function () {
                                that.sendLoading = null;
                            });
                        } else {
                            that.sendLoading.close();
                            message.confirm(res.msg, function () {
                                that.sendLoading = null;
                                that.getData();
                            }, '', '', '', false);
                        }
                    })
                } else {
                    that.sendLoading.close();
                    message.confirm(msg, function () {
                        that.sendLoading = null;
                        that.getData();
                    }, '', '', '', false);
                }
            },
            outemail(outtype) {
                let that = this;
                message.confirm('确定导出相关邮箱？', function() {
                    httpPost('m=yunying&c=yingxiao_tuiguang&a=xls', {outtype: outtype, xls_type: 'email'}).then(function (response) {
                        let res = response.data;

                        if (res.error > 0) {
                            message.error(res.msg);
                        } else {
                            utilFile.downloadFileByByte(res.data.file, res.data.file_name);
                        }
                    })
                })
            },
            finishemail(type){
                let that = this;
                httpPost('m=yunying&c=yingxiao_tuiguang&a=finish', {type: type, xls_type: 'email'}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    if (res.error > 0) {
                        message.error('操作失败请重试');
                    } else {
                        message.success('操作成功', function () {
                            that.getData();
                        });
                    }
                })
            },
        },

    };
</script>
<style scoped></style>