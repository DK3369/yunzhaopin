<template>
    <div class="setBasicAll">
        <div class="integralTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="260">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="320">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00309'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_pricename">
                                    <!-- <span slot="suffix" class="slotspan">天</span> -->
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00915'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00306'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_min_recharge"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00916'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00304'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.paypack_max_recharge">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_system_00311'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00916'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00303'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input @input="formatMoney('packprice_min_recharge')" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.packprice_min_recharge">
                                    <span slot="suffix" class="slotspan">{yun:}t key='common_02056'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00916'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00307'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input @input="formatMoney('money_min_recharge')" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.money_min_recharge">
                                    <span slot="suffix" class="slotspan">{yun:}t key='common_02056'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00916'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00310'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_priceunit"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00917'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00308'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_proportion">
                                    <span slot="suffix" class="slotspan">{yun:}t key='common_02090'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00918'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00305'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_signin">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00919'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00920'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_reg">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_01025'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_login">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_00990'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_userinfo">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_com_00186'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_emailcert">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='member_com_00071'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_mobliecert">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_user_00110'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_avatar">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_user_00112'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_question">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_user_00113'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_answer">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_00995'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_answerpl">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='wap_01590'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_invite_reg">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
                            <div class="TableTite">{yun:}t key='member_user_00056'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="list.integral_bind_wx">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
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
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
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
                    message.success("{yun:}t key='wap_user_00264'{/yun}");
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