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
                            <div class="TableTite">{yun:}t key='admin_00923'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="integral_ltcert">
                                    <span slot="suffix" class="slotspan">{yun:}t key='admin_00891'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00923'{/yun}</span>
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
        lietou: String
    },
    data: function () {
        return {
            value: '',
            input3: '',
            radio: '1',
            integral_ltcert: this.lietou,
            uri: "m=system&c=",
            saveLoading: false
        }
    },
    methods: {
        save: function () {

            let _this = this;
            let url = this.uri + "set_integral&a=lietou_save";
            let ruleForm = {
                integral_ltcert: _this.integral_ltcert,
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