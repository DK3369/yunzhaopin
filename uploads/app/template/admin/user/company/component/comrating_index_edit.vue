<template>
    <!--会员-企业-套餐服务：套餐设置-设置会员套餐-->
    <div v-loading="loading" class="drawerModlue">
        <div class="drawerModInfo drawerModInfoOne">
            <div class="adminBoldTips">
                <el-alert title="{yun:}t key='admin_user_company_00179'{/yun}" show-icon type="success"></el-alert>
                
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00193'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.type" label="1">{yun:}t key='admin_user_company_00198'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.type" label="2">{yun:}t key='admin_user_company_00199'{/yun}</el-radio>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00177'{/yun}"
                              type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00191'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.name" style="margin-right: 8px"></el-input>
                    <el-checkbox v-model="ruleForm.youhuiBool" label="{yun:}t key='admin_user_company_00190'{/yun}" border size="medium"></el-checkbox>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00021'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_price" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02056'{/yun}</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00022'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_time" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00693'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.sort" placeholder="{yun:}t key='admin_user_00342'{/yun}"></el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00197'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
			
			<div class="drawerModLis">
			    <div class="drawerModTite">
			        <span>{yun:}t key='admin_user_company_00189'{/yun}</span>
			    </div>
			    <div class="drawerModInpt">
			        <el-input v-model="ruleForm.max_time" placeholder="{yun:}t key='admin_user_00342'{/yun}">
			            <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
			        </el-input>
			    </div>
			    <div class="drawerModTips">
					<el-alert title="{yun:}t key='admin_user_company_00183'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                    <el-alert title="{yun:}t key='admin_user_company_00181'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
			    </div>
			</div>
			<div class="drawerModLis">
			    <div class="drawerModTite">
			        <span>{yun:}t key='admin_user_company_00125'{/yun}</span>
			    </div>
			    <div class="drawerModInpt">
			        <el-input v-model="ruleForm.suspend_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
			            <span slot="suffix" class="slotspan">{yun:}t key='common_02089'{/yun}</span>
			        </el-input>
			    </div>
			    <div class="drawerModTips">
					<el-alert title="{yun:}t key='admin_user_company_00185'{/yun}" type="info" show-icon :closable="false">
					</el-alert>
				</div>
			</div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00694'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-radio v-model="ruleForm.display" label="1">{yun:}t key='admin_user_company_00205'{/yun}</el-radio>
                    <el-radio v-model="ruleForm.display" label="0">{yun:}t key='admin_user_company_00203'{/yun}</el-radio>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00188'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00192'{/yun}</span>
                </div>
                <div class="drawerModInpt" style="display: flex;align-items: center;">
                    <el-upload ref="compic" :action="uploadAction" :on-change="uploadChange" :accept="pic_accept"
                               :show-file-list="false">
                        <el-button size="small" type="primary">{yun:}t key='wap_00540'{/yun}</el-button>
                    </el-upload>
                    <div class="up_sy_logo_div" style="margin-left: 15px; position: relative;">
                        <el-image v-if="ruleForm.com_pic" style="width:40px;height:40px;" :src="ruleForm.com_pic"
                                  :preview-src-list="ruleForm.com_pic ? [ruleForm.com_pic] : []"></el-image>
                    <span style=" display: block;width:150px;;position: absolute;left:60px;top:50%; margin-top: -10px;"><el-link type="danger" :underline="false" v-if="ruleForm.com_pic" @click="delpic"  icon="el-icon-delete">{yun:}t key='admin_user_company_00194'{/yun}</el-link></span>
                    </div>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00695'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.explains" type="textarea"
                              :autosize="{ minRows: 2, maxRows: 3}"></el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00202'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.integral_buy" placeholder="{yun:}t key='admin_user_00342'{/yun}" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')">
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00180'{/yun}" type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00196'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.service_discount" placeholder="{yun:}t key='admin_user_00342'{/yun}"><span slot="suffix"
                                                                                                 class="slotspan">%</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00176'{/yun}"
                              type="info" show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00195'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.resume" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='wap_00451'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.interview" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='resume_00029'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.breakjob_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='wap_com_00029'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02089'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.zph_num" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='wap_com_00039'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02089'{/yun}</span>
                    </el-input>
                    
                </div>
                <div class="drawerModTips">
                    <el-alert v-if="ruleForm.type == 2"
                              title="{yun:}t key='admin_user_company_00178'{/yun}" type="warning"
                              show-icon :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00696'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.job_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                    <el-alert title="{yun:}t key='admin_user_company_00182'{/yun}" type="info" show-icon
                              :closable="false">
                    </el-alert>
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>{yun:}t key='wap_com_00236'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.top_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='wap_user_00209'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.urgent_num" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='wap_com_00043'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.rec_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='wap_com_00041'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                    </el-input>
                </div>
                <div class="drawerModTips">
                </div>
            </div>
            <div class="drawerModLis drawerModInFlex">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_user_company_00201'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="ruleForm.freerefresh_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                        <template slot="prepend">{yun:}t key='wap_com_00029'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02089'{/yun}</span>
                    </el-input>
                    <el-input v-model="ruleForm.freelook_num" placeholder="{yun:}t key='admin_user_00342'{/yun}" style="margin-left: 10px;">
                        <template slot="prepend">{yun:}t key='admin_user_company_00200'{/yun}</template>
                        <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                    </el-input>
                    
                </div>
                <div class="drawerModTips">
                    <el-alert
                            title="{yun:}t key='admin_user_company_00175'{/yun}"
                            type="info" show-icon :closable="false"></el-alert>
                    <el-alert
                            title="{yun:}t key='admin_user_company_00174'{/yun}"
                            type="info" show-icon :closable="false"></el-alert>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}
            </el-button>
        </div>
    </div>
