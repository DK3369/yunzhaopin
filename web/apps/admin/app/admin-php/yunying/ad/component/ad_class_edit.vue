<template>
    <div class="drawerModlue">
        <!--广告类别 添加/修改-->
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00260') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.class_name" :placeholder="lc('admin_00217')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('member_user_00299') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.orders" :placeholder="lc('wap_user_00076')" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01130') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.place" label="1">PC</el-radio>
                    <el-radio v-model="ruleForm.place" label="2">WAP</el-radio>
                </div>
            </div>
            <div v-if="!changeToBuy" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01131') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.type" label="1">{{ lc('admin_yunying_00070') }}</el-radio>
                    <el-radio v-model="ruleForm.type" label="2">{{ lc('admin_yunying_00068') }}</el-radio>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01132') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.btype" label="1">{{ this.integral_pricename }}</el-radio>
                    <el-radio v-model="ruleForm.btype" label="2">{{ lc('member_user_00254') }}</el-radio>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_yunying_00067') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.integral_buy" @keyup="handleKeyupIntegral" :placeholder="lc('admin_yunying_00062')">
                        <template #suffix>
                            <span v-if="ruleForm.btype == '1'" class="slotspan">{{ lc('admin_yunying_00069') }}</span>
                            <span v-else-if="ruleForm.btype == '2'" class="slotspan">{{ lc('wap_com_00295') }}</span>
                        </template>
                    </el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_yunying_00064') }}</span>
                </div>
                <div class="drawerModInpt" style="display: flex;align-items: center;">
                    <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChange"
                        :show-file-list="false">
                        <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                    </el-upload>
                    <div class="up_sy_logo_div" style="margin-left: 15px;">
                        <el-image v-if="ruleForm.hrefn" style="width:100px;" :src="ruleForm.hrefn" :preview-src-list="ruleForm.hrefn ? [ruleForm.hrefn] : []"></el-image>
                    </div>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_yunying_00065') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.x" :placeholder="lc('admin_yunying_00061')">{{ lc('admin_yunying_00063') }}</el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_yunying_00066') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.y" :placeholder="lc('admin_yunying_00061')">{{ lc('admin_yunying_00063') }}</el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('member_com_00404') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" v-model="ruleForm.remark"></el-input>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.save') }}</el-button>
        </div>
    </div>
</template>

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
    props: {
        info: Object,
        integral_pricename: String,
        pic_maxsize: {type: [String, Number], default: ""},
        pic_type: {type: String, default: ""},
        changeToBuy: Boolean
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            ruleForm: {
                id: 0,
                class_name: '',
                orders: '',
                href: '',//{{ lc('wap_js_00081') }}
                hrefn: '',
                place: '',//{{ lc('admin_01174') }}
                type: "",//{{ lc('admin_01168') }}
                btype: '',//消费类型
                integral_buy: '',
                x: '',
                y: '',
                remark: '',
            },
            accept: '',
            file: [],//暂存文件
            submitLoading: false,
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    mounted() {
        this.handleUploadAccept();
    },
    methods: {
        handleUploadAccept() {
            let pic_type_temp = [];
            if (this.pic_type) {
                this.pic_type.split(",").forEach((item) => {
                    pic_type_temp.push("." + item);
                });
            }
            this.accept = pic_type_temp.join(",");
        },
        uploadChange(file) {
            if (file.status !== 'success') return;
            if (!this.checkFile(file)) return;
            this.ruleForm.hrefn = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.file = file.raw;
        },
        checkFile(file) {
            //  判断图片类型
            if (this.pic_type) {
                let picTypeArr = this.pic_type.split(',');
                let isImage = false;
                picTypeArr.forEach(item => {
                    if (file.raw.type === 'image/' + item) {
                        isImage = true;
                    }
                });
                if (!isImage) {
                    message.error(lc('admin_yunying_00058') + this.pic_type + "） {{ lc('common_02005') }}!");
                    return false;
                }
            }
            //  判断图片大小
            if (this.pic_maxsize > 0) {
                let isLtNumM = file.size / 1024 / 1024 < this.pic_maxsize;
                if (!isLtNumM) {
                    message.error("{{ lc('admin_yunying_00057') }} " + this.pic_maxsize + 'MB!');
                    return false;
                }
            }
            return true;
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));

            if (params.class_name == '') {
                message.error(lc('admin_01413'));
                return false;
            }
            if (params.place == '') {
                message.error(lc('admin_vue_00088'));
                return false;
            }
            if (params.type != '1' && params.type != '2') {
                message.error(lc('admin_vue_00089'));
                return false;
            }
            if (params.type == '1') {
                if (params.btype != '1' && params.btype != '2') {
                    message.error(lc('admin_vue_00090'));
                    return false;
                } else {
                    if (params.btype == '1' && params.integral_buy == '') {
                        message.error(lc('admin_vue_00129') + this.integral_pricename + '！');
                        return false;
                    } else if (params.btype == '2' && params.integral_buy == '') {
                        message.error(lc('admin_vue_00091'));
                        return false;
                    }
                }
                if (params.x == '') {
                    message.error(lc('admin_yunying_00059'));
                    return false;
                }
                if (params.y == '') {
                    message.error(lc('admin_yunying_00060'));
                    return false;
                }
            }

            delete params.href;
            delete params.hrefn;
            let formData = new FormData();
            Object.keys(params).forEach((key) => {
                if (Array.isArray(params[key])) {
                    params[key].forEach((v) => {
                        formData.append(key + '[]', v);
                    });
                } else {
                    formData.append(key, params[key]);
                }
            });
            if (this.file.length !== 0) {
                formData.append('file', this.file);
            }
            _this.submitLoading = true;
            httpPost('m=yunying&c=ad_class&a=addclass', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.clearForm();
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
        clearForm() {
            this.ruleForm.id = 0;
            this.ruleForm.class_name = '';
            this.ruleForm.orders = '';
            this.ruleForm.href = '';
            this.ruleForm.place = '';
            this.ruleForm.type = '';
            this.ruleForm.btype = '';
            this.ruleForm.integral_buy = '';
            this.ruleForm.x = '';
            this.ruleForm.y = '';
        },
        handleKeyupIntegral() {
            this.ruleForm.integral_buy = this.ruleForm.integral_buy.replace(/\D+/g, '')
        }
    },
    watch: {
        info: {
            handler: function (newValue, oldValue) {
                console.log('ad_class_edit watch', newValue);
                if (newValue && newValue.id) {
                    this.ruleForm = JSON.parse(JSON.stringify(newValue));
                }
            },
            deep: true,
            immediate: true
        },
        changeToBuy: {
            handler: function (newValue, oldValue) {
                console.log(newValue)
                if (newValue) {
                    this.ruleForm.type = '1';
                }
            },
            deep: true,
            immediate: true
        }
    }
}
</script>

<style scoped>
.dialog_item {
    margin-top: 25px;
    display: flex;
}

.item_span {
    width: 75px;
    text-align: right;
    display: block;
}

.dialog-footer {
    padding: 30px 0 0;
    text-align: right;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
}
</style>