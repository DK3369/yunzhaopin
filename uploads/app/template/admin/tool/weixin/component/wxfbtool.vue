<template>
    <div class="moduleElHight wxfbtool">
        <div class="moduleTable" style="display: flex; justify-content: space-between;">
            
            <div class="tableVueWidth">
                <table class="tableVue" style="overflow-y: auto; height: 100%;">
                    <thead>
                        <tr align="left">
                            <th width="150">名称</th>
                            <th>状态</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <div class="TableTite">场景</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="temptype" @input="temptypeFun">
                                        <el-radio label="0">微信公众号文章</el-radio>
                                        <el-radio label="1">社群推文</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">信息类型</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="type" @input="typeFun">
                                        <el-radio label="job">职位列表</el-radio>
                                        <el-radio label="resume">简历列表</el-radio>
                                        <el-radio label="company">企业列表</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job' || type=='company'">
                            <td>
                                <div class="TableTite">会员等级</div>
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
                                <div class="TableTite">职位推广</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox-group v-model="jobtg">
                                        <el-checkbox label="0">置顶职位</el-checkbox>
                                        <el-checkbox label="1">紧急职位</el-checkbox>
                                        <el-checkbox label="2">推荐职位</el-checkbox>
                                    </el-checkbox-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">职位更新时间</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="times">
                                        <el-radio label="0">不限</el-radio>
                                        <el-radio label="1">1天内</el-radio>
                                        <el-radio label="3">3天内</el-radio>
                                        <el-radio label="7">7天内</el-radio>
                                        <el-radio label="15">15天内</el-radio>
                                        <el-radio label="30">30天内</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">职位发布时间</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="ftimes">
                                        <el-radio label="0">不限</el-radio>
                                        <el-radio label="1">1天内</el-radio>
                                        <el-radio label="3">3天内</el-radio>
                                        <el-radio label="7">7天内</el-radio>
                                        <el-radio label="15">15天内</el-radio>
                                        <el-radio label="30">30天内</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">套餐开通时间</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="vtimes">
                                        <el-radio label="0">不限</el-radio>
                                        <el-radio label="1">1天内</el-radio>
                                        <el-radio label="7">1周内</el-radio>
                                        <el-radio label="30">1月内</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">指定职位</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="jobcopos" multiple filterable remote reserve-keyword placeholder="请输入关键词" :remote-method="job_search" :loading="job_searchloading">
                                        <el-option v-for="item in job_search_list" :key="item.value" :label="item.name" :value="item.value">
                                            <span style="float: left; color: #333; font-size: 14px;font-weight:bold;">{{item.name}}</span>
                                            <span style="float: right; color: #a5a5a5; font-size: x-small;">{{item.upname}}</span>
                                        </el-option>
                                    </el-select>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i> 指定职位(输入职位名称、ID、企业名称，选择职位)</div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">指定企业</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="copos" multiple filterable remote reserve-keyword placeholder="输入企业名称" :remote-method="company_search" :loading="company_searchloading">
                                        <el-option v-for="item in company_search_list" :key="item.value" :label="item.name" :value="item.value">
                                            <span style="float: left; color: #333; font-size: 14px;font-weight:bold;">{{item.name}}</span>
                                        </el-option>
                                    </el-select>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i>指定企业(输入企业名称，选择企业)</div>
                            </td>
                        </tr>
                        <tr v-show="type=='job' || type=='company'">
                            <td>
                                <div class="TableTite">工作地点</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-cascader :options="cityoptions" @change="jobCityChoose" :props="{checkStrictly: true}" clearable></el-cascader>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">职位类别</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-cascader :options="joboptions" @change="jobclassChoose" :props="{checkStrictly: true}" clearable></el-cascader>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">行业</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="hy" placeholder="请选择">
                                        <el-option v-for="item in hi" :key="item" :label="hyname[item]" :value="item"></el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">薪资待遇</div>
                            </td>
                            <td>
                                <div class="jianlidian">
                                    <div class="TableInpt">
                                        <el-input type="number" placeholder="最低薪资" v-model="minsalary">
                                            <span slot="suffix" class="slotspan">元</span>
                                        </el-input>
                                    </div>
                                    <div class="hxian"></div>
                                    <div class="TableInpt">
                                        <el-input type="number" placeholder="最高薪资" v-model="maxsalary">
                                            <span slot="suffix" class="slotspan">元</span>
                                        </el-input>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">福利待遇</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="请输入内容" v-model="welfare"></el-input>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i> 1；多项福利同时满足以空格分隔 如：五险 车补 双休
                                    2：多项福利满足一项以|分隔 如：五险|车补|双休</div>
                            </td>
                        </tr>
                        <tr v-show="type=='job'">
                            <td>
                                <div class="TableTite">关键词</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-input placeholder="请输入内容" v-model="keyword"></el-input>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">简历类型</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox :true-label='1' v-model="cvkind">置顶简历</el-checkbox>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">意向职位</div>
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
                                <div class="TableTite">意向地区</div>
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
                                <div class="TableTite">简历发布时间</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="rtimes">
                                        <el-radio label="0">不限</el-radio>
                                        <el-radio label="1">1天内</el-radio>
                                        <el-radio label="3">3天内</el-radio>
                                        <el-radio label="7">7天内</el-radio>
                                        <el-radio label="15">15天内</el-radio>
                                        <el-radio label="30">30天内</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">简历刷新时间</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="rltimes">
                                        <el-radio label="0">不限</el-radio>
                                        <el-radio label="1">1天内</el-radio>
                                        <el-radio label="3">3天内</el-radio>
                                        <el-radio label="7">7天内</el-radio>
                                        <el-radio label="15">15天内</el-radio>
                                        <el-radio label="30">30天内</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='resume'">
                            <td>
                                <div class="TableTite">学历要求</div>
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
                                <div class="TableTite">经验要求</div>
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
                                <div class="TableTite">简历完整度</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="whole">
                                        <el-radio label="55">55%以上</el-radio>
                                        <el-radio label="75">75%以上</el-radio>
                                        <el-radio label="95">95%以上</el-radio>
                                    </el-radio-group>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">站点/分站</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-select v-model="did" placeholder="请选择">
                                        <el-option value="">全部</el-option>
                                        <el-option v-for="(item,key) in domain" :key="key" :label="item" :value="key"></el-option>
                                    </el-select>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">企业类型</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-checkbox :true-label='1' v-model="bekind">名企</el-checkbox>
                                </div>
                            </td>
                        </tr>
                        <tr v-show="type=='company'">
                            <td>
                                <div class="TableTite">职位限制</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="rule">
                                        <el-radio label="0">不限</el-radio>
                                        <el-radio label="1">1条</el-radio>
                                        <el-radio label="2">2条</el-radio>
                                        <el-radio label="3">3条</el-radio>
                                    </el-radio-group>
                                </div>
                                <div class="tool_inputips"><i class="el-icon-question"></i>每家企业最多展示多少职位</div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">风格模板</div>
                            </td>
                            <td>
                                <div class="TableInpt">
                                    <el-radio-group v-model="tpl">
                                        <el-radio v-for="item in temps" :key="item.id" :label="item.id">{{item.title}}</el-radio>
                                    </el-radio-group>
                                </div>
                                <div class="TableInpt">
                                    <el-button @click="urlchange">添加模板</el-button>
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <div class="TableTite">信息数量</div>
                            </td>
                            <td>
                                <div class="TableButn">
                                    <el-radio-group v-model="num">
                                        <el-radio label="10">10条</el-radio>
                                        <el-radio label="20">20条</el-radio>
                                        <el-radio label="30">30条</el-radio>
                                        <el-radio label="40">40条</el-radio>
                                        <el-radio label="50">50条</el-radio>
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
                    <input type="button" class="wxpubtool_CZbth" id="copy" data-clipboard-action="copy" data-clipboard-target="#content" value="一键复制" />
                </div>
            </div>
        </div>
        <div class="setBasicButn" style="border: none;">
            <el-button type="primary" size="medium" :loading='post_loading' @click="save(false)">提交</el-button>
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
                message.success('复制成功！');
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