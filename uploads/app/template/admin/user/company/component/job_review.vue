<template>
    <div class="shbox">
        <div class="shinfo">
            <div class="shcomname">{{info.jobname}}</div>
            <div class="jobshcom">{{info.com_name}}
                <el-tag type="danger" v-if="info.rating_name" size="mini">{{info.rating_name}}</el-tag></div>
            <div class="sh_zwsz_add">
                联系人：{{info.linkman}} <span class="shcomtel_n">联系电话：{{info.tel}} </span> <span v-if="info.crm_name">业务员：{{info.crm_name}}</span>
            </div>
            <div class="shcomtel">
                <template v-if="info.reg_date_n">注册时间：{{info.reg_date_n}}</template>
                <template v-if="info.login_date_n">
                    <span class="shcomtel_n">最近登录时间：{{ info.login_date_n }} </span>
                    <span v-if="info.add_ip">IP：{{ info.add_ip }}</span>
                    <span v-if="info.add_ip" class="shcomtel_n">IP归属地：{{ info.ip_address }}</span>
                </template>
                <template v-else>
                    <span class="shcomtel_n">未登录</span>
                </template>
            </div>
            <div class="shshowall">
                <div class="shshow">
                    <div class="shshow_tit"><i class="el-icon-document"></i> 基本要求</div>
                    <div class="shshow_p">
                        <div class="" v-if="info.job_welfare">职位福利：
                            <el-tag size="mini" v-for="(item,key) in info.job_welfare" :key="key" style="margin-right: 5px;">{{item}}</el-tag>
                        </div>
                        <div class="">职位薪资：{{info.job_salary}} </div>
                        <div class="">经验要求：{{info.job_exp}} </div>
                        <div class="">学历要求：{{info.job_edu}} </div>
                        <div class="" v-if="info.job_number">招聘人数：{{info.job_number}}</div>
                        <div class="" v-else>招聘人数：若干人</div>
                        <div class="">到岗时间：{{info.job_report}} </div>
                        <div class="" v-if="info.job_sex">性别要求：{{info.job_sex}} </div>
                        <div class="" v-else>性别要求：不限性别 </div>
                        <div class="">婚况要求：{{info.job_marriage}}</div>
                        <div class="">工作地址：{{info.address}}</div>
                    </div>
                    <div class="shshow_tit"><i class="el-icon-office-building"></i> 职位描述</div>
                    <div class="shshow_p">
                        <div class="" v-html="info.description"></div>
                    </div>
                </div>
                <div class="shcz">
                    <template v-if="is_graduate==1 ">
                        <div v-if="r_status != 1">
                            <div class="wxsettip_small ">企业审核 </div>
                            <template>
                                <el-radio v-model="info.r_status" label="1">通过</el-radio>
                                <el-radio v-model="info.r_status" label="3">未通过</el-radio>
                            </template>
                            <div class="wxsettip_small ">职位审核 </div>
                            <el-checkbox v-model="job_status">同步审核</el-checkbox>
                            <div class="admin_jobshtip">同步说明：当前职位根据企业审核（除了未通过）同步审核</div>
                        </div>
                        <div v-else>
                            <div class="wxsettip_small ">职位审核 </div>
                            <template>
                                <el-radio v-model="info.state" label="1">通过</el-radio>
                                <el-radio v-model="info.state" label="3">未通过</el-radio>
                            </template>
                            <div class="wxsettip_small ">审核说明模板</div>
                            <el-select v-model="info.tpl" placeholder="请选择" @change="tplChange" clearable>
                                <el-option v-for="item in job_audit" :key="item" :label="comclass_name[item]"
                                           :value="item">
                                </el-option>
                            </el-select>
                        </div>
                        <div class="wxsettip_small ">审核状态说明 </div>
                        <el-input type="textarea" :rows="2" placeholder="请输入内容" v-model="info.statusbody">
                        </el-input>
                        <div class=" shczbth">
                            <el-button type="primary" @click="audit(1)" :disabled="submitLoading">提 交</el-button>
                        </div>
                        <div class=" shczbth" v-if="snum>1">
                            <el-button type="primary" @click="audit(2)" :disabled="submitLoading" plain>提交，并审核下一个</el-button>
                        </div>
                    </template>
                    <template v-else>
                        <div v-if="info.c_status == 2">
                            <div class="wxsettip_small ">锁定状态</div>
                            <template>
                                <el-radio v-model="info.c_status" label="1">正常</el-radio>
                                <el-radio v-model="info.c_status" label="2">锁定</el-radio>
                            </template>
                            <div class="wxsettip_small ">锁定原因</div>
                            <el-input type="textarea" disabled :rows="2" placeholder="锁定原因" :value="info.statusbody"></el-input>
                        </div>
                        <div v-else class="shcz">
                            <div class="wxsettip_small ">职位审核</div>
                            <template>
                                <el-radio v-model="info.state" label="1">通过</el-radio>
                                <el-radio v-model="info.state" label="3">未通过</el-radio>
                            </template>
                            <div class="wxsettip_small ">审核说明模板</div>
                            <el-select v-model="info.tpl" placeholder="请选择" @change="tplChange">
                                <el-option v-for="(item, index) in job_audit" :key="index" :label="comclass_name[item]" :value="item"></el-option>
                            </el-select>
                            <div class="wxsettip_small " v-if="info.r_status == 0">企业审核</div>
                            <template>
                                <el-checkbox v-if="info.r_status == 0" :value="true" disabled>同步审核</el-checkbox>
                            </template>
                            <div class="wxsettip_small ">审核状态说明</div>
                            <el-input type="textarea" :rows="2" placeholder="请输入审核状态说明" v-model="info.statusbody"></el-input>
                            <div class=" shczbth">
                                <el-button type="primary" :disabled="submitLoading" @click="audit(1)">提 交</el-button>
                            </div>
                            <div class=" shczbth" v-if="snum > 0">
                                <el-button type="primary" :disabled="submitLoading" @click="audit(2)" plain>提交，并审核下一个</el-button>
                            </div>
                        </div>
                    </template>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
