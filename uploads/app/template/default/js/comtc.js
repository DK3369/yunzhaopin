/* PC会员中心 - 应聘简历 - 查看联系方式 */
function isDownResume(eid, url, num, type){
	var i = layer.confirm('您还可以查看'+num+'份简历，是否查看？', {btn: ['查看']}, function() {
		layer.close(i);
		downResume(eid,url, type);
	});
}
// 企业查看简历前先判断简历状态是否为不公开
function com_lookresume_check(eid, status,state,param={}){
    if (status == 2) {// 简历状态为不公开
        layer.msg('该简历为非开放状态！',2,8);
        return false;
    } else if (status == 3) {// 简历状态为仅对投递企业开放
        $.get("index.php?c=hr&act=everApplied&eid="+eid+"&state="+state, function(data) {
            var data = eval('(' + data + ')');
            if (data.code == 400) {
                layer.msg('该简历仅对投递企业开放',2,8);
                return false;
			} else {
                com_lookresume(eid,state,param)
			}
        })
	} else {
        com_lookresume(eid,state,param)
	}
}
function com_lookresume(eid,state,param={}){
	var from = '';
	if(typeof param.from!=='undefined'){
		from = param.from;
	}
    $('body').css('overflow-y','hidden');
    $.layer({
        type: 2,
        shadeClose: true,
        title: '简历详情',
        area: ['880px', ($(window).height() - 130) +'px'],
        iframe: {src: `index.php?c=hr&act=resumeInfo&eid=${eid}&state=${state}&from=${from}`},
        close: function(){
        	if(from != 'rck'){
        		window.location.reload();
        	}
            $('body').css('overflow-y','auto');
        }
    });
}
function downResume(eid, url, drtype) {

	$.post(url, {eid : eid}, function(data) {

		var data 	=  eval('(' + data + ')');
		var status 	=  data.status; 		// 返回状态
		var type 	=  data.type; 			// 简历类型

		if (status == 1) {

			var msgList		=	data.msgList;
			var	msg			=	msgList.join('');
			var down		=	data.down;	// 下载简历提示语句

			if(down	==	1){
				if(type == 2){
					$('#tips_way').html('<div class="yun_prompt_release_tip">以下条件尚未满足，暂时无法查看联系方式，请按顺序完成：</div>');
				}else{
					$('#tips_way').html('<div class="yun_prompt_release_tip">以下条件尚未满足，暂时无法查看联系方式，请按顺序完成：</div>');
				}
			}

			$("#msgList").html(msg);

			var msgLayer	=	layer.open({

				type		:	1,
				title		:	'温馨提示',
				closeBtn	:	1,
				border		: 	[10, 0.3, '#000', true],
				area		: 	['auto', 'auto'],
				content		: 	$("#jobcheck")
			});

		} else if (status == 2) {

			$('#eid').val(eid);
			server_single('downresume',data.jifen,data.price);
			firstTab();
			var msglayer = layer.open({
				type : 1,
				title : '查看联系方式',
				closeBtn : 1,
				border : [ 10, 0.3, '#000', true ],
				area : [ '810px', 'auto' ],
				content : $("#tcmsg"),
				cancel:function(){
					window.location.reload();
				}
			});

		} else if(status == 3){

			$('body').css('overflow-y','hidden');
		    if(drtype && drtype == 1){
		    	// 简历弹窗里点的
		    	if(data.msg){	
					layer.msg(data.msg , 2, 8);return false;
				}else if(data.html){
					$('#link_style_div').html(data.html);
					var msglayer = layer.open({
						type: 1,
						title: '联系方式',
						closeBtn: 1,
						border: [10, 0.3, '#000', true],
						area: ['auto', 'auto'],
						content: $("#link_style_div"),
						cancel:function(){
							window.location.reload();
						}
					});
				}
		    }else{
		    	// 列表里点的
		    	$.layer({
		    		type: 2,
		    		shadeClose: true,
		    		title: '简历详情',
		    		area: ['880px', ($(window).height() - 130) +'px'],
		    		iframe: {src: 'index.php?c=hr&act=resumeInfo&eid='+eid},
		    		close: function(){
		    			window.location.reload();
		    		}
		    	});
		    }

		} else {
			if (data.url) {
				layer.msg(data.msg, 2, 9, function() {
					window.location.href = data.url;
				});
			} else {
				layer.msg(data.msg, 2, 8);
			}
		}


	});
}

