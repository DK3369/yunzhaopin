<template>
    <div style="overflow: hidden;position: relative;height: 100%;">
        <div style="overflow-y: auto;position: relative;height: calc(100% - 80px);">
            <div class="drawerModInfo drawerModInfoOne">
                <div class="drawerModTites">
                    <el-divider content-position="left">{yun:}t key='admin_00027'{/yun}</el-divider>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='default_00319'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.username" placeholder="{yun:}t key='admin_00010'{/yun}"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='wap_00402'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" type="password" v-model="info.password" placeholder="{yun:}t key='wap_00400'{/yun}" :readonly="pwdreadonly"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_00003'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='member_user_00230'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.name" placeholder="{yun:}t key='admin_00011'{/yun}"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00021'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="info.m_id" placeholder="{yun:}t key='admin_00014'{/yun}">
                            <el-option v-for="item in group" :key="item.id" :label="item.group_name" :value="item.id"></el-option>
                        </el-select>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00025'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <template>
                            <el-time-select style="width: 120px;"placeholder="{yun:}t key='admin_00028'{/yun}" v-model="info.login_start" :picker-options="{start: '06:00', step: '00:10',end: '22:00'}"></el-time-select>
                            <el-time-select style="margin-left:10px; width: 120px;" placeholder="{yun:}t key='wap_user_00096'{/yun}" v-model="info.login_end" :picker-options="{start: '06:00',step: '00:10',end: '22:00', minTime: info.login_start}"></el-time-select>
                        </template>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_00002'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00024'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.isdid" label="1">{yun:}t key='admin_00037'{/yun}</el-radio>
                        <el-radio v-model="info.isdid" label="2">{yun:}t key='admin_00038'{/yun}</el-radio>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00031'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.index_lookstatistc" label="2">{yun:}t key='member_com_00287'{/yun}</el-radio>
                        <el-radio v-model="info.index_lookstatistc" label="1">{yun:}t key='common.close'{/yun}</el-radio>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_00004'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00019'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.is_crm" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                        <el-radio v-model="info.is_crm" label="0">{yun:}t key='common.close'{/yun}</el-radio>
                    </div>
                </div>
            </div>
            <div class="drawerModInfo drawerModInfoOne" v-show="info.is_crm == 1">
                <div class="drawerModTites">
                    <el-divider content-position="left">{yun:}t key='admin_00018'{/yun}</el-divider>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00032'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.weixin" placeholder="{yun:}t key='admin_00012'{/yun}"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='wap_01619'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.mobile" placeholder="{yun:}t key='wap_js_00119'{/yun}"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='wap_com_00174'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.qq" placeholder="{yun:}t key='wap_com_00141'{/yun}"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00020'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.num" placeholder="{yun:}t key='admin_00009'{/yun}"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_00389'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>

                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00023'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.call_num" placeholder="{yun:}t key='admin_00013'{/yun}" style="width: 194px;">
                            <template slot="prepend">{yun:}t key='admin_00036'{/yun}</template>
                        </el-input>
                        <el-input type="number" v-model="info.tuoxin_num" placeholder="{yun:}t key='admin_00013'{/yun}" style="width: 194px; margin-left: 16px;">
                            <template slot="prepend">{yun:}t key='admin_00034'{/yun}</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span> </span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.follow_num" placeholder="{yun:}t key='admin_00013'{/yun}" style="width: 194px;">
                            <template slot="prepend">{yun:}t key='admin_00035'{/yun}</template>
                        </el-input>
                        <el-input type="number" v-model="info.deal_num" placeholder="{yun:}t key='admin_00013'{/yun}" style="width: 194px; margin-left: 16px;">
                            <template slot="prepend">{yun:}t key='admin_00033'{/yun}</template>
                        </el-input>
                    </div>
                </div>

                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00023'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.month_deal_num" placeholder="{yun:}t key='admin_00013'{/yun}">
                            <template slot="prepend">{yun:}t key='admin_00033'{/yun}</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00022'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.jobtai_ranking" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                        <el-radio v-model="info.jobtai_ranking" label="2">{yun:}t key='common.close'{/yun}</el-radio>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_00005'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00030'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-checkbox-group v-model="info.crm_duty">
                            <el-checkbox v-for="(day,index) in week" :key="day" :label="index">{{day}}</el-checkbox>
                        </el-checkbox-group>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00029'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="info.crm_city" multiple filterable remote reserve-keyword placeholder="{yun:}t key='wap_00510'{/yun}" :filter-method="getCity">
                            <el-option v-for="item in cities" :key="item.id" :label="item.label" :value="item.id"></el-option>
                        </el-select>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="{yun:}t key='admin_00001'{/yun}" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00017'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-upload class="upload-demo" :accept="pic_accept" action="" :auto-upload="false" :show-file-list="false" :on-change="upPhotoChange">
                            <el-button slot="trigger" size="small" type="primary">{yun:}t key='admin_00015'{/yun}</el-button>
                            <img class="el-upload-list__item-thumbnail" width="36" height="36" v-if="info.photo" :src="info.photo" alt="{yun:}t key='admin_00017'{/yun}"/>
                        </el-upload>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>{yun:}t key='admin_00016'{/yun}</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-upload class="upload-demo" :accept="pic_accept" action="" :auto-upload="false" :show-file-list="false" :on-change="upEwmChange">
                            <el-button slot="trigger" size="small" type="primary">{yun:}t key='member_user_00157'{/yun}</el-button>
                            <img class="el-upload-list__item-thumbnail" width="36" height="36" v-if="info.ewm" :src="info.ewm" alt="{yun:}t key='admin_00016'{/yun}"/>
                        </el-upload>
                    </div>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" :loading="save_load" @click="submitForm">{yun:}t key='common.submit'{/yun}</el-button>
        </div>
    </div>