</template>

<script>
module.exports = {
    props: {
        id: {type: [Number, String], default: 0},
        config: {type: [Object], default: {}},
    },
    data: function () {
        return {
            pic_accept: localStorage.getItem("pic_accept"),
            loading: false,
            submitLoading: false,
            ruleForm: {
                id: this.id,
                category: 1,
                //会员模式
                type: null,
                //会员名称
                name: '",
                //{yun:}t key='admin_user_company_00190'{/yun}
                youhuiBool: false,
                //优惠价格
                yh_price: "",
                //优惠日期
                time: null,
                //{yun:}t key='admin_user_company_00021'{/yun}
                service_price: "",
                //{yun:}t key='admin_user_company_00022'{/yun}
                service_time: "",
                //{yun:}t key='admin_00693'{/yun}
                sort: "",
                //{yun:}t key='admin_00694'{/yun}
                display: null,
                //{yun:}t key='admin_user_company_00192'{/yun}
                com_pic: "",
                //{yun:}t key='admin_00695'{/yun}
                explains: "",
                //{yun:}t key='wap_01823'{/yun}
                integral_buy: "0",
                //{yun:}t key='admin_user_company_00196'{/yun}
                service_discount: "',
                // Download resume
                resume: '',
                // Interview invitation
                interview: '',
                // refreshJob
                breakjob_num: '",
                //{yun:}t key='wap_com_00039'{/yun}
                zph_num: "',
                //
                chat_num: '",
                //{yun:}t key='admin_00696'{/yun}
                job_num: "",
                //{yun:}t key='wap_user_00209'{/yun}
                top_num: "",
                //{yun:}t key='wap_com_00043'{/yun}
                urgent_num: "",
                //{yun:}t key='wap_com_00041'{/yun}
                rec_num: "',
                // refreshJob
                freerefresh_num: '",
                //{yun:}t key='admin_user_company_00200'{/yun}
                freelook_num: "',
                //主动聊天
                freeactchat_num: '',
                //回复聊天
                freerepchat_num: '",
				//{yun:}t key='admin_user_company_00125'{/yun}
				suspend_num: 0,
				//最长有效期天数
				max_time:0
            },
            file: [],//暂存文件
            haspic:false,
            uploadAction: baseUrl + "m=common&c=common_upload'
        }
    },
    created() {
        if (this.id > 0) {
            this.getInfo();
        }
    },
    methods: {
        delpic(){
            if(this.haspic){
                var params = {id:this.id};
                delConfirm(this, params, this.delpicPost,"{yun:}t key='wap_user_00001'{/yun}");
            }else if (this.file.length !== 0) {
                this.file = [];
                this.ruleForm.com_pic = '';
                this.$refs.compic.clearFiles();
            }
        },
        delpicPost(params) {
            let _this = this;
            httpPost('m=user&c=company_comrating&a=delpic', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success("{yun:}t key='admin_user_00187'{/yun}");
                    _this.ruleForm.com_pic = '';
                    _this.file = [];
                    _this.$refs.compic.clearFiles();
                } else {
                    message.error("{yun:}t key='admin_user_00186'{/yun}");
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        uploadChange(file) {
            this.ruleForm.com_pic = URL.createObjectURL(file.raw);
            // 复刻文件信息
            this.file = file.raw;
        },
        getInfo() {
            let _this = this;
            let params = {id: this.id};
            _this.loading = true;
            httpPost('m=user&c=company_comrating&a=rating', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let row = res.data;
                    if (row.yh_price < 1) {
                        _this.ruleForm.youhuiBool = false;
                    } else {
                        _this.ruleForm.youhuiBool = true;
                    }
                    Object.keys(_this.ruleForm).forEach((key) => {
                        if (row.hasOwnProperty(key)) {
                            _this.ruleForm[key] = row[key];
                        }
                    });
                    if(_this.ruleForm.com_pic!=''){
                        _this.haspic = true;
                    }
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.time = Array.isArray(params.time) ? params.time.join('~') : null;
            if (params.youhuiBool) {
                params.youhui = 1;
            } else {
                delete params.youhui
            }
            delete params.youhuiBool;
            delete params.com_pic;
            
            
            if (params.name == '') {
                message.error("{yun:}t key='admin_company_00011'{/yun}");
                return false;
            }
            if (parseFloat(params.service_price) < 0) {
                message.error("{yun:}t key='admin_company_00012'{/yun}");
                return false;
            }
            if (params.youhuiBool) {
                if (parseFloat(params.yh_price) <= 0 || parseFloat(params.yh_price) > parseFloat(params.service_price)) {
                    message.error("{yun:}t key='admin_user_company_00186'{/yun}");
                    return false;
                }
                if (!params.time) {
                    message.error("{yun:}t key='admin_user_company_00187'{/yun}");
                    return false;
                }
            }
			
			if(parseInt(params.max_time)>0 && parseInt(params.service_time) > parseInt(params.max_time)){
				message.error("{yun:}t key='admin_user_company_00184'{/yun}");
				return false;
			}
			
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
            httpPost('m=user&c=company_comrating&a=saveclass', formData).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.$emit("child-event-list");
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
    },
    watch: {
        "ruleForm.yh_price": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.yh_price = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.service_price": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.service_price = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.service_time": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.service_time = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sort": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.sort = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.integral_buy": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.integral_buy = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.service_discount": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    if (newValue < 0) {
                        newValue = 0;
                    } else if (newValue > 100) {
                        newValue = 100;
                    }
                    this.ruleForm.service_discount = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.resume": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.resume = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.interview": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.interview = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.breakjob_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.breakjob_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.zph_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.zph_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        
        
        "ruleForm.job_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    if (newValue < 0) {
                        newValue = 0;
                    }
                    this.ruleForm.job_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.top_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.top_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.urgent_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.urgent_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.rec_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.rec_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.freerefresh_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.freerefresh_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.freelook_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.freelook_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        
		"ruleForm.suspend_num": {
		    handler(newValue, oldValue) {
		        if (typeof (newValue) == 'string') {
		            newValue = newValue.replace(/[^0-9.]/g, '');
		            this.ruleForm.suspend_num = newValue;
		        }
		    },
		    deep: true,
		    immediate: true
		},
		"ruleForm.max_time": {
		    handler(newValue, oldValue) {
		        if (typeof (newValue) == 'string') {
		            newValue = newValue.replace(/[^0-9.]/g, '');
		            this.ruleForm.max_time = newValue;
		        }
		    },
		    deep: true,
		    immediate: true
		},
	},
};
</script>
<style scoped>
.mt-10 {
    margin-top: 10px;
}

.coupon-select {
    display: flex;
    align-items: center;
}

.title-inner {
    color: rgb(144, 147, 153);
    font-size: 14px;
    width: 97px;
    text-align: center;
    padding: 0 10px;
}
</style>