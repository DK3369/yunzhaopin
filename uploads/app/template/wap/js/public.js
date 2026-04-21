$.ajaxSetup({
	xhrFields: {
		withCredentials: true
	},
	crossDomain:true 
});
// 获取表单提交值
function getFormValue(formid) {
	var itemForm = $("#" + formid).eq(0);

	var nameIndex = {}, //数组 name 索引
		field = {},
		fieldElem = itemForm.find('input,select,textarea') //获取所有表单域
	
	fieldElem.each(function(_, item) {
		var init_name; //初始 name

		item.name = (item.name || '').replace(/^\s*|\s*&/, '');
		if (!item.name) return;
		//用于支持数组 name
		if (/^.*\[\]$/.test(item.name)) {
			var key = item.name.match(/^(.*)\[\]$/g)[0];
			nameIndex[key] = nameIndex[key] | 0;
			init_name = item.name.replace(/^(.*)\[\]$/, '$1[' + (nameIndex[key]++) + ']');
		}

		if (/^checkbox|radio$/.test(item.type) && !item.checked) return; //复选框和单选框未选中，不记录字段     
		field[init_name || item.name] = item.value;
	});
	
	return field;
}

/**
 * @desc 邮箱格式验证
 */
function check_email(strEmail) {
	var emailReg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	if (emailReg.test(strEmail))
		return true;
	else
		return false;
}
/**
 * @desc 手机号码验证
 */
function isjsMobile(obj){
	var reg= /^[1][3456789]\d{9}$/;   
    if (obj.length != 11) return false;
    else if (!reg.test(obj)) return false;
    else if (isNaN(obj)) return false;
    else return true;
}
/**
 * @desc 电话验证
 */
function isjsTell(str) {
	//var result = str.match(/^((0\d{2,3})-)(\d{7,8})(-(\d{3,}))?$/);
	var result = str.match(/^[0-9-]+?$/);
	if (result == null) return false;
    return true;
}
function sign(){
	showLoading();
	$.post(wapurl+"index.php?c=ajax&a=sign",{rand:Math.random()},function(res){
		hideLoading();
		if(res.type=="-2"){
			showToast('操作失败', 2);return false;
		}else{ 
			showToast('签到成功+'+res.integral,2,function(){window.location.reload(true)});return false;
		}
	}, 'json');
}

function signok(){
    showToast('今日已签到，请明天再来', 2);return false;
}

function navigateTo(url){
	window.location.href = url;
}
function checkCode(id){
	if(document.getElementById(id)){
		document.getElementById(id).src=wapurl+"/authcode.inc.php?"+Math.random();
	}
}

function goBack(){
    if (navigator.userAgent.indexOf('Firefox') >= 0 ||
		navigator.userAgent.indexOf('Opera') >= 0 ||
		navigator.userAgent.indexOf('Safari') >= 0 ||
		navigator.userAgent.indexOf('Chrome') >= 0 ||
		navigator.userAgent.indexOf('WebKit') >= 0){

		if(window.history.length > 1){
			window.history.go( -1 );
		}else{
			location.href = wapurl;
		}
	}else{ //未知的浏览器
		window.history.go( -1 );
	}
}
/**
 * @desc 
 * @param imgObj
 * @param imgSrc
 * @param maxErrorNum
 * @returns
 */
