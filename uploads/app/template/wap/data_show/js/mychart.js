window.onload = function () {
    echart1();
    setTimeout(function(){
        echart2();
        echart3();
        echart4();
        echart5();
        echart6();
        echart7();
        echart8();
        echart9();
        echart10();
        echart11();
    }, 500)
}
var legendintrval1 = legendintrval3 = legendintrval7 = '';
// 数据图表
function echart1() {
    $.get('index.php?c=ajax&a=cityData', function (res) {
        var myChart = echarts.init(document.getElementById('main1'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            $('#city1').text(data[0].city_name)
            $('#city1rate').text(data[0].rate)
            $('#city2').text(data[1].city_name)
            $('#city2rate').text(data[1].rate)
            $('#city3').text(data[2].city_name)
            $('#city3rate').text(data[2].rate)
            // 初始化插件
            $('.city_counter').countUp({
                delay: 10,
                time: 2000
            });
            data.forEach(function(item,index){
                // if (index < 5) {
                    names.push({ name: item.city_name });
                    if (index>0&(index+1)%3 == 0) {
                        names.push('\n');
                        names.push('\n');
                    }
                // }
                valueName.push({ value: item.rate, name: item.city_name });
            });
        }
        var option = {
            color: ['#F4003E', '#029DF7', '#E7AD1B', '#58DBD3', '#424BBD'],
            tooltip: {
                trigger: 'item',
                formatter: "{b}: {d}%"
            },
            legend: {
                id: 'legend1',
                orient: 'vertical',
                left: 'right',
                // top: '30%',
                type: "scroll",
                pageIconSize: '',
                textStyle: {
                    color: '#fff',
                },
                pageTextStyle: {
                    color: "#101133",
                },
            },
            // legend: {
            //     id:'legend1',
            //     orient: 'vertical',
            //     left: 'right',
            //     textStyle: {
            //         color: '#fff'
            //     },
            //     type:'scroll',
            //     // data: names,
            // },
            series: [
                {
                    name: '占比',
                    type: 'pie',
                    radius: '54%',
                    center: ['30%', '40%'],
                    avoidLabelOverlap: false,
                    label: {
                        show: true,
                        position: 'inside',
                        formatter: '{d}%',
                        color: "#fff",
                        fontSize: 14,
                    },
                    itemStyle: {
                        // borderRadius: 25,
                        borderColor: '#101133',
                        borderWidth: 2
                    },
                    labelLine: {
                        show: false,
                    },
                    data: valueName
                }
            ]
        };
        myChart.setOption(option);
        var cont = valueName.length,
            i = 0;
        clearInterval(legendintrval1);
        legendintrval1 = setInterval(() => {
            myChart.dispatchAction({
                type: 'legendScroll',
                scrollDataIndex: i,
                legendId: 'legend1'
            });
            i = i < (cont - 3) ? i + 1 : 0;
        }, 1200);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json');
};
function echart2() {
    $.get('index.php?c=ajax&a=ageData', function (res) {
        var myChart = echarts.init(document.getElementById('main2'));
        var data = res.data;
        if (data.length > 0) {
            $('#age1').text(data[0].name)
            $('#age1rate').text(data[0].rate)
            $('#age2').text(data[1].name)
            $('#age2rate').text(data[1].rate)
            $('#age3').text(data[2].name)
            $('#age3rate').text(data[2].rate)
            $('#age4').text(data[3].name)
            $('#age4rate').text(data[3].rate)
            // 初始化插件
            $('.age_counter').countUp({
                delay: 10,
                time: 2000
            });
        }
        var option = {
            tooltip: {
                trigger: 'axis',
                formatter: "{b}: {c}%",
                axisPointer: {
                    type: 'shadow'
                }
            },
            grid: {
                top: '5%',
                left: '2%',
                right: '4%',
                bottom: '4%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                data: [data[0].name, data[1].name, data[2].name, data[3].name],
                axisLine: {
                    show: false
                },
                axisTick: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    }
                }
            },
            yAxis: {
                type: 'value',
                splitLine: {
                    lineStyle: {
                        // type: 'dashed',
                        color: '#ABD7FF'
                    },
                    show: true
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    }
                }
            },
            series: [
                {
                    name: '百分比',
                    data: [
                        {
                            value: data[0].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#029DF7"
                                    }], false)
                                }
                            }
                        },
                        {
                            value: data[1].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#F4003E"
                                    }], false)
                                }
                            }
                        },
                        {
                            value: data[2].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#58DBD3"
                                    }], false)
                                }
                            }
                        },
                        {
                            value: data[3].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#E7AD1B"
                                    }], false)
                                }
                            }
                        }
                    ],
                    type: 'bar',
                    barWidth: 20,
                }
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json');
};
function echart3() {
    $.get('index.php?c=ajax&a=expData', function (res) {
        var myChart = echarts.init(document.getElementById('main3'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            data.forEach(function(item,index){
                // names.push({ name: item.exp_n});
                // if (index == 1 || index == 3) {
                //     names.push('\n');
                //     names.push('\n');
                // }
                valueName.push({ value: item.rate, name: item.exp_n });
            });
        }
        var option = {
            color: ['#F4003E', '#029DF7', '#E7AD1B', '#58DBD3', '#424BBD'],
            tooltip: {
                trigger: 'item',
                formatter: "{b}: {d}%"
            },
            grid: {
                top: '15%',
            },
            legend: {
                id: 'legend3',
                orient: 'vertical',
                left: '1px',
                // top: '30%',
                type: "scroll",
                pageIconSize: '',
                textStyle: {
                    color: '#fff',
                },
                pageTextStyle: {
                    color: "#101133",
                },
            },
            // legend: {
            //     left: 'left',
            //     textStyle: {
            //         color: '#fff'
            //     },
            //     data: names,
            // },
            series: [
                {
                    name: '经验分布',
                    type: 'pie',
                    radius: ['40%', '80%'],
                    center: ['62%', '56%'],
                    roseType: 'area',
                    avoidLabelOverlap: false,
                    label: {
                        show: true,
                        position: 'inside',
                        formatter: '{d}%',
                        color: "#fff",
                        fontSize: 14,
                    },
                    itemStyle: {
                        // borderRadius: 25,
                        borderColor: '#101133',
                        borderWidth: 2
                    },
                    labelLine: {
                        show: false,
                    },
                    data: valueName
                }
            ]
        };
        myChart.setOption(option);
        var cont = valueName.length,
            i = 0;
        clearInterval(legendintrval3);
        legendintrval3 = setInterval(() => {
            myChart.dispatchAction({
                type: 'legendScroll',
                scrollDataIndex: i,
                legendId: 'legend3'
            });
            i = i < (cont - 3) ? i + 1 : 0;
        }, 1200);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart4() {
    $.get('index.php?c=ajax&a=eduData', function (res) {
        var myChart = echarts.init(document.getElementById('main4'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            $('#edu1').text(data[0].edu_n)
            $('#edu1rate').text(data[0].rate)
            $('#edu2').text(data[1].edu_n)
            $('#edu2rate').text(data[1].rate)
            $('#edu3').text(data[2].edu_n)
            $('#edu3rate').text(data[2].rate)
            // 初始化插件
            $('.edu_counter').countUp({
                delay: 10,
                time: 2000
            });
            data.forEach(function(item,index){
                names.push(item.edu_n);
                valueName.push(item.rate);
            });
        }
        var option = {
            tooltip: {
                trigger: 'axis',
                axisPointer: {
                    type: 'shadow'
                },
                formatter: '{b}\n{c}%'
            },
            grid: {
                top: '5%',
                left: '3%',
                right: '4%',
                bottom: '3%',
                containLabel: true
            },
            xAxis: {
                type: 'value',
                inverse: false,
                axisLine: {
                    show: false
                },
                axisTick: {
                    show: false
                },
                splitLine: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    },
                    show: false
                }
            },
            yAxis: {
                type: 'category',
                data: names,
                axisTick: {
                    show: false
                },
                inverse: true,
                axisLine: {
                    show: false
                },
                splitLine: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    },
                    show: true
                },
            },
            series: [
                {
                    name: '学历分布',
                    type: 'bar',
                    stack: 'total',
                    label: {
                        // show: true,
                        formatter: '{c}%'
                    },
                    emphasis: {
                        focus: 'series'
                    },
                    data: valueName,
                    barWidth: 20,
                    itemStyle: {
                        normal: {
                            //这里是颜色
                            color: function (params) {
                                var colorList = ['#424BBD', '#58DBD3', '#029DF7', '#F4003E', '#FF9337', '#FFC722', '#ca8622'];
                                return colorList[params.dataIndex]
                            }
                        },
                    }
                }
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart5() {
    $.get('index.php?c=ajax&a=userHyChart', function (res) {
        var myChart = echarts.init(document.getElementById('main5'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            data.forEach(function(item){
                names.push(item.month);
                valueName.push(item.num);
            });
        }
        var option = {
            color: ['#4367F9', '#46a6ff', '#5259F4'],
            // legend: {},
            // tooltip: {
            //     trigger: 'axis',
            //     axisPointer: {
            //         type: 'shadow'
            //     }
            // },
            grid: {
                top: '5%',
                left: '2%',
                right: '3%',
                bottom: '4%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                boundaryGap: false,
                data: names,
                axisLine: { show: false },
                axisTick: { show: false },
                splitLine: {
                    lineStyle: {
                        type: 'dashed',
                        color: '#f3f3f3'
                    },
                    show: false
                },
            },
            yAxis: {
                type: 'value',
                axisTick: {
                    show: false
                },
                axisLine: {
                    show: false
                },
                axisLabel: {
                    show: false, // 不显示坐标轴上的文字
                },
                splitLine: {
                    lineStyle: {
                        color: '#68a5df'
                    },
                    show: true
                },
            },
            series: [
                {
                    name: '活跃数',
                    data: valueName,
                    type: 'line',
                    // smooth: true,
                    areaStyle: {},
                    itemStyle: {
                        normal: {
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                offset: 0,
                                color: "#101133"
                            }, {
                                offset: 1,
                                color: "#4367F9"
                            }], false)
                        }
                    }
                },
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart6() {
    $.get('index.php?c=ajax&a=userRegChart', function (res) {
        var myChart = echarts.init(document.getElementById('main6'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            data.forEach(function(item){
                names.push(item.month);
                valueName.push(item.num);
            });
        }
        var option = {
            // tooltip: {
            //     trigger: 'axis',
            //     axisPointer: {
            //         type: 'shadow'
            //     }
            // },
            grid: {
                top: '5%',
                left: '2%',
                right: '4%',
                bottom: '4%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                data: names,
                axisLine: {
                    show: false
                },
                axisTick: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    }
                }
            },
            yAxis: {
                type: 'value',
                splitLine: {
                    lineStyle: {
                        // type: 'dashed',
                        color: '#ABD7FF'
                    },
                    show: true
                },
                axisLabel: {
                    show: false, // 不显示坐标轴上的文字
                }
            },
            series: [
                {
                    name: '注册数',
                    data: valueName,
                    type: 'bar',
                    // barWidth: 30,
                    barCategoryGap: 8,
                    itemStyle: {
                        normal: {
                            barBorderRadius: [15, 15, 0, 0],
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                offset: 0,
                                color: "#101133"
                            }, {
                                offset: 1,
                                color: "#F4003E"
                            }], false)
                        },
                    }
                }
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    },'json')
};
function echart7() {
    $.get('index.php?c=ajax&a=comcityData', function (res) {
        var myChart = echarts.init(document.getElementById('main7'));
        var data = res.data;
        var names = [],valueName = [];
        $('#comcity1').text(data[0].city_name)
        $('#comcity1rate').text(data[0].rate)
        $('#comcity2').text(data[1].city_name)
        $('#comcity2rate').text(data[1].rate)
        $('#comcity3').text(data[2].city_name)
        $('#comcity3rate').text(data[2].rate)
        // 初始化插件
        $('.comcity_counter').countUp({
            delay: 10,
            time: 2000
        });
        data.forEach(function(item,index){
            // if (index <= 4) {
            //     names.push({ name: item.city_name });
            //     if (index == 2) {
            //         names.push('\n');
            //         names.push('\n');
            //     }
            // }
            valueName.push({ value: item.rate, name: item.city_name });
        });
        var option = {
            color: ['#F4003E', '#029DF7', '#E7AD1B', '#58DBD3', '#424BBD'],
            tooltip: {
                trigger: 'item',
                formatter: "{b}:{d}%"
            },
            legend: {
                id: 'legend7',
                orient: 'vertical',
                left: 'right',
                // top: '30%',
                type: "scroll",
                pageIconSize: '',
                textStyle: {
                    color: '#fff',
                },
                pageTextStyle: {
                    color: "#101133",
                },
            },
            // legend: {
            //     left: 'right',
            //     textStyle: {
            //         color: '#fff'
            //     },
            //     data: names,
            // },
            series: [
                {
                    name: '地区分布',
                    type: 'pie',
                    radius: '54%',
                    center: ['35%', '40%'],
                    avoidLabelOverlap: false,
                    label: {
                        show: true,
                        position: 'inside',
                        formatter: '{d}%',
                        color: "#fff",
                        fontSize: 14,
                    },
                    itemStyle: {
                        // borderRadius: 25,
                        borderColor: '#101133',
                        borderWidth: 2
                    },
                    labelLine: {
                        show: false,
                    },
                    data: valueName
                }
            ]
        };
        myChart.setOption(option);
        var cont = valueName.length,
            i = 0;
        clearInterval(legendintrval7);
        legendintrval7 = setInterval(() => {
            myChart.dispatchAction({
                type: 'legendScroll',
                scrollDataIndex: i,
                legendId: 'legend7'
            });
            i = i < (cont - 3) ? i + 1 : 0;
        }, 1200);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart8() {
    $.get('index.php?c=ajax&a=comgmData', function (res) {
        var myChart = echarts.init(document.getElementById('main8'));
        var data = res.data;
        var names = [];
        $('#comgm1').text(data[0].mun_n)
        $('#comgm1rate').text(data[0].rate)
        $('#comgm2').text(data[1].mun_n)
        $('#comgm2rate').text(data[1].rate)
        $('#comgm3').text(data[2].mun_n)
        $('#comgm3rate').text(data[2].rate)
        $('#comgm4').text(data[3].mun_n)
        $('#comgm4rate').text(data[3].rate)
        // 初始化插件
        $('.comgm_counter').countUp({
            delay: 10,
            time: 2000
        });
        data.forEach(function(item){
            names.push(item.mun_n);
        });
        var option = {
            tooltip: {
                trigger: 'axis',
                formatter: "{b}<br/> {c}%",
                axisPointer: {
                    type: 'shadow'
                }
            },
            grid: {
                top: '5%',
                left: '2%',
                right: '4%',
                bottom: '4%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                data: names,
                axisLine: {
                    show: false
                },
                axisTick: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    }
                }
            },
            yAxis: {
                type: 'value',
                splitLine: {
                    lineStyle: {
                        // type: 'dashed',
                        color: '#ABD7FF'
                    },
                    show: true
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    }
                }
            },
            series: [
                {
                    name: '规模分布',
                    data: [
                        {
                            value: data[0].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#029DF7"
                                    }], false)
                                }
                            }
                        },
                        {
                            value: data[1].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#F4003E"
                                    }], false)
                                }
                            }
                        },
                        {
                            value: data[2].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#58DBD3"
                                    }], false)
                                }
                            }
                        },
                        {
                            value: data[3].rate,
                            itemStyle: {
                                normal: {
                                    color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                        offset: 0,
                                        color: "#101133"
                                    }, {
                                        offset: 1,
                                        color: "#E7AD1B"
                                    }], false)
                                }
                            }
                        }
                    ],
                    type: 'bar',
                    barWidth: 20,
                }
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart9() {
    $.get('index.php?c=ajax&a=comxzData', function (res) {
        var myChart = echarts.init(document.getElementById('main9'));
        var data = res.data;
        var names = [], valueName = [];
        $('#compr1').text(data[0].pr_n)
        $('#compr2').text(data[1].pr_n)
        $('#compr3').text(data[2].pr_n)
        data.forEach(function(item){
            names.push(item.pr_n);
            valueName.push(item.rate);
        });
        var option = {
            tooltip: {
                trigger: 'axis',
                axisPointer: {
                    type: 'shadow'
                },
                formatter: '{b}<br/>{c}%'
            },
            grid: {
                top: '5%',
                left: '3%',
                right: '4%',
                bottom: '3%',
                containLabel: true
            },
            xAxis: {
                type: 'value',
                inverse: false,
                axisLine: {
                    show: false
                },
                axisTick: {
                    show: false
                },
                splitLine: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    },
                    show: false
                }
            },
            yAxis: {
                type: 'category',
                data: names,
                axisTick: {
                    show: false
                },
                inverse: true,
                axisLine: {
                    show: false
                },
                splitLine: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    },
                    show: true
                },
            },
            series: [
                {
                    name: '性质分布',
                    type: 'bar',
                    stack: 'total',
                    label: {
                        // show: true,
                        formatter: '{c}%'
                    },
                    emphasis: {
                        focus: 'series'
                    },
                    data: valueName,
                    barWidth: 20,
                    itemStyle: {
                        normal: {
                            //这里是颜色
                            color: function (params) {
                                var colorList = ['#424BBD', '#58DBD3', '#029DF7', '#F4003E', '#FF9337', '#FFC722', '#4367F9'];
                                return colorList[params.dataIndex]
                            }
                        },
                    }
                }
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart10() {
    $.get('index.php?c=ajax&a=comLogChart', function (res) {
        var myChart = echarts.init(document.getElementById('main10'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            data.forEach(function(item){
                names.push(item.month);
                valueName.push(item.num);
            });
        }
        var option = {
            color: ['#4367F9', '#46a6ff', '#5259F4'],
            // legend: {},
            // tooltip: {
            //     trigger: 'axis',
            //     axisPointer: {
            //         type: 'shadow'
            //     }
            // },
            grid: {
                top: '5%',
                left: '2%',
                right: '3%',
                bottom: '4%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                boundaryGap: false,
                data: names,
                axisLine: { show: false },
                axisTick: { show: false },
                splitLine: {
                    lineStyle: {
                        type: 'dashed',
                        color: '#f3f3f3'
                    },
                    show: false
                },
            },
            yAxis: {
                type: 'value',
                axisTick: {
                    show: false
                },
                axisLine: {
                    show: false
                },
                axisLabel: {
                    show: false
                },
                splitLine: {
                    lineStyle: {
                        color: '#68a5df'
                    },
                    show: true
                },
            },
            series: [
                {
                    name: '登录次数',
                    data: valueName,
                    type: 'line',
                    // smooth: true,
                    areaStyle: {},
                    itemStyle: {
                        normal: {
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                offset: 0,
                                color: "#101133"
                            }, {
                                offset: 1,
                                color: "#4367F9"
                            }], false)
                        }
                    }
                },
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
function echart11() {
    $.get('index.php?c=ajax&a=comJobChart', function (res) {
        var myChart = echarts.init(document.getElementById('main11'));
        var data = res.data;
        var names = [],valueName = [];
        if (data.length > 0) {
            data.forEach(function(item){
                names.push(item.month);
                valueName.push(item.num);
            });
        }
        var option = {
            // tooltip: {
            //     trigger: 'axis',
            //     axisPointer: {
            //         type: 'shadow'
            //     }
            // },
            grid: {
                top: '5%',
                left: '2%',
                right: '4%',
                bottom: '4%',
                containLabel: true
            },
            xAxis: {
                type: 'category',
                data: names,
                axisLine: {
                    show: false
                },
                axisTick: {
                    show: false
                },
                axisLabel: {
                    textStyle: {
                        color: '#fff',
                        fontSize: 12,
                    }
                }
            },
            yAxis: {
                type: 'value',
                splitLine: {
                    lineStyle: {
                        // type: 'dashed',
                        color: '#ABD7FF'
                    },
                    show: true
                },
                axisLabel: {
                    show: false
                }
            },
            series: [
                {
                    name: '发布岗位',
                    data: valueName,
                    type: 'bar',
                    // barWidth: 30,
                    barCategoryGap: 8,
                    itemStyle: {
                        normal: {
                            barBorderRadius: [15, 15, 0, 0],
                            color: new echarts.graphic.LinearGradient(0, 1, 0, 0, [{
                                offset: 0,
                                color: "#101133"
                            }, {
                                offset: 1,
                                color: "#F4003E"
                            }], false)
                        },
                    }
                }
            ]
        };
        myChart.setOption(option);
        setTimeout(function () {
            window.addEventListener("resize", () => {
                myChart.resize();
            });
        }, 200);
    }, 'json')
};
