<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:47
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/verify_js.htm" */ ?>
<?php /*%%SmartyHeaderCode:136924833069e8963740bc54-26796336%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'dc045a6ca335fee738d10f4567ae5207c675850d' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/verify_js.htm',
      1 => 1700725938,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '136924833069e8963740bc54-26796336',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8963740fe91_01632545',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8963740fe91_01632545')) {function content_69e8963740fe91_01632545($_smarty_tpl) {?><?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==3) {?>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/geetest/gt.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/geetest/mobile.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['code_kind']==4) {?>
<?php echo '<script'; ?>
 src="https://cdn.dingxiang-inc.com/ctu-group/captcha-ui/index.js"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>var dxappid = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_dxappid'];?>
";<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/dingxiang/pc.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['code_kind']==5) {?>
<?php echo '<script'; ?>
 src="https://v.vaptcha.com/v3.js"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>var vaptchaid = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_vaptcha_vid'];?>
";<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/vaptcha/pc.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
<?php echo '<script'; ?>
 src="https://turing.captcha.qcloud.com/TCaptcha.js"><?php echo '</script'; ?>
>
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
<?php }?><?php }} ?>
