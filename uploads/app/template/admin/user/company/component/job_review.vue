<template>
    <div class="shbox">
        <div class="shinfo">
            <div class="shcomname">{{info.jobname}}</div>
            <div class="jobshcom">{{info.com_name}}
                <el-tag type="danger" v-if="info.rating_name" size="mini">{{info.rating_name}}</el-tag></div>
            <div class="sh_zwsz_add">
                {{ lc("admin_contact_person_value", [info.linkman]) }} <span class="shcomtel_n">{{ lc("admin_contact_phone_value", [info.tel]) }} </span> <span v-if="info.crm_name">{{ lc("admin_salesperson_value", [info.crm_name]) }}</span>
            </div>
            <div class="shcomtel">
                <template v-if="info.reg_date_n">{yun:}t key='admin_00734'{/yun}</template>
                <template v-if="info.login_date_n">
                    <span class="shcomtel_n">{yun:}t key='admin_00735'{/yun} </span>
                    <span v-if="info.add_ip">IP：{{ info.add_ip }}</span>
                    <span v-if="info.add_ip" class="shcomtel_n">{yun:}t key='admin_00736'{/yun}</span>
                </template>
                <template v-else>
                    <span class="shcomtel_n">{yun:}t key='admin_user_00139'{/yun}</span>
                </template>
            </div>
            <div class="shshowall">
                <div class="shshow">
                    <div class="shshow_tit"><i class="el-icon-document"></i> {yun:}t key='member_user_00194'{/yun}</div>
                    <div class="shshow_p">
                        <div class="" v-if="info.job_welfare">{yun:}t key='admin_00737'{/yun}
                            <el-tag size="mini" v-for="(item,key) in info.job_welfare" :key="key" style="margin-right: 5px;">{{item}}</el-tag>
                        </div>
                        <div class="">{yun:}t key='admin_00738'{/yun} </div>
                        <div class="">{yun:}t key='admin_00739'{/yun} </div>
                        <div class="">{yun:}t key='admin_00740'{/yun} </div>
                        <div class="" v-if="info.job_number">{{ lc("admin_headcount_value", [info.job_number]) }}</div>
                        <div class="" v-else>{yun:}t key='admin_user_company_00324'{/yun}</div>
                        <div class="">{{ lc("admin_arrival_time_value", [info.job_report]) }} </div>
                        <div class="" v-if="info.job_sex">{{ lc("admin_gender_requirement_value", [info.job_sex]) }} </div>
                        <div class="" v-else>{yun:}t key='admin_00741'{/yun} </div>
                        <div class="">{yun:}t key='admin_00742'{/yun}</div>
                        <div class="">{{ lc("admin_work_address_value", [info.address]) }}</div>
                    </div>
                    <div class="shshow_tit"><i class="el-icon-office-building"></i> {yun:}t key='wap_com_00289'{/yun}</div>
                    <div class="shshow_p">
                        <div class="" v-html="info.description"></div>
                    </div>
                </div>
                <div class="shcz">
                    <template v-if="is_graduate==1 ">
                        <div v-if="r_status != 1">
                            <div class="wxsettip_small ">{yun:}t key='admin_user_company_00134'{/yun} </div>
                            <template>
                                <el-radio v-model="info.r_status" label="1">{yun:}t key='admin_user_company_00161'{/yun}</el-radio>
                                <el-radio v-model="info.r_status" label="3">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                            </template>
                            <div class="wxsettip_small ">{yun:}t key='admin_user_company_00326'{/yun} </div>
                            <el-checkbox v-model="job_status">{yun:}t key='admin_user_company_00325'{/yun}</el-checkbox>
                            <div class="admin_jobshtip">{yun:}t key='admin_user_company_00323'{/yun}</div>
                        </div>
                        <div v-else>
                            <div class="wxsettip_small ">{yun:}t key='admin_user_company_00326'{/yun} </div>
                            <template>
                                <el-radio v-model="info.state" label="1">{yun:}t key='admin_user_company_00161'{/yun}</el-radio>
                                <el-radio v-model="info.state" label="3">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                            </template>
                            <div class="wxsettip_small ">{yun:}t key='admin_user_00244'{/yun}</div>
                            <el-select v-model="info.tpl" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="tplChange" clearable>
                                <el-option v-for="item in job_audit" :key="item" :label="comclass_name[item]"
                                           :value="item">
                                </el-option>
                            </el-select>
                        </div>
                        <div class="wxsettip_small ">{yun:}t key='admin_user_00365'{/yun} </div>
                        <el-input type="textarea" :rows="2" placeholder="{yun:}t key='wap_user_00076'{/yun}" v-model="info.statusbody">
                        </el-input>
                        <div class=" shczbth">
                            <el-button type="primary" @click="audit(1)" :disabled="submitLoading">{yun:}t key='member_com_00248'{/yun}</el-button>
                        </div>
                        <div class=" shczbth" v-if="snum>1">
                            <el-button type="primary" @click="audit(2)" :disabled="submitLoading" plain>{yun:}t key='admin_user_00239'{/yun}</el-button>
                        </div>
                    </template>
                    <template v-else>
                        <div v-if="info.c_status == 2">
                            <div class="wxsettip_small ">{yun:}t key='admin_00743'{/yun}</div>
                            <template>
                                <el-radio v-model="info.c_status" label="1">{yun:}t key='admin_user_00149'{/yun}</el-radio>
                                <el-radio v-model="info.c_status" label="2">{yun:}t key='admin_user_00150'{/yun}</el-radio>
                            </template>
                            <div class="wxsettip_small ">{yun:}t key='admin_00744'{/yun}</div>
                            <el-input type="textarea" disabled :rows="2" placeholder="{yun:}t key='admin_00744'{/yun}" :value="info.statusbody"></el-input>
                        </div>
                        <div v-else class="shcz">
                            <div class="wxsettip_small ">{yun:}t key='admin_user_company_00326'{/yun}</div>
                            <template>
                                <el-radio v-model="info.state" label="1">{yun:}t key='admin_user_company_00161'{/yun}</el-radio>
                                <el-radio v-model="info.state" label="3">{yun:}t key='wap_user_00167'{/yun}</el-radio>
                            </template>
                            <div class="wxsettip_small ">{yun:}t key='admin_user_00244'{/yun}</div>
                            <el-select v-model="info.tpl" placeholder="{yun:}t key='wap_user_00100'{/yun}" @change="tplChange">
                                <el-option v-for="(item, index) in job_audit" :key="index" :label="comclass_name[item]" :value="item"></el-option>
                            </el-select>
                            <div class="wxsettip_small " v-if="info.r_status == 0">{yun:}t key='admin_user_company_00134'{/yun}</div>
                            <template>
                                <el-checkbox v-if="info.r_status == 0" :value="true" disabled>{yun:}t key='admin_user_company_00325'{/yun}</el-checkbox>
                            </template>
                            <div class="wxsettip_small ">{yun:}t key='admin_user_00365'{/yun}</div>
                            <el-input type="textarea" :rows="2" placeholder="{yun:}t key='admin_00745'{/yun}" v-model="info.statusbody"></el-input>
                            <div class=" shczbth">
                                <el-button type="primary" :disabled="submitLoading" @click="audit(1)">{yun:}t key='member_com_00248'{/yun}</el-button>
                            </div>
                            <div class=" shczbth" v-if="snum > 0">
                                <el-button type="primary" :disabled="submitLoading" @click="audit(2)" plain>{yun:}t key='admin_user_00239'{/yun}</el-button>
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
                    message.error("{yun:}t key='admin_user_weipin_00015'{/yun}")
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
                    message.error("{yun:}t key='admin_company_00001'{/yun}")
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