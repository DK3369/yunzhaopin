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
                <el-tab-pane :label="lc('admin_tool_00689')" name="google">
                    <logingoogle :config="config" @post-set="postset"></logingoogle>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_tool_00690')" name="facebook">
                    <loginfacebook :config="config" @post-set="postset"></loginfacebook>
                </el-tab-pane>
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Loginqq from './component/loginqq.vue'
import Loginsina from './component/loginsina.vue'
import Logingoogle from './component/logingoogle.vue'
import Loginfacebook from './component/loginfacebook.vue'

const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }

function flag1(v) {
    return (v === 1 || v === '1' || v === true) ? '1' : '0'
}

function textOf(v) {
    return v == null ? '' : String(v)
}

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
                'logingoogle': Logingoogle,
                'loginfacebook': Loginfacebook,
            },
            created:function(){
                this.getInfo();


            },
            methods: {
                applyConfig(raw) {
                    const d = raw && typeof raw === 'object' ? raw : {}
                    this.config = {
                        sy_qqlogin: flag1(d.sy_qqlogin),
                        sy_qqappid: textOf(d.sy_qqappid),
                        sy_qqappkey: textOf(d.sy_qqappkey),
                        sy_qqdt: flag1(d.sy_qqdt),
                        sy_sinalogin: flag1(d.sy_sinalogin),
                        sy_sinaappid: textOf(d.sy_sinaappid),
                        sy_sinaappkey: textOf(d.sy_sinaappkey),
                        sy_googlelogin: flag1(d.sy_googlelogin),
                        sy_googleappid: textOf(d.sy_googleappid),
                        sy_googleappkey: textOf(d.sy_googleappkey),
                        sy_facebooklogin: flag1(d.sy_facebooklogin),
                        sy_facebookappid: textOf(d.sy_facebookappid),
                        sy_facebookappkey: textOf(d.sy_facebookappkey),
                    }
                },
                async getInfo() {
                    let that = this;
                    
                    httpPost('m=tool&c=fastlogin&a=index',{}).then((result)=>{
                        
                        var res = result.data;
                        if (res.error == 0) {
                            
                            that.applyConfig(res.data);
                            
                        }
                        
                    }).catch(function(e){
                        console.log(e)
                    })
                },
                async postset(e){
                    let param = {}
                    if(e.type=='qq'){
                        param = {
                            sy_qqlogin    : flag1(e.config.sy_qqlogin),
                            sy_qqappid    : textOf(e.config.sy_qqappid),
                            sy_qqappkey   : textOf(e.config.sy_qqappkey),
                            sy_qqdt       : flag1(e.config.sy_qqdt),
                        };
                    }else if(e.type=='sina'){
                        param = {
                            sy_sinalogin  : flag1(e.config.sy_sinalogin),
                            sy_sinaappid  : textOf(e.config.sy_sinaappid),
                            sy_sinaappkey : textOf(e.config.sy_sinaappkey),
                        };
                    }else if(e.type=='google'){
                        param = {
                            sy_googlelogin : flag1(e.config.sy_googlelogin),
                            sy_googleappid : textOf(e.config.sy_googleappid),
                            sy_googleappkey: textOf(e.config.sy_googleappkey),
                        };
                    }else if(e.type=='facebook'){
                        param = {
                            sy_facebooklogin : flag1(e.config.sy_facebooklogin),
                            sy_facebookappid : textOf(e.config.sy_facebookappid),
                            sy_facebookappkey: textOf(e.config.sy_facebookappkey),
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
