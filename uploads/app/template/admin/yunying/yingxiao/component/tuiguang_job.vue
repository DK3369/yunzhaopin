<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="400">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">推送方式</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="stype" @change="stypeChange">
                                    <el-radio :label="1">邮件</el-radio>
                                    <el-radio :label="2">短信</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>推送方式</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">选择用户</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="user" @change="getuser">
                                    <el-radio :label="1">最近7天刷新简历的个人</el-radio>
                                    <el-radio :label="2">最近3天注册的个人</el-radio>
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
                            <div class="TableTite">发送数量</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="sendnum" placeholder="请输入发送数量"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>注：如发送数量大于500，建议找专业的群发软件</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">选择职位</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="job" @change="getjob">
                                    <el-radio :label="1">最新职位</el-radio>
                                    <el-radio :label="2">推荐职位</el-radio>
                                    <el-radio :label="3">紧急职位</el-radio>
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
                            <div class="TableTite">推送职位数量</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="num" placeholder="请输入推送职位数量"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>注：推送数量超过已有职位数量则按已有数量推广</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 1">
                        <td>
                            <div class="TableTite">邮件主题</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="email_title" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>邮件主题</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="stype == 2">
                        <td>
                            <div class="TableTite">短信内容</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="content" type="textarea" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>短信内容</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="send">发送</el-button>
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
                    this.email_title = sy_webname + '向您推送的最新职位';
                } else if (val == 2) {
                    this.email_title = sy_webname + '向您推送的推荐职位';
                } else if (val == 3) {
                    this.email_title = sy_webname + '向您推送的紧急职位';
                }
            } else {
                if (val == 1) {
                    this.content = sy_webname + '向您推荐了符合您的最新职位，请点击' + sy_weburl + ' ！查看详情';
                } else if (val == 2) {
                    this.content = sy_webname + '向您推荐了符合您的推荐职位，请点击' + sy_weburl + ' ！查看详情';
                } else if (val == 3) {
                    this.content = sy_webname + '向您推荐了符合您的紧急职位，请点击' + sy_weburl + ' ！查看详情';
                }
            }
            if (val == 2 || val == 3) {
                httpPost('m=yunying&c=yingxiao_tuiguang&a=getjob', {job: val}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    if (data < 1) {
                        if (val == 2) {
                            message.error('没有推荐职位，无法推广');
                            return false;
                        } else {
                            message.error('没有紧急职位，无法推广');
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
                message.error('请选择推广的用户');
                return false;
            }
            if (sendnum < 1) {
                message.error('请输入需要发送的数量');
                return false;
            } else if (sendnum > 500) {
                message.error('数量过多，第三方发送服务器将会影响。建议找专业的群发软件');
                return false;
            }
            if (!job) {
                message.error('请选择需要推广的职位');
                return false;
            }
            if (stype == 1) {
                if (num < 1) {
                    message.error('请输入需要推广的职位数量');
                    return false;
                }
                if (title == '') {
                    message.error('请输入邮件主题');
                    return false;
                }
            } else {
                if (content == '') {
                    message.error('请输入短信内容');
                    return false;
                }
            }

            that.sendMsg(stype, user, sendnum, job, num, title, content, 3, 0, 0, 0, 0, "正在发送，请稍候。。。");
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