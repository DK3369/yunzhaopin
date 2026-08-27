<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_tool_00687')" name="first" :lazy="true">
                    <userrecord ref="first"></userrecord>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00688')" name="second" :lazy="true">
                    <userbd ref="second"></userbd>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00686')" name="third" :lazy="true">
                    <usertag ref="third"></usertag>
                </el-tab-pane>
                
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Userrecord from './component/userrecord.vue'
import Userbd from './component/userbd.vue'
import Usertag from './component/usertag.vue'

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
                }
            },
            components: {
                'userrecord': Userrecord,
                'userbd': Userbd,
                'usertag': Usertag, 
            },
            methods: {
                handleClick(tab, event) {
                    this.$nextTick(() => {
                        this.$refs[tab.name].doLayout();


                    });
                }
            }
        }
</script>
