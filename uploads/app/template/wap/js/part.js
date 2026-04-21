
//收藏兼职
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
//取消收藏
function CancelCollect(jobid){
	showLoading('取消收藏...')
	$.post(sy_weburl+"/api/wxapp/index.php?h=user&m=part&c=delfavpart", {ids: jobid}, function (data) {
		hideLoading();
		if (data.error == 1) {
			showToast('取消收藏成功', 2, function () {
				location.reload(true);
			});
		} else {
			showToast(data.msg);
		}
	});
}
//兼职报名
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
		showToast("请输入职位名称！",2);return false;
	}
	if($.trim($("#typeid").val())<1){
		showToast("请选择工作类型！",2);return false;
	}
	if($.trim($("#number").val())==""||$.trim($("#number").val())=="0"){
		showToast("请输入招聘人数！",2);return false;
	}
	var chk_value =[];
	$('input[name="worktime[]"]:checked').each(function(){
		chk_value.push($(this).val());
	});
	if(chk_value.length==0){
		showToast("请选择兼职时间！",2);return false;
	}
	var sdate=$("#sdate").val().split(' ');
	var edate=$("#edate").val().split(' ');
	var timetype=$("input[name='timetype']:checked").val();
	if(sdate==""){
		showToast("请选择开始日期！",2);return false;
	} 
	if(timetype!='1'){
		if(edate==""){
			showToast("请选择结束日期！",2);return false;
		}
		if(toDate(edate[0])<toDate(sdate[0])){
			showToast("开始日期不能大于结束日期！",2);return false;
		}
	}	
	if($.trim($("#salary").val())==""||$.trim($("#salary").val())=="0"){
		showToast("请输入薪水！",2);return false;
	}
	if($.trim($("#salary_typeid").val())==""){
		showToast("请选择薪水类型！",2);return false;
	}
	if($.trim($("#billing_cycleid").val())<1){
		showToast("请选择结算周期！",2);return false;
	}
		if($.trim($("#cityid").val())==""){
		showToast("请选择工作地点！",2);return false;
	}	
	if($.trim($("#address").val())==""){
		showToast("请输入详细地址！",2);return false;
	}
	if($.trim($("#map_x").val())==""||$.trim($("#map_y").val())==""){
		showToast("请选择地图！",2);return false;
	}	
	var content=UE.getEditor('description').hasContents();  
	
	if(content==""||content==false){
		showToast("请输入兼职内容！",2);return false;
	}else{
		var description =UE.getEditor('description').getContent();  
		document.getElementById("description").value=description;
	} 
	if($.trim($("#linkman").val())==""){
		showToast("请输入联系人！",2);return false;
	}
	
    var linktel=isjsMobile($.trim($("#linktel").val()));
	if($.trim($("#linktel").val())==""){
		showToast("请输入联系手机！",2);return false;
	}else if(linktel==false){
        showToast("请输入正确的联系手机！",2);return false;
  }
}
function ckpartjob(type){
	var val=$("#"+type+"id").find("option:selected").text();
	$('.'+type).html(val);
}