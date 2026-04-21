function pwdCheck(){
	var obj = $('#reg_password').val();
	if(obj==""){
		parent.layer.msg("密码不能为空！", 2, 8);
		return false;
	}else if(obj.length<6 || obj.length>20 ){
		parent.layer.msg("只能输入6至20位密码", 2, 8);
		return false;
	}else{
	 	$.ajax({
			type: "POST",
			async: false,
			url: "index.php?m=register&c=ajaxreg",
			data: {
				password: obj
			},
			success: function(data) {
				var res = JSON.parse(data);
				if(res.errcode != 0){
					var resume_exp = $.trim($("#resume_exp").val());
					var resume_edu = $.trim($("#resume_edu").val());
					var resume_pro = $.trim($("#resume_pro").val());
					
					parent.layer.msg(res.msg, 2, 8, function(){
						// 有错误提示，让内容返回
						if (resume_exp == 1 || resume_edu == 1 || resume_pro == 1) {
							addresumereturn();
						}
					});
					return false;
				}
			}
		});
	}
}
function nextaddresume() {
	var uname = $.trim($("#uname").val());
	var sex = $("#sex").val();
	var birthday = $.trim($("#birthday").val());
	var edu = $.trim($("#educid").val());
	var exp = $.trim($("#expid").val());
	var telphone = $.trim($("#telphone").val());
	var email = $.trim($("#email").val());
	var type = $.trim($("#typeid").val());
	var report = $.trim($("#reportid").val());
	var authcode = $.trim($("#authcode").val());
	var checkcode = $.trim($("#CheckCodefast").val());

	var resume_exp = $.trim($("#resume_exp").val());
	var resume_edu = $.trim($("#resume_edu").val());
	var resume_pro = $.trim($("#resume_pro").val());

	if (uname == "") {
		parent.layer.msg("请填写真实姓名！", 2, 8);
		return false;
	} else {
		if(parseInt(resumename) && parseInt(resumename) > 0 && !isChinaName(uname)){
			parent.layer.msg("姓名请输入2-6位汉字", 2, 8);
			return false;
		}
	}
	if (sex == "") {
		parent.layer.msg("请填写性别！", 2, 8);
		return false;
	}
	if (birthday == "") {
		parent.layer.msg("请选择出生年月！", 2, 8);
		return false;
	}
	if (edu == "") {
		parent.layer.msg("请选择最高学历！", 2, 8);
		return false;
	}
	if (exp == "") {
		parent.layer.msg("请选择工作经验！", 2, 8);
		return false;
	}

	if (telphone == '') {
		parent.layer.msg("请填写手机号码！", 2, 8);
		return false;
	} else {
		if (!isjsMobile(telphone)) {
			parent.layer.msg("手机号码格式错误！", 2, 8);
			return false;
		} else {
			var authcode = $("#authcode");
			if (authcode.length > 0 && authcode.val() == '') {
				parent.layer.msg("请输入短信验证码！", 2, 8);
				return false;
			}
		}
	}
	if (resume_exp == 1) {
		var exparr = expcreate.split(',');
		if (exparr.indexOf(exp) > -1 || isShow == 1) {
			$('#isexpshow').show();
		} else {
			$('#isexpshow').hide();

			$("#iscreateexp1").attr("checked", true);
			layui.use(['form'], function() {
				var form = layui.form;
				form.render();
			});
		}
	}
	if (resume_edu == 1) {
		var eduarr = educreate.split(',');
		if (eduarr.indexOf(edu) > -1 || isShow == 1) {
			$('#isedushow').show();
		} else {
			$('#isedushow').hide();

			$("#iscreateedu1").attr("checked", true);
			layui.use(['form'], function() {
				var form = layui.form;
				form.render();
			});
		}
	}
	if (resume_pro == 1) {
		var exparr = expcreate.split(',');
		if (exparr.indexOf(exp) > -1 || isShow == 1) {
			$('#isproshow').show();
		} else {
			$('#isproshow').hide();

			$("#iscreatepro").attr("checked", true);
			layui.use(['form'], function() {
				var form = layui.form;
				form.render();
			});
		}
	}
	$("#applydiv").hide();
	$("#nextaddresume").show();
}

function returnmessagejobfast(frame_id) {
	if (frame_id == '' || frame_id == undefined) {
		frame_id = 'supportiframefast';
	}

	var message = $(window.frames[frame_id].document).find("#layer_msg").val();

	if (message != null) {

		var layer_time = $(window.frames[frame_id].document).find("#layer_time").val();
		var layer_st = $(window.frames[frame_id].document).find("#layer_st").val();
		var layer_url = $(window.frames[frame_id].document).find("#layer_url").val();

		parent.layer.closeAll('loading');

		if (layer_st == '9') {
			parent.fastsuccess();
		} else {
			if (layer_url == '') {
				checkCode('vcode_img_temp_fast');
				parent.layer.msg(message, layer_time, Number(layer_st));
			} else {
				parent.layer.msg(message, layer_time, Number(layer_st), function() {
					parent.window.location.reload();
					return false;
				});
			}
		}
	}
}
function addresumereturn() {
	$("#applydiv").show();
	$("#nextaddresume").hide();
}

