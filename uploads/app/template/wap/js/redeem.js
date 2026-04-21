//积分兑换表单
function checkform_redeem_show(){

	var num=$("#num").val();
	var stock=$("#stock").val();
	var uid=$("#uid").val();
	var id=$("#id").val();
	var restriction=$("#restriction").val();
	var memberintegral=$("#memberintegral").val();
	var redeemintegral=$("#redeemintegral").val();

	if(!uid){
		showToast('您还没有登录，请先登录！', 2,function(){
			location.href=wapurl+"/index.php?c=login";
		});
		return false;
	}else if(num==0){
		showToast('请正确填写兑换数量！');
		return false;
	}else if(Number(num)>Number(restriction) && restriction!="0"){
		showToast('超出限购数量,请正确填写！');
		return false;
	}else if(Number(num)>Number(stock)){
		showToast('超出库存数量,请正确填写！');
		return false;
	}else if(Number(num)*redeemintegral>memberintegral){
		showToast('您的'+integral_pricename+'不足！',2,function(){
			window.location.href=wapurl+'member/index.php?c=pay';

		});
		return false;
	}	
	window.location.href=wapurl+"/index.php?c=redeem&a=dh&id="+id+"&num="+num;
}
//商品分类、排序
$(document).ready(function(){
	$('.nav_ft').hover(function(){ 
		$(this).find('.nav_ft_list').show(); 
	},function(){ 
		$(this).find('.nav_ft_list').hide(); 
	});
	$('.nav_rt').hover(function(){ 
		$(this).find('.nav_rt_list').show(); 
	},function(){ 
		$(this).find('.nav_rt_list').hide(); 
	});
})
function redeem_dh(){
	var id=$("#id").val();
	var num=$("#num").val();
	var linkman=$("#linkman").val();
	var linktel=$("#linktel").val();
	var dhbody=$("#dhbody").html();

console.info(123);return false;
	var body='';
	
	if(dhbody!=''){
		body="收货地址："+dhbody;
	}
  
	var other = $("#other").val();
	if(other!=''){
		body = body+" 用户留言："+other;
	}
	if(!linkman||!linktel||!dhbody){
		showToast('请填写收货人信息！');;
	}else{
		showConfirm('请输入账号登录密码', '账号登录密码',function(e){
			console.info(e);return false;
   			$.post(wapurl+"/index.php?c=redeem&a=savedh",{linkman:linkman,linktel:linktel,id:id,num:num,body:body,password:e.value},function(data){
					var data=eval('('+data+')');
	   
					if(data.errcode==9){
						showToast(data.msg,2,function(){window.location.href=data.url});
					}else{
						showToast(data.msg);	return false;
					}
					//$("#passshow").html(passshow);
				});
   		});
		document.querySelector('.mui-popup-input input').type='password' ;
		
	}
}