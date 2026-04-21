<template>
    <div class="shbox" v-loading="loading" style="overflow-y: auto;">
        <div class="shshow_tit">
            <i class="el-icon-office-building"></i> 基本资料
            <span class="shshow_cz">
                <el-button type="text" @click="openBasic">
                    <i class="el-icon-edit"></i>编辑资料
                </el-button>
            </span>
        </div>
        <div class="userinfo_box">
            <div class="userinfo_l"><img :src="resume.photo" width="70" height="70"></div>
            <div class="userinfo_r">
                <div class="userinfo_name">{{resume.name}}</div>
                <div class="userinfo">
                    {{ resume.sex_n }}
                    <span v-if="resume.age">，{{ resume.age }}岁</span>
                    <span v-if="resume.height">，{{ resume.height }}cm</span>
                    <span v-if="resume.weight">，{{ resume.weight }}kg</span>
                    <span v-if="resume.marriage_n">，{{ resume.marriage_n }}</span>
                    <span v-if="resume.living">，现居{{ resume.living }}</span>
                </div>
                <div class="userinfo" v-if="resume.edu_n || resume.exp_n">
                    <span v-if="resume.edu_n">{{ resume.edu_n }}学历 </span>
                    <span class="userline" v-if="resume.edu_n && resume.exp_n">|</span>
                    <span v-if="resume.exp_n">{{ resume.exp_n }}经验</span>
                </div>
            </div>
        </div>
        <div class="shshow_p">
            <div class="cominfo" v-if="resume.telphone"><i class="el-icon-mobile"></i>
                联系电话：{{ resume.telphone }}</div>
            <div class="cominfo" v-if="resume.email"><i class="el-icon-message"></i>
                联系邮箱：{{ resume.email }}</div>
            <div class="cominfo" v-if="resume.idcard"><i class="el-icon-postcard"></i>
                身份证号：{{ resume.idcard }}</div>
            <div class="cominfo" v-if="resume.domicile"><i class="el-icon-location-outline"></i>
                户籍所在地：{{ resume.domicile }}</div>
            <div class="cominfo" v-if="resume.address"><i class="el-icon-location-information"></i>
                详细地址：{{ resume.address }}</div>
        </div>

        <!--个人优势-->
        <div class="user_resume_list">
            <div class="shshow_tit">
                <i class="el-icon-medal-1"></i> 个人优势
            </div>
            <div class="shshow_p">
                <el-tag size="mini" v-for="(tagItem,key) in resume.arrayTag" :key="key">{{tagItem}}</el-tag>
                <div class="cominfo">{{resume.description}}</div>
            </div>
            <div class="user_resume_add">
                <div class="">总结优势，突出亮点，个人优势将突出展示给HR</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openTag">
                        <i class="el-icon-circle-plus-outline"></i> {{ (resume.arrayTag &&
                        resume.arrayTag.length > 0) || resume.description ? '编辑' : '添加' }}
                    </el-button>
                </div>
            </div>
        </div>
        <!--求职意向-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-notebook-2"></i> 求职意向</div>
            <div class="shshow_p" v-if="expectData.expect">
                <div class="cominfo">期望职位： {{expectData.expect.name}} </div>
                <div class="cominfo">从事职位： {{expectData.expect.job_classname}}</div>
                <div class="cominfo">期望地点： {{expectData.expect.city_classname}}</div>
                <div class="cominfo">期望薪资： {{expectData.expect.salary}}</div>
                <div class="cominfo">从事行业： {{expectData.expect.hy_n}}</div>
                <div class="cominfo">到岗时间： {{expectData.expect.report_n}}</div>
                <div class="cominfo">工作性质： {{expectData.expect.type_n}}</div>
                <div class="cominfo">求职状态： {{expectData.expect.jobstatus_n}}</div>
            </div>


            <div class="user_resume_add">
                <div class="">建议完善求职偏好</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openJob">
                        <i class="el-icon-circle-plus-outline"></i> {{ expectData.expect ? '编辑' : '添加' }}
                    </el-button>
                </div>
            </div>
        </div>

        <!--工作经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-suitcase-1"></i> 工作经历</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(work, workkey) in expectData.work" :key="workkey">
                <div class="user_resume_addname ">{{work.name}}
                    <el-button type="text" @click="openWork(workkey)">
                        <i class="el-icon-edit"></i> 修改
                    </el-button>
                    <el-button type="text" @click="delResumeFb('work', workkey, work.id)">
                        <i class="el-icon-delete"></i> 删除
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
                <div class="">展示工作经验、工作能力否符合岗位要求的重要依据</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openWork('')">
                        <i class="el-icon-circle-plus-outline"></i> 添加
                    </el-button>
                </div>
            </div>
        </div>
        <!--教育经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-school"></i> 教育经历</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(edu, edukey) in expectData.edu" :key="edukey">
                <div class="user_resume_addname ">{{edu.name}}
                    <el-button type="text" @click="openEdu(edukey)">
                        <i class="el-icon-edit"></i> 修改
                    </el-button>
                    <el-button type="text" @click="delResumeFb('edu', edukey, edu.id)">
                        <i class="el-icon-delete"></i> 删除
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
                <div class="">补足HR对学历背景的了解</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openEdu('')">
                        <i class="el-icon-circle-plus-outline"></i> 添加
                    </el-button>
                </div>
            </div>
        </div>
        <!--培训经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-data-analysis"></i> 培训经历</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(training, trainingKey) in expectData.training" :key="trainingKey">
                <div class="user_resume_addname ">{{training.name}}
                    <el-button type="text" @click="openTraining(trainingKey)">
                        <i class="el-icon-edit"></i> 修改
                    </el-button>
                    <el-button type="text" @click="delResumeFb('training', trainingKey, training.id)">
                        <i class="el-icon-delete"></i> 删除
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
                <div class="">展示培训经验否符合岗位要求的重要依据</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openTraining('')">
                        <i class="el-icon-circle-plus-outline"></i> 添加
                    </el-button>
                </div>
            </div>
        </div>
        <!--职业技能-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-reading"></i> 职业技能</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(skill, skillkey) in expectData.skill" :key="skillkey">
                <div class="user_resume_addname ">{{skill.name}}
                    <el-button type="text" @click="openSkill(skillkey)">
                        <i class="el-icon-edit"></i> 修改
                    </el-button>
                    <el-button type="text" @click="delResumeFb('skill', skillkey, skill.id)">
                        <i class="el-icon-delete"></i> 删除
                    </el-button>
                </div>
                <div class="user_resume_addjy">
                    <div class=" ">{{skill.ing_n}} </div>
                    <div class="user_resume_time">{{skill.longtime}} 年</div>
                </div>
                <div class="user_resume_ms" v-if="skill.pic">
                    <img :src="skill.pic" width="95" height="70" :preview-src-list="skill.pic">
                </div>
            </div>
            <!--循环-->

            <div class="user_resume_add">
                <div class="">技能专长建议填写职业技能为简历加分</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openSkill('')">
                        <i class="el-icon-circle-plus-outline"></i> 添加
                    </el-button>
                </div>
            </div>
        </div>
        <!--项目经历-->
        <div class="user_resume_list">
            <div class="shshow_tit"><i class="el-icon-wallet"></i> 项目经历</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(project, projectkey) in expectData.project" :key="projectkey">
                <div class="user_resume_addname ">{{project.name}}
                    <el-button type="text" @click="openProject(projectkey)">
                        <i class="el-icon-edit"></i> 修改
                    </el-button>
                    <el-button type="text" @click="delResumeFb('project', projectkey, project.id)">
                        <i class="el-icon-delete"></i> 删除
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
                <div class="">展示工作经验、能力，这也是HR判断是否符合岗位要求的重要依据。</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openProject('')">
                        <i class="el-icon-circle-plus-outline"></i> 添加
                    </el-button>
                </div>
            </div>
        </div>
        <!--其他描述-->
        <div class="user_resume_list" style="padding-bottom:80px; ;">
            <div class="shshow_tit"><i class="el-icon-mic"></i> 其他描述</div>
            <!--循环-->
            <div class="user_resume_show" v-for="(other, otherkey) in expectData.other" :key="otherkey">
                <div class="user_resume_addname ">{{other.name}}
                    <el-button type="text" @click="openOther(otherkey)">
                        <i class="el-icon-edit"></i> 修改
                    </el-button>
                    <el-button type="text" @click="delResumeFb('other', otherkey, other.id)">
                        <i class="el-icon-delete"></i> 删除
                    </el-button>
                </div>
                <div class="user_resume_ms">{{other.content}}</div>
            </div>
            <!--循环-->
            <div class="user_resume_add">
                <div class="">其他加分补充</div>
                <div class="user_resume_addbth">
                    <el-button type="text" @click="openOther('')">
                        <i class="el-icon-circle-plus-outline"></i> 添加
                    </el-button>
                </div>
            </div>
        </div>
        <!---编辑简历 基本资料-->
        <el-drawer title="编辑基本资料" :append-to-body="true" :visible.sync="drawerBasic" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                    <tr align="left">
                        <th width="120">名称</th>
                        <th width=" ">状态</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">姓名</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.name" placeholder="请输入姓名"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">性别</div>
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
                            <div class="TableTite">出生年月</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-date-picker v-model="ruleFormBasic.birthday" type="month" placeholder="选择月">
                                </el-date-picker>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">最高学历</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormBasic.edu" placeholder="请选择">
                                    <el-option v-for="edukey in userdata.user_edu" :key="edukey"
                                               :label="userclass_name[edukey]" :value="edukey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">工作经验</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormBasic.exp" placeholder="请选择">
                                    <el-option v-for="wordkey in userdata.user_word" :key="wordkey"
                                               :label="userclass_name[wordkey]" :value="wordkey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">联系电话</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.telphone" placeholder="请输入联系电话"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">联系邮箱</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.email" placeholder="请输入联系邮箱"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">身份证号</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.idcard" placeholder="请输入身份证号"
                                          @input="inputIdcard($event, 'ruleFormBasic', 'idcard')"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">户籍所在地</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.domicile" placeholder="请输入户籍所在地"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">现居地</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.living" placeholder="请输入现居地"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">详细地址</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.address" placeholder="请输入详细地址"></el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">身高</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.height" placeholder="请输入身高"
                                          @input="inputFloatNumber($event, 'ruleFormBasic', 'height')"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">体重</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.weight" placeholder="请输入体重"
                                          @input="inputFloatNumber($event, 'ruleFormBasic', 'weight')"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">婚姻状况</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormBasic.marriage" placeholder="请选择">
                                    <el-option v-for="marriagekey in userdata.user_marriage" :key="marriagekey"
                                               :label="userclass_name[marriagekey]" :value="marriagekey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">民族</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.nationality" placeholder="请输入民族"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">个人主页/博客</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.homepage" placeholder="请输入个人主页/博客"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">QQ</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormBasic.qq" placeholder="请输入QQ"> </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">个人二维码</div>
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
                            <div class="TableTite">自我介绍</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input type="textarea" :rows="2" placeholder="请自我介绍下吧"
                                          v-model="ruleFormBasic.description">
                                </el-input>
                            </div>
                        </td>
                    </tr>
                    </tbody>
                </table>
            </div>
            <div class="setBasicButn" style="border: none; height: 80px;">
                <el-button type="primary" size="medium" @click="submitBasic">提交</el-button>
            </div>


        </el-drawer>
        <!---编辑求职意向-->
        <el-drawer title="编辑求职意向" :append-to-body="true" :visible.sync="drawerJob" :wrapper-closable="false" size="60%">
            <div class="uploadTable" style="padding:0px 20px;">
                <table class="tableVue">
                    <thead>
                    <tr align="left">
                        <th width="120">名称</th>
                        <th width=" ">状态</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td>
                            <div class="TableTite">期望职位</div>
                        </td>
                        <td>
                            <div class="TableInpt">
                                <el-input v-model="ruleFormJob.name" placeholder="请输入期望职位">
                                </el-input>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">从事职位</div>
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
                            <div class="TableTite">期望地点</div>
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
                            <div class="TableTite">期望薪资</div>
                        </td>
                        <td>
                            <div class="TableInpt" style="max-width: 700px;">
                                <el-select v-model="ruleFormJob.minsalary" placeholder="请选择" @change="salaryChange" style="margin-right:8px;">
                                    <el-option v-for="maxsalary1Val in minsalaryList" :key="maxsalary1Val" :label="maxsalary1Val" :value="maxsalary1Val">
                                    </el-option>
                                </el-select>
                                <el-select v-model="ruleFormJob.maxsalary" placeholder="请选择">
                                    <el-option v-for="maxsalary2Val in maxsalaryList" :key="maxsalary2Val" :label="maxsalary2Val" :value="maxsalary2Val">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">从事行业</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.hy" placeholder="请选择">
                                    <el-option v-for="industrykey in industry_index" :key="industrykey"
                                               :label="industry_name[industrykey]" :value="industrykey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">到岗时间</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.report" placeholder="请选择">
                                    <el-option v-for="reportkey in userdata.user_report" :key="reportkey"
                                               :label="userclass_name[reportkey]" :value="reportkey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">工作性质</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.type" placeholder="请选择">
                                    <el-option v-for="typekey in userdata.user_type" :key="typekey"
                                               :label="userclass_name[typekey]" :value="typekey">
                                    </el-option>
                                </el-select>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <div class="TableTite">求职状态</div>
                        </td>
                        <td>
                            <div class="TableSelect">
                                <el-select v-model="ruleFormJob.jobstatus" placeholder="请选择">
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
                <el-button type="primary" size="medium" @click="submitJob">提交</el-button>
            </div>
        </el-drawer>

        <!---编辑个人优势-->
        <div class="modluDrawer">
            <el-dialog title="个人优势" :visible.sync="dialogTag" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">优势标签</div>
                    <div class="wxsettipBiaoqin">
                        <el-tag :key="tagkey" v-for="(tag, tagkey) in userTag" :disable-transitions="false"
                                @click="checkTag(tag)" :effect="ruleFormTag.tag.indexOf(tag) > -1 ? 'dark' : 'light'">
                            {{ tag }}
                        </el-tag>
                        <el-input class="input-new-tag" v-if="inputTag" v-model="tagval" autofoucs size="small"
                                  @keyup.enter.native="confirmTag">
                        </el-input>
                        <el-button v-else class="button-new-tag" size="small" @click="showTag">+ 新增
                        </el-button>
                    </div>
                    <div class="wxsettip_small ">自我评价</div>
                    <el-input type="textarea"
                              :placeholder="'用一两句话总结一下自己的优势，突出亮点。\n示例：\n2年销售管理经验，在担任区域负责人期间，带领区域同事做到移动业务量全省第一。口齿伶俐、思维灵敏，管理组织能力强，精通各种营销手段。'"
                              v-model="ruleFormTag.description" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogTag = false">取 消</el-button>
					<el-button type="primary" @click="submitTag">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑工作经历-->
        <div class="modluDrawer">
            <el-dialog title="工作经历" :visible.sync="dialogWork" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">公司名称</div>
                    <div class=""><el-input v-model="ruleFormWork.name" placeholder="请输入公司名称"></el-input> </div>
                    <div class="wxsettip_small ">担任职位</div>
                    <div class=""><el-input v-model="ruleFormWork.title" placeholder="请输入担任职位"></el-input> </div>
                    <div class="wxsettip_small ">工作时间</div>
                    <div class="wxsettip_Sealect" style="display: flex; align-items: center;">
                        <el-date-picker v-model="ruleFormWork.sdate" type="month" placeholder="选择开始时间">
                        </el-date-picker>
                        <el-date-picker style="margin: 0 8px;" :disabled="todayCheck" v-model="ruleFormWork.edate" type="month"
                                        placeholder="选择结束时间">
                        </el-date-picker>
                        <el-checkbox v-model="todayCheck" @change="todayChange($event, 'work')">至今</el-checkbox>
                    </div>
                    <div class="wxsettip_small ">工作内容</div>
                    <el-input type="textarea" :placeholder="'请简短介绍公司与自己负责的任务，分条罗列在什么项目中，通过某些动作或技能达到可量化的结果。\n示例：\n1、主要负责新员工入职培训；\n2、分析制定员工每月个人销售业绩；\n3、帮助员工提高每日客单价，整体店面管理工作等；'"
                              v-model="ruleFormWork.content" :autosize="{ minRows: 3, maxRows: 6 }">
                    </el-input>
                </div>

                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogWork = false">取 消</el-button>
					<el-button type="primary" @click="submitWork">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑学历-->
        <div class="modluDrawer">
            <el-dialog title="教育经历" :visible.sync="dialogEdu" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">学校名称</div>
                    <div class=""><el-input v-model="ruleFormEdu.name" placeholder="请输入学校名称"></el-input> </div>
                    <div class="wxsettip_small ">在校时间</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeEdu" type="monthrange" range-separator="至"
                                        start-placeholder="开始日期" end-placeholder="结束日期">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">最高学历</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormEdu.education" placeholder="请选择">
                            <el-option v-for="edukey in userdata.user_edu" :key="edukey" :label="userclass_name[edukey]"
                                       :value="edukey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">所学专业</div>
                    <div class=""><el-input v-model="ruleFormEdu.specialty" placeholder="请输入专业名称"></el-input> </div>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogEdu = false">取 消</el-button>
					<el-button type="primary" @click="submitEdu">确 定</el-button>
                </span>
            </el-dialog>
        </div>

        <!---编辑培训经历-->
        <div class="modluDrawer">
            <el-dialog title="培训经历" :visible.sync="dialogTraining" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">培训中心</div>
                    <div class=""><el-input v-model="ruleFormTraining.name" placeholder="请输入培训中心"></el-input> </div>
                    <div class="wxsettip_small ">培训方向</div>
                    <div class=""><el-input v-model="ruleFormTraining.title" placeholder="请输入培训方向"></el-input> </div>
                    <div class="wxsettip_small ">培训时间</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeTraining" type="monthrange" range-separator="至"
                                        start-placeholder="开始日期" end-placeholder="结束日期">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">培训描述</div>
                    <el-input type="textarea" placeholder="请简短介绍培训经历所获收获等；" v-model="ruleFormTraining.content"
                              :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogTraining = false">取 消</el-button>
					<el-button type="primary" @click="submitTraining">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑项目经历-->
        <div class="modluDrawer">
            <el-dialog title="项目经历" :visible.sync="dialogProject" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">项目名称</div>
                    <div class=""><el-input v-model="ruleFormProject.name" placeholder="请输入项目名称"></el-input> </div>
                    <div class="wxsettip_small ">担任职务</div>
                    <div class=""><el-input v-model="ruleFormProject.title" placeholder="请输入担任职务"></el-input> </div>
                    <div class="wxsettip_small ">项目时间</div>
                    <div class="wxsettip_Sealect">
                        <el-date-picker v-model="daterangeProject" type="monthrange" range-separator="至"
                                        start-placeholder="开始日期" end-placeholder="结束日期">
                        </el-date-picker>
                    </div>
                    <div class="wxsettip_small ">项目描述</div>
                    <el-input type="textarea" placeholder="请简短介绍公司与自己负责的任务，分条罗列在什么项目中，通过某些动作或技能达到可量化的结果。
