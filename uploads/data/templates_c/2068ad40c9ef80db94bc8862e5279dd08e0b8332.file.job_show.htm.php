<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:57:24
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/job_show.htm" */ ?>
<?php /*%%SmartyHeaderCode:139442235469e89b841c3c25-70862823%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '2068ad40c9ef80db94bc8862e5279dd08e0b8332' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/job_show.htm',
      1 => 1706496290,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '139442235469e89b841c3c25-70862823',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'config' => 0,
    'backurl' => 0,
    'job' => 0,
    'wlist' => 0,
    'wlists' => 0,
    'department' => 0,
    'lunbo' => 0,
    'link' => 0,
    'msgList' => 0,
    'msglist' => 0,
    'uid' => 0,
    'usertype' => 0,
    'job_jia' => 0,
    'waflist' => 0,
    'shareurl' => 0,
    'factlist' => 0,
    'fact' => 0,
    'key' => 0,
    'isweixin' => 0,
    'hbNum' => 0,
    'title' => 0,
    'description' => 0,
    'sexData' => 0,
    'v' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89b841f7d83_03380913',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89b841f7d83_03380913')) {function content_69e89b841f7d83_03380913($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/swiper/swiper.min.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" />
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/swiper/swiper.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<!-- 页面头部导航栏 -->
<div class="jobshwheader">
    <div class="jobshwheader_nav">
        <div class="jobshwheader_nav_left">
            <a href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>" class="jobshwheader_nav_left_return">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" style="width: 100%;">
            </a>
            <span class="jobshwheader_p"> 职位详情</span>
        </div>
        <div class="jobshwheader_nav_right">
            <ul>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['fav_job']) {?>
                <li onclick="cancelFavJob('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/ysc_n.png" alt="" style="width: 100%;"></li>
                <?php } else { ?>
                <li onclick="toJobfav()"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_collect.png" alt="" style="width: 100%;"></li>
                <?php }?>
                <li onclick="toReportCom()"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_complaint.png" alt="" style="width: 100%;"></li>
            </ul>
        </div>
    </div>
</div>
<!-- 页面主体部分 -->
<div class="min_body">
    <!-- 职位详情 -->
    <div class="job_describe">
        <!-- 职位要求及待遇岗位 -->
        <div class="job_describe_top">
            <div class="new_jobshowtop">
                <?php if ($_smarty_tpl->tpl_vars['job']->value['status']==1) {?>
                <div class="job_yxj"> <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/stamp.png"></div>
                <?php }?>
                <div class="new_jobshowname">
                    <?php echo $_smarty_tpl->tpl_vars['job']->value['jobname'];?>

                    <?php if ($_smarty_tpl->tpl_vars['job']->value['job_rec']==1) {?><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_recommend.png" alt=""><?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['job']->value['job_urgent']==1) {?><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jp.png" alt=""><?php }?>
                </div>
                <span class="new_jobshowxz"><?php echo $_smarty_tpl->tpl_vars['job']->value['job_salary'];?>
</span>
            </div>
            <div class="job_describe_top_require">
                <div class="job_describe_top_require_left">
                    <i><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_orientation.png" alt="" style="width: 100%;"></i>
                    <i><?php echo $_smarty_tpl->tpl_vars['job']->value['job_city_one'];?>
-<?php echo $_smarty_tpl->tpl_vars['job']->value['job_city_two'];?>
</i>
                </div>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_edu']) {?>
                <div class="job_describe_top_require_center">
                    <div class="job_describe_top_require_left">
                        <i><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_fixed.png" alt="" style="width: 100%;"></i>
                        <i><?php echo $_smarty_tpl->tpl_vars['job']->value['job_edu'];?>
</i>
                    </div>
                </div>
                <?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_exp']) {?>
                <div class="job_describe_top_require_right">
                    <div class="job_describe_top_require_left">
                        <i><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_education.png" alt="" style="width: 100%;"></i>
                        <i><?php echo $_smarty_tpl->tpl_vars['job']->value['job_exp'];?>
</i>
                    </div>
                </div>
                <?php }?>
            </div>
            <div class="newjob_show_sj">
                <span class="">更新 <?php echo $_smarty_tpl->tpl_vars['job']->value['lastupdate'];?>
</span>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['jobhits']>0) {?><span class="">浏览 <?php echo $_smarty_tpl->tpl_vars['job']->value['jobhits'];?>
</span><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['snum']>$_smarty_tpl->tpl_vars['config']->value['sy_sq_job_num']) {?><span class="">投递 <?php echo $_smarty_tpl->tpl_vars['job']->value['snum'];?>
份</span><?php }?>
            </div>
        </div>
        <!--平台核验-->
        <?php if ($_smarty_tpl->tpl_vars['job']->value['fact_status']==1) {?>
        <div class="yunnew_hybox" onclick="factShow()">
            <i class="yunnew_hyboxicon"></i>
            <div class="yunnew_hyzzsmbox">
                <span class="yunnew_hyboxname">平台核验</span>
                <div class="yunnew_hyzzsm">
                    <?php if ($_smarty_tpl->tpl_vars['job']->value['yyzz_status']==1) {?><span class="yunnew_hymini"><i class="yunnew_minicon"></i>资质核验</span><?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['job']->value['moblie_status']==1) {?><span class="yunnew_hymini"><i class="yunnew_minicon"></i>实名核验</span><?php }?>
                    <span class="yunnew_hymini"><i class="yunnew_minicon"></i>实地核验</span>
                </div>
            </div>
            <i class="yunnew_hyboxricon"></i>
        </div>
        <?php }?>
        <!--平台核验-->
        <!-- 职位福利 -->
        <?php if (!empty($_smarty_tpl->tpl_vars['job']->value['welfare'])) {?>
        <div class="job_describe_bottom">
            <div class="job_describe_cengter_header">职位福利</div>
            <div class="job_describe_bottom_welfare">
                <ul>
                    <?php  $_smarty_tpl->tpl_vars['wlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['wlist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job']->value['arraywelfare']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['wlist']->key => $_smarty_tpl->tpl_vars['wlist']->value) {
$_smarty_tpl->tpl_vars['wlist']->_loop = true;
?>
                    <li><?php echo $_smarty_tpl->tpl_vars['wlist']->value;?>
</li>
                    <?php } ?>
                </ul>
            </div>
        </div>
        <?php }?>
        <!-- 职位详情 -->
        <div class="job_describe_cengter">
            <div class="job_describe_cengter_header">职位详情</div>
            <ul class="job_describe_yq">
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_number']) {?><li>招 <?php echo $_smarty_tpl->tpl_vars['job']->value['job_number'];?>
</li><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_report']) {?><li><?php echo $_smarty_tpl->tpl_vars['job']->value['job_report'];?>
到岗</li><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_age']) {?><li><?php echo $_smarty_tpl->tpl_vars['job']->value['job_age'];?>
</li><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_sex']&&$_smarty_tpl->tpl_vars['config']->value['com_job_sexswitch']==1) {?><li><?php echo $_smarty_tpl->tpl_vars['job']->value['job_sex'];?>
</li><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_marriage']) {?><li><?php echo $_smarty_tpl->tpl_vars['job']->value['job_marriage'];?>
</li><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['is_graduate']) {?><li>接受应届生</li><?php }?>
                <?php if ($_smarty_tpl->tpl_vars['job']->value['job_lang']) {?>
                <?php  $_smarty_tpl->tpl_vars['wlists'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['wlists']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job']->value['job_lang']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['wlists']->key => $_smarty_tpl->tpl_vars['wlists']->value) {
$_smarty_tpl->tpl_vars['wlists']->_loop = true;
?>
                <li><?php echo $_smarty_tpl->tpl_vars['wlists']->value;?>
</li>
                <?php } ?>
                <?php }?>
                <?php if ($_smarty_tpl->tpl_vars['department']->value) {?><li><?php echo $_smarty_tpl->tpl_vars['department']->value;?>
</li><?php }?>
            </ul>
            <div class="newjob_js"> <?php echo $_smarty_tpl->tpl_vars['job']->value['description'];?>
</div>
        </div>
    </div>
    <!--广告-->
    <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"512","item"=>"\"lunbo\"","key"=>"“key“","random"=>"1","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[512];if(is_array($add_arr) && !empty($add_arr)){
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
    <div class="jobshow_ad"><?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
</div>
    <?php } ?>
    <!--广告-->
    <!-- 公司信息 -->
    <div class="corporate_information">
        <div class="corporate_information_header">公司信息</div>
        <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'company','a'=>'show','id'=>$_smarty_tpl->tpl_vars['job']->value['uid']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['job']->value['name'];?>
">
            <div class="corporate_information_message">
                <div class="corporate_information_message_logo">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['job']->value['logo'];?>
" alt="" width="100%">
                </div>
                <div class="corporate_information_message_name">
                    <div><?php echo $_smarty_tpl->tpl_vars['job']->value['name'];?>
</div>
                    <div class="com_j_info">
                        <span><?php echo $_smarty_tpl->tpl_vars['job']->value['job_mun'];?>
</span>
                        <span>· <?php echo $_smarty_tpl->tpl_vars['job']->value['job_pr'];?>
 ·</span>
                        <span><?php echo $_smarty_tpl->tpl_vars['job']->value['job_hy'];?>
</span>
                    </div>
                </div>
                <div class="corporate_information_message_details">
                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_more.png" alt="" width="100%">
                </div>
            </div>
        </a>
        <?php if ($_smarty_tpl->tpl_vars['link']->value['linkData']['address']) {?>
        <?php if ($_smarty_tpl->tpl_vars['job']->value['x']&&$_smarty_tpl->tpl_vars['job']->value['y']&&$_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
        <!-- 预留地图部分 -->
        <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'jobmap','id'=>$_smarty_tpl->tpl_vars['job']->value['id']),$_smarty_tpl);?>
">
            <div class="corporate_information_map">
                <div class="corporate_information_map_c">
                    <span class="corporate_information_map_sz"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['address'];?>
</span>
                </div>
                <img src="<?php echo $_smarty_tpl->tpl_vars['job']->value['staticimg'];?>
" alt="" width="100%">
            </div>
        </a>
        <?php } else { ?>
        <!-- 地址 -->
        <div class=" corporate_information_map_p"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['address'];?>
</div>
        <?php }?>
        <?php }?>
        <div class="wxtipbox">
            <div class="wxtip">
                <div class="wxtip_tit"><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
温馨提示 </div>
            </div>
            <div class=""> <?php echo $_smarty_tpl->tpl_vars['config']->value['sy_shenming'];?>

                <span onclick="toReportCom()" class="wxtip_bth">立即举报</span>
            </div>
        </div>
    </div>
    <?php if ($_smarty_tpl->tpl_vars['config']->value['com_message']==1) {?>
    <!-- 公司问答 -->
    <div class="company_questions">
        <div class="company_questions_header">
            <div class="company_questions_header_left">公司问答</div>
            <div onclick="zixun()" class="company_questions_header_right">
                <div class="company_questions_header_right_icon"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_question.png" alt="" width="100%"></div>
                <i class="company_questions_header_right_word">我要提问</i>
            </div>
        </div>
        <?php if ($_smarty_tpl->tpl_vars['msgList']->value) {?>
        <?php  $_smarty_tpl->tpl_vars['msglist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['msglist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['msgList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['msglist']->key => $_smarty_tpl->tpl_vars['msglist']->value) {
$_smarty_tpl->tpl_vars['msglist']->_loop = true;
?>
        <div class="company_questions_body">
            <div class="company_questions_body_top">
                <div class="company_questions_body_top_icon"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_issue.png" alt="" width="100%"></div>
                <i class="company_questions_body_top_ask"><?php echo $_smarty_tpl->tpl_vars['msglist']->value['content'];?>
</i>
            </div>
            <div class="company_questions_body_top">
                <div class="company_questions_body_top_icon"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_answer .png" alt="" width="100%"></div>
                <i class="company_questions_body_top_answer"><?php if ($_smarty_tpl->tpl_vars['msglist']->value['reply']) {
echo $_smarty_tpl->tpl_vars['msglist']->value['reply'];
} else { ?>企业尚未回复<?php }?></i>
            </div>
        </div>
        <?php } ?>
        <!--
            <div class="company_questions_box">
                <div class="company_questions_box_btn"> 查看全部回答 </div>
            </div>
            -->
        <?php } else { ?>
        <div class="jobshow_tw_box">
            <i class="jobshow_tw_boximg"></i> 对此职位有疑问？快来问问吧 !
            <div class="jobshow_tw_bth">
                <?php if ($_smarty_tpl->tpl_vars['uid']->value) {?>
                <?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>
                <a href="javascript:void(0)" onclick="zixun('<?php echo $_smarty_tpl->tpl_vars['uid']->value;?>
','<?php echo $_smarty_tpl->tpl_vars['usertype']->value;?>
');" class="">我要提问</a>
                <?php } else { ?>
                <a href="javascript:void(0)" onclick="showToast('只有个人用户才能提问');" class="">我要提问</a>
                <?php }?>
                <?php } else { ?>
                <a href="javascript:void(0)" onclick="pleaselogin('您还未登录个人账号，是否登录？','<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
')" class="">我要提问</a>
                <?php }?>
            </div>
        </div>
        <?php }?>
    </div>
    <?php }?>
    <!-- 推荐岗位 -->
    <?php if (!$_smarty_tpl->tpl_vars['usertype']->value||$_smarty_tpl->tpl_vars['usertype']->value=='1') {?>
    <div class="recommend_post" style="margin-top: 0px;;">
        <div class="recommend_post_header" style="margin:0.4rem 0;">相似职位</div>
        <div class="recommend_post_card_box">
            <!-- 卡片视图主体部分 -->
            <?php  $_smarty_tpl->tpl_vars['job_jia'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['job_jia']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("limit"=>"4","noid"=>"“@job.id“","nouid"=>"“@job.uid“","jobids"=>"“@job.job1“","namelen"=>"15","item"=>"“job_jia“","key"=>"“key“","nocache"=>"")
;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
        $Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		include_once  PLUS_PATH."/comrating.cache.php";
		include(CONFIG_PATH."db.data.php"); 
        $cache_array = $db->cacheget();
        $comclass_name  = $cache_array["comclass_name"];
        $comdata        = $cache_array["comdata"];
        $city_name      = $cache_array["city_name"];
        $job_name       = $cache_array["job_name"];
		$industry_name	= $cache_array["industry_name"];

		if($config[sy_web_site]=="1"){
			if($config[province]>0 && $config[province]!=""){
				$paramer[provinceid] = $config[province];
			}
			if($config[cityid]>0 && $config[cityid]!=""){
				$paramer[cityid] = $config[cityid];
			}
			if($config[three_cityid]>0 && $config[three_cityid]!=""){
				$paramer[three_cityid] = $config[three_cityid];
			}
			if($config[hyclass]>0 && $config[hyclass]!=""){
				$paramer[hy]=$config[hyclass];
			}
		}

		
		if($paramer[sdate]){
			$where = "`sdate`>".strtotime("-".intval($paramer[sdate])." day",time())." and `state`=1";
		}else{
			$where = "`state`=1";
		}
		
		//按照UID来查询（按公司地址查询可用GET[id]获取当前公司ID）
		if($paramer[com_id]){
			$where .= " AND `uid` = '$paramer[com_id]'";
			// 单查某企业下职位，排除城市、行业类别（排除分站）参数
            if(isset($paramer[provinceid])){unset($paramer[provinceid]);}
            if(isset($paramer[cityid])){unset($paramer[cityid]);}
            if(isset($paramer[three_cityid])){unset($paramer[three_cityid]);}
            if(isset($paramer[hy])){unset($paramer[hy]);}
		}
		
		if (!empty($paramer[depower])) {
		    trim($paramer[depower]) != 'all' && $where .= " AND `is_depower` = $paramer[depower]"; // all为降权和非降权均查询
		} else {
		    $where .= " AND `is_depower` = 2"; // 默认查询未降权的职位
		}

		//是否推荐职位
		if($paramer[rec]){
			
			$where.=" AND `rec_time`>=".time();
			
		}
		//企业认证条件
		if($paramer['cert']){
			
			$where.=" and `yyzz_status`=1";
		}
		//取不包含当前企业的职位
		if($paramer[nouid]){
			$where.= " and `uid`<>$paramer[nouid]";
		}
		//取不包含当前id的职位
		if($paramer[noid]){
			$where.= " and `id`<>$paramer[noid]";
		}
		//是否被锁定
		if($paramer[r_status]){
			$where.= " and `r_status`=2";
		}else{
			$where.= " and `r_status`=1";
		}
		//是否下架职位
		if($paramer[status]){
			$where.= " and `status`='1'";
		}else{
			$where.= " and `status`='0'";
		}
		//公司体制
		if($paramer[pr]){
			$where .= " AND `pr` =$paramer[pr]";
		}
		//公司行业分类
		if($paramer['hy']){
			$where .= " AND `hy` = $paramer[hy]";
		} 
		//职位大类
		if($paramer[job1]){
			$where .= " AND `job1` = $paramer[job1]";
		}
		//职位子类
		if($paramer[job1_son]){
			$where .= " AND `job1_son` = $paramer[job1_son]";
		}
		if($paramer[job1son]){
			$where .= " AND `job1_son` = $paramer[job1son]";
		}
		//职位三级分类
		if($paramer[job_post]){
			$where .= " AND (`job_post` IN ($paramer[job_post]))";
		}
		if($paramer[jobpost]){
			$where .= " AND (`job_post` IN ($paramer[jobpost]))";
		}
		//您可能感兴趣的职位--个人会员中心
		if($paramer['jobwhere']){
			$where .=" and ".$paramer['jobwhere'];
		}
		//职位分类综合查询
		if($paramer['jobids']){
			$where.= " AND (`job1` = '$paramer[jobids]' OR `job1_son`= '$paramer[jobids]' OR `job_post`='$paramer[jobids]')";
		}
		//职位分类区间,不建议执行该查询
		if($paramer['jobin']){
			$where .= " AND (`job1` IN ($paramer[jobin]) OR `job1_son` IN ($paramer[jobin]) OR `job_post` IN ($paramer[jobin]))";
		}
		//多选职位
		if($paramer["job"]){
			$where.=" AND `job_post` in ($paramer[job])";
		}
		//城市大类
		if($paramer[provinceid]){
			$where .= " AND `provinceid` = $paramer[provinceid]";
		}
		//城市子类
		if($paramer['cityid']){
			$where .= " AND (`cityid` IN ($paramer[cityid]))";
		}
		//城市三级子类
		if($paramer['three_cityid']){
			$where .= " AND (`three_cityid` IN ($paramer[three_cityid]))";
		}
		if($paramer['threecityid']){
			$where .= " AND (`three_cityid` IN ($paramer[threecityid]))";
		}
		if($paramer['cityin']){
			$where .= " AND `three_cityid` IN ($paramer[cityin])";
		}
		//学历
		if($paramer[edu]){
            $eduArr  = $comdata['job_edu'];
			$eduSort = 0;
			$eduIds  = array();
			// 职位搜索，排序比搜索小的都符合条件。如搜“硕士”，类别排序小于等于“硕士”排序的（要排除不限）都符合
			foreach ($eduArr as $k => $v) {
			    if ($v == $paramer[edu] && $comclass_name[$v] != "不限"){
			        $eduSort = $k;
                    break;
			    }
			}
			foreach ($eduArr as $k => $v) {
			    if ($k <= $eduSort && $comclass_name[$v] != "不限"){
			        $eduIds[] = $v;
			    }
			}
            if (!empty($eduIds)) {
            	$where .= " AND `edu` in (".@implode(",",$eduIds).")";
            }
		}
		//工作经验
		if($paramer[exp]){
            $expArr  = $comdata['job_exp'];
			$expSort = 0;
			$expIds  = array();
			// 职位搜索，排序比搜索小的都符合条件。如搜“五年”，类别排序小于等于“五年”排序的（要排除不限）都符合
            foreach ($expArr as $k => $v) {
                if ($v == $paramer[exp] && $comclass_name[$v] != "不限"){
                    $expSort = $k;
                    break;
                }
            }
            foreach ($expArr as $k => $v) {
                if ($k <= $expSort && $comclass_name[$v] != "不限"){
                    $expIds[] = $v;
                }
            }
            if (!empty($expIds)) {
            	$where .= " AND `exp` in (".@implode(",",$expIds).")";
            }
		}
		//到岗时间
		if($paramer[report]){
			$where .= " AND `report` = $paramer[report]";
		}
		//职位性质
		if($paramer[type]){
			$where .= " AND `type` = $paramer[type]";
		}
		//性别
		if($paramer[sex]){
			$where .= " AND `sex` = $paramer[sex]";
		}
		//应届生
		if($paramer[is_graduate]){
			$where .= " AND `is_graduate` = $paramer[is_graduate]";
		}
		//公司规模
		if($paramer[mun]){
			$where .= " AND `mun` = $paramer[mun]";
		}
		 
		if($paramer[minsalary] && $paramer[maxsalary]){
			$where.= " AND (`minsalary`>=".intval($paramer[minsalary])." and `minsalary`<=".intval($paramer[maxsalary])." and `maxsalary`<=".intval($paramer[maxsalary]).") ";

		}elseif($paramer[minsalary]&&!$paramer[maxsalary]){
			$where.= " AND (`minsalary`>=".intval($paramer[minsalary]).") ";

		}elseif(!$paramer[minsalary]&&$paramer[maxsalary]){
			$where.= " AND (`minsalary`<=".intval($paramer[maxsalary])." and `maxsalary`<=".intval($paramer[maxsalary]).") ";
		}
	    //福利待遇
		if($paramer[welfare]){
			$welfarename = $comclass_name[$paramer[welfare]];
            $where .=" AND `welfare` LIKE '%".$welfarename."%' ";
		}
		
		//城市区间,不建议执行该查询
		if($paramer[cityin]){
			$where .= " AND (`provinceid` IN ($paramer[cityin]) OR `cityid` IN ($paramer[cityin]) OR `three_cityid` IN ($paramer[cityin]))";
		}
		//紧急招聘urgent
		if($paramer[urgent]){
			$where.=" AND `urgent_time`>".time();
		}
		//更新时间区间
		if($paramer[uptime]){
			if($paramer[uptime]==1){
				$beginToday = strtotime('today');
				$where.=" AND lastupdate>$beginToday";
			}else{
				$time=time();
				$uptime = $time-($paramer[uptime]*86400);
				$where.=" AND lastupdate>$uptime";
			}
		}else{
		    if($config[sy_datacycle_job]>0){	
                // 后台-页面设置-数据周期	        
				$uptime = strtotime('-'.$config[sy_datacycle_job].' day');
				$where.=" AND lastupdate>$uptime";
		    }
		}		
		//按类似公司名称,不建议进行大数据量操作
		if($paramer[comname]){
			$where.=" AND `com_name` LIKE '%".$paramer[comname]."%'";
		}
		//按公司归属地,只适合查询一级城市分类
		if($paramer[com_pro]){
			$where.=" AND `com_provinceid` ='".$paramer[com_pro]."'";
		}
		// 关键字匹配
		if($paramer[keyword]){
		    $comuids    =   $db->select_all("company","`name` LIKE '%".$paramer['keyword']."%' OR `shortname` LIKE '%".$paramer['keyword']."%'","`uid`");
		    $cuidArr    =   array();
		    foreach($comuids as $v){
				$cuidArr[]=$v['uid'];
			}
            $where1     =   array();
			$where1[]   =   "`name` LIKE '%".$paramer[keyword]."%'";
			if($config['job_full_text_search'] == 1){
			    $where1[]   =   "`description` LIKE '%".$paramer[keyword]."%'";
			}
			if ($cuidArr) {
			    $where1[]   =   "`uid` in (".@implode(",",$cuidArr).")";
			}
            $cityid     =   array();
			foreach($city_name as $k=>$v){
				if(strpos($v,$paramer[keyword])!==false){
					$cityid[]=$k;
				}
			}
			if(!empty($cityid)){
                $class = array();
				foreach($cityid as $value){
					$class[]= "(provinceid = '".$value."' or cityid = '".$value."' or three_cityid = '".$value."')";
				}
				$where1[]=@implode(" or ",$class);
			}
			if($config['job_full_text_search'] == 1){
                $jobClassId =   array();
                foreach($job_name as $k=>$v){
                    if(strpos($v,$paramer[keyword])!==false){
                        $jobClassId[]=$k;
                    }
                }
                if(!empty($jobClassId)){
                    $class = array();
                    foreach($jobClassId as $value){
                    
                        $class[]= "(job1_son = '".$value."' or job_post = '".$value."')";
                    }
                    $where1[]=@implode(" or ",$class);
                }
			}
			$where.=" AND (".@implode(" or ",$where1).")";
		}

		//置顶招聘
		if($paramer[bid]){
		    $isZhiding = true;
			if($config[joblist_top]==0){
				//随机20条
				$paramer[limit] = 20;
			}elseif($config[joblist_top]==2){
			    //搜索置顶（职位分类|关键字）
			    $isZhiding = ($paramer[job1] || $paramer[job1_son] || $paramer[job1son] || $paramer[job_post] || $paramer[jobpost] || $paramer['jobwhere'] || $paramer['jobids'] || $paramer['jobin'] || $paramer["job"] || $paramer[keyword]) ? true : false;
			}
			
			if($isZhiding){
			    $where.="  and `xsdate`>'".time()."'";			
			}else{
			    $where.=" AND false";
			}
		} 
		//首页置顶
        if($paramer[istop]){
            $isIndexZhiding = true;
            if($config[joblist_top_index]==2){
                $paramer[limit] = 5;
            }elseif($config[joblist_top_index]==0){
                $isIndexZhiding = false;
            }
            if($isIndexZhiding){
			    $where.="  and `xsdate`>'".time()."'";			
			}else{
			    $where.=" AND false";
			}
        }
		//自定义查询条件，默认取代上面任何参数直接使用该语句
		if($paramer[where]){
			$where = $paramer[where];
		}

		//查询条数
		$limit = '';
		if($paramer[limit]){

			$limit = " limit ".$paramer[limit];
		}
		if($paramer[ispage]){
			$limit = PageNav($paramer,$_GET,"company_job",$where,$Purl,"",$paramer[islt]?$paramer[islt]:"6",$_smarty_tpl);        
		}

		//排序字段默认为更新时间
		//置顶设置为随机20条时，随机查询
		if($paramer[bid] && $config[joblist_top]==0){
			$order = " ORDER BY rand() ";
		}elseif($paramer[istop] && $config[joblist_top_index]==2){
		    $order = " ORDER BY rand() ";
		}else{
			if($paramer[order] && $paramer[order]!="lastdate"){
				$order = " ORDER BY ".str_replace("'","",$paramer[order])."  ";
			}else{
				$order = " ORDER BY `lastupdate` ";
			}
		}
		//排序规则 默认为倒序
		if($paramer[sort]){
			$sort = $paramer[sort];
		}else{
			$sort = " DESC";
		} 
		$where.=$order.$sort;
		
		$job_jia = $db->select_all("company_job",$where.$limit);

		if(is_array($job_jia) && !empty($job_jia)){
			$comuid=$jobid=array();
			foreach($job_jia as $key=>$value){
				if(in_array($value['uid'],$comuid)==false){$comuid[] = $value['uid'];}
				if(in_array($value['id'],$jobid)==false){$jobid[] = $value['id'];} 
			}
			$comuids = @implode(',',$comuid);
			$jobids = @implode(',',$jobid);
			//减少曝光量统计维度 只有列表才统计
			if($paramer[ispage]){
				$db->update_all("company_job", "`jobexpoure` = `jobexpoure` + 1", "`id` in ($jobids)");
			}
			

			if($comuids){
				$r_uids=$db->select_all("company","`uid` IN (".$comuids.")","`uid`,`hy`,`shortname`,`welfare`,`hotstart`,`hottime`,`fact_status`");
				if(is_array($r_uids)){
					foreach($r_uids as $key=>$value){
						if($value[shortname]){
    						$value['shortname_n'] = $value[shortname];
    					}
						if($value['hotstart']<=time() && $value['hottime']>=time()){
							$value['hotlogo'] = 1;
						}
                        $value['hy_n'] = $industry_name[$value[hy]];
						$r_uid[$value['uid']] = $value;
					}
				}
			}
			
 			if($paramer[bid]){
				$noids=array();
			}	
			if ($_COOKIE['uid'] && $_COOKIE['usertype']==1){
			    $lookJob =   $db->select_all("look_job","`uid` = ".$_COOKIE['uid'], "`jobid`");
			    if (!empty($lookJob)){
			        foreach($lookJob as $key=>$value){
						$lookJobIdArr[] = $value['jobid'];
					}
			    }
			}
			foreach($job_jia as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($job_jia[$key]);
					continue;
				}else{
					$job_jia[$key] = $db->array_action($value,$cache_array);
					$job_jia[$key][stime] = date("Y-m-d",$value[sdate]);
					$job_jia[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$job_jia[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$job_jia[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$job_jia[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$job_jia[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $job_jia[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $job_jia[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $job_jia[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $job_jia[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $job_jia[$key][job_number] = "";
					}else{
					    $job_jia[$key][job_number] = $value[zp_num]." 人";
					}			
                    $job_jia[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $job_jia[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $job_jia[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$job_jia[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$job_jia[$key][pr_n] = $comclass_name[$value[pr]];
					$job_jia[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$job_jia[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$job_jia[$key]['time'] = $job_jia[$key]['lastupdate'];
						$job_jia[$key]['redtime'] =1;
					}else{
						$job_jia[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$job_jia[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$job_jia[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$job_jia[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$job_jia[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$job_jia[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$job_jia[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$job_jia[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$job_jia[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$job_jia[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$job_jia[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$job_jia[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$job_jia[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$job_jia[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$job_jia[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$job_jia[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$job_jia[$key][name_n]);
						$job_jia[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$job_jia[$key][com_n]);
						$job_jia[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$job_jia[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $job_jia[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $job_jia[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($job_jia)){
				if($paramer[keyword]!=""&&!empty($job_jia)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$job_jia = $job_jia; if (!is_array($job_jia) && !is_object($job_jia)) { settype($job_jia, 'array');}
foreach ($job_jia as $_smarty_tpl->tpl_vars['job_jia']->key => $_smarty_tpl->tpl_vars['job_jia']->value) {
$_smarty_tpl->tpl_vars['job_jia']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['job_jia']->key;
?>
            <div class="recommend_post_card">
                <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['job_jia']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_jia']->value['name'];?>
">
                    <div class="recommend_post_card_top">
                        <div class="recommend_post_card_name"><?php echo $_smarty_tpl->tpl_vars['job_jia']->value['name_n'];?>
</div>
                        <div class="recommend_post_card_money"><?php echo $_smarty_tpl->tpl_vars['job_jia']->value['job_salary'];?>
</div>
                    </div>
                    <div class="newjob_info"><span><?php echo $_smarty_tpl->tpl_vars['job_jia']->value['job_city_one'];?>
-<?php echo $_smarty_tpl->tpl_vars['job_jia']->value['job_city_two'];?>
</span>
                        <?php if ($_smarty_tpl->tpl_vars['job_jia']->value['job_edu']) {?>
                        <i class="newjob_info_line"></i><span><?php echo $_smarty_tpl->tpl_vars['job_jia']->value['job_edu'];?>
学历</span>
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['job_jia']->value['job_exp']) {?>
                        <i class="newjob_info_line"></i> <span><?php echo $_smarty_tpl->tpl_vars['job_jia']->value['job_exp'];?>
经验</span>
                        <?php }?>
                    </div>
                    <?php if ($_smarty_tpl->tpl_vars['job_jia']->value['welfarename']) {?>
                    <div class="welfare">
                        <?php  $_smarty_tpl->tpl_vars['waflist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['waflist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job_jia']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['waflist']->key => $_smarty_tpl->tpl_vars['waflist']->value) {
$_smarty_tpl->tpl_vars['waflist']->_loop = true;
?>
                        <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['waflist']->value;?>
</span>
                        <?php } ?>
                    </div>
                    <?php }?>
                    <div class="recommend_post_card_bottom">
                        <div class="recommend_post_card_bottom_left">
                            <div class="recommend_post_card_bottom_left_logo"><img src="<?php echo $_smarty_tpl->tpl_vars['job_jia']->value['logo'];?>
" alt="" width="100%"></div>
                            <i class="recommend_post_card_bottom_left_word"><?php echo $_smarty_tpl->tpl_vars['job_jia']->value['com_name'];?>
</i>
                        </div>
                        <div class="recommend_post_card_bottom_right">
                            <?php if ($_smarty_tpl->tpl_vars['job_jia']->value['time']=='今天'||$_smarty_tpl->tpl_vars['job_jia']->value['time']=='昨天'||$_smarty_tpl->tpl_vars['job_jia']->value['redtime']=='1') {?>
                            <?php echo $_smarty_tpl->tpl_vars['job_jia']->value['time'];?>

                            <?php } else { ?>
                            <?php echo $_smarty_tpl->tpl_vars['job_jia']->value['time'];?>

                            <?php }?>
                        </div>
                    </div>
                </a>
            </div>
            <?php }
if (!$_smarty_tpl->tpl_vars['job_jia']->_loop) {
?>
            <div class="company_questions">
                <div class="wap_member_no">很抱歉,暂无相似职位！</div>
            </div>
            <?php } ?>
        </div>
    </div>
    <?php }?>
</div>
<div id="app" style="display: none;">
    <!-- 固定的手机尾部 new---------------------->
    <?php if ($_smarty_tpl->tpl_vars['job']->value['status']=='0') {?>
    <div class="yun_czfoot">
        <div class="yun_czfootfixed">
            <div class="yun_czfoot_c">
                <div class="yun_czfoot_l">
                    <div class="yun_czfoot_s">
                        <div class="yun_czfoot_s_p yun_czfoot_hmicon" @click="goHome">首页</div>
                    </div>
                    <div class="yun_czfoot_s">
                        <div class="yun_czfoot_s_p yun_czfoot_scicon" @click="footShare">分享</div>
                    </div>
                    <?php if ($_smarty_tpl->tpl_vars['usertype']->value!=1&&$_smarty_tpl->tpl_vars['uid']->value!='') {?>
                    <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_user_change']==1) {?>
                    <div class="yun_czfoot_s">
                        <div onclick="showToast('请先申请个人账户');" class="yun_czfoot_s_p yun_czfoot_jlicon">投简历</div>
                    </div>
                    <?php } else { ?>
                    <div class="yun_czfoot_s">
                        <div onclick="showToast('只有个人用户才能投递');" class="yun_czfoot_s_p yun_czfoot_jlicon">投简历</div>
                    </div>
                    <?php }?>
                    <?php } elseif ($_smarty_tpl->tpl_vars['job']->value['userid_job']) {?>
                    <div class="yun_czfoot_s">
                        <div class="yun_czfoot_s_p yun_czfoot_ytdicon">已投递</div>
                    </div>
                    <?php } elseif ($_smarty_tpl->tpl_vars['job']->value['invite_job']) {?>
                    <div class="yun_czfoot_s">
                        <div class="yun_czfoot_s_p yun_czfoot_ytdicon">已邀请</div>
                    </div>
                    <?php } elseif ($_smarty_tpl->tpl_vars['uid']->value=='') {?>
                    <?php if ($_smarty_tpl->tpl_vars['config']->value['resume_kstd']!=2) {?>
                    <div class="yun_czfoot_s">
                        <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'applyjobuid','jobid'=>$_smarty_tpl->tpl_vars['job']->value['id']),$_smarty_tpl);?>
" class="yun_czfoot_s_p yun_czfoot_jlicon">投简历</a>
                    </div>
                    <?php } else { ?>
                    <div class="yun_czfoot_s">
                        <a onclick="loginboxOpen()" class="yun_czfoot_s_p yun_czfoot_jlicon">投简历</a>
                    </div>
                    <?php }?>
                    <?php } else { ?>
                    <div class="yun_czfoot_s">
                        <div @click="jobapply()" class="yun_czfoot_s_p yun_czfoot_jlicon">投简历</div>
                    </div>
                    <?php }?>
                </div>
                <div class="yun_czfoot_r">
                    <?php if ($_smarty_tpl->tpl_vars['job']->value['linkopen']>1) {?>
                    <div class="yun_czfoot_lt yun_czfoot_lt_td" @click="rgetTel('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
')">拨打电话</div>
                    <?php } elseif ($_smarty_tpl->tpl_vars['link']->value['linkCode']==8) {?>
                    <div class="yun_czfoot_lt yun_czfoot_lt_td" @click="jobapply(1)">拨打电话</div>
                    <?php } else { ?>
                    <div class="yun_czfoot_lt yun_czfoot_lt_td" @click="openLinkTip">拨打电话</div>
                    <?php }?>
                </div>
            </div>
        </div>
    </div>
    <?php }?>
    <!-- 固定的手机尾部  new---------------------->
    <!--以下是查看联系方式弹出框----------------------------------------------------------------------------------->
    <van-dialog v-model="linkShow" :show-confirm-button="false" width="300" close-on-click-overlay>
        <template v-if="linkCode == 1">
            <div class="new_jobshow_telbox">
                <div class="new_jobshow_leftname"><span class="">联系方式</span></div>
                <div class="">
                    <?php if ($_smarty_tpl->tpl_vars['link']->value['linkData']['linkman']) {?>
                    <div class=""><span class=""><?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['linkman'];?>
</span> <?php if ($_smarty_tpl->tpl_vars['job']->value['linkjob']) {?>（<?php echo $_smarty_tpl->tpl_vars['job']->value['linkjob'];?>
）<?php }?></div>
                    <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['link']->value['linkData']['linktel']) {?>
                    <div class="new_jobshow_tellist new_jobshow_tellistpd"><a href="tel:<?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['linktel'];?>
" onclick="addtellog();" class="">手机 <?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['linktel'];?>
<span class="bd">拨打</span></a></div>
                    <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['link']->value['linkData']['linkphone']) {?>
                    <div class="new_jobshow_tellist"><a href="tel:<?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['linkphone'];?>
" onclick="addtellog();" class="new_jobshow_tellist_n">固话 <?php echo $_smarty_tpl->tpl_vars['link']->value['linkData']['linkphone'];?>
<span class="bd">拨打</span></a></div>
                    <?php }?>
                    <div class="new_jobshow_teltip">联系我时，请说是在<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
上看到的</div>
                </div>
            </div>
        </template>
        <template v-else-if="linkCode > 1 && linkCode < 6">
            <!--未开放联系方式，提示投递简历-->
            <div class="new_jobshow_name">温馨提示</div>
            <div class="new_jobshow_telnewbox">
                <div class="new_jobshow_tel"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkMsg'];
if (!$_smarty_tpl->tpl_vars['job']->value['userid_job']) {?>，请直接投递简历<?php }?></div>
                <?php if (!$_smarty_tpl->tpl_vars['job']->value['userid_job']) {?>
                <div class="new_jobshow_telbth" @click="jobapply()"><span class="new_jobshow_telbth_a">投递简历</span></div>
                <?php }?>
            </div>
        </template>
        <template v-else-if="linkCode == 6">
            <div class="newOobshoName">
                <div class="new_jobshow_name">温馨提示</div>
                <div class="new_jobshow_tel"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkMsg'];?>
，查看联系方式</div>
                <div class="new_jobshow_telbth">
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'register'),$_smarty_tpl);?>
" v-if="!uid">立即注册</a>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
" class="newOobshow" v-if="!uid">立即登录</a>
                </div>
            </div>
        </template>
        <template v-else-if="linkCode == 7">
            <div class="new_jobshow_name">温馨提示</div>
            <div class="new_jobshow_tel"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkMsg'];?>
</div>
            <div class="new_jobshow_telbth" v-if="linkSub != 1"><a href="<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
member/index.php?c=addresume" class="new_jobshow_telbth_a">创建简历</a></div>
            <div class="new_jobshow_telbth" v-else><a href="<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
member/index.php?c=resume" class="new_jobshow_telbth_a">修改简历</a></div>
        </template>
        <template v-else-if="linkCode == 8">
            <!--设置投递简历查看联系方式-->
            <div class="new_jobshow_name">温馨提示</div>
            <div class="new_jobshow_tel"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkMsg'];?>
</div>
            <div class="new_jobshow_telbth"><a href="javascript:void(0);" @click="jobapply()" class="new_jobshow_telbth_a">投递简历</a></div>
        </template>
    </van-dialog>
    <input id="companyname" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['com_name'];?>
" />
    <input id="jobname" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['jobname'];?>
" />
    <input id="companyuid" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['uid'];?>
" />
    <input id="jobid" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
" />
    <!-- 投递简历弹出框 -->
    <van-dialog v-model="telQrcodeBoxShow" title="联系方式" :show-confirm-button="false" close-on-click-overlay>
        <div class="tel_wxqrcodebox">
            <div id="tel_wxqrcode" class="tel_wxqrcodebox_ewm" v-html="telQrcode"></div>
            <div class="tel_wxqrcodebox_p">长按识别二维码查看联系方式</div>
        </div>
    </van-dialog>
    <!-- 举报弹出 -->
    <van-popup v-model="reportShow" position="bottom" round closeable>
        <div class="job_tckpd">
            <div class="job_tcktit">举报此职位</div>
            <div class="job_tcktip">请选择您的举报理由！</div>
            <div class="">
                <span v-for="(reason,index) in report_reasons" :key="index">
                    <span class="job_tckxz" :class="report_result.indexOf(reason) > -1 ? 'job_tckxz_cur' : ''" @click="chooseReason(reason)">{{reason}}</span>
                </span>
            </div>
            <div class="job_tcktextarea">
                <textarea placeholder="请简明扼要的阐述你的理由，以便工作人员更好的判断" id="r_reason" class=""></textarea>
            </div>
            <div class="job_tckyzmbox">
                <div class="job_tckyzm">
                    <input type="text" class="" id="authcode" maxlength="6" placeholder="输入图片验证码" autocomplete="off">
                </div>
                <a onclick="checkCode('vcode_img');"> <img id="vcode_img" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php" class="job_tckyzmimg" /> </a>
            </div>
            <div class="job_tckbth">
                <input class="" type="button" value="举 报" @click="reportSub" />
            </div>
        </div>
    </van-popup>
    <!--分享面板-->
    <van-share-sheet v-model="shareShow" title="分享到" :options="shareoptions" @select="shareSelect"></van-share-sheet>
    <!--海报-->
    <van-popup v-model="jobHbShow" round :style="{height:'100%',width:'100%',background:'none'}">
        <div class="boisuyeAll">
            <img id="bighb" :src="hbSrc" style="max-width: 100%;">
        </div>
        <div class="posterWapAll">
            <div class="goBackDo" @click="jobHbShow = false">
                <van-icon name="arrow-left" />
            </div>
            <template v-if="hbShow">
                <van-swipe :loop="false" :width="150">
                    <template v-for="(item, hbKey) in hbList">
                        <van-swipe-item @click="getNewJobHb(item.id)">
                            <div class="swipeItems">
                                <div class="swipeItemImg" :class="hb == item.id ? 'swipeItemImgColor' : ''">
                                    <img :src="item.pic_n" style="max-width: 100%;">
                                </div>
                                <div class="swipeItemText" v-if="hb == item.id">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/come-through.png" alt="">
                                </div>
                            </div>
                        </van-swipe-item>
                    </template>
                </van-swipe>
            </template>
            <div class="directionOptyte">
                <p>点击海报查看大图，长按保存后进行分享</p>
                <div class="directionAll" @click="hbShow = !hbShow">
                    <div class="directiTou" v-if="hbShow">
                        <van-icon name="arrow-up" />
                    </div>
                    <div class="directiTou" v-else>
                        <van-icon name="arrow-down" />
                    </div>
                </div>
            </div>
        </div>
    </van-popup>
    <!--复制文本弹出-->
    <van-dialog v-model="copyBoxShow" :show-confirm-button="false" close-on-click-overlay>
        <div class="job_tckpd">
            <div class="job_tcktit">复制文本</div>
            <div class="">
                <div class="job_tckwb" id="copyBoxText" v-html="wxpubtemp_html" style="white-space: pre-wrap;"></div>
                <div class="job_tckbth"><a href="javascript:;" class="fzwb" data-clipboard-action="copy" data-clipboard-target="#copyBoxText">复制文本</a></div>
            </div>
        </div>
    </van-dialog>
    <div class="copyUrl none" data-clipboard-text="<?php echo $_smarty_tpl->tpl_vars['shareurl']->value;?>
" data-clipboard-action="copy"></div>
    <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_h5_share']==1) {?>
    <div class="none" data-url='<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'share','id'=>$_smarty_tpl->tpl_vars['job']->value['id']),$_smarty_tpl);?>
' id="shareClick"></div>
    <?php } else { ?>
    <div class="none" id="shareClick"></div>
    <?php }?>
    <?php if ($_smarty_tpl->tpl_vars['config']->value['com_message']==1) {?>
    <!--问企业弹出-->
    <van-popup v-model="zixunShow" position="bottom" round closeable>
        <div class="job_tckpd">
            <div class="job_tcktit">问企业</div>
            <div class="job_tcktip">您的问题会在企业回答后展示在职位详情页！</div>
            <div class="job_tcktextarea">
                <textarea class=" mt10" id="reasons" placeholder="请输入你要咨询的问题">{{question}}</textarea>
                <input type="hidden" name="jobid" value="<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
" />
            </div>
            <div class="job_tckyzmbox">
                <div class="job_tckyzm">
                    <input type="text" class="" placeholder="请输入验证码" id="authcodes" maxlength="6">
                </div>
                <a onclick="checkCode('vcode_imgs');"> <img id="vcode_imgs" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php" class="job_tckyzmimg" /> </a>
            </div>
            <div class="ask_between">
                <div class="ask_between_l" v-if="hot_issues.length > 0">热门问题</div>
                <div class="ask_between_l" v-else></div>
                <div class="ask_between_r">默认为匿名提问</div>
            </div>
            <div class="ask_question" v-for="(item, asq_key) in hot_issues" :key="asq_key" :id="'asq_' + asq_key" @click="checkQuestion(asq_key)">{{asq_key+1}}.{{item}}</div>
            <div class="job_tckbth">
                <input class="" type="button" value="提交咨询" onclick="zixunSubs();" />
            </div>
        </div>
    </van-popup>
    <?php }?>
    <!-- 登录弹窗 -->
    <van-popup v-model="loginbox" position="center" round :style="{ width:'300px',background:'#fff'}">
        <div class="newOobshoName">
            <div class="new_jobshow_name">温馨提示</div>
            <div class="new_jobshow_tel"><?php echo $_smarty_tpl->tpl_vars['link']->value['linkMsg'];?>
，查看联系方式</div>
            <div class="new_jobshow_telbth">
                <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'register'),$_smarty_tpl);?>
" v-if="!uid">立即注册</a>
                <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
" class="newOobshow" v-if="!uid">立即登录</a>
            </div>
        </div>
    </van-popup>
    <van-popup v-model="joblinkshow" :position="joblinkshowpos" round closeable z-index="997">
        <div class="new_jobshow_telbox newObshowAlls" :style="{width: linkopenwidth}">
            <div class="newObshowTop">
                <div class="newHowTopTite">
                    <span>联系方式</span>
                </div>
                <div class="newHowTopData">
                    <div class="newHowTopImgs">
                        <div>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/interview_ipone.png" alt="">
                        </div>
                        <span>{{linktel}}</span>
                        <b>（{{linkman}}）</b>
                    </div>
                    <div class="newHowTopTells">
                        <a :href="'tel:' + linktel" onclick="addtellog();" class="">拨打电话</a>
                    </div>
                </div>
                <div class="new_jobshow_teltip">联系我时，请说是在<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
上看到的</div>
            </div>
            <div class="trainPopCon" v-if="joblinkopen > 1 && !uid && msgopen == 1 && msglogincode == 1">
                <div class="trainPopTitle">
                    <span>填写预留信息，方便企业与您联系</span>
                </div>
                <div class="trainPopFrom">
                    <form method="post" action="" onsubmit="return false;">
                        <div class="trainPopFromInt">
                            <span><i>*</i>手机号:</span>
                            <input type="text" id="rtel" name="rtel" autocomplete="off" onkeyup="this.value=this.value.replace(/[^0-9-]/g,'')" placeholder="请输入手机号">
                        </div>
                        <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"前台登录")!==false) {?>
                        <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==1) {?>
                        <div class="trainPopFromInt">
                            <span><i>*</i>验证码:</span>
                            <input class="formyanzhem" placeholder="请输入图片验证码" name="checkcode" id="checkcode" type="text" maxlength="6" />
                            <img id="vcode_img" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php" onclick="checkCode('vcode_img');" />
                        </div>
                        <?php }?>
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_login']=="1") {?>
                        <div class="trainPopFromInt">
                            <span><i>*</i>短信验证:</span>
                            <input type="text" value="" autocomplete="off" class="formyanzhem" placeholder="请输入验证码" name="moblie_code" id="rmoblie_code" />
                            <div class="linksudiv" id="send_msg_tips" onclick="rsendmsg('vcode_img');"><span id="rtime">获取验证码</span></div>
                        </div>
                        <?php }?>
                        <div class="trainPopFromInt">
                            <span><i>*</i>姓 名:</span>
                            <input type="text" id="rname" name="rname" autocomplete="off" placeholder="请输入姓名">
                        </div>
                        <div class="trainPopFromInt " @click="sexShow = true">
                            <span><i>*</i>性 别:</span>
                            <div class="xb_box" style="color:#cecccc;">
                                {{info.sex_n}}
                                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/lx_right.png" alt="" width="10" height="19">
                            </div>
                            <input type="hidden" name="sex" :value="info.sex">
                        </div>
                        <div class="trainPopFromInt">
                            <span><i>*</i>年 龄:</span>
                            <input type="text" id="rage" name="rage" autocomplete="off" onkeyup="this.value=this.value.replace(/[^0-9-]/g,'')" placeholder="请输入年龄">
                        </div>
                        <div class="trainPopFromBut">
                            <input type="hidden" id="rsend" name="rsend" value="0" />
                            <button @click="rsub">提交</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </van-popup>
    <van-popup v-model="sexShow" round position="bottom">
        <van-picker show-toolbar :columns="sex" :default-index="sexIndex" @cancel="sexShow = false" @confirm="sexConfirm" />
    </van-popup>
    <!--实地核验-->
    <van-popup v-model="fact_show" position="bottom" round :style="{ background: 'none', overflow: 'initial'}">
        <!--平台核验弹出框-->
        <div class="yunNewhyFlexs">
            <div class=" yun_newhy_box">
                <div class="yun_newhy_h1"><i class="yun_newhy_icon"></i><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
平台核验</div>
                <div class="yunNewhyConts">
                    <div class="yun_newhy_ztbox">
                        <?php if ($_smarty_tpl->tpl_vars['job']->value['yyzz_status']==1) {?>
                        <div class="yun_newhy_zt">
                            <i class="yun_newhy_zticon yun_newhy_zticon1"></i>
                            <div class="yun_newhy_zttit">资质核验</div>
                            <div class="yun_newhy_zt_p">招聘方的营业执照已通过认证</div>
                        </div>
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['job']->value['moblie_status']==1) {?>
                        <div class="yun_newhy_zt ">
                            <i class="yun_newhy_zticon yun_newhy_zticon2"></i>
                            <div class="yun_newhy_zttit">实名核验</div>
                            <div class="yun_newhy_zt_p">招聘人员已通过实名信息验证</div>
                        </div>
                        <?php }?>
                        <div class="yun_newhy_zt">
                            <i class="yun_newhy_zticon yun_newhy_zticon3"></i>
                            <div class="yun_newhy_zttit">实地核验</div>
                            <div class="yun_newhy_zt_p">平台已实地走访用人单位核实企业地址信息</div>
                        </div>
                    </div>
                    <div class="yun_newhy_hj">
                        <div class="navbox_jgw" style="height: auto;">
                            <div class="navboxGundImage">
                                <?php  $_smarty_tpl->tpl_vars['fact'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['fact']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['factlist']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['fact']->key => $_smarty_tpl->tpl_vars['fact']->value) {
$_smarty_tpl->tpl_vars['fact']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['fact']->key;
?>
                                <div>
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['fact']->value['picurl_n'];?>
" alt="" style="width: 100%;" @click="showfact('<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
')">
                                </div>
                                <?php } ?>
                            </div>
                        </div>
                    </div>
                    <div class="yun_newhy_tip ">
                        <div class="yun_newhy_tip_h1"><i class="yun_newhy_tip_h1icon"></i><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
平台核验服务</div>
                        <div class="yun_newhy_tip_p">我们会对职位信息进行全面考察，竭尽全力为您打造安全的求职环境</div>
                    </div>
                    <div class="yun_newhy_bthbox">
                        <button @click="fact_show=false" class="yun_newhy_bth">我知道了</button>
                    </div>
                </div>
            </div>
        </div>
        <!--平台核验-->
    </van-popup>
    <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"前台登录")!==false) {?>
    <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
    <input type='hidden' id="noblur" value="1" />
    <div class="gtdx-captcha">
        <div id="bind-captcha" data-id='send_msg_tips' data-type='click'></div>
        <input type='hidden' id="verify_token" name="verify_token" value="">
        <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
        <input type='hidden' id="verify_str" name="verify_str" value="" />
        <?php }?>
        <input type='hidden' id="popup-submit">
        <input type='hidden' id="bind-submit">
    </div>
    <?php }?>
    <?php }?>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/verify_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/clipboard/clipboard.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 type="text/javascript">
var wapurl = "<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
";
var throttleFlag;
var com_link_look = "<?php echo $_smarty_tpl->tpl_vars['config']->value['com_link_look'];?>
";
var isweixin = "<?php echo $_smarty_tpl->tpl_vars['isweixin']->value;?>
";
var usertype = "<?php echo $_smarty_tpl->tpl_vars['usertype']->value;?>
";
var uid = "<?php echo $_smarty_tpl->tpl_vars['uid']->value;?>
";
var com_login_link = "<?php echo $_smarty_tpl->tpl_vars['config']->value['com_login_link'];?>
";

var linkCode = '<?php echo $_smarty_tpl->tpl_vars['link']->value['linkCode'];?>
',
    linkSub = '<?php echo $_smarty_tpl->tpl_vars['link']->value['linkSub'];?>
';

var hbNum = '<?php echo $_smarty_tpl->tpl_vars['hbNum']->value;?>
';

var shareoptions = [{
        name: '微信',
        icon: '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/fx_wx.png',
        className: 'wechat'
    },
    {
        name: '复制文本',
        icon: '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/fx_wb.png',
        className: 'copytxt'
    },
    {
        name: '复制链接',
        icon: '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/fx_lj.png',
        className: 'link'
    }
];

'<?php if ($_smarty_tpl->tpl_vars['hbNum']->value>0) {?>'
shareoptions.splice(1, 0, {
    name: '分享海报',
    icon: '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/fx_hb.png',
    className: 'poster'
});
'<?php }?>'

var config = {
    url: '<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'share','id'=>$_smarty_tpl->tpl_vars['job']->value['id']),$_smarty_tpl);?>
',
    title: '<?php echo $_smarty_tpl->tpl_vars['title']->value;?>
',
    desc: '<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
',
    img: '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wx_sharelogo'];?>
',
    img_title: '<?php echo $_smarty_tpl->tpl_vars['job']->value['name'];?>
',
    from: '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
'
};
var sexData = [];
'<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['sexData']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>';
sexData.push({
    value: '<?php echo $_smarty_tpl->tpl_vars['key']->value;?>
',
    text: '<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
'
});
'<?php } ?>';

var factData = [];
'<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['factlist']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>'
factData.push('<?php echo $_smarty_tpl->tpl_vars['v']->value['picurl_n'];?>
');
'<?php } ?>'

var joblistvue = new Vue({
    el: '#app',
    data: {
        jobid: '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
',
        msgopen: '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_msg_isopen'];?>
',
        msglogincode: '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_msg_login'];?>
',
        joblinkopen: '<?php echo $_smarty_tpl->tpl_vars['job']->value['linkopen'];?>
',
        uid: '<?php echo $_smarty_tpl->tpl_vars['uid']->value;?>
',
        usertype: '<?php echo $_smarty_tpl->tpl_vars['usertype']->value;?>
',
        zixunShow: false,
        fav_job: '<?php echo $_smarty_tpl->tpl_vars['job']->value['fav_job'];?>
', //是否已收藏该职位

        //联系方式
        linkCode: linkCode ? parseInt(linkCode) : '',
        linkSub: linkSub ? parseInt(linkSub) : '',
        linkShow: false,
        telQrcodeBoxShow: false,
        telQrcode: '正在获取二维码...',
        //举报
        reportShow: false,
        report_reasons: [],
        report_result: [],
        //分享
        shareShow: false,
        shareoptions: shareoptions,

        //海报
        jobHbShow: false,
        hbList: [],
        hb: 0,
        hbSrc: '',

        hbShow: true,

        //复制文本
        copyBoxShow: false,
        // 热门问题
        hot_issues: [],
        question: '',
        prvBox: false,
        prvlinktel: '',
        prvtime: 0,
        prvusertel: '',
        wxpubtemp_html: '',

        linktel: '',
        linkman: '',
        // 登录弹框
        loginbox: false,
        linkopenBox: false,
        linkopen: '<?php echo $_smarty_tpl->tpl_vars['job']->value['linkopen'];?>
',
        joblinkshow: false,
        joblinkshowpos: 'center',

        sexShow: false,
        sex: sexData,
        sexIndex: 0,
        info: {
            sex: null,
            sex_n: '请选择性别'
        },
        linkopenwidth: '9.5rem',

        fact_show: false,
    },
    created() {
        $('#app').css('display', 'block');
        // 点拨打电话，直接投递简历，页面刷新后，弹出联系方式
        var applyForTel = sessionStorage.getItem("applyForTel");
        if (applyForTel) {
            this.openLinkTip();
            sessionStorage.removeItem("applyForTel");
        }
    },
    methods: {
        showfact: function(e) {

            var imgarr = factData;

            vant.ImagePreview({
                images: imgarr,
                startPosition: parseInt(e),
            });
        },
        getopenpos() {
            var that = this
            if (that.joblinkopen > 1 && !that.uid && that.msgopen == 1 && that.msglogincode == 1) {
                that.joblinkshowpos = 'bottom'
                that.linkopenwidth = ''
            } else {
                that.joblinkshowpos = 'center'
                that.linkopenwidth = '9.5rem'
            }
        },
        rgetTel() {
            var that = this;
            $.post(wapurl + "index.php?c=job&a=getJobLink", {
                id: that.jobid
            }, function(data) {
                var data = eval('(' + data + ')');
                that.linktel = data.linktel;
                that.linkman = data.linkman;
                that.getopenpos()
                that.joblinkshow = true;
            });
        },
        sexConfirm(e) {
            this.sexShow = false;
            this.info.sex = e.value;
            this.info.sex_n = e.text;
        },
        rsub: function() {
            var that = this;
            var rname = $("#rname").val();
            var rmoblie = $("#rtel").val();
            var rmoblie_code = $("#rmoblie_code").val();
            var rage = $("#rage").val();
            if (!rmoblie) {
                showToast('手机号不能为空！');
                return false;
            }
            if (rmoblie_code == '') {
                showToast('短信验证不能为空！');
                return false;
            }
            if (!rage) {
                showToast('年龄不能为空！');
                return false;
            }
            if (rage < 16 || rage > 100) {
                showToast('年龄不能低于16且不能大于100！');
                return false;
            }
            if (!that.info.sex) {
                showToast('请选择性别！');
                return false;
            }
            if (!rname) {
                showToast('姓名不能为空！');
                return false;
            }
            var param = {
                realname: rname,
                username: rmoblie,
                dynamiccode: rmoblie_code,
                authcode: $('#checkcode').val(),
                act_login: 1,
                age: rage,
                sex: that.info.sex,
                is_yuliu: 1,
                jid: that.jobid
            };
            showLoading();
            $.post(wapurl + 'index.php?c=login&a=mlogin', param, function(data) {
                if (data) {
                    var msg = data.msg;
                    if (data.errcode == 9) {
                        showToast(msg, 2, function() {
                            that.linkopenBox = false;
                            window.location.reload();
                        });
                    } else {
                        showToast(msg);
                        return false;
                    }
                }
            }, 'json');
        },
        openLinkTip: function() {
            var that = this;
            if (that.linkCode > 9) {

                this.getLink();
            } else if (that.linkCode < 9) {
                if (that.linkCode == 6) {
                    if(that.uid){
                        showToast('请先登录个人账号');
                    }else {
                        pleaselogin('您还未登录个人账号，是否登录？', '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
')
                    }
                } else {
                    that.linkShow = true;
                }
            } else if (isweixin) {

                this.telQrcode = '<img src="' + wapurl + 'index.php?c=job&a=telQrcode&id=' + that.jobid +
                    '" width="120" height="120">';
                this.telQrcodeBoxShow = true;
            } else {

                showToast('请打开微信查看那联系方式！');
            }
        },
        toJobfav: function() {
            var that = this;
            if (that.usertype != '1' && that.uid != '') {
                showToast('只有个人用户才能收藏');
            } else if (that.uid == '') {
                pleaselogin('您还未登录个人账号，是否登录？', '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
')
                return false;
            } else {
                jobfav('<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
');
            }
        },
        jobapply: function(showTel) {
            var that = this;
            var jobid = $("#jobid").val();

            showLoading()

            $.get(wapurl + "/index.php?c=job&a=comapply&type=sq&id=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
",
                function(data) {
                    hideLoading();
                    var data = eval('(' + data + ')');
                    if (data.state == 9) {
                        if (showTel) {
                            sessionStorage.setItem("applyForTel", 1);
                            window.location.reload();
                        } else {
                            showToast(data.msg, 2, function() {
                                window.location.reload();
                            });
                        }
                    } else {
                        if (that.linkCode == 1 && showTel) {

                            that.openLinkTip();
                        } else {
                            if (data.state == 1) {
                                pleaselogin('您还未登录个人账号，是否登录？', '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
')
                            } else if (data.state == 2) {
                                showToast('只有个人用户才能投递');
                                return false;
                            } else if (data.state == 7 || data.state == 101 || data.state == 102) {

                                showConfirm(data.msg, function() {
                                    location.href = wapurl + data.url;
                                }, '取消', '前往完善');

                            } else if (data.url) {
                                if (data.url == '1') {
                                    url = location.href;
                                }
                                showToast(data.msg, 2, function() {
                                    location.href = wapurl + data.url;
                                });
                            }
                        }
                    }
                }
            );
        },
        chooseReason: function(reason) {
            if (this.report_result.indexOf(reason) == -1) {
                this.report_result.push(reason);
            } else {
                var list = deepClone(this.report_result);
                for (var i = 0; i < list.length; i++) {
                    if (reason == list[i]) {
                        list.splice(i, 1);
                    }
                }
                this.report_result = list;
            }
        },
        reportSub: function() {
            var authcode = $("#authcode").val();
            var reason = "理由：";

            if (this.report_result.length > 0) {
                reason += this.report_result.join('，') + '；';
            } else {
                showToast('请选择举报理由！');
                return false;
            }

            var r6 = $("#r_reason").val();
            var reason = reason + r6;
            if (authcode == '') {
                showToast('请填写验证码！');
                return false;
            }

            showLoading()
            $.post(wapurl + "?c=job&a=report", {
                id: '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
',
                authcode: authcode,
                reason: reason
            }, function(data) {
                hideLoading();
                var data = eval('(' + data + ')');
                if (data.url) {
                    showToast(data.msg, 2, function() {
                        location.href = data.url;
                    });
                } else {
                    showToast(data.msg, 2, function() {
                        location.reload(true);
                    });
                }
            });
        },
        shareSelect: function(e) {
            var that = this;
            var classname = e.className;
            this.closetip();
            if (classname == 'wechat') {
                $('#shareClick').click();
            } else if (classname == 'poster') {
                that.getJobHb();
            } else if (classname == 'copytxt') {
                this.wbRequest();
            } else if (classname == 'link') {
                $('.copyUrl').click();
            }
        },
        getJobHb: function() {
            var that = this;
            showLoading();
            $.post(wapurl + "?c=job&a=getHbList", { rand: Math.random() }, function(data) {
                var data = eval('(' + data + ')');
                hideLoading();

                if (data.errcode == 1) {

                    that.hbList = data.hbList;
                    that.hb = data.hbList[0].id;

                    that.hbSrc = wapurl + '?c=ajax&a=getJobHb&id=' + that.jobid + '&hb=' + that.hb;
                    that.jobHbShow = true;
                    setTimeout(function() {
                        if (document.getElementById("bighb")) {
                            showLoading('生成中...');
                            document.getElementById("bighb").onload = function() {
                                hideLoading();
                            }
                        }
                    }, 10);
                } else {
                    showToast(data.errmsg);
                    return false;
                }
            });
        },
        getNewJobHb: function(hb) {

            var that = this;
            if (hb != that.hb) {
                that.hb = hb;
                that.hbSrc = wapurl + '?c=ajax&a=getJobHb&id=' + that.jobid + '&hb=' + hb;
                if (document.getElementById("bighb")) {
                    showLoading('生成中...');
                    document.getElementById("bighb").onload = function() {
                        hideLoading();
                    }
                }
            }
        },
        closetip: function() {
            var that = this;
            that.zixunShow = false;
            that.linkShow = false;
            that.telQrcodeBoxShow = false;
            that.reportShow = false;
            that.shareShow = false;
            that.jobHbShow = false;
            that.copyBoxShow = false;
        },
        checkQuestion: function(key) {
            this.question = this.hot_issues[key];
        },
        getLink: function() {
            var that = this;
            showLoading();
            $.post(wapurl + "?c=job&a=getLink", { jobid: '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
' }, function(data) {
                hideLoading();
                if (data.linkCode == 10) {
                    that.prvlinktel = data.prvlinktel;
                    that.prvusertel = data.prvusertel;
                    that.prvtime = data.prvtime;
                    that.countdown();
                    that.prvBox = true;
                } else if (data.linkCode == 11) {
                    showToast(data.linkMsg);
                } else {
                    if (that.linkCode == 6) {
                        showToast('请先登录个人账号');
                    } else {
                        that.linkShow = true;
                    }
                }
            }, 'json');
        },
        countdown() {
            var that = this;
            if (that.prvtime > 0) {
                setTimeout(function() {
                    that.prvtime = that.prvtime - 1;
                    that.countdown();
                }, 1000);
            }
        },
        prvBoxClose() {
            this.prvlinktel = '';
            this.prvusertel = '';
            this.prvtime = 0;
        },
        // loginboxOpen(){
        //     this.loginbox = true;
        // },
        loginboxClose() {
            this.loginbox = false;
        },
        wbRequest: function() {
            var that = this;
            showLoading();
            $.post(wapurl + "?c=job&a=getJobWb", { jobid: '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
' }, function(data) {
                hideLoading();
                if (data.wxpubtemp_html) {
                    that.wxpubtemp_html = data.wxpubtemp_html;
                    that.copyBoxShow = true;
                }
            }, 'json');
        },
        footShare: function() {
            this.shareShow = true;
        },
        goHome: function() {
            window.location.href = '<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
';
        }
    }
});
$(function() {
    var clipboard = new ClipboardJS(".copyUrl");
    clipboard.on('success', function(e) {
        showToast('复制成功！');
        joblistvue.closetip();
        e.clearSelection();
    });
    clipboard.on('error', function(e) {});

    var clipboard2 = new ClipboardJS(".fzwb");
    clipboard2.on('success', function(e) {
        showToast('复制成功！');
        joblistvue.closetip();
        e.clearSelection();
    });
    clipboard2.on('error', function(e) {});
})

function zixunSubs() {
    var authcode = $("#authcodes").val();
    var reason = $("#reasons").val();
    var jobid = $.trim($("input[name='jobid']").val());
    if (reason == '') {
        showToast('请填写咨询内容！');
        return false;
    }
    if (authcode == '') {
        showToast('请填写验证码！');
        return false;
    }

    showLoading()
    $.post(wapurl + "?c=job&a=msg", {
        authcode: authcode,
        content: reason,
        jobid: jobid
    }, function(data) {
        hideLoading();
        var data = eval('(' + data + ')');
        if (data.url) {
            showToast(data.msg, 2, function() {
                location.href = data.url;
            });
        } else {
            showToast(data.msg, 2, function() {
                location.reload(true);
            });
        }
    });
}

function addtellog() {
    $.post(wapurl + "?c=ajax&a=addJobTelLog", {
        jobid: '<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
',
    }, function(data) {

    });
}

function toJobfav() {
    joblistvue.toJobfav();
}

function cancelFavJob(id) {

    showLoading('请稍候...');

    $.post(wapurl + "?c=job&a=cancelJobFav", { id, id }, function(data) {
        hideLoading();
        var data = eval('(' + data + ')');
        if (data.errcode == 9) {

            showToast(data.msg, 2, function() {
                location.reload(true);
            });
        } else {

            showToast(data.msg);
            return false;
        }
    })
}

function jobfav(jobid) {
    showLoading('收藏中，请稍等...');
    $.get(wapurl + "?c=job&a=comapply&type=fav&id=<?php echo $_smarty_tpl->tpl_vars['job']->value['id'];?>
", function(data) {
        hideLoading();
        var data = eval('(' + data + ')');
        if (data.state == 0) {
            pleaselogin('您还未登录个人账号，是否登录？', '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
')
        } else if (data.state == 4) {
            showToast('只有个人用户才能收藏');
            return false;
        } else if (data.url) {
            if (data.url == '1') {
                url = location.href;
            }
            showToast(data.msg, 2, function() {
                location.href = data.url;
            });
        } else {
            showToast(data.msg, 2, function() {
                location.reload(true);
            });
        }
    })
}

function toReportCom() {
    var that = this;
    var url = '<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'getreport'),$_smarty_tpl);?>
';
    $.post(url, { rand: Math.random() }, function(res) {
        if (res) {
            joblistvue.$data.report_reasons = res.reason;
        }
    }, 'json');

    if (usertype == '1') {
        checkCode('vcode_img');
        joblistvue.$data.reportShow = true;
    } else if (uid != '') {
        showToast('只有个人用户才能举报');
    } else {
        pleaselogin('您还未登录个人账号，是否登录？', '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
');
    }
}

function pmLook() {
    if (joblistvue.$data.uid) {
        if (joblistvue.$data.usertype == '1') {
            window.location.href = '<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'compete','id'=>$_smarty_tpl->tpl_vars['job']->value['id']),$_smarty_tpl);?>
';
        } else {
            showToast('只有个人用户才能查看');
        }
    } else {
        showConfirm('您还未登录个人账号，是否登录？', function() {
            window.location.href = '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
';
        })
    }
}

function zixun() {
    if (joblistvue.$data.uid) {
        if (joblistvue.$data.usertype == '1') {
            checkCode('vcode_imgs');
            $.post('<?php echo smarty_function_url(array('d'=>'wxapp','m'=>'job','c'=>'getHotIssues'),$_smarty_tpl);?>
', { rand: Math.random() }, function(res) {
                if (res.data.list) {
                    joblistvue.$data.hot_issues = res.data.list
                }
            }, 'json')
            joblistvue.$data.zixunShow = true;
        } else {
            showToast('只有个人用户才能提问');
        }
    } else {
        showConfirm('您还未登录个人账号，是否登录？', function() {
            window.location.href = '<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
';
        })
    }
}
$(function() {
    '<?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>'
    var id = '<?php echo $_GET['id'];?>
';
    $.post(wapurl + 'index.php?c=job&a=history', { id: id }, function(data) {
        return true;
    });
    '<?php }?>'
});

function loginboxOpen() {
    joblistvue.$data.loginbox = true;
}

function rsendmsg(img) {
    var rsend = $("#rsend").val();
    var rmoblie = $("#rtel").val();
    if (rmoblie == "") {
        showToast("请填写手机号！");
        return false;
    } else if (!isjsMobile(rmoblie)) {
        showToast("手机格式不正确！");
        return false;
    }
    if (rsend > 0) {
        showToast('请不要频繁重复发送！');
        return false;
    }
    var code;
    var noblur = $("#noblur").val();
    var verify_token;
    var verify_str;
    var codesear = new RegExp('前台登录');
    if (codesear.test(code_web)) {
        if (code_kind == 1) {
            code = $.trim($("#checkcode").val());
            if (!code) {
                showToast('请填写图片验证码！');
                return false;
            }
        } else if (code_kind > 2) {
            verify_token = $('input[name="verify_token"]').val();
            if (verify_token == '') {
                if (code_kind == 6) {
                    $("#bind-captcha").trigger("click");
                } else {
                    $("#bind-submit").trigger("click");
                }
                return false;
            }
            verify_str = $('input[name="verify_str"]').val();
        }
    }
    if (rsend == 0) {

        showLoading();
        $.post(wapurl + "/index.php?c=login&a=sendmsg", {
            moblie: rmoblie,
            authcode: code,
            verify_token: verify_token,
            verify_str: verify_str,
            noblur: noblur,
            is_yuliu: 1
        }, function(data) {
            hideLoading();
            if (data) {
                var res = JSON.parse(data);
                if (res.error == 1) {
                    rsendtime("121");
                }
                showToast(res.msg, 2, function() {
                    if (res.error != 1) {
                        if (code_kind == 1) {
                            checkCode(img);

                        } else if (code_kind > 2) {
                            $("#popup-submit").trigger("click");
                        }
                    }
                });
            }
        })
    }
}

function rsendtime(i) {
    i--;
    if (i == -1) {
        $("#rtime").html("重新获取");
        $("#rsend").val(0)
    } else {
        $("#rsend").val(1)
        $("#rtime").html(i + "秒");
        setTimeout("rsendtime(" + i + ");", 1000);
    }
}

function factShow() {
    joblistvue.$data.fact_show = true;


}
<?php echo '</script'; ?>
>
</body>

</html><?php }} ?>
