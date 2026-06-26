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
                            <div class="TableTite">{yun:}t key='admin_00430'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_tiny" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_tiny')">
                                    <span slot="suffix" class="slotspan">{yun:}t key='wap_com_00049'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_user_weipin_00055'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00431'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_tiny_totalnum" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_tiny_totalnum')">
                                    <span slot="suffix" class="slotspan">{yun:}t key='wap_com_00049'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_user_weipin_00068'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00432'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.user_wjl" active-value="0" inactive-value="1">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00432'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_user_weipin_00069'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="ruleForm.user_wjl_link" label="1">{yun:}t key='admin_00417'{/yun}</el-radio>
                                <el-radio v-model="ruleForm.user_wjl_link" label="0">{yun:}t key='wap_js_00005'{/yun}</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_user_weipin_00069'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submit" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
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