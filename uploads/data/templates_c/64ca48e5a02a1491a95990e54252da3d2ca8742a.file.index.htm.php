<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:22:12
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:71423758369e8a154cce665-77789331%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '64ca48e5a02a1491a95990e54252da3d2ca8742a' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/index.htm',
      1 => 1706496289,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '71423758369e8a154cce665-77789331',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'com_style' => 0,
    'config' => 0,
    'des_resume' => 0,
    'de_resume' => 0,
    'atn' => 0,
    'look_jobnum' => 0,
    'statis' => 0,
    'vipIsDown' => 0,
    'JobNum' => 0,
    'lunbo' => 0,
    'company' => 0,
    'member' => 0,
    'hitsNum' => 0,
    'expoureNum' => 0,
    'company_rating' => 0,
    'todayStart' => 0,
    'hbNum' => 0,
    'jobs' => 0,
    'addjobnum' => 0,
    'jobids' => 0,
    'isPaused' => 0,
    'yrbtn' => 0,
    'guweninfo' => 0,
    'kfqq' => 0,
    'report' => 0,
    'sy_yearreport_tip' => 0,
    'uid' => 0,
    'yrshow' => 0,
    'normal_job_num' => 0,
    'un_refreshjob_num' => 0,
    'qrcode' => 0,
    'ggnum' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a154d0cc47_95657676',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a154d0cc47_95657676')) {function content_69e8a154d0cc47_95657676($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
>
    function Next() {
        $("#one_tip").hide();
        $("#two_tip").show();
    }
    function showindextip(id) {
        $(".indextiphide").hide();
        $("#indextiphide" + id).show();
    }
    function hideindextip() {
        $(".indextiphide").hide();
        $("#bg").hide();
    }
<?php echo '</script'; ?>
>

<div class="clear"></div>
<div class="memberSubject">
    <div class="admin_mainbody">
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        <div class="memberSubCont">
            <div class="memberSubRight">
                <div class="membRighTops">
                    <ul>
                        <li class="membRighTops_mr">
                            <div class="membRiTopText">
                                <div class="membRiTopInfo">
                                    <span>简历投递</span>
                                    <img class="laytips" id="jltdtips" data-id="jltdtips" data-title="用户投递简历累计数量"
                                         src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tiosa.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                </div>
                                <div class="membRiTopNum">
                                    <span> <a href="index.php?c=hr" target="_blank"><?php echo $_smarty_tpl->tpl_vars['des_resume']->value;?>
</a></span>
                                </div>
                                <?php if ($_smarty_tpl->tpl_vars['de_resume']->value>0) {?>
                                <div class="membRiTopInx">
                                    <span>有新简历没看，</span>
                                    <a href="index.php?c=hr" target="_blank">去处理></a>
                                </div>
                                <?php }?>
                            </div>
                            <div class="membRiTopImg">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/memimg1.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                            </div>
                        </li>
                        
                        <li>
                            <div class="membRiTopText">
                                <div class="membRiTopInfo">
                                    <span>对我感兴趣</span>
                                    <img class="laytips" id="gxqtips" data-id="gxqtips" data-title="关注我的人才"
                                         src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tiosa.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                </div>
                                <div class="membRiTopNum">
                                   <a href='index.php?c=attention_me' target="_blank">  <span><?php echo $_smarty_tpl->tpl_vars['atn']->value;?>
</span></a>
                                </div>
                                <div class="membRiTopInx">
                                    <span>意向人才，等你来聊，</span>
                                     <a href='index.php?c=attention_me' target="_blank">详情></a>
                                </div>
                            </div>
                            <div class="membRiTopImg">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/memimg3.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                            </div>
                        </li>
                        <li>
                            <div class="membRiTopText">
                                <div class="membRiTopInfo">
                                    <span>谁看过我</span>
                                    <img class="laytips" id="lookjobtips" data-id="lookjobtips" data-title="看过我发布职位的简历"
                                         src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tiosa.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                </div>
                                <div class="membRiTopNum">
                                   <a href='index.php?c=look_job' target="_blank">  <span><?php echo $_smarty_tpl->tpl_vars['look_jobnum']->value;?>
</span></a>
                                </div>
                                <div class="membRiTopInx">
                                    <span>看过我的人才，</span>
                                     <a href='index.php?c=look_job' target="_blank">详情></a>
                                </div>
                            </div>
                            <div class="membRiTopImg">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/memimg4.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                            </div>
                        </li>
                    </ul>
                </div>
            
                <div class="memberSubGuanl">
                
                               <div class="memberSubzaopi">
                            <div class="membSubGuanTite">
                                <span>招聘资源</span>  
                                <?php if ($_smarty_tpl->tpl_vars['statis']->value['rating_type']==1&&!$_smarty_tpl->tpl_vars['vipIsDown']->value&&$_smarty_tpl->tpl_vars['config']->value['com_integral_online']!=4) {?><a href="index.php?c=right&act=added" class=" ">购买资源点 ></a><?php }?> 
                            </div> 
                            <div class="membSubGuaTwo">
                                <ul>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>可上架职位数</span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['JobNum']->value)===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>份</b>
                                        </div>
                                         
                                    </li>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>可刷新职位数</span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['breakjob_num'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>条</b>
                                        </div>
                                       
                                    </li>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>可邀请面试数</span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['invite_resume'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>条</b>
                                        </div>
                                        
                                    </li>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>可下载简历数</span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['down_resume'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>条</b>
                                        </div>
                                       
                                    </li>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>可招聘会数 </span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['zph_num'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>次</b>
                                        </div>
                                       
                                    </li>
                                    
                                    <li>
                                    <div class="twoDivimg">
                                        <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                    </div>
                                    <div class="twoDivTite">
                                        <span>置顶天数</span>
                                    </div>
                                    <div class="twoDivNum">
                                        <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['top_num'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                        <b>天</b>
                                    </div>
                                                                           
                                     
                                    </li>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>紧急天数</span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['urgent_num'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>天</b>
                                        </div>
                                 
                                    </li>
                                    <li>
                                        <div class="twoDivimg">
                                            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/tuayuan.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                        </div>
                                        <div class="twoDivTite">
                                            <span>推荐天数</span>
                                        </div>
                                        <div class="twoDivNum">
                                            <span><?php echo (($tmp = @$_smarty_tpl->tpl_vars['statis']->value['rec_num'])===null||$tmp==='' ? 0 : $tmp);?>
</span>
                                            <b>天</b>
                                        </div>
                                  
                                    </li>
                                    
                                </ul>
                            </div>
                        </div>
                   
                
                </div>
               
                <div class="memberSubBanner">
                    <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"530","item"=>"\"lunbo\"","key"=>"“key“","random"=>"1","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[530];if(is_array($add_arr) && !empty($add_arr)){
				$i=0;$limit = 0;$length = 0;
				foreach($add_arr as $key=>$value){
					if($config['did']){
						if(($value['did']==$config['did']|| $value['did']==-1)&&$value['start']<time()&&$value['end']>time()){
							if($i>0 && $limit==$i){
								break;
							}
							if($length>0){
								$value['name'] = mb_substr($value['name'],0,$length);
							}
							if($paramer['type']!=""){
								if($paramer['type'] == $value['type']){
									$AdArr[] = $value;
								}
							}else{
								$AdArr[] = $value;
							}
							$i++;
						}
						
					}else{
						if(($value['did']==-1 || !$value['did']) && $value['start']<time()&&$value['end']>time()){
							if($i>0 && $limit==$i){
								break;
							}
							if($length>0){
								$value['name'] = mb_substr($value['name'],0,$length);
							}
							if($paramer['type']!=""){
								if($paramer['type'] == $value['type']){
									$AdArr[] = $value;
								}
							}else{
								$AdArr[] = $value;
							}
							$i++;
						}
						
					}
				}
				if (isset($attr['random']) && $attr['random'] && count($AdArr) > $attr['random']) {
			        $temp = [];
			        $random_keys = array_rand($AdArr, $attr['random']);

			        if($attr['random'] == 1) {
			            $temp[] = $AdArr[$random_keys];
			        } else {
			            foreach ($AdArr as $key => $value) {
			                if (in_array($key, $random_keys)) {
			                    $temp[$key] = $value;
			                }
			            }
			        }
			        $AdArr = $temp;
		        }
			}$AdArr = $AdArr; if (!is_array($AdArr) && !is_object($AdArr)) { settype($AdArr, 'array');}
foreach ($AdArr as $_smarty_tpl->tpl_vars["lunbo"]->key => $_smarty_tpl->tpl_vars["lunbo"]->value) {
$_smarty_tpl->tpl_vars["lunbo"]->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars["lunbo"]->key;
?>
                   <?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
 
                    <?php } ?>
                </div>
                <div class="membSubPoster"> 
				<?php if (($_smarty_tpl->tpl_vars['company']->value['r_status']!=1||!$_smarty_tpl->tpl_vars['company']->value['name']||$_smarty_tpl->tpl_vars['company']->value['yyzz_status']!=1||$_smarty_tpl->tpl_vars['vipIsDown']->value||($_smarty_tpl->tpl_vars['statis']->value['remind']==1&&!$_smarty_tpl->tpl_vars['vipIsDown']->value))) {?>
                <!--滚动-->
                  <div class="yun_Announcement" >
                    <ul class="tiplist">
                        <?php if ($_smarty_tpl->tpl_vars['company']->value['r_status']!=1) {?>
                            <?php if ($_smarty_tpl->tpl_vars['company']->value['r_status']==0) {?>
                                <li class="one">温馨提示：<span>您的帐号尚未审核，职位将在审核通过后展示</span><span id="forms">暂时无法查看简历的联系方式，我们会24小时内进行审核请保持电话畅通</span></li>
                            <?php } elseif ($_smarty_tpl->tpl_vars['company']->value['r_status']==2||$_smarty_tpl->tpl_vars['company']->value['r_status']==4) {?>
                                <li class="one">温馨提示：您的帐号已被锁定，锁定原因请咨询客服（<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel']!='' ? $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel'] : $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
）</li>
                            <?php } elseif ($_smarty_tpl->tpl_vars['company']->value['r_status']==3) {?>
                                <li class="one">温馨提示：您的帐号未通过审核<?php if ($_smarty_tpl->tpl_vars['member']->value['lock_info']) {?>，未通过原因：<?php echo $_smarty_tpl->tpl_vars['member']->value['lock_info'];?>
。 <?php } else { ?>。<?php }?>欢迎随时与我们取得联系（<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel']!='' ? $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel'] : $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
）</li>
                            <?php }?>
                        <?php }?>
                        <?php if (!$_smarty_tpl->tpl_vars['company']->value['name']) {?>
                            <li class="three">温馨提示：贵公司的资料还未填写完整，暂时还不可以发布职位哦! <span class="tiplist_bth"><a href="index.php?c=info">完善资料></a></span></li>
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['company']->value['yyzz_status']!=1) {?>
                            <li class="two">温馨提示：提升招聘效果，获得人才信任度，从认证企业资质开始~ <span class="tiplist_bth"><a href="index.php?c=binding">立即认证></a></span></li>
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['vipIsDown']->value) {?>
                            <li class="four">温馨提示：您办理的会员已到期！为保证您的服务正常使用，请尽快开通会员套餐，开通会员套餐有优惠哦~ <span class="tiplist_bth"><a href="index.php?c=right">查看详细></a></span></li>
                        <?php } elseif ($_smarty_tpl->tpl_vars['statis']->value['remind']==1&&!$_smarty_tpl->tpl_vars['vipIsDown']->value) {?>
                            <li class="four">温馨提示：您办理的会员将于近期到期！为保证您的服务正常使用，请尽快办理续费，续费套餐有优惠哦~ <span class="tiplist_bth"><a href="index.php?c=right">查看详细></a></span></li>
                        <?php }?>
                    </ul>
                </div>  
                <!--滚动-->
                <?php }?>
                    <div class="membSuosTite">
                        <span>智能推荐</span>
                        <a href="<?php echo smarty_function_url(array('m'=>'resume'),$_smarty_tpl);?>
">更多></a>
                    </div>
                    <div class="membSubposCont">
                        <ul id="resumelist"></ul>
                    </div>
                    <div class="msg_no none" id="noresumelist">
                        <p>暂无符合智能匹配职位条件的人才</p>
                         <a href="<?php echo smarty_function_url(array('m'=>'resume'),$_smarty_tpl);?>
" target="_blank" class="com_msg_no_bth com_submit ">手动筛选人才</a>
                    </div>
                </div>
            </div>
            <div class="memberSubLeft"> 
             
                <div class="membSubLeComs">
                    <div class="membLeComData">
                        <div class="membLeComLogo">
                            <a href="index.php?c=uppic">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['company']->value['logo'];?>
" alt="">
                            </a>
                        </div>
                        <div class="membLeComNam">
                            <?php if ($_smarty_tpl->tpl_vars['company']->value['name']) {?>
                            <div class="membLeComText">
                                <span><?php echo $_smarty_tpl->tpl_vars['company']->value['name'];?>
 </span>
                            </div>
                            <div class="membLeComslink">
                                <a href="index.php?c=info">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/qiye2.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                    <span>基本信息已完善></span>
                                </a>
                            </div>
                            <?php } else { ?>
                            <div class="membLeComslink">
                                <a href="index.php?c=info">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/qiye2.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                    <span style="color:#f60">您还未完善企业信息，点击完善！></span>
                                </a>
                            </div>
                            <?php }?>
                        </div>
                    </div>
               
                    <div class="com_n_data">
                         <div class="com_n_data_a"><div class="com_n_data_n"><?php echo $_smarty_tpl->tpl_vars['hitsNum']->value;?>
</div>浏览量 </div>
                         <div class="com_n_data_a"><div class="com_n_data_n"><?php echo $_smarty_tpl->tpl_vars['expoureNum']->value;?>
</div>曝光量 </div>   
						 <div class="com_n_data_a"><div class="com_n_data_n"><?php echo $_smarty_tpl->tpl_vars['statis']->value['integral'];?>
</div><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
 </div>             
                    </div>
                    
               
                 
                     <div class="new_com_vip">
                         <div class="new_com_vip_t">
                             <div class="new_com_vip_icon">
                                 <?php if ($_smarty_tpl->tpl_vars['company_rating']->value['com_pic']) {?>
                                 <img src="<?php echo $_smarty_tpl->tpl_vars['company_rating']->value['com_pic'];?>
" width="22" height="22" />
                                 <?php }?>
                             </div>
                             <div class="new_com_vip_name"><?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_name'];?>
</div>
                             <div>
                                 <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>$_smarty_tpl->tpl_vars['todayStart']->value) {?>
                                     <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_stime'],'%Y.%m.%d');?>
 - <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_etime'],'%Y.%m.%d');?>

                                 <?php } elseif ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']==0) {?>
                                     开通会员提升招聘效率
                                 <?php } else { ?>
                                     <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_stime'],'%Y.%m.%d');?>
 - <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_etime'],'%Y.%m.%d');?>

                                 <?php }?>
                             </div>
                         </div> 
                         <div class="new_vip ">
                        <div class="new_vip_a"><a href="index.php?c=paylogtc" > 我的服务</a></div>            
                        <div class="new_vip_a"><a href="index.php?c=integral" ><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
管理  </a></div>
                          <div class="new_vip_a"><a href="index.php?c=pay" >充值</a> </div>  
                         </div>
                             <a href="index.php?c=right" class="new_com_vip_bth">立即开通</a>
                             <?php if ($_smarty_tpl->tpl_vars['statis']->value['vip_etime']>=$_smarty_tpl->tpl_vars['todayStart']->value) {?>
                             <a href="index.php?c=right" class="new_com_vip_bth">会员升级</a>
                             <?php }?>
                     </div>
                 
                  <div class="con_new_mininav">
                  <div class="con_new_mininav_a"><a  href="index.php?c=zhaopin"> <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/nav1.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">招聘数据</a></div>           
                  <div class="con_new_mininav_a"><a  href="index.php?c=binding" ><img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/nav2.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">企业认证</a></div>               
                  <div class="con_new_mininav_a"><a  href="index.php?c=binding" ><img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/nav3.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">账号设置</a></div>    
                     </div>
                       <div class="con_new_mininav">
                   <div class="con_new_mininav_a"><a  href="index.php?c=look_resume" ><img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/nav4.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">浏览记录 </a></div>   
                                 <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_haibao_isopen']==1&&$_smarty_tpl->tpl_vars['hbNum']->value>0) {?>
                                 <div  class="con_new_mininav_a" onclick="selectHb('<?php echo count($_smarty_tpl->tpl_vars['jobs']->value);?>
')">
                                    
                                     <a href="javascript:;"><img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/nav5.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">招聘海报</a>
                                 </div>
                                 <?php }?>
                                 
                   <div class="con_new_mininav_a"><a  href="<?php echo smarty_function_url(array('m'=>'company','c'=>'show','id'=>'`$uid`'),$_smarty_tpl);?>
" target="_blank"><img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/nav6.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">企业主页</a></div>    
                   </div>
                   
                    
                    
                <div class="membLeCoNavs">
                         <div class="fbjob">
                             <a href="javascript:void(0)" onclick="jobadd_url('<?php echo $_smarty_tpl->tpl_vars['addjobnum']->value;?>
')">
                                 <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/fbi.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                 <span>发布职位</span>
                             </a>
                         </div>
                         <div class="sxjob">
                             <a href="javascript:;" onclick="return refreshJob('<?php echo $_smarty_tpl->tpl_vars['jobids']->value;?>
','<?php echo $_smarty_tpl->tpl_vars['statis']->value['upJobNum'];?>
', '', '<?php echo $_smarty_tpl->tpl_vars['isPaused']->value;?>
')">
                                 <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/sxi.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                 <span>刷新职位</span>
                             </a>
                         </div>
                </div>   </div>
                <!-- 广告位 Start-->
                <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"511","item"=>"\"lunbo\"","key"=>"“key“","random"=>"1","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[511];if(is_array($add_arr) && !empty($add_arr)){
				$i=0;$limit = 0;$length = 0;
				foreach($add_arr as $key=>$value){
					if($config['did']){
						if(($value['did']==$config['did']|| $value['did']==-1)&&$value['start']<time()&&$value['end']>time()){
							if($i>0 && $limit==$i){
								break;
							}
							if($length>0){
								$value['name'] = mb_substr($value['name'],0,$length);
							}
							if($paramer['type']!=""){
								if($paramer['type'] == $value['type']){
									$AdArr[] = $value;
								}
							}else{
								$AdArr[] = $value;
							}
							$i++;
						}
						
					}else{
						if(($value['did']==-1 || !$value['did']) && $value['start']<time()&&$value['end']>time()){
							if($i>0 && $limit==$i){
								break;
							}
							if($length>0){
								$value['name'] = mb_substr($value['name'],0,$length);
							}
							if($paramer['type']!=""){
								if($paramer['type'] == $value['type']){
									$AdArr[] = $value;
								}
							}else{
								$AdArr[] = $value;
							}
							$i++;
						}
						
					}
				}
				if (isset($attr['random']) && $attr['random'] && count($AdArr) > $attr['random']) {
			        $temp = [];
			        $random_keys = array_rand($AdArr, $attr['random']);

			        if($attr['random'] == 1) {
			            $temp[] = $AdArr[$random_keys];
			        } else {
			            foreach ($AdArr as $key => $value) {
			                if (in_array($key, $random_keys)) {
			                    $temp[$key] = $value;
			                }
			            }
			        }
			        $AdArr = $temp;
		        }
			}$AdArr = $AdArr; if (!is_array($AdArr) && !is_object($AdArr)) { settype($AdArr, 'array');}
