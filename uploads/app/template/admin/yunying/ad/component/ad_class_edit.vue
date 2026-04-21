<template>
    <div class="drawerModlue">
        <!--广告类别 添加/修改-->
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>类别名称：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.class_name" placeholder="请输入类别名称"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>排　　序：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.orders" placeholder="请输入内容" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告位置：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.place" label="1">PC</el-radio>
                    <el-radio v-model="ruleForm.place" label="2">WAP</el-radio>
                </div>
            </div>
            <div v-if="!changeToBuy" class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告类型：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.type" label="1">可购买</el-radio>
                    <el-radio v-model="ruleForm.type" label="2">不可购买</el-radio>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>消费类型：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.btype" label="1">{{ this.integral_pricename }}</el-radio>
                    <el-radio v-model="ruleForm.btype" label="2">金额</el-radio>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>购买费用：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.integral_buy" @keyup.native="handleKeyupIntegral" placeholder="请输入购买费用">
                        <span v-if="ruleForm.btype == '1'" slot="suffix" class="slotspan">积分/月</span>
                        <span v-else-if="ruleForm.btype == '2'" slot="suffix" class="slotspan">元/月</span>
                    </el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>位置示意图：</span>
                </div>
                <div class="drawerModInpt" style="display: flex;align-items: center;">
                    <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChange"
                        :show-file-list="false">
                        <el-button size="small" type="primary">上传图片</el-button>
                    </el-upload>
                    <div class="up_sy_logo_div" style="margin-left: 15px;">
                        <el-image v-if="ruleForm.hrefn" style="width:100px;" :src="ruleForm.hrefn" :preview-src-list="ruleForm.hrefn ? [ruleForm.hrefn] : []"></el-image>
                    </div>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告宽度：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.x" placeholder="请输入广告宽度">px(像素)</el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告高度：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.y" placeholder="请输入广告宽度">px(像素)</el-input>
                </div>
            </div>
            <div v-show="ruleForm.type == 1" class="drawerModLis">
                <div class="drawerModTite">
                    <span>备注说明：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" v-model="ruleForm.remark"></el-input>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">保存</el-button>
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
                href: '',//图片
                hrefn: '',
                place: '',//广告位置
                type: '',//广告类型
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
                    message.error('上传图片只能是 （' + this.pic_type + '） 格式!');
                    return false;
                }
            }
            //  判断图片大小
            if (this.pic_maxsize > 0) {
                let isLtNumM = file.size / 1024 / 1024 < this.pic_maxsize;
                if (!isLtNumM) {
                    message.error('上传图片大小不能超过 ' + this.pic_maxsize + 'MB!');
                    return false;
                }
            }
            return true;
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));

            if (params.class_name == '') {
                message.error('广告类别名称不能为空！');
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
                    message.error('请填写广告宽度！');
                    return false;
                }
                if (params.y == '') {
                    message.error('请填写广告高度！');
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