示例：
1、主要负责新员工入职培训；
2、分析制定员工每月个人销售业绩；
3、帮助员工提高每日客单价，整体店面管理工作等；" v-model="ruleFormProject.content" :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogProject = false">取 消</el-button>
					<el-button type="primary" @click="submitProject">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑其他-->
        <div class="modluDrawer">
            <el-dialog title="其他加分项" :visible.sync="dialogOther" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">标题</div>
                    <div class=""><el-input v-model="ruleFormOther.name" placeholder="请输入标题名称"></el-input> </div>
                    <div class="wxsettip_small ">描述</div>
                    <el-input type="textarea" v-model="ruleFormOther.content" placeholder="请简短介绍其他加分优势"
                              :autosize="{ minRows: 3, maxRows: 6 }"></el-input>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogOther = false">取 消</el-button>
					<el-button type="primary" @click="submitOther">确 定</el-button>
                </span>
            </el-dialog>
        </div>
        <!---编辑技能-->
        <div class="modluDrawer">
            <el-dialog title="职业技能" :visible.sync="dialogSkill" :with-header="true" :modal-append-to-body="false"
                       :show-close="true" width="450px" append-to-body>
                <div>
                    <div class="wxsettip_small ">技能名称</div>
                    <div class=""><el-input v-model="ruleFormSkill.name" placeholder="请输入技能名称"></el-input> </div>
                    <div class="wxsettip_small ">掌握时间</div>
                    <div class="wxsettip_Sealect">
                        <el-input v-model="ruleFormSkill.longtime" placeholder="请输入掌握时间">
                            <template slot="append">年</template>
                        </el-input>
                    </div>
                    <div class="wxsettip_small ">熟练程度</div>
                    <div class="wxsettip_Sealect">
                        <el-select v-model="ruleFormSkill.ing" placeholder="请选择">
                            <el-option v-for="ingkey in userdata.user_ing" :key="ingkey" :label="userclass_name[ingkey]"
                                       :value="ingkey">
                            </el-option>
                        </el-select>
                    </div>
                    <div class="wxsettip_small ">技能证书</div>
                    <div>
                        <el-upload class="avatar-uploader" list-type="picture" :accept="pic_accept" action="" :auto-upload="false"
                                   :on-change="handleChangeSkillPic" :show-file-list="false">
                            <img v-if="ruleFormSkill.pic_n" :src="ruleFormSkill.pic_n" class="avatar">
                            <i v-else class="el-icon-plus avatar-uploader-icon"></i>
                        </el-upload>
                    </div>
                </div>
                <span slot="footer" class="dialog-footer">
					<el-button @click="dialogSkill = false">取 消</el-button>
					<el-button type="primary" @click="submitSkill">确 定</el-button>
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
                // 个人优势
                dialogTag: false,
                ruleFormTag: {},
                userTag: [],
                inputTag: false,
                tagval: '',
                // 求职意向
                drawerJob: false,
                ruleFormJob: {},
                jobSelected: null,
                citySelected: null,
                minsalaryList: [],
                maxsalaryList: [],

                todayCheck: false, // 至今选中

                // 工作经历
                dialogWork: false,
                indexWork: -1,
                ruleFormWork: {},
                // 教育经历
                dialogEdu: false,
                indexEdu: -1,
                daterangeEdu: [],
                ruleFormEdu: {},
                // 培训经历
                dialogTraining: false,
                indexTraining: -1,
                daterangeTraining: [],
                ruleFormTraining: {},
                // 技能提升
                dialogSkill: false,
                indexSkill: -1,
                ruleFormSkill: {},
                // 项目经历
                dialogProject: false,
                indexProject: -1,
                daterangeProject: [],
                ruleFormProject: {},
                // 其他描述
                dialogOther: false,
                indexOther: -1,
                ruleFormOther: {},
            }
        },
        components: {
            'job_class': httpVueLoader('../../../component/job_class.vue'),
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
                this.$data[form][key] = val.replace(/[^0-9.]/g, '');
            },

            // 编辑资料
            openBasic() {
                let resume = this.resume;
                this.ruleFormBasic = {
                    uid: resume.uid,
                    name: resume.name,
                    sex: resume.sex,
                    birthday: resume.birthday ? new Date(resume.birthday) : '',
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
            // 个人优势
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
                        message.warning('请输入2-8个标签字符');
                        return false;
                    }
                    if (tag.length >= 5) {
                        message.warning('标签最多选择5个');
                        return false;
                    }
                    if (userTag.indexOf(tagval) > -1) {
                        message.warning('标签已存在');
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
                        message.warning('标签最多选择5个');
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.tag.length > 5) {
                    message.warning('标签最多选择5个');
                    return false;
                }
                if (ruleForm.description == '' || ruleForm.description == null) {
                    message.warning('请输入自我评价');
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
            // 求职意向
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
                    eid: typeof expect.id !== 'undefined' ? expect.id : '',
                    job_classid: expect.job_classid, // TODO 选择职位
                    city_classid: expect.city_classid, // TODO 选择城市
                    name: expect.name,
                    minsalary: expect.minsalary && expect.minsalary > 0 ? parseInt(expect.minsalary) : '',
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
                    message.warning('请输入期望职位');
                    return false;
                }
                if (typeof ruleForm.job_classid === 'undefined' || ruleForm.job_classid == "") {
                    message.warning('请选择从事职位');
                    return false;
                }
                if (typeof ruleForm.city_classid === 'undefined' || ruleForm.city_classid == '') {
                    message.warning('请选择期望地点');
                    return false;
                }
                if (ruleForm.minsalary == "" || ruleForm.minsalary == "0") {
                    message.warning('请输入期望薪资');
                    return false;
                }
                if (ruleForm.maxsalary && parseInt(ruleForm.maxsalary) <= parseInt(ruleForm.minsalary)) {
                    message.warning('最高薪资必须大于最低薪资');
                    return false;
                }
                if (ruleForm.report == "") {
                    message.warning('请选择到岗时间');
                    return false;
                }
                if (ruleForm.type == "") {
                    message.warning('请选择工作性质');
                    return false;
                }
                if (ruleForm.jobstatus == "") {
                    message.warning('请选择求职状态');
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

            // 工作经历
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning('请输入公司名称');
                    return false;
                }
                if (ruleForm.sdate == "") {
                    message.warning('请选择工作时间');
                    return false
                }
                ruleForm.sdate = formatMonth(ruleForm.sdate);
                if (ruleForm.edate != '') {
                    if (ruleForm.sdate >= ruleForm.edate) {
                        message.warning('结束日期必须大于开始日期');
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
                            work.edate_n = ruleForm.edate != '' ? ruleForm.edate : '至今';
                            that.expectData.work.unshift(work);
                        } else {
                            let work = that.expectData.work[indexWork];
                            work.name = ruleForm.name;
                            work.title = ruleForm.title;
                            work.sdate = 1;
                            work.sdate_n = ruleForm.sdate;
                            work.edate = ruleForm.edate != '' ? 2 : 0;
                            work.edate_n = ruleForm.edate != '' ? ruleForm.edate : '至今';
                            work.content = ruleForm.content;
                            that.expectData.work[indexWork] = deepClone(work);
                        }

                        message.success(res.msg, function () {
                            that.saveLoading = false;
                        });
                    }
                })
            },

            // 工作经历
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning('请输入学校名称');
                    return false;
                }
                if (daterangeEdu.length == 0) {
                    message.warning('请选择在校时间');
                    return false
                }
                if (ruleForm.education == "") {
                    message.warning('请选择最高学历');
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

            // 培训经历
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning('请输入培训中心');
                    return false;
                }
                if (daterangeTraining.length == 0) {
                    message.warning('请选择培训时间');
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

            // 职业技能
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning('请输入技能名称');
                    return false;
                }
                if (ruleForm.ing == "") {
                    message.warning('请选择熟练程度');
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

            // 项目经历
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning('请输入项目名称');
                    return false;
                }
                if (daterangeProject.length == 0) {
                    message.warning('请选择项目时间');
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
                        if (ruleForm.id == '') {
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

            // 其他描述
            openOther(index) {
                let expectData = this.expectData,
                    expect = expectData.expect,
                    otherList = expectData.other;

                if (index !== '') {
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
                    message.warning('请先完善求职意向');
                    return false;
                }
                if (ruleForm.name == "") {
                    message.warning('请输入标题名称');
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
                }, '确定要删除该项内容？');
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