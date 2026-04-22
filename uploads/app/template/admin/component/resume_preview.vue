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
                        <span v-if="resumeinfo.telphone">手机号：{{ resumeinfo.telphone }}</span>
                        <span v-if="resumeinfo.email">&nbsp;· 邮箱：{{ resumeinfo.email }}</span>
                    </span>
                    <span v-if="expect.add_ip" class="moreInOne">
                        <span>IP：{{ expect.add_ip }}</span>
                        <span style="padding-left: 15px;">IP地址：{{ expect.ip_address }}</span>
                    </span>
                </div>
            </div>
            <div class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>求职意向</span>
                </div>
                <div class="jiliTanJinCont">
                    <span>{{ expect.name }} · {{ expect.city_classname }} · {{ expect.report_n }} · {{ expect.type_n }} · {{ expect.jobstatus_n }}</span>
                    <span>
                        <template v-for="item in expect.expectjob">
                            {{ item }}&nbsp;
                        </template>· {{ expect.hy_n }}</span>
                    <span>期望薪资 {{ expect.salary }}</span>
                </div>
            </div>
            <div v-if="work.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>工作经历</span>
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
                    <span>教育经历</span>
                </div>
                <div v-for="(item, index) in edu" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="fw">{{item.name}}</span>
                    <span>{{ item.sdate_n }} ~ {{ item.edate_n }}</span>
                    <span class="moreInOne">
                        <span>最高学历：{{ item.education_n }}</span>
                        <span v-if="item.specialty" class="titleTwoSpace">所学专业：{{ item.specialty }}</span>
                    </span>
                </div>
            </div>
            <div v-if="training.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>培训经历</span>
                </div>
                <div v-for="(item, index) in training" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne">
                        <span class="fw">{{ item.name }}</span>
                        <span class="titleTwoSpace">培训方向：</span>
                        <span class="fw">{{ item.title }}</span>
                    </span>
                    <span>{{ item.sdate_n }} ~ {{ item.edate_n }}</span>
                    <span>{{ item.content }}</span>
                </div>
            </div>
            <div v-if="skill.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>职业技能</span>
                </div>
                <div v-for="(item, index) in skill" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne">
                        <span class="fw">{{ item.name }}</span>
                        <template v-if="item.longtime">
                            <span class="titleTwoSpace">掌握时间：</span>
                            <span class="fw">{{ item.longtime }}年</span>
                        </template>
                    </span>
                    <span>熟练程度：{{ item.ing_n }}</span>
                    <span v-if="item.pic" class="moreInOne">
                        <span>技能证书：</span>
                        <el-image style="width: 100px;" :src="item.pic" :preview-src-list="[item.pic]"></el-image>
                    </span>
                </div>
            </div>
            <div v-if="project.length" class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>项目经历</span>
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
                    <span>其他描述</span>
                </div>
                <div v-for="(item, index) in other" :key="index" class="jiliTanJinCont" :class="index > 0 ? 'moreTop' : ''">
                    <span class="moreInOne fw">{{ item.name }}</span>
                    <span>{{ item.content }}</span>
                </div>
            </div>
            <div class="jiliTanJinli">
                <div class="jiliTanJinTite">
                    <span>自我介绍</span>
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
            resumeinfo: {},//简历
            expect: {},//求职意向
            work: [],//工作经历
            edu: [],//教育经历
            training: [],//培训经历
            skill: [],//职业技能
            project: [],//项目经历
            other: [],//其他描述
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
            this.resumeinfo.exp_n && (arrTmp.push(this.resumeinfo.exp_n + '经验'));
            this.resumeinfo.edu_n && (arrTmp.push(this.resumeinfo.edu_n + '学历'));
            this.resumeinfo.age == 0 && (arrTmp.push('保密'));
            this.resumeinfo.age > 0 && (arrTmp.push(this.resumeinfo.age + '岁'));
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