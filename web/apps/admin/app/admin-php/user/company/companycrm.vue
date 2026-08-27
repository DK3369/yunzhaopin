<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte" style="height: initial;">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_00124')" name="comuser" lazy>
                    <companyuser ref="comuser" :status="status"></companyuser>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00001')" name="commq" :lazy="true">
                    <companymq ref="commq"></companymq>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_company_00002')" name="comexp" :lazy="true">
                    <expire ref="comexp"></expire>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00353')" name="unbind" :lazy="true">
                    <unbinding ref="unbind"></unbinding>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00177')" name="comlog" :lazy="true">
                    <comlog ref="comlog" :typelist="typeArr" :time="time" :scrolltop="true"></comlog>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Companyuser from './component/companyuser.vue'
import Companymq from './component/companymq.vue'
import Expire from './component/expire.vue'
import Unbinding from './component/unbinding.vue'
import Comlog from './component/comlog.vue'

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
                activeName: 'comuser',
                typeArr: [{
                    value: 1,
                    label: lc('admin_user_00140')
                }, {
                    value: 3,
                    label: lc('admin_user_00130')
                }],
                time: [{
                    value: 1,
                    label: lc('common_01940')
                }, {
                    value: 3,
                    label: lc('admin_user_00179')
                }, {
                    value: 7,
                    label: lc('admin_user_00178')
                }, {
                    value: 15,
                    label: lc('admin_user_00180')
                }, {
                    value: 30,
                    label: lc('admin_user_00175')
                }],
                status: '',
            }
        },
        components: {
            'companyuser': Companyuser,
            'companymq': Companymq,
            'expire': Expire,
            'unbinding': Unbinding,
            'comlog': Comlog,
        },
        created: function() {
            var that = this

            let query = window.parent.homeapp.$route.query;


            if (query.status) {
                that.status = query.status;
            }
            if (query.tabs) {
                that.activeName = query.tabs;
            }

        },
        methods: {
            handleClick(tab) {

            }
        }
    }
</script>
