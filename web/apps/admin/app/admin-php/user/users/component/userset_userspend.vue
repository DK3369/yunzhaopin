<template>
    <!--会员-个人-个人设置：消费设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00348') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_resume_top" :placeholder="lc('admin_user_00342')">
                                <template #suffix><span class="slotspan">{{ lc('admin_user_00350') }}</span></template>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_00348') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00347') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.pay_trust_resume" :placeholder="lc('admin_user_00342')">
                                <template #suffix><span class="slotspan">{{ lc('admin_user_00349') }}</span></template>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_00347') }}</span>
                        </div>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.submit') }}</el-button>
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
    data: function () {
        return {
            searchForm: {},
            ruleForm: {
                integral_resume_top_type: 2,
                //个人简历置顶费用
                integral_resume_top: '',
                //{{ lc('admin_user_00347') }}
                pay_trust_resume: '',
            },
            submitLoading: false,
        }
    },
    created() {
        this.getList();
    },
    methods: {
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            httpPost('m=user&c=users_userset&a=userspend', params, { hideloading: true }).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let config = res.data.config;
                    //手机认证才能申请悬赏职位
                    _this.ruleForm.integral_resume_top = config.integral_resume_top !== undefined ? config.integral_resume_top : '';
                    _this.ruleForm.pay_trust_resume = config.pay_trust_resume !== undefined ? config.pay_trust_resume : '';
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.config = lc('common.submit');
            _this.submitLoading = true;
            httpPost('m=user&c=users_userset&a=saveSpend', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.getList();
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        resetForm(formName) {
            //this.$refs[formName].resetFields();
        }
    },
    watch: {
        "ruleForm.integral_resume_top": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.integral_resume_top = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.pay_trust_resume": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.pay_trust_resume = newValue;
                }
            },
            deep: true,
            immediate: true
        },
    }
};
</script>
<style scoped></style>
