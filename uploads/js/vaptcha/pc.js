$(document).ready(function(){

	if(document.getElementById("bind-captcha")){
		vaptcha({
			  vid: vaptchaid, // Verification unit id.
			  type: "invisible", // Display type: invisible.
			  scene: 0, // Scene value; default is 0.
			  offline_server: "", // Offline-mode server address. If offline mode is not configured, any value is acceptable.
			  // Optional parameters.
			  //lang: 'auto', // Language. Default is auto. Options: auto, zh-CN, en, zh-TW, jp.
			  //https: true, // Use HTTPS. Default is true.
			}).then(function (vaptchaObj) {
			  obj = vaptchaObj; // Save the VAPTCHA instance to a local variable.
			  // Token method 1:
			  //vaptchaObj.renderTokenInput('.login-form')// Use this to add a token value to the form when submitting as a form.
			  // Token method 2:
			  vaptchaObj.listen("pass", function () {
				// Continue after successful verification.
				
				$("input[name='verify_token']").val(vaptchaObj.getToken());
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
			  });
			  // Triggered when the verification popup closes.
			  vaptchaObj.listen("close", function () {
				
			   
			  });
		});

	}
	
	
	$("#popup-submit").click(function(){
		
		
		$("input[name='verify_token']").val('');
		
		obj.reset();
		
	});
	$("#bind-submit").click(function(){
		
		
		obj.validate();

	});
});
