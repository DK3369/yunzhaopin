
$(function(){
	imgPreview();
})

function wapJsI18n(key) {
	return (typeof WAP_JS_I18N !== 'undefined' && WAP_JS_I18N[key]) ? WAP_JS_I18N[key] : '';
}

function imgPreview(){
	$(".imgPreview").click(function() {
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
// Loading overlay
function showLoading(msg) {
	msg = msg || wapJsI18n('loading');
	vant.Toast.loading({
		message: msg,
		duration: 0,
		forbidClick: true
	});
}
// Hide loading overlay
function hideLoading() {
	vant.Toast.clear();
}
// Toast notification
function showToast(msg, duration, func) {
	if (msg === undefined || msg === null) {
		msg = '';
	}
	if (duration === undefined || duration === null) {
		duration = 2;
	}
	vant.Toast({
		message: msg,
		duration: duration * 1000,
		forbidClick: true,
		onClose: function() {
			typeof func === 'function' && func();
		}
	});
}
// Alert dialog with confirm button
function showModal(msg, func, confirmText) {
	msg = msg || '';
	confirmText = confirmText || wapJsI18n('confirm');
	vant.Dialog.alert({
		title: wapJsI18n('warmTip'),
		message: msg,
		theme: 'round',
		confirmButtonText: confirmText
	}).then(function(){
		typeof func === 'function' && func();
	})
}
// Confirm dialog
function showConfirm(msg, success, cancelText, confirmText, cancel) {
	cancelText = cancelText || wapJsI18n('cancel');
	confirmText = confirmText || wapJsI18n('confirm');
	vant.Dialog.confirm({
		title: wapJsI18n('warmTip'),
		message: msg,
		theme: 'round',
		confirmButtonText: confirmText,
		cancelButtonText: cancelText
	}).then(function(){
		typeof success === 'function' && success();
	}).catch(function(){
		typeof cancel === 'function' && cancel();
	})
}
// Read URL query parameter
function getUrlKey(name){
	return decodeURIComponent((new RegExp('[?|&]'+name+'='+'([^&;]+?)(&|#|;|$)').exec(location.href)||[,""])[1].replace(/\+/g,'%20'))||null;
}
