<template>
<div id="moduapp" class="tableDome">
        <div class="tableDome_tip">
            <el-alert :title="lc('admin_system_00298')" type="success" show-icon>
            </el-alert>
        </div>
        <div class="moduleTable">
            <div class="moduleHuancun">
                <ul>
                    <li>
                        <div class="moduleHcTite">
                            <span>{{ lc('wap_00191') }}</span>
                        </div>
                        <div class="moduleHcKg">
                            <el-radio-group v-model="sy_index_cache">
                                <el-radio label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio label="2">{{ lc('resume_00030') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </li>
                    <li v-for="item in newModel">
                        <div class="moduleHcTite">
                            <span>{{item.value}}</span>
                        </div>
                        <div class="moduleHcKg">
                            <el-radio-group v-model="item.cache">
                                <el-radio v-model="radio" label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio v-model="radio" label="2">{{ lc('resume_00030') }}</el-radio>
                            </el-radio-group>
                        </div>
                    </li>
                </ul>
            </div>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">{{ lc('wap_user_00101') }}</el-button>
        </div>

    </div>
</template>

<script>
import Jibenset from './component/jibenset.vue'
import Navset from './component/navset.vue'
import Seoshezhi from './component/seoshezhi.vue'

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
                    radio: '1',
                    uri:"m=system&c=",
                    newModel:[],
                    sy_index_cache:''
                }

            },
            created(){
                this.getSettplcache();


            },
            components: {
                // 'jibenset': Jibenset,
                // 'navset': Navset,
                // 'seoshezhi': Seoshezhi,
            },
            methods: {
                getSettplcache(){
                    const _this = this;
                    let url =  this.uri+"set_config&a=settplcache";
                    httpGet(url, {}).then(function (response) {
                        var res = response.data;
                        if (res.error == 0) {
                            _this.newModel = res.data.newModel;
                            _this.sy_index_cache = res.data.sy_index_cache;
                        } else {
                            message.error(res.msg);
                        }
                    })
                },
                save:function(){
                    let url =  this.uri+"set_config&a=savetplcache";
                    let _this = this;
                    let ruleForm = {
                        sy_index_cache:this.sy_index_cache
                    }
                    for (const ruleFormKey in this.newModel) {
                        // console.log(ruleFormKey.cache);
                        let key = 'sy_'+ruleFormKey+"_cache"
                        ruleForm[key]=this.newModel[ruleFormKey].cache;
                    }
                    httpPost(url, ruleForm).then(function (response) {
                        var res = response.data;
                        if (res.error == 0) {
                            message.success(res.msg);
                            _this.getSettplcache();
                        } else {
                            message.error(res.msg);
                        }
                    })
                }
            }
        }
</script>
