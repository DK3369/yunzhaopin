<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>

                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00674'{/yun}</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_rz" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00675'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00671'{/yun}</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_author" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00675'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00667'{/yun}</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_author_htlogin" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00675'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00664'{/yun}</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-switch v-model="configdata.wx_popWin" active-value="1" inactive-value="0"></el-switch>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00675'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00670'{/yun}</div>
                    </td>
                    <td>
                        <div class="moduleHcKg">
                            <el-radio-group v-model="configdata.wx_welcom_type">
                                <el-radio label="nowxcom">{yun:}t key='admin_tool_00663'{/yun}</el-radio>
                                <el-radio label="wxcompic">{yun:}t key='wap_js_00081'{/yun}</el-radio>
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
                                        <div slot="tip" class="el-upload__tip">{yun:}t key='admin_tool_00677'{/yun}</div>
                                    </div>
                                    <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                                </el-upload>
                            </div>
                            <el-alert title="{yun:}t key='admin_tool_00669'{/yun}" type="info" show-icon></el-alert>

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
                        <div class="TableTite">{yun:}t key='admin_tool_00666'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input type="textarea" :rows="10" :maxlength="255"
                                      v-model="configdata.wx_welcom"></el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00637'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00662'{/yun}</div>
                    </td>
                    <td>
                        <el-input type="textarea" :rows="10" :maxlength="255" v-model="configdata.wx_search">
                        </el-input>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00638'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00676'{/yun}</div>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00665'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                       :show-file-list="false" :multiple="false" :data="uploadWxqcodeData"
                                       :accept="pic_accept" :limit="1" :on-success="uploadWxqcodeSuccess"
                                       list-type="picture">
                                <div v-if="configdata.sy_wx_qcode">
                                    <img :src="configdata.sy_wx_qcode" width="110px" height="110px" class="avatar">
                                    <div slot="tip" class="el-upload__tip">{yun:}t key='admin_tool_00677'{/yun}</div>
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
                        <div class="TableTite">{yun:}t key='admin_tool_00673'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                       :show-file-list="false" :multiple="false" :data="uploadWxlogoData"
                                       :accept="pic_accept" :limit="1" :on-success="uploadWxlogoSuccess"
                                       list-type="picture">
                                <div v-if="configdata.sy_wx_logo">
                                    <img :src="configdata.sy_wx_logo" width="110px" height="auto" class="avatar">
                                    <div slot="tip" class="el-upload__tip">{yun:}t key='admin_tool_00677'{/yun}</div>
                                </div>
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00669'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_tool_00672'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="avatar-uploader" :action="uploadUrl" :auto-upload="true"
                                       :show-file-list="false" :multiple="false" :data="uploadWxshareData"
                                       :accept="pic_accept" :limit="1" :on-success="uploadWxshareSuccess"
                                       list-type="picture">
                                <div v-if="configdata.sy_wx_sharelogo">
                                    <img :src="configdata.sy_wx_sharelogo" class="avatar">
                                    <div slot="tip" class="el-upload__tip">{yun:}t key='admin_tool_00677'{/yun}</div>
                                </div>
                                <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_tool_00668'{/yun}</span>
                        </div>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="post">{yun:}t key='common.submit'{/yun}</el-button>
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