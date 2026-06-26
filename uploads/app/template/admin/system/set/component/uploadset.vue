<template>
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                    <tr align="left">
                        <th width="180">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width="240">{yun:}t key='member_user_00181'{/yun}</th>
                        <th>{yun:}t key='member_com_00207'{/yun}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00382'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.pic_maxsize" @input="inputIntNumber($event, 'list', 'pic_maxsize')" placeholder="{yun:}t key='wap_user_00076'{/yun}">
                                    <span slot="suffix" class="slotspan">M</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00983'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00384'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.file_maxsize" @input="inputIntNumber($event, 'list', 'file_maxsize')" placeholder="{yun:}t key='wap_user_00076'{/yun}">
                                    <span slot="suffix" class="slotspan">M</span>
                                </el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00983'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00383'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.pic_type" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00984'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00385'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="list.file_type" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00985'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00388'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.is_picself">
                                    <el-radio  label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                    <el-radio  label="2">{yun:}t key='common.close'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00986'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00389'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.is_picthumb">
                                    <el-radio label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                    <el-radio label="2">{yun:}t key='common.close'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00987'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00392'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableButn">
                                <el-radio-group v-model="list.is_watermark">
                                    <el-radio  label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                    <el-radio  label="2">{yun:}t key='common.close'{/yun}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_00988'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00391'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="list.wmark_position" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="item in options" :key="item.value" :label="item.label"
                                        :value="item.value">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00381'{/yun}</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_system_00390'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableUpload">
                                <div class="TableUpload">
                                    <el-upload class="upload-demo" :action="action"
                                        :data="{name: 'sy_watermark',imgid: 'watermark',path:'logo',imgid:'imgwm',pytoken:pytoken}"
                                        :file-list="fileList" :limit="1"
                                        :on-success="handleAvatarSuccess"
                                        list-type="picture">
                                        <el-button size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                        <!-- <div slot="tip" class="el-upload__tip">只能上传jpg/png文件，且不超过500kb</div> -->
                                    </el-upload>
                                </div>
                            </div>
                        </td>
                        <td>
                            <div class="TableShuom">
                                <span>{yun:}t key='admin_system_00381'{/yun}</span>
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
                label: "{yun:}t key='admin_00989'{/yun}"
            }, {
                value: '3',
                label: "{yun:}t key='admin_00207'{/yun}"
            }, {
                value: '5',
                label: "{yun:}t key='admin_00990'{/yun}"
            }, {
                value: '7',
                label: "{yun:}t key='admin_00991'{/yun}"
            }, {
                value: '9',
                label: "{yun:}t key='admin_00992'{/yun}"
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