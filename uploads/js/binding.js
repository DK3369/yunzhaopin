function bindingPublicT(key, params, fallback) {
    var text;
    if (typeof yunT === 'function') {
        text = yunT(key, params, fallback);
    } else if (typeof yunAt === 'function') {
        text = yunAt(key, params, fallback);
    } else {
        text = fallback !== undefined ? fallback : key;
    }
    if (params && typeof text === 'string') {
        for (var name in params) {
            if (Object.prototype.hasOwnProperty.call(params, name)) {
                text = text.split('{' + name + '}').join(params[name]);
            }
        }
    }
    return text;
}

function getshow(id,title){
	if(id=='email'){
		checkCode('vcode_img');
	}else if(id=='moblie'){
		checkCode('pcode_img');
	}
	var moblie=$("#linktel").val();
	$("input[name=moblie]").val(moblie);
	var email=$("#linkmail").val();
	$("input[name=email]").val(email);
	var layindex = $.layer({
		type : 1,
		title :title,
		closeBtn : [0 , true],
		border : [10 , 0.3 , '#000', true],
		area : ['500px','auto'],
		page : {dom :"#"+id}
	});
	$("#layindex").val(layindex);
}

/**
 * @desc Send mobile binding verification code.
 * @param img Image captcha element.
 * @returns
 */
function sendmoblie(img){
	if($("#send").val()=="1"){
		return false;
	}
	var moblie=$("input[name=moblie]").val();
	var mobile=$("input[name=mobile]").val();
	var pcode=$("input[name=phoneimg_code]").val();
	
	if(pcode==""){
		layer.msg(bindingPublicT('binding_js_00001', null, 'Verification code cannot be empty!'),2,8);return false;
	}
	
	if(moblie==''){
		layer.msg(bindingPublicT('binding_js_00002', null, 'Mobile number cannot be empty!'),2,8);return false;
	}else if(mobile==moblie){
		layer.msg(bindingPublicT('binding_js_00003', null, 'Please bind a new number!'),2,8);return false;
	}else if(!isjsMobile(moblie)){
		layer.msg(bindingPublicT('binding_js_00004', null, 'Invalid mobile number format!'),2,8);return false;
	}  
	
	var i=layer.load(bindingPublicT('binding_js_00005', null, 'Processing, please wait...'),0);
	
	$.ajaxSetup({cache:false});
	
	$.post(weburl+"/member/index.php?m=ajax&c=mobliecert", {str:moblie,pcode:pcode},function(data) {
		
		layer.close(i);
		
		if(data){
		
			var res = JSON.parse(data);
			var icon = res.error == 1 ? 9 : 8;
			
			layer.msg(res.msg, 2, icon, function(){
				
				if(res.error == 1){
					sends(121);
				}else if(res.error == 106){
					checkCode(img);
				}
			});
		}
	})
}

/**
 * @desc SMS countdown.
 * @param i
 * @returns
 */
function sends(i){
	i--;
	if(i==-1){
		$('#time').html(bindingPublicT('binding_js_00006', null, 'Resend'));
		$("#send").val(0)
	}else{
		$("#send").val(1)
		$('#time').html(bindingPublicT('binding_js_00007', {seconds: i}, '{seconds}s'));
		setTimeout("sends("+i+");",1000);
	}
}

/**
 * @desc Save member center mobile verification.
 */
function check_moblie(){

	var moblie=$("input[name=moblie]").val();
	if(moblie==""){ 
		layer.msg(bindingPublicT('binding_js_00008', null, 'Please enter mobile number!'),2,8);return false;
	}
	
	var pcode=$("#phoneimg_code").val();
	if(pcode==""){ 
		layer.msg(bindingPublicT('binding_js_00009', null, 'Please enter image captcha!'),2,8,function(){getshow('moblie', bindingPublicT('binding_js_00010', null, 'Bind Mobile Number'));});return false;
	}
	
	var code=$("#moblie_code").val();
	if(code==""){ 
		layer.msg(bindingPublicT('binding_js_00011', null, 'Please enter SMS verification code!'),2,8);
		return false;
	}

	var i=layer.load(bindingPublicT('binding_js_00005', null, 'Processing, please wait...'),0);
	
	$.ajaxSetup({cache:false});
	
	$.post("index.php?c=binding&act=save",{moblie:moblie,code:code},function(data){
		
		layer.close(i);
		
		if(data==1){
			
			if($("#info").val()==1){
				
				$("#bdphone").html("<input type=\"text\" size=\"35\" name=\"linktel\" value=\""+moblie+"\" class=\"com_info_text\" style=\"width:250px;background:#D3D3D3;\" readonly=\"readonly\"/><a href=\"javascript:void(0)\"  onclick=\"getshow('moblie',bindingPublicT('binding_js_00010', null, 'Bind Mobile Number'));\" class=\"com_set_a\" >" + bindingPublicT('binding_js_00022', null, 'Rebind') + "</a>");
				
				layer.closeAll();
				
				layer.msg(bindingPublicT('binding_js_00012', null, 'Mobile bound successfully!'),2,9);
				
			}else{
				
				layer.msg(bindingPublicT('binding_js_00012', null, 'Mobile bound successfully!'),2,9,function(){location.reload();});
				
			}
			
		}else if(data==4){
			
			layer.msg(bindingPublicT('binding_js_00013', null, 'SMS verification code has expired. Please resend it!'),2,8,function(){
				$("#moblie_code").val('');
			});
			
		}else if(data==3){
			
			layer.msg(bindingPublicT('binding_js_00014', null, 'Incorrect SMS verification code!'),2,8,function(){$("#moblie_code").val('');});
			
		}else{
			
			layer.msg(bindingPublicT('binding_js_00015', null, 'Please request the SMS verification code first!'),2,8);
		}	
	})
}

