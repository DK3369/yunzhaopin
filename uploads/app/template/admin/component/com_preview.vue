<template>
    <div>
        <div v-loading="loading" class="uploadTable" style="padding:0 20px;">
            <div class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{Info.name}}</span>
                </div>
                <div class="jiliTanJinCont">
                    <span>IP：{{Info.login_ip}}</span>
                    <span v-if="Info.login_address">{{ lc('admin_00391') }}</span>
                </div>
				<div class="jiliTanJinCont" v-if="Info.shortname">
				    <span>{{ lc('admin_00392') }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.hy">
				    <span>{{ lc("admin_industry_value", [Info.hy_n]) }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.pr">
				    <span>{{ lc("admin_company_nature_value", [Info.pr_n]) }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.mun">
				    <span>{{ lc("admin_company_size_value", [Info.mun_n]) }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.money">
				    <span>{{ lc('admin_00393') }}</span>
				</div>
			</div>
			<div class="jiliTanJinli" v-if="Info.content">
			    <div class="jiliTanJinTite">
			        <span>{{ lc('wap_com_00160') }}</span>
			    </div>
				<div class="jiliTanJinCont" v-html="Info.content"></div>
			</div>
			<div class="jiliTanJinli">
			    <div class="jiliTanJinTite">
			        <span>{{ lc('wap_00462') }}</span>
			    </div>
				<div class="jiliTanJinCont" v-if="Info.linkman">
				    <span>{{ lc("admin_contact_person_value", [Info.linkman]) }} <span v-if="Info.linkjob">（{{Info.linkjob}})</span></span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linktel">
				    <span>{{ lc('admin_00394') }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linkphone">
				    <span>{{ lc('admin_00395') }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linkmail">
				    <span>{{ lc("admin_email_value", [Info.linkmail]) }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.linkqq">
				    <span>{{ lc('admin_00396') }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.website">
				    <span>{{ lc('admin_00397') }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.provinceid">
				    <span>{{ lc('admin_00398') }}</span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.comqcode">
				    <span>{{ lc('admin_00049') }}<el-image :src="Info.comqcode" width="150" height="150"></el-image></span>
				</div>
				<div class="jiliTanJinCont" v-if="Info.busstops">
				    <span>{{ lc("admin_bus_stop_value", [Info.busstops]) }}</span>
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