<template>
    <!--会员-企业-企业设置：头像设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th width="260">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">默认LOGO</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :action="action"
                                :data="{ name: 'sy_unit_icon', path: 'logo', imgid: 'imgicon', pytoken: pytoken }"
                                :on-success="(response, file, fileList) =>onSuccessLogo(response,file,fileList,'fileList')"
                                :accept="pic_accept"
                                :show-file-list="false">
                                <el-button size="small" type="primary">上传图片</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.sy_unit_icon" style="width:100px;" :src="ruleForm.sy_unit_icon"
                                    :preview-src-list="ruleForm.sy_unit_icon ? [ruleForm.sy_unit_icon] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>企业没有上传logo时展示的图片</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">默认顾问头像</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :action="action"
                                :data="{ name: 'sy_guwen', path: 'logo', imgid: 'imggw', pytoken: pytoken }"
                                :on-success="(response, file, fileList) =>onSuccessGuwen(response,file,fileList,'fileList')"
                                :accept="pic_accept"
                                :show-file-list="false">
                                <el-button size="small" type="primary">上传图片</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.sy_guwen" style="width:100px;" :src="ruleForm.sy_guwen"
                                    :preview-src-list="ruleForm.sy_guwen ? [ruleForm.sy_guwen] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>企业中心销售顾问的默认头像</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">默认企业横幅</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :action="action"
                                :data="{ name: 'sy_banner', path: 'logo', imgid: 'imgbanner', pytoken: pytoken }"
                                :on-success="(response, file, fileList) =>onSuccessBanner(response,file,fileList,'fileList')"
                                :accept="pic_accept"
                                :show-file-list="false">
                                <el-button size="small" type="primary">上传图片</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.sy_banner" style="width:100px;" :src="ruleForm.sy_banner"
                                    :preview-src-list="ruleForm.sy_banner ? [ruleForm.sy_banner] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>企业未上传横幅时的默认图片</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">二维码提示图</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :action="action"
                                :data="{ name: 'sy_member_ewm', path: 'logo', imgid: 'imgewm', pytoken: pytoken }"
                                :on-success="(response, file, fileList) =>onSuccessEwm(response,file,fileList,'fileList')"
                                :accept="pic_accept"
                                :show-file-list="false">
                                <el-button size="small" type="primary">上传图片</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="ruleForm.sy_member_ewm" style="width:100px;" :src="ruleForm.sy_member_ewm"
                                    :preview-src-list="ruleForm.sy_member_ewm ? [ruleForm.sy_member_ewm] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>未上传二维码时的默认图片</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            action: baseUrl + 'm=index&c=layui_upload',
            pytoken: localStorage.getItem("pytoken"),
            sy_weburl: localStorage.getItem("sy_weburl"),
            searchForm: {},
            ruleForm: {
                'submit': '提交',
                'sy_unit_icon': '',//默认LOGO
                'sy_guwen': '',//默认顾问头像
                'sy_banner': '',//默认企业横幅
                'sy_member_ewm': '',//未上传二维码提示图
            },
            submitLoading: false,
        }
    },
    created() {
        this.getList();
    },
    methods: {
        onSuccessLogo: function (res, file, fileList, arr) {
            fileList.splice(fileList.length - 1, 1);
            if (res.code === 0) {
                this.ruleForm.sy_unit_icon = res.data.url;
                if (fileList.length < 1) {
                    fileList = [{name: '', url: res.data.url}];
                } else {
                    fileList[0]['url'] = res.data.url;
                }
            } else {
                message.error(res.msg)
            }
        },
        onSuccessGuwen: function (res, file, fileList, arr) {
            fileList.splice(fileList.length - 1, 1);
            if (res.code === 0) {
                this.ruleForm.sy_guwen = res.data.url;
                if (fileList.length < 1) {
                    fileList = [{name: '', url: res.data.url}];
                } else {
                    fileList[0]['url'] = res.data.url;
                }
            } else {
                message.error(res.msg)
            }
        },
        onSuccessBanner: function (res, file, fileList, arr) {
            fileList.splice(fileList.length - 1, 1);
            if (res.code === 0) {
                this.ruleForm.sy_banner = res.data.url;
                if (fileList.length < 1) {
                    fileList = [{name: '', url: res.data.url}];
                } else {
                    fileList[0]['url'] = res.data.url;
                }
            } else {
                message.error(res.msg)
            }
        },
        onSuccessEwm: function (res, file, fileList, arr) {
            fileList.splice(fileList.length - 1, 1);
            if (res.code === 0) {
                this.ruleForm.sy_member_ewm = res.data.url;
                if (fileList.length < 1) {
                    fileList = [{name: '', url: res.data.url}];
                } else {
                    fileList[0]['url'] = res.data.url;
                }
            } else {
                message.error(res.msg)
            }
        },
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            httpPost('m=user&c=company_comset&a=logo', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let config = res.data.config;
                    _this.ruleForm.sy_unit_icon = config.sy_unit_icon ;
                    _this.ruleForm.sy_guwen = config.sy_guwen ;
                    _this.ruleForm.sy_banner = config.sy_banner;
                    _this.ruleForm.sy_member_ewm = config.sy_member_ewm ;
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            let formData = new FormData();
            Object.keys(params).forEach((key) => {
                formData.append(key, params[key].replace(_this.sy_weburl + '/', ''));
            });
            _this.submitLoading = true;
            httpPost('m=user&c=company_comset&a=logo', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                } else {
                    message.error(res.msg);
                }
                _this.$emit("child-event-list");
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        resetForm(formName) {
            //this.$refs[formName].resetFields();
        }
    },
};
</script>
<style scoped>

</style>