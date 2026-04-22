<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 19:01:05
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/header_cont.htm" */ ?>
<?php /*%%SmartyHeaderCode:93298134969e8aa71a7ab88-95193769%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '6c40beb89905f42c553950dd958829b63abcaa73' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/header_cont.htm',
      1 => 1700725939,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '93298134969e8aa71a7ab88-95193769',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'title' => 0,
    'keywords' => 0,
    'description' => 0,
    'wap_style' => 0,
    'config' => 0,
    'config_wapdomain' => 0,
    'wxapp' => 0,
    'backurl' => 0,
    'headertitle' => 0,
    'topplaceholder' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8aa71a91153_91925665',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8aa71a91153_91925665')) {function content_69e8aa71a91153_91925665($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><!DOCTYPE html PUBLIC "-//WAPFORUM//DTD XHTML Mobile 1.0//EN" "http://www.wapforum.org/DTD/xhtml-mobile10.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
	<head>
		<meta http-equiv="Content-Type" content="text/html;charset=utf-8" />
		<meta http-equiv="Cache-Control" content="no-cache" />
		<meta http-equiv="X-UA-compatible" content="IE=edge" />
		<title><?php echo $_smarty_tpl->tpl_vars['title']->value;?>
</title>
		<meta name="keywords" content="<?php echo $_smarty_tpl->tpl_vars['keywords']->value;?>
,wap" />
		<meta name="description" content="<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
" />
		<meta name="viewport"
			content="width=device-width, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0, user-scalable=no" />
		<meta http-equiv="Expires" content="0">
		<meta name="format-detection" content="telephone=no,email=no" />
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/flexible.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<link rel="stylesheet"
			href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/template/wap/css/css.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"
			type="text/css" />
		<link rel="stylesheet"
			href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/template/wap/css/base.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"
			type="text/css" />
		<link rel="stylesheet"
			href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/template/wap/css/yunwap.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"
			type="text/css" />
		<link href="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/lib/index.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"
			rel="stylesheet" />

		<!--wxapp 引用页面控制，不需要此返回-->
		<?php if (!$_smarty_tpl->tpl_vars['wxapp']->value) {?>

		<?php echo '<script'; ?>
>
			var weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
				wapurl = "<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
",
				code_web = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',
				code_kind = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
';
		<?php echo '</script'; ?>
>
		<?php }?>
	</head>

	<body>

		<!-- 邀请面试页面头部分 -->
		<?php if ($_GET['c']=='resume'&&$_GET['a']=='invite') {?>
		<div class="header">
			<div class="header_fixed">
				<div class="header_bg">
					<div class="header_cont">
						<a href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>"
							class="header_back hd-lbtn ">
						</a>
						<div class="header_h1"><?php echo $_smarty_tpl->tpl_vars['headertitle']->value;?>
</div>
					</div>
				</div>
			</div>
		</div>

		<!--wxapp 引用页面控制，不需要标题栏-->
		<?php } elseif (!$_smarty_tpl->tpl_vars['wxapp']->value) {?>

		<div class="header header_count">
			<div class="header_fixed">
				<div class="header_bg">
					<div class="header_cont">
						<a class="header_back hd-lbtn <?php if (($_GET['c']=='tiny'&&$_GET['a']=='add')||($_GET['c']=='once'&&$_GET['a']=='add')) {?>mui-action-back<?php }?>"
							href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>"></a>
						<div class="header_h1">

							<?php if ($_smarty_tpl->tpl_vars['topplaceholder']->value) {?>
							<div class="header_search">
								<form method="get" action="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/index.php">
									<input type="hidden" name="c" value="<?php echo $_GET['c'];?>
" />
									<div class="header_search_text">
										<input type="text" value="<?php echo $_GET['keyword'];?>
" name="keyword" class="input_search" placeholder="<?php echo $_smarty_tpl->tpl_vars['topplaceholder']->value;?>
">
									</div>
									<input type="submit" value=" " class="header_search_bth ">
								</form>
							</div>
							<?php } else { ?>
							<div class="header_words"> <?php echo $_smarty_tpl->tpl_vars['headertitle']->value;?>
</div>
							<?php }?>
						</div>
					</div>
				</div>
			</div>
		</div>
		<?php }?>
<?php }} ?>
