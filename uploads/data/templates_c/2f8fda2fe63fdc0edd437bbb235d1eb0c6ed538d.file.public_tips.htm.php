<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:43:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template//default/public_search/public_tips.htm" */ ?>
<?php /*%%SmartyHeaderCode:147648798569e89826cce371-51118015%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '2f8fda2fe63fdc0edd437bbb235d1eb0c6ed538d' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template//default/public_search/public_tips.htm',
      1 => 1707101813,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '147648798569e89826cce371-51118015',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'noPermission' => 0,
    'isPaused' => 0,
    'statis' => 0,
    'vipIsDown' => 0,
    'config' => 0,
    'rating_1' => 0,
    'rating_2' => 0,
    'add' => 0,
    'v' => 0,
    'sy_only_price' => 0,
    'usertype' => 0,
    'discount' => 0,
    'todayStart' => 0,
    'Info' => 0,
    'is_member' => 0,
    'order' => 0,
    'consultant' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89826cf8783_78935321',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89826cf8783_78935321')) {function content_69e89826cf8783_78935321($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><div class="yun_tipsbox" style="display: none;z-index:100000000000;" id="tcmsg">
    
	<div class="yun_tipsboxbox">
		<div id="tcbox">

			<?php if ($_smarty_tpl->tpl_vars['noPermission']->value==1||$_smarty_tpl->tpl_vars['isPaused']->value==1) {?>
			<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['temstyle']->value)."/default/public_search/vipTips.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

			<?php }?>
			<div class="yun_tipsboxbox_hello">尊敬的企业用户您好！</div>
			<div class="yun_tipsboxbox_level">
				您当前是<span class="yun_tipsboxbox_level_name"><?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_name'];?>
</span>，您的<?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']<strtotime('today')&&$_smarty_tpl->tpl_vars['statis']->value['vip_etime']!=0) {?>会员已到期<?php } else { ?>套餐已用完<?php }?>，升级会员享受更多特权，选择适合您的企业VIP套餐
			</div>
			<ul class="buymeal_tit" id='rating_type'>

				<?php if (!$_smarty_tpl->tpl_vars['vipIsDown']->value) {?>
					<li id="singleTab" rating='4' ishide='0'><a><span class="buymeal_tit_name">单项购买</span><em class="buymeal_p">组合购买更优惠</em></a></li>
				<?php }?>

				<?php if ($_smarty_tpl->tpl_vars['noPermission']->value==1||$_smarty_tpl->tpl_vars['isPaused']->value==1) {?>
				<?php if (($_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='2'||$_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='0')&&!empty($_smarty_tpl->tpl_vars['rating_1']->value)) {?> 
					<li class="buymeal_tit_cur" rating='1' ishide='0'><a><span class="buymeal_tit_name">套餐会员</span><em class="buymeal_p">购买套餐会员更划算</em></a><span class="buymeal_tit_tj">推荐</span></li>
				<?php }?> 
				
				<?php if (($_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='1'||$_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='0')&&!empty($_smarty_tpl->tpl_vars['rating_2']->value)) {?> 
					<li rating='2' ishide='0'><a><span class="buymeal_tit_name">时间会员</span><em class="buymeal_p">尊享会员权益</em></a></li>
				<?php }?> 
				
				<?php if (!$_smarty_tpl->tpl_vars['vipIsDown']->value&&$_smarty_tpl->tpl_vars['config']->value['com_integral_online']!=4&&$_smarty_tpl->tpl_vars['statis']->value['rating_type']!=2&&!empty($_smarty_tpl->tpl_vars['add']->value)) {?>
					<li rating='3' ishide='0'><a><span class="buymeal_tit_name">增值包</span><em class="buymeal_p">招聘资源合理搭配</em></a></li>
				<?php }?>
				<?php }?>
				
			</ul>
			
			<!--套餐展示区start-->
			<div class="buymeal_cont rating_type <?php if ($_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='1') {?>none<?php }?>" id="rating_1">
				<div class="layui-carousel" id='vip_1'>
					<div carousel-item>
						<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['rating_1']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
						<div>
							<div class="buymeal_info">
								<div class="buymeal_name"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</div>
								
								<?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==3&&!in_array('vip',$_smarty_tpl->tpl_vars['sy_only_price']->value)) {?>
								
									<?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['yh_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</div>
										<div class="buymeal_original_price">原价 <i class="buymeal_original_price_delete"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</i> <span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php } else { ?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</div>
										<div class="buymeal_original_price"><span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php }?>
								
								<?php } else { ?>
								
									<?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['yh_price'];?>
</span>元</div>
										<div class="buymeal_original_price">原价 <i class="buymeal_original_price_delete"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>
</i> <span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php } else { ?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>
</span>元</div>
										<div class="buymeal_original_price"><span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php }?>
								
								<?php }?>  
								
								<div class="buymeal_purchase"><a href="javascript:void(0);" onclick='buyVip(<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="buymeal_purchase_bth">购买</a></div>
								
							</div>
							
							<div class="buymeal_resources_cont">
								<!--简历资源-->
								<div class="buymeal_resources_list">
									<div class="buymeal_resources_name">简历资源</div>
									<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">面试邀请</span><span class="buymeal_resources_detailed_n"><?php if ($_smarty_tpl->tpl_vars['v']->value['interview']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['interview'];?>
</span>次<?php } else { ?>-<?php }?></div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">简历下载</span><span class="buymeal_resources_detailed_n"><?php if ($_smarty_tpl->tpl_vars['v']->value['resume']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['resume'];?>
</span>份<?php } else { ?>-<?php }?></div>
									<?php } else { ?>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">简历下载</span><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_resume'];?>
</span>份</div>
									<?php }?>
								</div>
								<!--职位资源-->
								<div class="buymeal_resources_list">
									<div class="buymeal_resources_name">职位资源</div>
									<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位上架</span><?php if ($_smarty_tpl->tpl_vars['v']->value['job_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['job_num'];?>
</span>份<?php } else { ?>-<?php }?></div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位推荐</span><?php if ($_smarty_tpl->tpl_vars['v']->value['rec_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['rec_num'];?>
</span>天<?php } else { ?>-<?php }?></div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位刷新</span><?php if ($_smarty_tpl->tpl_vars['v']->value['breakjob_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['breakjob_num'];?>
</span>次<?php } else { ?>-<?php }?></div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位置顶</span><?php if ($_smarty_tpl->tpl_vars['v']->value['top_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['top_num'];?>
</span>天<?php } else { ?>-<?php }?></div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位紧急</span><?php if ($_smarty_tpl->tpl_vars['v']->value['urgent_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['urgent_num'];?>
</span>天<?php } else { ?>-<?php }?></div>
									<?php } else { ?>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位上架</span><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_job_num'];?>
</span>份</div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位刷新</span><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_breakjob_num'];?>
</span>次</div>
									<?php }?>
								</div>
								<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
								<!--其他资源-->
								<div class="buymeal_resources_list">
									<div class="buymeal_resources_name">其他资源</div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">招聘会预定</span><?php if ($_smarty_tpl->tpl_vars['v']->value['zph_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['zph_num'];?>
</span>次<?php } else { ?>-<?php }?></div>
								</div>
								<?php }?>
								<!--套餐说明-->
								<?php if ($_smarty_tpl->tpl_vars['v']->value['explains']) {?><div class="buymeal_resources_explain">套餐说明：<?php echo $_smarty_tpl->tpl_vars['v']->value['explains'];?>
</div><?php }?>
								<i class="buymeal_resources_line"></i>
							</div>
						</div>
						<?php } ?>
					</div>
				</div>
			</div>
			<!--展示区 end-->
			
			<!--时间模式展示区start-->
			<div class="buymeal_cont rating_type  <?php if ($_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='2'||$_smarty_tpl->tpl_vars['config']->value['com_vip_type']=='0') {?>none<?php }?>" id="rating_2">
				<div class="layui-carousel" id='vip_2'>
					<div carousel-item>
						<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['rating_2']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
						<div>
							<div class="buymeal_info">
								<div class="buymeal_name"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</div>
								<?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==3&&!in_array('vip',$_smarty_tpl->tpl_vars['sy_only_price']->value)) {?>
								
									<?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['yh_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</div>
										<div class="buymeal_original_price">原价 <i class="buymeal_original_price_delete"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</i> <span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php } else { ?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</div>
										<div class="buymeal_original_price"><span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php }?>
								
								<?php } else { ?>
								
									<?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['yh_price'];?>
</span>元</div>
										<div class="buymeal_original_price">原价 <i class="buymeal_original_price_delete"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>
</i> <span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php } else { ?>
										<div class="buymeal_price"><span class="buymeal_price_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>
</span>元</div>
										<div class="buymeal_original_price"><span class=""><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?></span></div>
									<?php }?>
								
								<?php }?>  
								<div class="buymeal_purchase"><a href="javascript:void(0);" onclick='buyVip(<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
)' class="buymeal_purchase_bth">购买</a></div>
							</div>
							
							<div class="buymeal_resources_cont">
								<!--简历资源-->
								<div class="buymeal_resources_list">
									<div class="buymeal_resources_name">简历资源</div>
									<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">面试邀请</span>
										<?php if ($_smarty_tpl->tpl_vars['v']->value['interview']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['interview'];?>
</span>次/日
										<?php } else { ?><span class="buymeal_resources_detailed_n">-</span><?php }?>
									</div>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">简历下载</span>
										<?php if ($_smarty_tpl->tpl_vars['v']->value['resume']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['resume'];?>
</span>次/日
										<?php } else { ?><span class="buymeal_resources_detailed_n">-</span><?php }?>
									</div>
									<?php } else { ?>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">简历下载</span>
										<?php if ($_smarty_tpl->tpl_vars['v']->value['lt_resume']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_resume'];?>
</span>次/日
										<?php } else { ?><span class="buymeal_resources_detailed_n">-</span><?php }?>
									</div>
									<?php }?>
								</div>
								
								<!--职位资源-->
								<div class="buymeal_resources_list">
									<div class="buymeal_resources_name">职位资源</div>
									<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">职位上架</span>
										<span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['job_num'];?>
</span>份
									</div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位推荐</span><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['rec_num'];?>
</span><?php if ($_smarty_tpl->tpl_vars['v']->value['rec_num']>0) {?>天<?php }?></div>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">职位刷新</span>
										<?php if ($_smarty_tpl->tpl_vars['v']->value['breakjob_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['breakjob_num'];?>
</span>次/日
										<?php } else { ?><span class="buymeal_resources_detailed_n">-</span><?php }?>
									</div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位置顶</span><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['top_num'];?>
</span><?php if ($_smarty_tpl->tpl_vars['v']->value['top_num']>0) {?>天<?php }?></div>
									<div class="buymeal_resources_detailed"><span class="buymeal_resources_detailedname">职位紧急</span><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['urgent_num'];?>
</span><?php if ($_smarty_tpl->tpl_vars['v']->value['urgent_num']>0) {?>天<?php }?></div>
									<?php } else { ?>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">职位上架</span>
										<span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_job_num'];?>
</span>份
									</div>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">职位刷新</span>
										<?php if ($_smarty_tpl->tpl_vars['v']->value['lt_breakjob_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_breakjob_num'];?>
</span>份/日
										<?php } else { ?><span class="buymeal_resources_detailed_n">-</span><?php }?>
									</div>
									<?php }?>
								</div>
								<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
								<!--其他资源-->
								<div class="buymeal_resources_list">
									<div class="buymeal_resources_name">其他资源</div>
									<div class="buymeal_resources_detailed">
										<span class="buymeal_resources_detailedname">招聘会预定</span>
										<?php if ($_smarty_tpl->tpl_vars['v']->value['zph_num']>0) {?><span class="buymeal_resources_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['zph_num'];?>
</span>次/日
										<?php } else { ?><span class="buymeal_resources_detailed_n">-</span><?php }?>
									</div>
								</div>
								<?php }?>
								<!--说明-->
								<?php if ($_smarty_tpl->tpl_vars['v']->value['explains']) {?><div class="buymeal_resources_explain">套餐说明：<?php echo $_smarty_tpl->tpl_vars['v']->value['explains'];?>
</div><?php }?>
								<i class="buymeal_resources_line"></i>
							</div>
						</div>
						<?php } ?>
					</div>
				</div>
			</div>
			<!--展示区 end-->
			
			<!--增值包模式展示区start-->
			<div class="buymeal_incrementcont rating_type none" id='rating_3'>
				<div class="buymeal_incrementbox">
					<table class="buymeal_increment" cellpadding="0" cellspacing="0">
						<tr>
							<th align="left" width="150">套餐名称</th>
							<th align="left">套餐资源</th>
							<th width="80">普通会员价</th>
							<th width="100">VIP会员价</th>
							<th>操作</th>
						</tr>
						<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['add']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
							<tr>
								<td align="left"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</td>
								<?php if ($_smarty_tpl->tpl_vars['usertype']->value==2) {?>
								<td align="left">
									<?php if ($_smarty_tpl->tpl_vars['v']->value['interview']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">面试邀请 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['interview'];?>
</span>份</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['resume']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">简历下载</span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['resume'];?>
</span>份</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['job_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">职位上架 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['job_num'];?>
</span>份</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['breakjob_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">刷新职位 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['breakjob_num'];?>
</span>份</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['top_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">职位置顶 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['top_num'];?>
</span>天</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['rec_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">职位推荐 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['rec_num'];?>
</span>天</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['urgent_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">紧急招聘 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['urgent_num'];?>
</span>天</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['zph_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">报名招聘会 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['zph_num'];?>
</span>次</div><?php }?>
								</td>
								<?php } else { ?>
								<td align="left">
									<?php if ($_smarty_tpl->tpl_vars['v']->value['resume']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">简历下载</span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['resume'];?>
</span>份</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['lt_job_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">职位上架 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_job_num'];?>
</span>份</div><?php }?>
									<?php if ($_smarty_tpl->tpl_vars['v']->value['lt_breakjob_num']) {?><div class="buymeal_increment_detailed"><span class="buymeal_increment_detailed_name">刷新职位 </span><span class="buymeal_increment_detailed_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['lt_breakjob_num'];?>
</span>份</div><?php }?>
								</td>
								<?php }?>
								<?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==3&&!in_array('pack',$_smarty_tpl->tpl_vars['sy_only_price']->value)) {?>
									<td align="center"><span class="buymeal_resources_price"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</span></td>
									<td align="center">
										<?php if ($_smarty_tpl->tpl_vars['discount']->value['service_discount']) {?>
											<span class="buymeal_resources_price"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion']*$_smarty_tpl->tpl_vars['discount']->value['service_discount']*0.01;
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</span>
											<div class="buymeal_resources_detailed_zk">专享<?php echo $_smarty_tpl->tpl_vars['discount']->value['service_discount'];?>
折优惠</div>
										<?php } else { ?>
											<span class="buymeal_resources_price"> 升级会员</span>
											<div class="buymeal_resources_detailed_zk">享受折扣</div>
										<?php }?>
									</td>
								<?php } else { ?>
									<td align="center"><span class="buymeal_resources_price"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>
元</span></td>
									<td align="center">
										<?php if ($_smarty_tpl->tpl_vars['discount']->value['service_discount']) {?>
											<span class="buymeal_resources_price"><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['discount']->value['service_discount']*0.01;?>
元</span>
											<div class="buymeal_resources_detailed_zk">专享<?php echo $_smarty_tpl->tpl_vars['discount']->value['service_discount'];?>
折优惠</div>
										<?php } else { ?>
											<span class="buymeal_resources_price"> 升级会员</span>
											<div class="buymeal_resources_detailed_zk">享受折扣</div>
										<?php }?>
									</td>
								<?php }?> 
								
								<td align="center"><a href="javascript:void(0);" onclick='buyPack(<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
,<?php echo $_smarty_tpl->tpl_vars['statis']->value['vip_etime'];?>
,<?php echo $_smarty_tpl->tpl_vars['todayStart']->value;?>
)' class="buymeal_resources_bth">购买</a></td>
							</tr>
						<?php } ?>
					</table>
				</div>
			</div>
			<!--展示区 end-->
		</div>
		<!-- 单份消费形式 start -->
		<div class="rating_type none" id='rating_4'>
			<div class="single_outobox">
				<?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==4) {?>
				<!--情况三，强制使用金额消费模式-->
				<div class="new_pattern_paybox" style="padding-top:5px;">
					<div id="dayslist" class="new_pattern_paylist none">
						<span id="promote_name" class="new_pattern_paylist_name">置顶时长：</span>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="1" onclick="checkRadio(this.value,'radio')" checked="true" />1天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="3" onclick="checkRadio(this.value,'radio')" />3天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="7" onclick="checkRadio(this.value,'radio')" />7天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="15" onclick="checkRadio(this.value,'radio')" />15天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="30" onclick="checkRadio(this.value,'radio')" />30天</label>
						<input type="text" id='xdays' name='xdays' onclick="radioClean(this.value)" onkeyup="checkRadio(this.value,'input')" maxlength="3" oninput="myFunction(this)"  value="" placeholder="自定义置顶天数" class="new_pattern_paylist_choice_text" />
					</div>
					<div class="new_pattern_paylist">
						<span class="new_pattern_paylist_name">所需金额:</span>  
						<div class="new_pattern_paylist_tip">本次操作需支付<span class="single_box_price" id='price_int'></span>元</div>
					</div>
					 
					
					<div class='price_single_member'>
					 
						<div class="single_list">
							<span class="single_left">需支付：</span>
							<span class="single_payprice"><em class="single_payprice_n" id="single_order_price"></em> 元</span>
						</div>
						
						<form name="alipayment" id="payform_server" action="../api/alipay/alipayto.php" method="post" target="_blank" onsubmit="return payforms('single');">
							<div class="single_list">
								<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
									<span class="single_left">选择支付方式：</span>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('alipay','payform_server','single')" class="single_paybth single_paybthzfb">支付宝快捷支付</a>
									<?php }?>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('bank','payform_vip','all')" class="single_paybth single_paybthyl">其他支付方式</a>
									<?php }?>
								<?php }?> 
							</div>
							
							<input type="hidden" id="pay_type" name="pay_type" value="" />
			                <input type="hidden" id="order_single_id" name="dingdan" value="" />
			                <input type="hidden" id="server_subject" name="subject" value="" />
			                <input type="hidden" name="pay_bank" value="directPay">
						</form>
					</div>
				</div>
				<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==3) {?>
				<div class="new_pattern_paybox">
					<div id="dayslist" class="new_pattern_paylist none">
						<span id="promote_name" class="new_pattern_paylist_name">置顶时长：</span>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="1" onclick="checkRadio(this.value,'radio')" checked="checked" />1天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="3" onclick="checkRadio(this.value,'radio')" />3天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="7" onclick="checkRadio(this.value,'radio')" />7天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="15" onclick="checkRadio(this.value,'radio')" />15天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="30" onclick="checkRadio(this.value,'radio')" />30天</label>
						<input type="text" id='xdays' name='xdays' onclick="radioClean(this.value)" onkeyup="checkRadio(this.value,'input')" maxlength="3" oninput="myFunction(this)" value="" placeholder="自定义置顶天数" class="new_pattern_paylist_choice_text" />
					</div>

	          		<div class="new_pattern_paylist cI3_server">
	           			<span class="new_pattern_paylist_name">所需<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
:</span>
	           			<div class="new_pattern_paylist_tip">
							当前拥有<span class="single_box_price"><?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
 ， 本次操作将扣除您
							<span class="single_box_price" id="single_integral"></span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>

						</div>
	           		</div>
	           		<!--状态1： 积分不足情况积分购买-->
	           		<div id='pay_integral_buy'>
		           		
		           		<div class="new_pattern_paylist cI3_server">
		           			<span class="new_pattern_paylist_name">充值<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
：</span>
		           			<input type="text" value="" id="integral_int" maxlength='6' onkeyup="checkIntegralPaySingle(<?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
, <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_min_recharge'];?>
, <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
);" oninput="myFunction(this)" placeholder="请输入充值<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
数量" class="single_text" />
							<div style="width:400px; float: left; padding-left:10px;color:#999;line-height: 33px;">最低充值<font color="red"> <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_min_recharge'];?>
</font> <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
 ,兑换比为1元=<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_priceunit'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</div>
		           		</div>
		           		<div class="new_pattern_paylist">
		           			<span class="new_pattern_paylist_name">需支付：</span>
		           			<span class="single_payprice"><em class="single_payprice_n" id="price_int"></em> 元</span>
		           		</div>
		           		
		            	<form name="alipayment" id="payform_server" action="../api/alipay/alipayto.php" method="post" target="_blank" onsubmit="return payforms('single');">
			            	<div class="new_pattern_paylist">
			            		<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
				            		<span class="new_pattern_paylist_name">选择支付方式：</span>
							 		<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
				            			<a href="javascript:void(0);" onclick="payVip('alipay','payform_server','single')" class="single_paybth single_paybthzfb">支付宝快捷支付</a>
				            		<?php }?>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('bank','payform_vip','all')" class="single_paybth single_paybthyl">其他支付方式</a>
									<?php }?>
				            	<?php }?>
			            	</div>
			            	
			            	<input type="hidden" id="pay_type" name="pay_type" value="" />
			                <input type="hidden" id="order_single_id" name="dingdan" value="" />
			                <input type="hidden" id="server_subject" name="subject" value="" />
			                <input type="hidden" name="pay_bank" value="directPay" />
		            	</form>
	            	</div>
	            	
	            	<!--状态2： 积分充足-->
	            	<div id='integral_buy'>
		            	<div class="new_pattern_paylist cI3_server">
	            			<span class="new_pattern_paylist_name">&nbsp;</span>
	            			<a href="javascript:void(0);" onclick="integralBuy();" class="new_pattern_paylist_bth">确认支付</a> 
	            		</div>
	              		<div class="new_pattern_paylist cI3_server">
	              			<span class="new_pattern_paylist_name">&nbsp;</span>  
	              			<div class="new_pattern_paylist_tip_b">套餐购买更优惠，升级会员享受专属特权</div>
	              		</div>
	            	</div>
					
				</div>
				
				<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==2) {?>
				
				<!--情况三，强制使用金额消费模式-->
				<div class="new_pattern_paybox" style="padding-top:5px;">
					<div id="dayslist" class="new_pattern_paylist none">
						<span id="promote_name" class="new_pattern_paylist_name">置顶时长：</span>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="1" onclick="checkRadio(this.value,'radio')" checked="true" />1天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="3" onclick="checkRadio(this.value,'radio')" />3天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="7" onclick="checkRadio(this.value,'radio')" />7天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="15" onclick="checkRadio(this.value,'radio')" />15天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="30" onclick="checkRadio(this.value,'radio')" />30天</label>
						<input type="text" id='xdays' name='xdays' onclick="radioClean(this.value)" onkeyup="checkRadio(this.value,'input')" maxlength="3" oninput="myFunction(this)"  value="" placeholder="自定义置顶天数" class="new_pattern_paylist_choice_text" />
					</div>
					<div class="new_pattern_paylist">
						<span class="new_pattern_paylist_name">所需金额:</span>  
						<div class="new_pattern_paylist_tip">本次操作需支付<span class="single_box_price" id='price_int'></span>元</div>
					</div>
					 
	    
					<div class='price_single_member'>
					 
						<div class="single_list">
							<span class="single_left">需支付：</span>
							<span class="single_payprice"><em class="single_payprice_n" id="single_order_price"></em> 元</span>
						</div>
						
						<form name="alipayment" id="payform_server" action="../api/alipay/alipayto.php" method="post" target="_blank" onsubmit="return payforms('single');">
							<div class="single_list">
								<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
									<span class="single_left">选择支付方式：</span>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('alipay','payform_server','single')" class="single_paybth single_paybthzfb">支付宝快捷支付</a>
									<?php }?>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('bank','payform_vip','all')" class="single_paybth single_paybthyl">其他支付方式</a>
									<?php }?>
								<?php }?> 
							</div>
							
							<input type="hidden" id="pay_type" name="pay_type" value="" />
			                <input type="hidden" id="order_single_id" name="dingdan" value="" />
			                <input type="hidden" id="server_subject" name="subject" value="" />
			                <input type="hidden" name="pay_bank" value="directPay">
						</form>
					</div>
				</div>
				
				<?php } elseif ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==1) {?>
					
				<!--情况四，金额抵扣消费模式-->
				<div class="new_pattern_paybox"  style="padding-top:5px;">
					<div id="dayslist" class="new_pattern_paylist none">
						<span id="promote_name" class="new_pattern_paylist_name">置顶时长：</span>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="1" onclick="checkRadio(this.value,'radio')" checked="true" />1天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="3" onclick="checkRadio(this.value,'radio')" />3天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="7" onclick="checkRadio(this.value,'radio')" />7天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="15" onclick="checkRadio(this.value,'radio')" />15天</label>
						<label class="new_pattern_paylist_choice"><input type="radio" name='days' value="30" onclick="checkRadio(this.value,'radio')" />30天</label>
						<input type="text" id='xdays' name='xdays' onclick="radioClean(this.value)" onkeyup="checkRadio(this.value,'input')" maxlength="3" oninput="myFunction(this)"  value="" placeholder="自定义置顶天数" class="new_pattern_paylist_choice_text" />
					</div>
					<div class="new_pattern_paylist">
						<span class="new_pattern_paylist_name">所需金额:</span>  
						<div class="new_pattern_paylist_tip">本次操作需支付<span class="single_box_price" id='price_int'></span>元</div>
					</div>
					
	    
					<?php if ($_smarty_tpl->tpl_vars['statis']->value['integral']>0) {?>
					<div class="single_list cI3_server" id="integral_dk_div">
						<span class="single_left"><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
抵扣：</span>
						<input type="text"  id="server_integral_dk" value="" placeholder="请输入抵扣数量" maxlength='6' onkeyup="checkIntegralDKSingle(<?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
,<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
);" oninput="myFunction(this)" class="single_text" autocomplete="off">
						<span class="single_dq">您当前拥有<span class="single_box_price"><?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
（1元=<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
)</span>
					</div>
					<?php }?>
					
								
					<div class='price_single_member'>
					
						<div class="single_list">
							<span class="single_left">需支付：</span>
							<span class="single_payprice"><em class="single_payprice_n" id='single_order_price'></em> 元</span>
						</div>
						<form name="alipayment" id="payform_server" action="../api/alipay/alipayto.php" method="post" target="_blank" onsubmit="return payforms('single');">
							<div class="single_list" id='server_pay_div'>
								<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
									<span class="single_left">选择支付方式：</span>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('alipay','payform_server','single')" class="single_paybth single_paybthzfb">支付宝快捷支付</a>
									<?php }?>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
										<a href="javascript:void(0);" onclick="payVip('bank','payform_vip','all')" class="single_paybth single_paybthyl">其他支付方式</a>
									<?php }?>
								<?php }?> 
							</div>
							
							<input type="hidden" id="pay_type" name="pay_type" value="" />
	                		<input type="hidden" id="order_single_id" name="dingdan" value="" />
	                		<input type="hidden" id="server_subject" name="subject" value="" />
	                		<input type="hidden" name="pay_bank" value="directPay" />
	                		
	                		<div class="single_list none" id='server_integral_div'>
	                         	<span class="single_left">&nbsp;</span>
								<a href="javascript:void(0);" onclick="integralBuy();" class="single_bth">确认购买</a>
							</div>
						</form>
					</div>
				</div>
			<?php }?>	
				
			</div>
		</div>

				</div>
			</div>


<input type="hidden" name="orderid" id="orderid" value="" />
<input type="hidden" name="ratingid" id="ratingid" value="" />
<input type="hidden" name="comserviceid" id="comserviceid" value="" />
<input type="hidden" name='promoteid' id='promoteid' value='' />
<input type="hidden" name="partids" id="partids" value="" />
<input type="hidden" name="jobids" id="jobids" value="" />
<input type="hidden" name="eid" id="eid" value="<?php echo $_smarty_tpl->tpl_vars['Info']->value['id'];?>
" />
<input type="hidden" name="zid" id="zid" value="" />
<input type="hidden" name="bid" id="bid" value="" />

<input type="hidden" id="integral_pro" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
" />
<input type="hidden" id="integral_min" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_min_recharge'];?>
" />
<input type="hidden" id="integral_online" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['com_integral_online'];?>
" />
<!--<input type="hidden" id="member_meal" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['member_meal'];?>
" />-->
<input type="hidden" id="user_integral_all" value="<?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
" />
<input type="hidden" id="is_member" value="<?php echo $_smarty_tpl->tpl_vars['is_member']->value;?>
" />

<!-- 服务价格显示参数begin -->
<!-- 服务 -->
<input type="hidden" id="pay_server" value="" />
<!-- 价格 -->
<input type="hidden" id="server_intrgral_name" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_name'];?>
" />
<input type="hidden" id="server_integral_job" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_job'];?>
" />
<input type="hidden" id="server_integral_jobefresh" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_jobefresh'];?>
" />
<input type="hidden" id="server_integral_interview" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_interview'];?>
" />
<?php if ($_smarty_tpl->tpl_vars['is_member']->value) {?>
<input type="hidden" id="server_com_recjob" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['com_recjob'];?>
" />
<input type="hidden" id="server_com_urgent" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['com_urgent'];?>
" />
<input type="hidden" id="server_integral_job_top" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_job_top'];?>
" />
<input type="hidden" id="server_job_auto" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['job_auto'];?>
" />
<input type="hidden" id="jobautoids" value="" />
<?php }?>
<!-- 服务价格显示参数end -->

<!--以上购买产品点击购买弹出的样式为统一下方样式-->

<!--积分购买，只有套餐设置为积分消费，并且不是强制金额消费-->
<!--方式二.积分购买-->
<div class="yun_purchase_payment mt20 none" style="margin-left:20px;" id='buyVipDiv_2'>
	<div class="yun_purchase_payment_tit">
		<span class="yun_purchase_payment_titname">购买<span class="" id='service_name_2'></span></span>
	</div>
	<div class="yun_purchase_payment_tip">
		您当前拥有<span class="single_box_price"><?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
 , 本次购买需扣除<span class="single_box_price" id='vip_order_price_2'></span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>

	</div>
	<div class="single_bthbox">
		<a href="javascript:void(0);" onclick="integralBuy();" class="single_bth">确认购买</a>
	</div>
	<div class="single_tip">套餐购买更优惠，升级会员享受专属特权</div>
</div>

<!--方式三.积分购买但余额不足-->
<div class="yun_purchase_payment mt20 none" style="margin-left:20px;" id='buyVipDiv_3'>
	<div class="yun_purchase_payment_tittwo">
		<span class="yun_purchase_payment_titname">购买<span id='service_name_3'></span></span>
	</div>
	
	<div class="yun_purchase_payment_list">
		<span class="yun_purchase_payment_listname">产品价格：</span>
		<div class="yun_purchase_payment_price">
			<span class="yun_purchase_payment_price_n" id='service_price_3'></span> <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
 
			<span class="yun_purchase_payment_price_yj" id='origin_tag_3'>原价：<i class="buymeal_original_price_delete" id='service_origin_price_3'></i></span>
		</div>
	</div>
	
	<div class="yun_purchase_payment_list">
		<span class="yun_purchase_payment_listname"><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
充值：</span>
		<input type="text" value="" id='integral_int_vip' name='integral_int_vip' maxlength='6' onkeyup="checkIntegralPay(<?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
, <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_min_recharge'];?>
, <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
);" oninput="myFunction(this)" placeholder="请输入充值数量"  class="yun_purchase_text" >
		<span class="single_dq">您当前拥有<span class="single_box_price"><?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</span>
		
	</div>
	<div class="new_pattern_paylist_tip" style="padding-left:130px;color:#999">最低充值 <font color="red"><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_min_recharge'];?>
</font> <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
,兑换比为1元=<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_priceunit'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</div>
	<div class="yun_purchase_payment_list">
		<span class="yun_purchase_payment_listname">需支付：</span>
		<span class="single_payprice"><em class="single_payprice_n" id='vip_order_price_3'>0</em> 元</span>
	</div>
	
	<form name="alipayment" id="payform_vip" action="../api/alipay/alipayto.php" method="post" target="_blank" onsubmit="return payforms('vip');">
		<div class="yun_purchase_payment_list" style="padding-bottom:20px;">
			<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1'||$_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
				<span class="yun_purchase_payment_listname">选择支付方式：</span>
				
			 	<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
					<a href="javascript:void(0);" onclick="payVip('alipay','payform_vip','all')" class="single_paybth single_paybthzfb">支付宝快捷支付</a>
				<?php }?> 
				
				<?php if ($_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
					<a href="javascript:void(0);" onclick="payVip('bank','payform_vip','all')" class="single_paybth single_paybthyl">其他支付方式</a>
				<?php }?>
			<?php } else { ?>
			<span style="color:red; margin-left: 120px;">网站已关闭支付接口，请联系管理员</span>
			<?php }?>
 		</div>
 		
		<input type="hidden" id="pay_type_vip" name="pay_type" value="" />
		<input type="hidden" id="order_vip_id" name="dingdan" value="" />
		<input type="hidden" id="subject_3" name="subject" value="充值<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
购买会员" />
		<input type="hidden" name="pay_bank" value="directPay" />
	</form>
</div>

<!--方式一.金额购买-->
<div class="yun_purchase_payment mt20 none" style="margin-left:20px;" id='buyVipDiv_1'>

	<div class="yun_purchase_payment_tit">
		<span class="yun_purchase_payment_titname">购买<span id='service_name_1'></span></span>
 	</div>
	
	<div class="yun_purchase_payment_list">
		<span class="yun_purchase_payment_listname">产品价格：</span>
		<div class="yun_purchase_payment_price">
			<span class="yun_purchase_payment_price_n" id='service_price_1'></span> 元 
			<span class="yun_purchase_payment_price_yj" id='origin_tag_1'>原价：<i class="buymeal_original_price_delete" id='service_origin_price_1'></i></span>
		</div>
	</div>

	<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1'||$_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
		    
	
	<?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==1) {?>
		
		<div class="yun_purchase_payment_list cI4_server">
			<span class="yun_purchase_payment_listname"><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_printegral_dk_divicename'];?>
抵扣：</span>
			<input type="text" value="" placeholder="请输入抵扣数量" id='integral_dk_vip' name='integral_dk_vip' oninput="myFunction(this)" onkeyup="checkIntegralDK('<?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
','<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
','1');" class="yun_purchase_text" />
			<span class="single_dq">您当前拥有<span class="single_box_price"><?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
</span>
		</div>
		
	<?php }?>
	
		
	<?php }?>
	
	<div class='price_1'>
		<div class="yun_purchase_payment_list">
			<span class="yun_purchase_payment_listname">需支付：</span>
			<span class="single_payprice"><em class="single_payprice_n" id='vip_order_price_1'>0</em> 元</span>
		</div>
		
		<form name="alipayment" id="payform_vip" action="../api/alipay/alipayto.php" method="post" target="_blank" onsubmit="return payforms('vip');">
			<div class="yun_purchase_payment_list" id='vip_pay_div'>
			 	<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1'||$_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
			 		<span class="yun_purchase_payment_listname">选择支付方式：</span>
			 	 	<?php if ($_smarty_tpl->tpl_vars['config']->value['alipay']=='1') {?>
			 			<a href="javascript:void(0);" onclick="payVip('alipay','payform_vip','all')" class="single_paybth single_paybthzfb">支付宝快捷支付</a>
			 		<?php }?> 
			 		<?php if ($_smarty_tpl->tpl_vars['config']->value['bank']=='1') {?>
			 			<a href="javascript:void(0);" onclick="payVip('bank','payform_vip','all')" class="single_paybth single_paybthyl">其他支付方式</a>
			 		<?php }?>
				<?php } else { ?>
				<span style="color: red; margin-left: 120px;">网站已关闭支付接口，请联系管理员</span>
			 	<?php }?>
			</div>
			<input type="hidden" id="pay_type_vip" name="pay_type" value="" />
			<input type="hidden" id="order_vip_id" name="dingdan" value="" />
			<input type="hidden" id="subject_1" name="subject" value="购买会员" />
			<input type="hidden" name="pay_bank" value="directPay" />
			<div class="single_bthbox none" id='vip_integral_div'>
				<a href="javascript:void(0);" onclick='integralBuy();' class="single_bth">确认购买</a>
			</div>
		</form>
	</div>
</div>


<!--支付弹出框-->
<div id="payshow" style="width:450px; position:absolute;left:0px;top:0px; background:#fff; display:none;">
    <div class="payment_tip">
        请在新打开的支付页面上完成付款，付款完成前请不要关闭此窗口。<br> 如您在支付过程中遇到问题，请联系客服：
        <span class="payment_tip_s"><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
</span>
    </div>
    <div class="payment_bottom">
		<?php if ($_smarty_tpl->tpl_vars['is_member']->value) {?>
        <a href="index.php?c=paylog" class="payment_bottombutt">已完成付款</a>
		<?php if ($_smarty_tpl->tpl_vars['order']->value['id']) {?>
		<a href="index.php?c=payment&id=<?php echo $_smarty_tpl->tpl_vars['order']->value['id'];?>
" class="payment_bottom_bth2">重新支付</a>
		<?php } else { ?>
        <a id="repaya" href="javascript:location.reload();" class="payment_bottom_bth2">重新支付</a>
		<?php }?>
		<?php } else { ?>
		<a href="javascript:location.reload();" class="payment_bottombutt">已完成付款</a>
		<?php }?>
    </div>
</div>

<div class="yun_prompt_box" style="margin-top:20px;display:none;" id="jobcheck" >
	<!--提示部分 start-->
	<div class="yun_prompt_release">
		<div class="yun_prompt_release_hi">尊敬的用户您好！</div>
		<div id='tips_way'>
			<div class="yun_prompt_release_tip">以下条件尚未满足,暂时无法查看简历，请按顺序完成：</div>
		</div>
		<div id='msgList'></div>
		<div class="yun_prompt_writing_operation" id='gourl'>
			<?php if ($_smarty_tpl->tpl_vars['is_member']->value) {?>
			<a href="index.php?c=jobadd" class="yun_prompt_writing_operation_bth" <?php if ($_GET['act']=='resumeInfo') {?>target="_parent"<?php }?>>我已完成以上操作</a>
			<?php } else { ?>
			<a href="javascript:location.reload();" class="yun_prompt_writing_operation_bth">我已完成以上操作</a>
			<?php }?>
		</div>
	</div>
	<!--提示部分  end-->
</div>

<div id="wxbd" style="display: none;">
	<div id="wxbdtitle" class="yun_wxbd_tit">关注微信公众号，手机轻松招人</div>
	<div class="yun_wxbd_box">
		<div class="yun_wxbd_img_c">
			<div class="yun_wxbd_img" id="wx_bind_qrcode">
				<img src="" width="180" height="180" />
			</div>
		</div>
		<div id="gzwx_sx" class="none">
			<div class="wx_login_show_sxbox"><a href="javascript:void(0);" onclick="getwxlogincode()" class="wx_login_show_sxicon"></a>二维码已失效点击刷新</div>
		</div>
		<div class="yun_wxbd_p"> 扫码关注公众号</div>
		<div class="yun_wxbd_ok">
			<a class="sx_bot_qx" href="javascript:void(0)" >好，知道了</a>
		</div>
	</div>
</div>
<!-- 发布职位弹出框end -->
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
<style>
	.layui-carousel{ background:none;}
	.layui-carousel>[carousel-item]>*{background:none;}
	.layui-carousel>[carousel-item]:before { display:none;}
	.layui-carousel-arrow{ margin-top:-48px;}
</style>

<?php echo '<script'; ?>
>
	var pricename = '<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
';

	var single_can = '';
	single_can = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_single_can'];?>
';
	var single_can_arr = single_can.split(",");

	var only_price = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_only_price'];?>
';
	var only_price_arr = only_price.split(',');
	
	/* 选择不同会员/增值包/单次购买  */
	$(function(){
		$("ul#rating_type").on("click", "li", function() {
	        $(this).addClass("buymeal_tit_cur");
	        $(this).siblings('li').removeClass("buymeal_tit_cur");
	        var rating = $(this).attr("rating");
	        if(rating) {
	        	$(".rating_type").hide();
	        	$("#rating_"+rating).show();
	        } 
	    })
	})
	/* 会员套餐滚动 */
	layui.use('carousel', function() {
		var carousel = layui.carousel;
		
		// 套餐会员
		carousel.render({
			elem : '#vip_1',
			width : '748px',// 设置容器宽度
			arrow : 'hover' // 悬停显示箭头
			,autoplay:false // 自动切换关闭
		});
		// 时间会员
		carousel.render({
			elem : '#vip_2',
			width : '748px',	
			autoplay:false // 自动切换关闭
	 	});
	});

    function firstTab(){
		var dx = $('#singleTab').attr('ishide');
		if (dx == 0) {
			$('.taoCanQuanyi').hide();
		}
    	$("ul#rating_type li").each(function(){
    		if($(this).attr('ishide')=='0'){
    			$(this).click();
    			return false
    		}
    	})
    }