foreach ($AdArr as $_smarty_tpl->tpl_vars["lunbo"]->key => $_smarty_tpl->tpl_vars["lunbo"]->value) {
$_smarty_tpl->tpl_vars["lunbo"]->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars["lunbo"]->key;
?>
                <div class="yun_combanner"><?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
</div>
                <?php } ?>
                <!-- 广告位 End-->
                <!-- 年度报告-->
                <?php if ($_smarty_tpl->tpl_vars['yrbtn']->value) {?>
                    <div class="comIndexButn" onclick="yearreport()"> <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/ndtj.gif?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt=""> </div>
                <?php }?>
                <!-- 年度报告-->
                <div class="membSubLeKrfu">
                  
                     <div class="membLeKfuBand">
                         <?php if ($_smarty_tpl->tpl_vars['config']->value['wx_author']=='1'&&$_smarty_tpl->tpl_vars['member']->value['subscribe']!='1') {?>
                         <div class="membLeKfuBaGren membLeKfusuz">
                             <div class="membLeKfuBaIcon">
                                 <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/wechat1.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                 <span>未绑定</span>
                             </div>
                             <div class="membLeKfuBaLink">
                                 <a href="javascript:gzhShow();">绑定微信公众号></a>
                             </div>
                         </div>
                         <?php }?>
                         <?php if ($_smarty_tpl->tpl_vars['company']->value['yyzz_status']!='1') {?>
                         <div class="membLeKfuBaBlue membLeKfusuz">
                             <div class="membLeKfuBaIcon">
                                 <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/wechat2.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                                 <?php if ($_smarty_tpl->tpl_vars['company']->value['yyzz_status']=='2') {?>
                                 <span>未通过</span>
                                 <?php } else { ?>
                                 <span>未认证</span>
                                 <?php }?>
                             </div>
                             <div class="membLeKfuBaLink">
                                 <a href="index.php?c=binding&act=comcert">认证营业执照>></a>
                             </div>
                         </div>
                         <?php }?>
                     </div>
                    <div class="membLeCoTitel">
                        <span>专属客服</span>
                    </div>
                    <?php if ($_smarty_tpl->tpl_vars['guweninfo']->value['uid']) {?>
                    <div class="membLeCoTips">
                        <p>
                            你好，尊敬的
                            <span><?php echo $_smarty_tpl->tpl_vars['statis']->value['rating_name'];?>
</span>
                            我是你的专属招聘顾问,有问题请联系我!
                            <?php if ($_smarty_tpl->tpl_vars['guweninfo']->value['qq']) {?>
                            <a target="_blank" href="tencent://message/?uin=<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['qq'];?>
&Site=<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
&Menu=yes">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/qqimg.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                            </a>
                            <?php }?>
                        </p>
                    </div>
                    <div class="membLeCoUser">
                        <div class="membLeCoUsData">
                            <div class="membLeCoUsImg">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['photo_n'];?>
" style="width:40px; height: 40px" alt="">
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['guweninfo']->value['moblie']) {?>
                            
                            <div class="membLeCoUsTell">
                                <span class="membLeCoUsTellname"><?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['name'];?>
</span>
                                <span>电话：<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['moblie'];?>
</span>
                            </div>
                            <?php }?>
                        </div>
                        <?php if ($_smarty_tpl->tpl_vars['guweninfo']->value['ewm_n']) {?>
                        <div class="membLeCoUsButn">
                            <a href="javascript:kfwxShow('<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['ewm_n'];?>
');">加我微信</a>
                        </div>
                        <?php }?>
                        
                    </div>
                   
                    <?php } else { ?>
                    <div class="membLeCoTips">
                        <p>
                            网站客服
                            <?php if ($_smarty_tpl->tpl_vars['kfqq']->value) {?>
                            <a target="_blank" href="tencent://message/?uin=<?php echo $_smarty_tpl->tpl_vars['kfqq']->value;?>
&Site=<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
&Menu=yes">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/qqimg.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                            </a>
                            <?php }?>
                        </p>
                    </div>
                    <div class="membLeCoUser">
                        <div class="membLeCoUsData">
                            <div class="membLeCoUsImg">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_guwen'];?>
" style="width:40px; height: 40px" alt="">
                            </div>
                            <div class="membLeCoUsTell">
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_comwebtel']||$_smarty_tpl->tpl_vars['config']->value['sy_freewebtel']) {?>
                                <span>电话：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel']!='' ? $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel'] : $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
