<template>
<div id="moduapp" class="moduleDome">
        <div class="setDomeAll setDomeInte">
            <el-tabs v-model="activeName" @tab-click="handleClick">
                <el-tab-pane :label="lc('admin_yunying_00095')" name="first">
                    <jifencz :pricename="integral_pricename"></jifencz>
                </el-tab-pane>
                <el-tab-pane :label="lc('admin_yunying_00094')" name="second">
                    <comhytc :ratinglist="rating_list" :ratingid="ratingid"></comhytc>
                </el-tab-pane> 
                <el-tab-pane :label="lc('admin_yunying_00093')" name="three">
                    <comhyzzb :service-list="service_list"></comhyzzb>
                </el-tab-pane> 
            </el-tabs>
        </div>
    </div>
</template>

<script>
import Jifencz from './component/jifencz.vue'
import Comhytc from './component/comhytc.vue'
import Comhyzzb from './component/comhyzzb.vue'

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
                    rating_list:[],
                    service_list :[],
                    integral_pricename:'',
                    integral_priceunit:'',
                    
                    uri:"m=yunying&c=",
                    ratingid:''
                }
            },
            created:function (){
                this.index();


            },
            components: {
                'jifencz': Jifencz,
                'comhytc': Comhytc, 
                'comhyzzb': Comhyzzb, 
            },
            methods: {
                handleClick(tab, event) {
                    console.log(tab, event);
                },
                index:function () {
                    let _this = this;
                    let  url= _this.uri+'finance_recharge&a=index';
                    let sendData = {}
                    httpPost(url, sendData).then(function (response) {
                        let res = response.data;
                        if (res.error == 0) {
                            _this.rating_list = res.data.rating_list;
                            _this.service_list = res.data.service_list;
                            _this.integral_pricename = res.data.integral_pricename;
                            _this.integral_priceunit = res.data.integral_priceunit;
                            
                            _this.ratingid = res.data.ratingid
                            console.log(_this.rating_list);
                        }
                    })
                }
            }
        }
</script>
