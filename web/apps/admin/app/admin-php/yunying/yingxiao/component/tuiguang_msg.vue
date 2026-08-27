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
                                <span>{{ lc("admin_matching_users_count", [anniversary_m]) }} <span v-if="anniversary.ctime">{{ lc("admin_last_sent_time", [anniversary.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="anniversary.disabled==1 || anniversary_m == 0" plain size="small" @click="sendmsg('2','2','1','0','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="anniversary.disabled==1 || anniversary_m == 0" plain size="small" @click="outmobile('anniversary')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="anniversary.disabled==1 || anniversary_m == 0" plain size="small" @click="finishmobile('anniversary')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01113') }}</div>
                        </td>
                        <td>
                            <div class="TableTexta">
                                <span>{{ lc("admin_matching_users_count", [todaydue_m]) }} <span v-if="todaydue.ctime">{{ lc("admin_last_sent_time", [todaydue.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="todaydue.disabled==1 || todaydue_m == 0" plain size="small" @click="sendmsg('1','3','1','1','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="todaydue.disabled==1 || todaydue_m == 0" plain size="small" @click="outmobile('todaydue')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="todaydue.disabled==1 || todaydue_m == 0" plain size="small" @click="finishmobile('todaydue')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00175') }}</div>
                        </td>
                        <td>
                            <div class="TableTexta">
                                <span>{{ lc("admin_matching_users_count", [sevendue_m]) }} <span v-if="sevendue.ctime">{{ lc("admin_last_sent_time", [sevendue.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="sevendue.disabled==1 || sevendue_m == 0" plain size="small" @click="sendmsg('1','3','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="sevendue.disabled==1 || sevendue_m == 0" plain size="small" @click="outmobile('sevendue')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="sevendue.disabled==1 || sevendue_m == 0" plain size="small" @click="finishmobile('sevendue')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00173') }}</div>
                        </td>
                        <td>
                            <div class="TableTexta">
                                <span>{{ lc("admin_matching_users_count", [useradd_m]) }} <span v-if="useradd.ctime">{{ lc("admin_last_sent_time", [useradd.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="useradd.disabled==1 || useradd_m == 0" plain size="small" @click="sendmsg('1','4','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="useradd.disabled==1 || useradd_m == 0" plain size="small" @click="outmobile('useradd')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="useradd.disabled==1 || useradd_m == 0" plain size="small" @click="finishmobile('useradd')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00176') }}</div>
                        </td>
                        <td>
                            <div class="TableTexta">
                                <span>{{ lc("admin_matching_users_count", [userup_m]) }} <span v-if="userup.ctime">{{ lc("admin_last_sent_time", [userup.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="userup.disabled==1 || userup_m == 0" plain size="small" @click="sendmsg('1','5','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="userup.disabled==1 || userup_m == 0" plain size="small" @click="outmobile('userup')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="userup.disabled==1 || userup_m == 0" plain size="small" @click="finishmobile('userup')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00174') }}</div>
                        </td>
                        <td>
                            <div class="TableTexta">
                                <span>{{ lc("admin_matching_users_count", [addjob_m]) }} <span v-if="addjob.ctime">{{ lc("admin_last_sent_time", [addjob.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="addjob.disabled==1 || addjob_m == 0" plain size="small" @click="sendmsg('2','6','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="addjob.disabled==1 || addjob_m == 0" plain size="small" @click="outmobile('addjob')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="addjob.disabled==1 || addjob_m == 0" plain size="small" @click="finishmobile('addjob')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00177') }}</div>
                        </td>
                        <td>
                            <div class="TableTexta">
                                <span>{{ lc("admin_matching_users_count", [upjob_m]) }} <span v-if="upjob.ctime">{{ lc("admin_last_sent_time", [upjob.ctime_n]) }}</span></span>
                            </div>
                        </td>
                        <td>
                            <div class="TableCaozuo">
                                <el-button :disabled="upjob.disabled==1 || upjob_m == 0" plain size="small" @click="sendmsg('2','7','1','7','3','0',lc('admin_yunying_00170'),'0','0')">{{ lc('admin_user_00166') }}</el-button>
                                <el-button :disabled="upjob.disabled==1 || upjob_m == 0" plain size="small" @click="outmobile('upjob')">{{ lc('admin_01116') }}</el-button>
                                <el-button :disabled="upjob.disabled==1 || upjob_m == 0" plain size="small" @click="finishmobile('upjob')">{{ lc('admin_yunying_00180') }}</el-button>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>
    
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

export default {
    props: {
        
    },
    data: function () {
        return {
            addjob: {},
            anniversary: {},
            sevendue: {},
            todaydue: {},
            upjob: {},
            useradd: {},
            userup: {},

            addjob_m: 0,
            anniversary_m: 0,
            sevendue_m: 0,
            todaydue_m: 0,
            upjob_m: 0,
            useradd_m: 0,
            userup_m: 0,

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
            httpPost('m=yunying&c=yingxiao_tuiguang&a=msgtg', {}).then(function (response) {
                let res = response.data,
                    data = res.data;

                for (let key in data) {
                    that.$set(that.$data, key, data[key] ? data[key] : {}); // 批量赋值
                }
            })
        },
        getNums() {
            let that = this;
            httpPost('m=yunying&c=yingxiao_tuiguang&a=getBirthday', {type: 'moblie'}).then(function (response) {
                let res = response.data,
                    data = res.data;

                for (let key in data) {
                    that.$set(that.$data, key, data[key] ? data[key] : 0); // 批量赋值
                }
            })
        },
        sendmsg(type, emailtype, emailtpl, dayslimit, status, value, msg, sendok, sendno) {
            let that = this;
            if (status == "3") {
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
                    value: value,
                    sendok: sendok,
                    sendno: sendno,
                    pagelimit: pagelimit
                }, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    if (res.error == 3) {
                        that.sendmsg(type, emailtype, emailtpl, dayslimit, res.error, data.value, res.msg, data.sendok, data.sendno)
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
        outmobile(outtype) {
            let that = this;
            message.confirm(lc('admin_yunying_00202'), function() {
                httpPost('m=yunying&c=yingxiao_tuiguang&a=xls', {outtype: outtype, xls_type: 'mobile'}).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        utilFile.downloadFileByByte(res.data.file, res.data.file_name);
                    }
                })
            })
        },
        finishmobile(type){
            let that = this;
            httpPost('m=yunying&c=yingxiao_tuiguang&a=finish', {type: type, xls_type: 'mobile'}).then(function (response) {
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