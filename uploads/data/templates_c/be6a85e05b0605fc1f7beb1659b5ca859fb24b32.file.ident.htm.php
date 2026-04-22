<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:35:35
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/ident.htm" */ ?>
<?php /*%%SmartyHeaderCode:83477362169e896670a0b56-89154046%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'be6a85e05b0605fc1f7beb1659b5ca859fb24b32' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/ident.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '83477362169e896670a0b56-89154046',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'config' => 0,
    'backurl' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e896670c11a3_03241090',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e896670c11a3_03241090')) {function content_69e896670c11a3_03241090($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/jquery.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>

<style type="text/css">
	body {
		background: #fff
	}

	.header_h {
		display: none;
	}
</style>
<div class="login_tit"> <a class="hd-lbtn"
		href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>"><i
			class="header_top_l"></i></a>请选择您的身份
	<div class="regok_tit_p">之后你可以在“我的-设置”中切换</div>
</div>
<div class="regok_box">
	<ul>
		<li><a href="javascript:void(0);" onclick="chooseIdent('<?php echo smarty_function_url(array('m'=>'wap','c'=>'register','a'=>'ident','usertype'=>1),$_smarty_tpl);?>
')">
			<div class="regok_box_center">
				<div class="regok_box_logo">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/genius.png" alt="" width="100%" height="100%">
				</div>
				<div class="regok_box_user">
					<p>我要找工作</p>
					<span>我是求职者，我要找工作</span>
				</div>
				<div class="regok_box_nav">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_add.png" alt="" width="100%" height="100%" >
				</div>
			</div>
			</a></li>
		<li><a href="javascript:void(0);" onclick="chooseIdent('<?php echo smarty_function_url(array('m'=>'wap','c'=>'register','a'=>'ident','usertype'=>2),$_smarty_tpl);?>
')">
			<div class="regok_box_center">
				<div class="regok_box_logo">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/company.png" alt="" width="100%" height="100%">
				</div>
				<div class="regok_box_user">
					<p>我要招人</p>
					<span>我是企业，我要招人</span>
				</div>
				<div class="regok_box_nav">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/dredge_add.png" alt="" width="100%" height="100%" >
				</div>
			</div>
			</a></li>

	</ul>

</div>
</body>
<?php echo '<script'; ?>
>
var identFlag;
function chooseIdent(url){
	// 节流处理：在一定时间内，只能触发一次
	if (!identFlag) {
		identFlag = true;
		setTimeout(function(){
			identFlag = false;
		}, 3000);
	}else{
		return false;
	}
	// 处理浏览器历史记录，防止可以返回注册页面
	window.history.replaceState({}, "", wapurl+"index.php");
	window.location.href = url;
}
<?php echo '</script'; ?>
>
</html><?php }} ?>
