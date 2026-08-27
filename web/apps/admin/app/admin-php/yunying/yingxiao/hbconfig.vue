<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_yunying_00219')" name="set">
                    <hb_set ref="set"></hb_set>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_01122')" name="job" lazy>
                    <hb_job ref="job"></hb_job>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_01123')" name="com" lazy>
                    <hb_com ref="com"></hb_com>
                </el-tab-pane>
                <el-tab-pane :label="lc('wap_01590')" name="invite_reg" lazy>
                    <hb_invite_reg ref="invite_reg"></hb_invite_reg>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_yunying_00218')" name="gongzhao" lazy>
                    <hb_gongzhao ref="gongzhao"></hb_gongzhao>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import HbSet from './component/hb_set.vue'
import HbJob from './component/hb_job.vue'
import HbCom from './component/hb_com.vue'
import HbInviteReg from './component/hb_invite_reg.vue'
import HbGongzhao from './component/hb_gongzhao.vue'

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
                    activeName: 'set'
                }
            },
            components: {
                'hb_set': HbSet,
                'hb_job': HbJob,
                'hb_com': HbCom,
                'hb_invite_reg': HbInviteReg,
                'hb_gongzhao': HbGongzhao,
            },
            methods: {
                handleClick(tab, event) {
                    // this.$refs[tab.name].init();


                }
            }
        }
</script>
