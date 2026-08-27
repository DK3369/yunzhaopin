<template>
<div id="dataCollectionApp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_tool_00326')" name="first">
                <collection_set v-bind:locoy_config="locoyConfig"></collection_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00327')" name="second">
                <news_set v-bind:locoy_config="locoyConfig"></news_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00325')" name="third">
                <job_set v-bind:locoy_config="locoyConfig" v-bind:job_set="jobSet"></job_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00323')" name="fourth">
                <com_set v-bind:locoy_config="locoyConfig" v-bind:com_set="comSet"></com_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00354')" name="fifth">
                <user_set v-bind:locoy_config="locoyConfig" v-bind:user_set="userSet"></user_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00324')" name="sixth">
                <resume_set v-bind:locoy_config="locoyConfig" v-bind:resume_set="resumeSet"></resume_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('member_user_00059')" name="seventh">
                <account_set v-bind:locoy_config="locoyConfig" v-bind:account_set="accountSet"></account_set>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00328')" name="eighth">
                <part_set v-bind:locoy_config="locoyConfig" v-bind:part_set="partSet"></part_set>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import CollectionSet from './component/collectionSet.vue'
import NewsSet from './component/newsSet.vue'
import JobSet from './component/jobSet.vue'
import ComSet from './component/comSet.vue'
import UserSet from './component/userSet.vue'
import ResumeSet from './component/resumeSet.vue'
import AccountSet from './component/accountSet.vue'
import PartSet from './component/partSet.vue'

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
                locoyConfig: {},
                jobSet: 0,
                comSet: 0,
                userSet: 0,
                resumeSet: 0,
                accountSet: 0,
                partSet: 0
            }
        },
        components: {
            'collection_set': CollectionSet,
            'news_set': NewsSet,
            'job_set': JobSet,
            'com_set': ComSet,
            'user_set': UserSet,
            'resume_set': ResumeSet,
            'account_set': AccountSet,
            'part_set': PartSet,
        },
        created(){
            this.getCollectionConfig();


        },
        methods: {
            async getCollectionConfig() {
                let res = await httpPost('m=tool&c=dataCollection');
                if (res.data.error == 0) {

                    this.locoyConfig = res.data.data;
                }
            },
            handleClick(tab, event) {
                let that = this;

                if (tab._props.name == 'third') {

                    that.jobSet++;
                } else if (tab._props.name == 'fourth') {

                    that.comSet++;
                } else if (tab._props.name == 'fifth') {

                    that.userSet++;
                } else if (tab._props.name == 'sixth') {

                    that.resumeSet++;
                } else if (tab._props.name == 'seventh') {

                    that.accountSet++;
                } else if (tab._props.name == 'eighth') {

                    that.partSet++;
                }
            }
        }
    }
</script>