</span>
                                <?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_webmoblie']) {?>
                                <span>手机：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webmoblie'];?>
</span>
                                <?php }?>
                            </div>
                        </div>
                        <?php if (!empty($_smarty_tpl->tpl_vars['config']->value['sy_wx_qcode'])) {?>
                        <div class="membLeCoUsButn">
                            <a href="javascript:kfwxShow('<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wx_qcode'];?>
');">加我微信</a>
                        </div>
                        <?php }?>
                    </div>
                    <?php }?>
                    <div class="membLeCoTitel">
                        <span>向平台反馈</span>
                    </div>
                    <div class="membLeCofkui">
                        <p>若您无法通过在线客服解决问题时，可以向平台反馈</p>
                        <?php if (is_array($_smarty_tpl->tpl_vars['report']->value)&&$_smarty_tpl->tpl_vars['report']->value['result']) {?>
                        <a href="index.php?c=report&act=show">已反馈</a>
                        <?php } elseif ($_smarty_tpl->tpl_vars['guweninfo']->value['uid']) {?>
                        <a href="javascript:void(0)" onclick="reportgw('<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['uid'];?>
','投诉顾问');">我要反馈</a>
                        <?php }?>
                    </div>
                
                </div>
            </div>
            
        </div>
    </div>
</div>
<!--投诉顾问弹出框-->
<div id="<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['uid'];?>
" style="display: none;">
    <div class="Binding_pop_box" style="padding: 10px; width: 350px; height: 200px; background: #fff;">
        <div class="complaint_hi">尊敬的用户您好！</div>
        <div class="complaint_p">为了能够给您提供高质量的服务，反馈具体情况，我们会第一时间给您满意的答复！</div>
        <div class="complaint_p_gw" style="padding: 10px 0; color: #f00">您要投诉的顾问是：<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['name'];?>