/* PC 会员中心邀请面试*/
function inviteResume(obj){

	var jobid = $(obj).attr("jobid");
	var r_uid = $(obj).attr("uid");	// 邀请人才UID

	if(jobid){
		$("#nameid").val(jobid);
		selects( jobid, 'name', $(this).attr("jobname") );
	}

	var jobtype	=	'';

	if($(obj).attr("uid")){$("#uid").val($(obj).attr("uid"));}
	if($(obj).attr("username")){$("#username").val($(obj).attr("username"));}
	if($(obj).attr("jobtype")){jobtype=$(obj).attr("jobtype");}

	loadlayer();

	//判断是否达到每天最大操作次数
	$.post(weburl + '/index.php?m=ajax&c=ajax_day_action_check', {'type' : 'interview'}, function(data){

		layer.closeAll('loading');

		data = eval('(' + data + ')');

		if(data.status == -1){

			layer.msg(data.msg, 2, 8);

		}else if(data.status == 1){

			$.post(weburl+"/index.php?m=ajax&c=indexajaxresume",{show_job:1,jobid:jobid,jobtype:jobtype, ruid:r_uid},function(data){

				var data	=	eval('('+data+')');

				var status	=	data.status;

				if(data.jobname){ $("#name").val(data.jobname); }
				if(data.linkman){ $("#linkman").val(data.linkman); }
				if(data.linktel){ $("#linktel").val(data.linktel); }
				if(data.address){ $("#address").val(data.address); }

				if(status == 1){	//	没有职位

					var msgList		=	data.msgList;
					var	msg			=	msgList.join('');
					var	invite		=	data.invite;

					if(invite	==	1){
						$('#tips_way').html('<div class="yun_prompt_release_tip">以下条件尚未满足,暂时无法邀请面试，请按顺序完成：</div>');
					}

					$("#msgList").html(msg);

					var msgLayer	=	layer.open({

						type		:	1,
						title		:	'温馨提示',
						closeBtn	:	1,
						border		: 	[10, 0.3, '#000', true],
						area		: 	['auto', 'auto'],
						content		: 	$("#jobcheck")
					});

				}else if(status == 2){	//	购买服务

					server_single('invite');
					firstTab();
					var msglayer = layer.open({
						type: 1,
						title: '邀请面试',
						closeBtn: 1,
						border: [10, 0.3, '#000', true],
						area: ['810px', 'auto'],
						content: $("#tcmsg"),
						cancel:function(){
							window.location.reload();
						}
					});


				}else if(status == 3){	// 邀请面试
					var msglayer = layer.open({
						type: 1,
						title: '邀请面试',
						closeBtn: 1,
						border: [10, 0.3, '#000', true],
						area: ['auto', 'auto'],
						content: $("#job_box"),
						cancel:function(){
							window.location.reload();
						}
					});

				}else{

					if(data.login){

						showlogin(data.login);
					}else{

						layer.msg(data.msg , 2, 8);return false;
					}
				}
			});
		}
	});
}

function refreshAllJob(name, num, rtype, Paused){
	if(Paused == 1){

		layer.msg('当前账户会员权益已暂停，请联系客服开启服务~', 2, 8, function(){
			window.location.reload();
		})
		return false;
	}
	var chk_value = [];
	var jobnums = 0;
	var id	= '';

	$('input[name="' + name + '"]:checked').each(function() {
		chk_value.push($(this).val());
		id	=	id+','+$(this).val();
		jobnums++;
	});
	id	=	id.substring('1');

	if(parseInt(num) > parseInt(jobnums) && rtype == 1){
		var msg		=	'本次刷新'+jobnums+'个在招职位，将扣除'+jobnums+'份套餐刷新量，当前刷新职位套餐余量' + num + '份，是否继续？'
		layer.open({
			title	:	[ '温馨提示', 'background-color: #FF4351; color:#fff;' ],
			content	:	msg,
			btn		:	['确定','取消'],
			yes:function(index){
				layer.close(index);
				sxJob(id);
			}
		});
	}else{
		sxJob(id);
	}
}

