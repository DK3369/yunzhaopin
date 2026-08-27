<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_company_00011')" name="first" :lazy="true">
                    <company_job_refresh_log :type="1"></company_job_refresh_log>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00012')" name="second" :lazy="true">
                    <company_job_refresh_log :type="2"></company_job_refresh_log>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00010')" name="third" :lazy="true">
                    <company_job_refresh_log :type="3"></company_job_refresh_log>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import CompanyJobRefreshLog from './component/company_job_refresh_log.vue'

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
                'company_job_refresh_log': CompanyJobRefreshLog,
            },
            methods: {
                handleClick(tab, event) {
                    console.log(tab, event);


                }
            }
        }
</script>
