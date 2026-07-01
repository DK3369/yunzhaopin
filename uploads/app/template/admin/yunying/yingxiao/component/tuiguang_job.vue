<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{{ lc('member_com_00021') }}</th>
                        <th width="400">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00200') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="stype" @change="stypeChange">
                                    <el-radio :label="1">{{ lc('member_com_00018') }}</el-radio>
                                    <el-radio :label="2">{{ lc('admin_yunying_00201') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_yunying_00200') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01106') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="user" @change="getuser">
                                    <el-radio :label="1">{{ lc('admin_yunying_00188') }}</el-radio>
                                    <el-radio :label="2">{{ lc('admin_yunying_00196') }}</el-radio>
                                </el-radio-group>
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
                            <div class="TableTite">{{ lc('admin_yunying_00199') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="sendnum" :placeholder="lc('admin_yunying_00198')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_yunying_00182') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00300') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="job" @change="getjob">
                                    <el-radio :label="1">{{ lc('home.latest_jobs') }}</el-radio>
                                    <el-radio :label="2">{{ lc('home.recommended_jobs') }}</el-radio>
                                    <el-radio :label="3">{{ lc('member_com_00326') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span></span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 1">
                        <td>
                            <div class="TableTite">{{ lc('admin_01114') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="num" :placeholder="lc('admin_01115')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_yunying_00183') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 1">
                        <td>
                            <div class="TableTite">{{ lc('admin_yunying_00172') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="email_title" :placeholder="lc('wap_user_00076')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_yunying_00172') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 2">
                        <td>
                            <div class="TableTite">{{ lc('admin_00666') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="content" type="textarea" :placeholder="lc('wap_user_00076')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00666') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="send">{{ lc('resume_00033') }}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        list: Object
    },
    data: function () {
        return {
            sy_webname: '',
            sy_weburl: localStorage.getItem("sy_weburl"),

            stype: 1,
            user: '',
            sendnum: '',
            job: '',
            num: '',
            email_title: '',
            content: '',

            sendLoading: null,
        }
    },

    mounted() {

    },
    created: function () {
        this.init();
    },
    methods: {
        init() {
            this.getData();
        },

        getData() {
            let that = this;
            httpPost('m=yunying&c=yingxiao_tuiguang&a=job').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.sy_webname = data.sy_webname;
            })
        },

        resetData() {
            this.stype = 1;
            this.user = '';
            this.sendnum = '';
            this.job = '';
            this.num = '';
            this.email_title = '';
            this.content = '';
            this.sendLoading = null;
        },

        // 推送方式变更清空部分数据
        stypeChange(val) {
            this.user = '';
            this.sendnum = '';
            this.job = '';
            this.num = '';
            if (val == 1) {
                this.content = '';
            } else {
                this.email_title = '';
            }
        },

        getuser(val) {
            let that = this;
            httpPost('m=yunying&c=yingxiao_tuiguang&a=getuser', {user: val, msgType: that.stype}).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.sendnum = data;
            })
        },

        getjob(val) {
            let that = this,
                sy_webname = this.sy_webname,
                sy_weburl = this.sy_weburl,
                stype = this.stype;

            if (stype == '1') {
                if (val == 1) {
                    this.email_title = sy_webname + lc('admin_yunying_00194');
                } else if (val == 2) {
                    this.email_title = sy_webname + lc('admin_yunying_00193');
                } else if (val == 3) {
                    this.email_title = sy_webname + lc('admin_yunying_00195');
                }
            } else {
                if (val == 1) {
                    this.content = sy_webname + lc('admin_yunying_00185') + sy_weburl + " ！{{ lc('wap_00561') }}";
                } else if (val == 2) {
                    this.content = sy_webname + lc('admin_yunying_00184') + sy_weburl + " ！{{ lc('wap_00561') }}";
                } else if (val == 3) {
                    this.content = sy_webname + lc('admin_yunying_00186') + sy_weburl + " ！{{ lc('wap_00561') }}";
                }
            }
            if (val == 2 || val == 3) {
                httpPost('m=yunying&c=yingxiao_tuiguang&a=getjob', {job: val}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    if (data < 1) {
                        if (val == 2) {
                            message.error(lc('admin_yunying_00189'));
                            return false;
                        } else {
                            message.error(lc('admin_yunying_00190'));
                            return false;
                        }
                    }
                })
            }
        },

        send() {
            let that = this,
                stype = that.stype,
                user = that.user,
                sendnum = that.sendnum,
                job = that.job,
                num = that.num,
                title = that.email_title,
                content = that.content;

            if (!user) {
                message.error(lc('admin_yunying_00197'));
                return false;
            }
            if (sendnum < 1) {
                message.error(lc('admin_yunying_00191'));
                return false;
            } else if (sendnum > 500) {
                message.error(lc('admin_yunying_00181'));
                return false;
            }
            if (!job) {
                message.error(lc('admin_yunying_00192'));
                return false;
            }
            if (stype == 1) {
                if (num < 1) {
                    message.error(lc('admin_yunying_00187'));
                    return false;
                }
                if (title == '') {
                    message.error(lc('admin_yunying_00171'));
                    return false;
                }
            } else {
                if (content == '') {
                    message.error(lc('admin_01111'));
                    return false;
                }
            }

            that.sendMsg(stype, user, sendnum, job, num, title, content, 3, 0, 0, 0, 0, lc('admin_yunying_00170'));
        },

        sendMsg(stype, user, sendnum, job, num, title, content, status, pagelimit, value, sendok, sendno, msg) {
            let that = this;
            if (status == "3") {
                if (stype == "1") {
                    var pagelimit = 20;//邮件
                } else {
                    var pagelimit = 50;//短信
                }

                if (!this.sendLoading) {
                    this.sendLoading = this.$loading({
                        lock: true,
                        text: msg,
                        spinner: 'el-icon-loading',
                        background: 'rgba(0, 0, 0, 0.6)'
                    })
                }

                httpPost('m=yunying&c=yingxiao_tuiguang&a=sendjob', {
                    stype: stype,
                    user: user,
                    sendnum: sendnum,
                    job: job,
                    num: num,
                    email_title: title,
                    content: content,
                    value: value,
                    sendok: sendok,
                    sendno: sendno,
                    pagelimit: pagelimit
                }, {hideloading: true}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    if (res.error == 3) {
                        that.sendMsg(stype, user, sendnum, job, num, title, content, res.error, pagelimit, data.value, data.sendok, data.sendno, res.msg)
                    } else if (res.error > 0) {
                        that.sendLoading.close();
                        message.error(res.msg, function () {
                            that.sendLoading = null;
                        });
                    } else {
                        that.sendLoading.close();
                        message.confirm(res.msg, function () {
                            that.resetData();
                        }, '', '', '', false);
                    }
                })
            } else {
                that.sendLoading.close();
                message.confirm(msg, function () {
                    that.resetData();
                }, '', '', '', false);
            }
        },
    },
};
</script>
<style scoped></style>