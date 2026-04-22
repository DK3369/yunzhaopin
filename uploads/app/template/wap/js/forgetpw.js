var sendtypeData = [{
	type: 'moblie',
	text: '通过手机找回密码'
}, {
	type: 'email',
	text: '通过邮箱找回密码'
}, {
	type: 'shensu',
	text: '账号申诉'
}];

var Timer;
var smsTimer_time = 90; //倒数 90
var smsTimer_flag = 90; //倒数 90
var smsTime_speed = 1000; //速度 1秒钟
//发送手机短信
function send_msg() {
	var moblie = $('#moblie').val();
	var code;
	var verify_token,verify_str;
	if(moblie == "") {
		return showToast('请输入手机号');
	} else if(isjsMobile(moblie) == false) {
		return showToast('手机号格式错误');
	}
	var codesear = new RegExp('找回密码');
	if (codesear.test(code_web)) {
		if (code_kind == 1) {
			code = $.trim($("#authcode").val());
			if (!code) {
				showToast('请填写图片验证码！');
				return false;
			}
		} else if (code_kind > 2) {
			// 验证类型改成短信
			$('#bind-captcha').attr('data-id', 'send_msg_tip');
			$('#bind-captcha').attr('data-type', 'click');
			verify_token = $('input[name="verify_token"]').val();
			if (verify_token == '') {
				if (code_kind == 6) {
					$("#bind-captcha").trigger("click");
				} else {
					$("#bind-submit").trigger("click");
				}
				return false;
			}
			verify_str = $('input[name="verify_str"]').val();
		}
	}
	if(smsTimer_time == smsTimer_flag) {
		Timer = setInterval("smsTimer($('#send_msg_tip'))", smsTime_speed);
		$.post(wapurl + "?c=forgetpw&a=sendcode", {
			sendtype: 'moblie',
			moblie: moblie,
			authcode: code,
			verify_token: verify_token,
			verify_str: verify_str
		}, function(data) {
			if(data.error != 1) {
				clearInterval(Timer);
			}
			showToast(data.msg, 2, function(){
				if(data.error != 1){
					if(code_kind == 1) {
						checkCode('vcode_img');
					} else if(code_kind >2) {
						$("#popup-submit").trigger("click");
					}
				}
			});
		}, 'json');
	} else {
		return showToast('请勿重复发送！');
	}
}

function send_email() {
	var email = $('#email').val();
	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	if(email == "") {
		return showToast('请输入邮箱');
	} else if(!myreg.test(email)) {
		return showToast('邮箱格式错误');
	}
	if(smsTimer_time == smsTimer_flag) {
		Timer = setInterval("smsTimer($('#send_email_tip'))", smsTime_speed);
		$.post(wapurl + "?c=forgetpw&a=sendcode", {
			sendtype: 'email',
			email: email
		}, function(data) {
			if(data.error != 1) {
				clearInterval(Timer);
				return showToast(data.msg);
			}
		}, 'json');
	} else {
		return showToast('请勿重复发送！');
	}
}

function exitsid(id) {
	if(document.getElementById(id)) {
		return true;
	} else {
		return false;
	}
}
//倒计时
function smsTimer(obj) {
	if(smsTimer_flag > 0) {
		$(obj).html('重新发送(' + smsTimer_flag + 's)');
		$(obj).attr({
			'style': 'background:#909394;'
		});
		smsTimer_flag--;
	} else {
		$(obj).html('重新发送');
		$(obj).removeAttr('style');
		smsTimer_flag = smsTimer_time;
		clearInterval(Timer);
	}
}

