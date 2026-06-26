<template>
    <div class="drawerModlue" style="padding: 0;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_00534'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.username" placeholder="{yun:}t key='admin_01030'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='member_user_00232'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.password" placeholder="{yun:}t key='wap_00703'{/yun}" show-password></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00141'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.name" placeholder="{yun:}t key='admin_00011'{/yun}"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_system_00142'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="userInfo.did" placeholder="{yun:}t key='admin_01031'{/yun}">
                        <el-option v-for="item in domainOptionS" :key="item.value" :label="item.label"
                            :value="item.value"></el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{yun:}t key='admin_01029'{/yun}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="userInfo.m_id" placeholder="{yun:}t key='admin_01032'{/yun}">
                        <el-option v-for="item in groupOptionS" :key="item.value" :label="item.label"
                            :value="item.value"></el-option>
                    </el-select>
                </div>
            </div>
            <div slot="footer" class="dialog-footer">
                <el-button @click="closeAdminBox">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
                <el-button type="primary" @click="saveAdmin" :disabled="saveLoading">{yun:}t key='wap_com_00019'{/yun}</el-button>
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
                message.error("{yun:}t key='admin_00008'{/yun}");
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