</div>
        <div class="popjb_tip"></div>
        <div class="">
            <textarea id="reason" name="reason" class="complaint_text"></textarea>
        </div>
        <div class="complaint_bot">
            <input class="com_pop_bth_qd" onclick="reportSub('index.php?c=report')" type="button" value="确定">
            <input type='hidden' value="<?php echo $_smarty_tpl->tpl_vars['guweninfo']->value['uid'];?>
" id='eid' name='eid'>
            <input class="com_pop_bth_qx" type="button" value="取消" onclick="layer.close($('#layindex').val());">
        </div>
    </div>
</div>
<!--刷新职位提示弹出框-->
<div class="shuaxinDome">
    <div id="shuaxin" style="display: none;">
        <div class="yun_prompt_writingicon" style="padding-top: 0px">
            <!-- <i class="yun_prompt_writingicon_pm"></i> -->
            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/compimg1.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
        </div>
        <div class="yun_prompt_writing">刷新职位 , 提升排名</div>
        <div class="yun_prompt_writing_obtain">刷新职位提升职位显示排名，引起求职者的关注，提高投递率</div>
        <!-- <div class="yun_prompt_writing_tip">刷新职位，可提升职位的显示排名，从而提高职位的曝光量</div> -->
        <div class="yun_prompt_writing_operation">
            <a href="javascript:void(0)" onclick="refreshJob('<?php echo $_smarty_tpl->tpl_vars['jobids']->value;?>
