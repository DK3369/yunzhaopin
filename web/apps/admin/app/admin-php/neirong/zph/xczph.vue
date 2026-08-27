<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('wap_00558')" name="zph">
                    <xczphlist ref="zph"></xczphlist>
                </el-tab-pane>
                <el-tab-pane :label="lc('wap_00559')" name="com">
                    <xczphcom ref="com" :shstatus="status"></xczphcom>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_00322')" name="cd">
                    <xczphcd ref="cd"></xczphcd>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Xczphlist from './component/xczphlist.vue'
import Xczphcd from './component/xczphcd.vue'
import Xczphcom from './component/xczphcom.vue'

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
                activeName: 'zph',
                status: ''
            }
        },
        components: {
            'xczphlist': Xczphlist,
            'xczphcd': Xczphcd,
            'xczphcom': Xczphcom,
        },
        created: function() {
            var that = this
            let query = window.parent.homeapp.$route.query;


            if (query.status) {
                that.status = query.status;
            }
            if (query.tabs) {
                that.activeName = query.tabs;
            }
            setTimeout(function () {
                that.$nextTick(function () {
                    if (that.activeName == 'zph') {
                        that.$refs[that.activeName].getGroup()
                    }
                    that.$refs[that.activeName].getList()
                })
            }, 500)
        },
        methods: {
            handleClick(tab) {
                this.$refs[tab.name].getList()
            }
        }
    }
</script>
