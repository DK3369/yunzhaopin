function ShowPartDiv(id){
	var obj=document.getElementById(id);
	if(obj.style.display=='block'){
		$("#"+id).hide(200);
	}else{
		$("#"+id).show(200);
	}
}
$(function(){
	$('body').click(function (evt) {
		if($(evt.target).parents("#BillingCycle").length==0 && evt.target.id != "BillingCycleButton") {
		   $('#BillingCycle').hide();
		}
		if($(evt.target).parents("#PartType").length==0 && evt.target.id != "PartTypeButton") {
		   $('#PartType').hide();
		}
	})
	//分享显示隐藏
	$('.share_con').hover(
		function(){
			$('.newJsbg').show();							   
		},function(){
			$('.newJsbg').hide();	
		}
	);	
})
function CheckPartType(id,name){
	$("#PartTypeButton").val(name);
	$("#type").val(id);
	$('#PartType').hide();
}
//收藏兼职
function PartCollect(jobid,comid){
	layer.load('执行中，请稍候...',0);
	$.post(weburl+"/index.php?m=part&c=partcollect",{jobid:jobid,comid:comid},function(data){
		layer.closeAll('loading');
		var data=eval('('+data+')');
		if(Number(data.status)==9){
			layer.msg(data.msg, 2, Number(data.status), function (){
				location.reload();
			});return false;
		}
		layer.msg(data.msg, 2, Number(data.status));return false;
	})
}
//取消收藏
function CancelCollect(jobid){
	layer.load('执行中，请稍候...',0);
	$.post(weburl+"/member/index.php?c=partcollect&act=del", {ids: jobid}, function (data) {
		layer.closeAll('loading');
		var data=eval('('+data+')');
		if(Number(data.errcode)==9){
			layer.msg('取消收藏成功', 2, Number(data.errcode), function (){
				location.reload();
			});return false;
		}

		layer.msg(data.msg, 2, Number(data.errcode));return false;
	});
}

//兼职报名
function PartApply(jobid){
	layer.load('执行中，请稍候...',0);
	$.post(weburl+"/index.php?m=part&c=partapply",{jobid:jobid},function(data){
		layer.closeAll('loading');
		var data=eval('('+data+')');
		layer.msg(data.msg, 2, Number(data.status),function(){location.reload();});return false;
	})
}