','<?php echo $_smarty_tpl->tpl_vars['statis']->value['breakjob_num'];?>
');" class="yun_prompt_writing_operation_bth">立即刷新职位</a>
            <!-- <a class="sx_bot_qx" href="javascript:void(0)">暂不刷新</a> -->
        </div>
        <div class="shuxClose sx_bot_qx">
            <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/yuncloseicon.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
        </div>
    </div>
</div>
<!-- 会员到期，续费提醒 -->
<div id="updaterating" style="display: none;">
    <div class="renew_box">
        <div class="renew_hello">尊敬的企业用户您好</div>
        <div class="renew_p">
            你的会员服务<font color="#f00">已到期</font>，服务期限：<font color="#999"><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_stime'],'%Y.%m.%d');?>
-<?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['statis']->value['vip_etime'],'%Y-%m-%d');?>
</font>
            为了不影响您正常使用招聘服务，您可以致电客户经理或自助办理续费
        </div>
        <div class="renew_gx">非常感谢您一直以来的支持与厚爱！</div>
        <div class="renew_xf">
            <a class="renew_bth" href="index.php?c=right">自助续费</a>
            <a class="renew_bth_qx" href="javascript:void(0)">先看看 ,再续费</a>
        </div>
    </div>
</div>
<!--年度报告提示-->
<div class="yearrepBaog">
    <div id="yearreport_tip" class="" style="display: none; background-color: none;">
        <div class="companyYears">
            <div class="companyYeaImg">
                <img onclick="yearreport()" src="<?php echo $_smarty_tpl->tpl_vars['sy_yearreport_tip']->value;?>
