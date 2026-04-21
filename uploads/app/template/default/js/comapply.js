window.onload = function () {
	var oDiv = document.getElementById("float"),
		H = 600,
		Y = oDiv;
	while (Y) {
		H += Y.offsetTop;
		Y = Y.offsetParent;
	}
	window.onscroll = function () {
		var s = document.body.scrollTop || document.documentElement.scrollTop
		if (s > H) {
			if (headerfix == 1) {
				var top = '40px';
			} else {
				var top = '0px';
			}
			$("#float").attr('style', 'position:fixed;top:' + top + ';display:block');
		} else {
			$("#float").attr('style', '');
		}
	}
}

$(function () {
	$(".sq_job").on('click',function(){
		click_sq();
	});
	
	$(".job_hr_left_ly").click(function () {
		$("#hrmsg").show(500);
	});
	$("#hrmsg").hover(function () {
		$("#hrmsg").show();
	}, function () {
		$("#hrmsg").hide(500);
	});
	var cheight = $("#job_content").height();
	if (parseInt(cheight) > 270) {
		$("#job_content").attr('style', 'width:100%;height:270px; overflow:hidden');
		$(".company_show_more").show();
	}
	$(".popupClose").click(function () {
		$(".phonePop-up").hide();
	});
});

function showcc() {
	$("#job_content").attr('style', 'width:100%;height:auto; overflow:hidden');
	$(".company_show_more").find('a').html('收起');
	$(".company_show_more").find('a').attr('onclick', 'hidecc()');
}

function hidecc() {
	$("#job_content").attr('style', 'width:100%;height:270px; overflow:hidden');
	$(".company_show_more").find('a').html('查看更多');
	$(".company_show_more").find('a').attr('onclick', 'showcc()');
}
function click_sq(){
	loadlayer();
	$.post(weburl+"/index.php?m=ajax&c=sq_job",{companyname:com_name,jobname:jobname,companyuid:companyuid,jobid:jobid,eid:eid},function(res){
		layer.closeAll();
		res = eval('(' + res + ')');

		var data = res.errorcode;
		var msg = res.msg;

		if(data==4){
			layer.msg('该职位已邀请您面试，无需再投简历！', 2, 8);
		}else if(data==9){
			var i = layer.open({
				type: 1,
				title: '职位申请',
				closeBtn: 1,
				border: [10, 0.3, '#000', true],
				area: ['520px', 'auto'],
				content: $("#suc_job"),
				cancel:function(){
					window.location.reload();
				}
			});
		}else if(data==2){
			layer.msg('系统出错，请稍后再试！', 2,8);return false;
		}else if(data==3){
			layer.msg('您已申请过该职位！', 2,8);return false;
		}else if(data==31){
			layer.msg('您最近申请过该职位，请勿重复投递！', 3,8);return false;
		}else if(data==5){
			layer.msg('该职位已过期，不能申请该职位！', 2, 8);return false;
		}else if(data==6){
			layer.msg('该职位不存在！', 2, 8);return false;
		}else if(data==7 || data==101 || data==102){
			layer.msg(msg, 2 , 8 ,function(){window.open(weburl + "/" + res.url);window.event.returnValue = false;return false; });
		}else if(data==8 || data==12){
			msg = msg==''?'请选择投递的简历！':msg;
			layer.msg(msg, 2, 8,function(){
				window.location.href =weburl+"/member/index.php?c=expect&act=add";window.event.returnValue = false;return false; 
			});return false;
		}else if(data==10){
			layer.msg('请先公开您的简历！', 2, 8,function(){window.location.href =weburl + "/" + res.url;window.event.returnValue = false;return false; });return false;
		}else if(data==11){
			layer.msg(msg, 2, 8);return false;
		}else{
			layer.alert('请先登录！',0,'提示',function(){window.location.href="index.php?m=login&usertype=1";window.event.returnValue=false;return false;});
		}
	});
}

//招聘频道切换
function searchtype(id) {
	$(".search_curs").removeClass("search_curs");
	$("#type" + id).addClass("search_curs");
	$(".contentBox_conts").hide();
	$("#type_" + id).show();
}

