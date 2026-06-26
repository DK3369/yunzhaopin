<template>
    <!--会员-个人-个人设置：个人设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">{yun:}t key='member_com_00021'{/yun}</th>
                    <th width="680">{yun:}t key='member_user_00181'{/yun}</th>
                    <th>{yun:}t key='member_com_00207'{/yun}</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00336'{/yun}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_height_resume" label="{yun:}t key='wap_com_00375'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_idcard_status" label="{yun:}t key='wap_01030'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_msg_status" label="{yun:}t key='wap_com_00408'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_photo_status" label="{yun:}t key='member_user_00161'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.rshow_photo_status" label="{yun:}t key='admin_user_00190'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_trust_status" label="{yun:}t key='member_user_00276'{/yun}"></el-checkbox>
                        </div>
                    
                    </td>
                    <td>
                        <div class="TableShuom">
                            {yun:}t key='admin_00557'{/yun}
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00251'{/yun}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_status" label="{yun:}t key='wap_user_00197'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_revise_state" label="{yun:}t key='wap_00269'{/yun}"></el-checkbox>
                        </div>
                        <div v-if="ruleForm.resume_status == true || ruleForm.user_revise_state == true"
                             class="content-sub">
                            <div class="TableShuom" style="margin-bottom: 8px">
                                {yun:}t key='admin_00558'{/yun}
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
                        <div class="TableTite">{yun:}t key='admin_user_00337'{/yun}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_resume_status" label="{yun:}t key='wap_user_00197'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_gzgzh" label="{yun:}t key='wap_user_00191'{/yun}"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00338'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_kstd" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.resume_kstd" label="2">{yun:}t key='common.close'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_user_00327'{/yun}</div>
                        <!--<div style="color:red;">这一块要重新写样式</div>-->
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_exp" label="{yun:}t key='wap_00457'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_edu" label="{yun:}t key='wap_00459'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_project" label="{yun:}t key='wap_00465'{/yun}"></el-checkbox>
                        </div>
                        <div class="content-sub"
                             v-if="ruleForm.resume_create_exp == true && cache.hasOwnProperty('userdata')">
                            <el-checkbox-group v-model="ruleForm.expcreate">
                                <div v-for="(item,key) in cache.userdata.user_word" :key="key" class="tc_checkbox">
                                    <el-checkbox :label="item">{{ cache.userclass_name[item] }}</el-checkbox>
                                </div>
                            </el-checkbox-group>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info"> {yun:}t key='admin_00559'{/yun}
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
                                    {yun:}t key='admin_00560'{/yun}
                                </div>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>{yun:}t key='admin_user_00312'{/yun}</p>
                            <div class="TableShuom tc_checktip">
                                {yun:}t key='admin_00561'{/yun}
                            </div>
                            <!--TODO 7.0 统一类别选择-->
                            <!-- <el-button>设置不强制填写职位类别项</el-button> -->
                            <job_class ref="job_class" multiple :max="100" @confirm="confirmJob" :showsearch="false" :selected="jobSelected"></job_class>
                            <div style="overflow: hidden; position: relative; margin-top: 8px" v-if="ruleForm.sy_resume_job_classid && cache.hasOwnProperty('job_name')">
                                <el-tag style="margin:5px 5px 0 0;" v-for="(item,key) in ruleForm.sy_resume_job_classid" :key="key" @close="deltag(item)" closable>{{ cache.job_name[item] }}</el-tag>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>{yun:}t key='admin_user_00316'{/yun}</p>
                            <div class="contentRadio">
                                <el-radio v-model="ruleForm.sy_resume_kh_td" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                <el-radio v-model="ruleForm.sy_resume_kh_td" label="0">{yun:}t key='common.close'{/yun}</el-radio>
                            </div>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    {yun:}t key='admin_00562'{/yun}
                                </div>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>{yun:}t key='admin_user_00317'{/yun}</p>
                            <div class="contentRadio">
                                <el-radio v-model="ruleForm.resume_kstd_req" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                                <el-radio v-model="ruleForm.resume_kstd_req" label="2">{yun:}t key='common.close'{/yun}</el-radio>
                            </div>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    {yun:}t key='admin_00563'{/yun}
                                </div>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {yun:}t key='admin_user_00315'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00318'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_sqintegrity" placeholder="{yun:}t key='admin_user_00342'{/yun}">
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
                        <div class="TableTite">{yun:}t key='admin_user_00331'{/yun}</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_work_regiser" label="{yun:}t key='wap_00457'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_edu_regiser" label="{yun:}t key='wap_00459'{/yun}"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_project_regiser" label="{yun:}t key='wap_00465'{/yun}"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00325'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_rname_num" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='common_02073'{/yun}</span>
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
                        <div class="TableTite">{yun:}t key='admin_user_00321'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_number" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {yun:}t key='admin_user_00332'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00564'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_finder" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='common_02050'{/yun}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {yun:}t key='admin_user_00314'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00319'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_trust_number" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='common_02052'{/yun}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> {yun:}t key='admin_user_00322'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='wap_user_00238'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.user_name" label="1">{yun:}t key='admin_00565'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="2">{yun:}t key='member_com_00345'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="3">{yun:}t key='admin_user_00339'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="4">{yun:}t key='member_user_00230'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">{yun:}t key='admin_user_00311'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00566'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.user_pic" label="1">{yun:}t key='admin_00565'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.user_pic" label="2">{yun:}t key='member_com_00023'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.user_pic" label="3">{yun:}t key='admin_user_00340'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_00567'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_sx" label="1">{yun:}t key='admin_user_00329'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.resume_sx" label="2">{yun:}t key='admin_user_00328'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_user_00333'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_open_check" label="1">{yun:}t key='common.close'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="2">{yun:}t key='admin_user_00335'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="3">{yun:}t key='common.publish_job'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="4">{yun:}t key='admin_00568'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">
                                {yun:}t key='admin_user_00306'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_00569'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_user_visit_resume" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.sy_user_visit_resume" label="0">{yun:}t key='common.close'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">{yun:}t key='admin_user_00313'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00323'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_shresume_applyjob" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.sy_shresume_applyjob" label="0">{yun:}t key='common.close'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">{yun:}t key='admin_00570'{/yun}</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00320'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_resume_partapply" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.com_resume_partapply" label="0">{yun:}t key='common.close'{/yun}</el-radio>
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
                        <div class="TableTite">{yun:}t key='admin_user_00330'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_resumename_num" label="1">{yun:}t key='member_com_00287'{/yun}</el-radio>
                            <el-radio v-model="ruleForm.sy_resumename_num" label="0">{yun:}t key='common.close'{/yun}</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">
                                {yun:}t key='admin_00571'{/yun}
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">{yun:}t key='admin_user_00326'{/yun}</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sq_resume_interval" placeholder="{yun:}t key='admin_user_00342'{/yun}">
                                <span slot="suffix" class="slotspan">{yun:}t key='common_02067'{/yun}</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>{yun:}t key='admin_user_00310'{/yun}</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">{yun:}t key='common.submit'{/yun}
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
                config: "{yun:}t key='common.submit'{/yun}",
                //信息审核
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
            httpPost('m=user&c=users_userset&a=index", params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let config = res.data.config ? res.data.config : {};
                    // ----InfoAudit
                    //{yun:}t key='wap_com_00375'{/yun}
                    _this.ruleForm.user_height_resume = config.user_height_resume == "1" ? true : false;
                    //{yun:}t key='member_com_00014'{/yun}
                    _this.ruleForm.user_idcard_status = config.user_idcard_status == "1" ? true : false;
                    //{yun:}t key='wap_com_00408'{/yun}
                    _this.ruleForm.user_msg_status = config.user_msg_status == "0" ? true : false;
                    //{yun:}t key='member_user_00161'{/yun}
                    _this.ruleForm.user_photo_status = config.user_photo_status == "1' ? true : false;
                    // work
                    _this.ruleForm.rshow_photo_status = config.rshow_photo_status == '1" ? true : false;
                    //{yun:}t key='member_user_00276'{/yun}
                    _this.ruleForm.user_trust_status = config.user_trust_status == "0" ? true : false;
                    // ----ResumeAudit
                    //{yun:}t key='wap_user_00197'{/yun}
                    _this.ruleForm.resume_status = config.resume_status == "0" ? true : false;
                    //{yun:}t key='wap_00269'{/yun}
                    _this.ruleForm.user_revise_state = config.user_revise_state == "0' ? true : false;
                    _this.ruleForm.resume_statetime_start = config.resume_statetime_start !== undefined ? config.resume_statetime_start : '';
                    _this.ruleForm.resume_statetime_end = config.resume_statetime_end !== undefined ? config.resume_statetime_end : '";
                    //----{yun:}t key='admin_user_00337'{/yun}
                    //{yun:}t key='wap_user_00197'{/yun}
                    _this.ruleForm.user_resume_status = config.user_resume_status == "1" ? true : false;
                    //{yun:}t key='admin_user_company_00260'{/yun}
                    _this.ruleForm.user_gzgzh = config.user_gzgzh == "1" ? true : false;
                    //----{yun:}t key='admin_user_00338'{/yun}
                    _this.ruleForm.resume_kstd = config.resume_kstd !== undefined ? config.resume_kstd : null;
                    //----简历创建必填项
                    // Work experience
                    _this.ruleForm.resume_create_exp = config.resume_create_exp == "1' ? true : false;
                    // Educational experience
                    _this.ruleForm.resume_create_edu = config.resume_create_edu == '1' ? true : false;
                    // Project experience
                    _this.ruleForm.resume_create_project = config.resume_create_project == '1' ? true : false;
                    //工作经历选项
                    if (config.expcreate !== undefined && config.expcreate.length) {
                        _this.ruleForm.expcreate = config.expcreate.split(',');
                    } else {
                        _this.ruleForm.expcreate = [];
                    }
                    //教育经历选项
                    if (config.educreate !== undefined && config.educreate.length) {
                        _this.ruleForm.educreate = config.educreate.split(',");
                    } else {
                        _this.ruleForm.educreate = [];
                    }
                    //{yun:}t key='admin_00052'{/yun}
                    if (config.sy_resume_job_classid !== undefined && config.sy_resume_job_classid.length) {
                        _this.ruleForm.sy_resume_job_classid = config.sy_resume_job_classid.split(",");
                        _this.jobSelected = JSON.parse(JSON.stringify(res.data.selected))
                    } else {
                        _this.ruleForm.sy_resume_job_classid = [];
                    }
                    //跨职位类别投递，是否要判断完整度  1 0
                    _this.ruleForm.sy_resume_kh_td = config.sy_resume_kh_td !== undefined ? config.sy_resume_kh_td : null;
                    //{yun:}t key='admin_user_00338'{/yun}，是否遵从简历必填项  1 2
                    _this.ruleForm.resume_kstd_req = config.resume_kstd_req !== undefined ? config.resume_kstd_req : null;
                    //----{yun:}t key='admin_user_00318'{/yun}
                    _this.ruleForm.user_sqintegrity = config.user_sqintegrity !== undefined ? config.user_sqintegrity : "";
                    //----{yun:}t key='admin_user_00331'{/yun}
                    // Work experience
                    _this.ruleForm.user_work_regiser = config.user_work_regiser == "1' ? true : false;
                    // Educational experience
                    _this.ruleForm.user_edu_regiser = config.user_edu_regiser == '1' ? true : false;
                    // Project experience
                    _this.ruleForm.user_project_regiser = config.user_project_regiser == '1" ? true : false;
                    //{yun:}t key='admin_user_00325'{/yun}
                    _this.ruleForm.sy_rname_num = config.sy_rname_num !== undefined ? config.sy_rname_num : "";
                    //{yun:}t key='admin_user_00321'{/yun}
                    _this.ruleForm.user_number = config.user_number !== undefined ? config.user_number : "";
                    //{yun:}t key='admin_00564'{/yun}
                    _this.ruleForm.user_finder = config.user_finder !== undefined ? config.user_finder : "";
                    //{yun:}t key='admin_user_00319'{/yun}
                    _this.ruleForm.user_trust_number = config.user_trust_number !== undefined ? config.user_trust_number : "";
                    //{yun:}t key='wap_user_00238'{/yun}
                    _this.ruleForm.user_name = config.user_name !== undefined ? config.user_name : null;
                    //{yun:}t key='admin_00566'{/yun}
                    _this.ruleForm.user_pic = config.user_pic !== undefined ? config.user_pic : null;
                    //{yun:}t key='admin_00567'{/yun}
                    _this.ruleForm.resume_sx = config.resume_sx !== undefined ? config.resume_sx : null;
                    //{yun:}t key='admin_user_00333'{/yun}
                    _this.ruleForm.resume_open_check = config.resume_open_check !== undefined ? config.resume_open_check : null;
                    //{yun:}t key='admin_00569'{/yun}
                    _this.ruleForm.sy_user_visit_resume = config.sy_user_visit_resume !== undefined ? config.sy_user_visit_resume : null;
                    //{yun:}t key='admin_user_00323'{/yun}
                    _this.ruleForm.sy_shresume_applyjob = config.sy_shresume_applyjob !== undefined ? config.sy_shresume_applyjob : null;
                    //{yun:}t key='admin_user_00320'{/yun}
                    _this.ruleForm.com_resume_partapply = config.com_resume_partapply !== undefined ? config.com_resume_partapply : null;
                    //{yun:}t key='admin_user_00330'{/yun}
                    _this.ruleForm.sy_resumename_num = config.sy_resumename_num !== undefined ? config.sy_resumename_num : null;
                    //{yun:}t key='admin_user_00326'{/yun}
                    _this.ruleForm.sq_resume_interval = config.sq_resume_interval !== undefined ? config.sq_resume_interval : "';

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
