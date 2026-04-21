<template>
    <!--会员-企业-套餐服务：套餐设置-设置增值包-->
    <div v-loading="loading" class="drawerModlue">
        <div class="drawerModInfo drawerModInfoOne">
            <div class="adminBoldTips">
                <el-alert title="企业会员增值包等级关乎您的收入问题，请谨慎添加和修改并注意整体的合理性。" show-icon type="success"></el-alert>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>增值类型</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.type" placeholder="请选择" clearable>
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
                    <span>增值包价格</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_price" placeholder="请输入数字">
                        <template slot="prepend">服务价格</template>
                        <span slot="suffix" class="slotspan">元</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>增值包设置</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.job_num" placeholder="请输入数字">
                        <template slot="prepend">上架职位数</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    <el-input v-model="ruleForm.breakjob_num" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">刷新职位数</template>
                        <span slot="suffix" class="slotspan">次</span>
                    </el-input>
                    <el-input v-model="ruleForm.resume" placeholder="请输入数字">
                        <template slot="prepend">下载简历数</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    <el-input v-model="ruleForm.interview" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">邀请面试数</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    <el-input v-model="ruleForm.zph_num" placeholder="请输入数字">
                        <template slot="prepend">招聘会报名数</template>
                        <span slot="suffix" class="slotspan">份</span>
                    </el-input>
                    <el-input v-model="ruleForm.top_num" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">置顶天数</template>
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                    <el-input v-model="ruleForm.urgent_num" placeholder="请输入数字">
                        <template slot="prepend">紧急天数</template>
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                    <el-input v-model="ruleForm.rec_num" placeholder="请输入数字" style="margin-left: 10px;">
                        <template slot="prepend">推荐天数</template>
                        <span slot="suffix" class="slotspan">天</span>
                    </el-input>
                    
                </div>
                <div class="drawerModTips">
                    <el-alert title="上架职位数：套餐模式会员限制企业上架的职位数量，时间模式会员限制企业每天发布的职位数量" type="warning" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>排序</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="请输入数字">
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="大前小后" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
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
                //增值类型
                type: null,
                //服务价格
                service_price: '',
                //上架职位数
                job_num: '',
                //刷新职位数
                breakjob_num: '',
                //下载简历数
                resume: '',
                //邀请面试数
                interview: '',
                //招聘会报名数
                zph_num: '',
                //置顶天数
                top_num: '',
                //紧急天数
                urgent_num: '',
                //推荐天数
                rec_num: '',
                //sy_chat_name
                chat_num: '',
                //排序
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