function OnLogin() {
	// 关闭自身iframe层
	var index = parent.layer.getFrameIndex(window.name); //先得到当前iframe层的索引
	parent.layer.close(index); //再执行关闭 
	parent.showlogin('1');
}
function checkaddresume(img, imgreg, url) {
	var jobid = $.trim($("#jobid").val());
	var uname = $.trim($("#uname").val());
	var sex = $("#sex").val();
	var birthday = $.trim($("#birthday").val());
	var edu = $.trim($("#educid option:selected").val());
	var exp = $.trim($("#expid option:selected").val());
	var telphone = $.trim($("#telphone").val());
	var email = $.trim($("#email").val());
	var type = $.trim($("#typeid").val());
	var report = $.trim($("#reportid").val());
	
	var password = $("#reg_password").val();
	var resumeid = $("#resumeid").val();
	var jobid = $("#jobid").val();
	if (uname == "") {
		parent.layer.msg("请填写真实姓名！", 2, 8);return false;
	}
	if (sex == "") {
		parent.layer.msg("请填写性别！", 2, 8);return false;
	}
	if (edu == "") {
		parent.layer.msg("请选择最高学历！", 2, 8);return false;
	}
	if (exp == "") {
		parent.layer.msg("请选择工作经验！", 2, 8);return false;
	}

	if (telphone == '') {
		parent.layer.msg("请填写手机号码！", 2, 8);return false;
	} else {
		if (!isjsMobile(telphone)) {
			parent.layer.msg("手机号码格式错误！", 2, 8);return false;
		}
	}
	
	if (password == "") {
		parent.layer.msg("请输入密码！", 2, 8);return false;
	}else if (password.length < 6 || password.length > 20) {
		parent.layer.msg("请输入6至20位密码！", 2, 8);return false;
	}
	
	if(code_kind == '1') {
		var checkcode = $.trim($("#CheckCodefast").val());
		if (checkcode == '') {
			parent.layer.msg("请输入图片验证码！", 2, 8);
			return false;
		}
	} else if(code_kind >2) {
		//改变验证需要的id
		$("#bind-captcha").attr('data-id','subform');
		$("#bind-captcha").attr('data-type','submit');
		var verify_token = $('input[name="verify_token"]').val();
		if(verify_token == '') {
            if (code_kind == 6) {
                $("#bind-captcha").trigger("click");
            } else {
                $("#bind-submit").trigger("click");
            }
			return false;
		}
	
	}
	var authcode = $("#authcode");
	if (authcode.length > 0 && authcode.val() == '') {
		parent.layer.msg("请输入短信验证码！", 2, 8);
		return false;
	}
	var resume_exp = $('#resume_exp');
	if (resume_exp.length >0 && resume_exp.val() == '1') {
		var workname = document.getElementById('workname'), 
			worksdate = document.getElementById('worksdate'), 
			workedate = document.getElementById('workedate'), 
			worktitle = document.getElementById('worktitle'), 
			workcontent = document.getElementById('workcontent');
		if (workname.value == '') {
			parent.layer.msg("请填写公司名称！", 2, 8);
			return false;
		}
		if (worktitle.value == '') {
			parent.layer.msg("请填写担任职务！", 2, 8);
			return false;
		}
		if (worksdate.value == '') {
			parent.layer.msg("请填写入职时间！", 2, 8);
			return false;

		}
	}
	var resume_edu = $('#resume_edu');
	if (resume_edu.length >0 && resume_edu.val() == '1') {
		var eduname = document.getElementById('eduname'), 
			edusdate = document.getElementById('edusdate'), 
			eduedate = document.getElementById('eduedate'), 
			education = $.trim($("#educationcid option:selected").val()), 
			eduspec = document.getElementById('eduspec');
		if (eduname.value == '') {
			parent.layer.msg("请填写学校名称！", 2, 8);
			return false;
		}
		if (edusdate.value == '') {
			parent.layer.msg("请填写入学时间！", 2, 8);
			return false;
		}
		if (eduedate.value == '') {
			parent.layer.msg("请填写离校或预计离校时间！", 2, 8);
			return false;
		}
		
		if (education == '') {
			parent.layer.msg("请选择毕业学历！", 2, 8);
			return false;
		}
	}
	var resume_pro = $('#resume_pro');
	if (resume_pro.length >0 && resume_pro.val() == '1') {
		var proname = document.getElementById('proname'), 
			prosdate = document.getElementById('prosdate'), 
			proedate = document.getElementById('proedate'), 
			protitle = document.getElementById('protitle'), 
			procontent = document.getElementById('procontent');
		if (proname.value == '') {
			parent.layer.msg("请填写项目名称！", 2, 8);
			return false;
		}
		if (protitle.value == '') {
			parent.layer.msg("请填写项目担任职务！", 2, 8);
			return false;
		}
		if (prosdate.value == '') {
			parent.layer.msg("请填写项目开始时间！", 2, 8);
			return false;
		}
		if (proedate.value == '') {
			parent.layer.msg("请填写项目结束时间！", 2, 8);
			return false;
		}
	}
	parent.layer.load('执行中，请稍候...',0);
}

