<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:38
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/public_js.htm" */ ?>
<?php /*%%SmartyHeaderCode:127419939169e8962e7d8450-83261769%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '472f9d7d38644f79deb4688eced5b7c38804b0cb' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/publichtm/public_js.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '127419939169e8962e7d8450-83261769',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'config' => 0,
    'config_wapdomain' => 0,
    'ball' => 0,
    'navlist' => 0,
    'key' => 0,
    'v' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8962e7df237_62387722',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8962e7df237_62387722')) {function content_69e8962e7df237_62387722($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_function_tongji')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.tongji.php';
?><?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/jquery.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/vue.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
> 
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/lib/vant.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
> 
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/phpyun_vant.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
> 
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/nativeshare.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>



<?php if (($_smarty_tpl->tpl_vars['ball']->value!=2&&!empty($_GET['c'])&&!in_array($_GET['c'],array('job','resume','sysnews','login','register','forgetpw','ask','article')))||(!empty($_GET['a'])&&!in_array($_GET['a'],array('applyjobuid')))) {?>
<section id="ball" class="goBackHome">
    <a href="javascript:void(0)" onclick="yunfootClose()">
        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/severs.png" alt="">
    </a>
</section>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/tuozhuai.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php }?>
<!--底部导航弹出-->
<div id="footerVue" class="none">
	<van-popup v-model="yunfoot" position="bottom" round closeable>
		<footer>
		    <div class="footerbox">
				 <div class="footerbox_tit">
		        </div>
		        <div class="foot_nav_box">
				<dl class="index_navlist" style="height:70px;">
					<a href="<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
">
						<dt class="index_nav_icon"><image src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/blackhome.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" ></image></dt>
						<dd>回到首页</dd>
					</a>
				</dl>
				<dl class="index_navlist" style="height:70px;">
					<a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
">
						<dt class="index_nav_icon"><image src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/usercent.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" ></image></dt>
						<dd>用户中心</dd>
					</a>
				</dl>
	            <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['navlist']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
				<?php if ($_smarty_tpl->tpl_vars['key']->value<6) {?>
				<dl class="index_navlist" style="height:70px;">
					<a href="<?php echo $_smarty_tpl->tpl_vars['v']->value['url_n'];?>
">
						<dt class="index_nav_icon"><image src="<?php echo $_smarty_tpl->tpl_vars['v']->value['pic_n'];?>
" ></image></dt>
						<dd><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</dd>
					</a>
				</dl>
				<?php }?>
				<?php } ?>
		        </div>
		    </div>
		</footer>
	</van-popup>
</div>
<?php echo '<script'; ?>
>
var footerVue = new Vue({
        el: '#footerVue',
        data: {
			yunfoot: false
		},
		mounted(){
			document.getElementById('footerVue').style.display = 'block';
		}
	});
<?php echo '</script'; ?>
>

<?php if (!empty($_GET['c'])&&(!in_array($_GET['c'],array('job','resume'))||!empty($_GET['a']))) {?>
<div class='none'><?php echo smarty_function_tongji(array(),$_smarty_tpl);?>
</div>
<?php }?><?php }} ?>
