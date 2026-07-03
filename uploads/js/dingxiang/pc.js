$(document).ready(function(){
	if(document.getElementById("bind-captcha")){
		var uptime = 0;
		var myCaptcha = _dx.Captcha(document.getElementById('bind-captcha'), {
			appId: dxappid, // appId from the console's application management or application configuration module.
			style: 'popup',
			success: function (token) {
				myCaptcha.hide();
                if (uptime > 0 && ((new Date()).valueOf() - uptime) <= 500) {// Prevent duplicate auto-submit when refreshing captcha images fails.
                    myCaptcha.reload();
                    return false;
                }
				$("input[name='verify_token']").val(token);
			    // Submit operation.
				var type = $('#bind-captcha').attr('data-type');
				var dataid = $('#bind-captcha').attr('data-id');
				// Submit the form.
				if(type=='submit'){
					$('#'+dataid).submit();
				}else{
					// Simulate a click.
					$("#"+dataid).trigger("click");
				}
			   //console.log('token:', token)
			}
		})
	}
	$("#popup-submit").click(function(){
        uptime = (new Date()).valueOf();// Update time.
		$("input[name='verify_token']").val('');
		myCaptcha.reload();
		//throw SyntaxError();
	});
	$("#bind-submit").click(function(){
		myCaptcha.show();
	});
});
