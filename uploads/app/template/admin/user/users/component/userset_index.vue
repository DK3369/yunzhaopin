<template>
    <!--会员-个人-个人设置：个人设置-->
    <div class="setUpload">
        <div class="uploadTable">
            <table class="tableVue">
                <thead>
                <tr align="left">
                    <th width="200">名称</th>
                    <th width="680">状态</th>
                    <th>说明</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>
                        <div class="TableTite">信息审核</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_height_resume" label="优质简历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_idcard_status" label="身份认证"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_msg_status" label="求职咨询"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_photo_status" label="头像"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.rshow_photo_status" label="作品"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_trust_status" label="委托简历"></el-checkbox>
                        </div>
                    
                    </td>
                    <td>
                        <div class="TableShuom">
                            开启则需审核后展示
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">简历审核</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_status" label="创建简历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_revise_state" label="修改简历"></el-checkbox>
                        </div>
                        <div v-if="ruleForm.resume_status == true || ruleForm.user_revise_state == true"
                             class="content-sub">
                            <div class="TableShuom" style="margin-bottom: 8px">
                                创建、修改简历审核，可设置强制审核时间段。未配置或为空则全天审核，可跨天设置，如：22:00-08:00
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
                        <div class="TableTite">强制操作</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_resume_status" label="创建简历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_gzgzh" label="关注公众号"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">快速投递</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_kstd" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.resume_kstd" label="2">关闭</el-radio>
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
                        <div class="TableTite">创建简历必填项</div>
                        <!--<div style="color:red;">这一块要重新写样式</div>-->
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_exp" label="工作经历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_edu" label="教育经历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.resume_create_project" label="项目经历"></el-checkbox>
                        </div>
                        <div class="content-sub"
                             v-if="ruleForm.resume_create_exp == true && cache.hasOwnProperty('userdata')">
                            <el-checkbox-group v-model="ruleForm.expcreate">
                                <div v-for="(item,key) in cache.userdata.user_word" :key="key" class="tc_checkbox">
                                    <el-checkbox :label="item">{{ cache.userclass_name[item] }}</el-checkbox>
                                </div>
                            </el-checkbox-group>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info"> 工作经历非必填条件选择
                                    根据求职者填写基本信息时选择的工作经验，创建简历时可不强制填写工作经历
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
                                    说明：求职者创建简历时满足勾选条件的则可不强制填写
                                </div>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>根据求职者所选的职位类别设置是否强制填写以上必填选项</p>
                            <div class="TableShuom tc_checktip">
                                说明：例如设置的职位类别是“普工”，求职者创建简历时选择“普工”后可不强制填写以上任何条件，且投递时不判断简历完整度。
                            </div>
                            <!--TODO 7.0 统一类别选择-->
                            <!-- <el-button>设置不强制填写职位类别项</el-button> -->
                            <job_class ref="job_class" multiple :max="100" @confirm="confirmJob" :showsearch="false" :selected="jobSelected"></job_class>
                            <div style="overflow: hidden; position: relative; margin-top: 8px" v-if="ruleForm.sy_resume_job_classid && cache.hasOwnProperty('job_name')">
                                <el-tag style="margin:5px 5px 0 0;" v-for="(item,key) in ruleForm.sy_resume_job_classid" :key="key" @close="deltag(item)" closable>{{ cache.job_name[item] }}</el-tag>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>跨职位类别投递，是否要判断完整度</p>
                            <div class="contentRadio">
                                <el-radio v-model="ruleForm.sy_resume_kh_td" label="1">开启</el-radio>
                                <el-radio v-model="ruleForm.sy_resume_kh_td" label="0">关闭</el-radio>
                            </div>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    说明：符合不强制填写职位类别项的简历，投递设置项之外的职位，是否开启判断简历完整度。（例如“普工”简历投递设置项之外的职位，是否要判断完整度）
                                </div>
                            </div>
                        </div>
                        <div class="content-sub">
                            <p>快速投递，是否遵从简历必填项</p>
                            <div class="contentRadio">
                                <el-radio v-model="ruleForm.resume_kstd_req" label="1">开启</el-radio>
                                <el-radio v-model="ruleForm.resume_kstd_req" label="2">关闭</el-radio>
                            </div>
                            <div class="TableShuom">
                                <div class="tc_checktip el-icon-info">
                                    说明：快速投递是否遵从简历必填项，默认开启。关闭后，不再判断简历完整度
                                </div>
                            </div>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> 求职者创建简历时强制填写已勾选项目</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">申请职位要求简历完整度达到</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_sqintegrity" placeholder="请输入数字">
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
                        <div class="TableTite">简历置顶要求</div>
                    </td>
                    <td>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_work_regiser" label="工作经历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_edu_regiser" label="教育经历"></el-checkbox>
                        </div>
                        <div class="tc_checkbox">
                            <el-checkbox v-model="ruleForm.user_project_regiser" label="项目经历"></el-checkbox>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">简历求职意向字数</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sy_rname_num" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">字</span>
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
                        <div class="TableTite">个人会员发布简历数</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_number" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">份</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> 为空则不限</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">个人搜索器数量</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_finder" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">个</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> 数量太多，发送订阅邮件会很慢，为空则不限</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">个人会员向网站委托简历数</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.user_trust_number" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">份</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip"> 为空或0则关闭委托</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">姓名展示</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.user_name" label="1">用户自定义</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="2">编号</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="3">性别称呼</el-radio>
                            <el-radio v-model="ruleForm.user_name" label="4">真实姓名</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">编号：如NO.12，性别称呼：如冯先生/女士，真实姓名：如冯云鹏</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">个人简历头像</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.user_pic" label="1">用户自定义</el-radio>
                            <el-radio v-model="ruleForm.user_pic" label="2">显示</el-radio>
                            <el-radio v-model="ruleForm.user_pic" label="3">不显示</el-radio>
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
                        <div class="TableTite">个人简历刷新类型</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_sx" label="1">登录后自动刷新</el-radio>
                            <el-radio v-model="ruleForm.resume_sx" label="2">弹出框手动刷新</el-radio>
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
                        <div class="TableTite">简历模糊化</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.resume_open_check" label="1">关闭</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="2">企业登录</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="3">发布职位</el-radio>
                            <el-radio v-model="ruleForm.resume_open_check" label="4">下载简历(查看联系方式)</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">
                                简历模糊化是指对简历详情页的工作经历等栏目的内容进行模糊化处理，无法看到详细内容。若选择除"关闭"外的其他选项，需当前登录账号符合对应的条件才会关闭
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">个人用户访问简历权限</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_user_visit_resume" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.sy_user_visit_resume" label="0">关闭</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">若选择"关闭"，个人用户将无法直接访问简历信息</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">待审核简历可以投递</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_shresume_applyjob" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.sy_shresume_applyjob" label="0">关闭</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">若选择"关闭"，待审核简历将无法投递</div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">拥有简历才可报名兼职</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.com_resume_partapply" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.com_resume_partapply" label="0">关闭</el-radio>
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
                        <div class="TableTite">简历姓名限制</div>
                    </td>
                    <td>
                        <div class="TableButn">
                            <el-radio v-model="ruleForm.sy_resumename_num" label="1">开启</el-radio>
                            <el-radio v-model="ruleForm.sy_resumename_num" label="0">关闭</el-radio>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <div class="tc_checktip">
                                若选择"开启"，简历姓名不少于2个汉字,不大于6个汉字,只能是汉字禁止其他字符
                            </div>
                        </div>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="TableTite">简历重复投递周期</div>
                    </td>
                    <td>
                        <div class="TableInpt">
                            <el-input v-model="ruleForm.sq_resume_interval" placeholder="请输入数字">
                                <span slot="suffix" class="slotspan">天</span>
                            </el-input>
                        </div>
                    </td>
                    <td>
                        <div class="TableShuom">
                            <span>即使个人撤回申请或企业删除投递记录，X天内也无法重复投递。为0则不限制</span>
                        </div>
                    </td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="setBasicButn" style="border: none; height: 80px;">
            <el-button type="primary" size="medium" @click="submitForm('ruleForm')" :disabled="submitLoading">提交
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
                config: '提交',
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
            httpPost('m=user&c=users_userset&a=index', params).then(function (response) {
                let res = response.data;
                if (res.error === 0) {
                    let config = res.data.config ? res.data.config : {};
                    //----信息审核
                    //优质简历
                    _this.ruleForm.user_height_resume = config.user_height_resume == '1' ? true : false;
                    //身份证
                    _this.ruleForm.user_idcard_status = config.user_idcard_status == '1' ? true : false;
                    //求职咨询
                    _this.ruleForm.user_msg_status = config.user_msg_status == '0' ? true : false;
                    //头像
                    _this.ruleForm.user_photo_status = config.user_photo_status == '1' ? true : false;
                    //作品
                    _this.ruleForm.rshow_photo_status = config.rshow_photo_status == '1' ? true : false;
                    //委托简历
                    _this.ruleForm.user_trust_status = config.user_trust_status == '0' ? true : false;
                    //----简历审核
                    //创建简历
                    _this.ruleForm.resume_status = config.resume_status == '0' ? true : false;
                    //修改简历
                    _this.ruleForm.user_revise_state = config.user_revise_state == '0' ? true : false;
                    _this.ruleForm.resume_statetime_start = config.resume_statetime_start !== undefined ? config.resume_statetime_start : '';
                    _this.ruleForm.resume_statetime_end = config.resume_statetime_end !== undefined ? config.resume_statetime_end : '';
                    //----强制操作
                    //创建简历
                    _this.ruleForm.user_resume_status = config.user_resume_status == '1' ? true : false;
                    //关注微信公众号
                    _this.ruleForm.user_gzgzh = config.user_gzgzh == '1' ? true : false;
                    //----快速投递
                    _this.ruleForm.resume_kstd = config.resume_kstd !== undefined ? config.resume_kstd : null;
                    //----简历创建必填项
                    //工作经历
                    _this.ruleForm.resume_create_exp = config.resume_create_exp == '1' ? true : false;
                    //教育经历
                    _this.ruleForm.resume_create_edu = config.resume_create_edu == '1' ? true : false;
                    //项目经历
                    _this.ruleForm.resume_create_project = config.resume_create_project == '1' ? true : false;
                    //工作经历选项
                    if (config.expcreate !== undefined && config.expcreate.length) {
                        _this.ruleForm.expcreate = config.expcreate.split(',');
                    } else {
                        _this.ruleForm.expcreate = [];
                    }
                    //教育经历选项
                    if (config.educreate !== undefined && config.educreate.length) {
                        _this.ruleForm.educreate = config.educreate.split(',');
                    } else {
                        _this.ruleForm.educreate = [];
                    }
                    //设置不强制填写职位类别项
                    if (config.sy_resume_job_classid !== undefined && config.sy_resume_job_classid.length) {
                        _this.ruleForm.sy_resume_job_classid = config.sy_resume_job_classid.split(',');
                        _this.jobSelected = JSON.parse(JSON.stringify(res.data.selected))
                    } else {
                        _this.ruleForm.sy_resume_job_classid = [];
                    }
                    //跨职位类别投递，是否要判断完整度  1 0
                    _this.ruleForm.sy_resume_kh_td = config.sy_resume_kh_td !== undefined ? config.sy_resume_kh_td : null;
                    //快速投递，是否遵从简历必填项  1 2
                    _this.ruleForm.resume_kstd_req = config.resume_kstd_req !== undefined ? config.resume_kstd_req : null;
                    //----申请职位要求简历完整度达到
                    _this.ruleForm.user_sqintegrity = config.user_sqintegrity !== undefined ? config.user_sqintegrity : '';
                    //----简历置顶要求
                    //工作经历
                    _this.ruleForm.user_work_regiser = config.user_work_regiser == '1' ? true : false;
                    //教育经历
                    _this.ruleForm.user_edu_regiser = config.user_edu_regiser == '1' ? true : false;
                    //项目经历
                    _this.ruleForm.user_project_regiser = config.user_project_regiser == '1' ? true : false;
                    //简历求职意向字数
                    _this.ruleForm.sy_rname_num = config.sy_rname_num !== undefined ? config.sy_rname_num : '';
                    //个人会员发布简历数
                    _this.ruleForm.user_number = config.user_number !== undefined ? config.user_number : '';
                    //个人搜索器数量
                    _this.ruleForm.user_finder = config.user_finder !== undefined ? config.user_finder : '';
                    //个人会员向网站委托简历数
                    _this.ruleForm.user_trust_number = config.user_trust_number !== undefined ? config.user_trust_number : '';
                    //姓名展示
                    _this.ruleForm.user_name = config.user_name !== undefined ? config.user_name : null;
                    //个人简历头像
                    _this.ruleForm.user_pic = config.user_pic !== undefined ? config.user_pic : null;
                    //个人简历刷新类型
                    _this.ruleForm.resume_sx = config.resume_sx !== undefined ? config.resume_sx : null;
                    //简历模糊化
                    _this.ruleForm.resume_open_check = config.resume_open_check !== undefined ? config.resume_open_check : null;
                    //个人用户访问简历权限
                    _this.ruleForm.sy_user_visit_resume = config.sy_user_visit_resume !== undefined ? config.sy_user_visit_resume : null;
                    //待审核简历可以投递
                    _this.ruleForm.sy_shresume_applyjob = config.sy_shresume_applyjob !== undefined ? config.sy_shresume_applyjob : null;
                    //拥有简历才可报名兼职
                    _this.ruleForm.com_resume_partapply = config.com_resume_partapply !== undefined ? config.com_resume_partapply : null;
                    //简历姓名限制
                    _this.ruleForm.sy_resumename_num = config.sy_resumename_num !== undefined ? config.sy_resumename_num : null;
                    //简历重复投递周期
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
