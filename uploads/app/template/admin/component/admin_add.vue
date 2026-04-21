<template>
    <div style="overflow: hidden;position: relative;height: 100%;">
        <div style="overflow-y: auto;position: relative;height: calc(100% - 80px);">
            <div class="drawerModInfo drawerModInfoOne">
                <div class="drawerModTites">
                    <el-divider content-position="left">账号信息</el-divider>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>登录账号</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.username" placeholder="请输入登录账号"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>账号密码</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input @mousedown.native="pwdMousedown" @input="pwdchange" @focus="readonlyCtl(false)" @blur="readonlyCtl(true)" type="password" v-model="info.password" placeholder="请输入账号密码" :readonly="pwdreadonly"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="如果密码留空则不修改密码" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>真实姓名</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.name" placeholder="请输入真实姓名"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>所属分组</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="info.m_id" placeholder="请选择用户组">
                            <el-option v-for="item in group" :key="item.id" :label="item.group_name" :value="item.id"></el-option>
                        </el-select>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>登录限制</span>
                    </div>
                    <div class="drawerModInpt">
                        <template>
                            <el-time-select style="width: 120px;"placeholder="起始时间" v-model="info.login_start" :picker-options="{start: '06:00', step: '00:10',end: '22:00'}"></el-time-select>
                            <el-time-select style="margin-left:10px; width: 120px;" placeholder="结束时间" v-model="info.login_end" :picker-options="{start: '06:00',step: '00:10',end: '22:00', minTime: info.login_start}"></el-time-select>
                        </template>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="每天在选择固定时间内才能登录" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>登录分站</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.isdid" label="1">允许</el-radio>
                        <el-radio v-model="info.isdid" label="2">禁止</el-radio>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>首页统计</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.index_lookstatistc" label="2">开启</el-radio>
                        <el-radio v-model="info.index_lookstatistc" label="1">关闭</el-radio>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="站点后台首页数据统计权限" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>业务身份</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.is_crm" label="1">开启</el-radio>
                        <el-radio v-model="info.is_crm" label="0">关闭</el-radio>
                    </div>
                </div>
            </div>
            <div class="drawerModInfo drawerModInfoOne" v-show="info.is_crm == 1">
                <div class="drawerModTites">
                    <el-divider content-position="left">业务信息</el-divider>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>微信号</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.weixin" placeholder="请输入微信号"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>手机号</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.mobile" placeholder="请输入手机号"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>联系QQ</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="info.qq" placeholder="请输入联系QQ"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>客户数量</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.num" placeholder="请输入客户数量"></el-input>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="填‘0’或者不填写表示不限" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>

                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>每日目标</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.call_num" placeholder="请输入目标数" style="width: 194px;">
                            <template slot="prepend">通话量</template>
                        </el-input>
                        <el-input type="number" v-model="info.tuoxin_num" placeholder="请输入目标数" style="width: 194px; margin-left: 16px;">
                            <template slot="prepend">拓新量</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span> </span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.follow_num" placeholder="请输入目标数" style="width: 194px;">
                            <template slot="prepend">跟进量</template>
                        </el-input>
                        <el-input type="number" v-model="info.deal_num" placeholder="请输入目标数" style="width: 194px; margin-left: 16px;">
                            <template slot="prepend">成交额</template>
                        </el-input>
                    </div>
                </div>

                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>每日目标</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="number" v-model="info.month_deal_num" placeholder="请输入目标数">
                            <template slot="prepend">成交额</template>
                        </el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>榜单权限</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio v-model="info.jobtai_ranking" label="1">开启</el-radio>
                        <el-radio v-model="info.jobtai_ranking" label="2">关闭</el-radio>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="工作台榜单查看权限" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>轮值时间</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-checkbox-group v-model="info.crm_duty">
                            <el-checkbox v-for="(day,index) in week" :key="day" :label="index">{{day}}</el-checkbox>
                        </el-checkbox-group>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>轮值区域</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="info.crm_city" multiple filterable remote reserve-keyword placeholder="请输入关键词" :filter-method="getCity">
                            <el-option v-for="item in cities" :key="item.id" :label="item.label" :value="item.id"></el-option>
                        </el-select>
                    </div>
                    <div class="drawerModTips">
                        <el-alert title="业务轮值安排，可以设轮值时间和区域" type="info" show-icon :closable="false"></el-alert>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>职业形象照</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-upload class="upload-demo" :accept="pic_accept" action="" :auto-upload="false" :show-file-list="false" :on-change="upPhotoChange">
                            <el-button slot="trigger" size="small" type="primary">上传形象照</el-button>
                            <img class="el-upload-list__item-thumbnail" width="36" height="36" v-if="info.photo" :src="info.photo" alt="职业形象照"/>
                        </el-upload>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>微信二维码</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-upload class="upload-demo" :accept="pic_accept" action="" :auto-upload="false" :show-file-list="false" :on-change="upEwmChange">
                            <el-button slot="trigger" size="small" type="primary">上传二维码</el-button>
                            <img class="el-upload-list__item-thumbnail" width="36" height="36" v-if="info.ewm" :src="info.ewm" alt="微信二维码"/>
                        </el-upload>
                    </div>
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" :loading="save_load" @click="submitForm">提交</el-button>
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
                if (val == '') {
                    this.pwdreadonly = true
                    setTimeout(function(){ that.pwdreadonly = false, 100})
                }
            },
            // 修改密码框readonly属性，防止密码框展示浏览器记录的密码信息
            readonlyCtl: function(res){
                var that = this
                setTimeout(function(){
                    that.pwdreadonly = res
                }, 200)
            },
            // 初始化用户信息
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
                        username: '',
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
                    message.error('请填写用户名');
                    return false;
                }
                if (!that.info.uid && !that.info.password) {
                    message.error('请填写密码');
                    return false;
                }
                if (!that.info.name) {
                    message.error('请填写真实姓名');
                    return false;
                }
                if (!that.info.m_id) {
                    message.error('请选择用户组');
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
                    message.error('source 参数异常');
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