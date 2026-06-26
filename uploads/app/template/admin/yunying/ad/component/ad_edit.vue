<template>
    <div class="drawerModlue" v-loading="addloading">
        <!--运营-广告-广告管理 添加/修改-->
        <div class="drawerModInfo" style="height: calc(100% - 80px); overflow-y: auto;">
            <div class="adminBoldTips guangaoBanner">
                {yun:}t key='admin_01133'{/yun}
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01134'{/yun}</span>
                </div>
                <div class="drawerModInpt" style="display: flex; align-items: center;">
                    <el-input v-model="ruleForm.ad_name" placeholder="{yun:}t key='admin_00217'{/yun}"></el-input>
                    <el-checkbox v-model="ruleForm.targetChecked" label="{yun:}t key='admin_01152'{/yun}" @change="handleTarget"
                        style="padding-left: 20px;"></el-checkbox>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01135'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="ruleForm.did" filterable placeholder="">
                        <el-option v-for="item in domainData" :key="item.value" :label="item.label" :value="item.value">
                        </el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01136'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-cascader v-model="ruleForm.class_id" :options="classData" :props="{ emitPath: false }"
                        :show-all-levels="false" placeholder="" clearable style="width: 100%;"></el-cascader>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01137'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.is_open" label="1">{yun:}t key='admin_user_company_00205'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.is_open" label="0">{yun:}t key='common.close'{/yun}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01138'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-date-picker v-model="ruleForm.ad_time" type="daterange" range-separator="{yun:}t key='admin_company_00019'{/yun}" start-placeholder="{yun:}t key='admin_00343'{/yun}"
                        end-placeholder="{yun:}t key='admin_00344'{/yun}" value-format="yyyy-MM-dd">
                    </el-date-picker>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01139'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" v-model="ruleForm.remark"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00103'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="{yun:}t key='wap_user_00076'{/yun}"
                        onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_01153'{/yun}" type="info" show-icon :closable="false"></el-alert>
                </div>
            </div>
            
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01131'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.ad_type" label="word">{yun:}t key='admin_01140'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.ad_type" label="pic">{yun:}t key='admin_01141'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.ad_type" label="lianmeng">{yun:}t key='admin_yunying_00072'{/yun}</el-radio>
                </div>
            </div>

            <div v-if="ruleForm.ad_type == 'word'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01142'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.word_info" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01143'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.word_url" placeholder="{yun:}t key='wap_user_00076'{/yun}"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_01154'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>

            <div v-if="ruleForm.ad_type == 'pic'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01144'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="ruleForm.upload" label="upload">{yun:}t key='admin_01145'{/yun}</el-radio>
                        <el-radio v-model="ruleForm.upload" label="upload_pic">{yun:}t key='admin_01146'{/yun}</el-radio>
                    </div>
                </div>
                <div v-if="ruleForm.upload == 'upload'" class="drawerModLis">
                    <div class="drawerModTite">

                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_url_n" placeholder="{yun:}t key='admin_01155'{/yun}"></el-input>
                        <div class="up_sy_logo_div">
                            <el-image v-if="ruleForm.pic_url_n" style="width:100px;" :src="ruleForm.pic_url_n"
                                :preview-src-list="ruleForm.pic_url_n ? [ruleForm.pic_url_n] : []"></el-image>
                        </div>
                    </div>
                </div>
                <div v-if="ruleForm.upload == 'upload_pic'" class="drawerModLis">
                    <div class="drawerModTite">

                    </div>
                    <div class="drawerModInpt" style="display: flex;align-items: center;">
                        <el-upload :accept="pic_accept" :action="uploadAction" :on-change="uploadChange"
                            :show-file-list="false">
                            <el-button size="small" type="primary">{yun:}t key='wap_00540'{/yun}</el-button>
                        </el-upload>
                        <div class="up_sy_logo_div" style="margin-left: 15px;">
                            <el-image v-if="ruleForm.pic_upload_n" style="width:100px;" :src="ruleForm.pic_upload_n"
                                :preview-src-list="ruleForm.pic_upload_n ? [ruleForm.pic_upload_n] : []"></el-image>
                        </div>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01147'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_src" placeholder=""></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_01154'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01148'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_content" placeholder=""></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_01156'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01149'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_width" placeholder=""
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                            <template slot="append">{yun:}t key='admin_yunying_00063'{/yun}</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01150'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_height" placeholder=""
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                            <template slot="append">{yun:}t key='admin_yunying_00063'{/yun}</template>
                        </el-input>
                    </div>
                </div>
            </div>

            
            <div v-if="ruleForm.ad_type == 'lianmeng'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_01151'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="textarea" :rows="4" v-model="ruleForm.lianmeng_url"></el-input>
                    </div>
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
        id: Number,
        classData: Array,/*广告分类*/
        domainData: Array,/*站点*/
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            textAddEdit: "{yun:}t key='wap_js_00091'{/yun}",
            appad: 0,
            ruleForm: {
                id: 0,
                ad_name: '",//{yun:}t key='admin_01170'{/yun}
                target: "1",//2 {yun:}t key='admin_01152'{/yun}
                targetChecked: false,
                did: "0",//{yun:}t key='admin_user_00126'{/yun}
                class_id: "",//{yun:}t key='admin_01167'{/yun}
                is_open: null,//广告是否启用 1{yun:}t key='admin_user_company_00205'{/yun} 0{yun:}t key='common.close'{/yun}
                ad_time: null,//广告有效期
                remark: "",
                sort: null,//{yun:}t key='member_com_00022'{/yun}
                appurl: "",//移动端跳转链接
                ad_type: null,//{yun:}t key='admin_01168'{/yun}
                word_info: "',//文字信息
                word_url: '",//{yun:}t key='admin_01013'{/yun}
                upload: "upload',//图片地址
                pic_url: '',//图片远程地址
                pic_url_n: '',
                pic_upload_n: '',
                pic_src: '",//{yun:}t key='admin_00100'{/yun}
                pic_content: "',//图片描述
                pic_width: '',//图片宽度
                pic_height: '',//图片高度
                lianmeng_url: '',//广告联盟代码
            },
            file_pic: [],//暂存文件
            addloading: false,
            submitLoading: false,
            uploadAction: baseUrl + 'm=common&c=common_upload'
        }
    },
    mounted() {
        console.log('ad_edit mounted');
    },
    methods: {
        uploadChange(file) {
            this.ruleForm.pic_upload_n = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.file_pic = file.raw;
        },
        handleTarget(val) {
            this.ruleForm.target = (val ? 2 : 1);
        },
        getInfo() {
            let _this = this;
            let params = { id: this.id };
            _this.addloading = true;
            httpPost('m=yunying&c=ad&a=info', params).then(function (response) {
                _this.addloading = false;
                let res = response.data;
                if (res.error === 0) {
                    _this.appad = res.data.appad;

                    let info = res.data.info;
                    for (let index in _this.ruleForm) {
                        if (info.hasOwnProperty(index)) {
                            _this.ruleForm[index] = info[index];
                        }
                    }
                    _this.ruleForm.targetChecked = (_this.ruleForm.target == '2' ? true : false);
                    _this.ruleForm.did = (info.did === '' ? '0' : info.did);
                    if (info.time_start && info.time_end) {
                        _this.ruleForm.ad_time = [info.time_start, info.time_end];
                    }
                } else {
                    message.error("{yun:}t key='wap_js_00113'{/yun}");
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            if (params.ad_name == '') {
                message.error(lc('admin_vue_00093'));
                return false;
            }
            if ((!Array.isArray(params.ad_time)) || (Array.isArray(params.ad_time) && params.ad_time.length < 1)) {
                message.error("{yun:}t key='admin_01414'{/yun}");
                return false;
            }
            if (!params.ad_type) {
                message.error(lc('admin_vue_00094'));
                return false;
            } else {
                if (params.ad_type == "word" && params.word_info.trim() === '') {
                    message.error(lc('admin_vue_00095'));
                    return false;
                }
            }

            delete params.pic_upload_n;
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
            if (params.ad_type == 'pic' && this.file_pic.length !== 0) {
                formData.append('file', this.file_pic);
            }
            _this.submitLoading = true;
            httpPost('m=yunying&c=ad&a=ad_saveadd', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                } else {
                    message.error(res.msg);
                }
                _this.$emit("child-event");
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        handleKeyupIntegral() {
            this.ruleForm.integral_buy = this.ruleForm.integral_buy.replace(/\D+/g, '')
        }
    },
    watch: {
        id: {
            handler: function (newValue, oldValue) {
                console.log('ad_edit watch', newValue);
                if (newValue) {
                    this.textAddEdit = "{yun:}t key='wap_js_00073'{/yun}";
                    
                } else {
                    this.textAddEdit = "{yun:}t key='wap_js_00091'{/yun}";
                }
                this.getInfo();
            },
            deep: true,
            immediate: true
        },
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

.drawerModTite {
    width: 120px;
    /* //默认90px; */
}

.drawerModInpt {
    width: calc(100% - 130px);
    /* //默认100px; */
}

.drawerModTips {
    padding-left: 130px;
    /* //默认100px; */
}

.guangaoBanner {
    overflow: hidden;
    position: relative;
    width: calc(100% - 24px);
    height: 34px;
    padding: 0 12px;
    background: #f0f9eb;
    color: #67c23a;
    display: flex;
    align-items: center;
    border-radius: 4px;
}</style>