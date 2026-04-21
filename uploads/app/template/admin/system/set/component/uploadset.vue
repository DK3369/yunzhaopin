<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">名称</th>
                        <th width="240">状态</th>
                        <th>说明</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">上传图片大小</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.pic_maxsize" @input="inputIntNumber($event, 'list', 'pic_maxsize')" placeholder="请输入内容">
                                    <span slot="suffix" class="slotspan">M</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>允许上传文件的最大限制，单位为 M，不填则默认5M</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">上传文件大小</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.file_maxsize" @input="inputIntNumber($event, 'list', 'file_maxsize')" placeholder="请输入内容">
                                    <span slot="suffix" class="slotspan">M</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>允许上传文件的最大限制，单位为 M，不填则默认5M</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">上传图片类型</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.pic_type" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>允许上传图片的类型，多个类型以英文逗号（,）分隔，不填则默认jpg,png,jpeg,bmp,gif</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">上传文件类型</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.file_type" placeholder="请输入内容"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>允许上传文件的类型，多个类型以英文逗号（,）分隔，不填则默认rar,zip,doc,docx,xls</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">图片安全验证</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.is_picself">
                                    <el-radio  label="1">开启</el-radio>
                                    <el-radio  label="2">关闭</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>对图片源码进行扫描，检测是否包含非法代码</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">强制压缩图片</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.is_picthumb">
                                    <el-radio label="1">开启</el-radio>
                                    <el-radio label="2">关闭</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>根据图片原始比例重新压缩成新图片，彻底去除图片中可能包含的非法代码，但有可能会影响图片清晰度</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">自动水印</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.is_watermark">
                                    <el-radio  label="1">开启</el-radio>
                                    <el-radio  label="2">关闭</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>设置上传图片是否自动添加水印</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">水印位置</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="list.wmark_position" placeholder="请选择">
                                    <el-option v-for="item in options" :key="item.value" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>水印在上传图片中的位置</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">水印LOGO</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <div class="TableUpload">
                                    <el-upload class="upload-demo" :action="action"
                                        :data="{name: 'sy_watermark',imgid: 'watermark',path:'logo',imgid:'imgwm',pytoken:pytoken}"
                                        :file-list="fileList" :limit="1"
                                        :on-success="handleAvatarSuccess"
                                        list-type="picture">
                                        <el-button size="small" type="primary">点击上传</el-button>
                                        <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                    </el-upload>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>水印在上传图片中的位置</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="save" :disabled="saveLoading">提交</el-button>
        </div>
    </div>
</template>
    
<script>
module.exports = {
    props:{
        list:Object
    },
    data: function () {
        return {
            input: '',
            value1: true,
            value: '',
            textarea: '',
            radio: '1',
            weblogo:'',
            upLogoUrl:'',
            action: baseUrl+'m=index&c=layui_upload',
            pytoken:localStorage.getItem("pytoken"),
            options: [{
                value: '1',
                label: '左上'
            }, {
                value: '3',
                label: '右上'
            }, {
                value: '5',
                label: '居中'
            }, {
                value: '7',
                label: '左下'
            }, {
                value: '9',
                label: '右下'
            }],
            uri:"m=system&c=",
            files: [],
            fileList: [],
            status :true,
            saveLoading: false

        }
    },

    mounted() {

    },
    methods: {
        inputIntNumber(val, form, key) {
            this.$props[form][key] = val.replace(/[^0-9]/g,'');
        },
        save:function (){
            let _this = this;
            let url = this.uri+"set_config&a=save";
            let ruleForm = {
                config: 'uploadconfig',
                pic_maxsize: _this.list.pic_maxsize,
                file_maxsize: _this.list.file_maxsize,
                pic_type: _this.list.pic_type,
                file_type: _this.list.file_type,
                is_picself: _this.list.is_picself =="1" ? 1 : 2,
                is_picthumb: _this.list.is_picthumb=="1" ? 1 : 2,
                is_watermark: _this.list.is_watermark=="1" ? 1 : 2,
                wmark_position: _this.list.wmark_position,
            };
            _this.saveLoading = true;
            httpPost(url, ruleForm).then(function (response) {
                var res = response.data;
                if (res.error == 0) {
                    // 修改图片类型缓存
                    let pic_accept = '.' + _this.list.pic_type;
                    localStorage.setItem("pic_accept", pic_accept ? pic_accept.split(',').join(',.') : '.jpg,.png,.jpeg,.bmp,.gif');
                    message.success('操作成功');
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
        upLogoChange(file) {
            // 预览文件处理
            this.weblogo = URL.createObjectURL(file.raw);
            console.log('LOGO预览地址：' + this.weblogo);
            // 复刻文件信息
            this.files = file.raw;
        },
        handleAvatarSuccess(res, file){
            if (res.error == 0) {
                this.identity_card_img = res.data.pictures;
                this.disabled = true;
            }
        }
    },
    watch: {
        list:{
            handler(newValue, oldValue) {
                if(newValue.sy_ossurl && newValue.sy_watermark){
                    // {yun:}$config.sy_ossurl{/yun}/{yun:}$config.sy_watermark{/yun}
                    if (this.status){
                        this.fileList =[{
                            name: 'test.jpeg',
                            url: newValue.sy_ossurl+"/"+newValue.sy_watermark
                        }] ;
                        this.status = false;
                    }

                }
            },
            deep:true,
            immediate:true
        }
    }
};
</script>
<style scoped></style>