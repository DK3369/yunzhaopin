<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="360">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00924'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_logo', imgid: 'imglogo', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_logo_list')"
                                           :file-list="sy_logo_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00925'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00926'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action" :file-list="sy_member_logo_list"
                                    :data="{ name: 'sy_member_logo', imgid: 'imgmember', path: 'logo', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_member_logo_list')"
                                           list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00928'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action" :file-list="sy_unit_logo_list"
                                    :data="{ name: 'sy_unit_logo', imgid: 'imgunit', path: 'logo', source: 'back',pytoken: pytoken }"
                                    :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_unit_logo_list')"
                                           list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00334'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_wap_logo', imgid: 'imgwaplogo', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_wap_logo_list')"
                                           :file-list="sy_wap_logo_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00929'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action" :file-list="sy_wap_qcode_list"
                                    :data="{ name: 'sy_wap_qcode', imgid: 'imgwapqcode', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_wap_qcode_list')"
                                           list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00930'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_androidu_qcode', imgid: 'imgandroid', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_androidu_qcode_list')"
                                           :file-list="sy_androidu_qcode_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00931'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action" :file-list="sy_iosu_qcode_list"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_iosu_qcode_list')"
                                           :data="{ name: 'sy_iosu_qcode', imgid: 'imgios', path: 'logo', source: 'back',pytoken: pytoken }"
                                    list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00932'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_zph_icon', imgid: 'imgzph', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_zph_icon_list')"
                                           :file-list="sy_zph_icon_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00927'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00933'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_zphbanner_icon', imgid: 'imgzphbanner', path: 'logo', source: 'back',pytoken: pytoken }"
                                           :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_zphbanner_icon_list')"
                                           :file-list="sy_zphbanner_icon_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00934'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00935'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_cplogo', imgid: 'cpimglogo', path: 'logo', source: 'back',pytoken: pytoken }"
                                    :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_cplogo_list')"
                                    :file-list="sy_cplogo_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00936'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_00937'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <el-upload class="upload-demo" :accept="pic_accept" :action="action"
                                    :data="{ name: 'sy_gongzhaologo', imgid: 'gongzhaologo', path: 'logo', source: 'back',pytoken: pytoken }"
                                    :on-success="(response, file, fileList) =>onSuccess(response,file,fileList,'sy_gongzhaologo_list')"
                                    :file-list="sy_gongzhaologo_list" list-type="picture">
                                    <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                    <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                </el-upload>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00938'{/yun}</span>
                            </div>
                        </td>
                    </tr>

                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props: {
        list: Object
    },

    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            fileList: [
                {
                    name: '',
                    url: ''
                },
            ],
            uri: "m=system&c=",
            action: baseUrl+'m=index&c=layui_upload',
            pytoken:localStorage.getItem("pytoken"),
            sy_ossurl: '',
            sy_gongzhaologo_list: [],
            // sy_logo_list:'',
            sy_logo_list: [],
            sy_member_logo_list: [],
            sy_unit_logo_list: [],
            sy_px_logo_list: [],
            sy_wap_logo_list: [],
            sy_wap_qcode_list: [],
            sy_androidu_qcode_list: [],
            sy_iosu_qcode_list: [],
            sy_zph_icon_list: [],
            sy_zphbanner_icon_list: [],
            sy_cplogo_list: [],
            fileArr: ['sy_logo', 'sy_member_logo', 'sy_unit_logo', 'sy_lt_logo', 'sy_ltmember_logo', 'sy_px_logo', 'sy_wap_logo', 'sy_wap_qcode', 'sy_androidu_qcode', 'sy_iosu_qcode', 'sy_zph_icon', 'sy_zphbanner_icon', 'sy_cplogo', 'sy_gongzhaologo'],
            saveLoading: false
        }
    },
    mounted() {
        // ProcessData
        this.handleImage();
    },
    methods: {
        onSuccess:function (res, file,fileList,arr){
            if (res.code == 0) {
				this[arr] = [{
                            name: arr,
                            url: res.data.url
                        }];
            } else {
                this.$message.error(res.msg)
            }
        },
        save: function () {
            let _this = this;
            let url = this.uri + "set_config&a=save_logo";
            let ruleForm = {
                waterconfig: 1
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    message.success("{yun:}t key='wap_user_00264'{/yun}");
                    _this.$emit('get-list', true)
                } else {
                    message.error(res.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    _this.saveLoading = false;
                }, 2000);
            });
        },
        handleImage() {
            this.sy_ossurl = this.list.sy_ossurl;
            for (var e in this.fileArr) {
                if (this['list'][this.fileArr[e]]) {
                        this[this.fileArr[e] + '_list'] = [{
                            name: 'test.jpeg',
                            url: this.sy_ossurl + "/" + this['list'][this.fileArr[e]]
                        }]
                }

            }
        }
    },
};
</script>
<style scoped>
.el-upload-list--picture .el-upload-list__item {
    height: auto;
}

.TableUpload .el-upload-list--picture .el-upload-list__item {
    padding: 6px;
}

.el-upload-list--picture .el-upload-list__item-thumbnail {
    height: 60px;
    width: auto;
}</style>