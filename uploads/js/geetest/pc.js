$(document).ready(function(){


var handlerPopup = function (captchaObj) {
	// Success callback.
	
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
	
	// Add the captcha to the element with id captcha.
	
	//captchaObj.appendTo("#bind-captcha");
	// For more APIs, see: http://www.geetest.com/install/sections/idx-client-sdk.html

};

if($("#bind-captcha").length>0){
	$.ajax({
			url: weburl+"/index.php?m=geetest&t=" + (new Date()).getTime(), // Add a random number to prevent caching.
			type: "get",
			dataType: "json",
			success: function (data) {
				// Use the initGeetest API.
				// Parameter 1: configuration.
				// Parameter 2: callback. The first callback parameter is the captcha object, which can then be used for appendTo and similar events.
				initGeetest({
					gt: data.gt,
					challenge: data.challenge,
					product: "bind", // Product form: float, embed, or popup. This is only valid for the PC captcha.
					width:"100%",
					offline: !data.success, // Indicates whether the backend detected Geetest server downtime; usually no attention is needed.
					new_captcha: data.new_captcha
				}, handlerPopup);
			}
	});
}

});
