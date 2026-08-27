<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{{ lc('member_com_00021') }}</th>
                        <th width="500">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01106') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-radio-group v-model="utype" @change="utypeChange">
                                    <el-radio label="1">{{ lc('admin_user_00122') }}</el-radio>
                                    <el-radio label="2">{{ lc('admin_user_00124') }}</el-radio>
                                    <el-radio label="5">{{ lc('admin_system_00206') }}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01107') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr v-if="utype == 5">
                        <td>
                            <div class="TableTite">{{ lc('wap_01619') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-input v-model="userarr" :placeholder="lc('wap_js_00119')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01110') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00666') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="4" :placeholder="lc('admin_01111')" v-model="content">
                                </el-input>
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
                message.error(lc('admin_vue_00103'));
                return false;
            }
            if (utype == "5") {
                if (userarr == "") {
                    message.error(lc('wap_js_00119'));
                    return false;
                }
            }
            if (content == '') {
                message.error(lc('admin_01111'));
                return false;
            }

            that.sendDivMsg({
                utype: utype,
                userarr: userarr,
                content: content
            }, 1, lc('admin_yunying_00170'), 3);
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