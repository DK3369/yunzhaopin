<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName">
                <el-tab-pane :label="lc('admin_tool_00678')" name="first" :lazy="true">
                    <wxfbtool></wxfbtool>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00524')" name="second" :lazy="true">
                    <pubtemplist :temptype="0" :secondpage="secondpage" :thirdpage="thirdpage"></pubtemplist>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00525')" name="third" :lazy="true">
                    <pubtemplist :temptype="1" :secondpage="secondpage" :thirdpage="thirdpage"></pubtemplist>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00680')" name="fourth" :lazy="true">
                    <jobtw></jobtw>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00679')" name="fiveth" :lazy="true">
                    <comtw></comtw>
                </el-tab-pane>

            </el-tabs>
        </div>
    </div>
</template>

<script>
import Wxfbtool from './component/wxfbtool.vue'
import Pubtemplist from './component/pubtemplist.vue'
import Jobtw from './component/jobtw.vue'
import Comtw from './component/comtw.vue'

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
                    childPage:0,
                    thirdpage:0,
                    secondpage:0
                }
            },
            components: {
                'wxfbtool': Wxfbtool,
                'pubtemplist': Pubtemplist,
                'jobtw': Jobtw,
                'comtw': Comtw,
            },
            created:function(){
				if(typeof window.parent.homeapp.$route.params.activeName!='undefined'){
                    this.activeName =window.parent.homeapp.$route.params.activeName;


                    this.childPage =window.parent.homeapp.$route.params.page;
                    if (window.parent.homeapp.$route.params.activeName == 'second'){
                        this.secondpage = this.childPage
                        this.thirdpage = 0
                    }
                    if (window.parent.homeapp.$route.params.activeName == 'third'){
                        this.thirdpage = this.childPage
                        this.secondpage = 0
                    }

                }
            },
            methods: {

            }
        }
</script>
