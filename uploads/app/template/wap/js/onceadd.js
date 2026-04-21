
function oncesubmit(){
	var title = $.trim(document.getElementById('title').value),
		salary = $.trim(document.getElementById('salary').value),
		provinceid = $.trim(document.getElementById('provinceid').value),
		cityid = $.trim(document.getElementById('cityid').value),
		three_cityid = $.trim(document.getElementById('three_cityid').value),
		address = $.trim(document.getElementById('address').value),
		content = $.trim($("#content").val()),
		contents = $.trim($("#contents").val()),
		companyname = $.trim(document.getElementById('companyname').value),
		linkman = $.trim(document.getElementById('linkman').value),
		phone = $.trim(document.getElementById('phone').value),
		oncepricegearObj = document.getElementById('oncepricegear'),
		oncepricegear = oncepricegearObj ? $.trim(oncepricegearObj.value) : '',
		password = $.trim(document.getElementById('password').value),
		preview = $.trim(document.getElementById('preview').value),
		id = $.trim(document.getElementById('id').value),
		
		checkcode,
		verify_str,
		verify_token;

		if(!id || id == '') {
			id = 0;
		} else {
			id = id;
		}

		if(!pic || pic == '') {
			pic = '';
		} else {
			pic = pic;
		}
		if(title == '') {
			return showToast('请填写招聘名称！');
		}
		if(salary == '') {
			return showToast('请填写工资！');
		}
		var cionly ='';
		if(ct.length<=0 || ct=='new Array()'){
			cionly = '1';
		}
		if(cionly == '1'){
			if(provinceid == '') {
				return showToast('请选择工作地区！');
			}
		}else{
			if(cityid == '') {
				return showToast('请选择工作地区！');
			}
		}
		
		if(address == '') {
			return showToast('请填写详细地址！');
		}
		if((typeof(content) == 'undefined' || content == '') && contents == '') {
			return showToast('请填写招聘要求！');
		}
		if(companyname == '') {
			return showToast('请填写店面名称！');
		}
		if(linkman == '') {
			return showToast('请填写联系人！');
		}
		if(phone == '') {
			return showToast('请填写联系电话！');
		}
		if(isjsMobile(phone) == false) {
			return showToast('请注意联系电话格式！');
		}
		if(exitsid("moblie_code")){
			var moblie_code = $("#moblie_code").val();
			if(moblie_code == ''){
				return showToast('请填写短信验证码！');			
			}
			formData.append('moblie_code', moblie_code);
		}
		if (!id && oncepricegear_num == 0) {
			return showToast('招聘时长未配置，请联系管理员！');
		}
		if(!id && oncepricegear == '') {
			return showToast('请选择招聘时长！');
		}
		if (document.getElementById('yyzz') != null && document.getElementById('yyzzpreviewimg').getAttribute('src') == '') {
			return showToast('请上传店铺营业执照！');
		}
		if(password == '') {
			return showToast('请填写密码！');
		}
		if(code_web.indexOf("店铺招聘")>=0) {
			if(code_kind == 1) {
				var code = $('#checkcode').val();
				if(code == '') {
					return showToast('请填写验证码！');
				}
			}else if(code_kind > 2) {
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

		formData.append('title', title);
		formData.append('salary', salary);
		formData.append('provinceid', provinceid);
		formData.append('cityid', cityid);
		formData.append('three_cityid', three_cityid);
		formData.append('address', address);
		formData.append('companyname', companyname);
		formData.append('linkman', linkman);
		formData.append('phone', phone);
		formData.append('require', typeof(content) != 'undefined' && content != '' ? content : contents);
		formData.append('oncepricegear', oncepricegear);
		formData.append('password', password);
		if (document.getElementById('yyzz') != null) {
			formData.append('yyzzpreview', document.getElementById('yyzz').value != '' ? $.trim(document.getElementById('yyzzpreview').value) : '');
		}
		formData.append('preview', document.getElementById('pic').value != '' ? preview : '');
		formData.append('id', id);

		if(code_web.indexOf("店铺招聘") >= 0){
			if(code_kind == 1){
				formData.append('authcode', code);
			}else if(code_kind > 2){
			
				formData.append('verify_token', verify_token);
				formData.append('verify_str', verify_str);
			}
		}
		formData.append('submit', 1);
		showLoading();
		$.ajax({
			url: wapurl+"index.php?c=once&a=add",
			type: 'post',
			data: formData,
			contentType: false,
			processData: false,
			dataType: 'json',
			success: function(res) {
				hideLoading();
				var res = JSON.stringify(res);
				var data = JSON.parse(res);
				if(data.url) {
					showToast(data.msg, 2, function() {
						location.href = data.url;
					});
				} else {
					checkCode('vcode_img');
					showToast(data.msg, 2);
					return false;
				}
			}
		})
}