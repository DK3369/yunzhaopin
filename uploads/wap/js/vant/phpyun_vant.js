
$(function(){
	//图片预览
	
	imgPreview();
}) 
function imgPreview(){
	
	//图片预览
	$(".imgPreview").click(function() {
		console.log(123);
		var group = $(this).attr('data-group');
	    var thissrc = $(this).attr('data-src');
	    var imgarr = [];
	    var startPosition = 0;
	    $(".imgPreview[data-group='"+group+"']").each(function(index){
	    	imgsrc = $(this).attr('data-src');
	    	if(imgsrc){
	    		imgarr.push(imgsrc);
	    		if(thissrc==imgsrc){
	    			startPosition = index;
	    		}
	    	}
	    })
	    vant.ImagePreview({
	      images:imgarr,
	      startPosition: startPosition,
	    });
	});
}
// 加载框
function showLoading(msg = '加载中') {
	vant.Toast.loading({
		message: msg,
		duration: 0,
		forbidClick: true // 是否禁止背景点击
	});
}
// 关闭加载框
function hideLoading() {
	vant.Toast.clear();
}
// 轻提示
function showToast(msg = '', duration = 2, func) {
	vant.Toast({
		message: msg,
		duration: duration * 1000,
		forbidClick: true,
		onClose: function() {
			typeof func === 'function' && func();
		}
	});
}
// 待确定按钮的提示框
function showModal(msg = '', func, confirmText = '确定') {
	vant.Dialog.alert({
		title: '温馨提示',
		message: msg, // 提示内容
		theme: 'round',
		confirmButtonText: confirmText // 确定按钮文本
	}).then(function(){
		typeof func === 'function' && func();
	})
}
// 询问框
function showConfirm(msg, success, cancelText = '取消', confirmText = '确定', cancel) {
	vant.Dialog.confirm({
		title: '温馨提示',
		message: msg, // 提示内容
		theme: 'round',
		confirmButtonText: confirmText, // 确定按钮文本
		cancelButtonText: cancelText // 取消按钮文本
	}).then(function(){
		typeof success === 'function' && success();
	}).catch(function(){
		typeof cancel === 'function' && cancel();
	})
}
// 获取参数
function getUrlKey(name){
	return decodeURIComponent((new RegExp('[?|&]'+name+'='+'([^&;]+?)(&|#|;|$)').exec(location.href)||[,""])[1].replace(/\+/g,'%20'))||null;
}