function showImgDelay(imgObj,imgSrc,maxErrorNum){  
	if(maxErrorNum>0){
		imgObj.onerror=function(){
			
			showImgDelay(imgObj,imgSrc,maxErrorNum-1);
		};
		
		setTimeout(function(){
			imgObj.src	=	imgSrc;
		},500);
		
		maxErrorNum	=	parseInt(maxErrorNum) - parseInt(1);
    }
}
// 请先登录
function pleaselogin(msg,url){
	showConfirm(msg, function(){
		window.location.href = url;
	},'取消', '登录');
}
// 统一删除函数
function vant_del(msg = '', url){
	showConfirm(msg, function(){
		showLoading();
		$.get(url, function(res){
			if(!res.url){
				// 不需要跳转，返回上一页
				showToast(res.msg, 2, function(){
					window.history.go(-1);
				});
			}else {
				if(res.url == '1'){
					// 页面刷新
					showToast(res.msg, 2, function(){
						window.location.reload();
					});
				}else{
					// 跳转到需要的地址
					showToast(res.msg, 2, function(){
						window.location.href = res.url;
					});
				}
			}
		},'json');
	});
}
//退出
function islogout(url,msg) {	
	
	showConfirm('确认退出吗？',function(){
		window.localStorage.setItem("socketState", "2");
		window.location.href = url;
	});
}
$(document).ready(function () {
    $(".repeat_list").click(function(){
        
		if($(this).attr("eid")){
			var eid = $(this).attr("eid");
			$("#eid").val($(this).attr("eid"));
		}
        if($(this).attr("uid")){
			var uid = $(this).attr("uid");
			$("#uid").val($(this).attr("uid"));
		}
        if($(this).attr("r_name")){
			var r_name = $(this).attr("r_name");
			$("#r_name").val($(this).attr("r_name"));
		}
        $.post(wapurl+"/index.php?c=resume&a=report",{eid:eid,ruid:uid},function(data){
        	if(data==3){
                showToast('查看联系方式之后才可以举报', 2);return false;  
            }else if(data==1){
                showToast('您已经举报过简历！', 2);return false;  
            }else if(data==2){
                showConfirm('你确定举报这份简历？',function(){
                	location.href = wapurl+"index.php?c=reportlist&uid="+uid+"&eid="+eid+"&r_name="+r_name;
                })
				
            }
        });
	});
})
var jl_flag;
// 节流函数：在一定时间内，只能触发一次
function throttle(func, wait = 1000){
	if (!jl_flag) {
		jl_flag = true;
		// 如果是立即执行，则在wait毫秒内开始时执行
		typeof func === 'function' && func();
		timer = setTimeout(() => {
			jl_flag = false;
		}, wait);
	}

}

function footernav(type){
	var display =$("."+type).css('display');
	$("#footerjob").hide(); 
	if(display=='none'){ 
		$("."+type).show();
	}else{
		$("."+type).hide();
	}
}

// 问答关注功能
function attention(id,type,url){
	showLoading()
	$.post(url,{id:id,type:type},function(data){
		hideLoading();
		var data=eval('('+data+')');  
		if(type==1){var msg='关注';}else{var msg='+  关注';} 
		if(data.st==8){
			showToast(data.msg, 2);return false;	
		}else{		
			$(".num"+id).html(data.url+"人关注");
			$(".index_num"+id).html(data.url);
			
			if(data.tm==1){				
				$(".q"+id+">a").attr("class","watch_qxgz");
				$(".q"+id+">a").html("取消关注");
				showToast("关注成功！", 2,function(){location.reload(true);});return false; 
			}else{
				$(".q"+id+">a").attr("class","watch_gz");
				$(".q"+id+">a").html(msg);
				showToast("取消成功！", 2,function(){location.reload(true);});return false; 
			}				
		} 
	});
}

// 向左滚动
function marquee_l(time,id){
	$(function(){
		var _wrap=$(id);
		var _interval=time;
		var _moving;
		_wrap.hover(function(){
			clearInterval(_moving);
		},function(){
			_moving=setInterval(function(){
			var _field=_wrap.find('li:first');
			var _h=_field.height();
			_field.animate({marginLeft:-_h+'px'},800,function(){
			_field.css('marginLeft',10).appendTo(_wrap);
			})
		},_interval)
		}).trigger('mouseleave');
	});
}
// 向上滚动
function marquee(time,id){
	$(function(){
		var _wrap=$(id);
		var _interval=time;
		var _moving;
		_wrap.hover(function(){
			clearInterval(_moving);
		},function(){
			_moving=setInterval(function(){
			var _field=_wrap.find('li:first');
			var _h=_field.height();
			_field.animate({marginTop:-_h+'px'},800,function(){
			_field.css('marginTop',0).appendTo(_wrap);
			})
		},_interval)
		}).trigger('mouseleave');
	});
}


