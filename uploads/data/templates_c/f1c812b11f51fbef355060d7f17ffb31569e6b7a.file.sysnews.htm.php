<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:41:06
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/sysnews.htm" */ ?>
<?php /*%%SmartyHeaderCode:147834894369e897b2203703-54733982%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'f1c812b11f51fbef355060d7f17ffb31569e6b7a' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/sysnews.htm',
      1 => 1703143957,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '147834894369e897b2203703-54733982',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'config' => 0,
    'config_wapdomain' => 0,
    'isweixin' => 0,
    'usertype' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e897b22232e2_83821442',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e897b22232e2_83821442')) {function content_69e897b22232e2_83821442($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/member/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<style>
	body{ background-color: #fff;}
</style>
 <div class="chatnewcardbg">
     <div class="chatnewcardheader">消息</div> 
 	
 
	<!-- 滑动模块 -->
	<div class="chatnewcard">
	   <ul>
			<li data-url="invite" class="navigetali">
				<div class="card_logo">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/resume.png" alt="" width="100%" height="100%">
					<div id="wkyqnum" class="card_logo_circle" style="display: none;">
						<span id="wkyqnum_n" style="background: #f30; padding: 0.03rem 0.15rem;border-radius: 20px; color: #fff;"></span>
					</div>
				</div>
				<i class="card_word">面试通知</i>
			</li>
			<li data-url="sq" class="navigetali">
				<div class="card_logo">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/copy.png" alt="" width="100%" height="100%">
				</div>
				<i class="card_word">投递反馈</i>
			</li>
			<?php if ($_smarty_tpl->tpl_vars['config']->value['com_message']) {?>
			<li data-url="commsg" class="navigetali">
				<div class="card_logo">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/genius_consult.png" alt="" width="100%" height="100%">
					<div id="commsgnum" class="card_logo_circle" style="display: none;">
						<span id="commsgnum_n" style="background: #f30; padding: 0.03rem 0.15rem;border-radius: 20px; color: #fff;"></span>
					</div>
				</div>
				<i class="card_word">职位咨询</i>
			</li>
			<?php }?>
			<li data-url="sxnews" class="navigetali">
				<div class="card_logo">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/sixin.png" alt="" width="100%" height="100%">
					<div id="sxnum" class="card_logo_circle" style="display: none;">
						<span id="sxnum_n" style="background: #f30; padding: 0.03rem 0.15rem;border-radius: 20px; color: #fff;"></span>
					</div>
				</div>
				<i class="card_word">系统消息</i>
			</li>
		</ul>
	</div>
</div>
   
<!--  微信提示 -->
<div class="chatnewtipbox" id="gzhTsDiv" style="display: none;">
	<div class="chatnewtip" > 
		<i class="chatnewtipicon"></i>
		<div class="chatnewcard_gzhtip"> 
			<span> 开启微信通知，重要信息不错过</span>
			<span onclick="toSubscribe();" class="chatnewcard_gzhtip_set">  去设置></span>
		</div>	
	</div>
</div>
<!--  提示 -->
 
 <div class="chatnewcard_bgpd  ">
     
    
   	<!-- 页面整体对话框 -->
   	<div class="dialog_box none" id="sysChatbox">
	 
		<!-- 关注公众号弹框 -->
		<van-popup v-model="gzhShow" close-on-click-overlay="true" closeable="true" @close="closeGzhShow" round >
			<div class="gzh_gzbox">
				<div class="gzh_gzbox_n">关注公众号</div>
				<img :src="gzhUrl" />
				<div class="gzh_gzbox_p">长按识别二维码</div>
				<div class="gzh_gzbox_p">随时随地查找好工作</div>
			</div>
		</van-popup>
	</div>
</div>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/jquery.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/vue.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/lib/vant.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/phpyun_vant.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>
	var isWeiXin = '<?php echo $_smarty_tpl->tpl_vars['isweixin']->value;?>
',
			loopVal = null;
	var sysapp = new Vue({
		el: '#sysChatbox',
		data: {
			allchat: [],
			newchat: [],
			oldchat: [],
			chatCate: 'all',
			isWx: isWeiXin ? 1 : 2,
			gzhShow: false,
			gzhUrl: ''
		},
		created() {
			this.getSysmsg();
		},
		methods: {
			getSysmsg() {
				var self = this;
				$.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'msg','c'=>'sysnews'),$_smarty_tpl);?>
', {rand: Math.random()}, function (data) {
					var res = data.data;
					if (res.sxnum > 0) {
						$("#sxnum_n").text(res.sxnum);
						$("#sxnum").css('display', 'flex');
					}
					if (res.wkyqnum > 0) {
						$("#wkyqnum_n").text(res.wkyqnum);
						$("#wkyqnum").css('display', 'flex');
					}
					if (res.commsgnum > 0) {
						$("#commsgnum_n").text(res.commsgnum);
						$("#commsgnum").css('display', 'flex');
					}
					if (res.subscribe != 1){
						$('#gzhTsDiv').show();
						self.gzhUrl = res.gzhurl ? res.gzhurl : '';
					}
				}, 'json');
			}
		}
	});
	function toSubscribe() {
		if (sysapp.$data.isWx == 1) {

			sysapp.$data.gzhShow = true;
			gzhBind();
		} else {
			window.location.href = '<?php echo smarty_function_url(array('m'=>'wap','c'=>'wxconnect','login'=>1),$_smarty_tpl);?>
';
		}
	}

	var gzhConfirm = false,
			gzhBox = false;

	function isGzh() {
		$.post('index.php?c=isgzh', {rand: Math.random()}, function (res) {
			if (res.subscribe == 0 && !gzhConfirm) {
				gzhBox = true;
				sysapp.$data.gzhShow = true;
			} else if (res.subscribe == 2 && !gzhConfirm) {
				if (!gzhBox) {
					sysapp.$data.gzhShow = true;
					gzhBox = true;
				} else {
					gzhConfirm = true;
					sysapp.$data.gzhShow = false;
					showConfirm('本账号绑定的微信号，不是当前微信号，是否要换绑为当前微信号？', function () {
						wxBindChange();
					}, '否', '是');
				}
			} else if (res.subscribe == 1) {
				if (gzhBox) {
					window.location.reload();
				}
				$('#gzhTsDiv').hide();
				$('#gzhTsDiv').hide();
			}
		}, 'json');
	}

	function closeGzhShow() {
		clearTimeout(loopVal);
		loopVal = null;
	}

	function gzhBind() {
		loopVal = setInterval(function () {
			isGzh();
		}, 3000);
	}

	function wxBindChange() {
		showLoading();
		$.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'com','m'=>'index','c'=>'hbwx'),$_smarty_tpl);?>
', {rand: Math.random()}, function (res) {
			hideLoading();
			if (!res.error) {
				showToast(res.msg);
			}
		}, 'json');
	}
	var wapurl = '<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
',
			mine = {usertype: "<?php echo $_smarty_tpl->tpl_vars['usertype']->value;?>
"},
			apage = 0,
			npage = 0,
			opage = 0,
			newshow = false,
			oldshow = false;

	var needRefresh = window.sessionStorage.getItem("needRefresh");
	if (needRefresh != 'false' && needRefresh) {
		window.sessionStorage.setItem("needRefresh", false);
		location.reload();
	}
	$(function () {
		typeof fetchData !== 'undefined' && fetchData();
		// 消息页面-顶部导航点击
		$(".navigetali").on("click", function () {
			var url = $(this).attr('data-url');
			window.location.href = wapurl + 'member/index.php?c=' + url + '&chat=1';
		});
	});
<?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
