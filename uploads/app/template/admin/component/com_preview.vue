<template>
    <div>
        <div v-loading="loading" class="uploadTable" style="padding:0 20px;">
            <div class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{Info.name}}</span>
                </div>
                <div class="jiliTanJinCont">
                    <span>IP：{{Info.login_ip}}</span>
                    <span v-if="Info.login_address">IP归属地 {{Info.login_address}}</span>
                </div>
				<div class="jiliTanJinCont" v-if="Info.shortname">
				    <span>企业简称：{{Info.shortname}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.hy">
				    <span>从事行业：{{Info.hy_n}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.pr">
				    <span>企业性质：{{Info.pr_n}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.mun">
				    <span>企业规模：{{Info.mun_n}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.money">
				    <span>注册资金：{{Info.money}} {{Info.moneytype_n}}</span>
				</div>
			</div>
			<div class="jiliTanJinli" v-if="Info.content">
			    <div class="jiliTanJinTite">
			        <span>企业简介</span>
			    </div>
				<div class="jiliTanJinCont" v-html="Info.content"></div>
			</div>
			<div class="jiliTanJinli">
			    <div class="jiliTanJinTite">
			        <span>联系方式</span>
			    </div>
				<div class="jiliTanJinCont" v-if="Info.linkman">
				    <span>联系人：{{Info.linkman}} <span v-if="Info.linkjob">（{{Info.linkjob}})</span></span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linktel">
				    <span>联系手机：{{Info.linktel}} {{Info.infostatus=='1'?'（公开）':'（不公开）'}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linkphone">
				    <span>固定电话：{{Info.linkphone}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linkmail">
				    <span>联系邮箱：{{Info.linkmail}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linkqq">
				    <span>联系QQ：{{Info.linkqq}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.website">
				    <span>企业网址：{{Info.website}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.provinceid">
				    <span>企业地址：{{Info.job_city_one}} {{Info.job_city_two}} {{Info.job_city_three}} {{Info.address}}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.comqcode">
				    <span>公司二维码：<el-image :src="Info.comqcode" width="150" height="150"></el-image></span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.busstops">
				    <span>公交站点：{{Info.busstops}}</span>
				</div>
			</div>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        uid: {type: [Number, String], default: ''},
    },
    data: function () {
        return {
            loading: false,
            Info: {}
        }
    },
    created() {
        this.getInfo();
    },
    methods: {
        getInfo() {
            let _this = this;
            let params = {uid: this.uid};
            
            this.loading = true;
            httpPost('m=user&c=company_company&a=compreview', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.Info = res.data;
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        },
        
    }
}
</script>
<style scoped>
.uploadTable{
    width: calc(100% - 40px);
}
.moreTop{
    padding-top: 10px;
}
.titleTwoSpace{
    padding-left: 50px;
}
.moreInOne{
    display: flex;
}
.fw{
    font-weight: 900;
    color: #0a0a0a;
}
</style>