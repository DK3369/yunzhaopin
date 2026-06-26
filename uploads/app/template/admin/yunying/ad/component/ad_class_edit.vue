<template>
    <div class="drawerModlue">
        <!--广告类别 添加/修改-->
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00260'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.class_name" placeholder="{yun:}t key='admin_00217'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='member_user_00299'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.orders" placeholder="{yun:}t key='wap_user_00076'{/yun}" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01130'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.place" label="1">PC</el-radio>
                    <el-radio v-model="ruleForm.place" label="2">WAP</el-radio>
                </div>
            </div>
            <div v-if="!changeToBuy" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01131'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.type" label="1">{yun:}t key='admin_yunying_00070'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.type" label="2">{yun:}t key='admin_yunying_00068'{/yun}</el-radio>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01132'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.btype" label="1">{{ this.integral_pricename }}</el-radio>
                    <el-radio v-model="ruleForm.btype" label="2">{yun:}t key='member_user_00254'{/yun}</el-radio>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_yunying_00067'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.integral_buy" @keyup.native="handleKeyupIntegral" placeholder="{yun:}t key='admin_yunying_00062'{/yun}">
                        <span v-if="ruleForm.btype == '1'" slot="suffix" class="slotspan">{yun:}t key='admin_yunying_00069'{/yun}</span>
                        <span v-else-if="ruleForm.btype == '2'" slot="suffix" class="slotspan">{yun:}t key='wap_com_00295'{/yun}</span>
                    </el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_yunying_00064'{/yun}</span>
                </div>
                <div class="drawerModInpt" style="display: flex;align-items: center;">
                    <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChange"
                        :show-file-list="false">
                        <el-button size="small" type="primary">{yun:}t key='wap_00540'{/yun}</el-button>
                    </el-upload>
                    <div class="up_sy_logo_div" style="margin-left: 15px;">
                        <el-image v-if="ruleForm.hrefn" style="width:100px;" :src="ruleForm.hrefn" :preview-src-list="ruleForm.hrefn ? [ruleForm.hrefn] : []"></el-image>
                    </div>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_yunying_00065'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.x" placeholder="{yun:}t key='admin_yunying_00061'{/yun}">{yun:}t key='admin_yunying_00063'{/yun}</el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_yunying_00066'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.y" placeholder="{yun:}t key='admin_yunying_00061'{/yun}">{yun:}t key='admin_yunying_00063'{/yun}</el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='member_com_00404'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" v-model="ruleForm.remark"></el-input>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='common.save'{/yun}</el-button>
        </div>
    </div>
</template>

<script setup>
module.exports = {
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
                href: '",//{yun:}t key='wap_js_00081'{/yun}
                hrefn: "',
                place: '",//{yun:}t key='admin_01174'{/yun}
                type: "",//{yun:}t key='admin_01168'{/yun}
                btype: "',//消费类型
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
                    message.error("{yun:}t key='admin_yunying_00058'{/yun}" + this.pic_type + "） {yun:}t key='common_02005'{/yun}!");
                    return false;
                }
            }
            //  判断图片大小
            if (this.pic_maxsize > 0) {
                let isLtNumM = file.size / 1024 / 1024 < this.pic_maxsize;
                if (!isLtNumM) {
                    message.error("{yun:}t key='admin_yunying_00057'{/yun} " + this.pic_maxsize + 'MB!');
                    return false;
                }
            }
            return true;
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));

            if (params.class_name == '') {
                message.error("{yun:}t key='admin_01413'{/yun}");
                return false;
            }
            if (params.place == '') {
                message.error('请选择广告位置！');
                return false;
            }
            if (params.type != '1' && params.type != '2') {
                message.error('请选择广告类型！');
                return false;
            }
            if (params.type == '1') {
                if (params.btype != '1' && params.btype != '2') {
                    message.error('请选择消费模式！');
                    return false;
                } else {
                    if (params.btype == '1' && params.integral_buy == '') {
                        message.error('请输入购买' + this.integral_pricename + '！');
                        return false;
                    } else if (params.btype == '2' && params.integral_buy == '') {
                        message.error('请输入购买金额！');
                        return false;
                    }
                }
                if (params.x == '') {
                    message.error("{yun:}t key='admin_yunying_00059'{/yun}");
                    return false;
                }
                if (params.y == '') {
                    message.error("{yun:}t key='admin_yunying_00060'{/yun}");
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