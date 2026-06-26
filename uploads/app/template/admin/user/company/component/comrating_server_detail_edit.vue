<template>
    <!--会员-企业-套餐服务：套餐设置-设置增值包-->
    <div v-loading="loading" class="drawerModlue">
        <div class="drawerModInfo drawerModInfoOne">
            <div class="adminBoldTips">
                <el-alert title="{yun:}t key='admin_00699'{/yun}" show-icon type="success"></el-alert>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00214'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.type" placeholder="{yun:}t key='wap_user_00100'{/yun}" clearable>
                        <el-option
                            v-for="item in zzData"
                            :key="item.id"
                            :label="item.name"
                            :value="item.id">
                        </el-option>
                    </el-select>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00212'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_price" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='admin_user_company_00021'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02056'{/yun}</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00213'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.job_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='member_com_00033'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.breakjob_num" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='member_com_00035'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02089'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.resume" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='member_com_00034'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.interview" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='member_com_00037'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.zph_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='admin_user_company_00211'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.top_num" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='wap_user_00209'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.urgent_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='wap_com_00043'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.rec_num" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='wap_com_00041'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                    
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_00700'{/yun}" type="warning" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='member_com_00022'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00197'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        tid: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            loading: false,
            submitLoading: false,
            config: {},
            zzData: [],//增值类型列表
            ruleForm: {
                // value-addedtype
                type: null,
                //服务价格
                service_price: '",
                //{yun:}t key='member_com_00033'{/yun}
                job_num: "",
                //{yun:}t key='member_com_00035'{/yun}
                breakjob_num: "",
                //{yun:}t key='member_com_00034'{/yun}
                resume: "",
                //{yun:}t key='member_com_00037'{/yun}
                interview: "",
                //{yun:}t key='admin_user_company_00211'{/yun}
                zph_num: "",
                //{yun:}t key='wap_user_00209'{/yun}
                top_num: "",
                //{yun:}t key='wap_com_00043'{/yun}
                urgent_num: "",
                //{yun:}t key='wap_com_00041'{/yun}
                rec_num: "',
                //sy_chat_name
                chat_num: '',
                // Sort
                sort: '',
            },
        }
    },
    created() {
        this.getZzData();
        if (this.tid > 0) {
            this.getInfo();
        }
    },
    methods: {
        getInfo() {
            let _this = this;
            let params = {tid: this.tid};
            _this.loading = true;
            httpPost('m=user&c=company_comrating&a=edittc', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let row = res.data;
                    Object.keys(_this.ruleForm).forEach((key) => {
                        if (row.hasOwnProperty(key)) {
                            _this.ruleForm[key] = row[key];
                        }
                    });
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        },
        getZzData() {
            let _this = this;
            httpPost('m=user&c=company_comrating&a=zzData', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.config = res.data.config;
                    _this.zzData = res.data.zzlist;
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.tid = this.tid;
            if (!params.type) {
                message.error('请选择增值类型！');
                return false;
            }
            if (parseFloat(params.service_price) < 0) {
                message.error('请填写服务价格！');
                return false;
            }
            _this.submitLoading = true;
            httpPost('m=user&c=company_comrating&a=saves', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event-list");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
    },
    watch: {
        "ruleForm.service_price": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.service_price = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.job_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.job_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.breakjob_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.breakjob_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.resume": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.resume = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.interview": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.interview = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.zph_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.zph_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.top_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.top_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.urgent_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.urgent_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.rec_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.rec_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        
        "ruleForm.sort": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.sort = newValue;
                }
            },
            deep: true,
            immediate: true
        },
    },
};
</script>
<style scoped>
</style>