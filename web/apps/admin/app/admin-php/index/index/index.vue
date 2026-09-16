<template>
<div id="indexapp" class="homeModule">
        <div v-if="islook">
            <template v-if="index_lookstatistc=='2'">
                <hometop :topinfo="topinfo"></hometop>
                <div class="indexSubFlex">
                    <div class="indexSubLets">
                        <!-- <hometop :topinfo="topinfo"></hometop> -->
                        <homecenter></homecenter>
                    </div>
                    <div class="indexSubRigt">
                        <indexright></indexright>
                    </div>
                </div>
                <!-- <hometop :topinfo="topinfo"></hometop>
				<homecenter></homecenter> -->
                <homebottom :sysinfo="sysinfo"></homebottom>
            </template>
            <el-empty v-else :description="lc('admin_index_00081')"></el-empty>
        </div>
    </div>
</template>

<script>
import Hometop from './component/hometop.vue'
import Homecenter from './component/homecenter.vue'
import Homebottom from './component/homebottom.vue'
import Indexright from './component/indexright.vue'

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
                sysinfo: {},
                topinfo: {},
                dashFullPack: {
                    home: null,
                    ajax_statis: null,
                    month_statis: null,
                    ajax_right: null,
                    chart: null,
                },

                index_lookstatistc: '',
                islook: false,
            }
        },
        provide() {
            return {
                dashFullPack: this.dashFullPack,
            }
        },
        components: {
            'hometop': Hometop,
            'homecenter': Homecenter,
            'homebottom': Homebottom,
            'indexright': Indexright,
        },
        created: function() {

            this.getData();


        },
        methods: {
            getData() {
                var that = this;

                httpPost('m=index&c=dashboardFull').then(function(response) {
                    let res = response.data;
                    if (res.error == 0) {
                        const pack = res.data || {};
                        const home = pack.home || {};
                        that.sysinfo = home.sysinfo;
                        that.topinfo = home.topinfo;
                        that.index_lookstatistc = home.index_lookstatistc;
                        that.dashFullPack.home = pack.home;
                        that.dashFullPack.ajax_statis = pack.ajax_statis;
                        that.dashFullPack.month_statis = pack.month_statis;
                        that.dashFullPack.ajax_right = pack.ajax_right;
                        that.dashFullPack.chart = pack.chart;
                    }
                    that.islook = true;
                })
            },
        }
    }
</script>
