
// 
function PartCollect(jobid,comid){
	showLoading()
	$.post(wapurl+"?c=part&a=collect",{jobid:jobid,comid:comid},function(data){
		hideLoading();
		var data=eval('('+data+')');
		if(Number(data.status)==9){
			showToast(data.msg, 2,function(){
				location.reload(true);
			})
		}else{
			showToast(data.msg, 2);
		}
	})
}
// 
function CancelCollect(jobid){
	showLoading(WAP_JS_I18N.s6071a22e)
	$.post(sy_weburl+"/api/wxapp/index.php?h=user&m=part&c=delfavpart", {ids: jobid}, function (data) {
		hideLoading();
		if (data.error == 1) {
			showToast(WAP_JS_I18N.s5f88fe78, 2, function () {
				location.reload(true);
			});
		} else {
			showToast(data.msg);
		}
	});
}
// 
function PartApply(jobid){
	showLoading()
	$.post(wapurl+"/index.php?c=part&a=apply",{jobid:jobid},function(data){
		hideLoading();
		var data=eval('('+data+')');
		if(Number(data.status)==9){
			window.localStorage.setItem("needRefresh", 1);
			showToast(data.msg, 2,function(){
				location.reload(true);
			})
		}else{
			showToast(data.msg, 2);
		}
	})
}
function toDate(str){
	var sd=str.split("-");
	return new Date(sd[0],sd[1],sd[2]);
}
function CheckPost_part(){
	if($.trim($("#name").val())==""){
		showToast(WAP_JS_I18N.sdeffb737,2);return false;
	}
	if($.trim($("#typeid").val())<1){
		showToast(WAP_JS_I18N.s853c8076,2);return false;
	}
	if($.trim($("#number").val())==""||$.trim($("#number").val())=="0"){
		showToast(WAP_JS_I18N.s44323441,2);return false;
	}
	var chk_value =[];
	$('input[name="worktime[]"]:checked').each(function(){
		chk_value.push($(this).val());
	});
	if(chk_value.length==0){
		showToast(WAP_JS_I18N.sb5db7785,2);return false;
	}
	var sdate=$("#sdate").val().split(' ');
	var edate=$("#edate").val().split(' ');
	var timetype=$("input[name='timetype']:checked").val();
	if(sdate==""){
		showToast(WAP_JS_I18N.s46eb34ac,2);return false;
	} 
	if(timetype!='1'){
		if(edate==""){
			showToast(WAP_JS_I18N.s72752bb5,2);return false;
		}
		if(toDate(edate[0])<toDate(sdate[0])){
			showToast(WAP_JS_I18N.s379efabf,2);return false;
		}
	}	
	if($.trim($("#salary").val())==""||$.trim($("#salary").val())=="0"){
		showToast(WAP_JS_I18N.sa7dd3c74,2);return false;
	}
	if($.trim($("#salary_typeid").val())==""){
		showToast(WAP_JS_I18N.s31fc844c,2);return false;
	}
	if($.trim($("#billing_cycleid").val())<1){
		showToast(WAP_JS_I18N.se137a4be,2);return false;
	}
		if($.trim($("#cityid").val())==""){
		showToast(WAP_JS_I18N.s1220df68,2);return false;
	}	
	if($.trim($("#address").val())==""){
		showToast(WAP_JS_I18N.s67da70a6,2);return false;
	}
	if($.trim($("#map_x").val())==""||$.trim($("#map_y").val())==""){
		showToast(WAP_JS_I18N.s146512d4,2);return false;
	}	
	var content=UE.getEditor('description').hasContents();  
	
	if(content==""||content==false){
		showToast(WAP_JS_I18N.sa6ee3682,2);return false;
	}else{
		var description =UE.getEditor('description').getContent();  
		document.getElementById("description").value=description;
	} 
	if($.trim($("#linkman").val())==""){
		showToast(WAP_JS_I18N.s96c8f480,2);return false;
	}
	
    var linktel=isjsMobile($.trim($("#linktel").val()));
	if($.trim($("#linktel").val())==""){
		showToast(WAP_JS_I18N.sa7632d12,2);return false;
	}else if(linktel==false){
        showToast(WAP_JS_I18N.s000c8b79,2);return false;
  }
}
function ckpartjob(type){
	var val=$("#"+type+"id").find("option:selected").text();
	$('.'+type).html(val);
}