function ckjobreg(id) {
	var telphone = $.trim($("#telphone").val());
	var email = $.trim($("#email").val());
	if (id == 1) {
		if (telphone !== '') {
			if (!isjsMobile(telphone)) {
				parent.layer.msg("手机号码格式错误！", 2, 8);
				return false;
			} else {
				$.post(weburl + "/index.php?m=register&c=regmoblie", {
					moblie : telphone
				}, function(data) {
					if (data != 0) {
						parent.layer.msg("手机号码已被使用！", 2, 8);
						return false;
					}
				});
			}
			return true;
		} else {
			parent.layer.msg("手机号码格式错误！", 2, 8);
			return false;
		}
	} else {
		if (email == '') {
			parent.layer.msg("请填写联系邮箱！", 2, 8);
			return false;
		} else {
			var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
			if (!myreg.test(email)) {
				parent.layer.msg("邮箱格式错误！", 2, 8);
				return false;
			} else {
				$.post(weburl + "/index.php?m=register&c=ajaxreg", {
					email : email
				}, function(data) {
					if (data != 0) {
						parent.layer.msg("邮箱已被使用！", 2, 8);
						return false;
					}
				});
			}
		}
	}
}

var Timer;
var smsTimer_time = 90; // 倒数 90
var smsTimer_flag = 90; // 倒数 90
var smsTime_speed = 1000; // 速度 1秒钟
// 发送手机短信
function send_msg(url) {
	
	var telphone = $('#telphone').val();
	var checkcode,
		verify_token,
		verify_str;

	if (telphone == '') {
		parent.layer.msg("请填写手机号码！", 2, 8);
		return false;
	} else {
		if (!isjsMobile(telphone)) {
			parent.layer.msg("手机号码格式错误！", 2, 8);
			return false;
		} else {
			if(code_kind == '1') {
				checkcode = $.trim($("#CheckCodefast").val());
				if (checkcode == '') {
					parent.layer.msg("请输入图片验证码！", 2, 8);
					return false;
				}
			} else if(code_kind > 2) {
				//改变验证需要的id
				$("#bind-captcha").attr('data-id','send_msg_tips');
				$("#bind-captcha").attr('data-type','click');
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

			var returntype;
			parent.layer.load('执行中，请稍候...',0);
			$.ajax({
				async : false,
				type : "POST",
				url : weburl + "/index.php?m=register&c=regmoblie",
				dataType : 'text',
				data : {
					'moblie' : telphone
				},
				success : function(data) {
					parent.layer.closeAll('loading');
					if (data != 0) {
						returntype = 1;
					}
				}
			});
			if (returntype == 1) {
				layer.confirm('手机号已注册账号，您可直接登录？',{title:false}, function(){
					OnLogin();
				});
				return false;
			}
		}
	}
	if (smsTimer_time == smsTimer_flag) {
		Timer = setInterval("smsTimer($('#send_msg_tips'))", smsTime_speed);
		parent.layer.load('执行中，请稍候...',0);
		$.post(url, {
			moblie : telphone,
			code : checkcode,
			verify_token: verify_token,
			verify_str: verify_str
		}, function(data) {
			parent.layer.closeAll('loading');
			if (data) {
				var res = JSON.parse(data);
				if (res.error != 1) {
					clearInterval(Timer);
				}
				var icon = res.error == 1 ? 9 : 8;
				parent.layer.msg(res.msg, 2, icon, function() {
					if (res.error != 1) {
						clearInterval(Timer);
						if (code_kind == 1) {
							checkCode('vcode_imgs');
						} else if (code_kind>2) {
							$("#popup-submit").trigger("click");
						}
					}
				});
			}
		})
	} else {
		layer.msg('请勿重复发送！', 2, 8);
		return false;
	}
}

// 倒计时
function smsTimer(obj) {
	if (smsTimer_flag > 0) {
		$(obj).html('重新发送(' + smsTimer_flag + 's)');
		$(obj).attr({
			'style' : 'color:#f00;font-weight: bold;'
		});
		smsTimer_flag--;
	} else {
		$(obj).html('重新发送');
		$(obj).attr({
			'style' : 'color:#f00;font-weight: bold;'
		});
		smsTimer_flag = smsTimer_time;
		clearInterval(Timer);
	}
}