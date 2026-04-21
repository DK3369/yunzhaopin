<template>
    <div class="homeCenterAll">
        <div class="homeCentLeft">
            <div class="homeCentLeTite">
                <div class="homeCentLeName"><span>月数据统计</span></div>
                <div class="homeCentLeTime">
                    <el-date-picker v-model="date" type="month" value-format="yyyy-MM" placeholder="选择月" @change="dateChange" :picker-options="pickerOptions" :clearable="false">
                    </el-date-picker>
                </div>
            </div>
            <div class="homeCentLeEchat">
                <div class="homeLeEchatInfo">
                    <ul>
                        <li :class="tjTbName=='getweb'?'spanCurs':''" @click="clicktb('getweb')">
                            <span>个人注册</span>
                            <b>{{userNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='resumetj'?'spanCurs':''" @click="clicktb('resumetj')">
                            <span>简历</span>
                            <b>{{resumeNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='comtj'?'spanCurs':''" @click="clicktb('comtj')">
                            <span>企业注册</span>
                            <b>{{companyNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='jobtj'?'spanCurs':''" @click="clicktb('jobtj')">
                            <span>职位</span>
                            <b>{{jobNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='ujobtj'?'spanCurs':''" @click="clicktb('ujobtj')">
                            <span>简历投递</span>
                            <b>{{userjobNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='yqmstj'?'spanCurs':''" @click="clicktb('yqmstj')">
                            <span>邀请面试</span>
                            <b>{{yqmsNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='downresumetj'?'spanCurs':''" @click="clicktb('downresumetj')">
                            <span>简历下载</span>
                            <b>{{downreusmeNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='adtj'?'spanCurs':''" @click="clicktb('adtj')">
                            <span>广告点击</span>
                            <b>{{ggNumMon}}</b>
                        </li>
                        <li :class="tjTbName=='wxbdtj'?'spanCurs':''" @click="clicktb('wxbdtj')">
                            <span>微信绑定</span>
                            <b>{{wxbdNumMon}}</b>
                        </li>
                    </ul>
                </div>
                <div class="homeEchatSubject" v-show="tjTbName=='wxbdtj'">
                    <div class="homeEchatWebChat">
                        <div class="echatSubBine" id="main2"></div>
                    </div>
                    <div class="homeEchatWebText">
                        <div class="homeEchatWebInfo">
                            <h3>企业微信绑定（总数）</h3>
                            <b>{{wxbdcomNumMon}}</b>
                            <span>占比：{{comwx_percent}}%</span>
                        </div>
                        <div class="homeEchatWebInfo">
                            <h3>个人微信绑定（总数）</h3>
                            <b>{{wxbduserNumMon}}</b>
                            <span>占比：{{userwx_percent}}%</span>
                        </div>
                    </div>
                </div>
                <div class="homeEchatMain1" id="main1" v-show="tjTbName!='wxbdtj'"></div>
            </div>
        </div>
    </div>
</template>
<script>
module.exports = {
    data: function() {
        return {
			pickerOptions: {
			    disabledDate(time) {
					var ym = formatMonth(new Date());
			        var ymv = formatMonth(time);
			        return ymv > ym;
			    },
			},
            activeName: '1',
            erjilName: 'downresumedt',
            erjilLog: 'userrz',

            date: '',
            resumeNumMon: 0,
            jobNumMon: 0,
            companyNumMon: 0,
            userNumMon: 0,
            ggNumMon: 0,
            
            userjobNumMon: 0,
            yqmsNumMon: 0,
            downreusmeNumMon: 0,
            wxbdNumMon: 0,

            list: [],
            tjTbName: 'getweb',

            legendData: [],
            xAxisData: [],
            seriesList: [],
            msgNumData: [],

            wxbduserNumMon: 0,
            wxbdcomNumMon: 0,
			comwx_percent:0,
			userwx_percent:0,

        }
    },

    mounted() {
        this.date = formatMonth(new Date());
        this.monthTj();
        this.tjChangeSrc();
    },
    methods: {
        dateChange: function() {
            this.monthTj();
            this.tjChangeSrc();
        },

        monthTj: function() {
            var that = this;
            var param = {};
            if (this.date != '') {
                var datearr = this.date.split('-');
                let ym = new Date(datearr[0], datearr[1], 0).getDate(); // 月未最后一天
                var edate = this.date + '-' + (ym < 10 ? "0" + ym : ym);

                param.sdate = this.date;
                param.edate = edate;
            }
            httpPost('m=index&c=monthStatis', param, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    that.resumeNumMon = res.data.resumeNumMon;
                    that.jobNumMon = res.data.jobNumMon;
                    that.companyNumMon = res.data.companyNumMon;
                    that.userNumMon = res.data.userNumMon;
                    that.ggNumMon = res.data.ggNumMon;
                    
                    that.userjobNumMon = res.data.userjobNumMon;
                    that.yqmsNumMon = res.data.yqmsNumMon;
                    that.downreusmeNumMon = res.data.downreusmeNumMon;
                    that.wxbdNumMon = res.data.wxbdNumMon;
					that.wxbduserNumMon = res.data.wxbduserNumMon;
					that.wxbdcomNumMon = res.data.wxbdcomNumMon;
					that.userwx_percent = res.data.userwx_percent;
					that.comwx_percent = res.data.comwx_percent;
                    
                }
            })
        },
        clicktb: function(name) {
            this.tjTbName = name;
            this.tjChangeSrc();
        },
        tjChangeSrc: function() {
            var that = this;
            var param = {};
            var c = this.tjTbName;
            if (this.date != '') {
                var datearr = this.date.split('-');
                let ym = new Date(datearr[0], datearr[1], 0).getDate(); // 月未最后一天
                var edate = this.date + '-' + (ym < 10 ? "0" + ym : ym);

                param.sdate = this.date;
                param.edate = edate;
            }
            httpPost(`m=index&c=${c}`, param, { hideloading: true }).then(function(response) {
                let res = response.data;
                if (res.error == 0) {
                    
					var tjlist = res.data.list;
					var legendData = res.data.name;

					var xAxisData = [],
						seriesList = [],
						seriesObj = {},
						seriesData = [],
						seriesColor = ['#2778F8', '#F6B44C', '#20EBDA', '#F86138'];

					tjlist.forEach(function(item, key) {
						seriesData = [];
						for (let sdkey in item.list) {
							if (key == 0) {
								xAxisData.push(item.list[sdkey].td);
							}
							seriesData.push(item.list[sdkey].cnt);
						}

						seriesObj = {
							name: legendData[key],
							type: 'line',
							symbol: 'emptyCircle',
							itemStyle: {
								normal: {
									areaStyle: {
										width: 2,
										color: seriesColor[key]
									}
								}
							},
							areaStyle: {
								normal: {
									color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [{
											offset: 0,
											color: seriesColor[key]
										},
										{
											offset: 0.9,
											color: '#fff'
										}
									], false),
								}
							},
							data: seriesData
						};
						seriesList.push(seriesObj);
					})

					that.xAxisData = xAxisData;
					that.seriesList = seriesList;
					that.legendData = legendData;
					
					that.initCharts();
					

                }
            })
        },
        initCharts() {
            var that = this;
			
			var chartID = that.tjTbName == 'wxbdtj'?'main2':'main1';
            var myChart = echarts.init(document.getElementById(`${chartID}`));
            var option = {
                color: ['#2778F8', '#F6B44C', '#20EBDA', '#F86138'],
                legend: {
                    data: that.legendData
                },
                tooltip: {
                    trigger: 'axis',
                    axisPointer: {
                        type: 'shadow'
                    }
                },
                grid: {
                    top: '10%',
                    left: '2%',
                    right: '3%',
                    bottom: '4%',
                    containLabel: true
                },
                xAxis: {
                    type: 'category',
                    boundaryGap: false,
                    data: that.xAxisData,
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: {
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                yAxis: {
                    type: 'value',
                    axisLine: { show: false },
                    axisTick: { show: false },
                    splitLine: { //网格线
                        lineStyle: {
                            type: 'dashed',
                            color: '#f3f3f3'
                        },
                        show: true
                    }
                },
                series: that.seriesList
            };
            myChart.setOption(option, true);
            setTimeout(function() {
                window.addEventListener("resize", () => {
                    myChart.resize();
                });
            }, 200);
        },
        
        toMsgPath: function(item) {
            window.parent.homeapp.checkMenuTwo(item.menudata.nval, item.menudata.oval, item.menudata.tval, item.menudata.name, item.menudata.path, item.menudata.query)
        }
    },
};
</script>
<style scoped></style>