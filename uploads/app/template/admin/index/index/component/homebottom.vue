<template>
    <div class="homeBottomAll">
        <ul v-show="false" id="yunzx"></ul>
        <div class="homeCentLeName" style="padding: 12px 12px 0 12px;">
            <span>系统信息</span>
        </div>
        <div class="homeBotomInfo">
            <div class="homeBotText">
                <span>{{ lc("admin_phpyun_version", [sysinfo.version]) }} [ <label id="version_msg">无须更新!</label>]</span>
                <span>{{ lc("admin_available_space", [sysinfo.kongjian]) }}</span>
                <span>{{ lc("admin_php_version", [sysinfo.phpbanben]) }}</span>
                <span>{{ lc("admin_server_software", [sysinfo.soft]) }}</span>
            </div>
            <div class="homeBotText">
                <span>{{ lc("admin_mysql_version", [sysinfo.banben]) }}</span>
                <span>{{ lc("admin_user_server", [sysinfo.yonghu, sysinfo.server]) }}</span>
                <span>系统名称：OV6人才招聘</span>
                <span>开发团队：phpyun开发组</span>
            </div>
            <div class="homeBotText">
                <span>联系电话：400-880-5523</span>
                <span>官方网站：<el-link href='http://www.ov6.com'  target="_blank" :underline="false">http://www.ov6.com</el-link></span>
                <span>官方论坛：<el-link href='https://www.ov6.com/bbs'  target="_blank" :underline="false">https://www.ov6.com/bbs</el-link></span>
                <span>咨询QQ ：3367562 , 721799845 , 721799844</span>
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