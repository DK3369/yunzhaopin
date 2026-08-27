<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName">
                <el-tab-pane :label="lc('admin_tool_00466')" name="first">
                    <loginqq :config="config" @post-set="postset"></loginqq>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00465')" name="second">
                    <loginsina :config="config" @post-set="postset"></loginsina>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Loginqq from './component/loginqq.vue'
import Loginsina from './component/loginsina.vue'

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
                    config:{}
                }
            },
            components: {
                'loginqq': Loginqq,
                'loginsina': Loginsina,
            },
            created:function(){
                this.getInfo();


            },
            methods: {
                async getInfo() {
                    let that = this;
                    
                    httpPost('m=tool&c=fastlogin&a=index',{}).then((result)=>{
                        
                        var res = result.data;
                        if (res.error == 0) {
                            
                            that.config =res.data;
                            
                        }
                        
                    }).catch(function(e){
                        console.log(e)
                    })
                },
                async postset(e){
                    
                    let that = this;
                    if(e.type=='qq'){
                        var param = {
                            sy_qqlogin    : e.config.sy_qqlogin==1?1:0,
                            sy_qqappid    : e.config.sy_qqappid,
                            sy_qqappkey   : e.config.sy_qqappkey,
                            sy_qqdt       : e.config.sy_qqdt==1?1:0,

                        };
                    }else if(e.type=='sina'){
                        var param = {
                            
                            sy_sinalogin  : e.config.sy_sinalogin==1?1:0,
                            sy_sinaappid  : e.config.sy_sinaappid,
                            sy_sinaappkey : e.config.sy_sinaappkey,
                        };
                    }
                    startLoading();
                    httpPost('m=tool&c=fastlogin&a=save',param).then((result)=>{
                        endLoading();
                        var res = result.data;

                        message.success(res.msg,this.getInfo);

                    }).catch(function(e){
                        console.log(e)
                    })
                },
            }
        }
</script>