" style="width: 300px;" />
            </div>
            <a href="javascript:void(0)" class="comClos" onclick="closeyr();">
                <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/close.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
            </a>
        </div>
    </div>
</div>
<div id="wxcontent" style=" display:none;">
    <div class="yun_wxbd_box">
        <div class="yun_wxbd_img_c">
            <div id="wx_login_qrcode" class="yun_wxbd_img" style="border:1px solid #eee; line-height:180px;">
                正在获取二维码...
            </div>
            <div id="wx_sx" class="none">
                <div class="wx_login_show_sxbox">
                    <a href="javascript:void(0);" onclick="getwxbindcode()" class="wx_login_show_sxicon"></a>二维码已失效点击刷新
                </div>
            </div>
        </div>
        <div class="yun_wxbd_p"> 请使用微信扫描二维码绑定</div>
    </div>
</div>
<div class="wxbdtSubject">
    <div id="kfwx" style="display: none;">
        <div class="yun_wxbd_tit">扫码添加专属客服微信</div>
        <div class="yun_wxbd_box">
            <div class="yun_wxbd_img_c">
                <div class="yun_wxbd_img">
                    <img id="kfwx_qrcode" src="" width="180" height="180" />
                </div>
            </div>
            <div class="yun_wxbd_p"></div>
            <div class="dsdsdssaed">
                <a id="kfwx_close" href="javascript:void(0)" >
                    <img src="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/colsd2.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" alt="">
                </a>
            </div>
        </div>
    </div>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/public_search/hb.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<!--提示弹出框 end-->
<?php echo '<script'; ?>
 type="text/javascript">
var jobrefresh = '<?php echo $_COOKIE['jobrefresh'];?>
';
var setval,
    setwout;