/*	联系方式	  */
function showtel(id) {
	var loadi = layer.load('执行中，请稍候...', 0);
	$.post(weburl + "/job/index.php?c=comapply&a=gettel", {
		id: id
	}, function (data) {
		layer.close(loadi);
		var data = eval('(' + data + ')');

		if (data.linkCode == 10) {

			clearTimeout(t);

			$('#prvlinktel').html(data.prvlinktel);
			$('#prvusertel').html(data.prvusertel);
			$("#dial").remove();
			$("#dialpend").prepend('<a href="javascript:;" id="dial">请尽快拨打</a>');
			$('#prvtime').html(data.prvtime + '秒后过期');
			prvtime(data.prvtime, id);

			$('.phonePop-up').show();
			return false;

		} else if (data.linkCode == 11) {

			layer.msg(data.linkMsg, 2, 8);
			return false;
		} else {
			$('#linkman').html(data.linkman);
			$('#linktel').html(data.linktel);
			$('#linkphone').html(data.linkphone);
		}
		layui.use('layer', function() {
			var layer = layui.layer,
				$ = layui.$;
			layer.open({
				type: 1,
				title: '',
				zIndex:100,
				area: ['auto', 'auto'],
				content : $('#tel_show')
			});   
		});
	});
}

var t;

function prvtime(i, id) {
	i--;
	if (i == -1) {

		$("#prvtime").html('号码已过期');
		$("#dial").remove();
		$("#dialpend").prepend('<a href="javascript:;" onclick="showtel(' + id + ')" id="dial">重新获取</a>');
	} else {

		$("#prvtime").html(i + '秒后过期');
		t = setTimeout("prvtime(" + i + "," + id + ");", 1000);
	}
}

//求职咨询
function showmessage(uid, usertype) {
	if (uid) {

		if (usertype != 1) {
			layer.msg('只有求职者可以提问！', 2, 8);
			return false;
		}
		checkCode('vcode_imgs');
		var msgLayer = layer.open({

			type: 1,
			title: '我要提问',
			closeBtn: 1,
			border: [10, 0.3, '#000', true],
			area: ['auto', 'auto'],
			content: $("#showmessage")
		});
	} else {
		showlogin(1);
	}

}

//  微信扫码查看联系方式
function wxscanshowtel(id) {
	$('#tel_wxqrcode').html('<img src="index.php?c=comapply&a=telQrcode&id=' + id + '" width="120" height="120">');
	$.layer({
		type: 1,
		title: '微信扫码查看联系方式',
		closeBtn: [0, true],
		offset: ['100px', ''],
		border: [10, 0.3, '#000', true],
		area: ['300px', '320px'],
		page: {
			dom: "#telQrcodeBox"
		}
	});
}

// 推荐太多
function recommendToMuch(maxNum) {
	layer.msg('每天最多推荐' + maxNum + '次职位/简历！', 2, 8);
}

// 检查上一次推荐职位、简历的时间间隔
function recommendInterval(uid, url, interval) {
	var ajaxUrl = weburl + "/index.php?m=ajax&c=ajax_recommend_interval";
	layer.load('执行中，请稍候...', 0);
	$.post(ajaxUrl, {
		uid: uid
	}, function (data) {
		layer.closeAll('loading');
		data = eval('(' + data + ')');
		if (data.status == 0) {
			window.location = url;
			// window.open(url);
		} else {
			layer.msg(data.msg, 3, 8);
		}
	});
}

function applyjobuid(jobid) {
	$.layer({
		type: 2,
		fix: false,
		maxmin: false,
		shadeClose: true,
		title: '完善简历',
		area: ['760px', '680px'],
		iframe: {
			src: weburl + "/index.php?m=job&c=comapply&a=applyjobuid&jobid=" + jobid
		}
	})
}

function fastsuccess() {
	layer.closeAll();
	$.layer({
		type: 1,
		fix: false,
		maxmin: false,
		shadeClose: true,
		title: '简历投递成功',
		closeBtn: [0, true],
		area: ['550px', '500px'],
		page: {
			dom: "#fastsuccess"
		},
		close: function () {
			window.location.reload();
		}
	})
}
