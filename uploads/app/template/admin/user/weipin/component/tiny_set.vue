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
                            <div class="TableTite">前台普工简历发布次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_tiny" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_tiny')">
                                    <span slot="suffix" class="slotspan">次/天</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>提示：0为不限。按IP限制每天发布次数</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">前台普工简历发布总次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_tiny_totalnum" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_tiny_totalnum')">
                                    <span slot="suffix" class="slotspan">次/天</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>提示：0为不限。网站每天可以发布的普工简历总数量</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">开启普工简历审核</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.user_wjl" active-value="0" inactive-value="1">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>开启普工简历审核</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">开启普工联系方式查看</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="ruleForm.user_wjl_link" label="1">登录查看</el-radio>
                                <el-radio v-model="ruleForm.user_wjl_link" label="0">公开</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>开启普工联系方式查看</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submit" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    data: function () {
        return {
            saveLoading: false,

            ruleForm: {},
        }
    },

    mounted() {

    },
    created() {
        this.init();
    },
    methods: {
        init() {
            this.getData();
        },

        getData() {
            let that = this;
            httpPost('m=user&c=weipin_tiny&a=set').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.ruleForm = data.config;
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },

        submit() {
            let that = this,
                ruleForm = that.ruleForm;

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            httpPost('m=user&c=weipin_tiny&a=tinyset', ruleForm).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function() {
                        that.$set(that.ruleForm, 'sy_once_icon', '');
                    });
                }
            })
        },
    },
};
</script>
<style scoped></style>