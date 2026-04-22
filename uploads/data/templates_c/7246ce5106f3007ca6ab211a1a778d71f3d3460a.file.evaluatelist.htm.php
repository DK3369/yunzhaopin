<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 19:01:05
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/evaluatelist.htm" */ ?>
<?php /*%%SmartyHeaderCode:102789415869e8aa71a5a869-14870881%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '7246ce5106f3007ca6ab211a1a778d71f3d3460a' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/evaluatelist.htm',
      1 => 1700725934,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '102789415869e8aa71a5a869-14870881',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config_wapdomain' => 0,
    'config' => 0,
    'rows' => 0,
    'v' => 0,
    'total' => 0,
    'pagelink' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8aa71a78062_73990840',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8aa71a78062_73990840')) {function content_69e8aa71a78062_73990840($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header_cont.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<!-- <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/mui/css/mui.min.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css" /> -->
<link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/template/wap/css/style.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css"/>
<div class="evaluate_box" id="app">
    <!--列表-->
    <?php if ($_smarty_tpl->tpl_vars['rows']->value) {?>
        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
        <div class="evaluate_list">
            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'evaluate','a'=>'show','id'=>$_smarty_tpl->tpl_vars['v']->value['id']),$_smarty_tpl);?>
">
                <div class="evaluate_pic"><img src="<?php echo $_smarty_tpl->tpl_vars['v']->value['pic_n'];?>
" width="90"/></div>
                <div class="evaluate_name"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</div>
                <div class="evaluate_p"><?php echo $_smarty_tpl->tpl_vars['v']->value['description'];?>
</div>
                <div class="evaluate_cs"><?php echo $_smarty_tpl->tpl_vars['v']->value['visits'];?>
人访问过<span class="evaluate_p_bth">开始测试 ></span></div>
            </a>
        </div>
        <?php } ?>
        <?php if ($_smarty_tpl->tpl_vars['total']->value>20) {?>
        <van-pagination v-model="currentPage" :total-items="total" :items-per-page="perpage" force-ellipses @change="pageChange" />
        <?php }?>
    <?php } else { ?>
    <div class="wap_member_nosearch">
        <div class="wap_member_no_tip"> 很抱歉,这个星球没有测评试卷呢！</div>
    </div>
    <?php }?>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
>
    var currentPage = parseInt('<?php echo $_GET['page'];?>
'),
        total = parseInt('<?php echo $_smarty_tpl->tpl_vars['total']->value;?>
'),
        pagelink = '<?php echo $_smarty_tpl->tpl_vars['pagelink']->value;?>
';

    var vm = new Vue({
        el: '#app',
        data: {
            //分页相关
            currentPage: currentPage ? currentPage : 1,
            total: total,
            perpage: 20,
        },
        methods: {
            pageChange: function (e) {
                var pageurl = pagelink.replace('{{page}}', e);
                location.href = pageurl;
            }
        }
    })
<?php echo '</script'; ?>
>
<?php }} ?>