function refreshJob(id, num, rtype, Paused) {

	if(Paused == 1){

		layer.msg('当前账户会员权益已暂停，请联系客服开启服务~', 2, 8, function(){
			window.location.reload();
		})
		return false;
	}

	if (id == '') {
		layer.msg('您暂无发布中的职位！', 2, 8, function(){
			window.location.reload();
		});
		return false;

	} else{
		var chk_value = id.split(',');
		var jobnums	  = chk_value.length;

		if(parseInt(num) > parseInt(jobnums) && rtype == 1){
			var msg		=	'本次刷新'+jobnums+'个在招职位，将扣除'+jobnums+'份套餐刷新量，当前刷新职位套餐余量' + num + '份，是否继续？'
			layer.open({
				title	:	[ '温馨提示', 'background-color: #FF4351; color:#fff;' ],
				content	:	msg,
				btn		:	['确定','取消'],
				yes:function(index){
					layer.close(index);
					sxJob(id);
				}
			});
		}else{
			sxJob(id);
		}
	}
}

function sxJob(id){
	loadlayer();
	var chk_value = id.split(',');

	$.post(weburl + '/index.php?m=ajax&c=ajax_day_action_check', {'type' : 'refreshjob'}, function(data) {
		layer.closeAll('loading');
		data = eval('(' + data + ')');

		if (data.status == -1) {

			layer.msg(data.msg, 2, 8);
		} else if (data.status == 1) {

			var sxnum	=	data.sxnum;
			var num		=	chk_value.length;

			if(sxnum < num){
				layer.msg('今日还能刷新职位'+sxnum+'份', 2, 8);
				return false;
			}

			$('#jobids').val(chk_value);

			var jobids	=	$('#jobids').val();

			var ajaxUrl = weburl + "/member/index.php?c=job&act=refresh_job";

			$.post(ajaxUrl, {jobid : jobids}, function(data) {

				data = eval('(' + data + ')');

				if (data.error == 1) {

					layer.msg(data.msg, 2, 9, function() {
						window.location.reload();
					});

				} else if (data.error == 2) {

					server_single('sxjob',data.jifen,data.price);
					firstTab();
					var msglayer = layer.open({
						type : 1,
						title : '刷新职位',
						closeBtn : 1,
						border : [ 10, 0.3, '#000', true ],
						area : [ '810px', 'auto' ],
						content : $("#tcmsg"),
						cancel:function(){
							window.location.reload();
						}
					});

				} else {
					if (data.url) {
						layer.msg(data.msg, 2, 9, function() {
							window.location.href = data.url;
						});
					} else {
						layer.msg(data.msg, 2, 8);
					}
				}
			});
		}
	});

}

/*兼职刷新 */
function refreshPart(id, name) {

	$.post(weburl + '/index.php?m=ajax&c=ajax_day_action_check', {'type' : 'refreshjob'}, function(data) {

		data = eval('(' + data + ')');

		if (data.status == -1) {

			layer.msg(data.msg, 2, 8);
		} else if (data.status == 1) {

			if (id) {
				var chk_value = id;
				var i = 1;
			} else {
				var chk_value = [];
				var i = 0;
				$('input[name="' + name + '"]:checked').each(function() {
					chk_value.push($(this).val());
					i++;
				});
				if(chk_value.length == 0){
					layer.msg('请选择要刷新的兼职', 2, 8);
					return false;
				}
			}
			$('#partids').val(chk_value);

			var partids	=	$('#partids').val();

			var ajaxUrl = weburl + "/member/index.php?c=part&act=refresh_part";

			$.post(ajaxUrl, {partid : partids}, function(data) {

				data = eval('(' + data + ')');

				if (data.error == 1) {

					layer.msg(data.msg, 2, 9, function() {
						window.location.reload();
					});

				} else if (data.error == 2) {

					server_single('sxpart',data.jifen,data.price);
					firstTab();
					var msglayer = layer.open({
						type : 1,
						title : '刷新兼职',
						closeBtn : 1,
						border : [ 10, 0.3, '#000', true ],
						area : [ '810px', 'auto' ],
						content : $("#tcmsg"),
						cancel:function(){
							window.location.reload();
						}
					});

				} else {
					if (data.url) {
						layer.msg(data.msg, 2, 9, function() {
							window.location.href = data.url;
						});
					} else {
						layer.msg(data.msg, 2, 8);
					}
				}
			});
		}
	});
}

