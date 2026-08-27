<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_company_00015')" name="first" :lazy="true">
                    <comrating_index ref="first"></comrating_index>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00290')" name="second" :lazy="true">
                    <comrating_server ref="second"></comrating_server>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import ComratingIndex from './component/comrating_index.vue'
import ComratingServer from './component/comrating_server.vue'

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
                activeName: 'first'
            }
        },
        components: {
            'comrating_index': ComratingIndex,
            'comrating_server': ComratingServer,
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
