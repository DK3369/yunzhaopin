<template>
    <!--会员-企业-企业设置：消费设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <div class="admin_datatip">
                {{ lc("admin_integral_exchange_tip", [config.integral_pricename, config.integral_proportion, config.integral_priceunit, 1 / config.integral_proportion]) }}
            </div>
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="260">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00028'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_job" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00349'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00022'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_down_resume" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00349'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00218'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_interview" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00349'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00029'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_jobefresh" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00349'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00024'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.com_urgent" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00350'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00025'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.integral_job_top" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00350'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00023'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.com_recjob" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00350'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_com_00027'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.job_auto" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00350'{/yun}</span></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span> </span>
                        </div>
                    </td>
                </tr>
                <!--下载人才简历定价-->
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_company_00217'{/yun}</div>
                    </td>
                    <td>
                        <div v-for="(item,index) in data" :key="index" class="TableInpt" style="padding: 2px 0;">
                            <el-input v-model="item.days" style="width: 230px;" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <template slot="prepend">{yun:}t key='wap_00326'{/yun}</template>
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_company_00221'{/yun}</span>
                            </el-input>
                            <el-input v-model="item.price" style="width: 210px;margin-left: 10px;"  placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <template slot="prepend">{yun:}t key='admin_user_company_00220'{/yun}</template>
                                <span slot="suffix" class="slotspan">{yun:}t key='admin_user_00349'{/yun}</span>
                            </el-input>
                            <el-button type="text" style="margin-left: 10px;" @click="handleDelete(index)">{yun:}t key='common.delete'{/yun}</el-button>
                        </div>
                        <div>
                            <el-button type="text" icon="el-icon-circle-plus-outline" @click="handleAdd">{yun:}t key='admin_user_company_00219'{/yun}</el-button>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_user_company_00216'{/yun}<br>{yun:}t key='admin_user_company_00215'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            searchForm: {},
            //下载人才简历定价
            data: [],
            config: {},
            ruleForm: {
                //上架职位
                integral_job: '",
                //{yun:}t key='wap_com_00022'{/yun}
                integral_down_resume: "",
                //{yun:}t key='admin_user_company_00218'{/yun}
                integral_interview: "',
                // refreshJob
                integral_jobefresh: '",
                //{yun:}t key='wap_com_00024'{/yun}
                com_urgent: "",
                //{yun:}t key='wap_com_00025'{/yun}
                integral_job_top: "",
                //{yun:}t key='wap_com_00023'{/yun}
                com_recjob: "",
                //{yun:}t key='wap_com_00027'{/yun}
                job_auto: "',
                //sy_chat_name
                integral_chat_num: '',
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
            httpPost('m=user&c=company_comset&a=comspend', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    if (Array.isArray(res.data.data) && res.data.data.length) {
                        _this.data = res.data.data
                    }
                    let config = res.data.config ? res.data.config : {};
                    _this.config.integral_pricename = config.integral_pricename !== undefined ? config.integral_pricename : '';
                    _this.config.integral_proportion = config.integral_proportion !== undefined ? config.integral_proportion : '';
                    _this.config.integral_priceunit = config.integral_priceunit !== undefined ? config.integral_priceunit : '";
                    
                    //{yun:}t key='wap_com_00028'{/yun}
                    _this.ruleForm.integral_job = config.integral_job !== undefined ? config.integral_job : "";
                    //{yun:}t key='wap_com_00022'{/yun}
                    _this.ruleForm.integral_down_resume = config.integral_down_resume !== undefined ? config.integral_down_resume : "";
                    //{yun:}t key='admin_user_company_00218'{/yun}
                    _this.ruleForm.integral_interview = config.integral_interview !== undefined ? config.integral_interview : "';
                    // refreshJob
                    _this.ruleForm.integral_jobefresh = config.integral_jobefresh !== undefined ? config.integral_jobefresh : '";
                    //{yun:}t key='wap_com_00024'{/yun}
                    _this.ruleForm.com_urgent = config.com_urgent !== undefined ? config.com_urgent : "";
                    //{yun:}t key='wap_com_00025'{/yun}
                    _this.ruleForm.integral_job_top = config.integral_job_top !== undefined ? config.integral_job_top : "";
                    //{yun:}t key='wap_com_00023'{/yun}
                    _this.ruleForm.com_recjob = config.com_recjob !== undefined ? config.com_recjob : "";
                    //{yun:}t key='wap_com_00027'{/yun}
                    _this.ruleForm.job_auto = config.job_auto !== undefined ? config.job_auto : "';
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.config = "{yun:}t key='common.submit'{/yun}";
            let arrTmp = [];
            this.data.forEach((item) => {
                arrTmp.push(item.days + '_' + item.price);
            });
            params.integral_down_resume_dayprice = arrTmp.join(':');
            params.integral_job_type = 2;
            params.integral_down_resume_type = 2;
            params.integral_interview_type = 2;
            params.integral_jobefresh_type = 2;
            params.com_urgent_type = 2;
            params.com_recjob_type = 2;
            params.job_auto_type = 2;
            params.integral_jobtop_type = 2;
            _this.submitLoading = true;
            httpPost('m=user&c=company_comset&a=saveComspend', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    // _this.resetForm();
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
        },
        handleAdd() {
            this.data.push({days: '', price: ''});
        },
        handleDelete(index) {
            if (this.data.length <= 1) {
                message.error('再删就没有啦！');
                return false;
            }
            this.data.splice(index, 1);
        },
        /**
         * 消费设置里，根据积分兑换比例限制金额
         * @param price
         * @returns {number}
         */
        payintegral(price) {
            let pro = this.config.integral_proportion;
            let price_n = 0;
            if (pro > 0) {
                price_n = 1 / pro;
            }
            if (price < price_n && price > 0) {
                price = price_n;
            }
            return price;
        },
    },
    watch: {
        "ruleForm.integral_job": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.integral_job = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.integral_down_resume": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.integral_down_resume = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.integral_interview": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.integral_interview = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.integral_jobefresh": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.integral_jobefresh = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.com_urgent": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.com_urgent = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.integral_job_top": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.integral_job_top = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.com_recjob": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.com_recjob = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.job_auto": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    newValue = this.payintegral(newValue);
                    this.ruleForm.job_auto = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        data: {
            handler(newValue, oldValue) {
                if (Array.isArray(newValue)) {
                    newValue.forEach((item, index) => {
						if(item.days || item.price){
							item.days = item.days.replace(/[^0-9.]/g, '');
							let price = item.price.replace(/[^0-9.]/g, '');
							item.price = this.payintegral(price);
							this.data[index] = item;
						}
                    });
                }
            },
            deep: true,
            immediate: true
        },
    }
};
</script>
<style scoped>
.mt-10 {
    margin-top: 10px;
}
</style>