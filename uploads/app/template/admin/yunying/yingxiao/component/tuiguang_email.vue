<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="250">{{ lc('member_com_00021') }}</th>
                    <th>{{ lc('member_user_00181') }}</th>
                    <th width="300">{{ lc('member_user_00048') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00178') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [anniversary_e]) }} <span v-if="anniversary.ctime">{{ lc("admin_last_sent_time", [anniversary.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="anniversary.disabled==1 || anniversary_e == 0" plain size="mini" @click="sendemail('2','2','1','','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="anniversary.disabled==1 || anniversary_e == 0" plain size="mini" @click="outemail('anniversary')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="anniversary.disabled==1 || anniversary_e == 0" plain size="mini" @click="finishemail('anniversary')" >{{ lc('admin_yunying_00180') }}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_01113') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [todaydue_e]) }} <span v-if="todaydue.ctime">{{ lc("admin_last_sent_time", [todaydue.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="todaydue.disabled==1 || todaydue_e == 0" plain size="mini" @click="sendemail('1','3','1','1','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="todaydue.disabled==1 || todaydue_e == 0" plain size="mini" @click="outemail('todaydue')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="todaydue.disabled==1 || todaydue_e == 0" plain size="mini" @click="finishemail('todaydue')">{{ lc('admin_yunying_00180') }}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00175') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [sevendue_e]) }} <span v-if="sevendue.ctime">{{ lc("admin_last_sent_time", [sevendue.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="sevendue.disabled==1 || sevendue_e == 0" plain size="mini" @click="sendemail('1','3','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="sevendue.disabled==1 || sevendue_e == 0" plain size="mini" @click="outemail('sevendue')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="sevendue.disabled==1 || sevendue_e == 0" plain size="mini" @click="finishemail('sevendue')">{{ lc('admin_yunying_00180') }}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00173') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [useradd_e]) }} <span v-if="useradd.ctime">{{ lc("admin_last_sent_time", [useradd.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="useradd.disabled==1 || useradd_e == 0" plain size="mini" @click="sendemail('1','4','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="useradd.disabled==1 || useradd_e == 0" plain size="mini" @click="outemail('useradd')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="useradd.disabled==1 || useradd_e == 0" plain size="mini" @click="finishemail('useradd')">{{ lc('admin_yunying_00180') }}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00176') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [userup_e]) }} <span v-if="userup.ctime">{{ lc("admin_last_sent_time", [userup.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="userup.disabled==1 || userup_e == 0" plain size="mini" @click="sendemail('1','5','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="userup.disabled==1 || userup_e == 0" plain size="mini" @click="outemail('userup')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="userup.disabled==1 || userup_e == 0" plain size="mini" @click="finishemail('userup')">{{ lc('admin_yunying_00180') }}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00174') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [addjob_e]) }} <span v-if="addjob.ctime">{{ lc("admin_last_sent_time", [addjob.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="addjob.disabled==1 || addjob_e == 0" plain size="mini" @click="sendemail('2','6','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="addjob.disabled==1 || addjob_e == 0" plain size="mini" @click="outemail('addjob')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="addjob.disabled==1 || addjob_e == 0" plain size="mini" @click="finishemail('addjob')">{{ lc('admin_yunying_00180') }}</el-button>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_yunying_00177') }}</div>
                    </td>
                    <td>
                        <div class="TableTexta">
                            <span>{{ lc("admin_matching_users_count", [upjob_e]) }} <span v-if="upjob.ctime">{{ lc("admin_last_sent_time", [upjob.ctime_n]) }}</span></span>
                        </div>
                    </td>
                    <td>
                        <div class="TableCaozuo">
                            <el-button :disabled="upjob.disabled==1 || upjob_e == 0" plain size="mini" @click="sendemail('2','7','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00167') }}</el-button>
                            <el-button :disabled="upjob.disabled==1 || upjob_e == 0" plain size="mini" @click="outemail('upjob')">{{ lc('admin_01112') }}</el-button>
                            <el-button :disabled="upjob.disabled==1 || upjob_e == 0" plain size="mini" @click="finishemail('upjob')">{{ lc('admin_yunying_00180') }}</el-button>
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
                message.confirm(lc('admin_yunying_00179'), function() {
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
                        message.error(lc('admin_system_00397'));
                    } else {
                        message.success(lc('wap_user_00264'), function () {
                            that.getData();
                        });
                    }
                })
            },
        },

    };
</script>
<style scoped></style>