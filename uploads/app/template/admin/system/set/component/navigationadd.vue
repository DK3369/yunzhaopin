<template>
    <div class="drawerModlue">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">名称</th>
                    <th width="400">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">导航类别</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.nid" placeholder="请选择">
                                <el-option v-for="titem in type" :key="titem.id" :label="titem.typename" :value="titem.id">
                                </el-option>
                            </el-select>

                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>导航类别可自由修改可添加分类</span>   <el-link icon="el-icon-edit" type="primary" @click="openType">添加分类</el-link>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">导航名称</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入导航名称" v-model="ruleForm.name">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>导航是用户对网站核心栏目内容有清晰了解的指路牌</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">导航链接</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入导航链接" v-model="ruleForm.url">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>导航链接</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">伪静态链接</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="请输入伪静态链接" v-model="ruleForm.furl">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>伪静态链接是指在URL中包含了关键词或者其他词汇，看起来像静态链接</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">导航类型</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.type" placeholder="请选择">
                                <el-option key="1" label="站内链接" value="1"></el-option>
                                <el-option key="2" label="外部链接" value="2"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>站内链接如：job/ 外部链接如：http://www.phpyun.com/job/</span>
                        </div>
                    </td>
                </tr>

                <tr>
                    <td>
                        <div class="TableTite">排　　序</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input placeholder="排列顺序" v-model="ruleForm.sort">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>列表排序</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">弹出窗口</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.eject" label="1">新窗口</el-radio>
                            <el-radio v-model="ruleForm.eject" label="0">原窗口</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>弹出窗口</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">状　　态</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.model" label="hot">热</el-radio>
                            <el-radio v-model="ruleForm.model" label="new">新</el-radio>
                            <el-radio v-model="ruleForm.model" label="">无</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>状态</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">显　　示</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.display" label="1">是</el-radio>
                            <el-radio v-model="ruleForm.display" label="0">否</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>是否显示</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">导航图标</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="upload-demo" list-type="picture" action="" :accept="pic_accept" :auto-upload="false" :on-change="handleChange" :show-file-list="false">
                                <el-button slot="trigger" size="small" type="primary">点击上传</el-button>
                                <div v-if="ruleForm.pic_n != ''" class="el-upload-list el-upload-list--picture">
                                    <img :src="ruleForm.pic_n" style="width: 100px; height: 100px">
                                </div>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>仅限于wap端首页导航展示</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">提交</el-button>
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
                    message.error('上传头像图片只能是 （' + this.picType + '） 格式!');
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
                    message.warning('请填写导航名称');
                    return;
                }else{
                    formData.append('name', that.ruleForm.name);
                }
                if (typeof that.ruleForm.url == 'undefined' || that.ruleForm.url == '') {
                    message.warning('请填写链接地址');
                    return;
                }else{
                    formData.append('url', that.ruleForm.url);
                }
                if (typeof that.ruleForm.sort == 'undefined' || that.ruleForm.sort == '') {
                    message.warning('请填写导航排序');
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
