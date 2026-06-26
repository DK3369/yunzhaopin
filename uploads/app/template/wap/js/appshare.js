(function() {
	var plusReady = function(callback) {
		if(window.plus) {
			callback();
		} else {
			document.addEventListener('plusready', callback);
		}
	}
	var shareServices = {};
	var init = function() {
		plus.share.getServices(function(services) {
			for(var i = 0, len = services.length; i < len; i++) {
				shareServices[services[i].id] = services[i];
			}
		});
	};
	var isWechatInstalled = function() {
		return plus.runtime.isApplicationExist && plus.runtime.isApplicationExist({
			pname: 'com.tencent.mm',
			action: 'weixin://'
		});
	};

	function share(id, msg, callback) {
		var service = shareServices[id];
		if(!service) {
			callback && callback(false);
			return;
		}
		var _share = function() {
			service.send(msg, function() {
				plus.nativeUI.toast(WAP_JS_I18N.s74c11dae + service.description + WAP_JS_I18N.s15136bf7);
				callback && callback(true);
			}, function(e) {
				plus.nativeUI.toast(WAP_JS_I18N.s74c11dae + service.description + WAP_JS_I18N.sebe62412);
				callback && callback(false);
			})
		};
		if(service.authenticated) {
			_share(service, msg, callback);
		} else {
			service.authorize(function() {
				_share(service, msg, callback);
			}, function(e) {
				console.log(WAP_JS_I18N.s3fd33816);
				callback && callback(false);
			})
		}
	};

	function openSystem(msg, callback) {
		if(plus.share.sendWithSystem) {
			plus.share.sendWithSystem(msg, function() {
				// TODO
				//callback && callback(true);
			}, function() {
				// TODO
				//callback && callback(false);
			});
		} else {
			callback && callback(false);
		}
	}
	var open = function(msg, callback) {
		/**
		 *如下情况直接打开系统分享
		 * 1、未配置微信分享通道
		 * 2、用户手机未安装威胁你
		 * 3、360浏览器下
		 */

		/*if(shareServices.weixin && isWechatInstalled() && !/360\sAphone/.test(navigator.userAgent)) {
			plus.nativeUI.actionSheet({
				title: WAP_JS_I18N.s74c11dae,
				cancel: WAP_JS_I18N.s625fb26b,
				buttons: [{
					title: WAP_JS_I18N.sf8d4a7de
				}, {
					title: WAP_JS_I18N.s38b0b472
				}, {
					title: WAP_JS_I18N.s2534b1c6
				}]
			}, function(e) {
				var index = e.index;
				switch(index) {
					case 1: //分享到微信好友
						msg.extra = {
							scene: 'WXSceneSession'
						};
						share('weixin', msg, callback);
						break;
					case 2: //分享到微信朋友圈
						msg.title = msg.content;
						msg.extra = {
							scene: 'WXSceneTimeline'
						};
						share('weixin', msg, callback);
						break;
					case 3: //更多分享
						var url = msg.href ? ('( ' + msg.href + ' )') : '';
						msg.title = msg.title + url;
						msg.content = msg.content + url;
						openSystem(msg, callback);
						break;
				}
			})
		} else {*/
			// 
			//var url = msg.href ? ('( ' + msg.href + ' )') : '';
			//msg.title = msg.title + url+'sssss';
			//msg.content = msg.content + url+'sss';
			openSystem(msg, callback);
		//}
	};
	plusReady(init);
	window.plusShare = open;
})();