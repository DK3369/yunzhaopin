<template>
    <div class="shbox" v-loading="loading" style="overflow-y: auto;">
        <div class="shshow_tit">
            <i class="el-icon-office-building"></i> {yun:}t key='wap_user_00341'{/yun}
            <span class="shshow_cz">
                <el-button type="text" @click="openBasic">
                    <i class="el-icon-edit"></i>{yun:}t key='admin_user_00227'{/yun}
                </el-button>
            </span>
        </div>
        <div class="userinfo_box">
            <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"></div>
            <div class="userinfo_r">
                <div class="userinfo_name">{{resume.name}}</div>
                <div class="userinfo">
                    {{ resume.sex_n }}
                    <span v-if="resume.age">{yun:}t key='admin_user_00198'{/yun}</span>
                    <span v-if="resume.height">，{{ resume.height }}cm</span>
                    <span v-if="resume.weight">，{{ resume.weight }}kg</span>
                    <span v-if="resume.marriage_n">，{{ resume.marriage_n }}</span>
                    <span v-if="resume.living">{yun:}t key='admin_00468'{/yun}</span>
                </div>
                <div class="userinfo" v-if="resume.edu_n || resume.exp_n">
                    <span v-if="resume.edu_n">{yun:}t key='admin_00469'{/yun} </span>
                    <span class="userline" v-if="resume.edu_n && resume.exp_n">|</span>
                    <span v-if="resume.exp_n">{yun:}t key='admin_00470'{/yun}</span>
                </div>
            </div>
        </div>
        <div class="shshow_p">
            <div class="cominfo" v-if="resume.telphone"><i class="el-icon-mobile"></i>
                {{ lc("admin_contact_phone_value", [resume.telphone]) }}</div>
            <div class="cominfo" v-if="resume.email"><i class="el-icon-message"></i>
                {{ lc("admin_email_value", [resume.email]) }}</div>
            <div class="cominfo" v-if="resume.idcard"><i class="el-icon-postcard"></i>
                {{ lc("admin_idcard_value", [resume.idcard]) }}</div>
            <div class="cominfo" v-if="resume.domicile"><i class="el-icon-location-outline"></i>
                {{ lc("admin_domicile_value", [resume.domicile]) }}</div>
            <div class="cominfo" v-if="resume.address"><i class="el-icon-location-information"></i>
                {{ lc("admin_detail_address_value", [resume.address]) }}</div>
        </div>

        <!--个人优势-->
        <div class="user_resume_list">
            <div class="shshow_tit">
                <i class="el-icon-medal-1"></i> {yun:}t key='wap_user_00326'{/yun}
            </div>
            <div class="shshow_p">
                <el-tag size="mini" v-for="(tagItem,key) in resume.arrayTag" :key="key">{{tagItem}}</el-tag>
                <div class="cominfo">{{resume.description}}</div>
            </div>
            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00196'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openTag">
                        <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                        resume.arrayTag.length > 0) || resume.description ? '{yun:}t key='common.edit'{/yun}' : '{yun:}t key='wap_js_00091'{/yun}' }}
                    </el-button>
                </div>
            </div>
        </div>
        <!--求职意向-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-notebook-2"></i> {yun:}t key='wap_00460'{/yun}</div>
            <div class="shshow_p" v-if="expectData.expect">
                <div class="cominfo">{{ lc("admin_expected_position_value", [expectData.expect.name]) }} </div>
                <div class="cominfo">{{ lc("admin_current_position_value", [expectData.expect.job_classname]) }}</div>
                <div class="cominfo">{{ lc("admin_expected_location_value", [expectData.expect.city_classname]) }}</div>
                <div class="cominfo">{{ lc("admin_expected_salary_value", [expectData.expect.salary]) }}</div>
                <div class="cominfo">{{ lc("admin_industry_value", [expectData.expect.hy_n]) }}</div>
                <div class="cominfo">{{ lc("admin_arrival_time_value", [expectData.expect.report_n]) }}</div>
                <div class="cominfo">{{ lc("admin_work_nature_value", [expectData.expect.type_n]) }}</div>
                <div class="cominfo">{{ lc("admin_job_status_value", [expectData.expect.jobstatus_n]) }}</div>
            </div>


            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00205'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openJob">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='admin_00472'{/yun}
                    </el-button>
                </div>
            </div>
        </div>

        <!--工作经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-suitcase-1"></i> {yun:}t key='wap_00457'{/yun}</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(work, workkey) in expectData.work" :key="workkey">
                <div class="user_resume_addname ">{{work.name}}
                    <el-button type="text" @click="openWork(workkey)">
                        <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                    </el-button>
                    <el-button type="text" @click="delResumeFb('work', workkey, work.id)">
                        <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                    </el-button>
                </div>
                <div class="user_resume_addjy">
                    <div class=" ">{{work.title}}</div>
                    <div class="user_resume_time">{{work.sdate_n}}-{{work.edate_n}}</div>
                </div>
                <div class="user_resume_ms">{{work.content}}</div>
            </div>
            <!--循环-->
            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00195'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openWork('')">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                    </el-button>
                </div>
            </div>
        </div>
        <!--教育经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-school"></i> {yun:}t key='wap_00459'{/yun}</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu" :key="edukey">
                <div class="user_resume_addname ">{{edu.name}}
                    <el-button type="text" @click="openEdu(edukey)">
                        <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                    </el-button>
                    <el-button type="text" @click="delResumeFb('edu', edukey, edu.id)">
                        <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                    </el-button>
                </div>
                <div class="user_resume_addjy">
                    <div class=" ">{{ edu.specialty }}<span class="userline"
                                                            v-if="edu.specialty && edu.education_n">|</span>{{ edu.education_n }}</div>
                    <div class="user_resume_time">{{edu.sdate_n}}-{{edu.edate_n}}</div>
                </div>
            </div>
            <!--循环-->
            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00202'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openEdu('')">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                    </el-button>
                </div>
            </div>
        </div>
        <!--培训经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-data-analysis"></i> {yun:}t key='wap_00455'{/yun}</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training" :key="trainingKey">
                <div class="user_resume_addname ">{{training.name}}
                    <el-button type="text" @click="openTraining(trainingKey)">
                        <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                    </el-button>
                    <el-button type="text" @click="delResumeFb('training', trainingKey, training.id)">
                        <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                    </el-button>
                </div>
                <div class="user_resume_addjy">
                    <div class=" ">{{training.title}} </div>
                    <div class="user_resume_time">{{training.sdate_n}}-{{training.edate_n}}</div>
                </div>
                <div class="user_resume_ms">{{training.content}}</div>
            </div>
            <!--循环-->

            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00197'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openTraining('')">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                    </el-button>
                </div>
            </div>
        </div>
        <!--职业技能-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-reading"></i> {yun:}t key='wap_00461'{/yun}</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill" :key="skillkey">
                <div class="user_resume_addname ">{{skill.name}}
                    <el-button type="text" @click="openSkill(skillkey)">
                        <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                    </el-button>
                    <el-button type="text" @click="delResumeFb('skill', skillkey, skill.id)">
                        <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                    </el-button>
                </div>
                <div class="user_resume_addjy">
                    <div class=" ">{{skill.ing_n}} </div>
                    <div class="user_resume_time">{yun:}t key='admin_00473'{/yun}</div>
                </div>
                <div class="user_resume_ms" v-if="skill.pic">
                    <img :src="skill.pic" width="95" height="70" :preview-src-list="skill.pic">
                </div>
            </div>
            <!--循环-->

            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00199'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openSkill('')">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                    </el-button>
                </div>
            </div>
        </div>
        <!--项目经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-wallet"></i> {yun:}t key='wap_00465'{/yun}</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(project, projectkey) in expectData.project" :key="projectkey">
                <div class="user_resume_addname ">{{project.name}}
                    <el-button type="text" @click="openProject(projectkey)">
                        <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                    </el-button>
                    <el-button type="text" @click="delResumeFb('project', projectkey, project.id)">
                        <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                    </el-button>
                </div>
                <div class="user_resume_addjy">
                    <div class=" ">{{project.title}}</div>
                    <div class="user_resume_time">{{project.sdate_n}}-{{project.edate_n}}</div>
                </div>
                <div class="user_resume_ms">{{project.content}}</div>
            </div>
            <!--循环-->

            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00194'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openProject('')">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                    </el-button>
                </div>
            </div>
        </div>
        <!--其他描述-->
        <div class="user_resume_list" style="padding-bottom:80px; ;">
            <div class="shshow_tit"><i class="el-icon-mic"></i> {yun:}t key='admin_00068'{/yun}</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(other, otherkey) in expectData.other" :key="otherkey">
                <div class="user_resume_addname ">{{other.name}}
                    <el-button type="text" @click="openOther(otherkey)">
                        <i class="el-icon-edit"></i> {yun:}t key='wap_js_00073'{/yun}
                    </el-button>
                    <el-button type="text" @click="delResumeFb('other', otherkey, other.id)">
                        <i class="el-icon-delete"></i> {yun:}t key='common.delete'{/yun}
                    </el-button>
                </div>
                <div class="user_resume_ms">{{other.content}}</div>
            </div>
            <!--循环-->
            <div class="user_resume_add">
                <div class="">{yun:}t key='admin_user_00215'{/yun}</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openOther('')">
                        <i class="el-icon-circle-plus-outline"></i> {yun:}t key='wap_js_00091'{/yun}
                    </el-button>
                </div>
            </div>
        </div>
        <!---编辑简历 基本资料-->
        <el-drawer title="{yun:}t key='admin_00475'{/yun}" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                    <tr align="left">
                        <th width="120">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width=" ">{yun:}t key='member_user_00181'{/yun}</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_00529'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.name" placeholder="{yun:}t key='wap_user_00234'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_com_00303'{/yun}</div>
                        </td>
                        <td>
                            <div class="job_set_list">
                                <el-radio-group v-model="ruleFormBasic.sex">
                                    <el-radio v-for="(sex, sexkey) in user_sex" :label="sexkey" :key="sexkey">{{sex}}</el-radio>
                                </el-radio-group>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00236'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-date-picker v-model="ruleFormBasic.birthday" type="month" placeholder="{yun:}t key='admin_user_00192'{/yun}">
                                </el-date-picker>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00092'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormBasic.edu" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="edukey in userdata.user_edu" :key="edukey"
                                               :label="userclass_name[edukey]" :value="edukey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00240'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormBasic.exp" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="wordkey in userdata.user_word" :key="wordkey"
                                               :label="userclass_name[wordkey]" :value="wordkey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00265'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.telphone" placeholder="{yun:}t key='wap_com_00322'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_com_00016'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.email" placeholder="{yun:}t key='wap_com_00009'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00173'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.idcard" placeholder="{yun:}t key='admin_00476'{/yun}"
                                          @input="inputIdcard($event, 'ruleFormBasic', 'idcard')"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='member_user_00158'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.domicile" placeholder="{yun:}t key='admin_00477'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_user_00230'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.living" placeholder="{yun:}t key='admin_00478'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_01362'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.address" placeholder="{yun:}t key='wap_00905'{/yun}"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='member_user_00165'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.height" placeholder="{yun:}t key='admin_00479'{/yun}"
                                          @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='member_user_00160'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.weight" placeholder="{yun:}t key='admin_00480'{/yun}"
                                          @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_com_00282'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormBasic.marriage" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="marriagekey in userdata.user_marriage" :key="marriagekey"
                                               :label="userclass_name[marriagekey]" :value="marriagekey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='member_user_00164'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.nationality" placeholder="{yun:}t key='admin_00481'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='member_user_00155'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.homepage" placeholder="{yun:}t key='admin_00482'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">QQ</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.qq" placeholder="{yun:}t key='admin_user_00217'{/yun}"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='resume_00003'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action=""
                                           :auto-upload="false" :on-change="handleChangeWxewm" :show-file-list="false">
                                    <img v-if="ruleFormBasic.wxewm_n" :src="ruleFormBasic.wxewm_n" class="avatar">
                                    <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                                </el-upload>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_00527'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="{yun:}t key='admin_user_00208'{/yun}"
                                          v-model="ruleFormBasic.description">
                                </el-input>
                            </div>
                        </td>
                    </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitBasic">{yun:}t key='common.submit'{/yun}</el-button>
            </div>


        </el-drawer>
        <!---编辑求职意向-->
        <el-drawer title="{yun:}t key='admin_00483'{/yun}" :append-to-body="true" :visible.sync="drawerJob" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                    <tr align="left">
                        <th width="120">{yun:}t key='member_com_00021'{/yun}</th>
                        <th width=" ">{yun:}t key='member_user_00181'{/yun}</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00015'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormJob.name" placeholder="{yun:}t key='admin_00484'{/yun}">
                                </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_user_00218'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <!--7.0 统一类别选择-->
                                <job_class multiple :max="5" @confirm="confirmJob" :selected="jobSelected"></job_class>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='admin_user_00226'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <!--7.0 统一城市选择-->
                                <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected"></city_class>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00016'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableInpt" style="max-width: 700px;">
                                <el-select v-model="ruleFormJob.minsalary" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="salaryChange" style="margin-right:8px;">
                                    <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val" :label="maxsalary1Val" :value="maxsalary1Val">
                                    </el-option>
                                </el-select>
                                <el-select v-model="ruleFormJob.maxsalary" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val" :label="maxsalary2Val" :value="maxsalary2Val">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00010'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.hy" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="industrykey in industry_index" :key="industrykey"
                                               :label="industry_name[industrykey]" :value="industrykey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_com_00279'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.report" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="reportkey in userdata.user_report" :key="reportkey"
                                               :label="userclass_name[reportkey]" :value="reportkey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00012'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.type" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="typekey in userdata.user_type" :key="typekey"
                                               :label="userclass_name[typekey]" :value="typekey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">{yun:}t key='wap_user_00017'{/yun}</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.jobstatus" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                                    <el-option v-for="jobstatuskey in userdata.user_jobstatus" :key="jobstatuskey"
                                               :label="userclass_name[jobstatuskey]" :value="jobstatuskey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitJob">{yun:}t key='common.submit'{/yun}</el-button>
            </div>
        </el-drawer>

        <!---编辑个人优势-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_user_00326'{/yun}" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00219'{/yun}</div>
                    <div class="wxsettipBiaoqin">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                                @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input class="input-new-tag" v-if="inputTag" v-model="tagval" autofoucs size="small"
                                  @keyup.enter.native="confirmTag">
                        </el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showTag">{yun:}t key='admin_00474'{/yun}
                        </el-button>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_00463'{/yun}</div>
                    <el-input type="textarea"
                              :placeholder="{yun:}t key='admin_vue_00011'{/yun}"
                              v-model="ruleFormTag.description" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogTag = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitTag">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑工作经历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00457'{/yun}" :visible.sync="dialogWork" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_01403'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormWork.name" placeholder="{yun:}t key='wap_00137'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00091'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormWork.title" placeholder="{yun:}t key='wap_user_00045'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00223'{/yun}</div>
                    <div class="wxsettip_Sealect" style="display: flex; align-items: center;">
                        <el-date-picker v-model="ruleFormWork.sdate" type="month" placeholder="{yun:}t key='wap_com_00323'{/yun}">
                        </el-date-picker>
                        <el-date-picker style="margin: 0 8px;" :disabled="todayCheck" v-model="ruleFormWork.edate" type="month"
                                        placeholder="{yun:}t key='wap_com_00324'{/yun}">
                        </el-date-picker>
                        <el-checkbox v-model="todayCheck" @change="todayChange($event, 'work')">{yun:}t key='wap_js_00170'{/yun}</el-checkbox>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00086'{/yun}</div>
                    <el-input type="textarea" :placeholder="{yun:}t key='admin_vue_00012'{/yun}"
                              v-model="ruleFormWork.content" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>

                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogWork = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitWork">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑学历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00459'{/yun}" :visible.sync="dialogEdu" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00085'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormEdu.name" placeholder="{yun:}t key='wap_user_00044'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00220'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeEdu" type="monthrange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                                        start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00092'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormEdu.education" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option v-for="edukey in userdata.user_edu" :key="edukey" :label="userclass_name[edukey]"
                                       :value="edukey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00224'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormEdu.specialty" placeholder="{yun:}t key='wap_user_00042'{/yun}"></el-input> </div>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogEdu = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitEdu">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>

        <!---编辑培训经历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00455'{/yun}" :visible.sync="dialogTraining" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00221'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormTraining.name" placeholder="{yun:}t key='admin_00485'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00083'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormTraining.title" placeholder="{yun:}t key='admin_user_00209'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00222'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeTraining" type="monthrange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                                        start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00082'{/yun}</div>
                    <el-input type="textarea" placeholder="{yun:}t key='admin_user_00200'{/yun}" v-model="ruleFormTraining.content"
                              :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogTraining = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitTraining">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑项目经历-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00465'{/yun}" :visible.sync="dialogProject" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00099'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormProject.name" placeholder="{yun:}t key='wap_user_00046'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00225'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormProject.title" placeholder="{yun:}t key='admin_00486'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00229'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeProject" type="monthrange" range-separator="{yun:}t key='admin_company_00019'{/yun}"
                                        start-placeholder="{yun:}t key='admin_00343'{/yun}" end-placeholder="{yun:}t key='admin_00344'{/yun}">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00228'{/yun}</div>
                    <el-input type="textarea" :placeholder="{yun:}t key='admin_vue_00012'{/yun}" v-model="ruleFormProject.content" :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogProject = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitProject">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑其他-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='admin_user_00216'{/yun}" :visible.sync="dialogOther" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00103'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormOther.name" placeholder="{yun:}t key='admin_00487'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='admin_user_00231'{/yun}</div>
                    <el-input type="textarea" v-model="ruleFormOther.content" placeholder="{yun:}t key='admin_user_00203'{/yun}"
                              :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogOther = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitOther">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑技能-->
        <div class="modluDrawer">
            <el-dialog title="{yun:}t key='wap_00461'{/yun}" :visible.sync="dialogSkill" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00089'{/yun}</div>
                    <div class=""><el-input v-model="ruleFormSkill.name" placeholder="{yun:}t key='admin_user_00210'{/yun}"></el-input> </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_00458'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-input v-model="ruleFormSkill.longtime" placeholder="{yun:}t key='admin_user_00211'{/yun}">
                            <template slot="append">{yun:}t key='common_02077'{/yun}</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00094'{/yun}</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormSkill.ing" placeholder="{yun:}t key='wap_user_00100'{/yun}">
                            <el-option v-for="ingkey in userdata.user_ing" :key="ingkey" :label="userclass_name[ingkey]"
                                       :value="ingkey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">{yun:}t key='wap_user_00090'{/yun}</div>
                    <div>
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                                   :on-change="handleChangeSkillPic" :show-file-list="false">
                            <img v-if="ruleFormSkill.pic_n" :src="ruleFormSkill.pic_n" class="avatar">
                            <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                        </el-upload>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogSkill = false">{yun:}t key='admin_user_weipin_00043'{/yun}</el-button>
					<el-button type="primary" @click="submitSkill">{yun:}t key='wap_com_00019'{/yun}</el-button>
                </span>
            </el-dialog>
        </div>
    </div>
