<template>
<div id="gsdConfigApp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_tool_00351')" name="first">
                <ip_address v-bind:gsd_config="gsdConfig" v-bind:ip_num="ipNum"></ip_address>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00352')" name="second">
                <phone_address v-bind:gsd_config="gsdConfig" v-bind:phone_num="phoneNum"></phone_address>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import IpAddress from './component/ipAddress.vue'
import PhoneAddress from './component/phoneAddress.vue'

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
        data: function () {
            return {
                activeName: 'first',
                gsdConfig: {},

                ipNum: 0,
                phoneNum: 0
            }
        },
        components: {
            'ip_address': IpAddress,
            'phone_address': PhoneAddress,
        },
        created(){
            this.getGsdConfig();


        },
        methods: {
            async getGsdConfig() {
                let res = await httpPost('m=tool&c=gsdConfig',{},{hideloading: true});
                if (res.data.error == 0) {

                    this.gsdConfig = res.data.data;
                    this.gsdConfig.sy_ip = this.gsdConfig.sy_ip == '1' ? 1 : 2;
                    this.gsdConfig.sy_mobile = this.gsdConfig.sy_mobile == '1' ? 1 : 2;
                }
            },
            handleClick(tab, event) {
                let that = this;

                if (tab._props.name == 'first') {

                    that.ipNum++;
                } else if (tab._props.name == 'second') {

                    that.phoneNum++;
                }
            }
        }
    }
</script>
