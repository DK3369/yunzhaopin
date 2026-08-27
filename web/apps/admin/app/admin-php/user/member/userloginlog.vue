<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_00181')" name="first" :lazy="true">
                    <grlogin :typelist="typeArr"></grlogin>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00182')" name="second" :lazy="true">
                    <qylogin :typelist="typeArr"></qylogin>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Grlogin from './component/grlogin.vue'
import Qylogin from './component/qylogin.vue'

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
                activeName: 'first',
                typeArr: [{
                    value: '1',
                    label: lc('admin_user_00140')
                }, {
                    value: '2',
                    label: lc('wap_user_00102')
                }, {
                    value: '3',
                    label: lc('admin_user_00130')
                }],
            }
        },
        components: {
            'grlogin': Grlogin,
            'qylogin': Qylogin,
        },
        methods: {
            handleClick(tab, event) {
                console.log(tab, event);


            }
        }
    }
</script>
