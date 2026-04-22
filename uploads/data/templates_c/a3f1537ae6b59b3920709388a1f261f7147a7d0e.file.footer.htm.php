<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:43:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/footer.htm" */ ?>
<?php /*%%SmartyHeaderCode:173718344669e89826cccff6-84598473%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'a3f1537ae6b59b3920709388a1f261f7147a7d0e' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/footer.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '173718344669e89826cccff6-84598473',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'consultant' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89826ccda85_84317636',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89826ccda85_84317636')) {function content_69e89826ccda85_84317636($_smarty_tpl) {?>
<!-- layui 当前显示弹出框index-->
<input type='hidden' id="layindex" value="">

<div class="clear"></div>

<div class=footer>
    <div class=w900>
        <div class=footernav>
            <div class="footer_bot_p"><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webcopyright'];
echo $_smarty_tpl->tpl_vars['config']->value['sy_webrecord'];?>
 </div>
            <div class="footer_bot_p">Powered by <a target="_blank" href="http://www.ov6.com">PHPYun.</a></div>
        </div>
    </div>
</div>

<div class="clear"></div>

<div id="uclogin" style="display:none"></div>

<div class="clear"></div>
<div id="crmWechat" style="display:none;">
    <div class="yun_wxbd_box">
        <div class="yun_wxbd_img_c">
            <div class="yun_wxbd_img" style="border:1px solid #eee; line-height:180px;">
                <img src="<?php echo $_smarty_tpl->tpl_vars['consultant']->value['wechat'];?>
" width="180" height="180" />
            </div>
        </div>
        <div class="yun_wxbd_p"> 请使用微信扫描码</div>
    </div>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['temstyle']->value)."/default/public_search/public_tips.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


</body>

</html><?php }} ?>