/* 预约刷新 */
function reserveRefreshJob(job_id, status, space, end, s_time, e_time) {
	$("#reserve_id").val(job_id);
	if(status != ''){
		$("input[name=status][value='1']").attr("checked", status == '1' ? true : false);
		$("input[name=status][value='2']").attr("checked", status == '2' ? true : false);
	}else{
		var ajaxUrl = weburl+"/member/index.php?c=job&act=getRefresh";
		let pData = {job_id:job_id};
		let refreshStatus;
		$.ajaxSettings.async=false
		$.post(ajaxUrl, pData, function(data) {
			data = eval('(' + data + ')');
			if (data.error == 0) {
				refreshStatus = data.data.refreshStatus?data.data.refreshStatus:'';
				if (refreshStatus == 0){
					space = '60'
					$("input[name=status][value='1']").attr("checked", false);
					$("input[name=status][value='2']").attr("checked", true);
				}else{
					if(data.data.interval){
						space = data.data.interval== 0?'60':data.data.interval;
					}
					if (data.data.s_time){
						s_time = data.data.s_time;
					}
					if (data.data.e_time){
						e_time = data.data.e_time;
					}
					$("input[name=status][value='1']").attr("checked", false);
					$("input[name=status][value='2']").attr("checked", true);
				}

			}
		});

	}

	$('#intervalId').find('option').filter(function() {
		if ($(this).val() === '' || $(this).val() == 1) {
			return false;
		} else {
			return parseInt($(this).val()) < sy_reserve_refresh_interval;
		}
	}).remove();

	if(space != ''){
		var intervalArr = ['60', '120', '180', '240', '300', '360', '420', '480'];
		intervalArr = intervalArr.filter(function(value) {
			return parseInt(value) >= sy_reserve_refresh_interval;
		})

		if (intervalArr.indexOf(space) >= 0){

			$('#intervalDiv').hide();
			$('#intervalId').find("option[value="+space+"]").attr("selected", true);
		}else{

			$('#intervalDiv').show();
			$('#intervalId').find("option[value=1]").attr("selected", true);
		}
		$("#interval").val(space);
	}else{
		$("#interval").val('');
	}
	if(end != '' && end != '不限'){
		$("#end_time").val(end);
	}else{
		$("#end_time").val('');
	}
	if(s_time != '' && s_time != '不限'){
		$("#s_time").val(s_time);
	}else{
		$("#s_time").val('');
	}
	if(e_time != '' && e_time != '不限'){
		$("#e_time").val(e_time);
	}else{
		$("#e_time").val('');
	}
	$.layer({
		type : 1,
		title : '预约刷新',
		closeBtn : [ 0, true ],
		border : [ 10, 0.3, '#000', true ],
		area : [ '620px', 'auto'],
		page : {
			dom : '#reserve_box'
		}
	});
	form.render();
}

function reserveAllJob(name) {
	var chk_value = [];
	$('input[name="' + name + '"]:checked').each(function() {
		chk_value.push($(this).val());
	});
	$("#reserve_id").val(chk_value.join());

	$('#intervalId').find('option').filter(function() {
		if ($(this).val() === '' || $(this).val() == 1) {
			return false;
		} else {
			return parseInt($(this).val()) <= sy_reserve_refresh_interval;
		}
	}).remove();

	$.layer({
		type : 1,
		title : '预约刷新',
		closeBtn : [ 0, true ],
		border : [ 10, 0.3, '#000', true ],
		area : [ '620px', 'auto'],
		page : {
			dom : '#reserve_box'
		}
	});
	form.render();
}

function autoJobPromote(id, etime, day) {
	$('#auto_promote_etime').html(etime);
	$('#auto_promote_day').html(day);
	var html = "<input type='button' value='取消' onClick=\"layer.closeAll();\" class='set_tips_bth'>\n" + "<input type='button' value='确定' onClick=\"jobPromote(" + id + ", '', 5);\" class='set_tips_bth'>";

	$('#auto_btn').html(html);

	$.layer({
		type: 1,
		title: '自动刷新',
		closeBtn: [0, true],
		border: [10, 0.3, '#000', true],
		area: ['350px', 'auto'],
		page: {
			dom: '#auto_promote_div'
		}
	});
}

/**
 * 职位推广关闭提示
 * @param id
 * @param etime
 * @param day
 * @param type
 */