var yearreport_box = null;
var uid = '<?php echo $_smarty_tpl->tpl_vars['uid']->value;?>
';
var yrshow = '<?php echo $_smarty_tpl->tpl_vars['yrshow']->value;?>
';
layui.use(['layer'], function() {
    var layer = layui.layer,
        $ = layui.$;


    // layer.tips('左边么么哒', '.ritopimgs', {
    //   tips: [2, '#78BA32']
    // });


    $(document).on('mouseenter', '.laytips', function() {
        let data = $(this).data();
        layer_tips = layer.tips(data.title, '#' + data.id, {
            time: 0,
            tips: 2
        });
    }).on('mouseleave', '.laytips', function() {
        layer.close(layer_tips);
    });

    '<?php if ($_smarty_tpl->tpl_vars['yrshow']->value==1) {?>'
    yearreport_box = layer.open({
        type: 1,
        title: '',
        closeBtn: 0,
        area: ['360px', '365px'],
        content: $("#yearreport_tip")
    });

    '<?php } elseif ($_smarty_tpl->tpl_vars['company']->value['hy']!=''&&!$_smarty_tpl->tpl_vars['vipIsDown']->value&&$_smarty_tpl->tpl_vars['normal_job_num']->value!=0&&$_smarty_tpl->tpl_vars['un_refreshjob_num']->value!=0&&$_COOKIE['jobrefresh']!=1) {?>'
    var shuaxinlayer = layer.open({
        type: 1,
        title: '刷新职位',
        closeBtn: 0,
        border: [10, 0.3, '#000', true],
        area: ['500px', '330px'],
        content: $("#shuaxin")
    });
    $(".sx_bot_qx").click(function() {
        layer.close(shuaxinlayer);
    })
    '<?php } elseif ($_smarty_tpl->tpl_vars['vipIsDown']->value&&$_COOKIE['jobrefresh']==1) {?>'
    var updatelayer = layer.open({
        type: 1,
        title: '温馨提示',
        closeBtn: 0,
        border: [10, 0.3, '#000', true],
        area: ['430px', 'auto'],
        content: $("#updaterating")
    });
    $(".renew_bth_qx").click(function() {
        layer.close(updatelayer);
    })
    '<?php }?>'
});
$(document).ready(function() {
    var company_r_status = '<?php echo $_smarty_tpl->tpl_vars['company']->value['r_status'];?>
';
    var companyname = '<?php echo $_smarty_tpl->tpl_vars['company']->value['name'];?>
';
    var yyzz_status = '<?php echo $_smarty_tpl->tpl_vars['company']->value['yyzz_status'];?>
';
    var vipIsDown = '<?php echo $_smarty_tpl->tpl_vars['vipIsDown']->value;?>
';
    var statisremind = '<?php echo $_smarty_tpl->tpl_vars['statis']->value['remind'];?>
';
    if (company_r_status != 1 || !companyname || yyzz_status != 1 || vipIsDown || (statisremind == 1 && !vipIsDown)) {
        $(".yun_Announcement").attr("style", "display:block;");
    } else {
        $(".yun_Announcement").attr("style", "display:none;");
    }
    setTimeout(function() {
        hsresumelist('<?php echo $_smarty_tpl->tpl_vars['normal_job_num']->value;?>
');
    }, 200);
    var gzh = '<?php echo $_COOKIE['gzh'];?>
';
    var qrcode = '<?php echo $_smarty_tpl->tpl_vars['qrcode']->value;?>
';
    var popWin = '<?php echo $_smarty_tpl->tpl_vars['config']->value['wx_popWin'];?>
';

    if (!yrshow) {
        if (!gzh && qrcode == '1' && popWin == '1') {
            setTimeout(function() {
                gzhShow();
            }, 1000);
        }
    }
})

function getwxbindcode() {
    $.post('<?php echo smarty_function_url(array('m'=>'login','c'=>'wxlogin'),$_smarty_tpl);?>
', { t: 1 }, function(data) {
        if (data == 0) {
            $('#wx_login_qrcode').html('二维码获取失败..');
        } else {
            $('#wx_login_qrcode').html('<img src="' + data + '" width="180" height="180">');
            setval = setInterval(function() {
                $.post('<?php echo smarty_function_url(array('m'=>'login','c'=>'getwxloginstatus'),$_smarty_tpl);?>
', { t: 1 }, function(data) {
                    var data = eval('(' + data + ')');
                    if (data.url != '' && data.msg != '') {
                        clearInterval(setval);
                        setval = null;
                        layer.msg(data.msg, 2, 9, function() {
                            window.location.href = data.url;
                        });
                    } else if (data.url) {
                        window.location.href = data.url;
                    }
                });
            }, 2000);
            if (setwout) {
                clearTimeout(setwout);
                setwout = null;
            }
            setwout = setTimeout(function() {
                clearInterval(setval);
                setval = null;
                var wx_sx = $("#wx_sx").html();
                $('#wx_login_qrcode').html(wx_sx);
            }, 300 * 1000);
        }
    });
}

function wxshow() {
    $.layer({
        type: 1,
        title: '绑定微信',
        border: [10, 0.3, '#000', true],
        area: ['400px', 'auto'],
        page: { dom: "#wxcontent" },
        close: function() {
            if (setval) {
                clearInterval(setval);
                setval = null;
            }
            if (setwout) {
                clearTimeout(setwout);
                setwout = null;
            }
        }
    });
    getwxbindcode();
}

