<template>
<div id="jihuaapp" class="moduleElenAl">
        <div class="setDomeAll">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_system_00273')" name="cron">
                    <crontask ref="cron"></crontask>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00426')" name="log">
                    <tasklog ref="log"></tasklog>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Crontask from './component/crontask.vue'
import Tasklog from './component/tasklog.vue'

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
                activeName: 'cron'
            }
        },
        components: {
            'crontask': Crontask,
            'tasklog': Tasklog,
        },
        created: function() {
            var that = this
            let query = getUrlParams();


            if (query && query.topage) {
                if (query.topage == 2) {
                    that.activeName = 'log'
                }
            }
            setTimeout(() => {
                if (that.activeName == 'cron') {
                    this.$refs.cron.getList();
                } else {
                    this.$refs.log.getList();
                }
            }, 500)
        },
        methods: {
            handleClick(tab) {
                if (tab.name == 'cron') {
                    this.$refs.cron.getList();
                } else if (tab.name == 'log') {
                    this.$refs.log.getList();
                }
            }
        }
    }
</script>
