var throttleFlag;
var _reg = window.REG_I18N || {};

function login(){
	// 节流处理：在一定时间内，只能触发一次
	if (!throttleFlag) {
		throttleFlag = true;
		setTimeout(function(){
			throttleFlag = false;
		}, 1500);
	}else{
		return false;
	}
	var field = getFormValue('login_form');
	if(field.act_login ==0){
		if(field.username == ''){
			return showToast(_reg.enterUsername || '');
		}
		if(field.password == ''){
			return showToast(_reg.enterPassword || '');
		}
	
	}else{
		if(field.moblie == ""){
			return showToast(_reg.enterMobile || '');
		}
		if(field.dynamiccode ==""){
			return showToast(_reg.enterSmsCode || '');
		}
	}
	if(field.xieyicheck !=1){
		return showToast(_reg.agreeLoginProtocol || '');
	}
	var codesear=new RegExp(_reg.codeWebLogin || '');
	if(codesear.test(code_web)){
		if(code_kind==1){
			if(!field.authcode){
				return showToast(_reg.enterAuthCode || '');
			}					
		}else if(code_kind > 2){
			if(field.verify_token ==''){
				if (code_kind == 6) {
                    $("#bind-captcha").trigger("click");
                } else {
                    $("#bind-submit").trigger("click");
                }
				return false; 
			}
		}
	}
	showLoading();
	$.post(wapurl+'index.php?c=login&a=mlogin', field, 
		function(res){
			if(res.msg){
				showToast(res.msg);
				if($("#bind-captcha").length>0){
					$("#popup-submit").trigger("click");
				}
				if(res.msg.indexOf('script')>0){
					$('#uclogin').html(res.msg);
					res.msg = _reg.loginSuccess || '';
				}
				showToast(res.msg, res.tm, function () {
					if (res.url) {
						location.href = res.url; 
					}  
				});
				if (res.st==8) {
				    checkCode('vcode_img'); 
				}
				return false; 
			}else{
				// 登录成功，去掉点击事件，防止重复点击
				$("#login_bth").attr('onclick', '');
				// 处理缓存，返回登陆页面后刷新
	        	window.sessionStorage.setItem("needRefresh", true);
	            location.href = res.url;
				return false; 
			}
	},'json');
}

function checkRegById(id) {

	var obj = $.trim($('#'+id).val());
	if (id == 'u_name'){
		if (obj == ''){
			showToast(_reg.nameRequired || '', 2);
			return false;
		}else if (sy_resumename_num == 1 && !isChinaName(obj)){
			showToast(_reg.nameHanFormat || '');
			return false;
		}
	}else if (id == 'c_name'){
		if (obj == ''){
			if (obj == ''){
				showToast(_reg.companyNameRequired || '', 2);
				return false;
			}
		}else{
			$.post(wapurl + "index.php?c=register&a=checkComName", {c_name: obj}, function (data) {
				var data = eval('(' + data + ')');
				if (data.errcode == 1) {
					return showToast(_reg.companyNameExists || "");
				}
			});
		}
	}else if (id == 'c_link'){
		if (obj == ''){
			showToast(_reg.companyContactRequired || '', 2);
			return false;
		}
	}
}

