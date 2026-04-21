<template>
    <!--会员-个人-个人设置：头像设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th>状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">默认男生头像</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="upload-demo" :accept="pic_accept" :action="uploadAction" :on-change="uploadChangeMan"
                                :on-remove="handleRemoveMan" :file-list="fileListMan"
                                list-type="picture">
                                <el-button size="small" type="primary">点击上传</el-button>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>只能上传jpg/png/jpeg/gif文件</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">默认女生头像</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="upload-demo" :accept="pic_accept" :action="uploadAction" :on-change="uploadChangeWoman"
                                :on-remove="handleRemoveWoman" :file-list="fileListWoman"
                                list-type="picture">
                                <el-button size="small" type="primary">点击上传</el-button>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>只能上传jpg/png/jpeg/gif文件</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">默认职业技能和作品</div>
                    </td>
                    <td>
                        <div class="TableUpload" style="display: flex;align-items: center;">
                            <el-upload :action="action"
                                :data="{ name: 'sy_member_skill', path: 'logo', imgid: 'imgskill', pytoken: pytoken }"
                                :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'fileList')"
                                :accept="pic_accept"
                                :show-file-list="false">
                                <el-button size="small" type="primary">点击上传</el-button>
                            </el-upload>
                            <div class="up_sy_logo_div" style="margin-left: 15px;">
                                <el-image v-if="sy_member_skill" style="width:100px;" :src="sy_member_skill"
                                    :preview-src-list="sy_member_skill ? [sy_member_skill] : []"></el-image>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>只能上传jpg/png/jpeg/gif文件</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交</el-button>
        </div>

        <el-dialog :visible.sync="dialogVisible">
            <img width="100%" :src="dialogImageUrl" alt="">
        </el-dialog>
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
            curl:'',
            dialogImageUrl: '',
            dialogVisible: false,
            ruleForm: {
                //默认男生头像
                manicon_sys: [],
                //默认女生头像
                womanicon_sys: [],
            },
            sy_member_skill: '',
            fileListMan: [],
            fileListWoman: [],
            submitLoading: false,
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    created() {
        this.getList();
    },
    methods: {
        onSuccess: function (res, file, fileList, arr) {
            fileList.splice(fileList.length - 1, 1);
            if (res.code === 0) {
                this.sy_member_skill = res.data.url;
                if (fileList.length < 1) {
                    fileList = [{name: '', url: res.data.url}];
                } else {
                    fileList[0]['url'] = res.data.url;
                }
            } else {
                message.error(res.msg)
            }
        },
        uploadChangeMan(file, fileList) {
            if (file.status !== 'success') return;
            let numMax = 6;
            if (fileList.length > numMax) {
                fileList.splice(numMax, fileList.length - numMax);
                message.error("最多只能设置6个默认头像！");
                return false;
            }
            this.fileListMan = fileList;//新元素有raw
        },
        handleRemoveMan(file, fileList) {
            this.fileListMan = fileList;
        },
        uploadChangeWoman(file, fileList) {
            if (file.status !== 'success') return;
            let numMax = 6;
            if (fileList.length > numMax) {
                fileList.splice(numMax, fileList.length - numMax);
                message.error("最多只能设置6个默认头像！");
                return false;
            }
            this.fileListWoman = fileList;//新元素有raw
        },
        handleRemoveWoman(file, fileList) {
            this.fileListWoman = fileList;
        },
        getList() {
            let _this = this;
            httpPost('m=user&c=users_userset&a=logo', {}).then(function (response) {
                let res = response.data;
                this.curl  = res.data.curl;
                if (res.error === 0) {
                    _this.fileListMan = [];
                    res.data.sy_member_icon_arr.forEach((item) => {
                        _this.fileListMan.push({name: '', url: _this.curl + '/' + item});
                    });
                    _this.fileListWoman = [];
                    res.data.sy_member_iconv_arr.forEach((item) => {
                        _this.fileListWoman.push({name: '', url: _this.curl + '/' + item});
                    });
                    _this.sy_member_skill = '';
                    _this.sy_member_skill = res.data.sy_member_skill ? _this.curl + '/' + res.data.sy_member_skill : '';
                }
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let formData = new FormData();
            formData.append('submit', '提交');
            _this.fileListMan.forEach((item) => {
                if (item.hasOwnProperty('raw')) {
                    formData.append('man_files[]', item.raw);
                } else {
                    formData.append('manicon_sys[]', item.url.replace(_this.curl + '/', ''))
                }
            });
            _this.fileListWoman.forEach((item) => {
                if (item.hasOwnProperty('raw')) {
                    formData.append('woman_files[]', item.raw);
                } else {
                    formData.append('womanicon_sys[]', item.url.replace(_this.curl + '/', ''))
                }
            });
            _this.submitLoading = true;
            httpPost('m=user&c=users_userset&a=saveLogo', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.getList();
                } else {
                    message.error(res.msg);
                }
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
.TableUpload .el-upload-list {
    display: flex;
    flex-wrap: wrap;
}

.TableUpload .el-upload-list li {
    width: 92px;
    margin-right: 10px;
}
</style>