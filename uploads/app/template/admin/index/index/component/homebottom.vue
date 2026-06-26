<template>
    <div class="homeBottomAll">
        <ul v-show="false" id="yunzx"></ul>
        <div class="homeCentLeName" style="padding: 12px 12px 0 12px;">
            <span>{yun:}t key='admin_01056'{/yun}</span>
        </div>
        <div class="homeBotomInfo">
            <div class="homeBotText">
                <span>{{ lc("admin_phpyun_version", [sysinfo.version]) }} [ <label id="version_msg">{yun:}t key='admin_01057'{/yun}</label>]</span>
                <span>{{ lc("admin_available_space", [sysinfo.kongjian]) }}</span>
                <span>{{ lc("admin_php_version", [sysinfo.phpbanben]) }}</span>
                <span>{{ lc("admin_server_software", [sysinfo.soft]) }}</span>
            </div>
            <div class="homeBotText">
                <span>{{ lc("admin_mysql_version", [sysinfo.banben]) }}</span>
                <span>{{ lc("admin_user_server", [sysinfo.yonghu, sysinfo.server]) }}</span>
                <span>{yun:}t key='admin_01058'{/yun}</span>
                <span>{yun:}t key='admin_01059'{/yun}</span>
            </div>
            <div class="homeBotText">
                <span>{yun:}t key='admin_00069'{/yun}</span>
                <span>{yun:}t key='admin_01060'{/yun}<el-link href='http://www.ov6.com'  target="_blank" :underline="false">http://www.ov6.com</el-link></span>
                <span>{yun:}t key='admin_01061'{/yun}<el-link href='https://www.ov6.com/bbs'  target="_blank" :underline="false">https://www.ov6.com/bbs</el-link></span>
                <span>{yun:}t key='admin_01062'{/yun}</span>
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