function savepwd(){
	var password=$("#password").val();
	var passwordnew=$.trim($("#passwordnew").val());
	var passwordconfirm=$.trim($("#passwordconfirm").val());
	if(password<6){
		showToast('原密码不正确！',2);return false;
	}
	if(passwordnew.length<6){
		showToast('新密码长度必须大于等于6！',2);return false;
	}
	if(password == passwordnew){
		showToast('请输入新密码不同于原密码！', 2);return false;
	}
	if(passwordnew != passwordconfirm || passwordconfirm.length<6){
		showToast('两次输入密码不一致！', 2);return false;
	}
	showLoading()
	$.post(wapurl+"?c=ajax&a=setpwd",{password:password,passwordnew:passwordnew,passwordconfirm:passwordconfirm},function(data){
		hideLoading(); 
		var data=eval("("+data+")");
		if(data.type==9){
			window.localStorage.setItem("socketState", "2");
			showToast(data.msg,2,function(){
				window.location.href=wapurl+"index.php?c=login";
			});
		}else{
			showToast(data.msg,2);return false;
		}
	});
}
/****
* name:localStorage的key
* data:localStorage的Value
* expire:localStorage的过期时间,默认是1天后过期
****/
//设置缓存
function localStorageSet(name, data, expire = 24){
	expire = new Date().getTime() + expire * 60 * 60 * 1000;
	var obj = {
			data,
			expire
		};
	window.localStorage.setItem(name, JSON.stringify(obj));
}
//读取缓存
function localStorageGet(name){
	var storage = window.localStorage.getItem(name);
	var result = null;
	if (storage) {
		var obj = JSON.parse(storage);
		var time = new Date().getTime();
		if (time < obj.expire) {
			result = obj.data;
		} else {
			 window.localStorage.removeItem(name);
		}
	}
	return result;
}
// 判断arr是否为一个数组，返回一个bool值
function isArray (arr) {
    return Object.prototype.toString.call(arr) === '[object Array]';
}
// 深度克隆
function deepClone (obj) {
	// 对常见的“非”值，直接返回原来值
	if([null, undefined, NaN, false].includes(obj)) return obj;
    if(typeof obj !== "object" && typeof obj !== 'function') {
		//原始类型直接返回
        return obj;
    }
    var o = isArray(obj) ? [] : {};
    for(let i in obj) {
        if(obj.hasOwnProperty(i)){
            o[i] = typeof obj[i] === "object" ? deepClone(obj[i]) : obj[i];
        }
    }
    return o;
}

var timeout = null;
/**
 * 防抖原理：一定时间内，只有最后一次操作，再过wait毫秒后才执行函数
 *
 * @param {Function} func 要执行的回调函数
 * @param {Number} wait 延时的时间
 * @return null
 */
function debounce(func, wait = 500) {
	// 清除定时器
	if (timeout !== null) clearTimeout(timeout);
	timeout = setTimeout(function() {
		typeof func === 'function' && func();
	}, wait);
}


function atn(id,url){// 关注企业
	if(id){
		showLoading()
		$.post(url,{id:id},function(data){
			hideLoading(); 
			var data=eval('('+data+')');
			if(data.errcode==1){
                showToast(data.msg,2,function(){window.location.reload(true);});return false; 
              
			}else{
				showToast(data.msg);return false;
			}
		});
	}
}

function checkshowjob(type, id, operation_type) {  
    window.show_scrolltop = document.body.scrollTop;
    document.body.scrollTop = 0;
    
    if(type=='once'||type=='tiny'){
        if(type == 'once'){
            $('#once_password').val('');
            yunvue.$data.onceid = id;
        }else if (type=='tiny'){
            $('#tiny_password').val('');
            yunvue.$data.tinyid = id;
        }
        $('#checkcode').val('');
        checkCode('vcode_img');
        yunvue.$data.operation_type = operation_type;
        yunvue.$data.jobBox = true;

    }else{
        $("#"+type+"list").show();
        checkhide('info'); 
    }
}

