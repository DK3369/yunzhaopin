<template>
    <!-- Member > Individual > Personal Settings -->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{{ lc('member_com_00021') }}</th>
                    <th width="680">{{ lc('member_user_00181') }}</th>
                    <th>{{ lc('member_com_00207') }}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00336') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_height_resume" :label="lc('wap_com_00375')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_idcard_status" :label="lc('wap_01030')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_msg_status" :label="lc('wap_com_00408')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_photo_status" :label="lc('member_user_00161')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.rshow_photo_status" :label="lc('admin_user_00190')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_trust_status" :label="lc('member_user_00276')"></el-checkbox>
                        </div>
                    
                    </td>
                    <td>
                        <div class="TableShuom">
                            {{ lc('admin_00557') }}
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00251') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_status" :label="lc('wap_user_00197')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_revise_state" :label="lc('wap_00269')"></el-checkbox>
                        </div>
                        <div v-if="ruleForm.resume_status == true || ruleForm.user_revise_state == true"
                             class="content-sub">
                            <div class="TableShuom" style="margin-bottom: 8px">
                                {{ lc('admin_00558') }}
                            </div>
                            <el-time-picker v-model="ruleForm.resume_statetime_start" placeholder=""
                                            value-format="HH:mm" format="HH:mm" style="width: 150px;">
                            </el-time-picker>&nbsp;-
                            <el-time-picker v-model="ruleForm.resume_statetime_end" placeholder="" value-format="HH:mm"
                                            format="HH:mm" style="width: 150px;">
                            </el-time-picker>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00337') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_resume_status" :label="lc('wap_user_00197')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_gzgzh" :label="lc('wap_user_00191')"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00338') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_kstd" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.resume_kstd" label="2">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span></span>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00327') }}</div>
                        <!--<div style="color:red;">This section needs restyling</div>-->
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_exp" :label="lc('wap_00457')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_edu" :label="lc('wap_00459')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_project" :label="lc('wap_00465')"></el-checkbox>
                        </div>
                        <div class="content-sub"
                             v-if="ruleForm.resume_create_exp == true && cache.hasOwnProperty('userdata')">
                            <el-checkbox-group v-model="ruleForm.expcreate">
                                <div v-for="(item,key) in cache.userdata.user_word" :key="key" class="tc_checkbox">
                                    <el-checkbox :label="item">{{ cache.userclass_name[item] }}</el-checkbox>
                                </div>
                            </el-checkbox-group>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info"> {{ lc('admin_00559') }}
                                </div>
                            </div>
                        </div>
                        <div class="content-sub"
                             v-if="ruleForm.resume_create_edu == true && cache.hasOwnProperty('userdata')">
                            <el-checkbox-group v-model="ruleForm.educreate">
                                <div v-for="(item,key) in cache.userdata.user_edu" :key="key" class="tc_checkbox">
                                    <el-checkbox :label="item">{{ cache.userclass_name[item] }}</el-checkbox>
                                </div>
                            </el-checkbox-group>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    {{ lc('admin_00560') }}
                                </div>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>{{ lc('admin_user_00312') }}</p>
                            <div class="TableShuom tc_checktip">
                                {{ lc('admin_00561') }}
                            </div>
                            <!-- TODO: 7.0 unified category selector -->
                            <!-- <el-button>Set job category items as not required</el-button> -->
                            <job_class ref="job_class" multiple :max="100" @confirm="confirmJob" :showsearch="false" :selected="jobSelected"></job_class>
                            <div style="overflow: hidden; position: relative; margin-top: 8px" v-if="ruleForm.sy_resume_job_classid && cache.hasOwnProperty('job_name')">
                                <el-tag style="margin:5px 5px 0 0;" v-for="(item,key) in ruleForm.sy_resume_job_classid" :key="key" @close="deltag(item)" closable>{{ cache.job_name[item] }}</el-tag>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>{{ lc('admin_user_00316') }}</p>
                            <div class="contentRadio">
                                <el-radio v-model="ruleForm.sy_resume_kh_td" label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio v-model="ruleForm.sy_resume_kh_td" label="0">{{ lc('common.close') }}</el-radio>
                            </div>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    {{ lc('admin_00562') }}
                                </div>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>{{ lc('admin_user_00317') }}</p>
                            <div class="contentRadio">
                                <el-radio v-model="ruleForm.resume_kstd_req" label="1">{{ lc('member_com_00287') }}</el-radio>
                                <el-radio v-model="ruleForm.resume_kstd_req" label="2">{{ lc('common.close') }}</el-radio>
                            </div>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    {{ lc('admin_00563') }}
                                </div>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {{ lc('admin_user_00315') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00318') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_sqintegrity" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">%</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00331') }}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_work_regiser" :label="lc('wap_00457')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_edu_regiser" :label="lc('wap_00459')"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_project_regiser" :label="lc('wap_00465')"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00325') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_rname_num" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02073') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00321') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_number" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02052') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {{ lc('admin_user_00332') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00564') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_finder" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02050') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {{ lc('admin_user_00314') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00319') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_trust_number" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02052') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {{ lc('admin_user_00322') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('wap_user_00238') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.user_name" label="1">{{ lc('admin_00565') }}</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="2">{{ lc('member_com_00345') }}</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="3">{{ lc('admin_user_00339') }}</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="4">{{ lc('member_user_00230') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">{{ lc('admin_user_00311') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00566') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.user_pic" label="1">{{ lc('admin_00565') }}</el-radio>
                            <el-radio v-model="ruleForm.user_pic" label="2">{{ lc('member_com_00023') }}</el-radio>
                            <el-radio v-model="ruleForm.user_pic" label="3">{{ lc('admin_user_00340') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"></div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00567') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_sx" label="1">{{ lc('admin_user_00329') }}</el-radio>
                            <el-radio v-model="ruleForm.resume_sx" label="2">{{ lc('admin_user_00328') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"></div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00333') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_open_check" label="1">{{ lc('common.close') }}</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="2">{{ lc('admin_user_00335') }}</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="3">{{ lc('common.publish_job') }}</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="4">{{ lc('admin_00568') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">
                                {{ lc('admin_user_00306') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_00569') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_user_visit_resume" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.sy_user_visit_resume" label="0">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">{{ lc('admin_user_00313') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00323') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_shresume_applyjob" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.sy_shresume_applyjob" label="0">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">{{ lc('admin_00570') }}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00320') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_resume_partapply" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.com_resume_partapply" label="0">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"></div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00330') }}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_resumename_num" label="1">{{ lc('member_com_00287') }}</el-radio>
                            <el-radio v-model="ruleForm.sy_resumename_num" label="0">{{ lc('common.close') }}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">
                                {{ lc('admin_00571') }}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{{ lc('admin_user_00326') }}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sq_resume_interval" :placeholder="lc('admin_user_00342')">
                                <span slot="suffix" class="slotspan">{{ lc('common_02067') }}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{{ lc('admin_user_00310') }}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{{ lc('common.submit') }}
            </el-button>
        </div>
    </div>
</template>
<script>
module.exports = {
    data: function () {
        return {
            sy_weburl: localStorage.getItem("sy_weburl"),
            searchForm: {},
            ruleForm: {
                config: lc('common.submit'),
                // Information review
                user_height_resume: false,
                user_idcard_status: false,
                user_msg_status: false,
                user_photo_status: false,
                rshow_photo_status: false,
                user_trust_status: false,
                resume_status: false,
                user_revise_state: false,
                resume_statetime_start: '',
                resume_statetime_end: '',
                user_resume_status: false,
                user_gzgzh: false,
                resume_kstd: '1',//1 2
                resume_create_exp: false,
                resume_create_edu: false,
                resume_create_project: false,
                expcreate: [],
                educreate: [],
                sy_resume_job_classid: [],
                sy_resume_kh_td: null,//1 0
                resume_kstd_req: null,//1 2
                user_sqintegrity: '',
                user_work_regiser: false,
                user_edu_regiser: false,
                user_project_regiser: false,
                sy_rname_num: '',
                user_number: '',
                user_finder: '',
                user_trust_number: '',
                user_name: null,
                user_pic: null,
                resume_sx: null,
                resume_open_check: null,
                sy_user_visit_resume: null,//1 0
                sy_shresume_applyjob: null,//1 0
                com_resume_partapply: null,//1 0
                sy_resumename_num: null,//1 0
                sq_resume_interval: '',
            },
            cache: {},
            submitLoading: false,

            jobSelected:null,
        }
    },
    components: {
        'job_class': httpVueLoader('../../../component/job_class.vue'),
    },
    created() {
        this.getList();
    },
    methods: {
        deltag(val){
            let index = this.ruleForm.sy_resume_job_classid.indexOf(val);
            let newArr = index !== -1 ? [...this.ruleForm.sy_resume_job_classid.slice(0, index), ...this.ruleForm.sy_resume_job_classid.slice(index + 1)] : this.ruleForm.sy_resume_job_classid;
            this.ruleForm.sy_resume_job_classid = newArr;

            let a = {};
            for(let i in newArr){
                a[newArr[i]] = this.cache.job_name[newArr[i]];
            }
            this.jobSelected = a;
        },
        confirmJob(data) {
            this.ruleForm.sy_resume_job_classid = data.jobId;
        },
        getList() {
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.searchForm));
            for (let index in params) {
                (params[index] === '') && (params[index] = null);
            }
            httpPost('m=user&c=users_userset&a=index', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let config = res.data.config ? res.data.config : {};
                    // ----InfoAudit
                    //{{ lc('wap_com_00375') }}
                    _this.ruleForm.user_height_resume = config.user_height_resume == "1" ? true : false;
                    //{{ lc('member_com_00014') }}
                    _this.ruleForm.user_idcard_status = config.user_idcard_status == "1" ? true : false;
                    //{{ lc('wap_com_00408') }}
                    _this.ruleForm.user_msg_status = config.user_msg_status == "0" ? true : false;
                    //{{ lc('member_user_00161') }}
                    _this.ruleForm.user_photo_status = config.user_photo_status == '1' ? true : false;
                    // work
                    _this.ruleForm.rshow_photo_status = config.rshow_photo_status == '1' ? true : false;
                    //{{ lc('member_user_00276') }}
                    _this.ruleForm.user_trust_status = config.user_trust_status == "0" ? true : false;
                    // ----ResumeAudit
                    //{{ lc('wap_user_00197') }}
                    _this.ruleForm.resume_status = config.resume_status == "0" ? true : false;
                    //{{ lc('wap_00269') }}
                    _this.ruleForm.user_revise_state = config.user_revise_state == '0' ? true : false;
                    _this.ruleForm.resume_statetime_start = config.resume_statetime_start !== undefined ? config.resume_statetime_start : '';
                    _this.ruleForm.resume_statetime_end = config.resume_statetime_end !== undefined ? config.resume_statetime_end : '';
                    //----{{ lc('admin_user_00337') }}
                    //{{ lc('wap_user_00197') }}
                    _this.ruleForm.user_resume_status = config.user_resume_status == "1" ? true : false;
                    //{{ lc('admin_user_company_00260') }}
                    _this.ruleForm.user_gzgzh = config.user_gzgzh == "1" ? true : false;
                    //----{{ lc('admin_user_00338') }}
                    _this.ruleForm.resume_kstd = config.resume_kstd !== undefined ? config.resume_kstd : null;
                    // ---- Required fields for resume creation
                    // Work experience
                    _this.ruleForm.resume_create_exp = config.resume_create_exp == '1' ? true : false;
                    // Educational experience
                    _this.ruleForm.resume_create_edu = config.resume_create_edu == '1' ? true : false;
                    // Project experience
                    _this.ruleForm.resume_create_project = config.resume_create_project == '1' ? true : false;
                    // Work experience options
                    if (config.expcreate !== undefined && config.expcreate.length) {
                        _this.ruleForm.expcreate = config.expcreate.split(',');
                    } else {
                        _this.ruleForm.expcreate = [];
                    }
                    // Education experience options
                    if (config.educreate !== undefined && config.educreate.length) {
                        _this.ruleForm.educreate = config.educreate.split(',');
                    } else {
                        _this.ruleForm.educreate = [];
                    }
                    //{{ lc('admin_00052') }}
                    if (config.sy_resume_job_classid !== undefined && config.sy_resume_job_classid.length) {
                        _this.ruleForm.sy_resume_job_classid = config.sy_resume_job_classid.split(",");
                        _this.jobSelected = JSON.parse(JSON.stringify(res.data.selected))
                    } else {
                        _this.ruleForm.sy_resume_job_classid = [];
                    }
                    // Whether to check completeness for cross-category applications: 1/0
                    _this.ruleForm.sy_resume_kh_td = config.sy_resume_kh_td !== undefined ? config.sy_resume_kh_td : null;
                    // {{ lc('admin_user_00338') }}, whether to follow resume required fields: 1/2
                    _this.ruleForm.resume_kstd_req = config.resume_kstd_req !== undefined ? config.resume_kstd_req : null;
                    //----{{ lc('admin_user_00318') }}
                    _this.ruleForm.user_sqintegrity = config.user_sqintegrity !== undefined ? config.user_sqintegrity : "";
                    //----{{ lc('admin_user_00331') }}
                    // Work experience
                    _this.ruleForm.user_work_regiser = config.user_work_regiser == '1' ? true : false;
                    // Educational experience
                    _this.ruleForm.user_edu_regiser = config.user_edu_regiser == '1' ? true : false;
                    // Project experience
                    _this.ruleForm.user_project_regiser = config.user_project_regiser == '1' ? true : false;
                    //{{ lc('admin_user_00325') }}
                    _this.ruleForm.sy_rname_num = config.sy_rname_num !== undefined ? config.sy_rname_num : "";
                    //{{ lc('admin_user_00321') }}
                    _this.ruleForm.user_number = config.user_number !== undefined ? config.user_number : "";
                    //{{ lc('admin_00564') }}
                    _this.ruleForm.user_finder = config.user_finder !== undefined ? config.user_finder : "";
                    //{{ lc('admin_user_00319') }}
                    _this.ruleForm.user_trust_number = config.user_trust_number !== undefined ? config.user_trust_number : "";
                    //{{ lc('wap_user_00238') }}
                    _this.ruleForm.user_name = config.user_name !== undefined ? config.user_name : null;
                    //{{ lc('admin_00566') }}
                    _this.ruleForm.user_pic = config.user_pic !== undefined ? config.user_pic : null;
                    //{{ lc('admin_00567') }}
                    _this.ruleForm.resume_sx = config.resume_sx !== undefined ? config.resume_sx : null;
                    //{{ lc('admin_user_00333') }}
                    _this.ruleForm.resume_open_check = config.resume_open_check !== undefined ? config.resume_open_check : null;
                    //{{ lc('admin_00569') }}
                    _this.ruleForm.sy_user_visit_resume = config.sy_user_visit_resume !== undefined ? config.sy_user_visit_resume : null;
                    //{{ lc('admin_user_00323') }}
                    _this.ruleForm.sy_shresume_applyjob = config.sy_shresume_applyjob !== undefined ? config.sy_shresume_applyjob : null;
                    //{{ lc('admin_user_00320') }}
                    _this.ruleForm.com_resume_partapply = config.com_resume_partapply !== undefined ? config.com_resume_partapply : null;
                    //{{ lc('admin_user_00330') }}
                    _this.ruleForm.sy_resumename_num = config.sy_resumename_num !== undefined ? config.sy_resumename_num : null;
                    //{{ lc('admin_user_00326') }}
                    _this.ruleForm.sq_resume_interval = config.sq_resume_interval !== undefined ? config.sq_resume_interval : '';

                    _this.getBaseData();
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        getBaseData() {
            let _this = this;
            httpPost('m=user&c=users_userset&a=indexBaseData', {}, {hideloading: true}).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    _this.cache = Object.freeze(res.data);
                }
            }).catch(function (error) {
                console.log(error);
            });
        },
        submitForm(formName) {
            // this.$refs[formName].validate((valid) => {if (valid) {}});
            let _this = this;
            let params = JSON.parse(JSON.stringify(this.ruleForm));
            params.user_height_resume = params.user_height_resume ? 1 : 2;
            params.user_idcard_status = params.user_idcard_status ? 1 : 0;
            params.user_msg_status = params.user_msg_status ? 0 : 1;
            params.user_photo_status = params.user_photo_status ? 1 : 2;
            params.rshow_photo_status = params.rshow_photo_status ? 1 : 0;
            params.user_trust_status = params.user_trust_status ? 0 : 1;
            params.resume_status = params.resume_status ? 0 : 1;
            params.user_revise_state = params.user_revise_state ? 0 : 1;
            params.user_resume_status = params.user_resume_status ? 1 : 0;
            params.user_gzgzh = params.user_gzgzh ? 1 : 0;
            params.resume_create_exp = params.resume_create_exp ? 1 : 0;
            params.resume_create_edu = params.resume_create_edu ? 1 : 0;
            params.resume_create_project = params.resume_create_project ? 1 : 0;
            params.expcreate = params.expcreate.join(',');
            params.educreate = params.educreate.join(',');
            params.sy_resume_job_classid = params.sy_resume_job_classid.join(',');
            params.user_work_regiser = params.user_work_regiser ? 1 : 0;
            params.user_edu_regiser = params.user_edu_regiser ? 1 : 0;
            params.user_project_regiser = params.user_project_regiser ? 1 : 0;
            _this.submitLoading = true;
            httpPost('m=user&c=users_userset&a=save', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    message.success(res.msg);
                    _this.resetForm();
                    _this.getList();
                } else {
                    message.error(res.msg);
                }
            }).catch(function (error) {
                console.log(error);
            }).finally(function () {
                _this.submitLoading = false;
            });
        },
        resetForm(formName) {
            //this.$refs[formName].resetFields();
        },
    },
    watch: {
        "ruleForm.user_sqintegrity": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9.]/g, '');
                    this.ruleForm.user_sqintegrity = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.sy_rname_num": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.sy_rname_num = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.user_number": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.user_number = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.user_finder": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.user_finder = newValue;
                }
            },
            deep: true,
            immediate: true
        },
        "ruleForm.user_trust_number": {
            handler(newValue, oldValue) {
                if (typeof (newValue) == 'string') {
                    newValue = newValue.replace(/[^0-9]/g, '');
                    this.ruleForm.user_trust_number = newValue;
                }
            },
            deep: true,
            immediate: true
        },
    }
};
</script>
<style scoped>
.content-sub {
    padding: 8px 10px;
    border-radius: 3px;
    background-color: #f5f7fb;
    border: 1px solid #e6e6e6;
    position: relative;
    overflow: hidden;
    margin: 6px 0;
}

.content-sub p {
    font-size: 14px;
    font-weight: 600;
}
.contentRadio{
    overflow: hidden;
    position: relative;
    padding: 6px 0;
}
</style>
