$(document).ready(function(){
	if(document.getElementById("bind-captcha")){
		var uptime = 0;
		var myCaptcha = _dx.Captcha(document.getElementById('bind-captcha'), {
			appId: dxappid, //appId，在控制台中“应用管理”或“应用配置”模块获取
			style: 'popup',
			success: function (token) {
				myCaptcha.hide();
                if (uptime > 0 && ((new Date()).valueOf() - uptime) <= 500) {// 防止刷新验证码获取图片失败自动校验重复提交
                    myCaptcha.reload();
                    return false;
                }
				$("input[name='verify_token']").val(token);
			    //提交操作
				var type = $('#bind-captcha').attr('data-type');
				var dataid = $('#bind-captcha').attr('data-id');
				//提交表单
				if(type=='submit'){
					$('#'+dataid).submit();
				}else{
					//模拟点击
					$("#"+dataid).trigger("click");
				}
			   //console.log('token:', token)
			}
		})
	}
	$("#popup-submit").click(function(){
        uptime = (new Date()).valueOf();// 更新时间
		$("input[name='verify_token']").val('');
		myCaptcha.reload();
		//throw SyntaxError();
	});
	$("#bind-submit").click(function(){
		myCaptcha.show();
	});
});