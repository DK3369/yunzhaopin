<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:43:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/headnav.htm" */ ?>
<?php /*%%SmartyHeaderCode:43744051369e89826cbe3b9-53932069%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '4b6b12af8d7589495ce8c8438d5f171847bdf53c' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/headnav.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '43744051369e89826cbe3b9-53932069',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'vipIsDown' => 0,
    'company' => 0,
    'company_rating' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89826cc1c27_17617306',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89826cc1c27_17617306')) {function content_69e89826cc1c27_17617306($_smarty_tpl) {?><body>
	<div class="body_box">
		<header>
			<div class="header">
				<div class="header_fixed">
					<div class="header-logo fltL">
						<a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
" target="_blank">
							<img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_unit_logo'];?>
" class="png">
						</a>
					</div>
					<div class="user_headerright">
						<a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
" target="_blank" title="返回网站首页" class="user_m_fanh">返回首页</a>
						<div class="yun_m_headermsg" onmouseover="tzmsgNumShow('show')" onmouseout="tzmsgNumShow('hide')">
							<i class="yun_m_headermsg_icon"></i>通知中心
							<span class="yun_m_headermsg_n" id="tzmsgNum" style="display:none"></span>
							<div class="yun_m_headermsg_box" style="display:none">
								<div class="yun_m_headermsg_list">
									<a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/member/index.php?c=hr&state=1">职位申请
										<em class="yun_m_headermsg_list_n" id="jobApplyNum"></em>
									</a>
								</div>
								<div class="yun_m_headermsg_list">
									<a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/member/index.php?c=sysnews">系统消息
										<em class="yun_m_headermsg_list_n" id="sysmsgNum"></em>
									</a>
								</div>
								<?php if ($_smarty_tpl->tpl_vars['config']->value['com_message']==1) {?>
								<div class="yun_m_headermsg_list">
									<a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/member/index.php?c=msg">求职咨询
										<em class="yun_m_headermsg_list_n" id="jobAskNum"></em>
									</a>
								</div>
								<?php }?>
							</div>
						</div>
						<div class="header_m_navright">
							<?php if ((isset($_smarty_tpl->tpl_vars['vipIsDown']->value)||empty($_smarty_tpl->tpl_vars['company']->value['rating']))) {?>
							<!--未开通会员或会员到期，显示购买提示-->
							<div class="new_com_headervipbox" onMouseOver="headervipShow('show')" onMouseOut="headervipShow('hide')">
								<div class="new_com_headervip">开通超级会员</div>
								<div class="new_com_headervipshow" style="display: none;">
									<div class="new_com_headerviptop">
										<div class="new_com_headerviptop_c">你还不是VIP超级会员
											<a href="index.php?c=right" class="new_com_headervipbth">开通会员</a>
										</div>
									</div>
									<div class="new_com_headerviptit"></div>
									<div class="headervip">
										<a href="index.php?c=right" class="headervip_bth"><i class="headervipicon headervip1"></i>专属客服</a>
										<a href="index.php?c=right" target="_blank" class="headervip_bth"><i class="headervipicon headervip2"></i>身份标识</a>
										<a href="index.php?c=right" class="headervip_bth"><i class="headervipicon headervip3"></i>置顶推广</a>
										<a href="index.php?c=right" class="headervip_bth"><i class="headervipicon headervip4"></i>查看更多</a>
									</div>
									<div class="headerviptit">我的权益</div>
									<div class="headervip">
										<a href="index.php?c=paylogtc" class="headervip_bth"><i class="headervipicon headervip1"></i>我的服务</a>
										<a href="index.php?c=paylog" target="_blank" class="headervip_bth"><i class="headervipicon headervip2"></i>订单管理</a>
										
									</div>
									<div class="headervip">
										
										<a href="index.php?c=integral" class="headervip_bth"><i class="headervipicon headervip1"></i><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
管理</a>
									</div>
								</div>
							</div>
							<?php }?>
							<div class="yun_m_headertx" onMouseOver="headerInfoShow('show')" onMouseOut="headerInfoShow('hide')">
								<a href="index.php?c=uppic" class="yun_m_headertxa">
									<img src="<?php echo $_smarty_tpl->tpl_vars['company']->value['logo'];?>
" width="30" height="30" />
								</a>
								<div class="yun_m_headertx_hi"><?php echo $_smarty_tpl->tpl_vars['company']->value['linkman'];?>
</div>
								<div class="yun_m_header_info" style="display: none;">
									<div class="user_infobox">
										<div class="user_infobox_c">
											<div class="user_infobox_zg">
												<span class="user_infobox_zg_n"><?php echo $_smarty_tpl->tpl_vars['company']->value['linkman'];?>
</span><?php echo $_smarty_tpl->tpl_vars['company']->value['linkjob'];?>

												<i class="user_infobox_cj"></i>
											</div>
											<div class="user_infobox_comname">
												<?php if ($_smarty_tpl->tpl_vars['company']->value['name']) {?>
												<?php echo $_smarty_tpl->tpl_vars['company']->value['name'];?>
<a href="index.php?c=info" class="user_infobox_combj">编辑</a>
												<?php if ($_smarty_tpl->tpl_vars['company_rating']->value['com_pic']) {?> 
												<img src="<?php echo $_smarty_tpl->tpl_vars['company_rating']->value['com_pic'];?>
" width="16">
												<?php }?>
												<?php } else { ?>
												<a href="index.php?c=info" style="color: #f60; text-decoration: underline">您还未完善企业信息，点击完善！</a>
												<?php }?>
											</div>
										</div>
									</div>
									<div class="user_tc_tset">
										<a href="index.php?c=vs" title="修改密码" class="user_set_bth"><i class="user_set_icon user_set_icon2"></i>修改密码</a>
										<a href="index.php?c=binding" class="user_set_bth"><i class="user_set_icon user_set_icon3"></i>账号认证</a>
									</div>
									<div class="user_tcdl">
										<a href="javascript:void(0)" onClick="logout('index.php?act=logout')" title="退出登录" class="user_tcdlbth" style="border:none;">退出登录</a>
									</div>
								</div>
							</div>
							<span class="user_headertel">招聘咨询电话：<span class="user_headertel_n"><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel']!='' ? $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel'] : $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
</span></span>
						</div>
					</div>
				</div>
				<div class="clear"></div>
			</div>
		</header><?php }} ?>
