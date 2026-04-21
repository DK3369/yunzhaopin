// 节流函数
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
		
		// 如果是立即执行，则在wait毫秒内开始时执行
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
		// 滚动到底部加载新数据
		throttle(fetchData_list);
	}
}
// 获取当前滚动条的位置
function getScrollTop() { 
	var scrollTop = 0; 
	if (document.documentElement && document.documentElement.scrollTop) { 
		scrollTop = document.documentElement.scrollTop; 
	} else if (document.body) { 
		scrollTop = document.body.scrollTop; 
	} 
	return scrollTop; 
} 
// 获取当前可视范围的高度 
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
// 获取文档完整的高度 
function getScrollHeight() { 
	return Math.max(document.body.scrollHeight, document.documentElement.scrollHeight); 
}