<template>
    <div class="homeModuleTop">
        <div id="yunshengji"></div>
        <div class="homeTopSubcty">
            <div class="homeTopSubqunim">
                <div class="homeTopSubCoy">
                    <div class="homeTopLeList">
                        <div class="homeTopLeLTite">
                            <div class="homeTopLeLName"><span>{{ lc('admin_00083') }}</span></div>
                            <div class="homeTopLeLButn">
                                <span :class="ordertype=='day'?'spanxz':''" @click="ordertype='day'">{{ lc('admin_00091') }}</span>
                                <span :class="ordertype=='month'?'spanxz':''" @click="ordertype='month';dataChange('month', 'money')">{{ lc('admin_user_00147') }}</span>
                            </div>
                        </div>
                        <div class="homeTopLeMoney" @click="toOrder({order_state:'2'})">
                            <div class="homeTopMonNum"><span>￥</span>{{ordertype=='day'?numdata.moneyTotal:numdata.monthMoneyTotal}}</div>
                            <div class="homeTopMonTxt">
                                <span>{{ lc('admin_01054') }}</span>
                            </div>
                        </div>
                        <div class="homeTopLeTaoc">
                            <div @click="toOrder({order_state:'2',typedd:'1'})">
                                <span>{{ordertype=='day'?numdata.moneyVip:numdata.monthMoneyVip}}</span>
                                <b>{{ lc('admin_user_company_00034') }}</b>
                            </div>
                            <div @click="toOrder({order_state:'2',typedd:'5'})">
                                <span>{{ordertype=='day'?numdata.moneyService:numdata.monthMoneyService}}</span>
                                <b>{{ lc('admin_user_company_00290') }}</b>
                            </div>
                            <div @click="toOrder({order_state:'2'})">
                                <span>{{ordertype=='day'?numdata.moneyIntegral:numdata.monthMoneyIntegral}}</span>
                                <b>{{ lc('wap_user_00196') }}</b>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="homeTopSubCoy">
                    <div class="homeTopLeList homeTopLeLisTwo">
                        <div class="homeTopLeLTite">
                            <div class="homeTopLeLName"><span>{{ lc('admin_00081') }}</span></div>
                            <div class="homeTopLeLButn">
                                <span :class="memtype=='day'?'spanxz':''" @click="memtype='day'">{{ lc('admin_00091') }}</span>
                                <span :class="memtype=='month'?'spanxz':''" @click="memtype='month';dataChange('month', 'member')">{{ lc('admin_user_00147') }}</span>
                            </div>
                        </div>
                        <div class="homeTopLeMoney" @click="toMem()">
                            <div class="homeTopMonNum">{{memtype=='day'?numdata.memberNum:numdata.monthMemberNum}}</div>
                            <div class="homeTopMonTxt">
                                <span>{{ lc('admin_01055') }}</span>
                            </div>
                        </div>
                        <div class="homeTopLeTaoc">
                            <div @click="touserMem()">
                                <span>{{memtype=='day'?numdata.userNum:numdata.userNumMon}}</span>
                                <b>{{ lc('admin_system_00129') }}</b>
                            </div>
                            <div @click="tocomMem()">
                                <span>{{memtype=='day'?numdata.companyNum:numdata.companyNumMon}}</span>
                                <b>{{ lc('admin_user_company_00281') }}</b>
                            </div>
                            <div @click="toMem({utype:5})">
                                <span>{{memtype=='day'?numdata.otmemberNum:numdata.ommemberNum}}</span>
                                <b>{{ lc('admin_00087') }}</b>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="homeTopSubCoy homeTopSubFlwex">
                    <div class="homeTopSubBuue" @click="toPage(1,7,33,lc('wap_com_00105'),'/resume',{params:{time_type:'adtime', times: todayrange}})">
                        <div class="homeTopLeNum">
                            <div class="homeTopLeimg">
                                <img src="../../images/adxinw.png" alt="">
                            </div>
                            <div class="homeTopLshu">
                                <span>{{ lc('admin_00078') }}</span>
                                <b>{{numdata.resumeNum}}</b>
                            </div>
                        </div>
                        <div class="homeTopLeDay">
                            <div>
                                <img src="../../images/adminimg1.png" alt="">
                                <span>{{ lc("admin_yesterday_count", [numdata.oldresumeNum]) }}</span>
                            </div>
                        </div>
                    </div>
					<div class="homeTopSubBuue" @click="toCompanyJob({adtime:'1', times: todayrange})">
					    <div class="homeTopLeNum">
					        <div class="homeTopLeimg">
					            <img src="../../images/adzhiwei.png" alt="">
					        </div>
					        <div class="homeTopLshu">
					            <span>{{ lc('admin_00079') }}</span>
					            <b>{{numdata.jobNum}}</b>
					        </div>
					    </div>
					    <div class="homeTopLeDay">
					        <div>
					            <img src="../../images/adminimg3.png" alt="">
					            <span>{{ lc("admin_yesterday_count", [numdata.oldjobNum]) }}</span>
					        </div>
					    </div>
					</div>
                    
                </div>
                <div class="homeTopSubCoy homeTopSubFlwex">
                    <div class="homeTopSubBuue" @click="toPage(1,6,50,lc('admin_00090'),'/companyjoblog',{params:{time:'1'}})">
                        <div class="homeTopLeNum">
                            <div class="homeTopLeimg">
                                <img src="../../images/adjl.png" alt="">
                            </div>
                            <div class="homeTopLshu">
                                <span>{{ lc('admin_00080') }}</span>
                                <b>{{numdata.useridjobNum}}</b>
                            </div>
                        </div>
                        <div class="homeTopLeDay">
                            <div>
                                <img src="../../images/adminimg2.png" alt="">
                                <span>{{ lc("admin_yesterday_count", [numdata.olduseridjobNum]) }}</span>
                            </div>
                        </div>
                    </div>
                    <div class="homeTopSubBuue" @click="toPage(1,6,50,lc('admin_00090'),'/companyjoblog',{tab:'jobtellog',params:{end:'1'}})">
                        <div class="homeTopLeNum">
                            <div class="homeTopLeimg">
                                <img src="../../images/chatyh.png" alt="">
                            </div>
                            <div class="homeTopLshu">
                                <span>{{ lc('admin_00084') }}</span>
                                <b>{{numdata.tellognum}}</b>
                            </div>
                        </div>
                        <div class="homeTopLeDay">
                            <div>
                                <img src="../../images/adminimg4.png" alt="">
                                <span>{{ lc("admin_yesterday_count", [numdata.oldtellognum]) }}</span>
                            </div>
                        </div>
                    </div>
                    
                </div>

                <div class="homeTopSubCoy homeTopSubFlwex">
                    <div class="homeTopSubBuue">
                        <div class="homeTopLeNum" @click="toPage(1,7,38,lc('admin_00089'),'/xingwei',{params:{time:'1'}})">
                            <div class="homeTopLeimg">
                                <img src="../../images/xaizyh.png" alt="">
                            </div>
                            <div class="homeTopLshu">
                                <span>{{ lc('admin_00076') }}</span>
                                <b>{{numdata.downresumeNum}}</b>
                            </div>
                        </div>
                        <div class="homeTopLeDay" @click="toPage(1,7,38,lc('admin_00089'),'/xingwei',{params:{time:'1',activeName:'sixth'}})">
                            <div>
                                <span>{{ lc("admin_free_downloads_today", [numdata.freedownnum]) }}</span>
                            </div>
                        </div>
                    </div>
                    
                </div>
            </div>
            
        </div>
        
    </div>
