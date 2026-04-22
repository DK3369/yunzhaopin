<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 13:00:22
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/verify/verify_js.htm" */ ?>
<?php /*%%SmartyHeaderCode:191195814369e855e67b3167-89041622%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '680d43c28db101181bc7c0a4667b6d696c52df75' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/verify/verify_js.htm',
      1 => 1700725928,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '191195814369e855e67b3167-89041622',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e855e67d6007_83957706',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e855e67d6007_83957706')) {function content_69e855e67d6007_83957706($_smarty_tpl) {?><?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==3) {?>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/geetest/gt.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/geetest/pc.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
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
<?php }?>
<?php }} ?>
