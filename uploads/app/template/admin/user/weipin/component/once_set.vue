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
                            <div class="TableTite">前台店铺招聘发布次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_once" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_once')">
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
                            <div class="TableTite">前台店铺招聘发布总次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.sy_once_totalnum" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'sy_once_totalnum')">
                                    <span slot="suffix" class="slotspan">次/天</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>提示：0为不限。网站每天可以发布的店铺招聘总数量</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">查看店铺招聘联系方式</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="ruleForm.user_wzp_link" label="0">公开</el-radio>
                                <el-radio v-model="ruleForm.user_wzp_link" label="1">登录查看</el-radio>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>查看店铺招聘联系方式</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">店铺招聘审核</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.com_fast_status" active-value="0" inactive-value="1">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>店铺招聘审核</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">店铺上传营业执照</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-switch v-model="ruleForm.sy_once_yyzz" active-value="1" inactive-value="2">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>店铺上传营业执照</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">店铺招聘刷新次数</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleForm.com_xin" placeholder=""
                                          @input="inputIntNumber($event, 'ruleForm', 'com_xin')">
                                    <span slot="suffix" class="slotspan">次</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>店铺招聘刷新次数</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">默认店铺形象</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept"
                                           list-type="picture"
                                           action=""
                                           :auto-upload="false"
                                           :on-change="handleChangeLogo"
                                           :show-file-list="false">
                                    <el-button slot="trigger" size="small" type="primary">点击上传</el-button>
                                    <img class="el-upload-list__item-thumbnail" width="200" height="130" style="padding-left: 5px;"
                                         v-if="ruleForm.sy_once_icon_n" :src="ruleForm.sy_once_icon_n"/>
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>建议尺寸：标准尺寸200px*130px</span>
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