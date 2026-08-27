<template>
    <div class="moduleElHight">
        <div class="admin_datatip">
            <i class="el-icon-document"></i>
            {{ lc('admin_01257') }}
        </div>
        <div class=" moduleTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="200">{{ lc('member_com_00021') }}</th>
                        <th width="400">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01258') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="userarr" placeholder=" "></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01259') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01260') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="fs" label="1">{{ lc('admin_user_00156') }}</el-radio>
                                <el-radio v-model="fs" label="2">{{ lc('admin_yunying_00092') }}</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_01260') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01261') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="integral" size="20" maxlength="16" placeholder=" " onKeyUp="this.value=this.value.replace(/[^0-9.]/g,'')"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span></span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_00563') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input name="order_price" id="order_price" size="20" maxlength="16" v-model="order_price" onKeyUp="priceCk(this)" placeholder=" ">
                                    <template #suffix><span class="slotspan">{{ lc('common_02056') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span></span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_01246') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" :placeholder="lc('wap_user_00076')" v-model="remark">
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span></span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="save" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
            </div>
        </div>
    </div>
</template>
    
<script>
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
    props: {
        pricename: String,
    },
    data: function () {
        return {
            submitLoading:false,
            input: '',
            userarr:'',
            fs: '1',
            integral:'',
            order_price: '',
            remark: '',
            uri:"m=yunying&c=",
        }
    },
    mounted() {

    },
    methods: {
        save:function(){
            let userarr = this.userarr;
            let integral = this.integral;
            if (userarr == '') {
                message.error(lc('wap_01010'));
                return false;
            }
            if (integral < 1) {
                message.error(lc('admin_00245')+this.pricename+'！');
                return false;
            }
            let _this = this;
            let  url= _this.uri+'finance_recharge&a=jifenSave';
            let sendData ={
                fs:this.fs,
                integral:this.integral,
                order_price:this.order_price,
                remark:this.remark,
                userarr:userarr
            };
            this.submitLoading = true;
            httpPost(url, sendData).then(function (response) {
                let res = response.data;
                if (res.error == 0) {
                    message.success(res.msg);
                    _this.fs =1;
                    _this.userarr='';
                    _this.integral='';
                    _this.order_price= '';
                    _this.remark='';
                }else {
                    message.error(res.msg)
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });

        }
    },
};
</script>
<style scoped>
.moduleTable {
    max-height: calc(100% - (60px + 10px));
}
</style>