<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_00354')" name="first" :lazy="true">
                    <userset_index></userset_index>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00355')" name="second" :lazy="true">
                    <userset_logo></userset_logo>
                </el-tab-pane>
               <el-tab-pane :label="lc('admin_user_00356')" name="third" :lazy="true">
                   <userset_userspend></userset_userspend>
               </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import UsersetIndex from './component/userset_index.vue'
import UsersetLogo from './component/userset_logo.vue'
import UsersetUserspend from './component/userset_userspend.vue'

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
                'userset_index': UsersetIndex,
                'userset_logo': UsersetLogo,
                'userset_userspend': UsersetUserspend,
            },
            methods: {
                handleClick(tab, event) {
                    console.log(tab, event);


                }
            }
        }
</script>