</template>
<script>
module.exports = {
    props: {
        topinfo: {
            type: Object,
            default: {}
        },
    },
    data: function() {
        return {
            ordertype: 'day',
            memtype: 'day',
            timer: null,
            todayrange: [],
			monthrange:[],
            numdata: {
				freedownnum:0,
				
				tellognum:0,
				oldtellognum:0,
                memberNum: 0,
                otmemberNum: 0,
                userNum: 0,
                companyNum: 0,
                resumeNum: 0,
                useridjobNum: 0,
                olduseridjobNum: 0,
                oldresumeNum: 0,
                jobNum: 0,
                oldjobNum: 0,
                moneyTotal: '0',
                moneyVip: '0',
                moneyIntegral: '0',
                moneyService: '0',
                
                downresumeNum: 0,
                olddownresumeNum: 0,
                msgnum: '-',
            },
            dataType: '',
            dataArea: '',
            moneyLoad: false, // 有无加载过本月数据
            memberLoad: false, // 有无加载过本月数据
        }
    },

    mounted() {
        var today = formatDate(new Date());
		var month01 = formatMonth(new Date())+'-01';
        this.todayrange = [today, today];
		this.monthrange = [month01, today];
		
        this.getData();
    },
    methods: {
        

        getData: function() {
            var that = this,
                params = {};

            if (that.dataType) {
                params.type = that.dataType;
            }
            if (that.dataArea) {
                params.area = that.dataArea;
            }

            httpPost('m=index&c=ajax_statis', params, {hideloading: true}).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    if (that.dataType == 'month') {
                        if (that.dataArea == 'money') {
                            that.$set(that.numdata, 'monthMoneyTotal', res.data.monthMoneyTotal ? res.data.monthMoneyTotal : 0);
                            that.$set(that.numdata, 'monthMoneyVip', res.data.monthMoneyVip ? res.data.monthMoneyVip : 0);
                            that.$set(that.numdata, 'monthMoneyIntegral', res.data.monthMoneyIntegral ? res.data.monthMoneyIntegral : 0);
                            that.$set(that.numdata, 'monthMoneyService', res.data.monthMoneyService ? res.data.monthMoneyService : 0);
                            that.moneyLoad = true;
                        }
                        
                        if (that.dataArea == 'member') {
                            that.$set(that.numdata, 'monthMemberNum', res.data.monthMemberNum ? res.data.monthMemberNum : 0);
                            that.$set(that.numdata, 'ommemberNum', res.data.ommemberNum ? res.data.ommemberNum : 0);
                            that.$set(that.numdata, 'userNumMon', res.data.userNumMon ? res.data.userNumMon : 0);
                            that.$set(that.numdata, 'companyNumMon', res.data.companyNumMon ? res.data.companyNumMon : 0);
                            that.memberLoad = true;
                        }
                    } else {
                        that.numdata = res.data;
                    }
                }
            })
        },

        dataChange (type, area) {
            this.dataType = type;
            this.dataArea = area;
            if ((area == 'money' && this.moneyLoad) || (area == 'member' && this.memberLoad)) {
                return false;
            }
            this.getData();
        },
        
        toOrder: function(params = {}) {
            params.times = this.todayrange;
            if (this.ordertype == 'month') {
                params.times = this.monthrange;
            }
			
			window.parent.homeapp.checkMenuTwo(3,161,162,'admin_user_company_00136','/chongzhidd',params)
			
        },
        toMem: function(params = {}) {
			params.time_type = 'adtime';
            params.times = this.todayrange;
            if (this.memtype == 'month') {
                params.times = this.monthrange;
            }
			var query ={};
			query.tabs = 'first';
			query.params = params;
			
			window.parent.homeapp.checkMenuTwo(1,401,402,'admin_00086','/usercrm',query)
            
        },
        touserMem: function(params = {}) {
            params.time_type = 'adtime';
            params.times = this.todayrange;
            if (this.memtype == 'month') {
                params.times = this.monthrange;
            }
			
			window.parent.homeapp.checkMenuTwo(1,7,17,'admin_user_00122','/userscrm',params)
            
        },
        tocomMem: function(params = {}) {
            params.time_type = 'adtime';
            params.times = this.todayrange;
            if (this.memtype == 'month') {
                params.times = this.monthrange;
            }
			var query ={};
			query.params = params;
			window.parent.homeapp.checkMenuTwo(1,6,16,'admin_user_00124','/companycrm',query)
            
        },
        toPage: function(nval, oval, tval,name, path, query = null) {
			
			window.parent.homeapp.checkMenuTwo(nval,oval,tval,name,path,query)
			
        },
        toCompanyJob(params = {}) {
            window.parent.homeapp.checkMenuTwo(1, 6, 40, 'wap_com_00106', '/companyjob', params);
        },
        tomsg: function() {
            window.open('https://u.ov6.com/');
        },
        
    },
};
</script>
<style scoped></style>