function checkRegUser(){
	// 节流处理：在一定时间内，只能触发一次
	if (!throttleFlag) {
		throttleFlag = true;
		setTimeout(function(){
			throttleFlag = false;
		}, 1500);
	}else{
		return false;
	}
	
	var field = getFormValue('reg_form');
	var regway = field.regway;
	
	var isRealnameCheck = field.isRealnameCheck;
	
	var authcode;
	
	var verify_token;
	
	if(exitsid("username")) {
		var username = field.username;
		if(field.username == ''){
			return showToast(_reg.usernameRequired || '');return false;
		}else{
			let usernameCheck = true;
			$.ajax({
				async: false,
				url: wapurl+'index.php?c=register&a=ajaxreg',
				type: 'POST',
				data: {username:username},
				dataType: 'json',
				timeout: 30000,
				success: function(res) {
					if(res.errcode==1){
						usernameCheck = false;
						return showToast(_reg.usernameExists || "");
					}else if(res.errcode==2){
						usernameCheck = false;
						return showToast(_reg.usernameSpecialChar || "");
					}else if(res.errcode==3){
						usernameCheck = false;
						return showToast(_reg.usernameBanned || "");
					}else if(res.errcode==4){
						usernameCheck = false;
						return showToast(res.msg);
					}
				},
				error: function(err) {
					console.log(err);
					usernameCheck = false;
				}
			})
			if (!usernameCheck) {
				return false;
			}
		}
	}
	
	if(exitsid("moblie")) {
		var moblie = $("#moblie").val();
		if(moblie == "") {
			return showToast(_reg.enterMobile || "");
			return false;
		} else if(!isjsMobile(moblie)) {
			return showToast(_reg.mobileFormatError || "");
			return false;
		}
		let moblieCheck = true;
		$.ajax({
			async: false,
			url: wapurl+'index.php?c=register&a=regmoblie',
			type: 'POST',
			data: {moblie:moblie},
			timeout: 30000,
			success: function(res) {
				if (res == 2) {
					moblieCheck = false;
					return showToast(_reg.mobileBanned || "");
				}
			},
			error: function(err) {
				console.log(err);
				moblieCheck = false;
			}
		})
		if (!moblieCheck) {
			return false;
		}
	}
	
	if(exitsid("email")) {
		var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
		var email = $("#email").val();
		if(email == "") {
			return showToast(_reg.emailRequired || "");
			return false;
		} else if(!myreg.test(email)) {
			return showToast(_reg.emailFormatError || "");
			return false;
		}
	}
	var password = field.password;
	if(password == "") {
		return showToast(_reg.passwordRequired || "");
		return false;
	} else if(password.length < 6 || password.length > 20) {
		return showToast(_reg.passwordLength || "");
		return false;
	}
	if(exitsid("passconfirm")) {
		var passconfirm = field.passconfirm;
		if(passconfirm == "") {
			return showToast(_reg.confirmPasswordRequired || "");
			return false;
		} else if(password != passconfirm) {
			return showToast(_reg.passwordMismatch || "");
			return false;
		}
	}
	if(exitsid("moblie_code")) {
		if($("#moblie_code").val() == "") {
			return showToast(_reg.smsCodeRequired || '');
			return false;
		}
	}

	if($("#xieyicheck").val() ==0) {
		showToast(_reg.agreeRegisterProtocol || '');
		return false;
	}
	// 有发送短信验证码不需要触发验证
	// 1-实名认证，需要发送短信验证码
	// 2-手机号注册，有极验/顶象验证码
	var noblur = document.getElementById('noblur');
	var regway = $("#regway").val();
	var isRealnameCheck = $("#isRealnameCheck").val();
	// 1-邮箱/3-用户名注册且实名认证，需要发送短信验证码
	if(((regway == 1 || regway == 3) && isRealnameCheck != 1) || (regway == 2 && !noblur)){
		var codesear = new RegExp(_reg.codeWebRegister || '');
		if(codesear.test(code_web)) {
			if(code_kind == 1) {
				authcode = $.trim($("#checkcode").val());
				if(!authcode) {
					return showToast(_reg.imageCodeRequired || '');
					return false;
				}
			} else if(code_kind >2) {
				
				verify_token = $('input[name="verify_token"]').val();
				if(verify_token == '') {
					if (code_kind == 6) {
                        $("#bind-captcha").trigger("click");
					} else {
                        $("#bind-submit").trigger("click");
					}
					return false;
				}
			}
		}
	}
 	if (sy_reg_type == 2){
		if (field.reg_type == 1){

			field.reg_name = field.u_name;
		}else if (field.reg_type == 2){

			field.reg_name = field.c_name;
			field.reg_link = field.c_link;
		}
	}

	showLoading();
	$.post(wapurl+'index.php?c=register', field, 
		function(res){
			hideLoading();
			if(res.msg){
				if($("#bind-captcha").length>0){
					$("#popup-submit").trigger("click");
				}
				showToast(res.msg, res.tm, function () {
					if (res.url) {
						// 处理浏览器历史记录，防止可以返回注册页面
						window.history.replaceState({}, "", res.url);
						window.location.reload();
					}  
				});
				checkCode('vcode_img'); 
				return false;
			}else if (res.url) {
				// 注册成功，去掉点击事件，防止重复点击
				$("#login_bth").attr('onclick', '');
	        	// 处理缓存，返回登陆页面后刷新
	        	window.sessionStorage.setItem("needRefresh", true);
	            window.location.href = res.url;
				return false; 
	        }
	},'json');
	return false;
}

