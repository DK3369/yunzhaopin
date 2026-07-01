<template>
    <div class="setBasicAll">
        <div class="integralTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="260">{{ lc('member_com_00021') }}</th>
                        <th width="320">{{ lc('member_user_00181') }}</th>
                        <th>{{ lc('member_com_00207') }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('common.publish_resume') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_add_resume" onKeyUp="this.value=this.value.replace(/[^0-9.]/g,'')">
                                    <span slot="suffix" class="slotspan">{{ lc('admin_00891') }}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('common.publish_resume') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('wap_user_00106') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="number" :placeholder="lc('wap_user_00076')" v-model="list.integral_identity" onKeyUp="this.value=this.value.replace(/[^0-9.]/g,'')">
                                    <span slot="suffix" class="slotspan">{{ lc('admin_00891') }}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('wap_user_00106') }}</span>
                            </div>
                        </td>
                    </tr>
                    
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        list:Object,
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
    methods: {
        save: function () {
            let _this = this;
            let url = this.uri + "set_integral&a=saveSet";
            let ruleForm = {
                integral_add_resume: _this.list.integral_add_resume,
                integral_identity: _this.list.integral_identity,
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success(lc('wap_user_00264'));
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