<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_00352')" name="first" :lazy="true">
                    <renzheng_renzheng :status="status.first"></renzheng_renzheng>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00351')" name="second" :lazy="true">
                    <renzheng_logo :status="status.second"></renzheng_logo>
                </el-tab-pane>
                <el-tab-pane :label="lc('wap_00492')" name="third" :lazy="true">
                    <renzheng_show :status="status.third"></renzheng_show>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import RenzhengRenzheng from './component/renzheng_renzheng.vue'
import RenzhengLogo from './component/renzheng_logo.vue'
import RenzhengShow from './component/renzheng_show.vue'

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
                    status: {
                        first: '',
                        second: '',
                        third: '',
                    },
                }
            },
            components: {
                'renzheng_renzheng': RenzhengRenzheng,
                'renzheng_logo': RenzhengLogo,
                'renzheng_show': RenzhengShow,
             },
            created: function () {
                var that = this
                let query = window.parent.homeapp.$route.query;


                if (query.tabs) {
                    that.activeName = query.tabs;
                }
                if (query.status) {
                    that.status[that.activeName] = query.status;
                }
            },
            methods: {
                handleClick(tab, event) {
                    console.log(tab, event);
                }
            }
        }
</script>