function closeJobPromote(id, etime, day, type) {

	$('#close_promote_jobid').val(id);	// 推广职位ID

	if (etime != '') {					// 显示推广结束日期
		if (type == 1) {
			$('#close_peomote_etime_div').html('目前置顶职位的到期时间为：<span id="close_promote_etime" class="set_tips_n">' + etime + '</span>');
		} else if (type == 2) {
			$('#close_peomote_etime_div').html('目前推荐职位的到期时间为：<span id="close_promote_etime" class="set_tips_n">' + etime + '</span>');
		} else if (type == 3) {
			$('#close_peomote_etime_div').html('目前紧急招聘职位的到期时间为：<span id="close_promote_etime" class="set_tips_n">' + etime + '</span>');
		} else if (type == 4) {
			$('#close_peomote_etime_div').html('目前推荐兼职到期时间为：<span id="close_promote_etime" class="set_tips_n">' + etime + '</span>');
		}
		$('#close_peomote_etime_div').show();
	}
	if (type == 1) {

		var title = '关闭职位置顶';
		$('#close_promote_tc').html('您的职位置顶剩余：<span class="set_tips_n close_promote_tc">' + day + '</span>天');
		$('#close_promote_title').html(title);
		$('#close_promote_type').val('top');
	} else if (type == 2) {

		var title = '关闭职位推荐';
		$('#close_promote_tc').html('您的职位推荐剩余：<span class="set_tips_n close_promote_tc">' + day + '</span>天');
		$('#close_promote_title').html(title);
		$('#close_promote_type').val('rec');
	} else if (type == 3) {

		var title = '关闭职位紧急招聘';
		$('#close_promote_tc').html('您的职位紧急招聘剩余：<span class="set_tips_n close_promote_tc">' + day + '</span>天');
		$('#close_promote_title').html(title);
		$('#close_promote_type').val('urgent');
	} else if (type == 4) {

		var title = '关闭兼职推荐';
		$('#close_promote_tc').html('您的职位推荐剩余：<span class="set_tips_n close_promote_tc">' + day + '</span>天');
		$('#close_promote_title').html(title);
		$('#close_promote_type').val('recpart');
	}

	$.layer({
		type: 1,
		title: title,
		closeBtn: [0, true],
		border: [10, 0.3, '#000', true],
		area: ['350px', 'auto'],
		page: {
			dom: '#close_promote_div'
		}
	});
}
/**
 * 职位推广关闭请求
 */
function setJobPromoteClose() {

	var jobid		=	$("#close_promote_jobid").val();
	var type		= 	$("#close_promote_type").val();

	loadlayer();

	$.post('index.php?c=job&act=closeJobPromote', {jobid : jobid, type : type}, function(data) {

		layer.closeAll();

		var data 	= 	eval('(' + data + ')');
		var errcode = 	data.errcode;

		var msg  	= 	data.msg;

		if(errcode == 7){

			layer.msg(msg, 2, 8);
			return false;
		}else{

			layer.msg(msg, 2, errcode, function() {
				window.location.reload();
			})
		}
	});
}

