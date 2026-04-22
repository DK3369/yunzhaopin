<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:43:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/left.htm" */ ?>
<?php /*%%SmartyHeaderCode:131112117069e89826cc26e4-69953249%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '06090ddc431e51450ee01f255405cc3ad804b1d3' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/left.htm',
      1 => 1700725933,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '131112117069e89826cc26e4-69953249',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'leftCur' => 0,
    'company' => 0,
    'newResumeNum' => 0,
    'config' => 0,
    'uid' => 0,
    'leftNav' => 0,
    'nav' => 0,
    'hideNav' => 0,
    'hv' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89826ccbf61_62857690',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89826ccbf61_62857690')) {function content_69e89826ccbf61_62857690($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><div class="sidebar">
	<div class="left_box">
		<ul class="left_nav_ul">
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==1) {?>left_nav_newcur<?php }?>">
				<span>
					<a href="index.php" title="首页" class="new_com_nav_a"><i class="com_left_icon com_left_icon1"></i>企业中心</a>
				</span>
			</li>
			<?php if ($_smarty_tpl->tpl_vars['company']->value['is_nav']==1) {?>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==2) {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=job&w=1" title="发布中的职位" class="new_com_nav_a"><i class="com_left_icon com_left_icon2"></i>职位管理</a>
                    </span>
			</li>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==3) {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=hr" title="简历管理" class="new_com_nav_a"><i class="com_left_icon com_left_icon4"></i>简历管理<?php if ($_smarty_tpl->tpl_vars['newResumeNum']->value>0) {?><i class="com_icon com_icon_new"><?php echo $_smarty_tpl->tpl_vars['newResumeNum']->value;?>
</i><?php }?></a>
                    </span>
			</li>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==4) {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=invite" title="面试管理" class="new_com_nav_a"><i class="com_left_icon com_left_icon10"></i>面试管理</a>
                    </span>
			</li>
			
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==9) {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=right" class="new_com_nav_a"><i class="com_left_icon com_left_icon7"></i>会员服务</a>
                    </span>
			</li>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==5) {?>left_nav_newcur<?php }?>">
			        <span>
			            <a href="index.php?c=resume" class="new_com_nav_a"><i class="com_left_icon com_left_icon3"></i>人才库</a>
			        </span>
			</li>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==7) {?>left_nav_newcur<?php }?>">
			        <span>
			            <a href="index.php?c=zhaopinhui" class="new_com_nav_a"><i class="com_left_icon com_left_icon12"></i>招聘会</a>
			        </span>
			</li>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==10) {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=info" title="企业资料" class="new_com_nav_a"><i class="com_left_icon com_left_icon8"></i>企业资料</a>
                    </span>
			</li>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==11) {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=binding" title="账号设置" class="new_com_nav_a"><i class="com_left_icon com_left_icon11"></i>账号设置</a>
                    </span>
			</li>
			<li class="more_box <?php if ($_smarty_tpl->tpl_vars['leftCur']->value==12) {?>left_nav_newcur<?php }?>" style="position: relative; z-index: 100;" onMouseOver="leftmoreShow('show')" onMouseOut="leftmoreShow('hide')">
				<span><a title="更多服务" class="new_com_nav_a"><i class="com_left_icon com_left_icon9"></i>更多服务</a></span>
				<div class="user_more" style="display: none;">
					
					<div class="user_more_list">
						<span class="user_more_name">人才管理 ></span>
						<a href="index.php?c=record" class="user_more_a">推送简历</a>
						<a href="index.php?c=finder" class="user_more_a">人才搜索器</a>
					</div>
					<div class="user_more_list">
						<span class="user_more_name">其他服务 ></span>
						<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_special_web']==1) {?><a href="index.php?c=special" class="user_more_a">专题招聘</a><?php }?>
						<a href="index.php?c=report&act=show" title="投诉记录" class="user_more_a">投诉记录</a>
						<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_ask_web']==1) {?><a href="<?php echo smarty_function_url(array('m'=>'ask','c'=>'friend','a'=>'myquestion','uid'=>$_smarty_tpl->tpl_vars['uid']->value),$_smarty_tpl);?>
" target="_blank" title="我的问答" class="user_more_a">我的问答</a><?php }?>
						<a href="index.php?c=customize" class="user_more_a">导航自定义</a>
					</div>
				</div>
			</li>
			<?php } else { ?>
			<?php  $_smarty_tpl->tpl_vars['nav'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['nav']->_loop = false;
 $_smarty_tpl->tpl_vars['nk'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['leftNav']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['nav']->key => $_smarty_tpl->tpl_vars['nav']->value) {
$_smarty_tpl->tpl_vars['nav']->_loop = true;
 $_smarty_tpl->tpl_vars['nk']->value = $_smarty_tpl->tpl_vars['nav']->key;
?>
			<?php if ($_smarty_tpl->tpl_vars['nav']->value['show']==1) {?>
			<li class="<?php if ($_smarty_tpl->tpl_vars['leftCur']->value==2&&$_smarty_tpl->tpl_vars['nav']->value['url']=='job'||$_smarty_tpl->tpl_vars['leftCur']->value==3&&$_smarty_tpl->tpl_vars['nav']->value['url']=='hr'||$_smarty_tpl->tpl_vars['leftCur']->value==4&&$_smarty_tpl->tpl_vars['nav']->value['url']=='invite'||$_smarty_tpl->tpl_vars['leftCur']->value==5&&$_smarty_tpl->tpl_vars['nav']->value['url']=='resume'||$_smarty_tpl->tpl_vars['leftCur']->value==7&&$_smarty_tpl->tpl_vars['nav']->value['url']=='zhaopinhui'||$_smarty_tpl->tpl_vars['leftCur']->value==9&&$_smarty_tpl->tpl_vars['nav']->value['url']=='right'||$_smarty_tpl->tpl_vars['leftCur']->value==10&&$_smarty_tpl->tpl_vars['nav']->value['url']=='info'||$_smarty_tpl->tpl_vars['leftCur']->value==11&&$_smarty_tpl->tpl_vars['nav']->value['url']=='binding') {?>left_nav_newcur<?php }?>">
                    <span>
                        <a href="index.php?c=<?php echo $_smarty_tpl->tpl_vars['nav']->value['url'];
if ($_smarty_tpl->tpl_vars['nav']->value['url']=='job') {?>&w=1<?php }?>" title="<?php echo $_smarty_tpl->tpl_vars['nav']->value['name'];?>
" class="new_com_nav_a"  <?php if ($_smarty_tpl->tpl_vars['nav']->value['target']==2) {?>target="_blank"<?php }?>>
                            <i class="com_left_icon com_left_icon<?php echo $_smarty_tpl->tpl_vars['nav']->value['icon'];?>
"></i><?php echo $_smarty_tpl->tpl_vars['nav']->value['name'];?>

                            <?php if ($_smarty_tpl->tpl_vars['nav']->value['url']=='hr'&&$_smarty_tpl->tpl_vars['newResumeNum']->value>0) {?><i class="com_icon com_icon_new"><?php echo $_smarty_tpl->tpl_vars['newResumeNum']->value;?>
</i><?php }?>
                        </a>
                    </span>
			</li>
			<?php }?>
			<?php } ?>
			<li class="more_box <?php if ($_smarty_tpl->tpl_vars['leftCur']->value==12) {?>left_nav_newcur<?php }?>" style="position: relative; z-index: 100;" onMouseOver="leftmoreShow('show')" onMouseOut="leftmoreShow('hide')">
				<span><a title="更多服务" class="new_com_nav_a"><i class="com_left_icon com_left_icon9"></i>更多服务</a></span>
				<div class="user_more" style="display: none;">
					
					<div class="user_more_list">
						<span class="user_more_name">人才管理 ></span>
						<a href="index.php?c=record" class="user_more_a">推送简历</a>
						<a href="index.php?c=finder" class="user_more_a">人才搜索器</a>
					</div>
					<div class="user_more_list">
						<span class="user_more_name">其他服务 ></span>
						<?php  $_smarty_tpl->tpl_vars['hv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['hv']->_loop = false;
 $_smarty_tpl->tpl_vars['hk'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['hideNav']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['hv']->key => $_smarty_tpl->tpl_vars['hv']->value) {
$_smarty_tpl->tpl_vars['hv']->_loop = true;
 $_smarty_tpl->tpl_vars['hk']->value = $_smarty_tpl->tpl_vars['hv']->key;
?>
						<a href="index.php?c=<?php echo $_smarty_tpl->tpl_vars['hv']->value['url'];?>
" class="user_more_a"><?php echo $_smarty_tpl->tpl_vars['hv']->value['name'];?>
</a>
						<?php } ?>
						<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_special_web']==1) {?><a href="index.php?c=special" class="user_more_a">专题招聘</a><?php }?>
						<a href="index.php?c=report&act=show" title="投诉记录" class="user_more_a">投诉记录</a>
						<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_ask_web']==1) {?><a href="<?php echo smarty_function_url(array('m'=>'ask','c'=>'friend','a'=>'myquestion','uid'=>$_smarty_tpl->tpl_vars['uid']->value),$_smarty_tpl);?>
" target="_blank" title="我的问答" class="user_more_a">我的问答</a><?php }?>
						<a href="index.php?c=customize" class="user_more_a">导航自定义</a>
						
					</div>
				</div>
			</li>
			<?php }?>
		</ul>
	</div>
</div><?php }} ?>