function forgetPwNext() {
	var sendtype = $("#sendtype").val(),
		moblie = $("#moblie").val(),
		moblie_vcode = $("#moblie_vcode").val(),
		email = $("#email").val(),
		email_vcode = $("#email_vcode").val(),
		code = '';
	if(sendtype != "email" && sendtype != "moblie" && sendtype != "shensu") {
		return showToast("请选择找回密码方式");
	}
	if(sendtype == 'moblie') {
		if(moblie == "") {
			return showToast('请输入手机号');
		} else if(isjsMobile(moblie) == false) {
			return showToast('手机号格式错误');
		}
		if(moblie_vcode == "") {
			return showToast('请输入短信验证码');
		}
		code = moblie_vcode;
	} else if(sendtype == 'email') {
		var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
		if(email == "") {
			return showToast('请输入邮箱');
		} else if(!myreg.test(email)) {
			return showToast('邮箱格式错误');
		}
		if(email_vcode == "") {
			return showToast('请输入邮箱验证码');
		}
		code = email_vcode;
	}
	$.post(wapurl + "?c=forgetpw&a=checksendcode", {
		sendtype: sendtype,
		moblie: moblie,
		email: email,
		code: code
	}, function(data) {
		if(data.error == 0) {
			$("#path1").attr('class', 'currents_er');
			$("#path2").attr('class', 'currents');
			$("#backpicker").hide();
			$("#moblieshow").hide();
			$("#emailshow").hide();
			$("#shensushow").hide();
			$("#resetpw").show();
			$("#fuid").val(data.uid);
			$("#username").val(data.username);

			$("#fmobile").val(moblie);
			$("#femail").val(email);
			$("#fcode").val(code);
		} else {
			return showToast(data.msg);
		}
	}, 'json');
}

function editpw() {
	var uid = $("#fuid").val(),
		username = $("#username").val(),
		mobile = $.trim($("#fmobile").val()),
		email = $.trim($("#femail").val()),
		code = $("#fcode").val(),

		pwd = $.trim($("#password").val()),
		pwdconfirm = $.trim($("#passwordconfirm").val());
	if($.trim(uid) == "" || $.trim(username) == "") {
		showToast('请先验证后再修改密码', '提示', '确定', function() {
			location.reload(true);
		});
		return false;
	} else if(pwd.length < 6) {
		return showToast('密码长度必须大于等于6');
	} else if(pwd != pwdconfirm) {
		return showToast('两次输入密码不一致');
	} else {
		showLoading()
		$.post(wapurl + "?c=forgetpw&a=editpw", {
			username: username,
			uid: uid,
			mobile: mobile,
			email: email,
			code: code,
			password: pwd,
			passwordconfirm: pwdconfirm
		}, function(data) {
			hideLoading();
			if(data.error == 0) {
				$("#path2").attr('class', 'currents_er');
				$("#path3").attr('class', 'currents');
				$("#resetpw").hide();
				$("#succeed").show();
			} else {
				return showToast(data.msg);
			}
		}, 'json');
	}
}

function checklink(img) {
	var username = $("#username").val();
	var linkman = $("#linkman").val();
	var linkphone = $("#linkphone").val();
	var linkemail = $("#linkemail").val();
	if(linkman == '') {
		return showToast("请填写联系人！");
	}
	if(linkphone == '') {
		return showToast("请填写联系电话！");
	} else if(isjsMobile(linkphone) == false && isjsTell(linkphone) == false) {
		return showToast("联系电话格式错误！");
	}
	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	if(linkemail == '') {
		return showToast("请填写联系邮箱！");
	} else if(!myreg.test(linkemail)) {
		return showToast("邮箱格式错误！");
	}
	showLoading()
	$.post(wapurl + "?c=forgetpw&a=checklink", {
		username: username,
		linkman: linkman,
		linkphone: linkphone,
		linkemail: linkemail,
	}, function(data) {
		hideLoading();
		if(data.error == 0) {
			$("#path1").attr('class', 'currents_er');
			$("#path3").attr('class', 'currents');
			$("#backpicker").hide();
			$("#shensushow").hide();
			$("#finish").show();
		}else if(data.error == 8){
      showToast(data.msg, 2);
      return false;
    } else {
			showToast("系统正忙", 2, function() {
				location.reload(true);
			})
		}
	}, 'json');
}