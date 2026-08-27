<template>
    <div class="drawerModlue">
        <div class="moduleTable" style="max-height: calc(100% - 80px);">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="400">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00192') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.nid" :placeholder="lc('wap_user_00100')">
                                <el-option v-for="titem in type" :key="titem.id" :label="titem.typename" :value="titem.id">
                                </el-option>
                            </el-select>

                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00954') }}</span>   <el-link icon="el-icon-edit" type="primary" @click="openType">{{ lc('admin_00197') }}</el-link>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00191') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('admin_00180')" v-model="ruleForm.name">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00955') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00354') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('admin_00963')" v-model="ruleForm.url">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_system_00354') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00956') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('admin_00964')" v-model="ruleForm.furl">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00957') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00193') }}</div>
                    </td>
                    <td>
                        <div class="TableSelect" style="display: flex;align-items: center;">
                            <el-select v-model="ruleForm.type" :placeholder="lc('wap_user_00100')">
                                <el-option key="1" :label="lc('admin_00198')" value="1"></el-option>
                                <el-option key="2" :label="lc('admin_system_00663')" value="2"></el-option>
                            </el-select>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00958') }}</span>
                        </div>
                    </td>
                </tr>

                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00105') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input :placeholder="lc('admin_00965')" v-model="ruleForm.sort">
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00959') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('member_com_00020') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.eject" label="1">{{ lc('admin_00205') }}</el-radio>
                            <el-radio v-model="ruleForm.eject" label="0">{{ lc('admin_00203') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('member_com_00020') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00960') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.model" label="hot">{{ lc('common_02091') }}</el-radio>
                            <el-radio v-model="ruleForm.model" label="new">{{ lc('common_02081') }}</el-radio>
                            <el-radio v-model="ruleForm.model" label="">{{ lc('common_02082') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('member_user_00181') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_system_00356') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.display" label="1">{{ lc('common.yes') }}</el-radio>
                            <el-radio v-model="ruleForm.display" label="0">{{ lc('common.no') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_system_00355') }}</span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00961') }}</div>
                    </td>
                    <td>
                        <div class="TableUpload">
                            <el-upload class="upload-demo" list-type="picture" action="" :accept="pic_accept" :auto-upload="false" :on-change="handleChange" :show-file-list="false">
                                <el-button size="small" type="primary">{{ lc('wap_js_00071') }}</el-button>
                                <div v-if="ruleForm.pic_n != ''" class="el-upload-list el-upload-list--picture">
                                    <img :src="ruleForm.pic_n" style="width: 100px; height: 100px">
                                </div>
                            </el-upload>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_00962') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>

        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="save">{{ lc('common.submit') }}</el-button>
        </div>
    </div>
</template>
<!-- script -->
<script>
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

    export default {
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
                    message.error(lc('admin_vue_00126') + this.picType + lc('common_02005') + '!');
                    return false;
                }
                //  判断图片大小
                let isLtNumM = file.size / 1024 / 1024 < this.picMaxSize;
                if (!isLtNumM) {
                    message.error(lc('admin_vue_00127') + this.picMaxSize + 'MB!');
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
                    message.warning(lc('admin_system_00352'));
                    return;
                }else{
                    formData.append('name', that.ruleForm.name);
                }
                if (typeof that.ruleForm.url == 'undefined' || that.ruleForm.url == '') {
                    message.warning(lc('admin_system_00286'));
                    return;
                }else{
                    formData.append('url', that.ruleForm.url);
                }
                if (typeof that.ruleForm.sort == 'undefined' || that.ruleForm.sort == '') {
                    message.warning(lc('admin_system_00353'));
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