/* 职位推广：置顶、推荐、紧急招聘 */
function jobPromote(id, etime, type){

	loadlayer();

	$.post('index.php?c=job&act=jobPromote',{ type : type}, function(data){

		layer.closeAll('loading');

		var data	= 	eval('(' + data + ')');
		var status	=	data.status;
		var price   = 	data.price;
		// 套餐量充足
		if(status == 1){
			var num = data.num;
			$('#promote_jobid').val(id);	// 推广职位ID

			if(etime !=''){					// 显示推广结束日期
				if(type == 1){
					$('#peomote_etime_div').html('目前置顶职位的到期时间为：<span id="promote_etime" class="set_tips_n">'+etime+'</span>');
				}else if(type == 2){
					$('#peomote_etime_div').html('目前推荐职位的到期时间为：<span id="promote_etime" class="set_tips_n">'+etime+'</span>');
				}else if(type == 3){
					$('#peomote_etime_div').html('目前紧急招聘职位的到期时间为：<span id="promote_etime" class="set_tips_n">'+etime+'</span>');
				}else if(type == 4){
					$('#peomote_etime_div').html('目前推荐兼职到期时间为：<span id="promote_etime" class="set_tips_n">'+etime+'</span>');
				}else if(type == 5){
					$('#peomote_etime_div').html('目前职位自动刷新到期时间为：<span id="promote_etime" class="set_tips_n">'+etime+'</span>');
				}
				$('#peomote_etime_div').show();
			}
			if(type == 1){

				var title	=	'职位置顶';
				if (price == 0){
					$('#promote_tc').html('<font color="red">限时福利：</font>置顶职位金额 <font color="red">0</font> 元');
				} else {
					$('#promote_tc').html('您的职位置顶套餐剩余：<span class="set_tips_n promote_tc">'+num+'</span>天');
				}
				$('#promote_type').val('top');
			}else if(type == 2){

				var title	=	'职位推荐';
				if (price == 0){
					$('#promote_tc').html('<font color="red">限时福利：</font>推荐职位金额 <font color="red">0</font> 元');
				} else {
					$('#promote_tc').html('您的职位推荐套餐剩余：<span class="set_tips_n promote_tc">' + num + '</span>天');
				}
				$('#promote_type').val('rec');
			}else if(type == 3){

				var title	=	'职位紧急招聘';
				if (price == 0){
					$('#promote_tc').html('<font color="red">限时福利：</font>紧急招聘金额 <font color="red">0</font> 元');
				} else {
					$('#promote_tc').html('您的职位紧急招聘套餐剩余：<span class="set_tips_n promote_tc">' + num + '</span>天');
				}
				$('#promote_type').val('urgent');
			}else if(type == 4){

				var title	=	'兼职推荐';
				if (price == 0){
					$('#promote_tc').html('<font color="red">限时福利：</font>推荐兼职金额 <font color="red">0</font> 元');
				} else {
					$('#promote_tc').html('您的职位推荐套餐剩余：<span class="set_tips_n promote_tc">' + num + '</span>天');
				}
				$('#promote_type').val('recpart');
			}else if(type == 5){

				var chk_value = [];
				if (isNaN(id)) {
					// 批量刷新职位需要的id
					$('input[name="' + id + '"]:checked').each(function() {
						chk_value.push($(this).val());
					});
				} else {
					chk_value.push(id);
				}
				if(chk_value.length > 0){
					$("#promote_jobid").val(chk_value);
				}else{
					layer.msg('请选择要自动刷新的职位',2,8);return false;
				}

				var title	=	'自动刷新';
				$('#promote_tc').html('<font color="red">限时福利：</font>自动刷新金额 <font color="red">0</font> 元');
				$('#promote_type').val('autojob');
			}

			$.layer({
				type : 1,
				title : title,
				closeBtn : [ 0, true ],
				border : [ 10, 0.3, '#000', true ],
				area : [ '350px', 'auto'],
				page : {
					dom : '#promote_div'
				}
			});

		}else if(status == 2){
			$('#promoteid').val(id);// 推广职位ID
			var server,title;
			if(type == 1){
				server = 'jobtop';
				title = '职位置顶';
				$("#xdays").attr('placeholder','自定义置顶天数');
			} else if(type == 2){
				server = 'jobrec';
				title = '职位推荐';
				$("#xdays").attr('placeholder','自定义推荐天数');
			} else if(type == 3){
				server = 'joburgent';
				title = '紧急招聘';
				$("#xdays").attr('placeholder','自定义紧急天数');
			}else if(type == 4){
				server = 'partrec';
				title = '兼职推荐';
				$("#xdays").attr('placeholder','自定义推荐天数');
			}else if(type == 5){
				// 自动刷新没有套餐,只有单项购买
				$("#tcbox").hide();
				$("#rating_4").show();
				server = 'autojob';
				title = '职位自动刷新';
				$("#xdays").attr('placeholder','自定义刷新天数');
				// 职位自动刷新又批量处理
				var chk_value = [];
				if (isNaN(id)) {
					// 批量刷新职位需要的id
					$('input[name="' + id + '"]:checked').each(function() {
						chk_value.push($(this).val());
					});
				} else {
					chk_value.push(id);
				}
				if(chk_value.length > 0){
					$("#jobautoids").val(chk_value);
				}else{
					layer.msg('请选择要自动刷新的职位',2,8);return false;
				}
			}

			server_single(server, data.jifen, data.price);
			if (type!=5) {
				firstTab();
			}
			var msglayer = layer.open({
				type : 1,
				title : title,
				closeBtn : 1,
				border : [ 10, 0.3, '#000', true ],
				area : [ '810px', 'auto'],
				content : $('#tcmsg'),
				cancel:function(){
					if(type == 4){
						window.location.href = "index.php?c=partok&w=1";;
					}else{
						window.location.href = "index.php?c=job&w=1";
					}
				}
			});

		}else{
			layer.msg(data.msg,2,8);
		}
	});
}

