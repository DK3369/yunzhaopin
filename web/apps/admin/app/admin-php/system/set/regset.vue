<template>
<div id="regSetApp" class="tableDome">
    <div class="zhuceTable">
        <table class="tableVue">
            <thead>
            <tr align="left">
                <th width="180">{{ lc('member_com_00021') }}</th>
                <th width="560">{{ lc('member_user_00181') }}</th>
                <th>{{ lc('member_com_00207') }}</th>
            </tr>
            </thead>
            <tbody>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00566') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio v-model="config.reg_user_stop" label="1">{{ lc('member_com_00287') }}</el-radio>
                        <el-radio v-model="config.reg_user_stop" label="2">{{ lc('resume_00030') }}</el-radio>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00543') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00572') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-radio v-model="config.sy_reg_type" label="1">{{ lc('admin_system_00556') }}</el-radio>
                        <el-radio v-model="config.sy_reg_type" label="2">{{ lc('admin_system_00557') }}</el-radio>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00554') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00573') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-checkbox v-model="config.reg_moblie">{{ lc('default_00293') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_email">{{ lc('wap_00411') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_user">{{ lc('wap_00409') }}</el-checkbox>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span></span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00574') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-checkbox v-model="config.reg_passconfirm">{{ lc('admin_system_00571') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_real_name_check">{{ lc('member_com_00026') }}</el-checkbox>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span></span>
                    </div>
                </td>
            </tr>
            <tr v-if="config.reg_user==1">
                <td>
                    <div class="TableTite">{{ lc('wap_user_00366') }}</div>
                </td>
                <td>
                    <div class="youdandawei">
                        <div class="jianlidian">
                            <div class="TableInpt">
                                <el-input type="number" v-model="config.sy_reg_nameminlen" min="6"></el-input>
                            </div>
                            <div class="hxian"></div>
                            <div class="TableInpt">
                                <el-input type="number" v-model="config.sy_reg_namemaxlen" max="100"></el-input>
                            </div>
                        </div>
                        <div class="danwei">{{ lc('common_02053') }}</div>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span></span>
                    </div>
                </td>
            </tr>
            <tr v-if="config.reg_user==1">
                <td>
                    <div class="TableTite">{{ lc('admin_system_00564') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-checkbox v-model="config.reg_name_num">{{ lc('common_01997') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_name_zm">{{ lc('common_01973') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_name_sp">{{ lc('admin_01025') }}</el-checkbox>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00550') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00569') }}</div>
                </td>
                <td>
                    <div class="TableButn">
                        <el-checkbox v-model="config.reg_pw_num">{{ lc('common_01997') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_pw_zm">{{ lc('common_01973') }}</el-checkbox>
                        <el-checkbox v-model="config.reg_pw_sp">{{ lc('admin_01025') }}</el-checkbox>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00550') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00568') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input :placeholder="lc('wap_user_00076')" type="textarea" :rows="4" v-model="config.sy_def_email"></el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00539') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00558') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="config.sy_reg_interval" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('wap_js_00128') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00544') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00567') }}</div>
                </td>
                <td>
                    <div class="TableInpt">
                        <el-input v-model="config.sy_reg_invite" type="number" min="0">
                            <template #suffix><span class="slotspan">{{ lc('common_02089') }}</span></template>
                        </el-input>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00538') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00561') }}</div>
                </td>
                <td>
                    <div class="TableItags">
                        <el-tag :key="tag" v-for="tag in regconfig.regname" closable :disable-transitions="false" @close="handleClose(tag,'regname')">{{tag}}</el-tag>
                        <el-input class="input-new-tag" v-if="regnameinput" v-model="inputValue" ref="saveTagregname" size="small" @keyup.enter="$event.target.blur()" @blur="handleInputConfirm('regname')"></el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showInput('regname')">{{ lc('wap_js_00091') }}</el-button>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00540') }}<br>{{ lc('admin_system_00547') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00565') }}</div>
                </td>
                <td>
                    <div class="TableItags">
                        <el-tag :key="tag" v-for="tag in regconfig.mobile_number" closable :disable-transitions="false" @close="handleClose(tag,'mobile_number')">{{tag}}</el-tag>
                        <el-input type="number" class="input-new-tag" v-if="mobile_numberinput" v-model="inputValue" ref="saveTagmobile_number" size="small" @keyup.enter="$event.target.blur()" @blur="handleInputConfirm('mobile_number')"></el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showInput('mobile_number')">{{ lc('wap_js_00091') }}</el-button>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00540') }}<br>{{ lc('admin_system_00536') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00562') }}</div>
                </td>
                <td>
                    <div class="TableItags">
                        <el-tag :key="tag" v-for="tag in regconfig.mobile_white" closable :disable-transitions="false" @close="handleClose(tag,'mobile_white')">{{tag}}</el-tag>
                        <el-input class="input-new-tag" v-if="mobile_whiteinput" v-model="inputValue" ref="saveTagmobile_white" size="small" @keyup.enter="$event.target.blur()" @blur="handleInputConfirm('mobile_white')"></el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showInput('mobile_white')">{{ lc('wap_js_00091') }}</el-button>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00555') }}<br>{{ lc('admin_system_00541') }}</span>
                    </div>
                </td>
            </tr>
            <tr>
                <td>
                    <div class="TableTite">{{ lc('admin_system_00563') }}</div>
                </td>
                <td>
                    <div class="TableItags">
                        <el-tag :key="tag" v-for="tag in regconfig.mobile_black" closable :disable-transitions="false" @close="handleClose(tag,'mobile_black')">{{tag}}</el-tag>
                        <el-input class="input-new-tag" v-if="mobile_blackinput" v-model="inputValue" ref="saveTagmobile_black" size="small" @keyup.enter="$event.target.blur()" @blur="handleInputConfirm('mobile_black')"></el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showInput('mobile_black')">{{ lc('wap_js_00091') }}</el-button>
                    </div>
                </td>
                <td>
                    <div class="TableShuom">
                        <span>{{ lc('admin_system_00555') }}<br>{{ lc('admin_system_00545') }}</span>
                    </div>
                </td>
            </tr>
            </tbody>
        </table>
    </div>
    <div class="setBasicButn" style="border: none;">
        <el-button type="primary" size="medium" @click="submitForm" :disabled="saveLoading">{{ lc('wap_user_00176') }}</el-button>
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
        data: function () {
            return {
                config: {},
                regconfig: [],
                regnameinput: false,
                mobile_numberinput: false,
                mobile_whiteinput: false,
                mobile_blackinput: false,
                inputValue: '',
                saveLoading: false
            }
        },
        created() {
            this.getList();
        },
        methods: {
            async submitForm() {
                let that = this;
                let formData = {};
                if (that.config.reg_user_stop == '') {
                    that.config.reg_user_stop = 1;
                }
                if (that.config.sy_reg_type == '') {
                    that.config.sy_reg_type = 1;
                }
                formData['reg_user_stop'] = that.config.reg_user_stop;
                formData['sy_reg_type'] = that.config.sy_reg_type;
                formData['reg_moblie'] = that.config.reg_moblie == true ? 1 : 0;
                formData['reg_email'] = that.config.reg_email == true ? 1 : 0;
                formData['reg_user'] = that.config.reg_user == true ? 1 : 0;
                formData['sy_reg_nameminlen'] = parseInt(that.config.sy_reg_nameminlen);
                formData['sy_reg_namemaxlen'] = parseInt(that.config.sy_reg_namemaxlen);
                if (formData.reg_moblie == 1 && (that.config.sy_msg_appkey == '' || that.config.sy_msg_appsecret == '' || that.config.sy_msg_isopen != 1)) {
                    message.error(lc('admin_system_00548'), function () {
                        location.reload();
                    });
                    return false;
                }
                if (formData.reg_email == 1 && that.config.sy_email_set != 1) {
                    message.error(lc('admin_system_00549'), function () {
                        location.reload();
                    });
                    return false;
                }
                if (formData.reg_user_stop == 1 && formData.reg_user == 0 && formData.reg_mobile == 0 && formData.reg_email == 0) {
                    message.error(lc('admin_system_00537'));
                    return false;
                }
                if (formData.reg_user == 1) {
                    if (formData.sy_reg_nameminlen == 0 || formData.sy_reg_namemaxlen == 0 || formData.sy_reg_nameminlen == '' || formData.sy_reg_namemaxlen == '') {
                        message.error(lc('admin_system_00551'));
                        return false;
                    } else if (formData.sy_reg_namemaxlen <= formData.sy_reg_nameminlen) {
                        message.error(lc('admin_system_00546'));
                        return false;
                    }
                }
                formData['reg_name_num'] = that.config.reg_name_num == true ? 1 : 0;
                formData['reg_name_zm'] = that.config.reg_name_zm == true ? 1 : 0;
                formData['reg_name_sp'] = that.config.reg_name_sp == true ? 1 : 0;
                formData['reg_pw_num'] = that.config.reg_pw_num == true ? 1 : 0;
                formData['reg_pw_zm'] = that.config.reg_pw_zm == true ? 1 : 0;
                formData['reg_pw_sp'] = that.config.reg_pw_sp == true ? 1 : 0;
                formData['reg_passconfirm'] = that.config.reg_passconfirm == true ? 1 : 0;
                formData['reg_real_name_check'] = that.config.reg_real_name_check == true ? 1 : 0;
                formData['sy_def_email'] = that.config.sy_def_email;
                formData['sy_reg_interval'] = that.config.sy_reg_interval;
                formData['sy_reg_invite'] = that.config.sy_reg_invite;
                formData['regname'] = JSON.stringify(that.regconfig.regname);
                formData['mobile_number'] = JSON.stringify(that.regconfig.mobile_number);
                formData['mobile_white'] = JSON.stringify(that.regconfig.mobile_white);
                formData['mobile_black'] = JSON.stringify(that.regconfig.mobile_black);
                formData['config'] = 1;
                that.saveLoading = true;
                httpPost('m=system&c=set_regset&a=save', formData).then(function (response) {
                    var res = response.data;
                    if (res.error == 0) {
                        message.success(res.msg, function () {
                            that.getList();
                        });
                    } else {
                        message.error(res.msg);
                    }
                }).finally(function () {
                    setTimeout(function () {
                        that.saveLoading = false;
                    }, 2000);
                });
            },
            async getList() {
                let that = this;
                let param = {};

                httpPost('m=system&c=set_regset&a=index', param).then(function (response) {
                    let res = response.data;
                    if (res.error == 0) {
                        that.config = res.data.config;
                        that.regconfig = res.data.regconfig;

                        that.config.reg_passconfirm = that.config.reg_passconfirm == 1 ? true : false;
                        that.config.reg_real_name_check = that.config.reg_real_name_check == 1 ? true : false;
                        that.config.reg_moblie = that.config.reg_moblie == 1 ? true : false;
                        that.config.reg_email = that.config.reg_email == 1 ? true : false;
                        that.config.reg_user = that.config.reg_user == 1 ? true : false;
                        that.config.reg_name_num = that.config.reg_name_num == 1 ? true : false;
                        that.config.reg_name_zm = that.config.reg_name_zm == 1 ? true : false;
                        that.config.reg_name_sp = that.config.reg_name_sp == 1 ? true : false;
                        that.config.reg_pw_num = that.config.reg_pw_num == 1 ? true : false;
                        that.config.reg_pw_zm = that.config.reg_pw_zm == 1 ? true : false;
                        that.config.reg_pw_sp = that.config.reg_pw_sp == 1 ? true : false;

                        that.regconfig.regname = res.data.regconfig.regname?res.data.regconfig.regname:[];
                        that.regconfig.mobile_number = res.data.regconfig.mobile_number?res.data.regconfig.mobile_number:[];
                        that.regconfig.mobile_white = res.data.regconfig.mobile_white?res.data.regconfig.mobile_white:[];
                        that.regconfig.mobile_black = res.data.regconfig.mobile_black?res.data.regconfig.mobile_black:[];
                    }
                }).catch(function (error) {
                    console.log(error)
                })
            },
            handleClose(tag, name) {
                if (name == 'regname') {
                    this.regconfig.regname.splice(this.regconfig.regname.indexOf(tag), 1);
                } else if (name == 'mobile_number') {
                    this.regconfig.mobile_number.splice(this.regconfig.mobile_number.indexOf(tag), 1);
                } else if (name == 'mobile_white') {
                    this.regconfig.mobile_white.splice(this.regconfig.mobile_white.indexOf(tag), 1);
                } else if (name == 'mobile_black') {
                    this.regconfig.mobile_black.splice(this.regconfig.mobile_black.indexOf(tag), 1);
                }
            },
            showInput(name) {
                this.inputValue = '';
                if (name == 'regname') {
                    this.regnameinput = true;
                    this.$nextTick(_ => {
                        this.$refs.saveTagregname.$refs.input.focus();
                    });
                } else if (name == 'mobile_number') {
                    this.mobile_numberinput = true;
                    this.$nextTick(_ => {
                        this.$refs.saveTagmobile_number.$refs.input.focus();
                    });
                } else if (name == 'mobile_white') {
                    this.mobile_whiteinput = true;
                    this.$nextTick(_ => {
                        this.$refs.saveTagmobile_white.$refs.input.focus();
                    });
                } else if (name == 'mobile_black') {
                    this.mobile_blackinput = true;
                    this.$nextTick(_ => {
                        this.$refs.saveTagmobile_black.$refs.input.focus();
                    });
                }
            },
            handleInputConfirm(name) {
                let inputValue = this.inputValue;
                if (name == 'regname') {
                    this.regnameinput = false;
                    if (inputValue) {
                        if (this.regconfig.regname.indexOf(inputValue) != -1) {
                            message.warning(lc('admin_system_00560'));
                        } else {
                            this.regconfig.regname.push(inputValue);
                        }
                    }
                } else if (name == 'mobile_number') {
                    this.mobile_numberinput = false;
                    if (inputValue) {
                        if (this.regconfig.mobile_number.indexOf(inputValue) != -1) {
                            message.warning(lc('admin_system_00559'));
                        } else {
                            this.regconfig.mobile_number.push(inputValue);
                        }
                    }
                } else if (name == 'mobile_white') {
                    this.mobile_whiteinput = false;
                    if (inputValue) {
                        if (!isjsMobile(inputValue)){
                            message.warning(lc('admin_system_00553'));
                        }else if (this.regconfig.mobile_white.indexOf(inputValue) != -1) {
                            message.warning(lc('admin_system_00552'));
                        } else if (this.regconfig.mobile_black.indexOf(inputValue) != -1) {
                            message.warning(lc('admin_system_00542'));
                        } else {
                            this.regconfig.mobile_white.push(inputValue);
                        }
                    }
                } else if (name == 'mobile_black') {
                    this.mobile_blackinput = false;
                    if (inputValue) {
                        if (!isjsMobile(inputValue)){
                            message.warning(lc('admin_system_00553'));
                        }else if (this.regconfig.mobile_black.indexOf(inputValue) != -1) {
                            message.warning(lc('admin_01026'));
                        } else if (this.regconfig.mobile_white.indexOf(inputValue) != -1) {
                            message.warning(lc('admin_system_00631'));
                        } else {
                            this.regconfig.mobile_black.push(inputValue);
                        }
                    }
                }
            }
        }
    }
</script>
