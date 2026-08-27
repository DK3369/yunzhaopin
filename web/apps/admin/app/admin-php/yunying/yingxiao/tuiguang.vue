<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_01125')" name="email">
                    <email ref="email"></email>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_01126')" name="msg" lazy>
                    <msg ref="msg"></msg>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_01127')" name="resume" lazy>
                    <resume ref="resume"></resume>
                </el-tab-pane>
                <el-tab-pane :label="lc('wap_com_00236')" name="job" lazy>
                    <job ref="job"></job>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_01128')" name="custom_email" lazy>
                    <custom_email ref="custom_email"></custom_email>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_01129')" name="custom_msg" lazy>
                    <custom_msg ref="custom_msg"></custom_msg>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import TuiguangEmail from './component/tuiguang_email.vue'
import TuiguangMsg from './component/tuiguang_msg.vue'
import TuiguangResume from './component/tuiguang_resume.vue'
import TuiguangJob from './component/tuiguang_job.vue'
import TuiguangCustomEmail from './component/tuiguang_custom_email.vue'
import TuiguangCustomMsg from './component/tuiguang_custom_msg.vue'

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
                    activeName: 'email'
                }
            },
            components: {
                'email': TuiguangEmail,
                'msg': TuiguangMsg,
                'resume': TuiguangResume,
                'job': TuiguangJob,
                'custom_email': TuiguangCustomEmail,
                'custom_msg': TuiguangCustomMsg,
            },
            methods: {
                handleClick(tab, event) {
                    // this.$refs[tab.name].init();


                }
            }
        }
</script>
