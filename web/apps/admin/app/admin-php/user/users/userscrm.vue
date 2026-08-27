<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte" style="height: initial;">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_user_00122')" name="usersall">
                    <usersall ref="usersall"></usersall>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00353')" name="unbinding" lazy>
                    <unbinding ref="unbinding"></unbinding>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00176')" name="userlog" lazy>
                    <userlog ref="userlog"></userlog>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Usersall from './component/usersall.vue'
import Unbinding from './component/unbinding.vue'
import Userlog from './component/userlog.vue'

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
                activeName: 'usersall'
            }
        },
        components: {
            'usersall': Usersall,
            'unbinding': Unbinding,
            'userlog': Userlog,
        },
		created: function () {
		    var that = this
			
		},
        methods: {
            handleClick(tab, event) {
                // this.$refs[tab.name].init();


            }
        }
    }
</script>