function checkOncePassword(id,img){
    if($("#once_password").val()==''){
        showToast('请输入密码');
        return;
    }

    var operation_type=$("#operation_type").val();
    var checkcode = $("#checkcode").val();
    showLoading();
    $.post(wapurl + "/index.php?c=once&a=ajax", { id: id, password: $("#once_password").val(), operation_type: operation_type ,checkcode:checkcode }, function (data) {
        hideLoading(); 
        var data = eval('(' + data + ')');
        if(data.type == 1 || data.type == 2|| data.type ==5) {
            showToast(data.msg);
            checkCode(img);
            return false;
        }  else if(data.type == 3) {
            showToast(data.msg, 2, function() {
                window.location.reload();
            });
        } else if(data.type == 4) {
            showToast(data.msg, 2, function() {
                location.href = wapurl + 'index.php?c=once';
            });
        } else {
            location.href =data.url;
        }
    });
}

function checkCode(id){
    if(document.getElementById(id)){
        document.getElementById(id).src=wapurl+"/authcode.inc.php?"+Math.random();
    }
}

function checkTinyPassword(id, img){
	if($("#tiny_password").val()==''){
		showToast('请输入密码');
		return;
	}
	var operation_type = $("#operation_type").val();
	var checkcode = $("#checkcode").val();
	showLoading();
	$.post(wapurl + "/index.php?c=tiny&a=ajax", { id: id, password: $("#tiny_password").val(), operation_type: operation_type ,checkcode:checkcode}, function (data) {
		hideLoading(); 
		var data = eval('(' + data + ')');
		if(data.type == 1 || data.type == 2) {
			showToast(data.msg);
			checkCode(img);
			return false;
		}  else if(data.type == 3) {
			showToast(data.msg, 2, function() {
				window.location.reload();
			});
		} else if(data.type == 4) {
			showToast(data.msg, 2, function() {
				location.href = wapurl + 'index.php?c=tiny';
			});
		} else {
			location.href =data.url;
		}
	});
}
function form2json(target_form) {
    var json_form = '';
    $(target_form).find('input,select,textarea').each(function () {
        if($(this).attr('type')=='radio'){
			if ($(this).attr('name')&&$(this).attr('checked')=='checked') {
				json_form += ',' + $(this).attr('name') + ':"' + $(this).val().replace(/[\r\n]+/g, '\\n')+'"';
			}
		}else{
			if ($(this).attr('name')) {
				json_form += ',' + $(this).attr('name') + ':"' + $(this).val().replace(/[\r\n]+/g, '\\n')+'"';
			}
		}
    });
    return eval('({' + json_form.substring(1) + '})');
}
function formfile2json(target_form) {
    var json_form = '';
    var formData = new FormData(target_form);
    $(target_form).find('input,select').each(function () {
        if ($(this).attr('name')) {
            if ($(this)[0].type == 'file') {
                formData.append('file', $('input[type=file]', target_form).get(0).files[0]);
            } else {
                formData.append($(this).attr('name'), $(this).val());
            }
        }
    });
   
    return formData;
}

function form2string(target_form) {
    var json_form = '';
    $(target_form).find('input,select').each(function () {
        if ($(this).attr('name')) {
            json_form += '&' + $(this).attr('name') + '=' + $(this).val();
        }
    });
    return json_form;
}
function post2ajax(target_form) {
	showLoading();
    if ($('input[type=file]', target_form).length > 0) {
        $.ajax({
            url: $(target_form).attr('action'),
            data: formfile2json(target_form),
            processData: false,
            type: 'POST',
			async: false,  
			cache: false,
			contentType: false,
            success: function (data) {
				hideLoading();
                var json_data = eval('(' + data + ')');
                if (json_data.msg) {
					
					if($("#bind-captcha").length>0){
						$("#popup-submit").trigger("click");
					}
					if (json_data.st==10) {
					    checkCode('vcode_img'); 
					}
					showToast(json_data.msg, json_data.tm, function () { if (json_data.url) { location.href = json_data.url; } });
					return false; 
                } else if (json_data.url) {
                    location.href = json_data.url;
					return false; 
                }
            }
        });
    } else {
        if ($(target_form).attr('action') == 'get') {
            $.get($(target_form).attr('action') + form2string(target_form), function (data) {
				hideLoading();
                var json_data = eval('(' + data + ')');
                if (json_data.msg) {
					if($("#bind-captcha").length>0){
						$("#popup-submit").trigger("click");
					}
                    showToast(json_data.msg, json_data.tm, function () { if (json_data.url) { location.href = json_data.url; } });
					return false; 
				} else if (json_data.url) {
                    location.href = json_data.url;
					return false; 
                }
            });
        } else {
            $.post($(target_form).attr('action'), form2json(target_form), function (data) {
				hideLoading();
                var json_data = eval('(' + data + ')');
                if (json_data.msg) {
					if($("#bind-captcha").length>0){
						$("#popup-submit").trigger("click");
					}
					if(json_data.msg.indexOf('script')>0){
						$('#uclogin').html(json_data.msg);
						json_data.msg = '登录成功';
					}
					showToast(json_data.msg, json_data.tm, function () {
						if (json_data.url) {
							location.href = json_data.url; 
						}  
					});
					if (json_data.st==8) {
					    checkCode('vcode_img'); 
					}
					return false; 
                } else if (json_data.url) {
                	// 处理缓存，返回登陆页面后刷新
                	window.sessionStorage.setItem("needRefresh", true);
                    location.href = json_data.url;
					return false; 
                }
            });
        }
    }
    return false;
} 

