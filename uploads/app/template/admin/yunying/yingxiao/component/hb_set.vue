<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="560">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">招聘海报</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-switch v-model="ruleForm.sy_haibao_isopen"
                                           active-value="1" inactive-value="0">
                                </el-switch>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span></span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">海报网站标识</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio v-model="ruleForm.sy_haibao_web_type" label="1">文字</el-radio>
                                <el-radio v-model="ruleForm.sy_haibao_web_type" label="2">LOGO</el-radio>
                                <el-radio v-model="ruleForm.sy_haibao_web_type" label="3">留空</el-radio>
                            </div>
                            <!-- 选择文字时显示的内容 -->
                            <div v-if="ruleForm.sy_haibao_web_type == 1" class="TableButn" style="margin-top: 12px;">
                                <el-input v-model="ruleForm.sy_haibao_web_name" placeholder="请输入网站名称"></el-input>
                            </div>
                            <!-- 选择logo时显示的内容 -->
                            <div v-if="ruleForm.sy_haibao_web_type == 2" class="TableUpload" style="margin-top: 12px;">
                                <el-upload class="upload-demo" :accept="pic_accept"
                                           list-type="picture"
                                           action=""
                                           :auto-upload="false"
                                           :on-change="handleChangeLogo"
                                           :show-file-list="false">
                                    <el-button slot="trigger" size="small" type="primary">点击上传</el-button>
                                    <img class="el-upload-list__item-thumbnail" width="100" height="100" style="padding-left: 5px;"
                                         v-if="ruleForm.sy_haibao_web_logo_n" :src="ruleForm.sy_haibao_web_logo_n"/>
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>选择LOGO请上传透明图，建议图片最大尺寸960px X 130px</span>
                            </div>
                        </td>
                    </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submit" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {},
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            ruleForm: {},

            saveLoading: false,
        }
    },

    created() {
        this.getData();
    },
    methods: {
        init() {
            this.getData();
        },

        getData() {
            let that = this;
            httpPost('m=yunying&c=yingxiao_hbconfig').then(function (response) {
                let res = response.data,
                    data = res.data;

                that.ruleForm = data.config;
            })
        },

        // 上传时触发
        handleChangeLogo(file, fileList) {
            this.$set(this.ruleForm, 'sy_haibao_web_logo', file.raw);
            this.$set(this.ruleForm, 'sy_haibao_web_logo_n', file.url);
        },

        submit() {
            let that = this,
                ruleForm = that.ruleForm,
                formData = new FormData();

            if (ruleForm.sy_haibao_web_type == '1') {
                if(ruleForm.sy_haibao_web_name === ''){
                    message.error('请输入网站名称');
                    return false;
                }else if(ruleForm.sy_haibao_web_name.length > 12){
                    message.error('网站名称不能大于12个字符');
                    return false;
                }
            } else if (ruleForm.sy_haibao_web_type == '2') {
                if (!ruleForm.sy_haibao_web_logo_n) {
                    message.error('请上传LOGO图片');
                    return false;
                }
            }

            if (that.saveLoading) {
                return false;
            }
            that.saveLoading = true;

            $.each(ruleForm, function(key, value){
                if (key != 'sy_haibao_web_logo_n') {
                    formData.append(key, value);
                }
            });

            httpPost('m=yunying&c=yingxiao_hbconfig&a=saveSet', formData).then(function (response) {
                let res = response.data;

                that.saveLoading = false;
                if (res.error > 0) {
                    message.error(res.msg);
                } else {
                    message.success(res.msg, function() {
                        that.$set(that.ruleForm, 'sy_haibao_web_logo', '');
                    });
                }
            })
        },
    },
};
</script>
<style scoped></style>