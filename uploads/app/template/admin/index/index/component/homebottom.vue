<template>
    <div class="homeBottomAll">
        <ul v-show="false" id="yunzx"></ul>
        <div class="homeCentLeName" style="padding: 12px 12px 0 12px;">
            <span>{{ lc('admin_01056') }}</span>
        </div>
        <div class="homeBotomInfo">
            <div class="homeBotText">
                <span>{{ lc("admin_phpyun_version", [sysinfo.version]) }} [ <label id="version_msg">{{ lc('admin_01057') }}</label>]</span>
                <span>{{ lc("admin_available_space", [sysinfo.kongjian]) }}</span>
                <span>{{ lc("admin_php_version", [sysinfo.phpbanben]) }}</span>
                <span>{{ lc("admin_server_software", [sysinfo.soft]) }}</span>
            </div>
            <div class="homeBotText">
                <span>{{ lc("admin_mysql_version", [sysinfo.banben]) }}</span>
                <span>{{ lc("admin_user_server", [sysinfo.yonghu, sysinfo.server]) }}</span>
                <span>{{ lc('admin_01058') }}</span>
                <span>{{ lc('admin_01059') }}</span>
            </div>
            <div class="homeBotText">
                <span>{{ lc('admin_00069') }}</span>
                <span>{{ lc('admin_01060') }}<el-link href='http://www.ov6.com'  target="_blank" :underline="false">http://www.ov6.com</el-link></span>
                <span>{{ lc('admin_01061') }}<el-link href='https://www.ov6.com/bbs'  target="_blank" :underline="false">https://www.ov6.com/bbs</el-link></span>
                <span>{{ lc('admin_01062') }}</span>
            </div>
        </div>
    </div>
</template>

<script>
module.exports = {
	props: {
	    sysinfo: {
	        type: Object,
	        default: {}
	    },
	},
    data: function () {
        return {

        }
    },

    mounted() {

    },
    methods: {

    },
};
</script>
<style scoped></style>