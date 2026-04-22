<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:38
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/header.htm" */ ?>
<?php /*%%SmartyHeaderCode:80675788469e8962e7bb0d7-83289699%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'a5682459eb7293908b8df3cf42b416947163da7d' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/header.htm',
      1 => 1700725935,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '80675788469e8962e7bb0d7-83289699',
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
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8962e7bdc41_29555543',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8962e7bdc41_29555543')) {function content_69e8962e7bdc41_29555543($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><!DOCTYPE html>
<html lang="en">

	<head>
		<meta http-equiv="content-type" content="text/html; charset=utf-8">
		<meta http-equiv="X-UA-Compatible" content="IE=edge">
		<meta name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no">
		<meta http-equiv="Cache-Control" content="no-transform">
		<meta http-equiv="Cache-Control" content="no-siteapp" />
		<title><?php echo $_smarty_tpl->tpl_vars['title']->value;?>
</title>
		<meta name="keywords" content="<?php echo $_smarty_tpl->tpl_vars['keywords']->value;?>
,wap" />
		<meta name="description" content="<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
" />
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/flexible.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/css/base.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet">
		<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/css/yunwap.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet">
        <link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/css/css.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet">
		<link href="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/lib/index.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" />
	</head>
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
	<body>
<?php }} ?>
