$(document).ready(function(){


var handlerPopupMobile = function (captchaObj) {
	// Success callback.
	captchaObj.appendTo("#popup-captcha-mobile");

	captchaObj.onSuccess(function () {
		
		var validate = captchaObj.getValidate();
		
		if(validate){
			
	

			$("input[name='verify_token']").val(validate.geetest_challenge+'*'+validate.geetest_validate+'*'+validate.geetest_seccode);
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
		}

	});
	
	$("#bind-submit").click(function(){
		
		captchaObj.verify();
	});
	$("#popup-submit").click(function(){
		$("input[name='verify_token']").val('');
		
		captchaObj.reset();
	});
	

};

if($("#bind-captcha").length>0){

	$.ajax({
			url: wapurl+"/index.php?c=geetest&t=" + (new Date()).getTime(), 
			type: "get",
			dataType: "json",
			success: function (data) {
				
				initGeetest({
					
					gt: data.gt,
					challenge: data.challenge,
					offline: !data.success,
					product:'bind',
					width:"100%",
					new_captcha: data.new_captcha
				}, handlerPopupMobile);
			}
	});
}

});
