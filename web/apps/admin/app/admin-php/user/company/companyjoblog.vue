<template>
<div id="moduapp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_user_company_00352')" name="first" :lazy="true">
                <comlog_index ref="first" v-bind:apply_tab="applyTab" :searchjobid="searchJobId"></comlog_index>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00008')" name="useridmsg" :lazy="true">
                <comlog_useridmsg ref="useridmsg"></comlog_useridmsg>
            </el-tab-pane>
            <el-tab-pane :label="lc('wap_user_00268')" name="lookjob" :lazy="true">
                <comlog_lookjob ref="lookjob"></comlog_lookjob>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00007')" name="favjob" :lazy="true">
                <comlog_favjob ref="favjob"></comlog_favjob>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00009')" name="jobtellog" :lazy="true">
                <comlog_jobtellog ref="jobtellog"></comlog_jobtellog>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00006')" name="partapply" :lazy="true">
                <comlog_partapply ref="partapply"></comlog_partapply>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import ComlogIndex from './component/comlog_index.vue'
import ComlogUseridmsg from './component/comlog_useridmsg.vue'
import ComlogLookjob from './component/comlog_lookjob.vue'
import ComlogFavjob from './component/comlog_favjob.vue'
import ComlogJobtellog from './component/comlog_jobtellog.vue'
import ComlogPartapply from './component/comlog_partapply.vue'

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
                activeName: '',
                applyTab: 1,
                searchJobId: ''
            }
        },
        components: {
            'comlog_index': ComlogIndex,
            'comlog_useridmsg': ComlogUseridmsg,
            'comlog_lookjob': ComlogLookjob,
            'comlog_favjob': ComlogFavjob,
            'comlog_jobtellog': ComlogJobtellog,
            'comlog_partapply': ComlogPartapply,
        },
        mounted(){
			var that = this;


            let params = window.parent.homeapp.$route.params;
			let query = window.parent.homeapp.$route.query;
            if (params.tab == 'apply'){

                this.userTab = 0;
                this.activeName = 'first';
                if (params.job_id){
                    this.searchJobId = params.job_id;
                }
                if (params.com_id){
                    this.searchComId = params.com_id;
                }
            }else if(typeof params.tab!='undefined' && params.tab){
				this.activeName = params.tab;
			}else if(typeof query.tab!='undefined' && query.tab){
				this.activeName = query.tab;
			}else{
                this.activeName = 'first';
            }
			
        },
        methods: {
            handleClick(tab, event) {
                if (tab._props.name == 'first') {
                    this.applyTab++;
                }
            }
        }
    }
</script>
