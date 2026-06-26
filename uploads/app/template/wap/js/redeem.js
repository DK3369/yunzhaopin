// 
function checkform_redeem_show(){

	var num=$("#num").val();
	var stock=$("#stock").val();
	var uid=$("#uid").val();
	var id=$("#id").val();
	var restriction=$("#restriction").val();
	var memberintegral=$("#memberintegral").val();
	var redeemintegral=$("#redeemintegral").val();

	if(!uid){
		showToast(WAP_JS_I18N.s5945fbd1, 2,function(){
			location.href=wapurl+"/index.php?c=login";
		});
		return false;
	}else if(num==0){
		showToast(WAP_JS_I18N.s0c714e0a);
		return false;
	}else if(Number(num)>Number(restriction) && restriction!="0"){
		showToast(WAP_JS_I18N.s0a703283);
		return false;
	}else if(Number(num)>Number(stock)){
		showToast(WAP_JS_I18N.s8c5a39ec);
		return false;
	}else if(Number(num)*redeemintegral>memberintegral){
		showToast(WAP_JS_I18N.seb3fe0aa+integral_pricename+WAP_JS_I18N.scf4f169c,2,function(){
			window.location.href=wapurl+'member/index.php?c=pay';

		});
		return false;
	}	
	window.location.href=wapurl+"/index.php?c=redeem&a=dh&id="+id+"&num="+num;
}
// 、
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
		body=WAP_JS_I18N.se512d692+dhbody;
	}
  
	var other = $("#other").val();
	if(other!=''){
		body = body+WAP_JS_I18N.s8e5d7d0d+other;
	}
	if(!linkman||!linktel||!dhbody){
		showToast(WAP_JS_I18N.s44436bee);;
	}else{
		showConfirm(WAP_JS_I18N.se817719c, WAP_JS_I18N.sfe04a72c,function(e){
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