function isInArray(arr, value) {
    for (var i=0; i < arr.length; i++){
        if (value === arr[i]){
            return true;
        }
    }
    return false;
}

function index_job_new(allow_select_jobclass_count, target_jobclassin_names, target_jobclassin_ids, jobdiv_style, isShowBigClass)
{

    window.allow_select_jobclass_count = allow_select_jobclass_count;   //  选择数量
    window.target_jobclassin_names = target_jobclassin_names;           //  点击id
    window.target_jobclassin_ids = target_jobclassin_ids;               //  选择类别存放的id
    window.isShowBigClass = isShowBigClass;                             //  允许选择二级

    if (window.allow_select_jobclass_count == 1) {

        window.style = "style=visibility:hidden;";
    } else {

        window.style = "style=''";
    }

    var layerHtml = [];

    layerHtml.push('<section class="pupopChoice" id="ChooseJobClassLayer">' +
                        '<div class="pupopConAll">');

    /*头部搜索部分*/
    layerHtml.push('<div class="pupopConAllOne">' +
                        '<div class="conAllOneOice">' +
                            '<p>选择职位类别</p>' +
                            '<div class="conAllOneInput">' +
                                '<button><img src="images/fangd.png?v='+cachecode+'" alt=""></button>' +
                                '<input id="ChooseJobSearch" type="text" placeholder="搜索职位类别" value="">' +
                            '</div>' +
                        '</div>' +
                        '<div class="conAllOneClose">' +
                            '<img src="images/coloes.png?v='+cachecode+'" alt="">' +
                        '</div>' +
                    '</div>');

    /*分类信息内容 Start → */
    layerHtml.push('<div class="pupopTwoFrist" id="def">');

    /*一级分类*/
    layerHtml.push('<div class="pupopFristLeft"><ul id="newPublicClassJobOne"></ul></div>');

    /*二级分类（三级分类）*/
    layerHtml.push('<div class="pupopFristRight" id="newPublicClassJobTwo"></div>');

    layerHtml.push('</div>');
    /*分类信息内容 End ← */

    layerHtml.push('<div class="pupopTwoFrist" id="sea" style="display: none;" >');
    /*一级分类搜索*/
    layerHtml.push('<div class="pupopFristLeft"><ul id="newSPublicClassJobOne"></ul></div>');

    /*二级分类（三级分类）搜索*/
    layerHtml.push('<div class="pupopFristRight" id="newSPublicClassJobTwo"></div>');

    layerHtml.push('</div>');

    layerHtml.push('<div class="pupopNoSearch" style="display: none;"><div style=\"text-align:center; font-weight:bold; margin-top:250px;\">抱歉，没有找到结果！</div></div>');

    layerHtml.push('<div class="pupopConAllButt">\n' +
                        '<div class="footerp2">\n' +
                            '<div class="footerp2Name">\n' +
                                '<p>已选：</p>\n' +
                            '</div>\n' +
                            '<div class="footerpFlex">\n' +
                                '<div class="footerp2List">\n' +
                                '</div>' +
                            '</div>' +
                            // '<div class="footerp2More">\n' +
                            //     '<p>更多</p>\n' +
                            //     '<i class="layui-icon layui-icon-down"></i>\n' +
                            // '</div>' +
                        '</div>' +
                        '<div class="footerp4">\n' +
                            '<a class="footerp4Cancel">取消</a>\n' +
                            '<a class="footerp4Buts">确定</a>\n' +
                        '</div>' +
                    '</div>');

    layerHtml.push('</div></section>');

    $("body").append(layerHtml.join(""));

    var listHtml = [];

    var listTwoHtml = [];

    $(ji).each(function (i, data) { //一级类别显示

        if (i == 0){
            listHtml.push('<li class="fristCur">\n' +
                            '<div class="fristLeftp1">' +
                                '<span code=' + data + ' name=' + jn[data] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                '<p>'+jn[data]+'</p>\n' +
                            '</div>\n' +
                            '<div class="fristLeftp2">\n' +
                                '<i class="layui-icon layui-icon-right"></i>\n' +
                            '</div>\n' +
                        '</li>');
            listTwoHtml.push('<div class="fristRightCon fristOn"><div class="secondTab">');
        }else{
            listHtml.push('<li>\n' +
                            '<div class="fristLeftp1">' +
                                '<span code=' + data + ' name=' + jn[data] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                '<p>'+jn[data]+'</p>\n' +
                            '</div>\n' +
                            '<div class="fristLeftp2">\n' +
                                '<i class="layui-icon layui-icon-right"></i>\n' +
                            '</div>\n' +
                        '</li>');
            listTwoHtml.push('<div class="fristRightCon"><div class="secondTab">');
        }

        listTwoHtml.push('<div class="secondTabLeft">');

        listTwoHtml.push('<ul>');

        $(jt[data]).each(function(ii,data2){//二级类别显示
            if (i == 0 && ii == 0){
                listTwoHtml.push('<li class="secoCur" >\n' +
                            '<div class="tabLeftName" precode='+data+'>\n' +
                                '<span code='+data2+' name='+jn[data2]+' precode='+data+'><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                '<p>'+jn[data2]+'</p>\n' +
                            '</div>\n' +
                            '<div class="tabLeftFanx">\n' +
                                '<i class="layui-icon layui-icon-right"></i>\n' +
                            '</div>\n' +
                        '</li>');

            }else{
                listTwoHtml.push('<li>\n' +
                            '<div class="tabLeftName" precode='+data+'>\n' +
                                '<span code='+data2+' name='+jn[data2]+' precode='+data+'><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                '<p>'+jn[data2]+'</p>\n' +
                            '</div>\n' +
                            '<div class="tabLeftFanx">\n' +
                                '<i class="layui-icon layui-icon-right"></i>\n' +
                            '</div>\n' +
                        '</li>');

            }
        });

        listTwoHtml.push('</ul>');

        listTwoHtml.push('<div class="secondTabRight">');
        $(jt[data]).each(function(ii,data2) {//三级级类别显示
            if (ii == 0) {
                listTwoHtml.push('<div class="tabRightCont secoOn">\n' +
                    '<div class="thirdChoice">');
            }else{
                listTwoHtml.push('<div class="tabRightCont">\n' +
                    '<div class="thirdChoice">');
            }

            $(jt[data2]).each(function(iii,data3) {//三级级类别显示
                listTwoHtml.push('<div class="thirdList" code='+data3+' name='+jn[data3]+' precode='+data2+' fircode='+data+'>\n' +
                                    '<span><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                    '<p>'+jn[data3]+'</p>\n' +
                                '</div>');
            });

            listTwoHtml.push('</div>\n' +
                            '</div>');
        });
        listTwoHtml.push('</div>');
        listTwoHtml.push('</div>');

        listTwoHtml.push('</div></div>');
    });

    $("#newPublicClassJobOne").html(listHtml.join(""));
    $("#newPublicClassJobTwo").html(listTwoHtml.join(""));

    if ($(""+window.target_jobclassin_ids+"").val().length == 0) {

        $("#ChooseJobClassLayer .footerp2List").html("");
        $("#ChooseJobClassLayer .footerp2More").html("");
    }else {

        var JobClassArray = $(""+window.target_jobclassin_ids+"").val().length > 0 ? $(""+window.target_jobclassin_ids+"").val().split(",") : new Array(0);

        var firstJob = JobClassArray[0];

        $("#ChooseJobClassLayer .footerp2List div").remove();

        $(ji).each(function (i, data) {

            for(var i1=0;i1<JobClassArray.length;i1++){
                if(JobClassArray[i1]==data){

                    if (data == firstJob) {

                        var firstLi = $("#ChooseJobClassLayer .fristLeftp1 span[code=\"" + data + "\"]")[0].parentNode.parentNode;
                        $(firstLi).addClass("fristCur").siblings().removeClass('fristCur');
                        $(".fristRightCon").eq($(".pupopFristLeft li").index(firstLi)).addClass("fristOn").siblings().removeClass('fristOn');
                    }

                    $("#ChooseJobClassLayer .footerp2List").append('<p class="footNone" code="'+data+'" name="'+jn[data]+'"><span>'+jn[data]+'</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
                    $("#ChooseJobClassLayer .fristLeftp1 span[code=\"" + data + "\"]").addClass('spanLinkOne');

                    $("#ChooseJobClassLayer .tabLeftName[precode=\"" + data + "\"]").addClass('mustNot');
                    $("#ChooseJobClassLayer .thirdChoice div[fircode=\"" + data + "\"]").addClass('mustNotSan');
                }
            }

            $(jt[data]).each(function(ii,data2){

                for(var i2=0;i2<JobClassArray.length;i2++){
                    if(JobClassArray[i2]==data2){

                        if (data2 == firstJob) {

                            var firstLi = $("#ChooseJobClassLayer .fristLeftp1 span[code=\"" + data + "\"]")[0].parentNode.parentNode;
                            $(firstLi).addClass("fristCur").siblings().removeClass('fristCur');
                            $(".fristRightCon").eq($(".pupopFristLeft li").index(firstLi)).addClass("fristOn").siblings().removeClass('fristOn');

                            var secondLi = $("#ChooseJobClassLayer .tabLeftName span[code=\"" + data2 + "\"]")[0].parentNode.parentNode;
                            $(secondLi).addClass("secoCur").siblings().removeClass('secoCur');
                            $(".tabRightCont").eq($(".secondTabLeft li").index(secondLi)).addClass("secoOn").siblings().removeClass('secoOn');
                        }

                        $("#ChooseJobClassLayer .footerp2List").append('<p class="footNone" code="'+data2+'" precode="'+data+'" name="'+jn[data2]+'"><span>'+jn[data2]+'</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
                        $("#ChooseJobClassLayer .tabLeftName span[code=\"" + data2 + "\"]").addClass('spanLinkTwo');
                        $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + data2 + "\"]").addClass('mustNotSan');
                    }
                }

                $(jt[data2]).each(function (iii, data3) {

                    if (data3 == firstJob){

                        var firstLi =   $("#ChooseJobClassLayer .fristLeftp1 span[code=\"" + data + "\"]")[0].parentNode.parentNode;
                        $(firstLi).addClass("fristCur").siblings().removeClass('fristCur');
                        $(".fristRightCon").eq($(".pupopFristLeft li").index(firstLi)).addClass("fristOn").siblings().removeClass('fristOn');

                        var secondLi =  $("#ChooseJobClassLayer .tabLeftName span[code=\"" + data2 + "\"]")[0].parentNode.parentNode;
                        $(secondLi).addClass("secoCur").siblings().removeClass('secoCur');
                        $(".tabRightCont").eq($(".secondTabLeft li").index(secondLi)).addClass("secoOn").siblings().removeClass('secoOn');
                    }

                    for(var i3=0;i3<JobClassArray.length;i3++){
                        if(JobClassArray[i3]==data3){

                            $("#ChooseJobClassLayer .footerp2List").append('<p class="footNone" code="'+data3+'" precode="'+data2+'" fircode="'+data+'" name="'+jn[data3]+'"><span>'+jn[data3]+'</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
                            $("#ChooseJobClassLayer .thirdChoice div[code=\"" + data3 + "\"]").addClass('thirdLink');
                        }
                    }
                });
            });
        });
    }

    window.newjob_layer = $.layer({
        type: 1,
        title: '请选择职位类别',
        shift: 'top',
        closeBtn: [0, true],
        border: [10, 0.3, '#000', true],
        area: ['980px', '620px'],
        page: {dom: "#ChooseJobClassLayer"},
        close: function (index) {
            $("#ChooseJobClassLayer").remove();
        }
    });

    if (window.newjob_layer) {
        //  搜索职位类别
        $("#ChooseJobSearch").on("keyup", function () {
            var $this = $(this);
            var txt = $this.val();
            if (txt.length == 0) {//没有关键字，显示全部

                $("#ChooseJobClassLayer #def").show();
                $("#ChooseJobClassLayer #sea").hide();
                $("#ChooseJobClassLayer .pupopNoSearch").hide();
            } else {
                //搜索数据源
                var sListHtml = [];
                var sListTwoHtml = [];
                var iArr = [];
                jn.forEach(function (data, i) {
                    if (data.toString().indexOf(txt) > -1) {    //  匹配到关键字

                        iArr.push(i);                           //  类ID
                    }
                });

                var oneS = false;

                var oneArr = [];
                var oneSArr = [];
                var noOneArr = [];

                var twoIdsArr = [];

                jn.forEach(function (data, i) {
                    if (data.toString().indexOf(txt) > -1) {    //  匹配到关键字

                        $(ji).each(function (onek, onev) {
                            if (!isInArray(oneArr, onev)) {
                                oneArr.push(onev);
                            }
                            if (i == onev) {                     //  一级匹配到关键字
                                if (!isInArray(oneSArr, onev)) {
                                    oneSArr.push(onev);
                                }
                                if (!oneS) {
                                    sListHtml.push('<li class="fristCur">\n' +
                                        '<div class="fristLeftp1">' +
                                        '<span code=' + onev + ' name=' + jn[onev] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                        '<p>' + jn[onev] + '</p>\n' +
                                        '</div>\n' +
                                        '<div class="fristLeftp2">\n' +
                                        '<i class="layui-icon layui-icon-right"></i>\n' +
                                        '</div>\n' +
                                        '</li>');
                                } else {
                                    sListHtml.push('<li>\n' +
                                        '<div class="fristLeftp1">' +
                                        '<span code=' + onev + ' name=' + jn[onev] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                        '<p>' + jn[onev] + '</p>\n' +
                                        '</div>\n' +
                                        '<div class="fristLeftp2">\n' +
                                        '<i class="layui-icon layui-icon-right"></i>\n' +
                                        '</div>\n' +
                                        '</li>');
                                }
                                oneS = true;

                                sListTwoHtml.push('<div class="fristRightCon fristOn"><div class="secondTab">');

                                sListTwoHtml.push('<div class="secondTabLeft">');

                                sListTwoHtml.push('<ul>');

                                var twoS = false;

                                $(jt[onev]).each(function (twok, twov) {
                                    if (isInArray(iArr, twov)) {

                                        if (!isInArray(twoIdsArr, twov)) {

                                            twoIdsArr.push(twov);
                                        }

                                        if (!twoS) {
                                            sListTwoHtml.push('<li class="secoCur">\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        } else {

                                            sListTwoHtml.push('<li>\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        }

                                        twoS = true;
                                    }
                                });
                                if (!twoS) {
                                    $(jt[onev]).each(function (twok, twov) {
                                        if (!isInArray(twoIdsArr, twov)) {

                                            twoIdsArr.push(twov);
                                        }
                                        if (twok == 0) {
                                            sListTwoHtml.push('<li class="secoCur">\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        } else {
                                            sListTwoHtml.push('<li>\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        }
                                    });
                                }

                                sListTwoHtml.push('</ul>');

                                sListTwoHtml.push('<div class="secondTabRight">');

                                var thrKey = -1;
                                $(jt[onev]).each(function (twok, twov) {

                                    if (isInArray(twoIdsArr, twov)) {

                                        if (thrKey == -1) {

                                            sListTwoHtml.push('<div class="tabRightCont secoOn">\n' +
                                                '<div class="thirdChoice">');

                                            thrKey = twok;
                                        } else {
                                            sListTwoHtml.push('<div class="tabRightCont">\n' +
                                                '<div class="thirdChoice">');
                                        }

                                        var threeS = false;
                                        $(jt[twov]).each(function (threek, threev) {
                                            if (isInArray(iArr, threev)) {
                                                threeS = true;
                                                sListTwoHtml.push('<div class="thirdList" code=' + threev + ' name=' + jn[threev] + ' precode=' + twov + ' fircode=' + onev + '>\n' +
                                                    '<span><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                    '<p>' + jn[threev] + '</p>\n' +
                                                    '</div>');
                                            }
                                        });
                                        if (!threeS) {
                                            $(jt[twov]).each(function (threek, threev) {
                                                sListTwoHtml.push('<div class="thirdList" code=' + threev + ' name=' + jn[threev] + ' precode=' + twov + ' fircode=' + onev + '>\n' +
                                                    '<span><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                    '<p>' + jn[threev] + '</p>\n' +
                                                    '</div>');
                                            });
                                        }

                                        sListTwoHtml.push('</div>\n' +
                                            '</div>');
                                    }
                                });
                                sListTwoHtml.push('</div>');
                                sListTwoHtml.push('</div>');

                                sListTwoHtml.push('</div></div>');
                            }
                        });
                    }
                });

                if (oneSArr.length > 0) {

                    noOneArr = oneArr.filter(function (val) {
                        return !isInArray(oneSArr, val)
                    });
                } else {

                    noOneArr = oneArr;
                }

                if (noOneArr.length > 0) {

                    var firIdArr = [];
                    var secIdArr = [];

                    jn.forEach(function (data, i) {
                        if (data.toString().indexOf(txt) > -1) {    //  匹配到关键字
                            noOneArr.forEach(onev => {
                                $(jt[onev]).each(function (twok, twov) {
                                    if (i == twov) {
                                        if (!isInArray(firIdArr, onev)) {
                                            firIdArr.push(onev);
                                        }
                                        secIdArr.push(twov);
                                    }
                                });
                            });
                        }
                    });

                    if (firIdArr.length > 0 && secIdArr.length > 0) {

                        noOneArr = noOneArr.filter(function (val) {
                            return !isInArray(firIdArr, val)
                        });

                        $(ji).each(function (onek, onev) {
                            if (isInArray(firIdArr, onev)) {

                                if (!oneS) {
                                    sListHtml.push('<li class="fristCur">\n' +
                                        '<div class="fristLeftp1">' +
                                        '<span code=' + onev + ' name=' + jn[onev] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                        '<p>' + jn[onev] + '</p>\n' +
                                        '</div>\n' +
                                        '<div class="fristLeftp2">\n' +
                                        '<i class="layui-icon layui-icon-right"></i>\n' +
                                        '</div>\n' +
                                        '</li>');
                                } else {
                                    sListHtml.push('<li>\n' +
                                        '<div class="fristLeftp1">' +
                                        '<span code=' + onev + ' name=' + jn[onev] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                        '<p>' + jn[onev] + '</p>\n' +
                                        '</div>\n' +
                                        '<div class="fristLeftp2">\n' +
                                        '<i class="layui-icon layui-icon-right"></i>\n' +
                                        '</div>\n' +
                                        '</li>');
                                }
                                oneS = true;

                                sListTwoHtml.push('<div class="fristRightCon fristOn"><div class="secondTab">');

                                sListTwoHtml.push('<div class="secondTabLeft">');

                                sListTwoHtml.push('<ul>');

                                var twoS = false;
                                $(jt[onev]).each(function (twok, twov) {
                                    if (isInArray(secIdArr, twov)) {

                                        if (!twoS) {
                                            sListTwoHtml.push('<li class="secoCur">\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        } else {

                                            sListTwoHtml.push('<li>\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        }

                                        twoS = true;
                                    }
                                });

                                sListTwoHtml.push('</ul>');

                                sListTwoHtml.push('<div class="secondTabRight">');

                                var threeKey = 0;
                                $(jt[onev]).each(function (twok, twov) {
                                    if (isInArray(secIdArr, twov)) {
                                        if (threeKey == 0) {

                                            sListTwoHtml.push('<div class="tabRightCont secoOn">\n' +
                                                '<div class="thirdChoice">');
                                        } else {

                                            sListTwoHtml.push('<div class="tabRightCont">\n' +
                                                '<div class="thirdChoice">');
                                        }

                                        threeKey = 1;

                                        var threeS = false;
                                        $(jt[twov]).each(function (threek, threev) {
                                            if (isInArray(iArr, threev)) {
                                                threeS = true;
                                                sListTwoHtml.push('<div class="thirdList" code=' + threev + ' name=' + jn[threev] + ' precode=' + twov + ' fircode=' + onev + '>\n' +
                                                    '<span><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                    '<p>' + jn[threev] + '</p>\n' +
                                                    '</div>');
                                            }
                                        });

                                        sListTwoHtml.push('</div>\n' +
                                            '</div>');
                                    }
                                });
                                sListTwoHtml.push('</div>');
                                sListTwoHtml.push('</div>');

                                sListTwoHtml.push('</div></div>');
                            }
                        });
                    }
                }

                if (noOneArr.length > 0) {       //  查询第三级类别

                    var firIdArr = [];
                    var secIdArr = [];
                    var thrIdArr = [];

                    jn.forEach(function (data, i) {
                        if (data.toString().indexOf(txt) > -1) {    //  匹配到关键字

                            noOneArr.forEach(onev => {
                                $(jt[onev]).each(function (twok, twov) {
                                    $(jt[twov]).each(function (threek, threev) {
                                        if (i == threev) {

                                            if (!isInArray(firIdArr, onev)) {
                                                firIdArr.push(onev);
                                            }
                                            if (!isInArray(secIdArr, twov)) {
                                                secIdArr.push(twov);
                                            }
                                            thrIdArr.push(threev);
                                        }
                                    });
                                });
                            });
                        }
                    });

                    if (firIdArr.length > 0 && secIdArr.length > 0 && thrIdArr.length > 0) {

                        $(ji).each(function (onek, onev) {
                            if (isInArray(firIdArr, onev)) {

                                if (!oneS) {
                                    sListHtml.push('<li class="fristCur">\n' +
                                        '<div class="fristLeftp1">' +
                                        '<span code=' + onev + ' name=' + jn[onev] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                        '<p>' + jn[onev] + '</p>\n' +
                                        '</div>\n' +
                                        '<div class="fristLeftp2">\n' +
                                        '<i class="layui-icon layui-icon-right"></i>\n' +
                                        '</div>\n' +
                                        '</li>');
                                } else {
                                    sListHtml.push('<li>\n' +
                                        '<div class="fristLeftp1">' +
                                        '<span code=' + onev + ' name=' + jn[onev] + ' ><i class="layui-icon layui-icon-ok"></i></span>' +
                                        '<p>' + jn[onev] + '</p>\n' +
                                        '</div>\n' +
                                        '<div class="fristLeftp2">\n' +
                                        '<i class="layui-icon layui-icon-right"></i>\n' +
                                        '</div>\n' +
                                        '</li>');
                                }
                                oneS = true;

                                sListTwoHtml.push('<div class="fristRightCon fristOn"><div class="secondTab">');

                                sListTwoHtml.push('<div class="secondTabLeft">');

                                sListTwoHtml.push('<ul>');

                                var twoNS = false;
                                $(jt[onev]).each(function (twok, twov) {
                                    if (isInArray(secIdArr, twov)) {

                                        if (!twoNS) {
                                            sListTwoHtml.push('<li class="secoCur">\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        } else {

                                            sListTwoHtml.push('<li>\n' +
                                                '<div class="tabLeftName" precode=' + onev + '>\n' +
                                                '<span code=' + twov + ' name=' + jn[twov] + ' precode=' + onev + '><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                '<p>' + jn[twov] + '</p>\n' +
                                                '</div>\n' +
                                                '<div class="tabLeftFanx">\n' +
                                                '<i class="layui-icon layui-icon-right"></i>\n' +
                                                '</div>\n' +
                                                '</li>');
                                        }

                                        twoNS = true;
                                    }
                                });

                                sListTwoHtml.push('</ul>');

                                sListTwoHtml.push('<div class="secondTabRight">');

                                var threeKey = 0;
                                $(jt[onev]).each(function (twok, twov) {
                                    if (isInArray(secIdArr, twov)) {
                                        if (threeKey == 0) {

                                            sListTwoHtml.push('<div class="tabRightCont secoOn">\n' +
                                                '<div class="thirdChoice">');
                                        } else {

                                            sListTwoHtml.push('<div class="tabRightCont">\n' +
                                                '<div class="thirdChoice">');
                                        }

                                        threeKey = 1;

                                        var threeS = false;
                                        $(jt[twov]).each(function (threek, threev) {
                                            if (isInArray(thrIdArr, threev)) {
                                                threeS = true;
                                                sListTwoHtml.push('<div class="thirdList" code=' + threev + ' name=' + jn[threev] + ' precode=' + twov + ' fircode=' + onev + '>\n' +
                                                    '<span><i class="layui-icon layui-icon-ok"></i></span>\n' +
                                                    '<p>' + jn[threev] + '</p>\n' +
                                                    '</div>');
                                            }
                                        });

                                        sListTwoHtml.push('</div>\n' +
                                            '</div>');
                                    }
                                });
                                sListTwoHtml.push('</div>');
                                sListTwoHtml.push('</div>');

                                sListTwoHtml.push('</div></div>');
                            }
                        });
                    }
                }


                $("#newSPublicClassJobOne").html(sListHtml.join(""));
                $("#newSPublicClassJobTwo").html(sListTwoHtml.join(""));

                if (sListHtml.length == 0) {//没有关键字

                    $("#ChooseJobClassLayer #sea").hide();
                    $("#ChooseJobClassLayer .pupopNoSearch").show();
                } else {

                    setSearchChecked();
                    $("#ChooseJobClassLayer #sea").show();
                    $("#ChooseJobClassLayer .pupopNoSearch").hide();
                }

                $("#ChooseJobClassLayer #def").hide();
            }
        })

        //  一级类目展开操作
        $("body").on('click', ".pupopFristLeft li", function () {

            $(this).addClass("fristCur").siblings().removeClass('fristCur');

            $(".fristRightCon").eq($(".pupopFristLeft li").index(this)).addClass("fristOn").siblings().removeClass('fristOn');
        });

        //  二级类目展开操作
        $('body').on('click', ".secondTabLeft li", function () {

            $(this).addClass("secoCur").siblings().removeClass('secoCur');

            $(".tabRightCont").eq($(".secondTabLeft li").index(this)).addClass("secoOn").siblings().removeClass('secoOn');
        });

        //  弹窗右上角叉号关闭操作
        $(".conAllOneClose").on('click', function () {
            layer.close(window.newjob_layer);
        });

        //  一级类目选择操作
        $("body").on('click', ".fristLeftp1 span",function (event) {

            var jobClassId = $(this).attr('code');
            var jobClassName = $(this).attr('name');

            if ($(this).hasClass('spanLinkOne')) {

                $("#ChooseJobClassLayer .tabLeftName[precode=\"" + jobClassId + "\"]").removeClass('mustNot');
                $("#ChooseJobClassLayer .thirdChoice div[fircode=\"" + jobClassId + "\"]").removeClass('mustNotSan');
                $(this).removeClass('spanLinkOne');

                $("#ChooseJobClassLayer .footNone[code=\"" + jobClassId + "\"]").remove();

            } else {

                $("#ChooseJobClassLayer .tabLeftName[precode=\"" + jobClassId + "\"]").addClass('mustNot');
                $("#ChooseJobClassLayer .tabLeftName span[precode=\"" + jobClassId + "\"]").removeClass('spanLinkTwo');
                $("#ChooseJobClassLayer .thirdList[fircode=\"" + jobClassId + "\"]").removeClass('thirdLink');

                $("#ChooseJobClassLayer .thirdChoice div[fircode=\"" + jobClassId + "\"]").addClass('mustNotSan');
                $(this).addClass('spanLinkOne');

                $('.footerp2List').append('<p class="footNone" code="' + jobClassId + '" name="' + jobClassName + '"><span>' + jobClassName + '</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
                $("#ChooseJobClassLayer .footNone[precode=\"" + jobClassId + "\"]").remove();
                $("#ChooseJobClassLayer .footNone[fircode=\"" + jobClassId + "\"]").remove();
            }

            event.stopPropagation()
        });

        //  二级类目选择操作
        $("body").on('click', ".tabLeftName span", function (event) {

            var parent = this.parentNode;
            var pClassId = $(this).attr('precode');

            var jobClassId = $(this).attr('code');
            var jobClassName = $(this).attr('name');

            if ($(parent).hasClass('mustNot')) {            //  不可点击

                return false;
            } else if ($(this).hasClass('spanLinkTwo')) {   //  已选中

                $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + jobClassId + "\"]").removeClass('mustNotSan');
                $(this).removeClass('spanLinkTwo');

                $("#ChooseJobClassLayer .footNone[code=\"" + jobClassId + "\"]").remove();
            } else {                                        //  未选中

                $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + jobClassId + "\"]").addClass('mustNotSan');
                $("#ChooseJobClassLayer .thirdList[precode=\"" + jobClassId + "\"]").removeClass('thirdLink');
                $(this).addClass('spanLinkTwo');

                $('.footerp2List').append('<p class="footNone" code="' + jobClassId + '" precode="' + pClassId + '" name="' + jobClassName + '"><span>' + jobClassName + '</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
                $("#ChooseJobClassLayer .footNone[precode=\"" + jobClassId + "\"]").remove();
            }

            event.stopPropagation();
        });

        // 三级类目选择操作
        $("body").on('click', ".thirdList", function () {

            var fClassId = $(this).attr('fircode');
            var pClassId = $(this).attr('precode');

            var jobClassId = $(this).attr('code');
            var jobClassName = $(this).attr('name');

            if ($(this).hasClass('mustNotSan')) {

                return false;
            } else if ($(this).hasClass('thirdLink')) {

                $(this).removeClass('thirdLink');

                $("#ChooseJobClassLayer .footNone[code=\"" + jobClassId + "\"]").remove();
            } else {

                $(this).addClass('thirdLink')

                $('.footerp2List').append('<p class="footNone" code="' + jobClassId + '" precode="' + pClassId + '" fircode="' + fClassId + '" name="' + jobClassName + '"><span>' + jobClassName + '</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
            }
        });

        //  删除已选操作
        $('body').on('click', ".footerp2List p", function () {

            console.log(1212);

            var code = $(this).attr('code');
            $("#ChooseJobClassLayer .footNone[code=\"" + code + "\"]").remove();

            var precode = $(this).attr('precode');
            var fircode = $(this).attr('fircode');

            if (typeof (fircode) != 'undefined') {

                $("#ChooseJobClassLayer .thirdList[code=\"" + code + "\"]").removeClass('thirdLink');

                $("#ChooseJobClassLayer .tabLeftName[precode=\"" + code + "\"]").removeClass('mustNot');
                $("#ChooseJobClassLayer .thirdChoice div[fircode=\"" + code + "\"]").removeClass('mustNotSan');
            } else if (typeof (precode) != 'undefined') {

                $("#ChooseJobClassLayer .tabLeftName span[code=\"" + code + "\"]").removeClass('spanLinkTwo');

                $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + code + "\"]").removeClass('mustNotSan');
            } else {

                $("#ChooseJobClassLayer .fristLeftp1 span[code=\"" + code + "\"]").removeClass('spanLinkOne');
            }
        });

        //  取消操作
        $('.footerp4Cancel').on('click', function () {
            layer.close(window.newjob_layer);
        });

        //  确定操作
        $('.footerp4Buts').on('click', function () {

            var jobClassIdArray = [];
            var jobClassNameArray = [];
            $("#ChooseJobClassLayer .footerp2List p").each(function () {
                var code = $(this).attr('code');
                var name = $(this).attr('name');
                jobClassIdArray.push(code);
                jobClassNameArray.push('<p class="closeNone" code="' + code + '"><span>' + name + '</span><img class="listImg" src="images/chaja.png?v='+cachecode+'" alt=""></p>');
            });

            $("" + window.target_jobclassin_ids + "").val(jobClassIdArray.join(","));
            $("" + window.target_jobclassin_names + "").html(jobClassNameArray);

            layer.close(window.newjob_layer);
        });
    }
}

function setSearchChecked() {

    var JobClassArray = [];
    $('.footerp2List p').each(function () {

        JobClassArray.push($(this).attr('code'));
    });

    $(ji).each(function (i, data) {

        for(var i1=0;i1<JobClassArray.length;i1++){
            if(JobClassArray[i1]==data){


                $("#ChooseJobClassLayer .fristLeftp1 span[code=\"" + data + "\"]").addClass('spanLinkOne');

                $("#ChooseJobClassLayer .tabLeftName[precode=\"" + data + "\"]").addClass('mustNot');
                $("#ChooseJobClassLayer .thirdChoice div[fircode=\"" + data + "\"]").addClass('mustNotSan');
            }
        }

        $(jt[data]).each(function(ii,data2){

            for(var i2=0;i2<JobClassArray.length;i2++){
                if(JobClassArray[i2]==data2){

                    $("#ChooseJobClassLayer .tabLeftName span[code=\"" + data2 + "\"]").addClass('spanLinkTwo');
                    $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + data2 + "\"]").addClass('mustNotSan');
                }
            }

            $(jt[data2]).each(function (iii, data3) {
                for(var i3=0;i3<JobClassArray.length;i3++){
                    if(JobClassArray[i3]==data3){

                        $("#ChooseJobClassLayer .thirdChoice div[code=\"" + data3 + "\"]").addClass('thirdLink');
                    }
                }
            });
        });
    });
}