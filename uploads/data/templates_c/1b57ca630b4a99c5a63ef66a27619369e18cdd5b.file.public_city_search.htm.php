<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:45
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/public_city_search.htm" */ ?>
<?php /*%%SmartyHeaderCode:59497112369e89635b69cb0-10273803%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '1b57ca630b4a99c5a63ef66a27619369e18cdd5b' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/public_city_search.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '59497112369e89635b69cb0-10273803',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89635b6b473_81074278',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89635b6b473_81074278')) {function content_69e89635b6b473_81074278($_smarty_tpl) {?><?php echo '<script'; ?>
 type="text/javascript">
    var cityParam = {
        sy_web_city_one: '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_web_city_one'];?>
',
        sy_web_city_two: '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_web_city_two'];?>
',
    };

    // 选择分站地区默认
    <?php if ($_smarty_tpl->tpl_vars['config']->value['province']) {?>
        cityParam.sy_web_city_one = parseInt('<?php echo $_smarty_tpl->tpl_vars['config']->value['province'];?>
');
        cityParam.sy_web_city_two = parseInt('<?php echo $_smarty_tpl->tpl_vars['config']->value['cityid'];?>
');
        cityParam.sy_web_city_three = parseInt('<?php echo $_smarty_tpl->tpl_vars['config']->value['three_cityid'];?>
');
        cityParam.one_all = false;
        cityParam.two_all = cityParam.sy_web_city_two ? false : true;
        cityParam.three_all = cityParam.sy_web_city_three ? false : true;
    <?php }?>

    var cityData = cityCategory(cityParam);
<?php echo '</script'; ?>
><?php }} ?>
