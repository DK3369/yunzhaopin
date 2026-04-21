<template>
    <div class="moduleElHight">
        <div class="moduleSchools" style="height: 100%;">
            <div style="overflow-y: auto; position: relative; height: calc(100% - 80px);">
                <template v-if="!uid">
                    <div>账户信息</div>
                    <div class="drawerModLis">
                        <div class="drawerModTite">
                            <span>登录账户</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-tooltip class="item" v-model="usernameTips" manual effect="dark" :content="usernameMsg" placement="top">
                                <el-input v-model="ruleForm.username" placeholder="" @blur="checkUsername"></el-input>
                            </el-tooltip>
                        </div>
                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite">
                            <span>密码</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleForm.password" placeholder="" show-password @input="inputPassword($event, 'ruleForm', 'password')"></el-input>
                        </div>
                    </div>
                    <div class="drawerModLis" style="align-items: initial;">
                        <div class="drawerModTite">
                            <span>确认密码</span>
                        </div>
                        <div class="drawerModInpt">
                            <el-input v-model="ruleForm.passconfirm" placeholder="" show-password @input="inputPassword($event, 'ruleForm', 'passconfirm')"></el-input>
                        </div>
                    </div>
                </template>
                <div>基本资料</div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>用户姓名</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.resume_name" placeholder=""></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>性别</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-radio-group v-model="ruleForm.sex">
                            <el-radio v-for="(sex, sexkey) in user_sex" :key="sexkey" :label="sexkey">{{sex}}</el-radio>
                        </el-radio-group>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>教育程度</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="ruleForm.edu" placeholder="请选择">
                            <el-option v-for="edukey in userdata.user_edu" :key="edukey" :label="userclass_name[edukey]" :value="edukey"></el-option>
                        </el-select>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>现居住地</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.living" placeholder=""></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>工作经验</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-select v-model="ruleForm.exp" placeholder="请选择">
                            <el-option v-for="wordkey in userdata.user_word" :key="wordkey" :label="userclass_name[wordkey]" :value="wordkey"></el-option>
                        </el-select>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>出生年月</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-date-picker v-model="ruleForm.birthday" type="month" placeholder="选择月" :picker-options="pickerOptions"></el-date-picker>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>手机</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.moblie" placeholder="" @input="inputIntNumber($event, 'ruleForm', 'moblie')"></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>邮箱</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input v-model="ruleForm.email" placeholder=""></el-input>
                    </div>
                </div>
                <div class="drawerModLis">
                    <div class="drawerModTite">
                        <span>自我评价</span>
                    </div>
                    <div class="drawerModInpt">
                        <el-input type="textarea" :rows="2" v-model="ruleForm.description" placeholder=""></el-input>
                    </div>
                </div>
            </div>
            <div class="setBasicButn" style="border: none;">
                <el-button type="primary" size="medium" @click="save">下一步</el-button>
            </div>
        </div>
        <div class="modluDrawer">
            <!--新增简历-->
            <el-drawer title="新增简历" :visible.sync="drawerEdit" append-to-body :before-close="handleClose" :wrapper-closable="false" size="70%">
                <edit :uid="userId"></edit>
            </el-drawer>
        </div>
    </div>
