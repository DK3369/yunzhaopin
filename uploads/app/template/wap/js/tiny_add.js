
	function oncesubmit() {
		var name = $.trim(document.getElementById('username').value),
			id = $.trim(document.getElementById('id').value),
			sex = $.trim(document.getElementById('sex').value),
			exp = $.trim(document.getElementById('exp').value),
			job = $.trim(document.getElementById('job').value),
			provinceid = $.trim(document.getElementById('provinceid').value),
			cityid = $.trim(document.getElementById('cityid').value),
			three_cityid = $.trim(document.getElementById('three_cityid').value),
			production = $.trim(document.getElementById('production').value),
			mobile = $.trim(document.getElementById('mobile').value),
			password = $.trim(document.getElementById('password').value),
			checkcode,
			
			verify_token,
			verify_str;
		if(name == '') {
			return showToast(WAP_JS_I18N.sd71d86f8);
		}
		if(sex == '') {
			return showToast(WAP_JS_I18N.s27d2081e);
		}
		if(exp == '') {
			return showToast(WAP_JS_I18N.sc6843816);
		}
		var cionly ='';
		if(ct.length<=0 || ct=='new Array()'){
			cionly = '1';
		}
		if(cionly == '1'){
			if(provinceid == '') {
				return showToast(WAP_JS_I18N.sfb9ab0e0);
			}
		}else{
			if(cityid == '') {
				return showToast(WAP_JS_I18N.sfb9ab0e0);
			}
		}
		
		if(production == '') {
			return showToast(WAP_JS_I18N.sfd2b6431);
		}
		if(job == '') {
			return showToast(WAP_JS_I18N.s6e22a705);
		}
		if(mobile == '') {
			return showToast(WAP_JS_I18N.seb3b55e3);
		}
		if(isjsMobile(mobile) == false) {
			return showToast(WAP_JS_I18N.s3fbe0259);
		}
		if(exitsid("moblie_code")){
			var moblie_code = $("#moblie_code").val();
			if(moblie_code == ''){
				return showToast(WAP_JS_I18N.sd37489f1);			
			}			
		}
		if(password == '') {
			return showToast(WAP_JS_I18N.s46dd03e4);
		}
		if(code_web.indexOf(WAP_JS_I18N.s3ae6924b) >= 0) {
			if(code_kind == 1) {
				var checkcode = $("#checkcode").val();
				if(checkcode == '') {
					return showToast(WAP_JS_I18N.s377e9f00);
				}
			} else if(code_kind >2) {
				$("#bind-captcha").attr('data-id','oncesubmit');
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
		}
	
		$.post(wapurl + "/index.php?c=tiny&a=add", {
			id: id,
			name: name,
			sex: sex,
			exp: exp,
			job: job,
			provinceid: provinceid,
			cityid: cityid,
			three_cityid: three_cityid,
			production: production,
			mobile: mobile,
			password: password,
			authcode: checkcode,
			moblie_code:moblie_code,
			verify_token:verify_token,
			verify_str:verify_str,
			submit: 'submit'
		}, function(data) {
			if(data.url) {
				showToast(data.msg, 2, function() {
					location.href = data.url;
				});
			} else {
				checkCode('vcode_img');
				showToast(data.msg, 2);
				return false;
			}
	
		}, 'json');
}
