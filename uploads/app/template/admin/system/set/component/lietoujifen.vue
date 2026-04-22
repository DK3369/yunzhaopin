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
                            <div class="TableTite">猎头执照认证</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" placeholder="请输入内容" v-model="integral_ltcert">
                                    <span slot="suffix" class="slotspan">个积分</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>猎头执照认证</span>
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