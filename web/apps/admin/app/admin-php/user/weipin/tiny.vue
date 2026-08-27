<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('wap_js_00066')" name="tiny">
                    <tiny ref="tiny" :status="status"></tiny>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_weipin_00071')" name="tinyset" lazy>
                    <tinyset ref="tinyset"></tinyset>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Tiny from './component/tiny.vue'
import TinySet from './component/tiny_set.vue'

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
                activeName: 'tiny',
                status: '',
            }
        },
        components: {
            'tiny': Tiny,
            'tinyset': TinySet,
        },
        created: function () {
            var that = this
            let query = window.parent.homeapp.$route.query;


            if (query.status) {
                that.status = query.status;
            }
        },
        methods: {
            handleClick(tab, event) {
                // this.$refs[tab.name].init();
            }
        }
    }
</script>