/* 套餐内设置推广天数检测 */
function checkPromoteDay(){

	var promote_tc	=	$('.promote_tc').html();
	var promote_day	=	$('#promote_day').val();

	if(parseInt(promote_tc) < parseInt(promote_day)){
		$('#promote_day').val(parseInt(promote_tc));
	}
}

/* 职位推广设置 */
function setJobPromote() {

	var jobid		=	$("#promote_jobid").val();
	var days		= 	$("#promote_day").val();
	var type		= 	$("#promote_type").val();

	if (days == '') {

		layer.msg('请填写职位推广天数！', 2, 8);
		return false;
	}

	loadlayer();

	$.post('index.php?c=job&act=setJobPromote', {jobid : jobid, type : type, days : days}, function(data) {

		layer.closeAll('loading');

		var data 	= 	eval('(' + data + ')');
		var errcode = 	data.errcode;

		var msg  	= 	data.msg;

		if(errcode == 7){

			layer.msg(msg, 2, 8);
			return false;
		}else{
			layer.closeAll();
			layer.msg(msg, 2, errcode, function() {
				window.location.reload();
			})
		}
	});
}

/* 职位推广，选择天数 */
function checkRadio(value,type){

	if(value == ''){
		$("#price_int").text(0);
		$("#single_order_price").text(0);
		return false;
	}

	var price,
		server = $("#pay_server").val(),
		pro = $("#integral_pro").val(),
		online = $("#integral_online").val();
	if(server == 'partrec' || server == 'jobrec'){
		price = $("#server_com_recjob").val();
	}else if (server == 'joburgent'){
		price = $("#server_com_urgent").val();
	}else if (server == 'jobtop'){
		price = $("#server_integral_job_top").val();
	}else if (server == 'autojob'){
		price = $("#server_job_auto").val();
	}

	// type=radio 点击单选框 type=input 自定义天数输入
	if(type == 'radio'){
		$("#xdays").val('');
		$("input[value='"+value+"']").attr("checked","checked");
	}
	$('#server_integral_dk').val('');	//	积分清空

	$('#yhq_name_n').html('请选择');

	$("#server_pay_div").show();

	$("#server_integral_div").hide();

	$(".price_single_member").show();

	if(online == 2 || online == 1 || online==4){

		var singleJobPrice = accMul(parseFloat(value), parseFloat(price));
		if(server == 'autojob'){
			var autoids = $("#jobautoids").val();
			var idarr = autoids.split(',');
			var num	= idarr.length;
			var singleJobPrice	=	accMul(parseInt(num), singleJobPrice);
		}
		$('#price_int').text(singleJobPrice);
		$('#single_order_price').text(singleJobPrice);

	}else if(online == 3){

		var singleJobPrice = accMul(parseFloat(value), parseFloat(price));
		var jifen = accMul(parseFloat(singleJobPrice), parseInt(pro))
		if(server == 'autojob'){
			var autoids = $("#jobautoids").val();
			var idarr = autoids.split(',');
			var num	= idarr.length;
			jifen = accMul(parseInt(num), jifen);
		}
		$('#price_int').text(jifen);

		var integral =	$('#user_integral_all').val();

		var minJf	=	$("#integral_min").val();

		if(parseInt(jifen) > parseInt(integral)){

			$('#integral_buy').hide();
			$('#pay_integral_buy').show();
			if(parseInt(jifen) < parseInt(minJf)){

				$('#integral_int').val(minJf);
				$('#price_int').text(accDiv(minJf, pro));
			}else{
				$('#integral_int').val(jifen);
				$('#price_int').text(accDiv(jifen, pro));
			}

			$("#single_integral").text(jifen);
			$("#single_order_price").text(price);
		}else{
			$('#single_integral').text(jifen);
			$('#pay_integral_buy').hide();
			$('#integral_buy').show();
		}
	}
}

