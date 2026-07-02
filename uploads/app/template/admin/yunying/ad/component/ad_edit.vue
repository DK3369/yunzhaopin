<template>
    <div class="drawerModlue" v-loading="addloading">
        <!-- Operations > Ads > Ad Management: Add/Edit -->
        <div class="drawerModInfo" style="height: calc(100% - 80px); overflow-y: auto;">
            <div class="adminBoldTips guangaoBanner">
                {{ lc('admin_01133') }}
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01134') }}</span>
                </div>
                <div class="drawerModInpt" style="display: flex; align-items: center;">
                    <el-input v-model="ruleForm.ad_name" :placeholder="lc('admin_00217')"></el-input>
                    <el-checkbox v-model="ruleForm.targetChecked" :label="lc('admin_01152')" @change="handleTarget"
                        style="padding-left: 20px;"></el-checkbox>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01135') }}</span>
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
                    <span>{{ lc('admin_01136') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-cascader v-model="ruleForm.class_id" :options="classData" :props="{ emitPath: false }"
                        :show-all-levels="false" placeholder="" clearable style="width: 100%;"></el-cascader>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01137') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.is_open" label="1">{{ lc('admin_user_company_00205') }}</el-radio>
                    <el-radio v-model="ruleForm.is_open" label="0">{{ lc('common.close') }}</el-radio>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01138') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-date-picker v-model="ruleForm.ad_time" type="daterange" :range-separator="lc('admin_company_00019')" :start-placeholder="lc('admin_00343')"
                        :end-placeholder="lc('admin_00344')" value-format="yyyy-MM-dd">
                    </el-date-picker>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01139') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input type="textarea" :rows="2" v-model="ruleForm.remark"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00103') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" :placeholder="lc('wap_user_00076')"
                        onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert :title="lc('admin_01153')" type="info" show-icon :closable="false"></el-alert>
                </div>
            </div>
            
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01131') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.ad_type" label="word">{{ lc('admin_01140') }}</el-radio>
                    <el-radio v-model="ruleForm.ad_type" label="pic">{{ lc('admin_01141') }}</el-radio>
                    <el-radio v-model="ruleForm.ad_type" label="lianmeng">{{ lc('admin_yunying_00072') }}</el-radio>
                </div>
            </div>

            <div v-if="ruleForm.ad_type == 'word'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01142') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.word_info" :placeholder="lc('wap_user_00076')"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01143') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.word_url" :placeholder="lc('wap_user_00076')"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert :title="lc('admin_01154')" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
            </div>

            <div v-if="ruleForm.ad_type == 'pic'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01144') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="ruleForm.upload" label="upload">{{ lc('admin_01145') }}</el-radio>
                        <el-radio v-model="ruleForm.upload" label="upload_pic">{{ lc('admin_01146') }}</el-radio>
                    </div>
                </div>
                <div v-if="ruleForm.upload == 'upload'" class="drawerModLis">
                    <div class="drawerModTite">

                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_url_n" :placeholder="lc('admin_01155')"></el-input>
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
                            <el-button size="small" type="primary">{{ lc('wap_00540') }}</el-button>
                        </el-upload>
                        <div class="up_sy_logo_div" style="margin-left: 15px;">
                            <el-image v-if="ruleForm.pic_upload_n" style="width:100px;" :src="ruleForm.pic_upload_n"
                                :preview-src-list="ruleForm.pic_upload_n ? [ruleForm.pic_upload_n] : []"></el-image>
                        </div>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01147') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_src" placeholder=""></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert :title="lc('admin_01154')" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01148') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_content" placeholder=""></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert :title="lc('admin_01156')" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01149') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_width" placeholder=""
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                            <template slot="append">{{ lc('admin_yunying_00063') }}</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01150') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.pic_height" placeholder=""
                            onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                            <template slot="append">{{ lc('admin_yunying_00063') }}</template>
                        </el-input>
                    </div>
                </div>
            </div>

            
            <div v-if="ruleForm.ad_type == 'lianmeng'">
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{{ lc('admin_01151') }}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="textarea" :rows="4" v-model="ruleForm.lianmeng_url"></el-input>
                    </div>
                </div>
            </div>

        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.save') }}</el-button>
        </div>
    </div>
</template>

<script setup>
module.exports = {
    props: {
        id: Number,
        classData: Array,/* Ad category */
        domainData: Array,/* Site */
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            textAddEdit: lc('wap_js_00091'),
            appad: 0,
            ruleForm: {
                id: 0,
                ad_name: '',//{{ lc('admin_01170') }}
                target: "1",//2 {{ lc('admin_01152') }}
                targetChecked: false,
                did: "0",//{{ lc('admin_user_00126') }}
                class_id: "",//{{ lc('admin_01167') }}
                is_open: null,// Whether the ad is enabled: 1 open, 0 closed
                ad_time: null,// Ad validity period
                remark: "",
                sort: null,//{{ lc('member_com_00022') }}
                appurl: "",// Mobile redirect link
                ad_type: null,//{{ lc('admin_01168') }}
                word_info: '',// Text info
                word_url: '',//{{ lc('admin_01013') }}
                upload: 'upload',// Image path
                pic_url: '',// Remote image URL
                pic_url_n: '',
                pic_upload_n: '',
                pic_src: '',//{{ lc('admin_00100') }}
                pic_content: '',// Image description
                pic_width: '',// Image width
                pic_height: '',// Image height
                lianmeng_url: '',// Ad network code
            },
            file_pic: [],// Temporary files
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
            // Copy file info
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
                    message.error(lc('wap_js_00113'));
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
                message.error(lc('admin_01414'));
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
                    this.textAddEdit = lc('wap_js_00073');
                    
                } else {
                    this.textAddEdit = lc('wap_js_00091');
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
    /* // Default 90px; */
}

.drawerModInpt {
    width: calc(100% - 130px);
    /* // Default 100px; */
}

.drawerModTips {
    padding-left: 130px;
    /* // Default 100px; */
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