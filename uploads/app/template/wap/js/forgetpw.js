var Timer;
var smsTimer_time = 90; // Countdown seconds
var smsTimer_flag = 90; // Countdown seconds
var smsTime_speed = 1000; // Interval 1 second
var _fpw = window.FPW_I18N || {};

// Send SMS verification code
function send_msg() {
	var moblie = $('#moblie').val();
	var code;
	var verify_token,verify_str;
	if(moblie == "") {
		return showToast(_fpw.enterMobile || '');
	} else if(isjsMobile(moblie) == false) {
		return showToast(_fpw.mobileFormatError || '');
	}
	var codesear = new RegExp(_fpw.codeWebRecover || '');
	if (codesear.test(code_web)) {
		if (code_kind == 1) {
			code = $.trim($("#authcode").val());
			if (!code) {
				showToast(_fpw.enterImageCode || '');
				return false;
			}
		} else if (code_kind > 2) {
			// Change verification type to SMS
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
					} else if(code_kind > 2) {
						$("#popup-submit").trigger("click");
					}
				}
			});
		}, 'json');
	} else {
		return showToast(_fpw.doNotResend || '');
	}
}

function send_email() {
	var email = $('#email').val();
	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	if(email == "") {
		return showToast(_fpw.enterEmail || '');
	} else if(!myreg.test(email)) {
		return showToast(_fpw.emailFormatError || '');
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
		return showToast(_fpw.doNotResend || '');
	}
}

function exitsid(id) {
	if(document.getElementById(id)) {
		return true;
	} else {
		return false;
	}
}
// Countdown
function smsTimer(obj) {
	if(smsTimer_flag > 0) {
		$(obj).html((_fpw.resendPrefix || '') + smsTimer_flag + 's)');
		$(obj).attr({
			'style': 'background:#909394;'
		});
		smsTimer_flag--;
	} else {
		$(obj).html(_fpw.resend || '');
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
		return showToast(_fpw.selectRecoverMethod || '');
	}
	if(sendtype == 'moblie') {
		if(moblie == "") {
			return showToast(_fpw.enterMobile || '');
		} else if(isjsMobile(moblie) == false) {
			return showToast(_fpw.mobileFormatError || '');
		}
		if(moblie_vcode == "") {
			return showToast(_fpw.enterSmsCode || '');
		}
		code = moblie_vcode;
	} else if(sendtype == 'email') {
		var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
		if(email == "") {
			return showToast(_fpw.enterEmail || '');
		} else if(!myreg.test(email)) {
			return showToast(_fpw.emailFormatError || '');
		}
		if(email_vcode == "") {
			return showToast(_fpw.enterEmailCode || '');
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
		showToast(_fpw.verifyBeforeChange || '', _fpw.tip || '', _fpw.confirm || '', function() {
			location.reload(true);
		});
		return false;
	} else if(pwd.length < 6) {
		return showToast(_fpw.passwordMinLength || '');
	} else if(pwd != pwdconfirm) {
		return showToast(_fpw.passwordMismatch || '');
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
		return showToast(_fpw.enterContactName || '');
	}
	if(linkphone == '') {
		return showToast(_fpw.enterContactPhone || '');
	} else if(isjsMobile(linkphone) == false && isjsTell(linkphone) == false) {
		return showToast(_fpw.contactPhoneFormatError || '');
	}
	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	if(linkemail == '') {
		return showToast(_fpw.enterContactEmail || '');
	} else if(!myreg.test(linkemail)) {
		return showToast(_fpw.emailFormatErrorEx || '');
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
			showToast(_fpw.systemBusy || '', 2, function() {
				location.reload(true);
			})
		}
	}, 'json');
}