function sendbemail(img){
	var email=$("input[name=email]").val();

	var myreg = /^([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9\-]+@([a-zA-Z0-9\-]+[_|\_|\.]?)*[a-zA-Z0-9]+\.[a-zA-Z]{2,3}$/; 
	
	if(email==''){
		layer.msg(bindingPublicT('binding_js_00016', null, 'Email cannot be empty!'),2,8);return false;
	}else if(!myreg.test(email)){
		layer.msg(bindingPublicT('binding_js_00017', null, 'Invalid email format!'),2,8);return false;
	}
	
	var authcode=$("input[name=email_code]").val();
	
	if(authcode==""){
		layer.msg(bindingPublicT('binding_js_00001', null, 'Verification code cannot be empty!'),2,8);return false;
	}
	
	var i=layer.load(bindingPublicT('binding_js_00005', null, 'Processing, please wait...'),0);
	
	$.ajaxSetup({cache:false});
	
	$.post(weburl+"/member/index.php?m=ajax&c=emailcert",{email:email,authcode:authcode},function(data){
		
		layer.close(i);
		
		if(data){
			
			if(data=="4"){
				layer.msg(bindingPublicT('binding_js_00018', null, 'Incorrect verification code!'),2,8,function(){checkCode(img);});
			}
			
			if(data=="3"){
				layer.msg(bindingPublicT('binding_js_00019', null, 'Email is not configured. Please contact the administrator!'),2,8);
			}
			
			if(data=="2"){
				layer.msg(bindingPublicT('binding_js_00020', null, 'Email notifications are disabled. Please contact the administrator!'),2,8);
			}
			
			if(data=="1"){
				if($("#info").val()==1){
					$("#bdmail").html("<input type=\"text\" size=\"35\" name=\"linkmail\" value=\""+email+"\" class=\"com_info_text\" style=\"width:250px;background:#D3D3D3;\" readonly=\"readonly\"/><a href=\"javascript:void(0)\"  onclick=\"getshow('email',bindingPublicT('binding_js_00021', null, 'Bind Email'));\" class=\"com_set_a\" >" + bindingPublicT('binding_js_00022', null, 'Rebind') + "</a>");
					layer.closeAll();
					layer.msg(bindingPublicT('binding_js_00023', null, 'The email has been sent. Please check your inbox to verify it!'),2,9);
				}else{
					layer.msg(bindingPublicT('binding_js_00023', null, 'The email has been sent. Please check your inbox to verify it!'),2,9,function(){location.reload();});
				}
			}
		}else{
			layer.msg(bindingPublicT('binding_js_00024', null, 'Please log in again!'),2,8,function(){window.location.href =weburl;});
		} 
	})
}

function check_company_cert(){
	if($.trim($("#company_name").val())==''){
		layer.msg(bindingPublicT('binding_js_00025', null, 'Company full name cannot be empty!'),2,8);
		return false;
	}
	if($("#social_credit").val()=='' && com_social_credit=="1") {
        layer.msg(bindingPublicT('binding_js_00026', null, 'Please enter unified social credit code!'), 2, 8);
        return false;
    }
	if($("#old_cert").val()=='' && $("input[name=check]").val() == "") {
        layer.msg(bindingPublicT('binding_js_00027', null, 'Please upload business license/organization code certificate!'), 2, 8);
        return false;
    }
    if($("#old_owner_cert").val()=='' && $("input[name=owner_cert]").val() == "" && com_cert_owner=="1") {
        layer.msg(bindingPublicT('binding_js_00028', null, "Please upload the handler's ID card!"), 2, 8);
        return false;
    }
    if($("#old_wt_cert").val()=='' && $("input[name=wt_cert]").val() == "" && com_cert_wt=="1") {
        layer.msg(bindingPublicT('binding_js_00029', null, 'Please upload authorization letter/commitment letter!'), 2, 8);
        return false;
    }
    if($("#old_other_cert").val()=='' && $("input[name=other_cert]").val() == "" && com_cert_other=="1") {
        layer.msg(bindingPublicT('binding_js_00030', null, 'Please upload other supporting materials!'), 2, 8);
        return false;
    }
	$("#certform").submit();
	layer.load(bindingPublicT('binding_js_00005', null, 'Processing, please wait...'),0);
}
function check_user_cert(){
	if($.trim($("#idcard").val())==''){
		layer.msg(bindingPublicT('binding_js_00031', null, 'Please enter ID card number!'),2,8);return false;
	}
	if($.trim($("#name").val())==''){
		layer.msg(bindingPublicT('binding_js_00032', null, 'Please enter real name!'),2,8);return false;
	}
	if(checkIdcard($.trim($("#idcard").val()))==false){
		layer.msg(bindingPublicT('binding_js_00033', null, 'Please enter a valid ID card number!'),2,8);return false;
	}
	if($("#old_cert").val()=='' && $("input[name=file]").val() == "") {
        layer.msg(bindingPublicT('binding_js_00034', null, 'Please upload ID card photo!'), 2, 8);
        return false;
    }
	
	$("#certform").submit();
	layer.load(bindingPublicT('binding_js_00005', null, 'Processing, please wait...'),0);
}
// Prevent company certificate overwrite conflicts.
function getyyzzcom(title,width,height){
	var layindex = $.layer({
		type : 1,
		title :title,
		closeBtn : [0 , true], 
		offset: ['150px', ''],
		border : [10 , 0.3 , '#000', true],
		area : ['850px','auto'],
		page : {dom :"#yyzz"}
	});
	$("#layindex").val(layindex);
}