function exitsid(id) {
	if(document.getElementById(id)) {
		return true;
	} else {
		return false;
	}
}
function check_moblie() {
	var noblur = document.getElementById('noblur');
	var regway = $("#regway").val();
	var isRealnameCheck = $("#isRealnameCheck").val();
	
	var moblie = $("#moblie").val();
	if(moblie == "") {
 		$("#moblie_yes").hide();
		showToast(_reg.mobileEmpty || "");
		return false;
	}else if(!isjsMobile(moblie)){
		showToast(_reg.mobileFormatError || "");
		return false;
	}
	
	$.post(wapurl + "index.php?c=register&a=regmoblie", {
		moblie: moblie
	}, function(data) {
		if(data == 0 && moblie != "") {
			$("#moblie").attr('date', '1');
			$("#moblie_yes").show();
		} else {
			
			if(data == 2) {
				showToast(_reg.mobileBanned || "");
			} else {
				$("#zy_mobile").val(moblie);
				var data = eval('(' + data + ')');
				mobileUserd(data);
			}
		}
	});
}
function mobileUserd(data){
	$("#moblie").val("");
	$("#zy_uid").val(data.uid);
	$("#jcbind").css('dispaly',"block");
	yunvue.$data.desctoast = _reg.unbindMobileDesc || '';
	if(data.usertype=='1'){		
		yunvue.$data.zy_type = _reg.mobileRegisteredPersonal || '';		
		if(data.name){			
			yunvue.$data.zy_name=(_reg.personalNamePrefix || '')+data.name.substr(0,1)+"**";
		}
		
	}else if(data.usertype=='2'){
		yunvue.$data.zy_type = _reg.mobileRegisteredCompany || '';		
		if(data.name){			
			yunvue.$data.zy_name=(_reg.companyNamePrefix || '')+data.name;
		}
	}else if(data.usertype=='0'){
		$("#jcbind").css("display","none");
		yunvue.$data.zy_type = _reg.mobileRegistered || '';
		yunvue.$data.zy_name="";
	} 
	yunvue.$data.checkmobileshow = true
	
}
function CheckPW(){
	yunvue.$data.checkmobileshow = false;
	yunvue.$data.checkPWshow = true;
}
function check_password() {
	var password = $("#password").val();
	if(password == "") {
		return showToast(_reg.passwordRequired || '');
	} else {
		$.post(wapurl + "index.php?c=register&a=ajaxreg",{password:password},function(data){

	      	var data = eval('(' + data + ')');

	        if(data.errcode==4){
				showToast(data.msg);return false;
			}else{
	  			$("#password_yes").show();
	        }
	    });
		
	}
}
function check_username() {
	var username = $("#username").val();

	var reg = new RegExp('！',"g");
	var username = username.replace( reg , '!' );
	$("#username").val(username);
		
	if(username == "") {
		return showToast(_reg.usernameRequired || "");
	} else {
      $.post(wapurl + "index.php?c=register&a=ajaxreg",{username:username},function(data){
      	var data = eval('(' + data + ')');
        if(data.errcode==1){
			return showToast(_reg.usernameExists || "");
		}else if(data.errcode==2){
			return showToast(_reg.usernameSpecialChar || "");
		}else if(data.errcode==3){
			return showToast(_reg.usernameBanned || "");
		}else if(data.errcode==4){
			return showToast(data.msg);
		}else{
  			$("#username_yes").show();
        }
      });
	
	}
}
function check_email() {
	
	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	var email = $("#email").val();
	if(email == "") {
		$("#email_yes").hide();
		showToast(_reg.emailRequired || "");
		return false;
	}else if(!myreg.test(email)) {
		showToast(_reg.emailFormatError || "");
		return false;
	}
	$.post(wapurl + "index.php?c=register&a=regemail", {
		email: email
	}, function(data) {
		if(data == 0 && email != "") {
			$("#email_yes").show();
		} else {
			 
			var data = eval('(' + data + ')');
			$("#email").val("");
			$("#zy_uid").val(data.uid);
			$("#zy_email").val(email);
			$("#jcbind").css('dispaly',"block");
			yunvue.$data.desctoast = _reg.unbindEmailDesc || '';
			if(data.usertype=='1'){
				yunvue.$data.zy_type = _reg.emailRegisteredPersonal || '';
				if(data.name){
					yunvue.$data.zy_name=(_reg.personalNamePrefix || '')+data.name.substr(0,1)+"**";
				}
				
			}else if(data.usertype=='2'){				
				yunvue.$data.zy_type = _reg.emailRegisteredCompany || '';
				if(data.name){
					yunvue.$data.zy_name=(_reg.companyNamePrefix || '')+data.name;
				}
			}else if(data.usertype=='0'){
				$("#jcbind").css("display","none");
				yunvue.$data.zy_type = _reg.emailRegistered || '';
				yunvue.$data.zy_name="";
			} 
			
			yunvue.$data.checkmobileshow = true
		}
	});
}
function checkCode(id){
	if(document.getElementById(id)){
		document.getElementById(id).src=wapurl+"/authcode.inc.php?"+Math.random();
	}
}
function check_code() {
	var checkcode = $("#checkcode").val();
	if(checkcode == "") {
		$("#checkcode_yes").hide();
	} else {
		$("#checkcode_yes").show();
	}
}
function sendmsg(img) {
	var send = $("#send").val();
	var moblie = $("#moblie").val();
	var code;
	
	var verify_token;
	var verify_str;
	var codesear = new RegExp(_reg.codeWebRegister || '');
	if(moblie == "") {
		showToast(_reg.enterMobile || "");
		return false;
	}else if(!isjsMobile(moblie)){
		showToast(_reg.mobileFormatError || "");
		return false;
	}
	if(send > 0) {
		showToast(_reg.doNotResendFrequently || '');
		return false;
	}
	if(codesear.test(code_web)) {
		if(code_kind == 1) {
			code = $.trim($("#checkcode").val());
			if(!code) {
				showToast(_reg.enterImageCode || '');
				return false;
			}
		} else if(code_kind >2) {
			verify_token = $('input[name="verify_token"]').val();
			if(verify_token == '') {
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
	var noblur;
	var regway = $("#regway").val();
	var isRealnameCheck = $("#isRealnameCheck").val();
	if((regway == 1 || regway == 3) && isRealnameCheck == 1){
		noblur = 1;
	}else if(regway == 2){
		noblur = $("#noblur").val()
	}
	
	showLoading();
	$.post(wapurl + "/index.php?c=ajax&a=regcode", {
		moblie: moblie,
		code: code,
		
		verify_token:verify_token,
		verify_str:verify_str,
		noblur: noblur
	}, function(data) {
		hideLoading();
		if(data){
			$("#zy_mobile").val(moblie);
			var res = JSON.parse(data);
			if(res.errcode && noblur){
				mobileUserd(res.data);
			}else{
				showToast(res.msg);
				if(res.error == 1){
					sendtime("121");
				}else if(res.error == 106){
					checkCode(img);
				}else if(res.error == 107){
					$("#popup-submit").trigger("click");
				}else{
					if(code_kind==1){
						checkCode(img);
					}else if(code_kind>2){
						$("#popup-submit").trigger("click");
					}
				}
			}
		}
	})
}

function sendtime(i) {
	i--;
	if(i == -1) {
		$("#time").html(_reg.resend || '');
		$("#send").val(0)
	} else {
		$("#send").val(1)
		$("#time").html(i + (_reg.secondUnit || ''));
		setTimeout("sendtime(" + i + ");", 1000);
	}
}
function choosexie(e){
	if(e.value==1){
		e.value=0;
	}else{
		e.value=1;
	}
}
function post_pass() {
	if (!throttleFlag) {
		throttleFlag = true;
		setTimeout(function(){
			throttleFlag = false;
		}, 1500);
	}else{
		return false;
	}
	var zyuid = $("#zy_uid").val();
	var mobile = $("#zy_mobile").val();
	var email = $("#zy_email").val();
	var pw = $("#login_password").val();
	if(zyuid == "") {
		return showToast(_reg.userNotExist || '');
	}
	if(pw == "") {
		return showToast(_reg.enterPasswordAlt || '');
	}
	showLoading();
	$.post(wapurl + "index.php?c=register&a=writtenoff", {
		zyuid: zyuid,
		mobile: mobile,
		email: email,
		pw: pw
	}, function(data) {
		if(data == 2) {
			return showToast(_reg.passwordWrong || '');

		}else if(data == 4) {			
			
			showToast(_reg.accountLocked || '',2,function(){
				yunvue.$data.checkPWshow = false;
				location.reload(true);
			});

		} else if(data == 1){
			
			showToast(_reg.unbindSuccess || "", 2, function() {
				yunvue.$data.checkPWshow = false;
				location.reload(true);
			});
		}
	})
}
function checkwxbind(target_form) {
	if (!throttleFlag) {
		throttleFlag = true;
		setTimeout(function(){
			throttleFlag = false;
		}, 1500);
	}else{
		return false;
	}
	
	if(exitsid("moblie") && $("#login_sj_box").css('display') != 'none') {
		var moblie = $("#moblie").val();
		if(moblie == "") {
			showToast(_reg.enterMobile || "");
			return false;
			
		} else if(!isjsMobile(moblie)) {
			showToast(_reg.mobileFormatError || "");
			return false;
		}
	}
	if(exitsid("moblie_code") && $("#login_sj_box").css('display') != 'none') {
		if($("#moblie_code").val() == "") {
			showToast(_reg.smsCodeRequired || '');			
			return false;
		}
	}
	
	post2ajax(target_form);
	return false;
}

function bindacount(){
	if (!throttleFlag) {
		throttleFlag = true;
		setTimeout(function(){
			throttleFlag = false;
		}, 1500);
	}else{
		return false;
	}
	var provider=$.trim($("#provider").val());
	var username=$.trim($("#username").val());
	var password=$.trim($("#password").val()); 
	if(username==''||password==''){
		return showToast(_reg.usernamePasswordRequired || '');
	}
	var authcode;
	
	var verify_token,verify_str;
	var codesear=new RegExp(_reg.codeWebLogin || '');
	if(codesear.test(code_web)){
		if(code_kind==1){
			authcode=$.trim($("#checkcode").val());  
			if(!authcode){
				return showToast(_reg.enterAuthCode || '');
			}					
		}else if(code_kind>2){
		
			verify_token = $('input[name="verify_token"]').val();
			
			if(verify_token ==''){
				if (code_kind==6) {
					$("#bind-captcha").trigger("click");
				} else {
					$("#bind-submit").trigger("click");
				}
				return false; 
			}
			verify_str = $('input[name="verify_str"]').val();
		}
	}
	
	showLoading(_reg.processing || '');
    
    $.post(wapurl + "index.php?c=login&a=baloginsave",{provider:provider,username:username,password:password,authcode:authcode,verify_token:verify_token,verify_str:verify_str}, function (data) {

		hideLoading();

        var json_data = eval('(' + data + ')');

        if (json_data.msg) {
			if($("#bind-captcha").length>0){
				$("#popup-submit").trigger("click");
			}
			
			showToast(json_data.msg, json_data.tm, function () {
				if (json_data.url) {
					location.href = json_data.url; 
				}  
			});
			checkCode('vcode_img'); 
			
			return false; 
        } else if (json_data.url) {
            location.href = json_data.url;
			return false; 
        }

    });
    
    
    return false;
}
function creatacount(){
	if (!throttleFlag) {
		throttleFlag = true;
		setTimeout(function(){
			throttleFlag = false;
		}, 1500);
	}else{
		return false;
	}
	var provider=$.trim($("#provider").val());
	showLoading(_reg.processing || '');
	$.post(wapurl + "index.php?c=login&a=balogin", {provider:provider}, function(data) {
		hideLoading();
		data = eval('(' + data + ')');

		if (data.url != '' && data.msg != '') {
			showToast(data.msg,2, function() {
				window.location.href = data.url;
			});
		} else if (data.url != '') {
			window.location.href = data.url;
		}
	});
}
