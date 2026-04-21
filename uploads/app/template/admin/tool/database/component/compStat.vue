<template>
    <div class="moduleElHight">
        <div class="dataEchatFrom">
            <div class="dataEchFromTip">
                <span :class="search.days == -1 ? 'spanDay': ''" @click="handleSearch(-1)">昨天</span>
                <span :class="search.days == 1 ? 'spanDay': ''" @click="handleSearch(1)">今天</span>
                <span :class="search.days == 2 ? 'spanDay': ''" @click="handleSearch(2)">一周内</span>
                <span :class="search.days == 3 ? 'spanDay': ''" @click="handleSearch(3)">一月内</span>
                <span :class="search.days == 4 ? 'spanDay': ''" @click="handleSearch(4)">半年</span>
                <span :class="search.days == 5 ? 'spanDay': ''" @click="handleSearch(5)">一年</span>
            </div>
            <div class="dataEchFromInpt">
                <el-date-picker v-model="search.time" type="daterange" range-separator="至" :picker-options="pickerOptions" start-placeholder="开始日期" end-placeholder="结束日期" value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                <el-button type="primary" icon="el-icon-search" size="mini" @click="handleSearch(0)">查询</el-button>
            </div>
        </div>
        <div class="dataEchNumber">
            <div class="dataEchNuWione">
                <div class="dataEchNumxzen">
                    <div>
                        <span>新增个人</span>
                        <b>{{allNum.adduser}}</b>
                    </div>
                </div>
                <div class="dataEchNumSxin">
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim1.png" alt="">
                            <span>新增简历</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.addexpect}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim2.png" alt="">
                            <span>简历刷新</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.resumeRefresh}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim3.png" alt="">
                            <span>简历投递</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.resumeDelivery}}</b>
                        </div>
                    </div>
                </div>
            </div>
            <div class="dataEchNuWitwo">
                <div class="dataEchNumxzen" style="background: #5880FC;">
                    <div>
                        <span>新增企业</span>
                        <b>{{allNum.addcom}}</b>
                    </div>
                </div>
                <div class="dataEchNumSxin">
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim4.png" alt="">
                            <span>新增职位</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.addjob}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim5.png" alt="">
                            <span>职位刷新</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.jobRefresh}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim6.png" alt="">
                            <span>简历下载</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.downResume}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="../../images/dataim7.png" alt="">
                            <span>邀请面试</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.inviteInterview}}</b>
                        </div>
                    </div>
                    
                </div>
            </div>
        </div>

        <div class="dataEchatTite">
            <span>个人用户统计趋势图</span>
        </div>
        <div class="dataEchatInfo">
            <div class="dataEchatModul" id="main1"></div>
            <div class="dataEchatModul" id="main2"></div>
        </div>
        <div class="dataEchatTite">
            <span>企业用户统计趋势图</span>
        </div>
        <div class="dataEchatInfo">
            <div class="dataEchatModul" id="main3"></div>
            <div class="dataEchatModul" id="main4"></div>
        </div>
        <div class="dataEchatInfo" style="padding-top: 15px;">
            <div class="dataEchatModul" id="main5"></div>
            <div class="dataEchatModul" id="main6"></div>
        </div>
        <div class="dataEchatInfo" style="padding-top: 0; height: auto;" >
            <div class="dataEchatModul" id="main7"></div>
        </div>
    </div>
</template>

