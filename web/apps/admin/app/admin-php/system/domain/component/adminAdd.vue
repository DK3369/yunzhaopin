<template>
    <div class="drawerModlue" style="padding: 0;">
        <div class="drawerModInfo">
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_00534') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.username" :placeholder="lc('admin_01030')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('member_user_00232') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.password" :placeholder="lc('wap_00703')" show-password></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00141') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-input v-model="userInfo.name" :placeholder="lc('admin_00011')"></el-input>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_system_00142') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="userInfo.did" :placeholder="lc('admin_01031')">
                        <el-option v-for="item in domainOptionS" :key="item.value" :label="item.label"
                            :value="item.value"></el-option>
                    </el-select>
                </div>
            </div>
            <div class="drawerModLis">
                <div class="drawerModTite">
                    <span>{{ lc('admin_01029') }}</span>
                </div>
                <div class="drawerModInpt">
                    <el-select v-model="userInfo.m_id" :placeholder="lc('admin_01032')">
                        <el-option v-for="item in groupOptionS" :key="item.value" :label="item.label"
                            :value="item.value"></el-option>
                    </el-select>
                </div>
            </div>
            <div class="dialog-footer">
                <el-button @click="closeAdminBox">{{ lc('admin_user_weipin_00043') }}</el-button>
                <el-button type="primary" @click="saveAdmin" :disabled="saveLoading">{{ lc('wap_com_00019') }}</el-button>
            </div>
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
                message.error(lc('admin_vue_00045'));
                return false;
            }
            if (!self.userInfo.name) {
                message.error(lc('admin_00008'));
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