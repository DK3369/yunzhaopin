<template>
    <div class="homeBottomAll">
        <ul v-show="false" id="yunzx"></ul>
        <div class="homeCentLeName" style="padding: 12px 12px 0 12px;">
            <span>系统信息</span>
        </div>
        <div class="homeBotomInfo">
            <div class="homeBotText">
                <span>PHPYun程序版本： {{sysinfo.version}} [ <label id="version_msg">无须更新!</label>]</span>
                <span>可用空间(磁盘区)：{{sysinfo.kongjian}} M</span>
                <span>PHP 版本：{{sysinfo.phpbanben}}</span>
                <span>服务器软件：{{sysinfo.soft}}</span>
            </div>
            <div class="homeBotText">
                <span>MySQL 版本：{{sysinfo.banben}}</span>
                <span>用户 - 服务器：{{sysinfo.yonghu}} - {{sysinfo.server}}</span>
                <span>系统名称：phpyun人才招聘系统</span>
                <span>开发团队：phpyun开发组</span>
            </div>
            <div class="homeBotText">
                <span>联系电话：400-880-5523</span>
                <span>官方网站：<el-link href='http://www.phpyun.com'  target="_blank" :underline="false">http://www.phpyun.com</el-link></span>
                <span>官方论坛：<el-link href='https://www.phpyun.com/bbs'  target="_blank" :underline="false">https://www.phpyun.com/bbs</el-link></span>
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