function publicClassT(key, params, fallback) {
    var text = typeof yunT === 'function' ? yunT(key, params, fallback) : (fallback !== undefined ? fallback : key);
    if (params && typeof text === 'string') {
        for (var name in params) {
            if (Object.prototype.hasOwnProperty.call(params, name)) {
                text = text.split('{' + name + '}').join(params[name]);
            }
        }
    }
    return text;
}

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

    window.allow_select_jobclass_count = allow_select_jobclass_count;   // Selection limit.
    window.target_jobclassin_names = target_jobclassin_names;           // Target display field.
    window.target_jobclassin_ids = target_jobclassin_ids;               // Target ID field.
    window.isShowBigClass = isShowBigClass;                             // Allow second-level selection.

    if (window.allow_select_jobclass_count == 1) {

        window.style = "style=visibility:hidden;";
    } else {

        window.style = "style=''";
    }

    var layerHtml = [];

    layerHtml.push('<section class="pupopChoice" id="ChooseJobClassLayer">' +
                        '<div class="pupopConAll">');

    /* Header search area. */
    layerHtml.push('<div class="pupopConAllOne">' +
                        '<div class="conAllOneOice">' +
                            '<p>' + publicClassT('public_js_00001', null, 'Select Job Category') + '</p>' +
                            '<div class="conAllOneInput">' +
                                '<button><img src="images/fangd.png?v='+cachecode+'" alt=""></button>' +
                                '<input id="ChooseJobSearch" type="text" placeholder="' + publicClassT('public_js_00002', null, 'Search job categories') + '" value="">' +
                            '</div>' +
                        '</div>' +
                        '<div class="conAllOneClose">' +
                            '<img src="images/coloes.png?v='+cachecode+'" alt="">' +
                        '</div>' +
                    '</div>');

    /* Category content start. */
    layerHtml.push('<div class="pupopTwoFrist" id="def">');

    /* First-level categories. */
    layerHtml.push('<div class="pupopFristLeft"><ul id="newPublicClassJobOne"></ul></div>');

    /* Second-level and third-level categories. */
    layerHtml.push('<div class="pupopFristRight" id="newPublicClassJobTwo"></div>');

    layerHtml.push('</div>');
    /* Category content end. */

    layerHtml.push('<div class="pupopTwoFrist" id="sea" style="display: none;" >');
    /* First-level category search results. */
    layerHtml.push('<div class="pupopFristLeft"><ul id="newSPublicClassJobOne"></ul></div>');

    /* Second-level and third-level category search results. */
    layerHtml.push('<div class="pupopFristRight" id="newSPublicClassJobTwo"></div>');

    layerHtml.push('</div>');

    layerHtml.push('<div class="pupopNoSearch" style="display: none;"><div style=\"text-align:center; font-weight:bold; margin-top:250px;\">' + publicClassT('public_js_00003', null, 'Sorry, no results found.') + '</div></div>');

    layerHtml.push('<div class="pupopConAllButt">\n' +
                        '<div class="footerp2">\n' +
                            '<div class="footerp2Name">\n' +
                                '<p>' + publicClassT('public_js_00004', null, 'Selected:') + '</p>\n' +
                            '</div>\n' +
                            '<div class="footerpFlex">\n' +
                                '<div class="footerp2List">\n' +
                                '</div>' +
                            '</div>' +
                            // '<div class="footerp2More">\n' +
                            //     '<p>More</p>\n' +
                            //     '<i class="layui-icon layui-icon-down"></i>\n' +
                            // '</div>' +
                        '</div>' +
                        '<div class="footerp4">\n' +
                            '<a class="footerp4Cancel">' + publicClassT('public_js_00005', null, 'Cancel') + '</a>\n' +
                            '<a class="footerp4Buts">' + publicClassT('public_js_00006', null, 'Confirm') + '</a>\n' +
                        '</div>' +
                    '</div>');

    layerHtml.push('</div></section>');

    $("body").append(layerHtml.join(""));

    var listHtml = [];

    var listTwoHtml = [];

    $(ji).each(function (i, data) { // Display first-level categories.

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

        $(jt[data]).each(function(ii,data2){// Display second-level categories.
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
        $(jt[data]).each(function(ii,data2) {// Display third-level categories.
            if (ii == 0) {
                listTwoHtml.push('<div class="tabRightCont secoOn">\n' +
                    '<div class="thirdChoice">');
            }else{
                listTwoHtml.push('<div class="tabRightCont">\n' +
                    '<div class="thirdChoice">');
            }

            $(jt[data2]).each(function(iii,data3) {// Display third-level categories.
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
        title: publicClassT('public_js_00007', null, 'Select a job category'),
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
        // Search job categories.
        $("#ChooseJobSearch").on("keyup", function () {
            var $this = $(this);
            var txt = $this.val();
            if (txt.length == 0) { // No keyword; show all categories.

                $("#ChooseJobClassLayer #def").show();
                $("#ChooseJobClassLayer #sea").hide();
                $("#ChooseJobClassLayer .pupopNoSearch").hide();
            } else {
                // Search the data source.
                var sListHtml = [];
                var sListTwoHtml = [];
                var iArr = [];
                jn.forEach(function (data, i) {
                    if (data.toString().indexOf(txt) > -1) {    // Keyword matched.

                        iArr.push(i);                           // Category ID.
                    }
                });

                var oneS = false;

                var oneArr = [];
                var oneSArr = [];
                var noOneArr = [];

                var twoIdsArr = [];

                jn.forEach(function (data, i) {
                    if (data.toString().indexOf(txt) > -1) {    // Keyword matched.

                        $(ji).each(function (onek, onev) {
                            if (!isInArray(oneArr, onev)) {
                                oneArr.push(onev);
                            }
                            if (i == onev) {                     // First-level keyword matched.
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
                        if (data.toString().indexOf(txt) > -1) {    // Keyword matched.
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

                if (noOneArr.length > 0) {       // Search third-level categories.

                    var firIdArr = [];
                    var secIdArr = [];
                    var thrIdArr = [];

                    jn.forEach(function (data, i) {
                        if (data.toString().indexOf(txt) > -1) {    // Keyword matched.

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

                if (sListHtml.length == 0) { // No matching keyword.

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

        // Expand first-level category.
        $("body").on('click', ".pupopFristLeft li", function () {

            $(this).addClass("fristCur").siblings().removeClass('fristCur');

            $(".fristRightCon").eq($(".pupopFristLeft li").index(this)).addClass("fristOn").siblings().removeClass('fristOn');
        });

        // Expand second-level category.
        $('body').on('click', ".secondTabLeft li", function () {

            $(this).addClass("secoCur").siblings().removeClass('secoCur');

            $(".tabRightCont").eq($(".secondTabLeft li").index(this)).addClass("secoOn").siblings().removeClass('secoOn');
        });

        // Close with the top-right icon.
        $(".conAllOneClose").on('click', function () {
            layer.close(window.newjob_layer);
        });

        // Select first-level category.
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

        // Select second-level category.
        $("body").on('click', ".tabLeftName span", function (event) {

            var parent = this.parentNode;
            var pClassId = $(this).attr('precode');

            var jobClassId = $(this).attr('code');
            var jobClassName = $(this).attr('name');

            if ($(parent).hasClass('mustNot')) {            // Not clickable.

                return false;
            } else if ($(this).hasClass('spanLinkTwo')) {   // Selected.

                $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + jobClassId + "\"]").removeClass('mustNotSan');
                $(this).removeClass('spanLinkTwo');

                $("#ChooseJobClassLayer .footNone[code=\"" + jobClassId + "\"]").remove();
            } else {                                        // Not selected.

                $("#ChooseJobClassLayer .thirdChoice div[precode=\"" + jobClassId + "\"]").addClass('mustNotSan');
                $("#ChooseJobClassLayer .thirdList[precode=\"" + jobClassId + "\"]").removeClass('thirdLink');
                $(this).addClass('spanLinkTwo');

                $('.footerp2List').append('<p class="footNone" code="' + jobClassId + '" precode="' + pClassId + '" name="' + jobClassName + '"><span>' + jobClassName + '</span><img src="images/chaja.png?v='+cachecode+'" alt=""></p>');
                $("#ChooseJobClassLayer .footNone[precode=\"" + jobClassId + "\"]").remove();
            }

            event.stopPropagation();
        });

        // Select third-level category.
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

        // Remove selected category.
        $('body').on('click', ".footerp2List p", function () {


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

        // Cancel.
        $('.footerp4Cancel').on('click', function () {
            layer.close(window.newjob_layer);
        });

        // Confirm.
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