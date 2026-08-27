<template>
    <div class="moduleElHight">
        <div class="dataEchatFrom">
            <div class="dataEchFromTip">
                <span :class="search.days == -1 ? 'spanDay': ''" @click="handleSearch(-1)">{{ lc('common_02000') }}</span>
                <span :class="search.days == 1 ? 'spanDay': ''" @click="handleSearch(1)">{{ lc('common_01940') }}</span>
                <span :class="search.days == 2 ? 'spanDay': ''" @click="handleSearch(2)">{{ lc('admin_tool_00142') }}</span>
                <span :class="search.days == 3 ? 'spanDay': ''" @click="handleSearch(3)">{{ lc('admin_tool_00143') }}</span>
                <span :class="search.days == 4 ? 'spanDay': ''" @click="handleSearch(4)">{{ lc('admin_tool_00146') }}</span>
                <span :class="search.days == 5 ? 'spanDay': ''" @click="handleSearch(5)">{{ lc('admin_tool_00145') }}</span>
            </div>
            <div class="dataEchFromInpt">
                <el-date-picker v-model="search.time" type="daterange" :range-separator="lc('admin_company_00019')" :picker-options="pickerOptions" :start-placeholder="lc('admin_00343')" :end-placeholder="lc('admin_00344')" value-format="timestamp" @change="handleSearch(0)"></el-date-picker>
                <el-button type="primary" icon="el-icon-search" size="small" @click="handleSearch(0)">{{ lc('admin_user_weipin_00049') }}</el-button>
            </div>
        </div>
        <div class="dataEchNumber">
            <div class="dataEchNuWione">
                <div class="dataEchNumxzen">
                    <div>
                        <span>{{ lc('admin_user_00305') }}</span>
                        <b>{{allNum.adduser}}</b>
                    </div>
                </div>
                <div class="dataEchNumSxin">
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim1.png" alt="">
                            <span>{{ lc('admin_user_00193') }}</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.addexpect}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim2.png" alt="">
                            <span>{{ lc('admin_tool_00176') }}</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.resumeRefresh}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim3.png" alt="">
                            <span>{{ lc('member_com_00152') }}</span>
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
                        <span>{{ lc('admin_user_company_00162') }}</span>
                        <b>{{allNum.addcom}}</b>
                    </div>
                </div>
                <div class="dataEchNumSxin">
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim4.png" alt="">
                            <span>{{ lc('member_com_00250') }}</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.addjob}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim5.png" alt="">
                            <span>{{ lc('wap_com_00045') }}</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.jobRefresh}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim6.png" alt="">
                            <span>{{ lc('wap_com_00042') }}</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.downResume}}</b>
                        </div>
                    </div>
                    <div class="dataEchNumList">
                        <div class="dataEchNumName">
                            <img src="/admin/php-admin/images/dataim7.png" alt="">
                            <span>{{ lc('resume_00029') }}</span>
                        </div>
                        <div class="dataEchNumSusn">
                            <b>{{allNum.inviteInterview}}</b>
                        </div>
                    </div>
                    
                </div>
            </div>
        </div>

        <div class="dataEchatTite">
            <span>{{ lc('admin_tool_00177') }}</span>
        </div>
        <div class="dataEchatInfo">
            <div class="dataEchatModul" id="main1"></div>
            <div class="dataEchatModul" id="main2"></div>
        </div>
        <div class="dataEchatTite">
            <span>{{ lc('admin_tool_00178') }}</span>
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
const httpPost = (...a) => window.httpPost(...a)
const lc = (...a) => window.lc(...a)
const message = typeof window !== 'undefined' && window.message ? window.message : { success(){}, error(){}, warning(){}, confirm(){}, alert(){}, open(){} }
const delConfirm = (...a) => window.delConfirm(...a)
const formatDate = (...a) => window.formatDate(...a)
const formatMonth = (...a) => window.formatMonth(...a)
const formatDatetime = (...a) => window.formatDatetime(...a)
const deepClone = (...a) => window.deepClone(...a)
const scrollToTop = (...a) => window.scrollToTop(...a)
const isEmpty = (...a) => window.isEmpty(...a)
const isArray = (...a) => window.isArray(...a)
const $ = typeof window !== 'undefined' && window.$ ? window.$ : Object.assign(function(){ return { length: 0 } }, {})
const echarts = typeof window !== 'undefined' && window.echarts ? window.echarts : { init(){ return { setOption(){}, resize(){} } }, graphic: { LinearGradient: function(){} } }

    export default {
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
                        text: window.yunAdminT(lc('admin_tool_00179')),
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
                        text: window.yunAdminT(lc('admin_tool_00180')),
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
                        text: window.yunAdminT(lc('admin_tool_00181')),
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
                        text: window.yunAdminT(lc('admin_tool_00108')),
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
                                    //右，下，{{ lc('wap_01540') }}，上
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
                        text: window.yunAdminT(lc('admin_tool_00182')),
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
                        text: window.yunAdminT(lc('admin_tool_00114')),
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