function exitsid(id) {
	if(document.getElementById(id)) {
		return true;
	} else {
		return false;
	}
}

function get_comment(aid,show,url){ 
	$(".pl_menu").hide();
	var style=$(".review"+aid).css("display");
	var info=$(".review"+aid+" ul").html();
	if(style=="none"||show>0){ 
		if((info==''||info==null)||show>0){
			showLoading();
			$.post(url,{aid:aid},function(data){
				hideLoading(); 
				var html='';  
				var datas = Array();
				data = data.replace(/\s+/g,"[[space]]");// eval的字符串中有空格会出错
				datas = eval("("+data+")");
				$.each(datas,function(key,val){
					html+="<li>"+
							"<div class=\"menu_p1_tx\"><img src=\""+val.pic.replace(/\[\[space\]\]/g,' ')+"\" onerror=\"showImgDelay(this,'"+val.errorpic.replace(/\[\[space\]\]/g,' ')+"',2);\"/></div>"+
							"<div class=\"menu_right\">"+
								"<div class=\"menu_rig_h2\">"+
									"<span class=\"menu_user\"><a href=\""+val.url.replace(/\[\[space\]\]/g,' ')+"\">"+val.nickname.replace(/\[\[space\]\]/g,' ')+"</a>：</span>"+
									"<span class=\"menu_mes\">"+val.content.replace(/\[\[space\]\]/g,' ')+"</span>"+
								"</div>"+ 
								"<div class=\"menu_date\">"+
									"<span>"+val.date.replace(/\[\[space\]\]/g,' ')+"</span>"+
								"</div>"+
							"</div>"+ 
						"</div>"+
						"<div class=\"clear\"></div>"+
					"</li>"; 
					$(".review"+aid+" ul").html(html); 
				});	 
			});
		}
		$(".review"+aid).show();
	}else{
		$(".review"+aid).hide();
	} 
} 

function for_comment(aid,qid,url,comurl){
	var content=$.trim($("#comment_"+aid).val()); 
	if(content=="" || content=="undefined"){
		showToast('评论内容不能为空！');return false; 
	}else{
		showLoading();
		$.post(url,{aid:aid,qid:qid,content:content},function(msg){
			hideLoading(); 
			if(msg=='1'){
				$("#comment_"+aid).val("");
				var com_num=$("#com_num_"+aid).html();  
				com_num=parseInt(com_num)+parseInt(1);
				$("#com_num_"+aid).html(com_num); 
				get_comment(aid,'1',comurl);
			}else if(msg=='0'){
				showToast('评论失败！');return false; 
			}else if(msg=='no_login'){ 
				showToast('请先登录！');return false; 
			}else{
				showToast(msg);return false; 
			}
		});
	}
} 

function yunfootClose(){
	footerVue.$data.yunfoot = !footerVue.$data.yunfoot;
}

function isChinaName(name) {
    var pattern = /^[\u4E00-\u9FA5]{2,6}$/;
    return pattern.test(name);
}