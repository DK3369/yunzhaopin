function datePublicT(key, params, fallback) {
    var text;
    if (typeof yunT === 'function') {
        text = yunT(key, params, fallback);
    } else if (typeof yunAt === 'function') {
        text = yunAt(key, params, fallback);
    } else {
        text = fallback !== undefined ? fallback : key;
    }
    if (params && typeof text === 'string') {
        for (var name in params) {
            if (Object.prototype.hasOwnProperty.call(params, name)) {
                text = text.split('{' + name + '}').join(params[name]);
            }
        }
    }
    return text;
}

function datePublicEsc(text) {
    return String(text)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

function datePublicTitle(options) {
    return '<div class="title"><img class="img_ft" src="' + userstyle + '/images/tit_ft.png" title="' +
        datePublicEsc(datePublicT('date_js_00001', null, 'Earlier')) + '" id="' + options.type + 'up"/>' +
        datePublicEsc(datePublicT('date_js_00004', {start: options.start, end: options.end}, '{start} - {end}')) +
        '<img id="' + options.type + 'next" class="img_rt" src="' + userstyle + '/images/tit_rt.png" title="' +
        datePublicEsc(datePublicT('date_js_00002', null, 'Later')) + '"></div>';
}

function datePublicYear(year) {
    return datePublicEsc(datePublicT('date_js_00003', {year: year}, '{year}'));
}

function datePublicMonth(month) {
    return datePublicEsc(datePublicT('date_js_00006', {month: month}, '{month}'));
}

function datePublicDay(day) {
    return datePublicEsc(datePublicT('date_js_00008', {day: day}, '{day}'));
}

(function ($) {
    $.fn.yDate = function (options) {
        var $id = this;
        var defaults = {
            valueid: "value",
            start: 1997,
            end: 2016,
			nextid:'',
			newid:'',
            number: 5,
            titleColor: "#c1c1c1",
            fontColor: "white"
        }
        var options = $.extend(defaults, options);
        this.children().remove();
        this.append(datePublicTitle(options));
        var years = parseInt(options.end) - parseInt(options.start);
        var num = (100 / parseInt(options.number)) + "%";
		var text='<div class="list_ct"><ul>';
        for (var i = 0; i < years + 1; i++) {
            text += "<li class=\""+options.type+"Date-year\"><a href=\"javascript:void(0);\">" + datePublicYear(parseInt(options.start) + i) + "</a></li>";
        }
		text+='</ul></div>';
		this.append(text);
        Bind(options);
        up(options, $id, years);
        next(options, $id, years);
    };
    function up(options, $id, years) {
        $("#"+options.type+"up").unbind();
        $("#"+options.type+"up").click(function () {
            options.start = options.start - years;
            options.end = options.end - years;
            $id.children().remove();
            $id.append(datePublicTitle(options));
            years = parseInt(options.end) - parseInt(options.start);
            num = (100 / parseInt(options.number)) + "%";
            var text='<div class="list_ct"><ul>';
			for (var i = 0; i < years + 1; i++) {
                text+= "<li class=\""+options.type+"Date-year\"><a href=\"javascript:void(0);\">" + datePublicYear(parseInt(options.start) + i) + "</a></li>";
            }
			text+='</ul></div>';
			$id.append(text);
            up(options, $id, years);
            next(options, $id, years);
            Bind(options);
        });
    }
    function next(options, $id, years) {
        $("#"+options.type+"next").unbind();
        $("#"+options.type+"next").click(function () {
            options.start = options.start + years;
            options.end = options.end + years;
            $id.children().remove();
            $id.append(datePublicTitle(options));
            years = parseInt(options.end) - parseInt(options.start);
            num = (100 / parseInt(options.number)) + "%";
            var text='<div class="list_ct"><ul>';
			for (var i = 0; i < years + 1; i++) {
                text+= "<li class=\""+options.type+"Date-year\"><a href=\"javascript:void(0);\">" + datePublicYear(parseInt(options.start) + i) + "</a></li>";
            }
			text+='</ul></div>';
			$id.append(text);
            up(options, $id, years);
            next(options, $id, years);
            Bind(options);
        });
    }
    function Bind(options) {
        $("."+options.type+"Date-year").unbind();
        callback = $("."+options.type+"Date-year").bind("click", function () {
            var index = $("."+options.type+"Date-year").index(this);
            result = $("."+options.type+"Date-year").eq(index).text().substr(0,4);
            $("#" + options.valueid + "").val(result);
            $("#" + options.valueid + "id").val(result);
			$("#"+options.nextid).attr('style','display:block;');
			$("#"+options.newid).attr('style','display:none;');
        });
    }
})(jQuery);

(function ($) {
    $.fn.mDate = function (options) {
        var $id = this;
        var defaults = {
            valueid: "value",
            start: 1,
            end: 12,
			nextid:'',
			newid:'',
            number: 5
        }
        var options = $.extend(defaults, options);
        this.children().remove();
        this.append('<div class="title">' + datePublicEsc(datePublicT('date_js_00005', null, 'Month')) + '</div>');
        var num = (100 / parseInt(options.number)) + "%";
		var text='<div class="list_ct"><ul>';
        for (var i = 1; i <=12; i++) {
			if(i<10){
				i='0'+i;
			}
            text += "<li class=\""+options.type+"Date-month\"><a href=\"javascript:void(0);\">" + datePublicMonth(i) + "</a></li>";
        }
		text+='</ul></div>';
		this.append(text);
        Bind(options);
    };
    function Bind(options) {
        $("."+options.type+"Date-month").unbind();
        callback = $("."+options.type+"Date-month").bind("click", function () {
            var index = $("."+options.type+"Date-month").index(this);
            result = $("."+options.type+"Date-month").eq(index).text().substr(0,2);
            $("#" + options.valueid + "").val(result);
            $("#" + options.valueid + "id").val(result);
			$("#"+options.nextid).html('<script>var year=$("#'+options.befvalue+'").val();var month=$("#'+options.valueid+'").val();$("#'+options.nextid+'").dDate({valueid: "'+options.nextvalue+'",type:"'+options.type+'",year:year,month:month,newid:"'+options.nextid+'",number: 5});</script>');
			$("#"+options.nextid).show();
			$("#"+options.newid).attr('style','display:none;'); 
        });
    }
})(jQuery);

(function ($) {
    $.fn.dDate = function (options) {
        var $id = this;
        var defaults = {
            valueid: "value",
			nextid:'',
			newid:'',
            number: 5
        }
        var options = $.extend(defaults, options);
        this.children().remove();
        this.append('<div class="title">' + datePublicEsc(datePublicT('date_js_00007', null, 'Day')) + '</div>');
		var yearval=parseInt(options.year);
		var monthval=parseInt(options.month);
        var num = (100 / parseInt(options.number)) + "%";
		var text='<div class="list_ct"><ul>';		
		if(monthval==1||monthval==3||monthval==5||monthval==7||monthval==8||monthval==10||monthval==12){
			for(var i = 1; i <=31; i++){
				if(i<10){
					i='0'+i;
				}
				text += "<li class=\""+options.type+"Date-day\"><a href=\"javascript:void(0);\">" + datePublicDay(i) + "</a></li>";
			}
		}else if(monthval==4||monthval==6||monthval==9||monthval==11){
			for(var i = 1; i <=30; i++){
				if(i<10){
					i='0'+i;
				}
				text += "<li class=\""+options.type+"Date-day\"><a href=\"javascript:void(0);\">" + datePublicDay(i) + "</a></li>";
			}
		}else if(monthval==2){
			if(yearval%4==0&&(yearval%100!=0||yearval%400==0)){
				for(var i = 1; i <=29; i++){
					if(i<10){
						i='0'+i;
					}
					text += "<li class=\""+options.type+"Date-day\"><a href=\"javascript:void(0);\">" + datePublicDay(i) + "</a></li>";
				}
			}else{
				for(var i = 1; i <=28; i++){
					if(i<10){
						i='0'+i;
					}
					text += "<li class=\""+options.type+"Date-day\"><a href=\"javascript:void(0);\">" + datePublicDay(i) + "</a></li>";
				}
			}
		}
		text+='</ul></div>';
		this.append(text);
        Bind(options);
    };
    function Bind(options) {
        $("."+options.type+"Date-day").unbind();
        callback = $("."+options.type+"Date-day").bind("click", function () {
            var index = $("."+options.type+"Date-day").index(this);
            result = $("."+options.type+"Date-day").eq(index).text().substr(0,2);
            $("#" + options.valueid + "").val(result);
            $("#" + options.valueid + "id").val(result);			
			$("#"+options.newid).attr('style','display:none;'); 
        });
    }
})(jQuery);