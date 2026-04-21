<template>
    <!--会员-个人-个人设置：消费设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">个人简历置顶费用</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_resume_top" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">元/天</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>个人简历置顶费用</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">个人委托简历费用</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.pay_trust_resume" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">元/份</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>个人委托简历费用</span>
                        </div>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            searchForm: {},
            ruleForm: {
                integral_resume_top_type: 2,
                //个人简历置顶费用
                integral_resume_top: '',
                //个人委托简历费用
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
            params.config = '提交';
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
