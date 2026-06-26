// 
var timer, flag;
var loaded = true;
var throttle = function(func, wait = 1000){
	if (!flag) {
		flag = true;
		if(document.getElementById('pageLoading')){
			document.getElementById('pageLoading').classList.remove('none');
		}
		if(document.getElementById('pageNoMore')){
			document.getElementById('pageNoMore').classList.add('none');
		}
		
		// ，wait
		typeof func === 'function' && func();
		timer = setTimeout(() => {
			flag = false;
		}, wait);
	}
}
window.onscroll = function() {
	var a = getScrollTop();
	var b = getClientHeight();
	var c = getScrollHeight();
	if(c - b - a < 1){
		// 
		throttle(fetchData_list);
	}
}
// 
function getScrollTop() { 
	var scrollTop = 0; 
	if (document.documentElement && document.documentElement.scrollTop) { 
		scrollTop = document.documentElement.scrollTop; 
	} else if (document.body) { 
		scrollTop = document.body.scrollTop; 
	} 
	return scrollTop; 
} 
// 
function getClientHeight() { 
	var clientHeight = 0; 
	if (document.body.clientHeight && document.documentElement.clientHeight) { 
		clientHeight = Math.min(document.body.clientHeight, document.documentElement.clientHeight); 
	} 
	else { 
		clientHeight = Math.max(document.body.clientHeight, document.documentElement.clientHeight); 
	} 
	return clientHeight; 
} 
// 
function getScrollHeight() { 
	return Math.max(document.body.scrollHeight, document.documentElement.scrollHeight); 
}