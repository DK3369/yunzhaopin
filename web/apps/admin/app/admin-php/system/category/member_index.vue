<template>
<div id="systemvip">
        <div class="setDomeAll setDomehuiyuan">
            <!--会员分类-->
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_system_00129')" name="userclass" :lazy="true">
                    <member_userclass></member_userclass>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00281')" name="comclass" :lazy="true">
                    <member_comclass></member_comclass>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import MemberUserclass from './component/member_userclass.vue'
import MemberComclass from './component/member_comclass.vue'

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
                    activeName: 'userclass'
                }
            },
            mounted() {
            },
            components: {
                'member_userclass': MemberUserclass,
                'member_comclass': MemberComclass,
            },
            methods: {
                handleClick(tab, event) {
                }
            }
        }
</script>
