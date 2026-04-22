var scrollHeight = 0,
	playreload = null,
	page = 1,
	islink = 1,
	socket = null,
	ping_timer = null,
	pytoken = localStorage.getItem('pytoken'),
    xjhsyncid = null,
	player = null,
	mine = null,
	xjhid = null;

// function socketInit(){
//     if ("WebSocket" in window) {
//         webSocket(socketUrl);
//         if (xjhsyncid) {
//             chatUnlogin();
// 		}
//         hiddenCommon();
//     } else {
//         console.log("您的浏览器不支持聊天!");
//     }
//     $("body").click(function(e){
//         var con = $("#commonly");   // 设置目标区域
//         var face = $("#face");
//         if ((!con.is(e.target) && con.has(e.target).length === 0) && (!face.is(e.target) && face.has(e.target).length === 0)) {
//             commonHide();
//         }
//     });
//     $("#face").click(function(){
//         face();
//     });
// }
// function webSocket(socketUrl){
// 	if(islink == 1){
// 		if(ping_timer){
// 			clearInterval(ping_timer);
// 			ping_timer = null;
// 		}
// 		socket = new WebSocket(socketUrl);
// 		socket.onopen = function() {
// 			var ping = {
// 				type: 'ping'
// 			};
// 			ping_timer = setInterval(function() {
// 				if (socket){
// 					if(socket.readyState == 1){
// 						socket.send(JSON.stringify(ping));
// 					}else{
// 						socket.close();
// 					}
// 				}
// 			}, 30000);
// 		};
//
// 		socket.onerror = function() {
// 			islink = 2;
// 			socket = null;
// 			if(ping_timer){
// 				clearInterval(ping_timer);
// 				ping_timer = null;
// 			}
// 		};
//
// 		socket.onclose = function() {
// 			socket = null;
// 			if(ping_timer){
// 				clearInterval(ping_timer);
// 				ping_timer = null;
// 			}
// 		};
// 		//捕捉socket端发来的事件
// 		socket.onmessage = function(event) {
// 			var e = JSON.parse(event.data);
// 			//console.log(e);
// 			switch (e.type) {
// 				//显示收到的消息
// 				case 'xjhMessage':
// 					getMessage(e.data);
// 					break;
// 				case 'error':
// 					islink = 2;
// 					if(ping_timer){
// 						clearInterval(ping_timer);
// 						ping_timer = null;
// 					}
// 					break;
// 				case 'startLive':
// 					startLive(e.data);
// 					break;
// 				case 'stopLive':
// 					stopLive(e.data);
// 					break;
// 			}
// 		};
// 	}
// }
// 未登录用户加载聊天数据
function chatUnlogin(){
    httpPost('m=neirong&c=xjhlive&a=getUnloginToken', {chattype: 'admin'}).then(function (response) {
        if (response.data.error == 0) {
            var t = setInterval(function(){
                if (socket && socket.readyState == 1){
                    clearInterval(t);
                    if(response.data.data){
                        var bindData = response.data.data
                        bindData.group = 'xjh_' + xjhsyncid;
                        var bind = {
                            type: 'bind',
                            data: bindData
                        }
                        socket.send(JSON.stringify(bind));
                    }
                    window.localStorage.setItem("islink", "1");
                }
            },100);
            setTimeout(function(){
                if(t){
                    clearInterval(t);
                }
            },10000);
        }
    }).catch(function (error) {
        console.log(error);
    })
}
function hiddenCommon(){
	// 检查页面是否后台运行
	var yunhdn, yunvsb;
	if (typeof document.hidden !== "undefined") {
		yunhdn = "hidden";
		yunvsb = "visibilitychange";
	} else if (typeof document.msHidden !== "undefined") {
		yunhdn = "msHidden";
		yunvsb = "msvisibilitychange";
	} else if (typeof document.webkitHidden !== "undefined") {
		yunhdn = "webkitHidden";
		yunvsb = "webkitvisibilitychange";
	}
	document.addEventListener(yunvsb, function(){
		// 前台运行后，如socket断开，重新连接
		if(!document[yunhdn]){
			if (!socket || (socket && socket.readyState != 1)) {
				webSocket(socketUrl);
                if (xjhsyncid) {
                    chatUnlogin();
                }
			}
		}
	});
}
function getMessage(msg){
	if(msg.xjhid == 'xjh_' + xjhsyncid && (msg.id != mine.id || (msg.id == mine.id && msg.plat != 'pc')) && msg.msgtype != 'zan') {
		chatRender(msg);
	}
}
function chatRender(msg){
	// 渲染消息内容
	var	newlog = {
	    chatid: msg.chatid,
		msgtype: msg.msgtype,
		avatar: msg.avatar,
		username: msg.username,
		usertype: msg.usertype,
		content: rexContent(msg.content)
	};
	if(msg.timestamp - pastSend > 1800 * 1000) {
		newlog.time = timeFormat(msg.timestamp);
		pastSend = msg.timestamp;
	}
	if(msg.job){
		newlog.job = msg.job;
	}
	app.$data.chatlist.push(newlog);
	setTimeout(function(){
		var scroll = document.getElementById('chat_content').scrollHeight;
		chatScrollTo(scroll);
	},100);
}
// 页面滚动
function chatScrollTo(ypos) {
	var clientHeight = document.getElementById('chat_content').clientHeight;
	if(ypos > clientHeight){
		// 兼容各种浏览器
		document.getElementById('chat_content').scrollTop = ypos;
	}
}
// 转义内容
function rexContent(content){
	content = (content||'').replace(/&(?!#?[a-zA-Z0-9]+;)/g, '&amp;')
    .replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/'/g, '&#39;').replace(/"/g, '&quot;') //XSS
	.replace(/face\[([^\s\[\]]+?)\]/g, function(face){  //转义表情
		var alt = face.replace(/^face/g, '');
		return '<img class="chat_bq_img" alt="'+ alt +'" title="'+ alt +'" src="' + faces([alt]) + '">';
	})
	return content;
}
//表情库
function faces(fc){
    var alt = {1:'[龇牙]',2:'[调皮]',3:'[流汗]',4:'[偷笑]',5:'[再见]',6:'[敲打]',7:'[擦汗]',8:'[流泪]',9:'[大哭]',10:'[嘘]',11:'[酷]',12:'[抓狂]',13:'[可爱]',14:'[色]',15:'[害羞]',16:'[得意]',17:'[吐]',18:'[微笑]',19:'[怒]',20:'[尴尬]',21:'[惊恐]',22:'[冷汗]',23:'[白眼]',24:'[傲慢]',25:'[难过]',26:'[惊讶]',27:'[疑问]',28:'[么么哒]',29:'[困]',30:'[憨笑]',31:'[撇嘴]',32:'[阴险]',33:'[奋斗]',34:'[发呆]',35:'[左哼哼]',36:'[右哼哼]',74:'[抱抱]',37:'[坏笑]',38:'[鄙视]',39:'[晕]',40:'[可怜]',41:'[饥饿]',42:'[咒骂]',43:'[折磨]',44:'[抠鼻]',45:'[鼓掌]',46:'[糗大了]',47:'[打哈欠]',48:'[快哭了]',49:'[吓]',50:'[闭嘴]',51:'[大兵]',52:'[委屈]',53:'[NO]',54:'[OK]',56:'[弱]',57:'[强]',60:'[握手]',63:'[胜利]',58:'[抱拳]',66:'[凋谢]',99:'[米饭]',108:'[蛋糕]',112:'[西瓜]',70:'[啤酒]',89:'[瓢虫]',62:'[勾引]',82:'[爱你]',69:'[咖啡]',72:'[月亮]',68:'[刀]',55:'[差劲]',59:'[拳头]',65:'[便便]',79:'[炸弹]',107:'[菜刀]',82:'[心碎了]',83:'[爱心]',71:'[太阳]',97:'[礼物]',92:'[皮球]',137:'[骷髅]',123:'[闪电]',80:'[猪头]',67:'[玫瑰]',98:'[篮球]',64:'[乒乓]',101:'[红双喜]',139:'[麻将]',73:'[彩带]',61:'[爱你]',95:'[示爱]',111:'[衰]',109:'[蜡烛]'},arr = {};
	$.each(alt, function(index, item){
		arr[item] = weburl + '/js/im/emoji_'+ index +'@2x.png';
    });
	if(fc){
		return arr[fc];
	}else{
		return arr;
	}
}
//在焦点处插入内容
function focusInsert(obj, str, nofocus){
    var result, val = obj.value;
    nofocus || obj.focus();
    if(document.selection){ //ie
		result = document.selection.createRange();
		document.selection.empty();
		result.text = str;
		return ''
    } else {
		result = [val.substring(0, obj.selectionStart), str, val.substr(obj.selectionEnd)];
		nofocus || obj.focus();
		return result.join('');
    }
}

// 格式化时间戳
function timeFormat(sendtime) {
	var nowtime = new Date();
	if (sendtime) {
		sendtime = new Date(sendtime);
	} else {
		sendtime = nowtime;
	}
	var year = sendtime.getFullYear(),
		nowyear = nowtime.getFullYear();
		month = sendtime.getMonth(),
		day = sendtime.getDate(),
		hour = sendtime.getHours(),
		minutes = sendtime.getMinutes();

	month = month + 1;
	if (month < 10) {
		month = '0' + month;
	}
	if (hour < 10 && hour > 0) {
		hour = '0' + hour;
	}
	if (minutes < 10) {
		minutes = '0' + minutes;
	}
	if (day < 10 && day > 0) {
		day = '0' + day;
	}
	var time = month + '-' + day + ' ' + hour + ':' + minutes;
	if(year != nowyear){
		time = year + '-' + time;
	}
	return time;
}
// 获取文档完整的高度
function getScrollHeight() {
	return Math.max(document.body.scrollHeight, document.documentElement.scrollHeight);
}

// 直播
function livePlay(live_hls, live_poster){
    // debugger
	if (!document.getElementById('live_video')) {
		$('#livebody').append('<video id="live_video" preload="auto" playsinline webkit-playsinline></video>');
	}
	if (player) {
		player.dispose();
		player = null;
	}
	setTimeout(function() {
		if(document.getElementById('live_video')){
			var body = document.body,
				poster = live_poster;
			player = new TCPlayer('live_video', {
		        "sources": [{src: live_hls}],
				"autoplay": true,
				"width": '800',
				"height": '400',
				"poster": poster
			});

			var playerint = null;
			player.on('error', function(error) {
				player.dispose();
				player = null;
				if(!playerint){
					playerint = setInterval(function(){
						livePlay();
					},3000);
				}
			});
			player.on('playing', function(error) {
				if(playerint){
					clearInterval(playerint);
					playerint = null;
				}
			});
			// 播放器重载
			if(playreload){
				clearInterval(playreload);
				playreload = null;
			}
			playreload = setInterval(function(){
				if(player && player.player_ == null){
					livePlay();
				}
			},3000);
		}
	}, 100);
}
function playered(url, mbstyle){
	// debugger
	if(document.getElementById('playered')){
		// 实例化播放器
		player = new QPlayer({
			url: url,
			container: document.getElementById("playered"),
			autoplay: true,
			loggerLevel: 3,
			defaultViewConfig:{
				settingsIcon: mbstyle +'/images/qiniuset.png'
			}
		});
	}
}