<template>
    <div class="drawerModlue">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="400">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00192'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.nid" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option v-for="titem in type" :key="titem.id" :label="titem.typename" :value="titem.id">
                                </el-option>
                            </el-select>

                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00954'{/yun}</span>   <el-link icon="el-icon-edit" type="primary" @click="openType">{yun:}t key='admin_00197'{/yun}</el-link>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00191'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00180'{/yun}" v-model="ruleForm.name">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00955'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_system_00354'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00963'{/yun}" v-model="ruleForm.url">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_system_00354'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00956'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00964'{/yun}" v-model="ruleForm.furl">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00957'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00193'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.type" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                <el-option key="1" label="站内链接" value="1"></el-option>
                                <el-option key="2" label="外部链接" value="2"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00958'{/yun}</span>
                        </div>
                    </td>
                </tr>

                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_system_00105'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="{yun:}t key='admin_00965'{/yun}" v-model="ruleForm.sort">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00959'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='member_com_00020'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.eject" label="1">{yun:}t key='admin_00205'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.eject" label="0">{yun:}t key='admin_00203'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='member_com_00020'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00960'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.model" label="hot">{yun:}t key='common_02091'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.model" label="new">{yun:}t key='common_02081'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.model" label="">{yun:}t key='common_02082'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='member_user_00181'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_system_00356'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.display" label="1">{yun:}t key='common.yes'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.display" label="0">{yun:}t key='common.no'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_system_00355'{/yun}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00961'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="upload-demo" list-type="picture" action="" :accept="pic_accept" :auto-upload="false" :on-change="handleChange" :show-file-list="false">
                                <el-button slot="trigger" size="small" type="primary">{yun:}t key='wap_js_00071'{/yun}</el-button>
                                <div v-if="ruleForm.pic_n != ''" class="el-upload-list el-upload-list--picture">
                                    <img :src="ruleForm.pic_n" style="width: 100px; height: 100px">
                                </div>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_00962'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
<!-- script -->
<script>
    module.exports = {
        props: ['id'],
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                type: [],
                ruleForm: {
                    type: '1',
                    eject: '0',
                    model: '',
                    display: '0',
                    pic: '',
                    pic_n: ''
                },
                files: [],
                picMaxSize: '5',
                picType: '',
                saveLoading: false,
            }
        },
        components: {
        },
        created: function () {
            this.getInfo();
        },
        methods: {
            getInfo() {
                let that = this;
                httpPost('m=system&c=set_navigation&a=add', {id: that.id ? that.id : ''}).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    that.type = data.type;
                    that.picMaxSize = data.picMaxSize;
                    that.picType = data.picType;
                    if (that.id) {
                        that.ruleForm = data.info;
                    }
                })
            },
            // 删除图片
            handleRemove(file) {
                this.ruleForm.pic = '';
            },
            // 上传时触发
            handleChange(file) {
                let that = this;
                //  判断图片类型
                console.log(that.picType);
                let picTypeArr = this.picType.split(',');
                let isImage = false;
                picTypeArr.forEach(item => {
                    if (file.raw.type === 'image/' + item) {
                        isImage = true;
                    }
                });
                if (!isImage) {
                    message.error('上传头像图片只能是 （' + this.picType + "） {yun:}t key='common_02005'{/yun}!");
                    return false;
                }
                //  判断图片大小
                let isLtNumM = file.size / 1024 / 1024 < this.picMaxSize;
                if (!isLtNumM) {
                    message.error('上传头像图片大小不能超过 ' + this.picMaxSize + 'MB!');
                    return false;
                }

                // 预览文件处理
                this.ruleForm.pic_n = URL.createObjectURL(file.raw);

                // 复刻文件信息
                this.files = file.raw;
            },

            save() {
                let that = this;
                let formData = new FormData();
                if (typeof that.ruleForm.name == 'undefined' || that.ruleForm.name == '') {
                    message.warning("{yun:}t key='admin_system_00352'{/yun}");
                    return;
                }else{
                    formData.append('name', that.ruleForm.name);
                }
                if (typeof that.ruleForm.url == 'undefined' || that.ruleForm.url == '') {
                    message.warning("{yun:}t key='admin_system_00286'{/yun}");
                    return;
                }else{
                    formData.append('url', that.ruleForm.url);
                }
                if (typeof that.ruleForm.sort == 'undefined' || that.ruleForm.sort == '') {
                    message.warning("{yun:}t key='admin_system_00353'{/yun}");
                    return;
                }else{
                    formData.append('sort', that.ruleForm.sort);
                }
                formData.append('nid', that.ruleForm.nid);
                formData.append('furl', that.ruleForm.furl);
                formData.append('type', that.ruleForm.type);
                formData.append('eject', that.ruleForm.eject);
                formData.append('model', that.ruleForm.model);
                formData.append('display', that.ruleForm.display);

                if (this.files.length !== 0) {

                    formData.append('file', this.files);
                }
                if (that.id > 0) {

                    formData.append('id', this.id);
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=system&c=set_navigation&a=save', formData).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.$emit("child-event");
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
            openType() {
                custoapp.drawerAdd = false;
                custoapp.drawerType = true;
            },
        },
        watch: {
            id: function (val, oldVal) {
                this.ruleForm = {
                    type: '1',
                    eject: '0',
                    model: '',
                    display: '0',
                    pic: '',
                    pic_n: ''
                };
                this.getInfo();
            },
        }
    };
</script>
