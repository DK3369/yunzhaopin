<template>
<div id="databaseApp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_tool_00277')" name="first" :lazy="true">
                <back_up></back_up>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00275')" name="second" :lazy="true">
                <backd_up :need="need"></backd_up>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00276')" name="three" :lazy="true">
                <db_optimize :optimize="optimize"></db_optimize>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00278')" name="four" :lazy="true">
                <db_clear></db_clear>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import BackUp from './component/backUp.vue'
import BackdUp from './component/backdUp.vue'
import DbOptimize from './component/dbOptimize.vue'
import DbClear from './component/dbClear.vue'

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
                need: 0,
                optimize: 0
            }
        },
        components: {
            'back_up': BackUp,
            'backd_up': BackdUp,
            'db_optimize': DbOptimize,
            'db_clear': DbClear,
        },
        methods: {
            handleClick(tab, event) {
                let that = this
                if (tab._props.name == 'second'){

                    that.need++;


                } else if (tab._props.name == 'three'){

                    that.optimize++;
                }
            }
        }
    }
</script>
