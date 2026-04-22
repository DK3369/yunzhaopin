// JavaScript Document
 
$(document).ready(function(){
	$(".fairs_disp_position").click(function(){
		$('.zph_make_icon_yxz').addClass('zph_make_icon_kyd');
		$('.zph_make_icon_yxz').removeClass('zph_make_icon_yxz');
		$(this).find('i').addClass('zph_make_icon_yxz');
		aid = $(this).attr("aid");
		var price = $(this).attr("price");
		zph_id = $("#zph_id").val();
		var aname = $(this).attr("aname");
		
		showLoading();
	
		$.post(wapurl+"index.php?c=ajax&a=ajaxComjob",{zph:1,id:zph_id},function(data){
			
			hideLoading();
			var data	=	eval('('+data+')');
			
			if(data.status == 1){
				reserve_vue.$data.zphzwname = aname;
				reserve_vue.$data.reserveShow = true;
				reserve_vue.$data.zphzwprice = price;
			}else{
				
				showToast(data.msg,2);return false;
			}
		});
	});
	
	$(".fairs_disp_position1").click(function(){
		showModal('该展位已被预定，请重新选择');return false;
	}); 
	
});


function closeShowStatus(aid){
  	location.reload(true);
}


function clickzph() {
	
	var jobid = "",
	zid = zph_id,
	id = aid;

	if(jobIds){
		jobid=jobIds.join(',');
	}
	
	if(!jobid){
		
		showToast('请选择参展职位', 2);return false;
	}else{
		showLoading();
		$.post(wapurl + "index.php?c=ajax&a=ajaxzphjob",{zid : zid, id : id,jobid:jobid}, function(data) {
			hideLoading();
			if (data.status == 1 || data.status == 3) {	// 1-报名成功 3-展位有其他人报名
				showToast(data.msg,2,function(){
					location.reload();
				});
			} else if (data.status == 2) {
				showConfirm(data.msg,function(){
					window.location.href = wapurl + "member/index.php?c=server&server=zph&id=" + zid + "&bid=" + id;
				})
			} else{ 
				if(data.login == 1){
					pleaselogin('您还未登录企业账号，是否登录？',wapurl+'/index.php?c=login')
				}else{
					showToast(data.msg, 2);return false;
				}
			}
		},'json');
	}
}