</template>
<script>
    module.exports = {
        props: {
            uid: {
                type: [Number, String]
            }
        },
        data: function () {
            return {
                userId: 0,
                ruleForm: {
                    birthday: new Date('1990-01')
                },
                pickerOptions: {
                    disabledDate(time) {
                        let year = parseInt((new Date()).getFullYear());
                        year = year - 16;
                        return time.getTime() > new Date(year + '-12-31');
                    },
                },

                // 用户名提示
                usernameTips: false,
                usernameMsg: '',

                // 选择数据
                user_sex: {},
                userdata: {},
                userclass_name: {},

                saveLoading: false,

                drawerEdit: false,
            }
        },
        components: {
            'edit': httpVueLoader('./resume_edit.vue'),
        },
        created() {
            this.getInfo();
        },
        methods: {
            handleClose(done) {
                this.$emit("child-event");
                this.drawerEdit = false;
                done();
            },
            getInfo() {
                let that = this,
                    params = {};

                if (this.uid > 0) {
                    that.ruleForm.uid = that.uid;
                    params.uid = that.uid;
                }else if (this.userId > 0) {
                    that.ruleForm.uid = that.userId;
                    params.uid = that.userId;
                }
                params.add = 2;

                httpPost('m=user&c=users_resume&a=add', params).then(function (response) {
                    let res = response.data,
                        data = res.data;

                    that.user_sex = data.user_sex;
                    that.userdata = data.userdata;
                    that.userclass_name = data.userclass_name;

                    if (data.resume) {
                        let resume = data.resume;
                        that.ruleForm = {
                            uid: resume.uid,
                            resume_name: resume.name,
                            sex: resume.sex,
                            edu: resume.edu && resume.edu > 0 ? resume.edu : '',
                            living: resume.living,
                            exp: resume.exp && resume.exp > 0 ? resume.exp : '',
                            birthday: resume.birthday ? new Date(resume.birthday) : new Date('1990-01'),
                            moblie: resume.telphone,
                            email: resume.email,
                            description: resume.description
                        };
                    }
                })
            },

            checkUsername() {
                let that = this,
                    username = this.ruleForm.username;

                if (isEmpty(username)) {
                    that.usernameTips = false;
                    return false;
                }

                httpPost('m=user&c=users_resume&a=checkUsername', {username: username}).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.usernameTips = true;
                        that.usernameMsg = res.msg;
                    } else {
                        that.usernameTips = false;
                    }
                })
            },

            inputIntNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9]/g, '');
            },
            inputPassword(val, form, key) {
                this.$data[form][key] = val.replace(/^ +| +$/g, '');
            },

            save() {
                let that = this,
                    params = that.ruleForm;

                var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
                if (isEmpty(this.uid)) {
                    if (isEmpty(params.username)) {
                        message.warning("登录账户不能为空");
                        return false;
                    }
                    if (isEmpty(params.password)) {
                        message.warning("密码不能为空");
                        return false;
                    }
                    if (params.password.length < 6) {
                        message.warning("密码长度不能小6位字符");
                        return false;
                    }
                    if (isEmpty(params.passconfirm)) {
                        message.warning("请再次输入密码");
                        return false;
                    }
                    if (params.password != params.passconfirm) {
                        message.warning("两次密码不一致");
                        return false;
                    }
                }
                if (isEmpty(params.resume_name)) {
                    message.warning("用户姓名不能为空");
                    return false;
                }
                if (isEmpty(params.sex)) {
                    message.warning("性别不能为空");
                    return false;
                }
                if (isEmpty(params.living)) {
                    message.warning("现居住地不能为空");
                    return false;
                }
                if (isEmpty(params.birthday)) {
                    message.warning("出生年月不能为空");
                    return false;
                }
                if (isEmpty(params.moblie)) {
                    message.warning("手机号码不能为空");
                    return false;
                } else if (!isjsMobile(params.moblie)) {
                    message.warning("手机号码格式错误");
                    return false;
                }
                if (!isEmpty(params.email) && !myreg.test(params.email)) {
                    message.warning("邮箱格式错误");
                    return false;
                }
                if (isEmpty(params.description)) {
                    message.warning("自我评价不能为空");
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=user&c=users_resume&a=add', params).then(function (response) {
                    let res = response.data;

                    that.saveLoading = false;
                    if (res.error > 0) {
                        message.error(res.msg);
                    } else {
                        that.userId = res.data.uid;
                        that.openEdit();
                        message.success(res.msg);
                    }
                })
            },

            // 添加/修改简历
            openEdit() {
                this.drawerEdit = true
            },
        },
        watch: {
            uid: function (val, oldVal) {
                this.userId = parseInt(val);
                this.ruleForm = {};
                this.getInfo();
            }
        }
    };
</script>
<style scoped></style>