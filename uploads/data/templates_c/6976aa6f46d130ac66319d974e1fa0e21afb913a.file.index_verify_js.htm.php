<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 13:00:14
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/verify/index_verify_js.htm" */ ?>
<?php /*%%SmartyHeaderCode:111224160569e855de0d7ea1-69141134%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '6976aa6f46d130ac66319d974e1fa0e21afb913a' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/verify/index_verify_js.htm',
      1 => 1703061688,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '111224160569e855de0d7ea1-69141134',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e855de0dab69_45951778',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e855de0dab69_45951778')) {function content_69e855de0dab69_45951778($_smarty_tpl) {?><!--首页模板专用-->
<?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"前台登录")!==false) {?>
	<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==3) {?>
	<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/geetest/gt.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
	<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['code_kind']==4) {?>
	<?php echo '<script'; ?>
 src="https://cdn.dingxiang-inc.com/ctu-group/captcha-ui/index.js"><?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
>var dxappid = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_dxappid'];?>
";<?php echo '</script'; ?>
>
	<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['code_kind']==5) {?>
	<?php echo '<script'; ?>
 src="https://v.vaptcha.com/v3.js"><?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
>var vaptchaid = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_vaptcha_vid'];?>
";<?php echo '</script'; ?>
>
	<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
	<!--<?php echo '<script'; ?>
 src="https://turing.captcha.qcloud.com/TCaptcha.js"><?php echo '</script'; ?>
>-->
	<?php echo '<script'; ?>
>var tecentappid = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_tecentappid'];?>
",web_url = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
";<?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/tecent/pc.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
	<?php }?>
<?php }?>
<?php }} ?>