function radioClean(obj){

	$("input[name='days']").attr("checked",false);

	$(".price_single_member").show();

	if(obj == ''){
		$('#single_integral').html(0);
		$('#single_order_price').html(0);
		$('#integral_int').val('');
		$('#server_integral_dk').val('');
		$('#price_int').html(0);

		
	}
}

/* 积分抵扣，单项购买 */
function checkIntegralDKSingleJob(integral, integral_pro, type) {

	var integral_dk		= 	$("#integral_dk_job_"+type).val();	// 	抵扣输入积分

	var single_price	= 	$("#single_"+type+"_price").text();	//	需支付金额金额

	

	var integral_need	= 	accMul(parseFloat(single_price), parseInt(integral_pro)); 	// 	所需积分

	if(parseInt(integral_need) == 0){

		var integral_need	=	1;
	}else if(integral_need > parseInt(integral_need)){

		var integral_need 	= 	accAdd(parseInt(integral_need), 1);
	}
	if(integral_dk){

		if(parseInt(integral_dk) > 0){
			$("#integral_dk_job_"+type).val(parseInt(integral_dk));
		}

		if(parseInt(integral) >= parseInt(integral_need)) { 					// 	拥有积分充足

			if(parseInt(integral_dk) > parseInt(integral_need)) { 				// 	输入抵扣积分超过所需积分

				$("#integral_dk_job_"+type).val(parseInt(integral_need));		//	抵扣积分变更最大所需积分

				var order_price	= accSub(parseFloat(single_price), accDiv(integral_need, parseInt(integral_pro))); 			// 	抵扣后金额  0

			} else {
				var order_price	= accSub(parseFloat(single_price), accDiv(parseInt(integral_dk), parseInt(integral_pro)));	//	抵扣积分后所需金额

			}

		} else {																//	拥有积分不充足

			if(parseInt(integral_dk) > parseInt(integral)) {					//	抵扣所有积分

				$("#integral_dk_job_"+type).val(parseInt(integral));

				var order_price	= accSub(parseFloat(single_price), accDiv(integral, parseInt(integral_pro)));		//	抵扣积分后所需金额

			} else {

				var order_price	= accSub(parseFloat(single_price), accDiv(integral_dk, parseInt(integral_pro)));			//	抵扣积分后所需金额
			}

		}

		/* 根据抵扣后金额，判断支付方式 */
		if(order_price <= 0) {

			$("#single_"+type+"_order_price").html('<em class="single_payprice_n">0</em> 元');
			$("#"+type+"_pay_div").hide();
			$("#"+type+"_integral_div").show();
		} else {

			$("#single_"+type+"_order_price").html('<em class="single_payprice_n">'+order_price+'</em> 元');
			$("#"+type+"_pay_div").show();
			$("#"+type+"_integral_div").hide();
		}
	}else{
		$("#single_"+type+"_order_price").html('<em class="single_payprice_n">'+single_price+'</em> 元');

		$("#"+type+"_pay_div").show();
		$("#"+type+"_integral_div").hide();
	}
}

/* 积分模式，积分不足的情况，充值积分输入检测 */
function checkIntegralPaySingleJobTg(type,integral, integral_min, integral_pro){

	var single_integral	=	$('#single_'+type+'_integral').html(); 					//	单项购买所需积分
	var integral		=	integral;												//	目前拥有积分

	var integral_need	=	accSub(parseInt(single_integral), parseInt(integral));	//	单选购买减去账号积分，还需要积分数量
	var integral_min	=	integral_min;											//	站点最低充值积分
	var integral_pro	=	integral_pro;											//	站点积分比例

	if(parseInt(integral_need) < parseInt(integral_min)){

		var	minJifen	=	integral_min;
	}else{

		var	minJifen	=	integral_need;
	}

	var integral_int	=	$('#integral_int').val();

	if(parseInt(integral_int) < parseInt(minJifen)){

		$('#integral_int').val(minJifen);

		$('#price_int').html(accDiv(minJifen, integral_pro));

	}else if(integral_int != ''){

		$('#price_int').html(accDiv(parseInt(integral_int), integral_pro));

	} else{
		$('#integral_int').val(minJifen);
		$('#price_int').html(accDiv(parseInt(minJifen),integral_pro));
	}
}
