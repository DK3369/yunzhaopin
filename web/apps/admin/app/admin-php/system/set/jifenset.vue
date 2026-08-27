<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_system_00458')" name="first">
                    <integset  :list="data" @get-list="getList" v-if="activeName == 'first'"></integset>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00455')" name="second">
                    <gerenjifen :list="data" @get-list="getList" v-if="activeName == 'second'"></gerenjifen>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_system_00456')" name="third">
                    <comjifen :list="data" @get-list="getList"
                              v-if="activeName == 'third'"></comjifen>
                </el-tab-pane>
                
                <el-tab-pane :label="lc('admin_system_00457')" name="sixth">
                    <jifenyouhui v-if="activeName == 'sixth'"></jifenyouhui>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Integset from './component/integset.vue'
import Gerenjifen from './component/gerenjifen.vue'
import Comjifen from './component/comjifen.vue'
import Peixunjifen from './component/peixunjifen.vue'
import Jifenyouhui from './component/jifenyouhui.vue'

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
                    data:{},
                    uri: "m=system&c=",
                    integral_ltcert:'',
                    integral_px_banner:'',
                }
            },
            created() {
                this.getList();


            },
            components: {
                'integset': Integset,
                'gerenjifen': Gerenjifen,
                'comjifen': Comjifen,
                'peixunjifen': Peixunjifen,
                'jifenyouhui': Jifenyouhui,
            },
            methods: {
                handleClick(tab, event) {
                    console.log(tab, event);
                },
                getList:function (){
                    let _this = this;
                    let url = _this.uri + 'set_integral&a=index';
                    httpPost(url, {}).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.data = res.data;
                            _this.integral_ltcert = res.data.integral_ltcert
                            _this.integral_px_banner = res.data.integral_px_banner
                        }
                    })
                }
            }
        }
</script>
