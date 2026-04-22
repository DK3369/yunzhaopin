<template>
    <div class="drawerModlue" style="padding: 0;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>用户名：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.username" placeholder="请输入登录用户名"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>密码：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.password" placeholder="请输入登录密码" show-password></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>真实姓名：</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.name" placeholder="请输入真实姓名"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>分站：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="userInfo.did" placeholder="请选择所属分站">
                        <el-option v-for="item in domainOptionS" :key="item.value" :label="item.label"
                            :value="item.value"></el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>分站用户组：</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="userInfo.m_id" placeholder="请选择所在用户组">
                        <el-option v-for="item in groupOptionS" :key="item.value" :label="item.label"
                            :value="item.value"></el-option>
                    </el-select>
                </div>
            </div>
            <div slot="footer" class="dialog-footer">
                <el-button @click="closeAdminBox">取 消</el-button>
                <el-button type="primary" @click="saveAdmin" :disabled="saveLoading">确 定</el-button>
            </div>
        </div>

    </div>
</template>

<script>
module.exports = {
    props: {
        admin_uid: Number
    },
    watch: {
        admin_uid: {
            handler(newValue, oldValue) {
                this.getCache();
                if (newValue != 0) {

                    this.getUserInfo();
                } else {

                    this.resetUserInfo();
                }
            },
            deep: true,
            immediate: true
        }
    },
    data: function () {
        return {
            userInfo: {
                username: '',
                password: '',
                name: '',
                did: '',
                m_id: ''
            },
            domainOptionS: [],
            groupOptionS: [],
            saveLoading: false
        }
    },
    mounted() {
    },
    methods: {
        async getCache() {
            let res = await httpPost('m=system&c=domain_group&a=getAdminCache');
            if (res.data.error == 0) {
                this.domainOptionS = [];
                this.groupOptionS = [];
                var groupList = res.data.data.groupArr;
                groupList.forEach((item) => {
                    this.groupOptionS.push({ value: item.id, label: item.name });
                });
                var domainList = res.data.data.domainArr;
                domainList.forEach((item) => {
                    this.domainOptionS.push({ value: item.id, label: item.name });
                });
            }
        },
        getUserInfo: function () {
            var self = this;
            httpPost('m=system&c=domain_group&a=adminInfo', { uid: this.admin_uid }).then(function (res) {
                if (res.data.error == 0) {
                    self.userInfo = res.data.data;
                }
            });
        },
        resetUserInfo: function () {
            var self = this;
            self.userInfo = {};
        },
        saveAdmin: function () {
            var self = this;
            var params = {};

            if (!self.userInfo.username) {
                message.error('请填写登录用户名');
                return false;
            }
            if (!self.userInfo.name) {
                message.error('请填写真实姓名');
                return false;
            }
            params = self.userInfo;
            if (self.admin_uid > 0) {

                params.uid = self.admin_uid;
            }
            self.saveLoading = true;
            httpPost('m=system&c=domain_group&a=saveAdmin', params).then(function (res) {
                if (res.data.error == 0) {
                    message.success(res.data.msg, function () {

                        self.$emit("child-event");
                    });
                } else {

                    message.error(res.data.msg);
                }
            }).finally(function () {
                setTimeout(function () {
                    self.saveLoading = false;
                }, 2000);
            });
        },
        closeAdminBox: function () {
            var self = this;

            self.$emit('child-event');
        }
    },
};
</script>
<style scoped>
.drawerModInfo{
    padding-right: 0;
}
.dialog-footer{
    padding: 10px 0px 20px;
    text-align: right;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
}
</style>