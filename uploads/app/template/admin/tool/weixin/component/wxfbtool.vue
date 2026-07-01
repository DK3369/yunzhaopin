<template>
    <div class="moduleElHight wxfbtool">
        <div class="moduleTable" style="display: flex; justify-content: space-between;">
            
            <div class="tableVueWidth">
                <table class="tableVue" style="overflow-y: auto; height: 100%;">
                    <thead>
                        <tr align="left">
                            <th width="150">{{ lc('member_com_00021') }}</th>
                            <th>{{ lc('member_user_00181') }}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00624') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="temptype" @input="temptypeFun">
                                        <el-radio label="0">{{ lc('admin_tool_00630') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_tool_00612') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00629') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="type" @input="typeFun">
                                        <el-radio label="job">{{ lc('default_00236') }}</el-radio>
                                        <el-radio label="resume">{{ lc('admin_tool_00631') }}</el-radio>
                                        <el-radio label="company">{{ lc('default_00113') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job' || type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('admin_user_company_00018') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox-group v-model="rating">
                                        <el-checkbox v-for="rat in ratinglist" :key="rat.id" :label="rat.id">{{rat.name}}</el-checkbox>
                                    </el-checkbox-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00236') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox-group v-model="jobtg">
                                        <el-checkbox label="0">{{ lc('member_com_00327') }}</el-checkbox>
                                        <el-checkbox label="1">{{ lc('member_com_00326') }}</el-checkbox>
                                        <el-checkbox label="2">{{ lc('home.recommended_jobs') }}</el-checkbox>
                                    </el-checkbox-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('admin_user_company_00272') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="times">
                                        <el-radio label="0">{{ lc('common_01936') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_tool_00616') }}</el-radio>
                                        <el-radio label="3">{{ lc('admin_tool_00619') }}</el-radio>
                                        <el-radio label="7">{{ lc('admin_tool_00622') }}</el-radio>
                                        <el-radio label="15">{{ lc('admin_tool_00609') }}</el-radio>
                                        <el-radio label="30">{{ lc('admin_tool_00610') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00634') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="ftimes">
                                        <el-radio label="0">{{ lc('common_01936') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_tool_00616') }}</el-radio>
                                        <el-radio label="3">{{ lc('admin_tool_00619') }}</el-radio>
                                        <el-radio label="7">{{ lc('admin_tool_00622') }}</el-radio>
                                        <el-radio label="15">{{ lc('admin_tool_00609') }}</el-radio>
                                        <el-radio label="30">{{ lc('admin_tool_00610') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00607') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="vtimes">
                                        <el-radio label="0">{{ lc('common_01936') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_tool_00616') }}</el-radio>
                                        <el-radio label="7">{{ lc('admin_tool_00615') }}</el-radio>
                                        <el-radio label="30">{{ lc('admin_tool_00617') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00550') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="jobcopos" multiple filterable remote reserve-keyword :placeholder="lc('wap_00510')" :remote-method="job_search" :loading="job_searchloading">
                                        <el-option v-for="item in job_search_list" :key="item.value" :label="item.name" :value="item.value">
                                            <span style="float: left; color: #333; font-size: 14px;font-weight:bold;">{{item.name}}</span>
                                            <span style="float: right; color: #a5a5a5; font-size: x-small;">{{item.upname}}</span>
                                        </el-option>
                                    </el-select>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i> {{ lc('admin_tool_00604') }}</div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00527') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="copos" multiple filterable remote reserve-keyword :placeholder="lc('admin_tool_00635')" :remote-method="company_search" :loading="company_searchloading">
                                        <el-option v-for="item in company_search_list" :key="item.value" :label="item.name" :value="item.value">
                                            <span style="float: left; color: #333; font-size: 14px;font-weight:bold;">{{item.name}}</span>
                                        </el-option>
                                    </el-select>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i>{{ lc('admin_tool_00605') }}</div>
                            </td>
                        </tr>
                        <tr v-show="type=='job' || type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('member_user_00198') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-cascader :options="cityoptions" @change="jobCityChoose" :props="{checkStrictly: true}" clearable></el-cascader>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('wap_user_00018') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-cascader :options="joboptions" @change="jobclassChoose" :props="{checkStrictly: true}" clearable></el-cascader>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('admin_user_company_00373') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="hy" :placeholder="lc('wap_user_00100')">
                                        <el-option v-for="item in hi" :key="item" :label="hyname[item]" :value="item"></el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00290') }}</div>
                            </td>
                            <td>
                                <div class="jianlidian">
                                    <div class="TableInpt">
                                        <el-input type="number" :placeholder="lc('wap_user_00013')" v-model="minsalary">
                                            <span slot="suffix" class="slotspan">{{ lc('common_02056') }}</span>
                                        </el-input>
                                    </div>
                                    <div class="hxian"></div>
                                    <div class="TableInpt">
                                        <el-input type="number" :placeholder="lc('wap_user_00014')" v-model="maxsalary">
                                            <span slot="suffix" class="slotspan">{{ lc('common_02056') }}</span>
                                        </el-input>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00173') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input :placeholder="lc('wap_user_00076')" v-model="welfare"></el-input>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i> {{ lc('admin_tool_00625') }}</div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">{{ lc('admin_00149') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input :placeholder="lc('wap_user_00076')" v-model="keyword"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('member_com_00107') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox :true-label='1' v-model="cvkind">{{ lc('wap_user_00210') }}</el-checkbox>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00353') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <div class="TableInpt">
                                        <!--7.0 统一类别选择-->
                                        <job_class @confirm="confirmJob" :selected="jobSelected"></job_class>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00127') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <div class="TableInpt">
                                        <!--7.0 统一城市选择-->
                                        <city_class multiple :max="5" @confirm="confirmCity" :selected="citySelected"></city_class>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00633') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="rtimes">
                                        <el-radio label="0">{{ lc('common_01936') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_tool_00616') }}</el-radio>
                                        <el-radio label="3">{{ lc('admin_tool_00619') }}</el-radio>
                                        <el-radio label="7">{{ lc('admin_tool_00622') }}</el-radio>
                                        <el-radio label="15">{{ lc('admin_tool_00609') }}</el-radio>
                                        <el-radio label="30">{{ lc('admin_tool_00610') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00632') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="rltimes">
                                        <el-radio label="0">{{ lc('common_01936') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_tool_00616') }}</el-radio>
                                        <el-radio label="3">{{ lc('admin_tool_00619') }}</el-radio>
                                        <el-radio label="7">{{ lc('admin_tool_00622') }}</el-radio>
                                        <el-radio label="15">{{ lc('admin_tool_00609') }}</el-radio>
                                        <el-radio label="30">{{ lc('admin_tool_00610') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00283') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox-group v-model="bd">
                                        <el-checkbox v-for="item in  useri.user_edu" :key="item" :label="item">{{usern[item]}}</el-checkbox>
                                    </el-checkbox-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('wap_com_00287') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox-group v-model="exp">
                                        <el-checkbox v-for="item in  useri.user_word" :key="item" :label="item">{{usern[item]}}</el-checkbox>
                                    </el-checkbox-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">{{ lc('wap_00328') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="whole">
                                        <el-radio label="55">{{ lc('admin_tool_00626') }}</el-radio>
                                        <el-radio label="75">{{ lc('admin_tool_00627') }}</el-radio>
                                        <el-radio label="95">{{ lc('admin_tool_00628') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00608') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="did" :placeholder="lc('wap_user_00100')">
                                        <el-option value="">{{ lc('common.all') }}</el-option>
                                        <el-option v-for="(item,key) in domain" :key="key" :label="item" :value="key"></el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('wap_00087') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox :true-label='1' v-model="bekind">{{ lc('wap_00299') }}</el-checkbox>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00613') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="rule">
                                        <el-radio label="0">{{ lc('common_01936') }}</el-radio>
                                        <el-radio label="1">{{ lc('admin_user_00256') }}</el-radio>
                                        <el-radio label="2">{{ lc('admin_tool_00623') }}</el-radio>
                                        <el-radio label="3">{{ lc('admin_tool_00576') }}</el-radio>
                                    </el-radio-group>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i>{{ lc('admin_tool_00606') }}</div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00614') }}</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="tpl">
                                        <el-radio v-for="item in temps" :key="item.id" :label="item.id">{{item.title}}</el-radio>
                                    </el-radio-group>
                                </div>
                                <div class="TableInpt">
                                    <el-button @click="urlchange">{{ lc('member_com_00354') }}</el-button>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">{{ lc('admin_tool_00611') }}</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="num">
                                        <el-radio label="10">{{ lc('api_alipay_00001') }}</el-radio>
                                        <el-radio label="20">{{ lc('admin_user_company_00299') }}</el-radio>
                                        <el-radio label="30">{{ lc('admin_tool_00618') }}</el-radio>
                                        <el-radio label="40">{{ lc('admin_tool_00620') }}</el-radio>
                                        <el-radio label="50">{{ lc('admin_tool_00621') }}</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="wxfbtooltable">
                <div class="wxfbtoolshoww">
                    <!--wxpubtool_gj_con样式需要有white-space: pre-wrap;  wxpubtool_gj_con_1不需要-->
                    <div id="content" v-html="htmlcon" :class="temptype=='1'?'wxpubtool_gj_con':'wxpubtool_gj_con_1'"></div>
                </div>
                <div class="wxfbtoolfuzhis">
                    <input type="button" class="wxpubtool_CZbth" id="copy" data-clipboard-action="copy" data-clipboard-target="#content" value=lc('admin_tool_00526') />
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" :loading='post_loading' @click="save(false)">{{ lc('common.submit') }}</el-button>
        </div>
    </div>
</template>
<script>
var weburl = localStorage.getItem("sy_weburl");

module.exports = {
    data: function() {
        return {
            alltemps: [],
            temps: [],
            ratinglist: [],
            domain: {},
            hi: hi,
            hyname: hyname,
            useri: useri,
            usern: usern,

            job_searchloading: false,
            job_search_list: [],

            company_searchloading: false,
            company_search_list: [],

            cityoptions: [],
            joboptions: [],

            temptype: '0',
            type: 'job',
            rating: [],
            jobtg: [],
            times: '0',
            ftimes: '0',
            vtimes: '0',
            jobcopos: [],
            copos: [],
            provinceid: '',
            cityid: '',
            three_cityid: '',
            job1: '',
            job1_son: '',
            job_post: '',
            minsalary: '',
            maxsalary: '',
            welfare: '',
            keyword: '',
            num: '10',
            hy: '',
            bd: [],
            exp: [],
            cvkind: 0,
            rtimes: '0',
            rltimes: '0',
            whole: '55',
            did: '',
            bekind: '',
            rule: '0',
            // jobclass_list:[],
            job_classid: '',
            // cityclass_list:[],
            city_classid: '',
            tpl: '',
            jobSelected: null,
            citySelected: null,

            post_loading: false,
            htmlcon: '',
        }
    },
    components: {
        'job_class': httpVueLoader('../../../component/job_class.vue'),
        'city_class': httpVueLoader('../../../component/city_class.vue'),
    },
    mounted() {
        this.init();
    },
    methods: {
        confirmJob(data) {
            this.job_classid = data.jobId.join(',');
        },
        confirmCity(data) {
            this.city_classid = data.cityId.join(',');
        },
        init: function() {

            this.cityoptions = cityCascader();
            this.joboptions = jobCascader();

            const clipboard = new ClipboardJS("#copy");
            clipboard.on('success', function(e) {
                message.success(window.yunAdminT(lc('wap_com_00254')));
                e.clearSelection();
            });
            clipboard.on('error', function(e) {});

            this.getInfo();
        },
        showtemps: function() {

            var alltemps = deepClone(this.alltemps);
            var temptype = this.temptype;
            var type = this.type;
            var tpl = '';
            var temps = [];

            for (let i in alltemps) {
                if (alltemps[i].type == type && alltemps[i].temptype == temptype) {
                    temps.push(alltemps[i]);
                }
            }

            if (temps.length > 0) {
                tpl = temps[0].id;
            }

            this.temps = temps;
            this.tpl = tpl;
        },
        async getInfo() {

            let that = this;

            httpPost('m=tool&c=fabutool&a=pubtool', {}).then((result) => {

                var res = result.data;

                if (res.error == 0) {
                    that.alltemps = res.data.temps;
                    that.ratinglist = res.data.rating;
                    that.domain = res.data.domain;

                    this.showtemps();
                    this.save();
                }
            }).catch(function(e) {
                console.log(e)
            })
        },
        temptypeFun: function() {
            this.showtemps();
        },
        typeFun: function() {
            this.showtemps();
        },
        async job_search(query) {

            let that = this;

            var job_search_list = [];


            that.job_searchloading = true;

            var params = {
                keyword: query
            };

            httpPost('m=tool&c=fabutool&a=getWxpubJob', params).then((result) => {
                that.job_searchloading = false;
                var res = result.data;

                if (res.error == 0) {
                    job_search_list = res.data;
                }

                that.job_search_list = job_search_list;

            }).catch(function(e) {
                console.log(e)
            })
        },
        async company_search(query) {

            let that = this;

            var company_search_list = [];


            that.company_searchloading = true;

            var params = {
                keyword: query
            };

            httpPost('m=tool&c=fabutool&a=getComBySearch', params).then((result) => {
                that.company_searchloading = false;
                var res = result.data;

                if (res.error == 0) {
                    company_search_list = res.data;
                }

                that.company_search_list = company_search_list;

            }).catch(function(e) {
                console.log(e)
            })
        },
        jobCityChoose: function(val) {

            var provinceid = cityid = three_cityid = '';

            if (val.length > 0) {
                provinceid = val[0];
                cityid = val.length > 1 ? val[1] : '';
                three_cityid = val.length > 2 ? val[2] : '';
            }

            this.provinceid = provinceid;
            this.cityid = cityid;
            this.three_cityid = three_cityid;

        },
        jobclassChoose: function(val) {

            var job1 = job1_son = job_post = '';

            if (val.length > 0) {
                job1 = val[0];
                job1_son = val.length > 1 ? val[1] : '';
                job_post = val.length > 2 ? val[2] : '';
            }

            this.job1 = job1;
            this.job1_son = job1_son;
            this.job_post = job_post;
        },
        save: function(hideloading = true) {

            let that = this;
            var postData = {
                type: this.type,
                num: this.num,
                tpl: this.tpl,
            };
            var type = this.type;

            if (type == 'job') {

                postData.param = this.jobtg.join(",");

                postData.rating = this.rating.join(",");
                postData.times = this.times;
                postData.ftimes = this.ftimes;
                postData.jobcopos = this.jobcopos.join(",");
                postData.keyword = this.keyword;
                postData.welfare = this.welfare;
                postData.cityid = this.cityid;
                postData.provinceid = this.provinceid;
                postData.three_cityid = this.three_cityid;
                postData.job1 = this.job1;
                postData.job1_son = this.job1_son;
                postData.job_post = this.job_post;
                postData.maxsalary = this.maxsalary;
                postData.minsalary = this.minsalary;
            } else if (type == 'resume') {
                postData.rtimes = this.rtimes;
                postData.rltimes = this.rltimes;
                postData.cvkind = this.cvkind;
                postData.job_class = this.job_classid;
                postData.city_class = this.city_classid;
                postData.bd = this.bd.join(",");
                postData.exp = this.exp.join(",");
                postData.whole = this.whole;
                postData.tmptype = '2';
            } else if (type == 'company') {
                postData.vtimes = this.vtimes;
                postData.rating = this.rating.join(",");
                postData.copos = this.copos.join(",");
                postData.did = this.did;
                postData.rule = this.rule;
                postData.bekind = this.bekind;
                postData.cityid = this.cityid;
                postData.provinceid = this.provinceid;
                postData.three_cityid = this.three_cityid;
                postData.hy = this.hy;
            }

            that.post_loading = true;

            httpPost('m=tool&c=fabutool&a=Getpubtool', postData, { hideloading: hideloading }).then((result) => {

                that.post_loading = false;

                that.htmlcon = result.data;

                that.$nextTick(() => {
                    // 将图片转成base64，防止复制到其他地方会跨域
                    $("#content").find('img').each(function() {
                        var _this = $(this);
                        var backImg = new Image();
                        backImg.src = _this.attr('src');;
                        backImg.onload = function() {
                            // 将是人才网域名的图片转成base64
                            if (backImg.src.indexOf(weburl) > -1) {
                                var base = getBase64Image(backImg);
                                _this.attr('src', base);
                            }
                        };
                    });
                })


            }).catch(function(e) {
                console.log(e)
            })
        },
        urlchange:function(){
            // let type = this.type;
            window.parent.homeapp.topage({name: 'addpubtemp',path: '/addpubtemp',params:{sid:'',temptype_v:this.temptype,type:this.type,setType:true}});

        }
    },
};
</script>
<style scoped>
.moduleTable {
    height: calc(100% - 80px);
    max-height: initial;
}

.wxfbtoolfuzhis {
    overflow: hidden;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
}
.wxfbtooltable{
	background: url(../../../images/newsjbg.png) center no-repeat;
	background-size: 100% 100%;
	height: 600px;
	width: 300px;
	margin: 0;
}
.tableVueWidth{
    overflow-y: auto;
    position: relative;
    width: calc(100% - 320px);
    height: 100%;
}
.tableVue{
	width: 100%;
}
.wxfbtoolshoww{
	width: calc(100% - 40px);
	padding: 0 20px;
	border: none;
	height: 470px;
	margin-top: 47px;
}
</style>