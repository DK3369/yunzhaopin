<template>
<div id="moduapp" class="moduleElenAl">
        <div class="setDomeAll">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_system_00349')" name="first" :lazy="true">
                    <mobanone ref="first" ></mobanone>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00135')" name="second" :lazy="true">
                    <mobantwo ref="second" ></mobantwo>
                </el-tab-pane>
                <el-tab-pane :label="lc('member_user_00189')" name="three" :lazy="true">
                    <mobanthree ref="three"></mobanthree>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00577')" name="fourth" :lazy="true">
                    <mobanfour ref="fourth" ></mobanfour>
                </el-tab-pane>

            </el-tabs>
        </div>
    </div>
</template>

<script>
import Mobanone from './component/mobanone.vue'
import Mobantwo from './component/mobantwo.vue'
import Mobanthree from './component/mobanthree.vue'
import Mobanfour from './component/mobanfour.vue'
import Mobanfive from './component/mobanfive.vue'

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
                'mobanone': Mobanone,
                'mobantwo': Mobantwo,
                'mobanthree': Mobanthree,
                'mobanfour': Mobanfour,
                'mobanfive': Mobanfive,
            },
            methods: {
                handleClick(tab, event) {
                    
                }
            }
        }
</script>
