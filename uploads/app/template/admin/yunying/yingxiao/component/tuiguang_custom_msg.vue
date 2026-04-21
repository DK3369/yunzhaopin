<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="500">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">选择用户</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="utype" @change="utypeChange">
                                    <el-radio label="1">个人用户</el-radio>
                                    <el-radio label="2">企业用户</el-radio>
                                    <el-radio label="5">自定义用户</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>注：全部用户发送，时间较长，建议分批发送</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="utype == 5">
                        <td>
                            <div class="TableTite">手机号</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="userarr" placeholder="请输入手机号"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>多个手机号请用,(半角)隔开</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">短信内容</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="4" placeholder="请输入短信内容" v-model="content">
                                </el-input>
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
    props: {},
    data: function () {
        return {
            utype: '',
            userarr: '',
            content: '',

            sendLoading: null,
        }
    },

    mounted() {

    },
    methods: {
        init() {
        },

        utypeChange(val) {
            if (val != 5) {
                this.userarr = '';
            }
        },

        resetData() {
            this.utype = '';
            this.userarr = '';
            this.content = '';
            this.sendLoading = null;
        },

        send() {
            let that = this,
                utype = that.utype,
                userarr = that.userarr,
                content = that.content;

            if (!utype) {
                message.error('请选择发送信息的用户');
                return false;
            }
            if (utype == "5") {
                if (userarr == "") {
                    message.error('请输入手机号');
                    return false;
                }
            }
            if (content == '') {
                message.error('请输入短信内容');
                return false;
            }

            that.sendDivMsg({
                utype: utype,
                userarr: userarr,
                content: content
            }, 1, "正在发送，请稍候。。。", 3);
        },

        sendDivMsg(params, page, msg, status) {
            let that = this;
            if (status == "3") {
                if (!this.sendLoading) {
                    this.sendLoading = this.$loading({
                        lock: true,
                        text: msg,
                        spinner: 'el-icon-loading',
                        background: 'rgba(0, 0, 0, 0.6)'
                    })
                }

                params.page = page;
                httpPost('m=yunying&c=yingxiao_tuiguang&a=msgsave', params, {hideloading: true}).then(function (response) {
                    let res = response.data;

                    if (res.error == 3) {
                        page++;
                        that.sendDivMsg(params, page, res.msg, res.error);
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