</template>

<script>
    module.exports = {
        props: {
            id: String,
            uid: Number
        },
        data: function () {
            return {
                pic_accept: localStorage.getItem("pic_accept"),
                loading: true,
                saveLoading: false,
                refreshList: false,

                resume: {},
                expectData: {},

                // 缓存
                user_sex: {},
                userclass_name: {},
                userdata: {},
                industry_index: [],
                industry_name: {},

                eid: 0, // 简历ID

                // 编辑基本资料
                drawerBasic: false,
                ruleFormBasic: {},
                // personal advantage
                dialogTag: false,
                ruleFormTag: {},
                userTag: [],
                inputTag: false,
                tagval: '",
                // Job intention
                drawerJob: false,
                ruleFormJob: {},
                jobSelected: null,
                citySelected: null,
                minsalaryList: [],
                maxsalaryList: [],

                todayCheck: false, // 至今选中

                // Work experience
                dialogWork: false,
                indexWork: -1,
                ruleFormWork: {},
                // Educational experience
                dialogEdu: false,
                indexEdu: -1,
                daterangeEdu: [],
                ruleFormEdu: {},
                // Training experience
                dialogTraining: false,
                indexTraining: -1,
                daterangeTraining: [],
                ruleFormTraining: {},
                // 技能提升
                dialogSkill: false,
                indexSkill: -1,
                ruleFormSkill: {},
                // Project experience
                dialogProject: false,
                indexProject: -1,
                daterangeProject: [],
                ruleFormProject: {},
                // {yun:}t key='admin_00068'{/yun}
                dialogOther: false,
                indexOther: -1,
                ruleFormOther: {},
            }
        },
        components: {
            "job_class': httpVueLoader('../../../component/job_class.vue'),
            'city_class': httpVueLoader('../../../component/city_class.vue'),
        },
        created() {
            this.getInfo();
        },
        methods: {
            async getInfo() {
                let params = {};
                if (typeof this.uid !== "undefined") {
                    params.uid = this.uid;
                }
                if (typeof this.id !== "undefined" || this.eid > 0) {
                    params.id = this.eid > 0 ? this.eid : this.id;
                }

                let response = await httpPost('m=user&c=users_resume&a=editResume', params, {hideLoading: true});
                let res = response.data,
                    data = res.data;

                this.resume = data.resume ? data.resume : {};
                this.expectData = data.expectData;

                this.user_sex = data.user_sex;
                this.userclass_name = data.userclass_name;
                this.userdata = data.userdata;
                this.industry_index = data.industry_index;
                this.industry_name = data.industry_name;
                this.loading = false;

                if (typeof this.id === "undefined" && this.eid == 0) { // 新增简历时，优先弹出求职意向框
                    this.openJob();
                }
            },

            inputIntNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9]/g,'');
            },
            inputFloatNumber(val, form, key) {
                this.$data[form][key] = val.replace(/[^0-9.]/g, '");
            },

            // {yun:}t key='admin_user_00227'{/yun}
            openBasic() {
                let resume = this.resume;
                this.ruleFormBasic = {
                    uid: resume.uid,
                    name: resume.name,
                    sex: resume.sex,
                    birthday: resume.birthday ? new Date(resume.birthday) : "',
                    edu: resume.edu && resume.edu > 0 ? resume.edu : '',
                    exp: resume.exp && resume.exp > 0 ? resume.exp : '',
                    telphone: resume.telphone,
                    email: resume.email,
                    idcard: resume.idcard,
                    domicile: resume.domicile,
                    living: resume.living,
                    address: resume.address,
                    height: resume.height,
                    weight: resume.weight,
                    marriage: resume.marriage && resume.marriage > 0 ? resume.marriage : '',
                    nationality: resume.nationality,
                    homepage: resume.homepage,
                    qq: resume.qq,
                    description: resume.description,
                    wxewm_n: resume.wxewm_n
                };
                this.drawerBasic = true;
            },
            // 上传时触发
            handleChangeWxewm(file, fileList) {
                this.$set(this.ruleFormBasic, 'file', file.raw);
                this.$set(this.ruleFormBasic, 'wxewm_n', file.url);
            },
            submitBasic() {
                let that = this,
                    ruleForm = that.ruleFormBasic,
                    formData = new FormData();

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                $.each(ruleForm, function (key, value) {
                    if (key != 'wxewm_n') {
                        if (key == 'birthday' && value !== '' ) {
                            value = formatMonth(value);
                        }
                        if(value !== '' && value != null){
                            formData.append(key, value);
                        }
                    }
                });

                httpPost('m=user&c=users_member&a=editSave', formData).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.drawerBasic = false;
                        that.refreshList = true;
                        that.getInfo(); // 重新拉取详情
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
            // personal advantage
            openTag() {
                let resume = deepClone(this.resume),
                    // expect = this.expectData.expect,
                    user_tag = this.userdata.user_tag,
                    userclass_name = this.userclass_name,
                    userTag = [];

                if (user_tag.length > 0) {
                    user_tag.forEach(function (item) {
                        userTag.push(userclass_name[item]);
                    })
                }
                if (resume.arrayTag && resume.arrayTag.length > 0) {
                    resume.arrayTag.forEach(function (item) {
                        if (userTag.indexOf(item) < 0) {
                            userTag.push(item); // 不在已有标签里的,追加标签
                        }
                    })
                }

                this.userTag = userTag;
                this.ruleFormTag = {
                    uid: resume.uid,
                    // eid: expect ? expect.id : '',
                    tag: resume.arrayTag ? resume.arrayTag : [],
                    description: resume.description
                };
                this.dialogTag = true;
            },
            showTag() {
                this.tagval = '';
                this.inputTag = true;
            },
            confirmTag() {
                let tag = this.ruleFormTag.tag
                userTag = this.userTag,
                    tagval = this.tagval,
                    len = tagval.length;

                if (len > 0) {
                    if (len < 2 || len > 8) {
                        message.warning("{yun:}t key='wap_user_00060'{/yun}");
                        return false;
                    }
                    if (tag.length >= 5) {
                        message.warning("{yun:}t key='admin_user_00206'{/yun}");
                        return false;
                    }
                    if (userTag.indexOf(tagval) > -1) {
                        message.warning("{yun:}t key='wap_user_00074'{/yun}");
                        return false;
                    }
                    tag.push(tagval);
                    userTag.push(tagval);
                    this.ruleFormTag.tag = tag;
                    this.userTag = userTag;
                }
                this.inputTag = false;
            },
            checkTag(val) {
                let tag = this.ruleFormTag.tag,
                    index = tag.indexOf(val);

                if (index > -1) { // 二次点击取消选中
                    tag.splice(index, 1);
                } else { // 首次点击选中
                    if (tag.length >= 5) {
                        message.warning("{yun:}t key='admin_user_00206'{/yun}");
                        return false;
                    }
                    tag.push(val);
                }

                this.ruleFormTag.tag = tag;
            },
            submitTag() {
                let that = this,
                    ruleForm = that.ruleFormTag;

                if (ruleForm.eid == '') {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.tag.length > 5) {
                    message.warning("{yun:}t key='admin_user_00206'{/yun}");
                    return false;
                }
                if (ruleForm.description == '' || ruleForm.description == null) {
                    message.warning("{yun:}t key='admin_01319'{/yun}");
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=user&c=users_resume&a=saveTag', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogTag = false;
                        that.refreshList = true;
                        that.resume.arrayTag = ruleForm.tag;
                        that.resume.description = ruleForm.description;
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },
            // Job intention
            openJob() {
                let resume = this.resume,
                    expect = this.expectData.expect;

                this.jobSelected = expect.jobnameArr;
                this.citySelected = expect.citynameArr;

                let salaryList = deepClone(this.expectData.salary),
                    maxsalaryList = [];
                salaryList.forEach(function(item, index) {
                    if (index > 0) {
                        if (expect.maxsalary > 0) {
                            if (parseInt(expect.minsalary) < parseInt(item)) {
                                maxsalaryList.push(item)
                            }
                        } else {
                            maxsalaryList.push(item)
                        }
                    }
                })
                salaryList.splice(salaryList.length-1, 1);
                this.minsalaryList = salaryList;
                this.maxsalaryList = maxsalaryList;

                this.ruleFormJob = {
                    uid: resume.uid,
                    eid: typeof expect.id !== 'undefined' ? expect.id : '",
                    job_classid: expect.job_classid, // TODO {yun:}t key='admin_00300'{/yun}
                    city_classid: expect.city_classid, // TODO {yun:}t key='member_user_00362'{/yun}
                    name: expect.name,
                    minsalary: expect.minsalary && expect.minsalary > 0 ? parseInt(expect.minsalary) : "',
                    maxsalary: expect.maxsalary && expect.maxsalary > 0 ? parseInt(expect.maxsalary) : '',
                    hy: expect.hy && expect.hy > 0 ? expect.hy : '',
                    report: expect.report && expect.report > 0 ? expect.report : '',
                    type: expect.type && expect.type > 0 ? expect.type : '',
                    jobstatus: expect.jobstatus && expect.jobstatus > 0 ? expect.jobstatus : '',
                };
                this.drawerJob = true;
            },
            salaryChange(val) {
                let that = this,
                    maxsalaryList = [],
                    i = 0;
                this.expectData.salary.forEach(function(item, index) {
                    if (parseInt(val) < parseInt(item)) {
                        maxsalaryList.push(item)
                        if (i === 0) {
                            that.ruleFormJob.maxsalary = item;
                        }
                        i++;
                    }
                })
                this.maxsalaryList = maxsalaryList;
            },
            confirmJob(data) {
                this.ruleFormJob.job_classid = data.jobId.join(',');
            },
            confirmCity(data) {
                this.ruleFormJob.city_classid = data.cityId.join(',');
            },
            submitJob() {
                let that = this,
                    ruleForm = that.ruleFormJob;

                if (typeof ruleForm.name === 'undefined' || ruleForm.name == "") {
                    message.warning("{yun:}t key='admin_00484'{/yun}");
                    return false;
                }
                if (typeof ruleForm.job_classid === 'undefined' || ruleForm.job_classid == "") {
                    message.warning(lc('admin_vue_00013'));
                    return false;
                }
                if (typeof ruleForm.city_classid === 'undefined' || ruleForm.city_classid == '') {
                    message.warning(lc('admin_vue_00014'));
                    return false;
                }
                if (ruleForm.minsalary == "" || ruleForm.minsalary == "0") {
                    message.warning(lc('admin_vue_00015'));
                    return false;
                }
                if (ruleForm.maxsalary && parseInt(ruleForm.maxsalary) <= parseInt(ruleForm.minsalary)) {
                    message.warning("{yun:}t key='member_user_00095'{/yun}");
                    return false;
                }
                if (ruleForm.report == "") {
                    message.warning("{yun:}t key='wap_00980'{/yun}");
                    return false;
                }
                if (ruleForm.type == "") {
                    message.warning("{yun:}t key='wap_js_00163'{/yun}");
                    return false;
                }
                if (ruleForm.jobstatus == "") {
                    message.warning("{yun:}t key='wap_00934'{/yun}");
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=user&c=users_resume&a=saveExpect', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.drawerJob = false;
                        that.refreshList = true;
                        that.eid = res.data.eid;
                        that.getInfo(); // 重新拉取详情
                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // 至今选择
            todayChange(val, type) {
                if (type == 'work') {
                    this.$set(this.ruleFormWork, 'edate', '');
                }
            },

            // Work experience
            openWork(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    workList = expectData.work;

                if (index !== '') {
                    let work = deepClone(workList[index])
                    this.ruleFormWork = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: work.id,
                        name: work.name,
                        title: work.title,
                        sdate: work.sdate > 0 ? new Date(work.sdate_n) : '',
                        edate: work.edate > 0 ? new Date(work.edate_n) : '',
                        content: work.content,
                    };

                    if (work.edate == '0') {
                        this.todayCheck = true;
                    }
                    this.indexWork = index;
                } else {
                    this.ruleFormWork = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: '',
                        name: '',
                        title: '',
                        sdate: '',
                        edate: '',
                        content: '',
                    };
                    this.todayCheck = false;
                    this.indexWork = -1
                }

                this.dialogWork = true;
            },
            submitWork() {
                let that = this,
                    indexWork = that.indexWork,
                    ruleForm = that.ruleFormWork;

                if (ruleForm.eid == "") {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning("{yun:}t key='wap_00137'{/yun}");
                    return false;
                }
                if (ruleForm.sdate == "") {
                    message.warning("{yun:}t key='admin_user_00213'{/yun}");
                    return false
                }
                ruleForm.sdate = formatMonth(ruleForm.sdate);
                if (ruleForm.edate != '') {
                    if (ruleForm.sdate >= ruleForm.edate) {
                        message.warning("{yun:}t key='admin_user_00201'{/yun}");
                        return false
                    }
                    ruleForm.edate = formatMonth(ruleForm.edate);
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=user&c=users_resume&a=work', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogWork = false;
                        that.refreshList = true;

                        // 拼接工作经历数据 - 减少请求服务器
                        if (ruleForm.id == '') {
                            let work = deepClone(ruleForm);
                            work.id = res.data.id;
                            work.sdate = 1;
                            work.sdate_n = ruleForm.sdate;
                            work.edate = ruleForm.edate != '' ? 2 : 0;
                            work.edate_n = ruleForm.edate != '' ? ruleForm.edate : "{yun:}t key='wap_js_00170'{/yun}";
                            that.expectData.work.unshift(work);
                        } else {
                            let work = that.expectData.work[indexWork];
                            work.name = ruleForm.name;
                            work.title = ruleForm.title;
                            work.sdate = 1;
                            work.sdate_n = ruleForm.sdate;
                            work.edate = ruleForm.edate != '' ? 2 : 0;
                            work.edate_n = ruleForm.edate != '' ? ruleForm.edate : "{yun:}t key='wap_js_00170'{/yun}";
                            work.content = ruleForm.content;
                            that.expectData.work[indexWork] = deepClone(work);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // Work experience
            openEdu(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    eduList = expectData.edu;

                if (index !== '') {
                    let edu = deepClone(eduList[index])
                    this.ruleFormEdu = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: edu.id,
                        name: edu.name,
                        education: edu.education > 0 ? edu.education : '',
                        specialty: edu.specialty,
                        title: '', // 此字段没实际意义，暂时占位
                    };
                    this.daterangeEdu = [
                        new Date(edu.sdate_n),
                        new Date(edu.edate_n)
                    ];
                    this.indexEdu = index;
                } else {
                    this.ruleFormEdu = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: '',
                        name: '',
                        sdate: '',
                        edate: '',
                        education: '',
                        specialty: '',
                        title: '', // 此字段没实际意义，暂时占位
                    };
                    this.daterangeEdu = [];
                    this.indexEdu = -1
                }

                this.dialogEdu = true;
            },
            submitEdu() {
                let that = this,
                    indexEdu = that.indexEdu,
                    daterangeEdu = that.daterangeEdu,
                    ruleForm = that.ruleFormEdu;

                if (ruleForm.eid == "") {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning("{yun:}t key='wap_user_00044'{/yun}");
                    return false;
                }
                if (daterangeEdu.length == 0) {
                    message.warning(lc('admin_vue_00016'));
                    return false
                }
                if (ruleForm.education == "") {
                    message.warning("{yun:}t key='wap_user_00049'{/yun}");
                    return false
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                ruleForm.sdate = formatMonth(daterangeEdu[0]);
                ruleForm.edate = formatMonth(daterangeEdu[1]);

                httpPost('m=user&c=users_resume&a=edu', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogEdu = false;
                        that.refreshList = true;

                        // 拼接工作经历数据 - 减少请求服务器
                        if (ruleForm.id == '') {
                            let edu = deepClone(ruleForm);
                            edu.id = res.data.id;
                            edu.sdate_n = ruleForm.sdate;
                            edu.edate_n = ruleForm.edate;
                            edu.education_n = that.userclass_name[ruleForm.education];
                            that.expectData.edu.unshift(edu);
                        } else {
                            let edu = that.expectData.edu[indexEdu];
                            edu.name = ruleForm.name;
                            edu.title = ruleForm.title;
                            edu.sdate_n = ruleForm.sdate;
                            edu.edate_n = ruleForm.edate;
                            edu.education = ruleForm.education;
                            edu.education_n = that.userclass_name[ruleForm.education];
                            edu.specialty = ruleForm.specialty;
                            that.expectData.edu[indexEdu] = deepClone(edu);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // Training experience
            openTraining(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    trainingList = expectData.training;

                if (index !== '') {
                    let training = deepClone(trainingList[index])
                    this.ruleFormTraining = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: training.id,
                        name: training.name,
                        title: training.title,
                        content: training.content,
                    };
                    this.daterangeTraining = [
                        new Date(training.sdate_n),
                        new Date(training.edate_n)
                    ];
                    this.indexTraining = index;
                } else {
                    this.ruleFormTraining = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: '',
                        name: '',
                        title: '',
                        sdate: '',
                        edate: '',
                        content: '',
                    };
                    this.daterangeTraining = [];
                    this.indexTraining = -1
                }

                this.dialogTraining = true;
            },
            submitTraining() {
                let that = this,
                    indexTraining = that.indexTraining,
                    daterangeTraining = that.daterangeTraining,
                    ruleForm = that.ruleFormTraining;

                if (ruleForm.eid == "") {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning("{yun:}t key='admin_00485'{/yun}");
                    return false;
                }
                if (daterangeTraining.length == 0) {
                    message.warning("{yun:}t key='admin_user_00212'{/yun}");
                    return false
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                ruleForm.sdate = formatMonth(daterangeTraining[0]);
                ruleForm.edate = formatMonth(daterangeTraining[1]);

                httpPost('m=user&c=users_resume&a=training', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogTraining = false;
                        that.refreshList = true;

                        // 拼接工作经历数据 - 减少请求服务器
                        if (ruleForm.id == '') {
                            let training = deepClone(ruleForm);
                            training.id = res.data.id;
                            training.sdate_n = ruleForm.sdate;
                            training.edate_n = ruleForm.edate;
                            that.expectData.training.unshift(training);
                        } else {
                            let training = that.expectData.training[indexTraining];
                            training.name = ruleForm.name;
                            training.title = ruleForm.title;
                            training.sdate_n = ruleForm.sdate;
                            training.edate_n = ruleForm.edate;
                            training.content = ruleForm.content;
                            that.expectData.training[indexTraining] = deepClone(training);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // Vocational skills
            openSkill(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    skillList = expectData.skill;

                if (index !== '') {
                    let skill = deepClone(skillList[index])
                    this.ruleFormSkill = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: skill.id,
                        name: skill.name,
                        longtime: skill.longtime,
                        ing: skill.ing,
                        pic_n: skill.pic,
                    };
                    this.indexSkill = index;
                } else {
                    this.ruleFormSkill = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: '',
                        name: '',
                        longtime: '',
                        ing: '',
                        pic_n: '',
                    };
                    this.indexSkill = -1
                }

                this.dialogSkill = true;
            },
            // 上传时触发
            handleChangeSkillPic(file, fileList) {
                this.$set(this.ruleFormSkill, 'file', file.raw);
                this.$set(this.ruleFormSkill, 'pic_n', file.url);
            },
            submitSkill() {
                let that = this,
                    indexSkill = that.indexSkill,
                    ruleForm = that.ruleFormSkill,
                    formData = new FormData();

                if (ruleForm.eid == "") {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning("{yun:}t key='admin_user_00210'{/yun}");
                    return false;
                }
                if (ruleForm.ing == "") {
                    message.warning("{yun:}t key='wap_user_00072'{/yun}");
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                $.each(ruleForm, function (key, value) {
                    if (key != 'pic_n') {
                        formData.append(key, value);
                    }
                });

                httpPost('m=user&c=users_resume&a=skill', formData).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogSkill = false;
                        that.refreshList = true;

                        // 拼接工作经历数据 - 减少请求服务器
                        if (ruleForm.id == '') {
                            let skill = deepClone(ruleForm);
                            skill.id = res.data.id;
                            skill.ing_n = that.userclass_name[ruleForm.ing];
                            skill.pic = ruleForm.pic_n;
                            that.expectData.skill.push(skill);
                        } else {
                            let skill = that.expectData.skill[indexSkill];
                            skill.name = ruleForm.name;
                            skill.longtime = ruleForm.longtime;
                            skill.ing_n = that.userclass_name[ruleForm.ing];
                            skill.pic = ruleForm.pic_n;
                            that.expectData.skill[indexSkill] = deepClone(skill);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // Project experience
            openProject(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    projectList = expectData.project;

                if (index !== '') {
                    let project = deepClone(projectList[index])
                    this.ruleFormProject = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: project.id,
                        name: project.name,
                        title: project.title,
                        content: project.content,
                    };
                    this.daterangeProject = [
                        new Date(project.sdate_n),
                        new Date(project.edate_n)
                    ];
                    this.indexProject = index;
                } else {
                    this.ruleFormProject = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: '',
                        name: '',
                        title: '',
                        sdate: '',
                        edate: '',
                        content: '',
                    };
                    this.daterangeProject = [];
                    this.indexProject = -1
                }

                this.dialogProject = true;
            },
            submitProject() {
                let that = this,
                    indexProject = that.indexProject,
                    daterangeProject = that.daterangeProject,
                    ruleForm = that.ruleFormProject;

                if (ruleForm.eid == "") {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning("{yun:}t key='wap_user_00046'{/yun}");
                    return false;
                }
                if (daterangeProject.length == 0) {
                    message.warning("{yun:}t key='admin_user_00214'{/yun}");
                    return false
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                ruleForm.sdate = formatMonth(daterangeProject[0]);
                ruleForm.edate = formatMonth(daterangeProject[1]);

                httpPost('m=user&c=users_resume&a=project', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogProject = false;
                        that.refreshList = true;

                        // 拼接工作经历数据 - 减少请求服务器
                        if (ruleForm.id == '") {
                            let project = deepClone(ruleForm);
                            project.id = res.data.id;
                            project.sdate_n = ruleForm.sdate;
                            project.edate_n = ruleForm.edate;
                            that.expectData.project.unshift(project);
                        } else {
                            let project = that.expectData.project[indexProject];
                            project.name = ruleForm.name;
                            project.title = ruleForm.title;
                            project.sdate_n = ruleForm.sdate;
                            project.edate_n = ruleForm.edate;
                            project.content = ruleForm.content;
                            that.expectData.project[indexProject] = deepClone(project);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // {yun:}t key='admin_00068'{/yun}
            openOther(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    otherList = expectData.other;

                if (index !== "') {
                    let other = deepClone(otherList[index])
                    this.ruleFormOther = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: other.id,
                        name: other.name,
                        content: other.content,
                    };
                    this.indexOther = index;
                } else {
                    this.ruleFormOther = {
                        uid: expectData.uid,
                        eid: expect.id,
                        id: '',
                        name: '',
                        content: '',
                    };
                    this.indexOther = -1
                }

                this.dialogOther = true;
            },
            submitOther() {
                let that = this,
                    indexOther = that.indexOther,
                    ruleForm = that.ruleFormOther;

                if (ruleForm.eid == "") {
                    message.warning("{yun:}t key='admin_user_00207'{/yun}");
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning("{yun:}t key='admin_00487'{/yun}");
                    return false;
                }

                if (that.saveLoading) {
                    return false;
                }
                that.saveLoading = true;

                httpPost('m=user&c=users_resume&a=other', ruleForm).then(function (response) {
                    let res = response.data;

                    if (res.error > 0) {
                        that.saveLoading = false;
                        message.error(res.msg);
                    } else {
                        that.dialogOther = false;
                        that.refreshList = true;

                        // 拼接工作经历数据 - 减少请求服务器
                        if (ruleForm.id == '') {
                            let other = deepClone(ruleForm);
                            other.id = res.data.id;
                            that.expectData.other.push(other);
                        } else {
                            let other = that.expectData.other[indexOther];
                            other.name = ruleForm.name;
                            other.content = ruleForm.content;
                            that.expectData.other[indexOther] = deepClone(other);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // 公用删除附表数据
            delResumeFb(type, index, id) {
                let that = this,
                    expectData = that.expectData;

                delConfirm(this, {}, function () {
                    httpPost('m=user&c=users_resume&a=delResumeFb', {
                        table: type,
                        id: id,
                        eid: expectData.expect.id,
                        uid: expectData.uid,
                    }).then(function (response) {
                        let res = response.data;

                        if (res.error > 0) {
                            message.error(res.msg);
                        } else {
                            that.refreshList = true;
                            that.expectData[type].splice(index, 1);
                            message.success(res.msg);
                        }
                    })
                }, "{yun:}t key='admin_user_00204'{/yun}");
            },
        },
        watch: {
            id: function (val, oldVal) {
                if (typeof this.id !== 'undefined') {
                    this.loading = true;
                    this.getInfo();
                }
            },
            uid: function (val, oldVal) {
                if (typeof this.id === 'undefined') {
                    this.loading = true;
                    this.getInfo();
                }
            }
        }
    };
</script>
<style scoped></style>