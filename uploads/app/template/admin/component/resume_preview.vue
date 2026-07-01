<template>
    <div>
        <div v-loading="loading" class="uploadTable" style="padding:0 20px;">
            <div class="jiliTanDatas">
                <div class="jiliTanDaImgs">
                    <el-image :src="resumeinfo.photo ? resumeinfo.photo : ''"></el-image>
                </div>
                <div class="jiliTanDaText">
                    <h3>{{ resumeinfo ? resumeinfo.name : '' }}</h3>
                    <span>{{ stringExpEduAge }}</span>
                    <span v-if="!info.hasOwnProperty('matching')" class="moreInOne">
                        <span v-if="resumeinfo.telphone">{{ lc("admin_mobile_value", [resumeinfo.telphone]) }}</span>
                        <span v-if="resumeinfo.email">&nbsp;· {{ lc('member_user_00282') }}：{{ resumeinfo.email }}</span>
                    </span>
                    <span v-if="expect.add_ip" class="moreInOne">
                        <span>IP：{{ expect.add_ip }}</span>
                        <span style="padding-left: 15px;">{{ lc('admin_00400') }}</span>
                    </span>
                </div>
            </div>
            <div class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00460') }}</span>
                </div>
                <div class="jiliTanJinCont">
                    <span>{{ expect.name }} · {{ expect.city_classname }} · {{ expect.report_n }} · {{ expect.type_n }} · {{ expect.jobstatus_n }}</span>
                    <span>
                        <template v-for="item in expect.expectjob">
                            {{ item }}&nbsp;
                        </template>· {{ expect.hy_n }}</span>
                    <span>{{ lc('admin_00401') }}</span>
                </div>
            </div>
            <div v-if="work.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00457') }}</span>
                </div>
                <div v-for="(item, index) in work" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne">
                        <span class="fw">{{ item.name }}</span>
                        <span v-if="item.title" class="fw titleTwoSpace">{{ item.title }}</span>
                    </span>
                    <span>{{ item.sdate_n }} ~ {{ item.edate_n }}</span>
                    <span>{{ item.content }}</span>
                </div>
            </div>
            <div v-if="edu.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00459') }}</span>
                </div>
                <div v-for="(item, index) in edu" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="fw">{{item.name}}</span>
                    <span>{{ item.sdate_n }} ~ {{ item.edate_n }}</span>
                    <span class="moreInOne">
                        <span>{{ lc('admin_00402') }}</span>
                        <span v-if="item.specialty" class="titleTwoSpace">{{ lc('admin_00403') }}</span>
                    </span>
                </div>
            </div>
            <div v-if="training.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00455') }}</span>
                </div>
                <div v-for="(item, index) in training" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne">
                        <span class="fw">{{ item.name }}</span>
                        <span class="titleTwoSpace">{{ lc('member_user_00008') }}</span>
                        <span class="fw">{{ item.title }}</span>
                    </span>
                    <span>{{ item.sdate_n }} ~ {{ item.edate_n }}</span>
                    <span>{{ item.content }}</span>
                </div>
            </div>
            <div v-if="skill.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00461') }}</span>
                </div>
                <div v-for="(item, index) in skill" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne">
                        <span class="fw">{{ item.name }}</span>
                        <template v-if="item.longtime">
                            <span class="titleTwoSpace">{{ lc('member_user_00083') }}</span>
                            <span class="fw">{{ lc('admin_00404') }}</span>
                        </template>
                    </span>
                    <span>{{ lc('admin_00405') }}</span>
                    <span v-if="item.pic" class="moreInOne">
                        <span>{{ lc('resume_00012') }}</span>
                        <el-image style="width: 100px;" :src="item.pic" :preview-src-list="[item.pic]"></el-image>
                    </span>
                </div>
            </div>
            <div v-if="project.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00465') }}</span>
                </div>
                <div v-for="(item, index) in project" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne">
                        <span class="fw">{{ item.name }}</span>
                        <span v-if="item.title" class="fw titleTwoSpace">{{ item.title }}</span>
                    </span>
                    <span>{{ item.sdate_n }} ~ {{ item.edate_n }}</span>
                    <span>{{ item.content }}</span>
                </div>
            </div>
            <div v-if="other.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('admin_00068') }}</span>
                </div>
                <div v-for="(item, index) in other" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne fw">{{ item.name }}</span>
                    <span>{{ item.content }}</span>
                </div>
            </div>
            <div class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>{{ lc('wap_00527') }}</span>
                </div>
                <div class="jiliTanJinCont">
                    <span>{{ resumeinfo.description }}</span>
                </div>
            </div>
        </div>
    </div>
</template>
<script>
module.exports = {
    props: {
        id: {type: [Number, String], default: 0},
        uid: {type: [Number, String], default: 0},
    },
    data: function () {
        return {
            loading: false,
            stringExpEduAge: '',
            info: {},
            resumeinfo: {},//{{ lc('common.resume') }}
            expect: {},//{{ lc('wap_00460') }}
            work: [],//{{ lc('wap_00457') }}
            edu: [],//{{ lc('wap_00459') }}
            training: [],//{{ lc('wap_00455') }}
            skill: [],//{{ lc('wap_00461') }}
            project: [],//{{ lc('wap_00465') }}
            other: [],//{{ lc('admin_00068') }}
        }
    },
	watch: {
		id: {
			handler(val) {
				this.getInfo();
			},
			immediate: true,
			deep: true,
		},
		uid: {
			handler(val) {
				this.getInfo();
			},
			immediate: true,
			deep: true,
		},
	},
    methods: {
        getInfo() {
            let _this = this;
            let params = {};
            if (this.id > 0) {
                params = {id: this.id};
            } else {
                params = {uid: this.uid};
            }
            this.loading = true;
            httpPost('m=user&c=users_resume&a=resumePreview', params, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.info = Object.freeze(res.data);
                    if (res.data.resumeinfo) {
                        _this.resumeinfo = Object.freeze(res.data.resumeinfo);
                        _this.handleExpEduAge();
                    }
                    res.data.expect && (_this.expect = Object.freeze(res.data.expect));
                    res.data.work && (_this.work = Object.freeze(res.data.work));
                    res.data.edu && (_this.edu = Object.freeze(res.data.edu));
                    res.data.training && (_this.training = Object.freeze(res.data.training));
                    res.data.skill && (_this.skill = Object.freeze(res.data.skill));
                    res.data.project && (_this.project = Object.freeze(res.data.project));
                }
                _this.loading = false;
            }).catch(function (error) {
                console.log(error);
            });
        },
        handleExpEduAge() {
            let arrTmp = [];
            this.resumeinfo.exp_n && (arrTmp.push(this.resumeinfo.exp_n + lc('home.experience_suffix')));
            this.resumeinfo.edu_n && (arrTmp.push(this.resumeinfo.edu_n + lc('home.education_suffix')));
            this.resumeinfo.age == 0 && (arrTmp.push(lc('wap_user_00333')));
            this.resumeinfo.age > 0 && (arrTmp.push(this.resumeinfo.age + lc('home.age_suffix')));
            arrTmp.length && (this.stringExpEduAge = arrTmp.join(' · '));
        }
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