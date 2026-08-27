<template>
    <div class="setBasicAll">
        <div class="integralTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="260">{{ lc('member_com_00021') }}</th>
                        <th width="320">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00309') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="list.integral_pricename">
                                    <!-- <template #suffix><span class="slotspan">天</span></template> -->
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00915') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00306') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_min_recharge"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00916') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00304') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.paypack_max_recharge">
                                    <template #suffix><span class="slotspan">{{ lc('admin_system_00311') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00916') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00303') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input @input="formatMoney('packprice_min_recharge')" :placeholder="lc('wap_user_00076')" v-model="list.packprice_min_recharge">
                                    <template #suffix><span class="slotspan">{{ lc('common_02056') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00916') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00307') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input @input="formatMoney('money_min_recharge')" :placeholder="lc('wap_user_00076')" v-model="list.money_min_recharge">
                                    <template #suffix><span class="slotspan">{{ lc('common_02056') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00916') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00310') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input :placeholder="lc('wap_user_00076')" v-model="list.integral_priceunit"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00917') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00308') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_proportion">
                                    <template #suffix><span class="slotspan">{{ lc('common_02090') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00918') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_system_00305') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_signin">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00919') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00920') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_reg">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_01025') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_login">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_00990') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_userinfo">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_com_00186') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_emailcert">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('member_com_00071') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_mobliecert">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_user_00110') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_avatar">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_user_00112') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_question">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_user_00113') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_answer">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_00995') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_answerpl">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_01590') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_invite_reg">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('member_user_00056') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_bind_wx">
                                    <template #suffix><span class="slotspan">{{ lc('admin_00891') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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
        list: Object
    },
    data: function () {
        return {
            value: '',
            input3: '',
            radio: '1',
            uri: "m=system&c=",
            saveLoading: false
        }
    },
    mounted() {

    },
    methods: {
		formatMoney:function(key){
			formatMoney(this.list[key],this.list,key,this);
		},
        save: function () {
            let _this = this;
            let url = this.uri + "set_integral&a=save";
            let ruleForm = {
                integral_pricename: _this.list.integral_pricename,
                integral_priceunit: _this.list.integral_priceunit,
                integral_min_recharge: _this.list.integral_min_recharge,
                money_min_recharge: _this.list.money_min_recharge,
                paypack_max_recharge: _this.list.paypack_max_recharge,
                packprice_min_recharge: _this.list.packprice_min_recharge,
                integral_proportion: _this.list.integral_proportion,
                integral_signin: _this.list.integral_signin,
                integral_reg: _this.list.integral_reg,
                integral_login: _this.list.integral_login,
                integral_userinfo: _this.list.integral_userinfo,
                integral_emailcert: _this.list.integral_emailcert,
                integral_mobliecert: _this.list.integral_mobliecert,
                integral_avatar: _this.list.integral_avatar,
                integral_question: _this.list.integral_question,
                integral_answer: _this.list.integral_answer,
                integral_answerpl: _this.list.integral_answerpl,
                integral_invite_reg: _this.list.integral_invite_reg,
                integral_bind_wx: _this.list.integral_bind_wx,
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success(lc('wap_user_00264'));
                    _this.$emit('get-list', true)
                } else {
                    message.error(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        }
    },
};
</script>
<style scoped></style>