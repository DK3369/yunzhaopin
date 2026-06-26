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
                            <div class="TableTite">{yun:}t key='admin_00414'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_once" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_once')">
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
                            <div class="TableTite">{yun:}t key='admin_00415'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_once_totalnum" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_once_totalnum')">
                                    <span slot="suffix" class="slotspan">{yun:}t key='wap_com_00049'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_user_weipin_00054'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00416'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="ruleForm.user_wzp_link" label="0">{yun:}t key='wap_js_00005'{/yun}</el-radio>
                                <el-radio v-model="ruleForm.user_wzp_link" label="1">{yun:}t key='admin_00417'{/yun}</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00416'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_user_weipin_00057'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.com_fast_status" active-value="0" inactive-value="1">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_user_weipin_00057'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_user_weipin_00056'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.sy_once_yyzz" active-value="1" inactive-value="2">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_user_weipin_00056'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00418'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.com_xin" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'com_xin')">
                                    <span slot="suffix" class="slotspan">{yun:}t key='common_02089'{/yun}</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00418'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00419'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept"
                                           list-type="picture"
                                           action=""
                                           :auto-upload="false"
                                           :on-change="handleChangeLogo"
                                           :show-file-list="false">
                                    <el-button slot="trigger" size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <img class="el-upload-list__item-thumbnail" width="200" height="130" style="padding-left: 5px;"
                                         v-if="ruleForm.sy_once_icon_n" :src="ruleForm.sy_once_icon_n"/>
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00420'{/yun}</span>
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