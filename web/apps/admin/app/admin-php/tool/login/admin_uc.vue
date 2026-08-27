<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName">
                <el-tab-pane :label="lc('admin_tool_00441')" name="first">
                    <ucucenter :configdata="config" :ucinfodata="ucinfo" @ucenter-post="ucenterPost"></ucucenter>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00435')" name="second">
                    <ucwind :configdata="config" :ucinfodata="pw_ucinfo" @pw-post="pwPost"></ucwind>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Ucucenter from './component/ucucenter.vue'
import Ucwind from './component/ucwind.vue'

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
        data: function() {
            return {
                activeName: 'first',
                config: {},

                ucinfo: {},
                pw_ucinfo: {},
            }
        },
        components: {
            'ucucenter': Ucucenter,
            'ucwind': Ucwind,
        },
        created: function() {
            this.getInfo();


        },
        methods: {
            async getInfo() {
                let that = this;
                startLoading();
                httpPost('m=tool&c=admin_uc&a=index', {}).then((result) => {
                    endLoading();
                    var res = result.data;
                    if (res.error == 0) {

                        that.config = res.data.config;
                        that.ucinfo = res.data.ucinfo;
                        that.pw_ucinfo = res.data.pw_ucinfo;
                    }

                }).catch(function(e) {
                    console.log(e)
                })
            },
            async ucenterPost(e) {
                var that = this;
                var config_v = e.config;
                var ucinfo_v = e.ucinfo;

                var param = {};

                param.sy_uc_type = config_v.sy_uc_type == 'uc_center' ? 'uc_center' : '';
                param.UC_DBHOST = ucinfo_v.UC_DBHOST;
                param.UC_DBUSER = ucinfo_v.UC_DBUSER;
                param.UC_DBPW = ucinfo_v.UC_DBPW;
                param.UC_DBNAME = ucinfo_v.UC_DBNAME;
                param.UC_DBTABLEPRE_NEW = ucinfo_v.UC_DBTABLEPRE_NEW;
                param.UC_KEY = ucinfo_v.UC_KEY;
                param.UC_API = ucinfo_v.UC_API;
                param.UC_CHARSET = ucinfo_v.UC_CHARSET;
                param.UC_APPID = ucinfo_v.UC_APPID;
                param.UC_APP = ucinfo_v.UC_APP;
                param.UC_EMAIL = ucinfo_v.UC_EMAIL;
                startLoading();
                httpPost('m=tool&c=admin_uc&a=ucsave', param).then((result) => {
                    endLoading();
                    var res = result.data;

                    message.success(res.msg, () => {
                        that.ucinfo = ucinfo_v;
                        that.config.sy_uc_type = param.sy_uc_type
                    });

                }).catch(function(e) {
                    console.log(e)
                })
            },
            async pwPost(e) {
                var that = this;
                var config_v = e.config;
                var ucinfo_v = e.ucinfo;

                var param = {};

                param.sy_pw_type = config_v.sy_pw_type == 'pw_center' ? 'pw_center' : '';
                param.UC_DBHOST = ucinfo_v.UC_DBHOST;
                param.UC_DBUSER = ucinfo_v.UC_DBUSER;
                param.UC_DBPW = ucinfo_v.UC_DBPW;
                param.UC_DBNAME = ucinfo_v.UC_DBNAME;
                param.UC_DBTABLEPRE = ucinfo_v.UC_DBTABLEPRE;
                param.UC_KEY = ucinfo_v.UC_KEY;
                param.UC_API = ucinfo_v.UC_API;
                param.UC_CHARSET = ucinfo_v.UC_CHARSET;
                param.UC_APPID = ucinfo_v.UC_APPID;
                param.UC_APP = ucinfo_v.UC_APP;
                startLoading();
                httpPost('m=tool&c=admin_uc&a=pwsave', param).then((result) => {
                    endLoading();
                    var res = result.data;

                    message.success(window.lc('wap_user_00104'), () => {
                        that.pw_ucinfo = ucinfo_v;
                        that.config.sy_pw_type = param.sy_pw_type
                    });

                }).catch(function(e) {
                    console.log(e)
                })
            }
        }
    }
</script>
