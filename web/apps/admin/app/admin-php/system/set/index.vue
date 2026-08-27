<template>
<div id="setapp" class="moduleElenAl">
        <div class="setDomeAll">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_system_00452')" name="first">
                    <jibenset :list="list" @get-list="getList" v-if="activeName == 'first'"></jibenset>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00453')" name="second">
                    <anquanset :list="list" @get-list="getList" v-if="activeName == 'second'"></anquanset>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00450')" name="third">
                    <yanzhenm :list="list" @get-list="getList" v-if="activeName == 'third'"></yanzhenm>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00449')" name="fourth">
                    <logopeiz :list="list" @get-list="getList" v-if="activeName == 'fourth'"></logopeiz>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_user_00153')" name="fifth">
                    <ditu :list="list" @get-list="getList" v-if="activeName == 'fifth'"></ditu>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00454')" name="sixth">
                    <huancun :list="list" @get-list="getList" v-if="activeName == 'sixth'"></huancun>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00451')" name="seventh">
                    <uploadset :list="list" @get-list="getList" v-if="activeName == 'seventh'"></uploadset>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Jibenset from './component/jibenset.vue'
import Uploadset from './component/uploadset.vue'
import Anquan from './component/anquan.vue'
import Yanzhenm from './component/yanzhenm.vue'
import Logopeiz from './component/logopeiz.vue'
import Ditu from './component/ditu.vue'
import Huancun from './component/huancun.vue'

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
                    list: {},
                    uri: "m=system&c=",
                }
            },
            components: {
                'jibenset': Jibenset,
                'uploadset': Uploadset,
                'anquanset': Anquan,
                'yanzhenm': Yanzhenm,
                'logopeiz': Logopeiz,
                'ditu': Ditu,
                'huancun': Huancun,
            },
            created() {
                this.getList();


            },
            methods: {
                handleClick(tab, event) {
                    //console.log(tab, event);
                },
                getList: function () {
                    let _this = this;
                    let url = _this.uri + 'set_config&a=index';
                    httpPost(url, {}).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.list = res.data;
							var systemTab = localStorage.getItem('systemTab');
							if(systemTab){
								_this.activeName = systemTab;
								localStorage.removeItem('systemTab');
							}
                        }
                    })
                }
            }
        }
</script>
