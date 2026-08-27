<template>
<div id="moduapp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
			<el-tab-pane :label="lc('admin_user_00358')" name="first" :lazy="true">
                <userlog_down ref="first"></userlog_down>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00360')" name="second" :lazy="true">
                <userlog_lookresume ref="second"></userlog_lookresume>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00359')" name="third" :lazy="true">
                <userlog_talentpool ref="third"></userlog_talentpool>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00361')" name="fourth" :lazy="true">
                <userlog_sxlog ref="fourth"></userlog_sxlog>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_00362')" name="fifth" :lazy="true">
                <userlog_trust ref="fifth"></userlog_trust>
            </el-tab-pane>
			<el-tab-pane :label="lc('admin_user_00357')" name="sixth" :lazy="true">
                <userlog_freedown ref="sixth"></userlog_freedown>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import UserlogSxlog from './component/userlog_sxlog.vue'
import UserlogTrust from './component/userlog_trust.vue'
import UserlogLookresume from './component/userlog_lookresume.vue'
import UserlogDown from './component/userlog_down.vue'
import UserlogTalentpool from './component/userlog_talentpool.vue'
import UserlogFreedown from './component/userlog_freedown.vue'

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
            'userlog_sxlog': UserlogSxlog,
            'userlog_trust': UserlogTrust,
            'userlog_lookresume': UserlogLookresume,
            'userlog_down': UserlogDown,
            'userlog_talentpool': UserlogTalentpool,
			'userlog_freedown': UserlogFreedown,
        },
		created(){
			var that = this;


		    let params = window.parent.homeapp.$route.params;
			let query = window.parent.homeapp.$route.query;
		    
			if (!$.isEmptyObject(query.params)) {
				params = {...params,...query.params};
			}
			
			if (!$.isEmptyObject(params)) {
				if(typeof params.activeName!='undefined' && params.activeName!=''){
					that.activeName = params.activeName;
				}
			}
		},
        methods: {
            handleClick(tab, event) {
                //console.log(tab, event);
            }
        }
    }
</script>
