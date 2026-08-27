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
                            <div class="TableTite">{{ lc('admin_00414') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_once" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_once')">
                                    <template #suffix><span class="slotspan">{{ lc('wap_com_00049') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_weipin_00055') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00415') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_once_totalnum" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_once_totalnum')">
                                    <template #suffix><span class="slotspan">{{ lc('wap_com_00049') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_weipin_00054') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00416') }}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="ruleForm.user_wzp_link" label="0">{{ lc('wap_js_00005') }}</el-radio>
                                <el-radio v-model="ruleForm.user_wzp_link" label="1">{{ lc('admin_00417') }}</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00416') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_user_weipin_00057') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.com_fast_status" active-value="0" inactive-value="1">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_weipin_00057') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_user_weipin_00056') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.sy_once_yyzz" active-value="1" inactive-value="2">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_user_weipin_00056') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00418') }}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.com_xin" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'com_xin')">
                                    <template #suffix><span class="slotspan">{{ lc('common_02089') }}</span></template>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00418') }}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{{ lc('admin_00419') }}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept"
                                           list-type="picture"
                                           action=""
                                           :auto-upload="false"
                                           :on-change="handleChangeLogo"
                                           :show-file-list="false">
                                    <el-button size="small" type="primary">{{ lc('wap_js_00071') }}</el-button>
                                    <img class="el-upload-list__item-thumbnail" width="200" height="130" style="padding-left: 5px;"
                                         v-if="ruleForm.sy_once_icon_n" :src="ruleForm.sy_once_icon_n"/>
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{{ lc('admin_00420') }}</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submit" :disabled="saveLoading">{{ lc('common.submit') }}</el-button>
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
            pic_accept: localStorage.getItem("pic_accept"),
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
            httpPost('m=user&c=weipin_once&a=set').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.ruleForm = data.config;
            })
        },

        inputIntNumber(val, form, key) {
            this.$data[form][key] = val.replace(/[^0-9]/g,'');
        },

        // 上传时触发
        handleChangeLogo(file, fileList) {
            this.$set(this.ruleForm, 'sy_once_icon', file.raw);
            this.$set(this.ruleForm, 'sy_once_icon_n', file.url);
        },

        submit() {
            let that = this,
                ruleForm = that.ruleForm,
                formData = new FormData();

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            $.each(ruleForm, function(key, value){
                if (key != 'sy_once_icon_n') {
                    formData.append(key, value);
                }
            });

            httpPost('m=user&c=weipin_once&a=onceset', formData).then(function (response) {
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