<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>
	var gzsetval,
		gzsetwout;
		
	function getwxlogincode(){
		$.post('<?php echo smarty_function_url(array('m'=>'login','c'=>'wxlogin'),$_smarty_tpl);?>
',{t:1},function(data){
			if(data==0){
				$('#wx_bind_qrcode').html('二维码获取失败..');
			}else{
				$('#wx_bind_qrcode').html('<img src="'+data+'" width="180" height="180">');
				gzsetval = setInterval(function(){
					$.post('<?php echo smarty_function_url(array('m'=>'login','c'=>'getwxloginstatus'),$_smarty_tpl);?>
',{t:1},function(data){
						var data=eval('('+data+')');
						if(data.url!='' && data.msg!=''){
							clearInterval(gzsetval);
							gzsetval = null;
							layer.msg(data.msg, 2, 9,function(){window.location.href=data.url;});
						}else if(data.url){
							window.location.href=data.url;
						}
					});
				}, 2000);
				if(gzsetwout){
					clearTimeout(gzsetwout);
					gzsetwout = null;
				}
				gzsetwout = setTimeout(function(){
					clearInterval(gzsetval);
					gzsetval = null;
					var gzwx_sx = $("#gzwx_sx").html();
					$('#wx_bind_qrcode').html(gzwx_sx);
				},300*1000);
			}
		});
	}
	function gzhShow(){
		var wxlayer = layer.open({
			type: 1,
			title: false,
			closeBtn: 0,
			border: [10, 0.3, '#000', true],
			area: ['500px', 'auto'],
			content: $("#wxbd"),
			end: function(){
				if(gzsetwout){
					clearTimeout(gzsetwout);
					gzsetwout = null;
				}
				if(gzsetval){
					clearInterval(gzsetval);
					gzsetval = null;
				}
			}
		});
		getwxlogincode();
		$(".sx_bot_qx").click(function () {
			layer.close(wxlayer);
		})
	}
<?php echo '</script'; ?>
><?php }} ?>
