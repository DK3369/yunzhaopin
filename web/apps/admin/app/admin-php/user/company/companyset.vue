<template>
<div id="moduapp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_user_company_00014')" name="first" :lazy="true">
                <comset_index></comset_index>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00355')" name="logo" :lazy="true">
                <comset_logo></comset_logo>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00356')" name="comspend" :lazy="true">
                <comset_comspend></comset_comspend>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00015')" name="rating" :lazy="true">
                <comset_rating></comset_rating>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import ComsetIndex from './component/comset_index.vue'
import ComsetLogo from './component/comset_logo.vue'
import ComsetComspend from './component/comset_comspend.vue'
import ComsetRating from './component/comset_rating.vue'

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
            'comset_index': ComsetIndex,
            'comset_logo': ComsetLogo,
            'comset_comspend': ComsetComspend,
            'comset_rating': ComsetRating
        },
        methods: {
            handleClick(tab, event) {
                console.log(tab, event);


            }
        }
    }
</script>