</template>
<script>
    module.exports = {
        props: {
            source: {type: String, default: 'useradd'}, // 编辑数据
            week: {type: Object, default: null}, // 轮值时间
            group: {type: Array, default: null}, // 轮值时间
            user: {type: Object, default: null} // 编辑数据
        },
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                loading: true,

                info: this.user,
                photo_file: [],
                ewm_file: [],
                cities: [],
                all_city: [],

                save_load: false,
                pwdreadonly: true
            }
        },
        created() {
            var that = this
            for (id in cn) {
                that.all_city.push({label: cn[id], id: id})
            }
            that.initData()
        },
        methods: {
            //用来阻止第二次或更多次点击密码输入框时下拉用户密码清单的框一闪而过的问题
            pwdMousedown(){
                var that = this
                this.pwdreadonly = true
                setTimeout(function(){ that.pwdreadonly = false, 100})
            },
            // 防止密码框内容清楚后展示用户密码清单
            pwdchange: function(val){
                var that = this
                if (val == '") {
                    this.pwdreadonly = true
                    setTimeout(function(){ that.pwdreadonly = false, 100})
                }
            },
            // 修改密码框readonly{yun:}t key='wap_js_00085'{/yun}，防止密码框展示浏览器记录的密码信息
            readonlyCtl: function(res){
                var that = this
                setTimeout(function(){
                    that.pwdreadonly = res
                }, 200)
            },
            // {yun:}t key='admin_00007'{/yun}
            initData() {
                var that = this
                if (that.info && that.info.uid) {
                    if (that.info.crm_city) {
                        that.cities = []
                        for (index in that.info.crm_city) {
                            let id = that.info.crm_city[index]
                            that.cities.push({label: cn[id], id: id})
                        }
                    }
                } else {
                    this.info = {
                        uid: 0,
                        username: "',
                        password: '',
                        name: '',
                        m_id: '',
                        control_login: null,
                        login_start: '',
                        login_end: '',
                        isdid: '2',
                        index_lookstatistc: '2',
                        is_crm: '0',
                        weixin: '',
                        mobile: '',
                        qq: '',
                        num: '',
                        call_num: '',
                        tuoxin_num: '',
                        follow_num: '',
                        deal_num: '',
                        month_deal_num: '',
                        jobtai_ranking: '2',
                        crm_duty: [],
                        crm_city: [],
                        photo: '',
                        ewm: ''
                    }
                }
            },
            getCity(query) {
                var that = this
                if (query !== '') {
                    this.loading = true;
                    setTimeout(() => {
                        this.loading = false;
                        this.cities = that.all_city.filter(item => {
                            if (item.label.includes(query)) {
                                return item
                            }
                        });
                        that.cities
                    }, 200);
                } else {
                    this.options = [];
                }
            },
            upPhotoChange(file) {
                // 预览文件处理
                this.info.photo = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.photo_file = file.raw;
            },
            upEwmChange(file) {
                // 预览文件处理
                this.info.ewm = URL.createObjectURL(file.raw);
                // 复刻文件信息
                this.ewm_file = file.raw;
            },
            submitForm() {
                let that = this;
                let formData = new FormData();
                if (!that.info.username) {
                    message.error(lc('wap_js_00056'));
                    return false;
                }
                if (!that.info.uid && !that.info.password) {
                    message.error("{yun:}t key='wap_js_00061'{/yun}");
                    return false;
                }
                if (!that.info.name) {
                    message.error("{yun:}t key='admin_00008'{/yun}");
                    return false;
                }
                if (!that.info.m_id) {
                    message.error("{yun:}t key='admin_00014'{/yun}");
                    return false;
                }
                for (i in that.info) {
                    formData.append(i, that.info[i])
                }
                if (that.photo_file.length !== 0) {
                    formData.append('photo_file', that.photo_file)
                }
                if (that.ewm_file.length !== 0) {
                    formData.append('ewm_file', that.ewm_file)
                }
                that.save_load = true;

                let url = '';
                if (that.source == 'useradd') {
                    formData.append('useradd', 1);
                    url = 'm=system&c=role_user&a=save';
                } else if (that.source == 'crmAdd') {
                    formData.append('crmAdd', 1);
                    url = 'm=crm&c=salesman&a=saveCrm';
                } else {
                    message.error("source {yun:}t key='upgrade_00015'{/yun}");
                    return false;
                }

                httpPost(url, formData).then(function (res) {
                    that.save_load = false;
                    if (res.data.error == 0) {
                        that.$message.success({
                            message: res.data.msg,
                            onClose: function () {
                                that.$emit('complete');
                            }
                        });
                    } else {
                        that.$message.error(res.data.msg);
                    }
                });
            },
        },
        watch: {
            user: function (val, oldVal) {
                if (val.id && oldVal.id && val.uid != oldVal.uid) {
                }
                this.info = val;
                this.initData();
            }
        }
    }
</script>
<style scoped>
    .uploadTable{width:calc(100% - 40px)}
    .moreTop{padding-top:10px}
    .titleTwoSpace{padding-left:50px}
    .moreInOne{display:flex}
    .fw{font-weight:900;color:#0a0a0a}
    .mingdEltags{overflow:hidden;position:relative;display:flex;align-items:center;padding-top:3px}
    .mingdEltags .el-tag{overflow:hidden;position:relative;margin:3px 4px!important}
    .el-input-group__prepend{background: #f5f7fa; padding: 0 14px;}
</style>