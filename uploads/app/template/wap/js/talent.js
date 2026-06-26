function ckresume(type){
	var val=$("#"+type).find("option:selected").text(); 
	$('.'+type).html(val); 
}

function tresume(){	
	var id = $.trim(document.getElementById('id').value),
		name=$.trim(document.getElementById('name').value),
		hy=$.trim(document.getElementById('hy').value),
		provinceid=$.trim(document.getElementById('provinceid').value),
		cityid=$.trim(document.getElementById('cityid').value),
		three_cityid=$.trim(document.getElementById('three_cityid').value),
		minsalary=$.trim(document.getElementById('minsalary').value),
		maxsalary=$.trim(document.getElementById('maxsalary').value),
		jobstatus=$.trim(document.getElementById('jobstatus').value),
		jobname=$.trim(document.getElementById('jobname').value),
		sex=$.trim(document.getElementById('sex').value),
		age=$.trim(document.getElementById('age').value),
		edu=$.trim(document.getElementById('edu').value),
		exp=$.trim(document.getElementById('exp').value),
		telphone=$.trim(document.getElementById('telphone').value),
		living=$.trim(document.getElementById('living').value),
		expinfo=$.trim(document.getElementById('expinfo').value),
		eduinfo=$.trim(document.getElementById('eduinfo').value),
		skillinfo=$.trim(document.getElementById('skillinfo').value),
		projectinfo=$.trim(document.getElementById('projectinfo').value);
	if(name==""){
		return mui.toast(WAP_JS_I18N.sd71d86f8);return false;
	}
	if(sex==''){
		return mui.toast(WAP_JS_I18N.s27d2081e);return false;
	}
	if(age==''){
		return mui.toast(WAP_JS_I18N.se8950cb1);return false;
	}
	
	if(edu==''){
		return mui.toast(WAP_JS_I18N.s1125855c);return false;
	}
	if(exp==''){
		return mui.toast(WAP_JS_I18N.sdd9b65f4);return false;
	}
	if(telphone==''){
		return mui.toast(WAP_JS_I18N.s2864ebae);return false;
	}else{
		if(!isjsMobile(telphone)){
			return mui.toast(WAP_JS_I18N.s7f2c8eb4);return false;
		}
	}
	
	if(living==''){
		return mui.toast(WAP_JS_I18N.sbb04b415);return false;
	}
	
	if(jobname==""){
		return mui.toast(WAP_JS_I18N.s67d1ef91);return false;
	}
	if(hy==""){
		return mui.toast(WAP_JS_I18N.s75754a1b);return false;
	}
	if(minsalary==""){
		return mui.toast(WAP_JS_I18N.s7bceedae);return false;
	}
	if(maxsalary){
		if(parseInt(maxsalary)<=parseInt(minsalary)){
			return mui.toast(WAP_JS_I18N.s97c45b89);return false;
		}
	}
	var cionly ='';
	if(ct.length<=0 || ct=='new Array()'){
		cionly = '1';
	}
	if(cionly == '1'){
		if(provinceid == '') {
			return mui.toast(WAP_JS_I18N.s6cf18fb6);return false;
		}
	}else{
		if(cityid==""){
			return mui.toast(WAP_JS_I18N.s6cf18fb6);return false;
		}
	}
	
	
	if(jobstatus==""){
		return mui.toast(WAP_JS_I18N.s2e155c8c);return false;
	}		

	if(expinfo==""){
		return mui.toast(WAP_JS_I18N.s2c549ec0);return false;
	}
	if(eduinfo==""){
		return mui.toast(WAP_JS_I18N.s13833753);return false;
	}
	document.getElementById('resumesubmit').innerText=WAP_JS_I18N.sabe2c5d2;
	document.getElementById('resumesubmit').id='submit';
	mui.post(wapurl + "/member/index.php?c=savetalentexpect", 
		{id:id,name:name,hy:hy,jobname:jobname,provinceid:provinceid,cityid:cityid,three_cityid:three_cityid,minsalary:minsalary,maxsalary:maxsalary,jobstatus:jobstatus,sex:sex,age:age,edu:edu,exp:exp,telphone:telphone,living:living,eduinfo:eduinfo,expinfo:expinfo,skillinfo:skillinfo,projectinfo:projectinfo,submit:'submit'}, function(data) {
			if(data.error=='1'){
				showToast(WAP_JS_I18N.s3b6eb9c6,2,function(){window.location.href=wapurl+'/member/index.phpindex.php?c=talent';}); 
			}else{
				return mui.toast(date.msg);
			}
		}, 'json');
}
$(document).ready(function(){	
	// 
	$(".lt_reward_sq").click(function(){
		
		var jobid=$(this).attr('data-jobid');
		var eid=$(this).attr('data-eid');
		
		
		showLoading()
		$.post(wapurl+"/member/index.php?c=talentsqjob",{jobid:jobid,eid:eid},function(data){
			hideLoading();
			var data=eval('('+data+')');
			if(data.error==1){          
				showToast(WAP_JS_I18N.sd8a56db4,2,function(){location.reload(true);});
				
			}else{
				showToast(data.msg, 2);return false;
			}
		});
	})
	
})

function tsendmoblie(){
	if($("#send").val()=="1"){
		return false;
	}
	var moblie=$("input[name=linktel]").val();
	var authcode=$("input[name=authcode]").val();
	if(moblie==''){
		showToast(WAP_JS_I18N.s54f98bcb,2);return false;
	}else if(!isjsMobile(moblie)){
		showToast(WAP_JS_I18N.s7f2c8eb4,2);return false;
	}
	if(!authcode){
		showToast(WAP_JS_I18N.sa239e34a,2);return false;
	}
	showLoading();
	$.post(wapurl+"/index.php?c=ajax&a=mobliecert", {str:moblie,code:authcode},function(data) {
		hideLoading();
		if(data){
			var res = JSON.parse(data);
			showToast(res.msg, 2, function(){
				if(res.error == 1){
					tsend(121);
				}else if(res.error == 106){
					checkCode('vcode_img');
				}
			});
		}
	})
}
function tsend(i){
	i--;
	if(i==-1){
		$("#time").html(WAP_JS_I18N.s029ca60d);
		$("#send").val(0);
	}else{
		$("#send").val(1);
		$("#time").html(i+"秒");
		setTimeout("tsend("+i+");",1000);
	}
}
function telstatus(){
	var id = $('#telid').val();
	var linktel = $('#linktel').val();
	
	if(linktel==""){ 
		showToast(WAP_JS_I18N.se4e2c965,2);return false;
	}
	var code=$("#moblie_code").val();
	if(code==""){ 
		showToast(WAP_JS_I18N.se434e644,2);return false;
	}
	
	showLoading();
	$.ajaxSetup({cache:false});
	$.post(wapurl+"/member/index.php?c=telstatus",{id:id,linktel:linktel,code:code},function(data){
		hideLoading();
		data = eval('('+data+')');
		if(data.error=='1'){
			
			showToast(WAP_JS_I18N.s20462283,2,function(){window.location.href=wapurl+'/member/index.php?c=talent';}); 
			
		}else{
			showToast(data.msg,2); 
		}
	})
}