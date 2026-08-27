<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_00176')" name="first" :lazy="true">
                    <userslog :typelist="typeArr" :time="time"></userslog>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00177')" name="second" :lazy="true">
                    <comlog :typelist="typeArr" :time="time"></comlog>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Userslog from './component/userslog.vue'
import Comlog from './component/comlog.vue'
import Gqlog from './component/gqlog.vue'

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
                activeName: 'first',
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
                uid: ''
            }
        },
        components: {
            'userslog': Userslog,
            'comlog': Comlog,
            'gqlog': Gqlog,
        },
        mounted() {
            let params = window.parent.homeapp.$route.params;


            if (params.tab == 'five') {
                this.uid = params.uid;
                this.activeName = params.tab;
            }
        },
        created: function() {
        },
        methods: {
            handleClick(tab, event) {
                console.log(tab, event);
            }
        }
    }
</script>
