<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="460">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_yunying_00200'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="stype" @change="stypeChange">
                                    <el-radio :label="1">{yun:}t key='member_com_00018'{/yun}</el-radio>
                                    <el-radio :label="2">{yun:}t key='admin_yunying_00201'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_yunying_00200'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_yunying_00216'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="com" @change="getcom">
                                    <el-radio :label="1">{yun:}t key='admin_01117'{/yun}</el-radio>
                                    <el-radio :label="2">{yun:}t key='admin_01118'{/yun}</el-radio>
                                    <el-radio :label="3">{yun:}t key='admin_01119'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_yunying_00203'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_yunying_00199'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="sendnum" placeholder="{yun:}t key='admin_yunying_00198'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_yunying_00182'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_yunying_00217'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="resume" @change="resumeChange">
                                    <el-radio :label="1">{yun:}t key='admin_yunying_00215'{/yun}</el-radio>
                                    <el-radio :label="2">{yun:}t key='home.latest_talents'{/yun}</el-radio>
                                    <el-radio :label="3">{yun:}t key='admin_yunying_00213'{/yun}</el-radio>
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
                            <div class="TableTite">{yun:}t key='admin_01120'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="num" placeholder="{yun:}t key='admin_01121'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_yunying_00204'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 1">
                        <td>
                            <div class="TableTite">{yun:}t key='admin_yunying_00172'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="email_title" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_yunying_00172'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 2">
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00666'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="content" type="textarea" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00666'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="send">{yun:}t key='resume_00033'{/yun}</el-button>
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
            com: '',
            sendnum: '',
            resume: '',
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
            httpPost('m=yunying&c=yingxiao_tuiguang&a=resume').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.sy_webname = data.sy_webname;
            })
        },

        resetData() {
            this.stype = 1;
            this.com = '';
            this.sendnum = '';
            this.resume = '';
            this.num = '';
            this.email_title = '';
            this.content = '';
            this.sendLoading = null;
        },

        // 推送方式变更清空部分数据
        stypeChange(val) {
            this.com = '';
            this.sendnum = '';
            this.resume = '';
            this.num = '';
            if (val == 1) {
                this.content = '';
            } else {
                this.email_title = '';
            }
        },

        getcom(val) {
            let that = this;
            httpPost('m=yunying&c=yingxiao_tuiguang&a=getcom', {com: val, msgType: that.stype}).then(function (response) {
                let res = response.data,
                    data = res.data;

                that.sendnum = data;
            })
        },

        resumeChange(val) {
            let sy_webname = this.sy_webname,
                sy_weburl = this.sy_weburl;
            if (this.stype == 1) {
                if (val == 1) {
                    this.email_title = sy_webname + "{yun:}t key='admin_yunying_00211'{/yun}";
                } else if (val == 2) {
                    this.email_title = sy_webname + "{yun:}t key='admin_yunying_00212'{/yun}";
                } else if (val == 3) {
                    this.email_title = sy_webname + "{yun:}t key='admin_yunying_00208'{/yun}";
                }
            } else {
                if (val == 1) {
                    this.content = sy_webname + "{yun:}t key='admin_yunying_00206'{/yun}" + sy_weburl + "！{yun:}t key='wap_00561'{/yun}";
                } else if (val == 2) {
                    this.content = sy_webname + "{yun:}t key='admin_yunying_00207'{/yun}" + sy_weburl + "！{yun:}t key='wap_00561'{/yun}";
                } else if (val == 3) {
                    this.content = sy_webname + "{yun:}t key='admin_yunying_00205'{/yun}" + sy_weburl + "！{yun:}t key='wap_00561'{/yun}";
                }
            }
        },

        send() {
            let that = this,
                stype = that.stype,
                com = that.com,
                sendnum = that.sendnum,
                resume = that.resume,
                num = that.num,
                title = that.email_title,
                content = that.content;

            if (!com) {
                message.error("{yun:}t key='admin_yunying_00214'{/yun}");
                return false;
            }
            if (sendnum < 1) {
                message.error("{yun:}t key='admin_yunying_00191'{/yun}");
                return false;
            } else if (sendnum > 500) {
                message.error("{yun:}t key='admin_yunying_00181'{/yun}");
                return false;
            }
            if (!resume) {
                message.error("{yun:}t key='admin_yunying_00210'{/yun}");
                return false;
            }
            if (stype == 1) {
                if (num === '' || num < 1) {
                    message.error("{yun:}t key='admin_yunying_00209'{/yun}");
                    return false;
                }
                if (title == '') {
                    message.error("{yun:}t key='admin_yunying_00171'{/yun}");
                    return false;
                }
            } else {
                if (content == '') {
                    message.error("{yun:}t key='admin_01111'{/yun}");
                    return false;
                }
            }

            that.sendMsg(stype, com, sendnum, resume, num, title, content, 3, 0, 0, 0, 0, "{yun:}t key='admin_yunying_00170'{/yun}");
        },

        sendMsg(stype, com, sendnum, resume, num, title, content, status, pagelimit, value, sendok, sendno, msg) {
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

                httpPost('m=yunying&c=yingxiao_tuiguang&a=sendresume', {
                    stype: stype,
                    com: com,
                    sendnum: sendnum,
                    resume: resume,
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
                        that.sendMsg(stype, com, sendnum, resume, num, title, content, res.error, pagelimit, data.value, data.sendok, data.sendno, res.msg);
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