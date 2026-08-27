<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte" style="height: initial;">
			<resumeall ref="all" :status="status.all"></resumeall>
        </div>
    </div>
</template>

<script>
import Resumeall from './component/resumeall.vue'

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
                    status: '',
                }
            },
            components: {
                'resumeall': Resumeall,
            },
            created: function () {
                var that = this
                let query = window.parent.homeapp.$route.query;


                if (query.status) {
                    that.status = query.status;
                }
				
            },
            methods: {
                
            }
        }
</script>
