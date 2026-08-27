<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName">
                <el-tab-pane :label="lc('admin_tool_00379')" name="first">
                    <emailfwq></emailfwq>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00374')" name="second">
                    <emailmb></emailmb>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Emailfwq from './component/emailfwq.vue'
import Emailmb from './component/emailmb.vue'

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
                    activeName: 'first'
                }
            },
            components: {
                'emailfwq': Emailfwq,
                'emailmb': Emailmb,
            },
            methods: {

            }
        }
</script>