function kfwxShow(qrcode) {
    $("#kfwx_qrcode").attr('src', qrcode);

    var kfwxLayer = layer.open({
        type: 1,
        title: false,
        closeBtn: 0,
        border: [10, 0.3, '#000', true],
        area: ['400px', 'auto'],
        content: $("#kfwx"),
        end: function(){
            if (setval) {
                clearInterval(setval);
                setval = null;
            }
            if (setwout) {
                clearTimeout(setwout);
                setwout = null;
            }
        }
    });
    $("#kfwx_close").click(function () {
        layer.close(kfwxLayer);
    });
}

function reportgw(eid, title) {
    var layindex = $.layer({
        type: 1,
        title: title,
        closeBtn: [0, true],
        border: [10, 0.3, '#000', true],
        area: ['400px', '330px'],
        page: {
            dom: "#" + eid
        }
    });
    $("#layindex").val(layindex);
}

function reportSub(url) {
    var reason = $("#reason").val();
    var eid = $("#eid").val();
    if (reason == '') {
        layer.msg('请填写投诉内容！', 2, 8);
        return false;
    }
    loadlayer();
    $.post(url, {
        reason: reason,
        eid: eid
    }, function(data) {
        layer.closeAll('loading');
        layer.close($('#layindex').val());
        if (data == '0') {
            layer.msg('投诉失败！', 2, 8);
        } else if (data == '1') {
            layer.msg('投诉成功！', 2, 9, function() {
                window.location.reload();
            });
        } else if (data == '2') {
            layer.msg('已投诉成功，等待管理员回复！', 2, 8);
        }
    });
}

function hsresumelist(jobnum) {
    $.get('index.php?c=index&act=resumeajax', { rand: Math.random() }, function(data) {
        var res = JSON.parse(data);
        if (res.list.length > 0) {
            var html = '';
            for (var i = 0; i < res.list.length; i++) {
                html += '<li>\n' +
                    '    <div class="postdivTite">\n' +
                    '        <span onclick="com_lookresume_check(' + res.list[i].id + ',' + res.list[i].status + ')">' + res.list[i].jobname + '</span>\n' +
                    '        <b>' + res.list[i].sex_n + ' . ' + res.list[i].age_n + ' . ' + res.list[i].exp_n + ' . ' + res.list[i].edu_n + '</b>\n' +
                    '    </div>\n' +
                    '    <div class="postdivInfo">\n' +
                    '        <div class="postdivData">\n' +
                    '            <div class="postdivImg" onclick="com_lookresume_check(' + res.list[i].id + ',' + res.list[i].status + ')">\n' +
                    '                <img src="' + res.list[i].photo + '" alt="">\n' +
                    '            </div>\n' +
                    '            <div class="postdivName">\n' +
                    '                <span onclick="com_lookresume_check(' + res.list[i].id + ',' + res.list[i].status + ')">' + res.list[i].username_n + '</span>\n' +
                    // '<b>今日被看' + res.list[i].looknum + '次</b>\n' +
                    '            </div>\n' +
					
                    '        </div>\n' + 
					'        <div class="postdivChatrs">\n' +
					'            <span class="spanDown" onclick="com_lookresume_check(' + res.list[i].id + ',' + res.list[i].status + ')">查看简历</span>\n' +
					'        </div>\n' +
					'    </div>\n' +
                   
                   
                    '</li>';
            }
            $("#resumelist").prepend(html);
        } else {
            $("#noresumelist").removeClass('none');
        }
    })
}

function notweixin() {
    layer.msg('网站暂未绑定微信公众号，请联系管理员，电话：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel']!='
        ' ? $_smarty_tpl->tpl_vars['config']->value['sy_comwebtel'] : $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
', 2, 8);
}
'<?php if ($_smarty_tpl->tpl_vars['ggnum']->value>1) {?>'
marquee("6000", ".yun_Announcement  ul");
'<?php }?>'

function closeyr() {
    layer.close(yearreport_box);
}

function yearreport() {
    closeyr();
    var url = weburl + '/index.php?m=ajax&c=lastYearReport&uid=' + uid;

    var loading = layer.load('生成中...', 0);

    var image = new Image();
    image.src = url;
    image.onload = function() {
        layer.close(loading);
        layer.open({
            type: 1,
            title: false,
            content: '<div class="hb_tc"><img src="' + image.src + '" style="max-width: 100%;"><div class="hb_tc_bth"><a href="javascript:;" onclick="downYearReport();" class="hb_tc_xz">下载海报</a></div></div>',
            area: ['360px', 'auto'],
            offset: '55px',
            closeBtn: 0,
            shadeClose: true
        });
    };
}

function downYearReport() {

    var loading = layer.load('下载中...', 0);
    var url = weburl + '/index.php?m=ajax&c=lastYearReport&uid=' + uid;

    var image = new Image();
    image.src = url;
    image.onload = function() {
        layer.closeAll();
        var a = document.createElement('a'); // 创建一个a节点插入的document
        var event = new MouseEvent('click') // 模拟鼠标click点击事件
        a.download = '年度报告'; // 设置a节点的download属性值
        a.href = url; // 将图片的src赋值给a节点的href
        a.dispatchEvent(event);
    }
}
<?php echo '</script'; ?>
>

<!--底部需要引入，不引入部分功能异常-->
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
