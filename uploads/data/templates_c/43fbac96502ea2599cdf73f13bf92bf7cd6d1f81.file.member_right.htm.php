<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:07
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/member_right.htm" */ ?>
<?php /*%%SmartyHeaderCode:102137362669e8a0d75b6b39-99624965%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '43fbac96502ea2599cdf73f13bf92bf7cd6d1f81' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/member_right.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '102137362669e8a0d75b6b39-99624965',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'statis' => 0,
    'vipIsDown' => 0,
    'todayStart' => 0,
    'noPermission' => 0,
    'isPaused' => 0,
    'rows' => 0,
    'v' => 0,
    'meal' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d75d96a3_78169302',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d75d96a3_78169302')) {function content_69e8a0d75d96a3_78169302($_smarty_tpl) {?><?php if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


<div class="w1000">
    <div class="admin_mainbody">
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        <div class=right_box>
            <div id="grow_freedom" class="grow_mod_wrap my_freedom" style="padding:0px;border:none;">
             <div class="newmember_tit">
				<ul>
					<li class="newmember_titcur"> <a href="index.php?c=right">会员服务</a></li>
					<li> <a  href="index.php?c=pay" >充值服务</a></li>
					<li> <a  href="index.php?c=paylogtc" class=" ">我的服务</a></li>
					<li><a  href="index.php?c=integral"><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
管理</a></li>
				</ul>
			</div>
			<div class="clear"></div> 
             
             <div class="newmember_screenbox">
                <div class="newmember_screen">
                    <ul>
                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_vip_type']==2) {?>
                            <li class="job_list_tit_cur"><a href="index.php?c=right" >套餐会员</a></li>
                        <?php }?>
                    
                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_vip_type']==1) {?>
                            <li><a href="index.php?c=right&act=time" >时间会员</a></li>
                        <?php }?>
                    
                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_vip_type']==0) {?>
                            <li class="job_list_tit_cur"><a href="index.php?c=right" >套餐会员</a></li>
                            <li><a href="index.php?c=right&act=time" >时间会员</a></li>
                        <?php }?>
                    
                        <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']!=2&&!$_smarty_tpl->tpl_vars['vipIsDown']->value&&$_smarty_tpl->tpl_vars['config']->value['com_integral_online']!=4) {?>
                        <li><a href="index.php?c=right&act=added" >增值包</a></li>
                        <?php }?>
                        </ul>
             </div> 
             </div> 
             
            <div class="clear"></div>
                    
            <div class=admincont_box>
                     
                        <div class="com_body">  
                        <iframe id="fdingdan"  name="fdingdan" onload="returnmessage('fdingdan');" style="display:none"></iframe>

                     
                        <div class="com_new_tip">
                                
                                                <span class="com_new_tip_h">温馨小提示</span>您当前是：<?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_name'];?>

                            服务到期为：
                            <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value) {?>
                                <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_stime'],'%Y-%m-%d');?>
 - <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_etime'],'%Y-%m-%d');?>

                            <?php } elseif ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']==0) {?>
                                <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_stime'],'%Y-%m-%d');?>
 - 永久
                            <?php } else { ?>
                                <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_stime'],'%Y-%m-%d');?>
 -
                                <span class="comindex_money_pd_s"><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_etime'],'%Y-%m-%d');?>
</span>
                                开通企业套餐，尊享招聘特权
                            <?php }?>
                                               
                                            </div>
                         
                        <div class="vip_box">
                            <?php if ($_smarty_tpl->tpl_vars['noPermission']->value==1||$_smarty_tpl->tpl_vars['isPaused']->value==1) {?>
                            <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['temstyle']->value)."/default/public_search/vipTips.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

                            <?php } else { ?>
                            <div class="vip_box_db">套餐权益对比</div>
                            <ul>
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <li class="vip_box_list">
                                    <div class="vip_box_list_c">
                                    <div class="vip_box_left">
                                        <div class="vip_box_left_name"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
<i class="vip_box_left_name_line"></i></div>
                                        <div class="vip_box_left_money_b">
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==3&&!$_smarty_tpl->tpl_vars['meal']->value) {?>
                                                <span class="" id="need_integral<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
">
                                                    <?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
                                                        <?php echo $_smarty_tpl->tpl_vars['v']->value['yh_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>

                                                    <?php } else { ?>
                                                        <?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];
}?>
                                                </span>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
                                                    <div class="vip_box_left_money_yj">
                                                        原价：<i><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price']*$_smarty_tpl->tpl_vars['config']->value['integral_proportion'];?>
</i>
                                                    </div>
                                                <?php }?>
                                            <?php } else { ?>
                                                <span class="vip_box_left_money" id="need_price<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
">

                                                    <?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
                                                        <?php echo $_smarty_tpl->tpl_vars['v']->value['yh_price'];?>

                                                    <?php } else { ?>
                                                        <?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>

                                                    <?php }?></span>元/<span><?php if ($_smarty_tpl->tpl_vars['v']->value['service_time']>0) {
echo $_smarty_tpl->tpl_vars['v']->value['service_time'];?>
天<?php } else { ?>永久<?php }?>
                                                </span>
                                                <?php if ($_smarty_tpl->tpl_vars['v']->value['time_start']<time()&&$_smarty_tpl->tpl_vars['v']->value['time_end']>time()) {?>
                                                    <div class="vip_box_left_money_yj">原价：<i><?php echo $_smarty_tpl->tpl_vars['v']->value['service_price'];?>
</i></div>
                                                <?php }?>
                                            <?php }?>
                                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_integral_online']==3&&!$_smarty_tpl->tpl_vars['meal']->value) {?>
                                                <?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>

                                            <?php }?>
                                        </div>
                                        <a href="javascript:buyVip('<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
');" class="vip_box_kt">立即开通</a>
                                    </div>
                                    <div class="vip_box_table">
                                        <div class="vip_box_data">上架职位数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['job_num'];?>
</span></div>
                                        <div class="vip_box_data">刷新职位数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['breakjob_num'];?>
</span></div>
                                        <div class="vip_box_data">下载简历数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['resume'];?>
</span></div>
                                        <div class="vip_box_data">邀请面试数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['interview'];?>
</span></div>
                                        <div class="vip_box_data">招聘会次数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['zph_num'];?>
</span></div>
                                        <div class="vip_box_data">置顶天数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['top_num'];?>
</span></div>
                                        <div class="vip_box_data">紧急天数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['urgent_num'];?>
</span></div>
                                        <div class="vip_box_data">推荐天数<span class="vip_box_data_n"><?php echo $_smarty_tpl->tpl_vars['v']->value['rec_num'];?>
</span></div>
                                        
                                    </div>
                                    <div class="vip_box_sm">
                                        <span class="vip_box_sm_s">套餐说明：</span>

                                        <?php if ($_smarty_tpl->tpl_vars['v']->value['explains']) {
echo $_smarty_tpl->tpl_vars['v']->value['explains'];
} else {
echo $_smarty_tpl->tpl_vars['v']->value['name'];
}?>

                                        <?php if ($_smarty_tpl->tpl_vars['v']->value['integral_buy']>0) {?>赠送<?php echo $_smarty_tpl->tpl_vars['v']->value['integral_buy'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];
}?>


                                    </div></div>
                                </li>
                                <?php }
if (!$_smarty_tpl->tpl_vars['v']->_loop) {
?>
                                <div class="msg_no">
                                    <p>亲爱的用户，目前没有套餐会员服务</p>
                                </div>
                                <?php } ?>
                            </ul>
                            <?php }?>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .serve_zz_h_nmb_tcjg_box{width:100%; text-align:center; padding-bottom:10px;}
    .serve_zz_h_nmb_tcjg{ font-size:14px;color:#999; text-decoration:line-through; font-weight:normal}
</style>
<?php echo '<script'; ?>
>
    
    $(function(){
        $(".shuoming").hover(
            function(){
                $(this).find('.com_grade_smbox_cont').show();
            },
            function(){
                $(this).find('.com_grade_smbox_cont').hide();
            }
        );
    });

    $(document).ready(function() {
        $(".com_grade_smicon").hover(function() {
            var pid=$(this).attr("pid");
            $("#rating"+pid).show();
            $("#ratingtu"+pid).show();
        },function(){
            var pid=$(this).attr("pid");
            $("#rating"+pid).hide();
            $("#ratingtu"+pid).hide();
        });
    });
 
<?php echo '</script'; ?>
> 
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
