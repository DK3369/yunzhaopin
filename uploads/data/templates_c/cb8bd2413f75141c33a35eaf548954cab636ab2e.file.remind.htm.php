<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:35:40
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/public/remind.htm" */ ?>
<?php /*%%SmartyHeaderCode:151225015069e8966c37c209-82719293%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'cb8bd2413f75141c33a35eaf548954cab636ab2e' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/public/remind.htm',
      1 => 1700725934,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '151225015069e8966c37c209-82719293',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'remind' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8966c37d5c9_64939591',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8966c37d5c9_64939591')) {function content_69e8966c37d5c9_64939591($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php if ($_smarty_tpl->tpl_vars['remind']->value['info']) {?>

<?php echo '<script'; ?>
>
    var wapurl = '<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
';
    var url = '<?php echo $_smarty_tpl->tpl_vars['remind']->value['url'];?>
';
    var info = '<?php echo $_smarty_tpl->tpl_vars['remind']->value['info'];?>
';
    var btninfo = '<?php echo $_smarty_tpl->tpl_vars['remind']->value['btn'];?>
';

    showConfirm(info, function () {
        location.href = url;
    }, '退出登录', btninfo, function () {
        islogout('<?php echo smarty_function_url(array('m'=>'wap','c'=>'loginout'),$_smarty_tpl);?>
', '确认退出吗？');
    });
<?php echo '</script'; ?>
>
<?php }?>
<?php }} ?>
