function sendmoblie(img){
	if($("#send").val()=="1"){
		return false;
	}
	var moblie=$("input[name=moblie]").val();
	var authcode=$("input[name=authcode]").val();
	if(moblie==''){
		showToast(WAP_JS_I18N.s54f98bcb,2);return false;
	}else if(!isjsMobile(moblie)){
		showToast(WAP_JS_I18N.s7f2c8eb4,2);return false;
	}
	if(!authcode){
		showToast(WAP_JS_I18N.sb7579ede,2);return false;
	}
	showLoading();
	$.post(wapurl+"/index.php?c=ajax&a=mobliecert", {str:moblie,code:authcode},function(data) {
		hideLoading();
		if(data){
			var res = JSON.parse(data);
			showToast(res.msg, 2, function(){
				if(res.error == 1){
					sends(121);
				}else if(res.error == 106){
					checkCode(img);
				}
			});
		}
	})
}
function sends(i){
	i--;
	if(i==-1){
		$("#time").html(WAP_JS_I18N.s029ca60d);
		$("#send").val(0);
	}else{
		$("#send").val(1);
			$("#time").html(i+WAP_JS_I18N.ssecond);
		setTimeout("sends("+i+");",1000);
	}
}


function check_moblie(img){

	var moblie=$("input[name=moblie]").val();
	var authcode=$("input[name=authcode]").val();
	var code=$("#moblie_code").val();
	
	if(moblie==""){ 
		showToast(WAP_JS_I18N.se4e2c965,2);return false;
	}else if(code==""){ 
		showToast(WAP_JS_I18N.se434e644,2);return false;
	}else if(!authcode){
		showToast(WAP_JS_I18N.sa239e34a,2);return false;
	}
	
	showLoading();
	
	$.post("index.php?c=binding",{moblie:moblie,code:code},function(data){

		hideLoading();
		
		if(data==1){
			if(usertype=='4'){
				showToast(WAP_JS_I18N.sb7c75e16,2,function(){window.location.href = 'index.php?c=binding'}); 
			}else{
				showToast(WAP_JS_I18N.sb7c75e16,2,function(){window.location.href = 'index.php?c=set'}); 
			}				
		}else if(data==4){
			showToast(WAP_JS_I18N.sbdf6e3b2,2);
		}else if(data==3){
			showToast(WAP_JS_I18N.s9ca54186,2);
		}else{
			showToast(WAP_JS_I18N.s140d55ec,2); 
		}
	})
}

function check_email(img){
	
	var email=$("input[name='email']").val();
	var authcode=$("input[name='authcode']").val();
	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/;
	
	if(email==''){
		showToast(WAP_JS_I18N.sf7b16299,2);return false;
	}else if(!myreg.test(email)){
		showToast(WAP_JS_I18N.s5dfb1f62,2);return false;
	}else if(!authcode){
		showToast(WAP_JS_I18N.see948b48,2);return false;
	}
	
	showLoading();
	
	$.post(wapurl + '/index.php?c=ajax&a=emailcert',{email:email,authcode:authcode},function(data){
		hideLoading();
		if(data){
			if(data=="3"){
				showToast(WAP_JS_I18N.s7dd29307,2);
			}else if(data=="2"){
				showToast(WAP_JS_I18N.s451becee,2);
			}else if(data=="1"){
				if(usertype=='4'){
					showToast(WAP_JS_I18N.s2e71d44e,2,function(){window.location.href = 'index.php?c=binding'});
				}else{
					showToast(WAP_JS_I18N.s2e71d44e,2,function(){window.location.href = 'index.php?c=set'});
				}				
			}else if(data=="5"){
				showToast(WAP_JS_I18N.see948b48,2);
			}else if(data=="4"){
				showToast(WAP_JS_I18N.s9918a236,2,function(){checkCode(img)});
			}
		}else{
			showToast(WAP_JS_I18N.sf344099d,2);
		} 
	})
}