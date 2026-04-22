<template>
    <div class="setBasicAll">
        <div class="integralTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="260">名称</th>
                        <th width="320">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">积分代替词</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="list.integral_pricename">
                                    <!-- <span slot="suffix" class="slotspan">天</span> -->
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>默认为积分，例：金币</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">最低充值积分</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_min_recharge"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>0 表示不限</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">赏金转换积分次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.paypack_max_recharge">
                                    <span slot="suffix" class="slotspan">次/日</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>0 表示不限</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">赏金转换积分最低金额</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input @input="formatMoney('packprice_min_recharge')" placeholder="请输入内容" v-model="list.packprice_min_recharge">
                                    <span slot="suffix" class="slotspan">元</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>0 表示不限</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">最低充值金额</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input @input="formatMoney('money_min_recharge')" placeholder="请输入内容" v-model="list.money_min_recharge">
                                    <span slot="suffix" class="slotspan">元</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>0 表示不限</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">积分单位</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input placeholder="请输入内容" v-model="list.integral_priceunit"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>默认为点，例：个，位</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">积分兑换比例</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_proportion">
                                    <span slot="suffix" class="slotspan">点</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>例：1元=30点积分</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">每日签到送积分</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_signin">
                                    <span slot="suffix" class="slotspan">个积分</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>第六日起，获得积分将翻倍。</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">会员注册</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_reg">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">每天第一次登录</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_login">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">完善基本资料</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_userinfo">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">邮箱认证</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_emailcert">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">手机认证</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_mobliecert">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">上传头像</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_avatar">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">发布问题</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_question">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">回答问题</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_answer">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">评论回答</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_answerpl">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">邀请注册</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_invite_reg">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
                            <div class="TableTite">绑定微信</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="list.integral_bind_wx">
                                    <span slot="suffix" class="slotspan">个积分</span>
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
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
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
                    message.success('操作成功');
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