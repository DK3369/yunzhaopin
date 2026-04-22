<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>

                <tr>
                    <td>
                        <div class="TableTite">微信手机登录</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_rz" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>必须为已认证的服务号</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">微信PC扫码登录</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_author" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>必须为已认证的服务号</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">后台微信登录</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_author_htlogin" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>必须为已认证的服务号</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">会员中心二维码弹窗</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_popWin" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>必须为已认证的服务号</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">增加欢迎形式</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-radio-group v-model="configdata.wx_welcom_type">
                                <el-radio label="nowxcom">不增加</el-radio>
                                <el-radio label="wxcompic">图片</el-radio>
                            </el-radio-group>
                        </div>
                        <div v-show="configdata.wx_welcom_type == 'wxcompic'">
                            <div style="padding:10px;">
                                <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                           :show-file-list="false" :multiple="false" :data="uploadWxcompicData"
                                           :accept="pic_accept" :limit="1" :on-success="uploadWxcompicSuccess"
                                           list-type="picture">
                                    <div v-if="configdata.sy_wxcom_pic">
                                        <img :src="configdata.sy_wxcom_pic" width="120px" height="120px" class="avatar">
                                        <div slot="tip" class="el-upload__tip">点击上传图片</div>
                                    </div>
                                    <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                                </el-upload>
                            </div>
                            <el-alert title="图片大小900px * 500px" type="info" show-icon></el-alert>

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
                        <div class="TableTite">关注欢迎语</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input type="textarea" :rows="10" :maxlength="255"
                                      v-model="configdata.wx_welcom"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>在微信公众平台获取</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">搜索提示</div>
                    </td>
                    <td>
                        <el-input type="textarea" :rows="10" :maxlength="255" v-model="configdata.wx_search">
                        </el-input>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>自定义填写，需和微信公众平台一致</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">搜索无内容引导</div>
                    </td>
                    <td>
                        <el-input type="textarea" :rows="10" :maxlength="255" v-model="configdata.wx_search_no">
                        </el-input>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">公众号二维码</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                       :show-file-list="false" :multiple="false" :data="uploadWxqcodeData"
                                       :accept="pic_accept" :limit="1" :on-success="uploadWxqcodeSuccess"
                                       list-type="picture">
                                <div v-if="configdata.sy_wx_qcode">
                                    <img :src="configdata.sy_wx_qcode" width="110px" height="110px" class="avatar">
                                    <div slot="tip" class="el-upload__tip">点击上传图片</div>
                                </div>
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">

                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">微信封面</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                       :show-file-list="false" :multiple="false" :data="uploadWxlogoData"
                                       :accept="pic_accept" :limit="1" :on-success="uploadWxlogoSuccess"
                                       list-type="picture">
                                <div v-if="configdata.sy_wx_logo">
                                    <img :src="configdata.sy_wx_logo" width="110px" height="auto" class="avatar">
                                    <div slot="tip" class="el-upload__tip">点击上传图片</div>
                                </div>
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>图片大小900px * 500px</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">微信分享图片</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                       :show-file-list="false" :multiple="false" :data="uploadWxshareData"
                                       :accept="pic_accept" :limit="1" :on-success="uploadWxshareSuccess"
                                       list-type="picture">
                                <div v-if="configdata.sy_wx_sharelogo">
                                    <img :src="configdata.sy_wx_sharelogo" class="avatar">
                                    <div slot="tip" class="el-upload__tip">点击上传图片</div>
                                </div>
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>图片大小300px * 300px</span>
                        </div>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="post">提交</el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        config: {
            type: Object,
            default: {}
        },
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            configdata: {},

            uploadUrl: localStorage.getItem("baseUrl") + '?m=index&c=layui_upload',

            uploadWxcompicData: {
                name: 'sy_wxcom_pic',
                path: 'logo',
                imgid: 'imgwxcompic',
                pytoken: localStorage.getItem("pytoken")
            },

            uploadWxqcodeData: {
                name: 'sy_wx_qcode',
                path: 'logo',
                imgid: 'imgqcode',
                pytoken: localStorage.getItem("pytoken")
            },

            uploadWxlogoData: {
                name: 'sy_wx_logo',
                path: 'logo',
                imgid: 'imglogo',
                pytoken: localStorage.getItem("pytoken")
            },

            uploadWxshareData: {
                name: 'sy_wx_sharelogo',
                path: 'logo',
                imgid: 'imgshare',
                pytoken: localStorage.getItem("pytoken")
            },
        }
    },

    watch: {
        config: {
            handler(val, oldVal) {
                this.configdata = val;
            },
            immediate: true,
            deep: true,
        }
    },
    methods: {
        post: function () {
            this.$emit('post-set', {type: 'wxtyset', config: this.configdata})
        },
        uploadWxcompicSuccess: function (res, file) {
            if (res.code == 0) {
                this.config.sy_wxcom_pic = res.data.url
            } else {
                message.error(res.msg);
            }
        },
        uploadWxqcodeSuccess: function (res, file) {
            if (res.code == 0) {
                this.config.sy_wx_qcode = res.data.url
            } else {
                message.error(res.msg);
            }
        },
        uploadWxlogoSuccess: function (res, file) {
            if (res.code == 0) {
                this.config.sy_wx_logo = res.data.url
            } else {
                message.error(res.msg);
            }
        },
        uploadWxshareSuccess: function (res, file) {
            if (res.code == 0) {
                this.config.sy_wx_sharelogo = res.data.url
            } else {
                message.error(res.msg);
            }
        }
    },
};
</script>
<style scoped></style>