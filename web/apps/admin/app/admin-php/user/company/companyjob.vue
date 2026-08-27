<template>
<div id="moduapp" class="moduleDome">
    <div class="setDomeAll setDomeInte" style="height: initial;">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_user_company_00004')" name="joball" :lazy="true">
                <joball ref="joball" :state="state.joball" :crmindex="fromCrmIndex" :adtime="adtime" :uid="uid" :status="status" :keyword="keyword" :type="type" :scrolltop="true"></joball>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00005')" name="partjob" :lazy="true">
                <partjob ref="partjob" :state="state.partjob"></partjob>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_user_company_00003')" name="refresh" :lazy="true">
                <refresh ref="refresh"></refresh>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import Joball from './component/joball.vue'
import Partjob from './component/partjob.vue'
import Refresh from './component/refresh.vue'

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
                activeName: 'joball',
                uid: '',
                state: {
                    joball: '',
                    partjob: '',
                },
                status: '',
                fromCrmIndex: '',
                adtime: '',
                keyword:'',
                type:'',
            }
        },
        components: {
            'joball': Joball,
            'partjob': Partjob,
            'refresh': Refresh
        },
        created: function () {
            var that = this
            let query = window.parent.homeapp.$route.query;



            if (!$.isEmptyObject(query)) {
                if (query.uid) {
                    that.uid = query.uid;
                }
                if (query.status) {
                    that.status = query.status;
                }
                if (query.fromCrmIndex) {
                    that.fromCrmIndex = query.fromCrmIndex;
                }
                if (query.adtime) {
                    that.adtime = query.adtime;
                }
                if (query.keyword){
                    that.keyword = query.keyword;
                }
                if (query.type){
                    that.type = query.type;
                }
                console.log(query);
                if (query.tabs) {
                    that.activeName = query.tabs;
                }
                if (query.state) {
                    that.state[that.activeName] = query.state;
                }
            }
        },
        methods: {
            handleClick() {

            }
        }
    }
</script>