module.exports = {
    props:{
        // that.auditInfo.state = that.auditInfo.state == 3 ? '3' : '1',
        comclass_name:{
            type :Object,
            default:{}
        },
        is_graduate:{
            type: Number,
            default:0
        },
        job_audit:{
            type:Array,
            default:[]
        },
        id:{
            type:Number,
            default:[]
        }
    },
    data: function () {
        return {
            loading: false,
            submitLoading:false,
            type: '1',
            tpl:'',
            total: 0,
            tableHig: true,
            tableData: [],
            weburl: '',
            comuid: '',
            sort_type: '',
            sort_col: '',
            cansearch: true,
            prevPage: 0,
            job_status: this.is_graduate == 1 ?true:false,
            info:{
                state:''
            },
            snum:0,
            r_status:''
        }
    },
    watch: {
        id: {
            handler(val) {
                this.status(val);
            },
            immediate: true,
            deep: true,
        },
    },
    methods: {
        audit(atype){
            let that = this;
            let url = '';
            let params = {};
            if (this.is_graduate == 0){
                if (!that.info.state) {
                    message.error("请选择审核状态")
                    return false;
                }
                params = {
                    single: 1,
                    status: that.info.state,
                    pid: that.info.id,
                    uid: that.info.uid,
                    statusbody: that.info.statusbody,
                    atype: atype
                };
                if (that.info.c_status == 2) {
                    message.error("锁定操作异常")
                    return false;
                } else {
                    params.lock_status = 1;
                }
                if (that.info.r_status == '0') {
                    url = 'm=user&c=company_job&a=cjobstatus';
                } else {
                    url = 'm=user&c=company_job&a=status';
                }
            }else{
                params = {
                    single: 1,
                    atype: atype,
                };
                if (that.submitLoading) {
                    return;
                }
                that.submitLoading = true;
                if (that.r_status != 1){
                    params.r_status = that.info.r_status;
                    params.job_status = 1;
                    params.statusbody = that.info.statusbody;
                    params.cid = that.info.id;
                    params.cuid = that.info.uid;
                    url = "m=user&c=school_graduate&a=cjobstatus";
                }else{
                    url = 'm=user&c=school_graduate&a=status';
                    params.status = that.info.state;
                    params.pid = that.info.id;
                    params.statusbody = that.info.statusbody;
                }
            }
            httpPost(url, params).then(function(response) {
                let res = response.data;
                that.submitLoading = false;
                if (res.error == 0){
                    message.success(res.msg);
                    if (atype == 1){
                        that.$emit("confirm");
                    }else{
                        let id = '';
                        if (that.is_graduate == 0){
                            id = res.data.job.id;
                        }else{
                            id = res.data.id;
                        }
                        if(id){
                            that.status(id);
                        }
                    }
                }else{
                    message.error(res.msg);
                }
            })
        },
        status(id){
            let that = this;
            let url = '';
            if (this.is_graduate == 0){
                url = 'm=user&c=company_job&a=jobAudit';
            }else {
                url = "m=user&c=school_graduate&a=jobAudit";
            }
            httpPost(url, {id:id},{hideloading: true}).then(function(response) {
                let res = response.data;
                that.info = res.data.info;
                that.snum = res.data.snum;
                that.info.state = res.data.info.state =="0"?"1":res.data.info.state;
                that.r_status = res.data.info.r_status;
            })
        },
        tplChange(e){
            this.info.statusbody = this.comclass_name[e];
        },
    },
};
</script>