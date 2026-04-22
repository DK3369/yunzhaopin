<template>
    <div class="drawerModlue" v-loading="addloading">
        <!--运营-广告-广告管理 添加/修改-->
        <div class="drawerModInfo" style="height: calc(100% - 80px); overflow-y: auto;">
            <div class="adminBoldTips guangaoBanner">
                {{ textAddEdit }}广告时，请正确选择分类和类型。广告分类由：“分站、主站”和广告形式（联盟广告、图片）等个性化设置。
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告名称：</span>
                </div>
                <div class="drawerModInpt" style="display: flex; align-items: center;">
                    <el-input v-model="ruleForm.ad_name" placeholder="请输入类别名称"></el-input>
                    <el-checkbox v-model="ruleForm.targetChecked" label="新窗口打开" @change="handleTarget"
                        style="padding-left: 20px;"></el-checkbox>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>使用范围：</span>
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
                    <span>广告所属分类：</span>
                </div>
                <div class="drawerModInpt">
                    <el-cascader v-model="ruleForm.class_id" :options="classData" :props="{ emitPath: false }"
                        :show-all-levels="false" placeholder="" clearable style="width: 100%;"></el-cascader>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告是否启用：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.is_open" label="1">启用</el-radio>
                    <el-radio v-model="ruleForm.is_open" label="0">关闭</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告有效期：</span>
                </div>
                <div class="drawerModInpt">
                    <el-date-picker v-model="ruleForm.ad_time" type="daterange" range-separator="至" start-placeholder="开始日期"
                        end-placeholder="结束日期" value-format="yyyy-MM-dd">
                    </el-date-picker>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>备注：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" v-model="ruleForm.remark"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>排序：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="请输入内容"
                        onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="越大越在前" type="info" show-icon :closable="false"></el-alert>
                </div>
            </div>
            
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>广告类型：</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.ad_type" label="word">文字广告</el-radio>
                    <el-radio v-model="ruleForm.ad_type" label="pic">图片广告</el-radio>
                    <el-radio v-model="ruleForm.ad_type" label="lianmeng">联盟广告</el-radio>
                </div>
            </div>

            <div v-if="ruleForm.ad_type == 'word'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>文字信息：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.word_info" placeholder="请输入内容"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>文字链接：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.word_url" placeholder="请输入内容"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="外部链接请加上“http://”" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>

            <div v-if="ruleForm.ad_type == 'pic'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>图片地址：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="ruleForm.upload" label="upload">远程地址</el-radio>
                        <el-radio v-model="ruleForm.upload" label="upload_pic">本地上传</el-radio>
                    </div>
                </div>
                <div v-if="ruleForm.upload == 'upload'" class="drawerModLis">
                    <div class="drawerModTite">

                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_url_n" placeholder="请输入远程地址"></el-input>
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
                            <el-button size="small" type="primary">上传图片</el-button>
                        </el-upload>
                        <div class="up_sy_logo_div" style="margin-left: 15px;">
                            <el-image v-if="ruleForm.pic_upload_n" style="width:100px;" :src="ruleForm.pic_upload_n"
                                :preview-src-list="ruleForm.pic_upload_n ? [ruleForm.pic_upload_n] : []"></el-image>
                        </div>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>图片链接：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_src" placeholder=""></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="外部链接请加上“http://”" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>图片描述：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_content" placeholder=""></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="例如：公司名称或图片介绍，可留空" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>图片宽度：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_width" placeholder=""
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                            <template slot="append">px(像素)</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>图片高度：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_height" placeholder=""
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                            <template slot="append">px(像素)</template>
                        </el-input>
                    </div>
                </div>
            </div>

            
            <div v-if="ruleForm.ad_type == 'lianmeng'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>联盟广告代码：</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="textarea" :rows="4" v-model="ruleForm.lianmeng_url"></el-input>
                    </div>
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
        id: Number,
        classData: Array,/*广告分类*/
        domainData: Array,/*站点*/
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            textAddEdit: '添加',
            appad: 0,
            ruleForm: {
                id: 0,
                ad_name: '',//广告名称
                target: '1',//2 新窗口打开
                targetChecked: false,
                did: '0',//使用范围
                class_id: '',//广告分类
                is_open: null,//广告是否启用 1启用 0关闭
                ad_time: null,//广告有效期
                remark: '',
                sort: null,//排序
                appurl: '',//移动端跳转链接
                ad_type: null,//广告类型
                word_info: '',//文字信息
                word_url: '',//文字链接
                upload: 'upload',//图片地址
                pic_url: '',//图片远程地址
                pic_url_n: '',
                pic_upload_n: '',
                pic_src: '',//图片链接
                pic_content: '',//图片描述
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
                    message.error('暂无数据');
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            if (params.ad_name == '') {
                message.error('广告名称不能为空！');
                return false;
            }
            if ((!Array.isArray(params.ad_time)) || (Array.isArray(params.ad_time) && params.ad_time.length < 1)) {
                message.error('请填写广告有效期！');
                return false;
            }
            if (!params.ad_type) {
                message.error('请选择一种广告类型！');
                return false;
            } else {
                if (params.ad_type == "word" && params.word_info.trim() === '') {
                    message.error('请填写文字信息！');
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
                    this.textAddEdit = '修改';
                    
                } else {
                    this.textAddEdit = '添加';
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