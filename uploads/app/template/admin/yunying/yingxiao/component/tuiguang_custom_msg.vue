<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="500">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_01106'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="utype" @change="utypeChange">
                                    <el-radio label="1">{yun:}t key='admin_user_00122'{/yun}</el-radio>
                                    <el-radio label="2">{yun:}t key='admin_user_00124'{/yun}</el-radio>
                                    <el-radio label="5">{yun:}t key='admin_system_00206'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01107'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="utype == 5">
                        <td>
                            <div class="TableTite">{yun:}t key='wap_01619'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="userarr" placeholder="{yun:}t key='wap_js_00119'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_01110'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00666'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="4" placeholder="{yun:}t key='admin_01111'{/yun}" v-model="content">
                                </el-input>
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
                    message.error("{yun:}t key='wap_js_00119'{/yun}");
                    return false;
                }
            }
            if (content == '') {
                message.error("{yun:}t key='admin_01111'{/yun}");
                return false;
            }

            that.sendDivMsg({
                utype: utype,
                userarr: userarr,
                content: content
            }, 1, "{yun:}t key='admin_yunying_00170'{/yun}", 3);
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