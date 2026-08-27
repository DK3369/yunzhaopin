<template>
<div id="toolDataBoardApp" class="moduleDome">
    <div class="setDomeAll setDomeInte">
        <el-tabs v-model="activeName" @tab-click="handleClick">
            <el-tab-pane :label="lc('admin_tool_00281')" name="first">
                <comp_stat></comp_stat>
            </el-tab-pane>
            <el-tab-pane :label="lc('admin_tool_00280')" name="second" :lazy="true">
                <class_stat></class_stat>
            </el-tab-pane>
            <el-tab-pane v-if="show" :label="lc('admin_tool_00279')" name="three" :lazy="true">
                <fenxiabiao></fenxiabiao>
            </el-tab-pane>
        </el-tabs>
    </div>
</div>
</template>

<script>
import CompStat from './component/compStat.vue'
import ClassStat from './component/classStat.vue'
import Fenxiabiao from './component/fenxiabiao.vue'

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
                show :false
            }
        },
        components: {
            'comp_stat': CompStat,
            'class_stat': ClassStat,
            'fenxiabiao': Fenxiabiao,
        },
        created(){
            this.auth();


        },
        methods: {
            auth:function () {
                var self = this;
                var params ={
                    navi_id:1067
                }
                httpPost('m=tool&c=dataBoard&a=getAuth', params).then(function(response) {
                    let res = response.data;
                    if (res.error == 0) {
                        self.show= res.data.status;
                    }
                }).catch(function(error) {
                    console.log(error);
                })
            },
            handleClick(tab, event) {
            }
        }
    }
</script>