<script>
    module.exports = {
        data: function () {
            return {
                search: {
                    days: 2,
                    time: ''
                },
                pickMinData: '',
                pickerOptions:{
                    onPick: ({maxDate, minDate}) => {
                        if (minDate && this.pickMinData){

                            this.pickMinData = null
                        } else if (minDate) {

                            this.pickMinData = minDate.getTime()
                        }
                    }
                },
                allNum: [],
                list: []
            }
        },
        created() {
            this.handleSearch(2);
        },
        methods: {
            handleSearch(day) {
                let that = this;
                if (day != 0){

                    that.search.days = day;
                    that.search.time = '';
                }else{

                    that.search.days = '';
                }

                let params = JSON.parse(JSON.stringify(that.search));
                httpPost('m=tool&c=dataBoard', params, {hideloading: true}).then(function (res) {
                    if (res.data.error == 0) {

                        let data = res.data.data;

                        that.allNum = data.allNum;
                        that.list = data.list;

                        that.initCharts1();
                        that.initCharts2();
                        that.initCharts3();
                        that.initCharts4();
                        that.initCharts5();
                        that.initCharts6();
                        
                    }
                });
            },
            initCharts1() {
                let that = this;
                let addUserList =   JSON.parse(JSON.stringify(that.list.adduser.list));
                let addExpeList =   JSON.parse(JSON.stringify(that.list.addexpect.list));
                let xAxisData = [];
                for (let i in addUserList){
                    xAxisData.push(addUserList[i].date);
                }
                let seriesArr = [];
                seriesArr.name = [that.list.adduser.name, that.list.addexpect.name];
                let oneData = [];
                let twoData = [];
                for (let i in addUserList){
                    oneData.push(addUserList[i].count);
                }
                for (let i in addExpeList){
                    twoData.push(addExpeList[i].count);
                }
                seriesArr.data = [oneData, twoData]
                var myChart = echarts.init(document.getElementById('main1'));
                var option = {
                    color: ['#39c3d5', '#46a6ff', '#5259F4'],
                    title: {
                        text: '用户/简历趋势对比图',
                        textStyle: {
                            color: '#333',
                            fontWeight: '500',
                            fontSize: '16'
                        }
                    },
                    legend: {
                        left: 'right'
                    },
                    tooltip: {
                        trigger: 'axis',
                        axisPointer: {
                            type: 'shadow'
                        }
                    },
                    grid: {
                        top: '12%',
                        left: '2%',
                        right: '3%',
                        bottom: '4%',
                        containLabel: true
                    },
                    xAxis: {
                        type: 'category',
                        boundaryGap: false,
                        data: xAxisData,
                        axisLine: {show: false},
                        axisTick: {show: false},
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
                        minInterval: 1,
                        axisLine: {show: false},
                        axisTick: {show: false},
                        splitLine: { //网格线
                            lineStyle: {
                                type: 'dashed',
                                color: '#f3f3f3'
                            },
                            show: true
                        }
                    },
                    series: [
                        {
                            name: seriesArr.name[0],
                            data: seriesArr.data[0],
                            type: 'line',
                            smooth: true,
                            areaStyle: {}
                        },
                        {
                            name: seriesArr.name[1],
                            data: seriesArr.data[1],
                            type: 'line',
                            smooth: true,
                            areaStyle: {}
                        },
                    ]
                };
                myChart.setOption(option);
                setTimeout(function () {
                    window.addEventListener("resize", () => {
                        myChart.resize();
                    });
                }, 200);
            },
            initCharts2() {
                let that = this;
                let deliverList =   JSON.parse(JSON.stringify(that.list.resumeDelivery.list));
                let refreshList =   JSON.parse(JSON.stringify(that.list.resumeRefresh.list));
                let xAxisData = [];
                for (let i in deliverList){
                    xAxisData.push(deliverList[i].date);
                }
                let seriesArr = [];
                seriesArr.name = [that.list.resumeDelivery.name, that.list.resumeRefresh.name];
                let oneData = [];
                let twoData = [];
                for (let i in deliverList){
                    oneData.push(deliverList[i].count);
                }
                for (let i in refreshList){
                    twoData.push(refreshList[i].count);
                }
                seriesArr.data = [oneData, twoData]
                var myChart = echarts.init(document.getElementById('main2'));
                var option = {
                    title: {
                        text: '简历投递/刷新趋势对比图',
                        textStyle: {
                            color: '#333',
                            fontWeight: '500',
                            fontSize: '16'
                        }
                    },
                    legend: {
                        left: 'right',
                    },
                    tooltip: {
                        trigger: 'axis',
                        axisPointer: {
                            type: 'shadow'
                        }
                    },
                    grid: {
                        top: '12%',
                        left: '2%',
                        right: '3%',
                        bottom: '2%',
                        containLabel: true
                    },
                    xAxis: {
                        type: 'category',
                        data: xAxisData,
                        axisLine: {show: false},
                        axisTick: {show: false},
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
                        minInterval: 1,
                        axisLine: {show: false},
                        axisTick: {show: false},
                        splitLine: {
                            lineStyle: {
                                type: 'dashed',
                                color: '#f3f3f3'
                            },
                            show: true
                        }
                    },
                    series: [
                        {
                            name: seriesArr.name[0],
                            data: seriesArr.data[0],
                            type: 'bar',
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                                {
                                    offset: 0,
                                    color: '#399EFC'
                                },
                                {
                                    offset: 1,
                                    color: '#3ED572'
                                }
                            ])
                        },
                        {
                            name: seriesArr.name[1],
                            data: seriesArr.data[1],
                            type: 'bar',
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                                {
                                    offset: 0,
                                    color: '#FEC59A'
                                },
                                {
                                    offset: 1,
                                    color: '#FEA15C'
                                }
                            ])
                        },
                    ]
                };
                myChart.setOption(option);
                setTimeout(function () {
                    window.addEventListener("resize", () => {
                        myChart.resize();
                    });
                }, 200);
            },
            initCharts3() {
                let that = this;
                let comList =   JSON.parse(JSON.stringify(that.list.addcom.list));
                let jobList =   JSON.parse(JSON.stringify(that.list.addjob.list));
                let xAxisData = [];
                for (let i in comList){
                    xAxisData.push(comList[i].date);
                }
                let seriesArr = [];
                seriesArr.name = [that.list.addcom.name, that.list.addjob.name];
                let oneData = [];
                let twoData = [];
                for (let i in comList){
                    oneData.push(comList[i].count);
                }
                for (let i in jobList){
                    twoData.push(jobList[i].count);
                }
                seriesArr.data = [oneData, twoData]
                var myChart = echarts.init(document.getElementById('main3'));
                var option = {
                    color: ['#23C9C9', '#1890FF'],
                    title: {
                        text: '企业/职位趋势对比图',
                        textStyle: {
                            color: '#333',
                            fontWeight: '500',
                            fontSize: '16'
                        }
                    },
                    legend: {
                        left: 'right',
                        icon: 'circle'
                    },
                    tooltip: {
                        trigger: 'axis',
                        axisPointer: {
                            type: 'shadow'
                        }
                    },
                    grid: {
                        top: '12%',
                        left: '2%',
                        right: '3%',
                        bottom: '2%',
                        containLabel: true
                    },
                    xAxis: {
                        type: 'category',
                        data: xAxisData,
                        axisLine: {show: false},
                        axisTick: {show: false},
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
                        minInterval: 1,
                        axisLine: {show: false},
                        axisTick: {show: false},
                        splitLine: {
                            lineStyle: {
                                type: 'dashed',
                                color: '#f3f3f3'
                            },
                            show: true
                        }
                    },
                    series: [
                        {
                            name: seriesArr.name[0],
                            data: seriesArr.data[0],
                            type: 'bar',
                            stack: 'total',
                            barWidth: '25'
                        },
                        {
                            name: seriesArr.name[1],
                            data: seriesArr.data[1],
                            type: 'bar',
                            stack: 'total',
                            barWidth: '25'
                        },
                    ]
                };
                myChart.setOption(option);
                setTimeout(function () {
                    window.addEventListener("resize", () => {
                        myChart.resize();
                    });
                }, 200);
            },
            initCharts4() {
                let that = this;
                let downList =   JSON.parse(JSON.stringify(that.list.downResume.list));
                let xAxisData = [];
                let seriesArr = [];
                let oneData = [];
                for (let i in downList){
                    xAxisData.push(downList[i].date);
                    oneData.push(downList[i].count);
                }
                seriesArr.name = that.list.downResume.name;
                seriesArr.data = oneData;
                var myChart = echarts.init(document.getElementById('main4'));
                var option = {
                    color: ['#1890FF', '#23C9C9', '#5259F4'],
                    title: {
                        text: '简历下载统计',
                        textStyle: {
                            color: '#333',
                            fontWeight: '500',
                            fontSize: '16'
                        }
                    },
                    legend: {
                        left: 'right'
                    },
                    tooltip: {
                        trigger: 'axis',
                        axisPointer: {
                            type: 'shadow'
                        }
                    },
                    grid: {
                        top: '12%',
                        left: '2%',
                        right: '3%',
                        bottom: '4%',
                        containLabel: true
                    },
                    xAxis: {
                        type: 'category',
                        boundaryGap: false,
                        data: xAxisData,
                        axisLine: {show: true},
                        axisTick: {show: true},
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
                        minInterval: 1,
                        axisLine: {show: true},
                        axisTick: {show: true},
                        splitLine: { //网格线
                            lineStyle: {
                                type: 'dashed',
                                color: '#f3f3f3'
                            },
                            show: true
                        }
                    },
                    series: [
                        {
                            name: seriesArr.name,
                            data: seriesArr.data,
                            type: 'line',
                            // smooth: true,
                            areaStyle: {
                                normal: {
                                    //右，下，左，上
                                    color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                                        {
                                            offset: 0,
                                            color: '#1890FF'
                                        },
                                        {
                                            offset: 0.7,
                                            color: '#fff'
                                        }
                                    ], false),
                                }
                            },
                        }
                    ]
                };
                myChart.setOption(option);
                setTimeout(function () {
                    window.addEventListener("resize", () => {
                        myChart.resize();
                    });
                }, 200);
            },
            initCharts5() {
                let that = this;
                let refreshList =   JSON.parse(JSON.stringify(that.list.jobRefresh.list));
                let xAxisData = [];
                let seriesArr = [];
                let oneData = [];
                for (let i in refreshList){
                    xAxisData.push(refreshList[i].date);
                    oneData.push(refreshList[i].count);
                }
                seriesArr.name = that.list.jobRefresh.name;
                seriesArr.data = oneData
                var myChart = echarts.init(document.getElementById('main5'));
                var option = {
                    title: {
                        text: '职位刷新统计',
                        textStyle: {
                            color: '#333',
                            fontWeight: '500',
                            fontSize: '16'
                        }
                    },
                    legend: {
                        left: 'right',
                    },
                    tooltip: {
                        trigger: 'axis',
                        axisPointer: {
                            type: 'shadow'
                        }
                    },
                    grid: {
                        top: '12%',
                        left: '2%',
                        right: '3%',
                        bottom: '2%',
                        containLabel: true
                    },
                    xAxis: {
                        type: 'category',
                        data: xAxisData,
                        axisLine: {show: false},
                        axisTick: {show: false},
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
                        minInterval: 1,
                        axisLine: {show: false},
                        axisTick: {show: false},
                        splitLine: {
                            lineStyle: {
                                type: 'dashed',
                                color: '#f3f3f3'
                            },
                            show: true
                        }
                    },
                    series: [
                        {
                            name: seriesArr.name,
                            data: seriesArr.data,
                            type: 'bar',
                            barWidth: '25',
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [
                                {
                                    offset: 0,
                                    color: '#D2E9FF'
                                },
                                {
                                    offset: 1,
                                    color: '#8896FB'
                                }
                            ])
                        },
                    ]
                };
                myChart.setOption(option);
                setTimeout(function () {
                    window.addEventListener("resize", () => {
                        myChart.resize();
                    });
                }, 200);
            },
            initCharts6() {
                let that = this;
                let inviteList =   JSON.parse(JSON.stringify(that.list.inviteInterview.list));
                let xAxisData = [];
                let seriesArr = [];
                let oneData = [];
                for (let i in inviteList){
                    xAxisData.push(inviteList[i].date);
                    oneData.push(inviteList[i].count);
                }
                seriesArr.name = that.list.inviteInterview.name;
                seriesArr.data = oneData;
                var myChart = echarts.init(document.getElementById('main6'));
                var option = {
                    color: ['#39c3d5', '#46a6ff', '#5259F4'],
                    title: {
                        text: '邀请面试统计',
                        textStyle: {
                            color: '#333',
                            fontWeight: '500',
                            fontSize: '16'
                        }
                    },
                    legend: {
                        left: 'right'
                    },
                    tooltip: {
                        trigger: 'axis',
                        axisPointer: {
                            type: 'shadow'
                        }
                    },
                    grid: {
                        top: '12%',
                        left: '2%',
                        right: '3%',
                        bottom: '4%',
                        containLabel: true
                    },
                    xAxis: {
                        type: 'category',
                        boundaryGap: false,
                        data: xAxisData,
                        axisLine: {show: false},
                        axisTick: {show: false},
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
                        minInterval: 1,
                        axisLine: {show: false},
                        axisTick: {show: false},
                        splitLine: { //网格线
                            lineStyle: {
                                type: 'dashed',
                                color: '#f3f3f3'
                            },
                            show: true
                        }
                    },
                    series: [
                        {
                            name: seriesArr.name,
                            data: seriesArr.data,
                            type: 'line',
                            smooth: true,
                            areaStyle: {}
                        },

                    ]
                };
                myChart.setOption(option);
                setTimeout(function () {
                    window.addEventListener("resize", () => {
                        myChart.resize();
                    });
                }, 200);
            },
            
        },
    };
</script>
<style scoped>
    .moduleElHight {
        overflow-y: auto;
    }

    .el-date-editor .el-range-separator {
        line-height: 27px;
    }

    .el-date-editor .el-range__icon {
        line-height: 27px;
    }
    .dataEchFromTip span{
        cursor: pointer;
    }
</style>