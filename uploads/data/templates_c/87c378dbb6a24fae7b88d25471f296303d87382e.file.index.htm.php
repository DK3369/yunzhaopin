<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 13:00:14
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/index/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:172695775569e855de02eda1-43465243%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '87c378dbb6a24fae7b88d25471f296303d87382e' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/index/index.htm',
      1 => 1700725931,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '172695775569e855de02eda1-43465243',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'title' => 0,
    'keywords' => 0,
    'description' => 0,
    'style' => 0,
    'config' => 0,
    'ishtml' => 0,
    'bannerFlag' => 0,
    'lunbo' => 0,
    'adlist_73' => 0,
    'adlist_72' => 0,
    'job_index' => 0,
    'j' => 0,
    'v' => 0,
    'job_name' => 0,
    'job_type' => 0,
    'jobclassurl' => 0,
    'vv' => 0,
    'vvv' => 0,
    'urgent_list' => 0,
    'announcementlist' => 0,
    'adlist_13' => 0,
    'adlist_14' => 0,
    'adlist_15' => 0,
    'hotjoblist' => 0,
    'job_list' => 0,
    'adlist_92' => 0,
    'ulist_rec' => 0,
    'uid' => 0,
    'usertype' => 0,
    'eid' => 0,
    'ulist' => 0,
    'plist' => 0,
    'nlist' => 0,
    'key' => 0,
    'alist' => 0,
    'linklist' => 0,
    'linklist2' => 0,
    'footer_ad' => 0,
    'left_ad' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e855de0889d0_18757016',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e855de0889d0_18757016')) {function content_69e855de0889d0_18757016($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_function_listurl')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.listurl.php';
if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
if (!is_callable('smarty_function_webspecial')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.webspecial.php';
?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN""http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <meta http-equiv="Content-Type" content="text/html;charset=utf-8"/>

    <title><?php echo $_smarty_tpl->tpl_vars['title']->value;?>
</title>

    <meta name="keywords" content="<?php echo $_smarty_tpl->tpl_vars['keywords']->value;?>
"/>
    <meta name="description" content="<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
"/>

    <link href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/index.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>
    <link href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/css.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>
    <link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/css/layui.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>
    <link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/swiper/swiper-bundle.min.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>

    <?php if ($_smarty_tpl->tpl_vars['ishtml']->value) {?>
    <?php echo '<script'; ?>
 src="<?php echo smarty_function_url(array('m'=>'ajax','c'=>'wjump'),$_smarty_tpl);?>
" language="javascript"><?php echo '</script'; ?>
>
    <?php }?>
</head>
<body class="body_bg">
<!-- 首页弹出广告 start-->
<style type="text/css">
    .yhq_tip {
        display: none;
    }
    .header_fixed {
        z-index: 999;
    }
    .tcbanner {
        width: 100%;
        text-align: center;
        position: fixed;
        z-index: 1002;
		height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
    }
    .tcbanner_gb {
        padding-top: 20px;
        text-align: center;
        color: #fff;
        background: rgba(0, 0, 0, 0.5);
        padding: 3px 10px;
        border-radius: 20px;
        position: fixed;
        top: 100px;
        right: 50px;
        display: inline-block;
        z-index: 1002;
        cursor: pointer;
    }
	.yhq_tip_bg{background:rgba(0%,0%,0%,0.5);bottom:0;display:none;left:0;position:absolute;right:0;top 0;width:100%;height:100%;z-index:1001;}
</style>

<?php if (!$_smarty_tpl->tpl_vars['bannerFlag']->value) {?>
<div id="yhq_tip" class="yhq_tip">
	<?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"506","item"=>"\"lunbo\"","key"=>"“key“","random"=>"1","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[506];if(is_array($add_arr) && !empty($add_arr)){
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
	<div class="tcbanner" id="adContent"><?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
</div>
	<?php } ?>
    <span id="yhq_tip_close" class="tcbanner_gb">关闭</span>
    <div id="yhq_tip_bg" class="yhq_tip_bg" style="display: block;"></div>
</div>
<?php }?>
<!-- 首页弹出广告 end-->
<div class="index_zs_banner index_zs_banner2 none" id="js_ads_banner_top">
    <?php  $_smarty_tpl->tpl_vars['adlist_73'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['adlist_73']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"73","limit"=>"1","item"=>"“adlist_73“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[73];if(is_array($add_arr) && !empty($add_arr)){
				$i=0;$limit = 1;$length = 0;
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['adlist_73']->key => $_smarty_tpl->tpl_vars['adlist_73']->value) {
$_smarty_tpl->tpl_vars['adlist_73']->_loop = true;
?> <?php echo $_smarty_tpl->tpl_vars['adlist_73']->value['html'];?>
 <?php } ?>
</div>

<div class="index_zs_banner index_zs_banner1" id="js_ads_banner_top_slide">
    <?php  $_smarty_tpl->tpl_vars['adlist_72'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['adlist_72']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"72","limit"=>"1","item"=>"“adlist_72“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[72];if(is_array($add_arr) && !empty($add_arr)){
				$i=0;$limit = 1;$length = 0;
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['adlist_72']->key => $_smarty_tpl->tpl_vars['adlist_72']->value) {
$_smarty_tpl->tpl_vars['adlist_72']->_loop = true;
?> <?php echo $_smarty_tpl->tpl_vars['adlist_72']->value['html'];?>
 <?php } ?>
</div>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/index_header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


<div class="clear"></div>

<div class="w1200">
    <div class="first_floor ">
        <div class="first_floor_top">
            <!-- 职位类别-->
            <div class="yunheader_60zwlb">
                <div class="leftNav ">
                    <div id="menuLst" class="menuLst">
                        <ul>
                            <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['job_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                            <?php if ($_smarty_tpl->tpl_vars['j']->value<11) {?>
                            <li class="lst<?php echo $_smarty_tpl->tpl_vars['j']->value;?>
 " onmouseover="show_job(<?php echo $_smarty_tpl->tpl_vars['j']->value;?>
,'1');" onmouseout="hide_job(<?php echo $_smarty_tpl->tpl_vars['j']->value;?>
);"><b></b>
                                <a class="link" href="<?php echo smarty_function_url(array('m'=>'job','c'=>'search','job1'=>$_smarty_tpl->tpl_vars['v']->value),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</a><i></i>
                                <div class="lstCon">
                                    <div class="lstConClass">
                                        <?php  $_smarty_tpl->tpl_vars['vv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vv']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['job_type']->value[$_smarty_tpl->tpl_vars['v']->value]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vv']->key => $_smarty_tpl->tpl_vars['vv']->value) {
$_smarty_tpl->tpl_vars['vv']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['vv']->key;
?>
                                        <dl>
                                            <dt>
                                                <a href="<?php echo $_smarty_tpl->tpl_vars['jobclassurl']->value;?>
job1=<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
&job1_son=<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['vv']->value];?>
</a>
                                            </dt>
                                            <dd>
                                                <?php  $_smarty_tpl->tpl_vars['vvv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['vvv']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job_type']->value[$_smarty_tpl->tpl_vars['vv']->value]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['vvv']->key => $_smarty_tpl->tpl_vars['vvv']->value) {
$_smarty_tpl->tpl_vars['vvv']->_loop = true;
?>
                                                <a href="<?php echo $_smarty_tpl->tpl_vars['jobclassurl']->value;?>
job1=<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
&job1_son=<?php echo $_smarty_tpl->tpl_vars['vv']->value;?>
&job_post=<?php echo $_smarty_tpl->tpl_vars['vvv']->value;?>
"><?php echo $_smarty_tpl->tpl_vars['job_name']->value[$_smarty_tpl->tpl_vars['vvv']->value];?>
</a>
                                                <?php } ?>
                                            </dd>
                                            <dd style="display:block;clear:both;float:inherit;width:100%;font-size:0;line-height:0;"></dd>
                                        </dl>
                                        <?php } ?>
                                    </div>
                                </div>
                            </li>
                            <?php }?>
                            <?php } ?>
                        </ul>
                    </div>
                </div>
                <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/js/pop_up.js" language="javascript"><?php echo '</script'; ?>
>
                <!--悬浮结束-->
            </div>

            <!-- 幻灯 公告-->
            <div class="index_frist_box">
                <div class="index_huandeng" id="ban">
                    <div class="layui-carousel" id="test1" lay-filter="test1">
                        <div carousel-item="">
                            <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"3","item"=>"\"lunbo\"","key"=>"“key“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[3];if(is_array($add_arr) && !empty($add_arr)){
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
                            <?php echo $_smarty_tpl->tpl_vars['lunbo']->value['lay_html'];?>

                            <?php } ?>
                        </div>
                    </div>
                </div>
                <!-- 新急聘 -->
                <div id="newurgentCtrl" class="yunheader_60jpbox swiper-container">
                    <div class="swiper-wrapper">
                        <?php  $_smarty_tpl->tpl_vars['urgent_list'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['urgent_list']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("namelen"=>"30","comlen"=>"30","urgent"=>"“1“","limit"=>"32","key"=>"“key“","item"=>"“urgent_list“","name"=>"“urgent_list1“","nocache"=>"")
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
		
		$urgent_list = $db->select_all("company_job",$where.$limit);

		if(is_array($urgent_list) && !empty($urgent_list)){
			$comuid=$jobid=array();
			foreach($urgent_list as $key=>$value){
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
			foreach($urgent_list as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($urgent_list[$key]);
					continue;
				}else{
					$urgent_list[$key] = $db->array_action($value,$cache_array);
					$urgent_list[$key][stime] = date("Y-m-d",$value[sdate]);
					$urgent_list[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$urgent_list[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$urgent_list[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$urgent_list[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$urgent_list[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $urgent_list[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $urgent_list[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $urgent_list[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $urgent_list[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $urgent_list[$key][job_number] = "";
					}else{
					    $urgent_list[$key][job_number] = $value[zp_num]." 人";
					}			
                    $urgent_list[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $urgent_list[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $urgent_list[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$urgent_list[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$urgent_list[$key][pr_n] = $comclass_name[$value[pr]];
					$urgent_list[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$urgent_list[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$urgent_list[$key]['time'] = $urgent_list[$key]['lastupdate'];
						$urgent_list[$key]['redtime'] =1;
					}else{
						$urgent_list[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$urgent_list[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$urgent_list[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$urgent_list[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$urgent_list[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$urgent_list[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$urgent_list[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$urgent_list[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$urgent_list[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$urgent_list[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$urgent_list[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$urgent_list[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$urgent_list[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$urgent_list[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$urgent_list[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$urgent_list[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$urgent_list[$key][name_n]);
						$urgent_list[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$urgent_list[$key][com_n]);
						$urgent_list[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$urgent_list[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $urgent_list[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $urgent_list[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($urgent_list)){
				if($paramer[keyword]!=""&&!empty($urgent_list)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$urgent_list = $urgent_list; if (!is_array($urgent_list) && !is_object($urgent_list)) { settype($urgent_list, 'array');}
foreach ($urgent_list as $_smarty_tpl->tpl_vars['urgent_list']->key => $_smarty_tpl->tpl_vars['urgent_list']->value) {
$_smarty_tpl->tpl_vars['urgent_list']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['urgent_list']->key;
?>
                        <div class="swiper-slide">
                            <div class="js_new">
                                <a href="<?php echo $_smarty_tpl->tpl_vars['urgent_list']->value['job_url'];?>
" target="_blank" title="<?php echo $_smarty_tpl->tpl_vars['urgent_list']->value['name'];?>
" class="yunheader_60jp">
                                    <i class="yunheader_60jpicon"></i>
                                    <div class="yunheader_60jplogo">
                                        <img src="<?php echo $_smarty_tpl->tpl_vars['urgent_list']->value['logo'];?>
">
                                    </div>
                                    <div class="yunheader_60jpbane"><?php echo $_smarty_tpl->tpl_vars['urgent_list']->value['name'];?>
</div>
                                    <div class="yunheader_60jpxz">
                                        <?php if ($_smarty_tpl->tpl_vars['urgent_list']->value['job_salary']!='面议') {
}
echo $_smarty_tpl->tpl_vars['urgent_list']->value['job_salary'];?>

                                    </div>
                                </a>
                                <div class="yunheader_60jpcom">
                                    <?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['urgent_list']->value['com_name']),0,14,'utf-8');?>

                                </div>
                            </div>
                        </div>
                        <?php } ?>
                    </div>
                </div>
                <!-- 新急聘 -->
            </div>

            <!-- 登录-->
            <div class="fastloginbox ">
                <div class="hp_login"></div>
                <div class="new_gg  fl">
                    <div class="new_gg_tit">网站公告<a href="<?php echo smarty_function_url(array('m'=>'announcement'),$_smarty_tpl);?>
" class="new_gg_titmore">更多</a></div>
                    <ul>
                        <?php  $_smarty_tpl->tpl_vars['announcementlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['announcementlist']->_loop = false;
$announcementlist=array();$time=time();$paramer=array("limit"=>"4","item"=>"“announcementlist“","t_len"=>"30","nocache"=>"")
;
		global $db,$db_config,$config;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		$Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		$where = 1;
		//分站
		if($config['did']){
			$where.=" and (`did`='".$config['did']."' or `did`=-1)";
		}else{
			$where.=" and (`did`=-1 OR `did`=0 OR did='')";
		}
        $where.=" and (`startime`<=".time()." or `startime`=0 or `startime` is null)";
        $where.=" and (`endtime`>".time()." or `endtime`=0 or `endtime` is null)";
		if($paramer[limit]){
			$limit=" LIMIT ".$paramer[limit];
		}else{
			$limit=" LIMIT 20";
		}
		if($paramer[ispage]){
			$limit = PageNav($paramer,$_GET,"admin_announcement",$where,$Purl,"",0,$_smarty_tpl);
		}
		//排序字段 默认按照xuanshang排序
		if($paramer[order]){
			$where.="  ORDER BY `".$paramer[order]."`";
		}else{
			$where.="  ORDER BY `startime`";
		}
		//排序方式默认倒序
		if($paramer[sort]){
			$where.=" ".$paramer[sort];
		}else{
			$where.=" DESC";
		}

		$announcementlist=$db->select_all("admin_announcement",$where.$limit);
		if(is_array($announcementlist)){
			foreach($announcementlist as $key=>$value){
				//截取标题
				if($paramer[t_len]){
					$announcementlist[$key][title_n] = mb_substr($value['title'],0,$paramer[t_len],"utf-8");
				}
				$announcementlist[$key][time]=date("Y-m-d",$value[startime]);
				$announcementlist[$key][url] = Url("announcement",array("id"=>$value[id]),"1");
			}
		}$announcementlist = $announcementlist; if (!is_array($announcementlist) && !is_object($announcementlist)) { settype($announcementlist, 'array');}
foreach ($announcementlist as $_smarty_tpl->tpl_vars['announcementlist']->key => $_smarty_tpl->tpl_vars['announcementlist']->value) {
$_smarty_tpl->tpl_vars['announcementlist']->_loop = true;
?>
                        <li><a href="<?php echo $_smarty_tpl->tpl_vars['announcementlist']->value['url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['announcementlist']->value['title_n'];?>
"><?php echo $_smarty_tpl->tpl_vars['announcementlist']->value['title_n'];?>
</a></li>
                        <?php } ?>
                    </ul>
                </div>
            </div>
        </div>
    </div>
    <!-- 首页banner 区域-->
    <div class="index_banner fl">
        
        <div class="index_banner_1250 fl">
            <?php  $_smarty_tpl->tpl_vars['adlist_13'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['adlist_13']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"13","item"=>"“adlist_13“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[13];if(is_array($add_arr) && !empty($add_arr)){
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['adlist_13']->key => $_smarty_tpl->tpl_vars['adlist_13']->value) {
$_smarty_tpl->tpl_vars['adlist_13']->_loop = true;
?>
            <div class="b_w1200 b_tip"><?php echo $_smarty_tpl->tpl_vars['adlist_13']->value['lay_html'];?>
</div>
            <?php } ?>
            <?php  $_smarty_tpl->tpl_vars['adlist_14'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['adlist_14']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"14","item"=>"“adlist_14“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[14];if(is_array($add_arr) && !empty($add_arr)){
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['adlist_14']->key => $_smarty_tpl->tpl_vars['adlist_14']->value) {
$_smarty_tpl->tpl_vars['adlist_14']->_loop = true;
?>
            <div class="b_w289 b_tip"><?php echo $_smarty_tpl->tpl_vars['adlist_14']->value['lay_html'];?>
</div>
            <?php } ?>
            <?php  $_smarty_tpl->tpl_vars['adlist_15'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['adlist_15']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"15","item"=>"“adlist_15“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[15];if(is_array($add_arr) && !empty($add_arr)){
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['adlist_15']->key => $_smarty_tpl->tpl_vars['adlist_15']->value) {
$_smarty_tpl->tpl_vars['adlist_15']->_loop = true;
?>
            <div class="b_w143 b_tip"><?php echo $_smarty_tpl->tpl_vars['adlist_15']->value['lay_html'];?>
</div>
            <?php } ?>
        </div>
    </div>

    <div class="index_frame_right">
        <div class="yunheader_60_tit"><a href="<?php echo smarty_function_listurl(array('m'=>'company','rec'=>1),$_smarty_tpl);?>
" target="_blank" class="yunheader_60_tit_a"><i class="yunheader_60_tit_line"></i>名企招聘<i class="yunheader_60_tit_rline"></i></a></div>
        <div class="index_mq_box">
            <div class="index_mq_box_cont">
                <?php  $_smarty_tpl->tpl_vars['hotjoblist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['hotjoblist']->_loop = false;
global $db,$db_config,$config;$paramer=array("item"=>"“hotjoblist“","limit"=>"12","nocache"=>"")
;$hotjoblist=array();
		
		$time = time();
		//处理传入参数，并且构造分页参数
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr['arr'];
		$Purl =  $ParamerArr['purl'];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		//是否属于分站下
		if($config[sy_web_site]=="1"){
			$jobwheres="";
			if($config[province]>0 && $config[province]!=""){
				$jobwheres.=" and `provinceid`='$config[province]'";
			}
			if($config[cityid]>0 && $config[cityid]!=""){
				$jobwheres.=" and `cityid`='$config[cityid]'";
			}
			if($config[three_cityid]>0 && $config[three_cityid]!=""){
				$jobwheres.=" and `three_cityid`='$config[three_cityid]'";
			}
			if($config[hyclass]>0 && $config[hyclass]!=""){
				$jobwheres.=" and `hy`='".$config[hyclass]."'";
			} 
			if($jobwheres){
				$comlist=$db->select_all("company","`hottime`>$time ".$jobwheres,"`uid`");
				if(is_array($comlist)){
					foreach($comlist as $v){
						$cuid[]=$v[uid];
					}
				}
				$hotwhere=" and `uid` in (".@implode(",",$cuid).")";
			} 
		}
        $limit = "";
        if($paramer['limit']){
			$limit=" limit ".$paramer['limit'];
		}
		$where = "`time_start`<$time AND `time_end`>$time".$hotwhere;
		//排除不是已审核的企业
		$cominfo=$db->select_all("company","`rec`=1 AND `hotstart`<$time AND `hottime`>$time AND `r_status`<>1","`uid`");
		if($cominfo&&is_array($cominfo)){
    		foreach($cominfo as $v){
    		    $cinfouid[]=$v[uid];
    		}
    		$where.=" AND `uid` not in (".@implode(",",$cinfouid).")";
		}

        if($config['hotcom_top'] == 1){
            // 职位更新时间(职位修改时，会更新名企表lastupdate字段)
            $order = " ORDER BY `lastupdate` DESC ";
        }elseif($config['hotcom_top'] == 2){
            // 随机
            $order = " ORDER BY rand() ";
        }else{
            // 后台手动设置
            $order = " ORDER BY `sort` DESC ";
        };
		//分页
		if($ispage){
			$limit = PageNav($paramer,$_GET,"hotjob",$where,$Purl,'0',$_smarty_tpl);;
		}
		$where.=$order;
        
        $Query = $db->query("SELECT * FROM $db_config[def]hotjob where ".$where.$limit); 
		while($rs = $db->fetch_array($Query)){
			$hotjoblist[] = $rs;
			$ListId[] =  $rs[uid];
		}
        
		//是否需要查询对应职位
		$ComId  =   @implode(",",$ListId);
		$comList=   $db->select_all("company","`uid` IN ($ComId)","`shortname`,`uid`,`hy`,`mun`,`provinceid`,`cityid`,`three_cityid`, `r_status`");
		
		if($config[sy_datacycle_job]>0){
			    
		    $uptime =   strtotime('-'.$config[sy_datacycle_job].' day');
		    $JobList=   $db->select_all("company_job","`uid` IN ($ComId) and state=1 and r_status=1 and status=0 and lastupdate > $uptime $jobwheres","`id`,`uid`,`name`");
	    }else{
	    
	        $JobList=   $db->select_all("company_job","`uid` IN ($ComId) and state=1 and r_status=1 and status=0 $jobwheres","`id`,`uid`,`name`");
	    }
		
		$statis=$db->select_all("company_statis","`uid` IN ($ComId)","`uid`,`comtpl`");
		if(is_array($ListId)){
		    
		    foreach($hotjoblist as $key=>$value){
				foreach($comList as $v){
					if($v['uid'] == $value['uid']){
					    if($v['r_status'] != 1){ 
					        unset($hotjoblist[$key]);
					    }
					}
				}
			}
		    $JobIds =   array();
			//处理类别字段
			$cache_array = $db->cacheget();
			foreach($hotjoblist as $key=>$value){
				$hotjoblist[$key]["hot_pic"]=checkpic($value[hot_pic],$config[sy_unit_icon]);
				foreach($comList as $v){
				    
					if($value['uid']==$v['uid']){
						if($v['shortname']){
							$hotjoblist[$key]["username"]= $v[shortname];
						}
						$hotjoblist[$key]["hy"]= $cache_array[industry_name][$v[hy]];
						$hotjoblist[$key]["mun_n"]= $cache_array[comclass_name][$v[mun]];
						$hotjoblist[$key]["job_city_one"]= $cache_array[city_name][$v[provinceid]];
						$hotjoblist[$key]["job_city_two"]= $cache_array[city_name][$v[cityid]];
					}
				}
				$i=0;$j=0;
				$hotjoblist[$key]["num"]=0;
				if(is_array($JobList)){
					foreach($JobList as $ke=>$va){ 
						if($value[uid]==$va[uid]){
							if($j<3){
								$hotjob_url = Url("job",array("c"=>"comapply","id"=>"$va[id]"),"1");
								$curl=  Url("company",array("c"=>"show","id"=>$value[uid]));
								$va[name] = mb_substr($va[name],0,8,"utf-8");
								$hotjoblist[$key]["hotjob"].="<div class='index_mq_box_cont_showjoblist'><a href=\"$hotjob_url\">".$va[name]."</a></div>";
						        $JobIds[] = $va['id'];
							}else{
                                if($j==3){
                                    $hotjoblist[$key]["hotjob"].="<div class='index_mq_box_cont_showjobmore'><a href=\"$curl\">更多职位</a></div>";
							     }
							}
                            $j++;
						}
					}

					
					$hotjoblist[$key]["job"].="<div class=\"area_left\"> ";
					
					foreach($JobList as $k=>$v){
						if($value[uid]==$v[uid] && $i<5){
							$job_url = Url("job",array("c"=>"comapply","id"=>"$v[id]"),"1");
							$v[name] = mb_substr($v[name],0,10,"utf-8");
							$hotjoblist[$key]["job"].="<a href='".$job_url."'>".$v[name]."</a>";
							$i++;
						}
						if($value[uid]==$v[uid]){
							$hotjoblist[$key]["num"]=$hotjoblist[$key]["num"]+1;
						}
					}

					foreach($statis as $v){
						if($value['uid']==$v['uid']){
							if($v['comtpl'] && $v['comtpl']!="default"){
								$jobs_url = Url("company",array("c"=>"show","id"=>$value[uid]))."#job";
							}else{
								$jobs_url = Url("company",array("c"=>"show","id"=>$value[uid]));
							}
						}
					}
					$com_url = Url("company",array("c"=>"show","id"=>$value[uid]));
					$hotjoblist[$key]["job"].="</div><div class=\"area_right\"><a href='".$com_url."'>".$value["username"]."</a></div>";
					$hotjoblist[$key]["url"]=$com_url;
				}
			}
			if(!empty($JobIds)){
			    //$db -> update_all("company_job", "`jobexpoure` = `jobexpoure` + 1", "`id` IN (".@implode(',',$JobIds).")");
			}
		}$hotjoblist = $hotjoblist; if (!is_array($hotjoblist) && !is_object($hotjoblist)) { settype($hotjoblist, 'array');}
foreach ($hotjoblist as $_smarty_tpl->tpl_vars['hotjoblist']->key => $_smarty_tpl->tpl_vars['hotjoblist']->value) {
$_smarty_tpl->tpl_vars['hotjoblist']->_loop = true;
?>
                <div class="tlogo">
                    <ul>
                        <li onmouseover="showDiv2(this)" onmouseout="showDiv2(this)">
                            <div class="index_mq_box_pic">
                                <a href="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['url'];?>
" target="_blank" class="tlogo_p_a" title="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['username'];?>
">
                                    <img class="on lazy" src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['hot_pic'];?>
" alt="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['username'];?>
"/>
                                </a>
                            </div>
                            <div class="index_mq_box_name nowrap"><?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['username'];?>
</div>
                            <div class="index_mq_box_info"><?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['mun_n'];?>
<i class="index_newjob_info_line">|</i><?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['hy'];?>
</div>
                            <div class="index_mq_box_hot"><span class="index_mq_box_hot_n"><?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['num'];?>
</span>个热招职位</div>
                            <div class="index_mq_box_cont_showall">
                                <div class="index_mq_box_cont_showall_c">
                                    <div class="index_mq_box_cont_bg"></div>
                                    <div class="index_mq_box_cont_showjob">
                                        <div class="index_mq_box_cont_showjob_c">
                                            <div class="index_mq_box_cont_showcomname">
                                                <a href="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['url'];?>
" target="_blank" title="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['username'];?>
"><?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['username'];?>
</a>
                                            </div>
                                            <div class="index_mq_box_cont_showcomname_linebox">
                                                <i class="index_mq_box_cont_showcomname_line"></i>
                                            </div>
                                            <?php if ($_smarty_tpl->tpl_vars['hotjoblist']->value['num']>0) {?>
                                                <?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['hotjob'];?>

                                            <?php } else { ?>
                                                <div class="index_mq_box_cont_showjobmore"><a>暂无招聘职位</a></div>
                                            <?php }?>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </li>
                    </ul>
                </div>
                <?php } ?>
            </div>
        </div>
        <div class="yunheader_60lookmore"><a href="<?php echo smarty_function_listurl(array('m'=>'company','rec'=>1),$_smarty_tpl);?>
">查看更多</a></div>
    </div>

    <div class="index_zl_box">
        <div class="yunheader_60_tit"><a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'search'),$_smarty_tpl);?>
" target="_blank" class="yunheader_60_tit_a"><i class="yunheader_60_tit_line"></i>推荐职位<i class="yunheader_60_tit_rline"></i></a></div>
        <!-- 推荐职位 -->
        <div class="index_newjobbox index_zw_item">
            <ul>
                <?php  $_smarty_tpl->tpl_vars['job_list'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['job_list']->_loop = false;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","namelen"=>"9","rec"=>"1","comlen"=>"18","limit"=>"32","item"=>"“job_list“","name"=>"“job_list1“","nocache"=>"")
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
		
		$job_list = $db->select_all("company_job",$where.$limit);

		if(is_array($job_list) && !empty($job_list)){
			$comuid=$jobid=array();
			foreach($job_list as $key=>$value){
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
			foreach($job_list as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($job_list[$key]);
					continue;
				}else{
					$job_list[$key] = $db->array_action($value,$cache_array);
					$job_list[$key][stime] = date("Y-m-d",$value[sdate]);
					$job_list[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$job_list[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$job_list[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$job_list[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$job_list[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $job_list[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $job_list[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $job_list[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $job_list[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $job_list[$key][job_number] = "";
					}else{
					    $job_list[$key][job_number] = $value[zp_num]." 人";
					}			
                    $job_list[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $job_list[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $job_list[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$job_list[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$job_list[$key][pr_n] = $comclass_name[$value[pr]];
					$job_list[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$job_list[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$job_list[$key]['time'] = $job_list[$key]['lastupdate'];
						$job_list[$key]['redtime'] =1;
					}else{
						$job_list[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$job_list[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$job_list[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$job_list[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$job_list[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$job_list[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$job_list[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$job_list[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$job_list[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$job_list[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$job_list[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$job_list[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$job_list[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$job_list[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$job_list[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$job_list[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$job_list[$key][name_n]);
						$job_list[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$job_list[$key][com_n]);
						$job_list[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$job_list[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $job_list[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $job_list[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($job_list)){
				if($paramer[keyword]!=""&&!empty($job_list)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$job_list = $job_list; if (!is_array($job_list) && !is_object($job_list)) { settype($job_list, 'array');}
foreach ($job_list as $_smarty_tpl->tpl_vars['job_list']->key => $_smarty_tpl->tpl_vars['job_list']->value) {
$_smarty_tpl->tpl_vars['job_list']->_loop = true;
?>
                <li>
                    <div class="index_newjobname">
                        <a href="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['name'];?>
"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['name_n'];?>
</a>
                        <span class="index_newjob_info_xz"><?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_salary']!='面议') {
}
echo $_smarty_tpl->tpl_vars['job_list']->value['job_salary'];?>
</span>
                    </div>

                    <div class="index_newjob_info nowrap">
                        <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_city_two'];?>

                        <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_exp']) {?>
                            <i class="index_newjob_info_line">|</i><?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_exp'];?>
经验
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_edu']) {?>
                            <i class="index_newjob_info_line">|</i> <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_edu'];?>
学历
                        <?php }?>
                    </div>
                    <div class="index_newjob_com nowrap">
                        <img src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['logo'];?>
" class="index_newjob_com_tx"/>
                        <div class="index_newjob_comname">
                            <a href="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['com_url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['com_n'];?>
"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['com_n'];?>
</a>
                            <?php if ($_smarty_tpl->tpl_vars['job_list']->value['ratlogo']) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['ratlogo'];?>
" alt="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['ratname'];?>
" width="14" height="14"/>
                            <?php }?>
                            <?php if ($_smarty_tpl->tpl_vars['job_list']->value['yyzz_status']==1) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/disc_icon10.png" alt="认证企业" width="14" height="14"/>
                            <?php }?>
                        </div>
                        <div class="index_newjob_cominfo"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['hy_n'];?>
<i class="index_newjob_info_line">|</i> <?php echo $_smarty_tpl->tpl_vars['job_list']->value['mun_n'];?>
</div>
                    </div>
                </li>
                <?php } ?>
            </ul>
        </div>
        <div class="yunheader_60lookmore"><a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'search'),$_smarty_tpl);?>
">查看更多</a></div>
        <div class="yunheader_60_tit"><a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'search'),$_smarty_tpl);?>
" target="_blank" class="yunheader_60_tit_a"><i class="yunheader_60_tit_line"></i>最新职位<i class="yunheader_60_tit_rline"></i></a></div>

        <!-- 最新职位 -->
        <div class="index_newjobbox index_zw_item">
            <ul>
                <?php  $_smarty_tpl->tpl_vars['job_list'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['job_list']->_loop = false;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","namelen"=>"9","comlen"=>"18","limit"=>"32","item"=>"“job_list“","name"=>"“job_list1“","nocache"=>"")
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
		
		$job_list = $db->select_all("company_job",$where.$limit);

		if(is_array($job_list) && !empty($job_list)){
			$comuid=$jobid=array();
			foreach($job_list as $key=>$value){
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
			foreach($job_list as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($job_list[$key]);
					continue;
				}else{
					$job_list[$key] = $db->array_action($value,$cache_array);
					$job_list[$key][stime] = date("Y-m-d",$value[sdate]);
					$job_list[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$job_list[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$job_list[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$job_list[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$job_list[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $job_list[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $job_list[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $job_list[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $job_list[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $job_list[$key][job_number] = "";
					}else{
					    $job_list[$key][job_number] = $value[zp_num]." 人";
					}			
                    $job_list[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $job_list[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $job_list[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$job_list[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$job_list[$key][pr_n] = $comclass_name[$value[pr]];
					$job_list[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$job_list[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$job_list[$key]['time'] = $job_list[$key]['lastupdate'];
						$job_list[$key]['redtime'] =1;
					}else{
						$job_list[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$job_list[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$job_list[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$job_list[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$job_list[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$job_list[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$job_list[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$job_list[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$job_list[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$job_list[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$job_list[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$job_list[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$job_list[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$job_list[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$job_list[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$job_list[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$job_list[$key][name_n]);
						$job_list[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$job_list[$key][com_n]);
						$job_list[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$job_list[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $job_list[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $job_list[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($job_list)){
				if($paramer[keyword]!=""&&!empty($job_list)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$job_list = $job_list; if (!is_array($job_list) && !is_object($job_list)) { settype($job_list, 'array');}
foreach ($job_list as $_smarty_tpl->tpl_vars['job_list']->key => $_smarty_tpl->tpl_vars['job_list']->value) {
$_smarty_tpl->tpl_vars['job_list']->_loop = true;
?>
                <li>
                    <div class="index_newjobname">
                        <a href="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['name'];?>
"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['name_n'];?>
</a>
                        <span class="index_newjob_info_xz"><?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_salary']!='面议') {
}
echo $_smarty_tpl->tpl_vars['job_list']->value['job_salary'];?>
</span>
                    </div>
                    <div class="index_newjob_info nowrap">
                        <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_city_two'];?>

                        <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_exp']) {?>
                            <i class="index_newjob_info_line">|</i><?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_exp'];?>
经验
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_edu']) {?>
                            <i class="index_newjob_info_line">|</i> <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_edu'];?>
学历
                        <?php }?>
                    </div>
                    <div class="index_newjob_com nowrap">
                        <img src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['logo'];?>
" class="index_newjob_com_tx"/>
                        <div class="index_newjob_comname"><a href="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['com_url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['com_n'];?>
"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['com_n'];?>
</a>
                            <?php if ($_smarty_tpl->tpl_vars['job_list']->value['ratlogo']) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['ratlogo'];?>
" alt="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['ratname'];?>
" width="14" height="14"/>
                            <?php }?>
                            <?php if ($_smarty_tpl->tpl_vars['job_list']->value['yyzz_status']==1) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/disc_icon10.png" alt="认证企业" width="14" height="14"/>
                            <?php }?>
                        </div>
                        <div class="index_newjob_cominfo"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['hy_n'];?>
<i class="index_newjob_info_line">|</i> <?php echo $_smarty_tpl->tpl_vars['job_list']->value['mun_n'];?>
</div>
                    </div>
                </li>
                <?php } ?>
            </ul>
        </div>
        <div class="yunheader_60lookmore"><a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'search'),$_smarty_tpl);?>
">查看更多</a></div>
    </div>

    <!-- 热招人才 banner 区域-->
    <div class="index_banner fl">
        <div class="index_banner_1250 fl">
            <?php  $_smarty_tpl->tpl_vars['adlist_92'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['adlist_92']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"92","limit"=>"5","item"=>"“adlist_92“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[92];if(is_array($add_arr) && !empty($add_arr)){
				$i=0;$limit = 5;$length = 0;
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['adlist_92']->key => $_smarty_tpl->tpl_vars['adlist_92']->value) {
$_smarty_tpl->tpl_vars['adlist_92']->_loop = true;
?>
            <div class="b_w1200 b_tip"><?php echo $_smarty_tpl->tpl_vars['adlist_92']->value['lay_html'];?>
</div>
            <?php } ?>
        </div>
    </div>

    <!-- 最新简历 -->
    <div class="index_zl_box">
        <div class="yunheader_60_tit"><a href="<?php echo smarty_function_url(array('m'=>'resume','c'=>'search'),$_smarty_tpl);?>
" target="_blank" class="yunheader_60_tit_a"><i class="yunheader_60_tit_line"></i>推荐人才<i class="yunheader_60_tit_rline"></i></a></div>
        <!-- 推荐人才 -->
        <div class="tjuser_list">
            <ul>
                <?php  $_smarty_tpl->tpl_vars['ulist_rec'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['ulist_rec']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
$ulist_rec=array();global $db,$db_config,$config;
		if(is_array($_GET)){
			foreach($_GET as $key=>$value){
				if($value=='0'){
					unset($_GET[$key]);
				}
			}
		}
		$paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","item"=>"“ulist_rec“","post_len"=>"10","limit"=>"8","recg"=>"1","key"=>"“key“","name"=>"“userlist2“","nocache"=>"")
;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		$Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }

	    //处理类别字段
	    include(CONFIG_PATH."db.data.php");	
		$cache_array    = $db->cacheget();
        $fscache_array  = $db->fscacheget();
        $userdata       = $cache_array["userdata"];
		$userclass_name = $cache_array["user_classname"];
		$city_name      = $cache_array["city_name"];
        $city_index     = $cache_array["city_index"];
		$job_name		= $cache_array["job_name"];
        $job_index		= $cache_array["job_index"];
		$job_type		= $cache_array["job_type"];
		$industry_name	= $cache_array["industry_name"];
        $city_parent    = $fscache_array["city_parent"];
        $job_parent     = $fscache_array["job_parent"];

		//是否属于分站下
		if($config['sy_web_site']=="1"){
			if($config[province]>0 && $config[province]!=""){
				$paramer[provinceid] = $config[province];
			}
			if($config['cityid']>0 && $config['cityid']!=""){
				$paramer['cityid']=$config['cityid'];
			}
			if($config['three_cityid']>0 && $config['three_cityid']!=""){
				$paramer['three_cityid']=$config['three_cityid'];
			}
			if($config['hyclass']>0 && $config['hyclass']!=""){
				$paramer['hy']=$config['hyclass'];
			}
		}

		
		$where = "a.`defaults`=1 and a.`state`=1 and a.`r_status`=1 AND a.`status`=1";

        //关注我公司的人才--条件
		if($paramer[where_uid]){
			$where .=" AND a.`uid` in (".$paramer['where_uid'].")";
		}
		//黑名单不能查看已拉黑的个人用户简历
		if($_COOKIE['uid']&&$_COOKIE['usertype']=="2"){
			$blacklist=$db->select_all("blacklist","`p_uid`='".$_COOKIE['uid']."'","c_uid");
			if(is_array($blacklist)&&$blacklist){
				foreach($blacklist as $v){
					$buid[]=$v['c_uid'];
				}
			    $where .=" AND a.`uid` NOT IN (".@implode(",",$buid).")";
			}
		}

        //置顶
		if($paramer[topdate]){
			$where .=" AND a.`top`=1 AND a.`topdate`>'".time()."'";
		}
		if($paramer[top]){
			$where .=" AND a.`top`=1 AND a.`topdate`>'".time()."'";
		}
        //身份认证
		if($paramer[idcard]){
			$where .=" AND a.`idcard_status`=1";
		}
		//优质人才推荐
		if($paramer[rec]){
			$where .=" AND a.`rec`=1";
		}
		//普通推荐
		if($paramer[recg]){
			$where .=" AND a.`rec_resume`=1";
		}
		//作品
		if($paramer[work]){
			$show=$db->select_all("resume_show","1 group by eid","`eid`");
			if(is_array($show)){
				foreach($show as $v){
					$eid[]=$v['eid'];
				}
			}
			$where .=" AND a.`id` in (".@implode(",",$eid).")";
		}
		//标签
		if($paramer[tag]){
		    $tagname=$userclass_name[$paramer[tag]];
			$oneYear    =   strtotime("-1 year");
			$tag=$db->select_all("resume","`def_job`>0 and `r_status`=1 and `status`=1 and `lastupdate` > '".$oneYear."' and FIND_IN_SET('".$tagname."',`tag`)","`def_job`");
			if(is_array($tag)){
				foreach($tag as $v){
					$tagid[]=$v['def_job'];
				}
			}
			$where .=" AND a.`id` in (".@implode(",",$tagid).")";
		}
		//更新时间区间
		if($paramer[uptime]){
			if($paramer[uptime]==1){
				$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
    			$where.=" AND a.`lastupdate`>$beginToday";
    		}else{
    			$time=time();
				$uptime = $time-($paramer[uptime]*86400);
				$where.=" AND a.`lastupdate`>$uptime";
    		}
		}else{
		    if($config[sy_datacycle]>0){
                // 后台-页面设置-数据周期
		        $uptime = strtotime('-'.$config[sy_datacycle].' day');
				$where.=" AND a.`lastupdate`>$uptime";
		    }
		}		
		//添加时间区间，即审核时间
		if($paramer[adtime]){
			$time=time();
			$adtime = $time-($paramer[adtime]*86400);
			$where.=" AND a.`status_time`>$adtime";
		}
		//是否有照片
		if($paramer[pic]=="1"){
			$where .=" AND a.`photo`<>'' AND a.`phototype`!=1 AND a.`defphoto`= 1";
		}
        //行业
		if($paramer['hy']!=""){
			$where .= " AND a.`hy` IN (".$paramer['hy'].")";
		}
        $job_col = $city_col = "";
        $cjwhere = "";
		if($paramer[threecityid]){
		    $city_col = "three_cityid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[threecityid]";
		}elseif($paramer[three_cityid]){
		    $city_col = "three_cityid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[three_cityid]";
		}elseif($paramer[cityid]){
		    $city_col = "cityid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[cityid]";
		}elseif($paramer[provinceid]){
		    $city_col = "provinceid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[provinceid]";
		}
        //城市区间,不建议执行该查询
		if($paramer[cityin]){
            if($city_parent[$paramer[cityin]]=='0'){
		        $city_col = "provinceid";
				$cjwhere .= " AND cj.`$city_col` = $paramer[cityin]";
			}elseif(in_array($city_parent[$paramer[cityin]],$city_index)){
		        $city_col = "cityid";
				$cjwhere .= " AND cj.`$city_col` = $paramer[cityin]";
			}elseif($city_parent[$paramer[cityin]]>0){
		        $city_col = "three_cityid";
				$cjwhere .= " AND cj.`$city_col` = $paramer[cityin]";
			}
		}
		
		if($paramer[jobpost]){
		    $job_col = "job_post";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[jobpost]";
		}elseif($paramer[job1son]){
		    $job_col = "job1_son";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job1son]";
		}elseif($paramer[job_post]){
		    $job_col = "job_post";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job_post]";
		}elseif($paramer[job1_son]){
		    $job_col = "job1_son";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job1_son]";
		}elseif($paramer[job1]){
		    $job_col = "job1";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job1]";
		}
        //职位区间,不建议执行该查询
		if($paramer[jobin]){
            if($job_parent[$paramer[jobin]]=='0'){
		        $job_col = "job1";
				$cjwhere .=" AND cj.`$job_col`= $paramer[jobin]";
			}elseif(in_array($job_parent[$paramer[jobin]],$job_index)){
		        $job_col = "job1_son";
				$cjwhere .=" AND cj.`$job_col`= $paramer[jobin]";
			}elseif($job_parent[$paramer[jobin]]>0){
		        $job_col = "job_post";
				$cjwhere .=" AND cj.`$job_col`= $paramer[jobin]";
			}
		}
		// 拼接唯一标识字段
		if($city_col || $job_col){
		    if($city_col && $job_col){
		        $cjwhere .= " AND cj.`{$city_col}_{$job_col}_num`= 1";
		    }elseif($city_col){
		        $cjwhere .= " AND cj.`{$city_col}_num`= 1";
		    }elseif($job_col){
		        $cjwhere .= " AND cj.`{$job_col}_num`= 1";
		    }
		}
		
		//工作经验
		if($paramer[exp]){
		    if (stripos($userclass_name[$paramer[exp]], "应届生") !== false){
                // 应届生特殊，只搜应届生
                $where .= " AND a.`exp`=".$paramer[exp];
            }else{
                $expArr  = $userdata['user_word'];
    			$expSort = 0;
    			$expIds  = array();
    			// 简历搜索，排序比搜索大的都符合条件。如搜“五年”，类别排序大于等于“五年”排序的（要排除不限）都符合
                foreach ($expArr as $k => $v) {
                    if ($v == $paramer[exp] && $userclass_name[$v] != "不限"){
                        $expSort = $k;
                        break;
                    }
                }
                foreach ($expArr as $k => $v) {
                    if ($k >= $expSort && $userclass_name[$v] != "不限"){
                        $expIds[] = $v;
                    }
                }
                if (!empty($expIds)) {
                	$where .= " AND a.`exp` in (".@implode(",",$expIds).")";
                }
            }
		}
		//学历
		if($paramer[edu]){

		    $eduArr  = $userdata['user_edu'];
			$eduSort = 0;
			$eduIds  = array();
			// 简历搜索，排序比搜索大的都符合条件。如搜“硕士”，类别排序大于等于“硕士”排序的（要排除不限）都符合
			foreach ($eduArr as $k => $v) {
			    if ($v == $paramer[edu] && $userclass_name[$v] != "不限"){
			        $eduSort = $k;
                    break;
			    }
			}
			foreach ($eduArr as $k => $v) {
			    if ($k >= $eduSort && $userclass_name[$v] != "不限"){
			        $eduIds[] = $v;
			    }
			}
            if (!empty($eduIds)) {
            	$where .= " AND a.`edu` in (".@implode(",",$eduIds).")";
            }
		}
		//性别
		if($paramer[sex]){
			$where .=" AND a.`sex`=$paramer[sex]";
		}
		//到岗时间
		if($paramer[report]){
			$where .=" AND a.`report`=$paramer[report]";
		}
		//工作性质
		if($paramer[type]){
			$where .= " AND a.`type`=$paramer[type]";
		}
		if($paramer[notid]){
			$where.= " and `id`<>$paramer[notid]";
		}
		//简历完整度
		
		if($paramer[integrity]){
		 	$integrityR	=	$arr_data["integrity_val"];
		 	
		 	$where.= " AND a.`integrity`>='".$integrityR[$paramer[integrity]]."'";

		}

		//关键字
		if($paramer[keyword]){
			$where1 = array();
			$where1[]="a.`name` LIKE '%$paramer[keyword]%'";
			$where1[]="a.`uname` LIKE '%$paramer[keyword]%'";
			
			//搜索工作经历 
			$workList = $db->select_all('resume_work',"`name` LIKE '%$paramer[keyword]%' OR `title` LIKE '%$paramer[keyword]%' OR `content` LIKE '%$paramer[keyword]%' ORDER BY id DESC limit 500","`eid`");
			if(!empty($workList)){
				$workId = array();
				foreach($workList as $value){
					$workId[] = $value['eid'];
				}
				$where1[]	=	"a.id IN (".implode(',',$workId).")";
			}
           
            $where.=" AND (".@implode(" or ",$where1).")";
		}
		//薪资待遇
		if($paramer[minsalary]&&$paramer[maxsalary]){
			$where.= " AND ((a.`minsalary`<=".intval($paramer[minsalary])." and a.`maxsalary`>=".intval($paramer[minsalary]).") 
						or (a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`>=".intval($paramer[maxsalary])."))";
		}elseif($paramer[minsalary]&&!$paramer[maxsalary]){
			$where.= " AND ((a.`minsalary`<=".intval($paramer[minsalary])." and a.`maxsalary`>=".intval($paramer[minsalary]).") 
						or (a.`minsalary`>=".intval($paramer[minsalary])." and a.`maxsalary`>=".intval($paramer[minsalary]).")
						or (a.`minsalary`!=0 and  a.`maxsalary`=0))";
		}elseif(!$paramer[minsalary]&&$paramer[maxsalary]){
			$where.= " AND ((a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`>=".intval($paramer[maxsalary]).")
						or (a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`<=".intval($paramer[maxsalary]).")  
						or (a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`=0) 
						or (a.`minsalary`=0 and a.`maxsalary`!=0)
						)";
		}
		//年龄
		if($paramer[minage]&&$paramer[maxage]){
			$mintime=date("Y-m-d",strtotime("-".$paramer[minage]." year"));
			$maxtime=date("Y-m-d",strtotime("-".$paramer[maxage]." year"));
			$where.= " AND a.`birthday`>= '".$maxtime."' and a.`birthday`<='".$mintime."'";
		}elseif($paramer[minage]&&!$paramer[maxage]){
			$mintime=date("Y-m-d",strtotime("-".$paramer[minage]." year"));
			$where.= " AND a.`birthday`<='".$mintime."'";
		}elseif(!$paramer[minage]&&$paramer[maxage]){
			$maxtime=date("Y-m-d",strtotime("-".$paramer[maxage]." year"));
			$where.= " AND a.`birthday`>='".$maxtime."'";
		}
        //排序字段默认为更新时间
		if($paramer[order] && $paramer[order]!="lastdate"){
			if($paramer[order]=='topdate'){
				$nowtime=time();
				$order = " ORDER BY if(a.`topdate`>$nowtime,a.`topdate`,a.`lastupdate`) ";
			}else{
				$order = " ORDER BY a.`".$paramer[order]."` ";
			}
		}else{
			$order = " ORDER BY a.`lastupdate` ";
		}
		//排序规则 默认为倒序
		$sort = $paramer[sort]?$paramer[sort]:"DESC";
		//查询条数
		if($paramer[limit]){
			$limit=" LIMIT ".$paramer[limit];
		}
		//自定义查询条件，默认取代上面任何参数直接使用该语句
		if($paramer[where]){
			$where = $paramer[where];
		}
        $pagewhere = "";$joinwhere = "";
        if($cjwhere){
            $pagewhere.=" ,`".$db_config[def]."resume_city_job_class`  cj ";
            $joinwhere .= " a.`id`=cj.`eid` " . $cjwhere;
        }
        
		if($paramer[ispage]){
            // 查询分页
			if($paramer["height_status"]){
				$limit = PageNav($paramer,$_GET,"resume_expect",$where,$Purl,"",$paramer[islt]?$paramer[islt]:"3",$_smarty_tpl,$pagewhere,$joinwhere);
			}else{
				$limit = PageNav($paramer,$_GET,"resume_expect",$where,$Purl,"","0",$_smarty_tpl,$pagewhere,$joinwhere);
			}
		}

		if($paramer[topdate] && $_GET[page]>1){
			$ulist_rec = array();
		}else{
			
			$select="a.`id`,a.`uid`,a.`name`,a.`hy`,a.`job_classid`,a.`city_classid`,a.`jobstatus`,a.`type`,a.`report`,a.`lastupdate`,a.`rec`,a.`top`,a.`topdate`,a.`rec_resume`,a.`ctime`,a.`uname`,a.`idcard_status`,a.`minsalary`,a.`maxsalary`";
			if($pagewhere!=""){

				$sql = "select ".$select." from `".$db_config[def]."resume_expect` a ".$pagewhere." where ".$joinwhere." and ".$where.$order.$sort.$limit;
				$ulist_rec=$db->DB_query_all($sql,"all");

			}else{
				$sql = "select ".$select." from `".$db_config[def]."resume_expect` a where ".$where.$order.$sort.$limit;
				$ulist_rec=$db->DB_query_all($sql,"all");
			}
		}
        
		if(!empty($ulist_rec) && is_array($ulist_rec)){
			
			//如果存在top，则说明请求来自排行榜页面
			if($paramer['top']){
				$uids=$m_name=array();
				foreach($ulist_rec as $k=>$v){
					$uids[]=$v[uid];
				}

				$member=$db->select_all($db_config[def]."member","`uid` in(".@implode(',',$uids).")","uid,username");
				foreach($member as $val){
					$m_name[$val[uid]]=$val['username'];
				}
			}
			$uid = $eid = array();
			foreach($ulist_rec as $key=>$value){
				
				$uid[] = $value['uid'];
				$eid[] = $value['id'];
			}
			$eids = @implode(',',$eid);
			$uids = @implode(',',$uid);
            $resume=$db->select_all("resume","`uid` in(".$uids.")","uid,name,nametype,tag,sex,moblie_status,edu,exp,photo,phototype,defphoto,photo_status,birthday");
			foreach($resume as $v){
				$ruids[] = $v['uid'];
			}
			foreach($ulist_rec as $k=>$v){
				if(!in_array($v['uid'],$ruids)){
					unset($ulist_rec[$k]);
					continue;
				}
			}
			if($paramer[topdate]){
				$noids=array();
			}

			if($paramer[workexp] == 1){
				$eduList  = $db -> select_all("resume_edu","`eid` in(".$eids.")");
				if(!empty($eduList)){
					foreach($eduList as $key=>$value){
						$eduListNew[$value['eid']][] = $value;
					}
					foreach($eduListNew as $k=>$eduv){
						$edumin				=		0;
						$edumax				=		0;
						$edutitle			=	array();
						$education			=	array();
						foreach($eduv as $v){
							if($v['sdate']>0 && $edumin==0){
								$edumin		=		$v['sdate'];
							}elseif($edumin>$v['sdate']){
								$edumin		=		$v['sdate'];
							}
							if($v['edate']==0 ){
								$edumax		=		0;
							}elseif($edumax<$v['edate']){
								$edumax		=		$v['edate'];
							}
						
							$education[]	=		$userclass_name[$v['education']];

							$edutitle[]		=		$v['specialty'];
						}
						$return =array();
						$return['edumin']		=		date('Y-m',$edumin);
						$return['edumax']		=		$edumax  == 0 ?  '至今': date('Y',$edumax);
						$return['education']	=		@implode(',',$education);
						$return['eduspecialty']		=		@implode('、',$edutitle);
						
						$return['edu_time']		=		$return['edumin']."-".$return['edumax'];
						
						if($return['eduspecialty']){
						    $workexpList[$k]['edu_content']	=	$return['education'].'学历 · '.$return['eduspecialty'].' · 毕业于'.$return['edumax'].'年';
						}else{
						    $workexpList[$k]['edu_content']	=	$return['education'].'学历 · 毕业于'.$return['edumax'].'年';
						}
					}
					
				}

				$workList  = $db -> select_all("resume_work","`eid` in(".$eids.")");
				if(!empty($workList)){
					foreach($workList as $key=>$value){
						$workListNew[$value['eid']][] = $value;
					}
					foreach($workListNew as $k=>$workv){
						
						$whour     		=   0;
						$hour     	 	=   array();
						$time      		=   time();
						$workmin		=	0;
						$workmax		=	0;
						$worknum		=	count($workv);
						$wtitle			=	array();
						foreach($workv as $v){
							/* 计算每份工作时长(按月) */
							
							if($v['sdate']>0 && $workmin==0){
								$workmin		=		$v['sdate'];
							}elseif($workmin>$v['sdate']){
								$workmin		=		$v['sdate'];
							}
							
							if($v['edate']==0 ){
								$workmax		=		0;
							}elseif($workmax<$v['edate']){
								$workmax		=		$v['edate'];
							}
			
							$wtitle[]			=		$v['title'];
							
							$hour[]   			=		$workTime;
							$whour    			+=   	$workTime;
						}
						
						$workavg   =   ceil($whour/count($hour));
						$return = array();
						$return['worknum']		=		$worknum  > 0 ?  $worknum:0;
						$return['workavg']		=		$workavg  > 0 ?  $workavg:0;
						$return['workmin']		=		date('Y-m',$workmin);
						$return['workmax']		=		$workmax  == 0 ?  '至今': date('Y-m',$workmax);
						$return['worktit']		=		@implode(',',$wtitle);
						
						$return['work_time']	=		$return['workmin'].'-'.$return['workmax'];
						
						if($return['worktit']!=''){
							$workexpList[$k]['work_content']	=	' 参加过'.$return['worknum'].'份工作 · 涉及'.$return['worktit'].'等岗位';
						}else{
							$workexpList[$k]['work_content']	=	' 参加过'.$return['worknum'].'份工作';
						}
					}
				}
			}
			if ($_COOKIE['uid'] && $_COOKIE['usertype']==2){
			    $lookResume =   $db->select_all("look_resume","`com_id` = ".$_COOKIE['uid'], "`resume_id`");
			    if (!empty($lookResume)){
			        foreach($lookResume as $key=>$value){
						$lookResumeId[] = $value['resume_id'];
					}
			    }
			    if($paramer[isrck] == 1){
                $downList = $freedownList = array();
                    $downList = $db->select_all('down_resume', "`comid` = ".$_COOKIE['uid'] . " and `uid` in (".$uids.")", "`comid`,`eid`");
                    $freedownList = $db->select_all('freedown_resume', "`comid` = ".$_COOKIE['uid'] . " and `uid` in (".$uids.")", '`comid`,`eid`');
			    $vipetime = 0;
			    $vip_etime = $db->select_all('company', "`uid` = ".$_COOKIE['uid'], "`vipetime`");
			    $vipetime = $vip_etime[0][vipetime] ? $vip_etime[0][vipetime] : 0;
			    $userid_msg = $db->select_all('userid_msg', "`fid` = ".$_COOKIE['uid'] . " and `uid` in (".$uids.") and `isdel` = 9",'`uid`');
			    }
			}
			foreach($ulist_rec as $k=>$v){
				if($paramer[topdate]){
					$noids[] = $v[id];
				}
				//筛除重复
				if($paramer[noid]=='1' && !empty($noids) && in_array($v['id'],$noids)){
					unset($ulist_rec[$k]);
					continue;
				}
			    foreach($resume as $val){
			        if($v['uid']==$val['uid']){
			    		$ulist_rec[$k]['edu_n']=$userclass_name[$val['edu']];
				        $ulist_rec[$k]['exp_n']=$userclass_name[$val['exp']];
			            if($val['birthday']){
							
							$ageArr = explode('-',$val['birthday']);
						
							if(count($ageArr)==3){// 原来数据生日格式为年月日，如果是旧格式，改为年月
								
								$val['birthday'] = $ageArr[0].'-'.$ageArr[1];
								
							}
							
							$year = date("Y",strtotime($val['birthday']));
							$ulist_rec[$k]['age'] =date("Y")-$year;
						}
						if($val['sex']==152){
							$val['sex']='1';
						}elseif ($val['sex']==153){
							$val['sex']='2';
						}
						$ulist_rec[$k]['sex'] =$arr_data[sex][$val['sex']];
		                $ulist_rec[$k]['phototype']=$val[phototype];
		                $photo=$icon="";
		                if($val['defphoto']==2){
		                	$photo=$val['photo'];
		                }else{
							if($config['user_pic']==1 || empty($config['user_pic'])){
				                if($val['photo'] && $val['photo_status']==0 && $val['phototype']!=1){
		            				$photo=$val['photo'];
		            			}else{
		            				if($val['sex']==1){
		            					$icon=$config['sy_member_icon'];
		            				}else{
		            					$icon=$config['sy_member_iconv'];
		            				}
		            			}
		            			
							}elseif($config['user_pic']==2){
								if($val['photo']&& $val['photo_status']==0){
									$photo=$val['photo'];
								}else{
									if($val['sex']==1){
										$icon=$config['sy_member_icon'];
									}else{
										$icon=$config['sy_member_iconv'];
									}
								}
							}elseif($config['user_pic']==3){
								if($val['sex']==1){
									$icon=$config['sy_member_icon'];
								}else{
									$icon=$config['sy_member_iconv'];
								}
							}
						}
						$ulist_rec[$k]['photo']=checkpic($photo,$icon);
						if($val['tag']){
                            $ulist_rec[$k]['tag']=explode(',',$val['tag']);
	                    }
                        $ulist_rec[$k]['nametype']=$val[nametype];
                        $ulist_rec[$k]['moblie_status']=$val[moblie_status];
                        //名称显示处理
						if($config['user_name']==1 || !$config['user_name']){
    						if($val['nametype']==3){
    						    if($val['sex']==1){
    						        $ulist_rec[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."先生";
    						    }else{
    						        $ulist_rec[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."女士";
    						    }
    						}elseif($val['nametype']==2){
    						    $ulist_rec[$k]['username_n'] = "NO.".$v['id'];
    						}else{
    							$ulist_rec[$k]['username_n'] = $val['name'];
    						}
						}elseif($config['user_name']==3){
							if($val['sex']==1){
								$ulist_rec[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."先生";
							}else{
								$ulist_rec[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."女士";
							}
						}elseif($config['user_name']==2){
							$ulist_rec[$k]['username_n'] = "NO.".$v['id'];
						}elseif($config['user_name']==4){
							$ulist_rec[$k]['username_n'] = $val['name'];
						}
                    }
                }
				//更新时间显示方式
				$time=$v['lastupdate'];
				//今天开始时间戳
				$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
				//昨天开始时间戳
				$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
				if($time>$beginYesterday && $time<$beginToday){
					$ulist_rec[$k]['time'] = "昨天";
				}elseif($time>$beginToday){
					$ulist_rec[$k]['time'] = lastupdateStyle($v['lastupdate']);
					$ulist_rec[$k]['redtime'] =1;
				}else{
					$ulist_rec[$k]['time'] = date("Y-m-d",$v['lastupdate']);
				} 
                
                // 前天
				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

				if($v['ctime']>$beforeYesterday){
					$ulist_rec[$k]['newtime'] =1;
				}
				$ulist_rec[$k]['user_jobstatus_n']=$userclass_name[$v['jobstatus']];
// 				$ulist_rec[$k]['job_city_one']=$city_name[$v['provinceid']];
// 				$ulist_rec[$k]['job_city_two']=$city_name[$v['cityid']];
// 				$ulist_rec[$k]['job_city_three']=$city_name[$v['three_cityid']];
                $ulist_rec[$k]['salary_n'] = salaryUnit($v['minsalary'], $v['maxsalary']);
				$ulist_rec[$k]['report_n']=$userclass_name[$v['report']];
				$ulist_rec[$k]['type_n']=$userclass_name[$v['type']];
				$ulist_rec[$k]['lastupdate']=date("Y-m-d",$v['lastupdate']);
					
				$ulist_rec[$k]['user_url']=Url("resume",array("c"=>"show","id"=>$v['id']),"1");
				$ulist_rec[$k]["hy_info"]=$industry_name[$v['hy']];
				if($paramer['top']){
					$ulist_rec[$k]['m_name']=$m_name[$v['uid']];
					$ulist_rec[$k]['user_url']=Url("ask",array("c"=>"friend","a"=>"myquestion","uid"=>$v['uid']));
				}
				$ulist_rec[$k]['work_content']=$workexpList[$v['id']]['work_content'];
				$ulist_rec[$k]['edu_content']=$workexpList[$v['id']]['edu_content'];

				$kjob_classid=@explode(",",$v['job_classid']);
				$kjob_classid=array_unique($kjob_classid);	
				$jobname=array();
				if(is_array($kjob_classid)){
					foreach($kjob_classid as $val){
					    if($val!=''){
					        if($paramer['keyword']){
                               $jobname[]=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$job_name[$val]);
                            }else{
                               $jobname[]=$job_name[$val];
                            }
                        }
					}
				}
				//$ulist_rec[$k]['job_post']=@implode(",",$jobname);
				$ulist_rec[$k]['expectjob']=$jobname;
				$kcity_classid=@explode(",",$v['city_classid']);
				$kcity_classid=array_unique($kcity_classid);	
				$cityname=array();
				if(is_array($kcity_classid)){
					foreach($kcity_classid as $val){
					    if($val!=''){
					       
                            $cityname[]=$city_name[$val];
                            
                        }
					}
				}
                //$ulist_rec[$k]['citylist']=@implode("/",$cityname);
				$ulist_rec[$k]['expectcity']=$cityname;
				//截取标题
				if($paramer['post_len']){
					$postname[$k]=@implode(",",$jobname);
					$ulist_rec[$k]['job_post_n']=mb_substr($postname[$k],0,$paramer[post_len],"utf-8");
				}
                if($paramer['city_len']){
					$scityname[$k]=@implode("/",$cityname);
					$ulist_rec[$k]['city_name_n']=mb_substr($scityname[$k],0,$paramer[city_len],"utf-8");
				}
				//  是否浏览过
				$ulist_rec[$k]['isLookEd'] = 0;
				if(in_array($v['id'], $lookResumeId)){
				    $ulist_rec[$k]['isLookEd'] = 1;
				}
				if($paramer[isrck] == 1){
				//  是否已下载
				$ulist_rec[$k]['islink'] = 0;
				foreach ($downList as $dv) {
                    if ($v['id'] == $dv['eid']) {
                        $ulist_rec[$k]['islink'] = 1;
                    }
                }
                foreach ($freedownList as $dv) {
                    if ($v['id'] == $dv['eid']) {
                        if ($vipetime) {
                            $ulist_rec[$k]['islink'] = 1;
                        }else{
                            $ulist_rec[$k]['islink'] = 0;
                        }
                    }
                }
                //  是否已邀约
                $ulist_rec[$k]['userid_msg'] = 0;
                foreach($userid_msg as $val){
                    if($v['uid']==$val['uid']){
                        $ulist_rec[$k]['userid_msg'] = 1;
                        }
                    }
                }
			}
			foreach($ulist_rec as $k=>$v){
               if($paramer['keyword']){
					$ulist_rec[$k]['username_n']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$v['username_n']);
					$ulist_rec[$k]['job_post']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$ulist_rec[$k]['job_post']);
					$ulist_rec[$k]['job_post_n']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$ulist_rec[$k]['job_post_n']);
					$ulist_rec[$k]['city_name_n']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$ulist_rec[$k]['city_name_n']);
				}
            }

			
			if($paramer['keyword']!=""&&!empty($ulist_rec)){
				addkeywords('5',$paramer['keyword']);
			}
		}$ulist_rec = $ulist_rec; if (!is_array($ulist_rec) && !is_object($ulist_rec)) { settype($ulist_rec, 'array');}
foreach ($ulist_rec as $_smarty_tpl->tpl_vars['ulist_rec']->key => $_smarty_tpl->tpl_vars['ulist_rec']->value) {
$_smarty_tpl->tpl_vars['ulist_rec']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['ulist_rec']->key;
?>
                <li>
                    <div class="tjuser_photo"><img src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['photo'];?>
" width="80" height="80"></div>
                    <div class="tjuser_name">
                        <a <?php if ($_smarty_tpl->tpl_vars['config']->value['com_search']==1&&!$_smarty_tpl->tpl_vars['uid']->value) {?> href="<?php echo smarty_function_url(array('m'=>'login'),$_smarty_tpl);?>
" <?php } elseif ($_smarty_tpl->tpl_vars['usertype']->value==1) {
if ($_smarty_tpl->tpl_vars['config']->value['sy_user_change']==1) {?>onclick="layer.msg('请先申请企业用户', 2, 8)" <?php } else { ?>onclick="layer.msg('只有企业用户才能查看', 2, 8)"<?php }?> <?php } else { ?>href="<?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['user_url'];?>
" target="_blank" <?php }?>>
                        <?php if (in_array($_smarty_tpl->tpl_vars['ulist_rec']->value['id'],$_smarty_tpl->tpl_vars['eid']->value)) {?> <?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['uname'];?>
 <?php } else { ?> <?php echo mb_substr($_smarty_tpl->tpl_vars['ulist_rec']->value['username_n'],0,10,"utf-8");?>
 <?php }?>
                        </a>
                    </div>
                    <div class="index_tjresume_user"></div>
                    <div class="tjuser_nameinfo">
                        <?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['age'];?>
岁<i class="index_resume_userinfo_line">|</i><?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['exp_n'];?>
经验<i class="index_resume_userinfo_line">|</i><?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['edu_n'];?>
学历
                    </div>
                    <div class="tjuser_yx">
                        意向：<span class="index_resume_useryx_n"><?php echo $_smarty_tpl->tpl_vars['ulist_rec']->value['job_post_n'];?>
</span>
                    </div>
                </li>
                <?php } ?>
            </ul>
        </div>

        <!-- 最新人才 -->
        <div class="index_resume_user_list index_zw_item">
            <ul>
                <?php  $_smarty_tpl->tpl_vars['ulist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['ulist']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
$ulist=array();global $db,$db_config,$config;
		if(is_array($_GET)){
			foreach($_GET as $key=>$value){
				if($value=='0'){
					unset($_GET[$key]);
				}
			}
		}
		$paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","item"=>"“ulist“","post_len"=>"10","limit"=>"16","key"=>"“key“","name"=>"“userlist1“","nocache"=>"")
;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		$Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }

	    //处理类别字段
	    include(CONFIG_PATH."db.data.php");	
		$cache_array    = $db->cacheget();
        $fscache_array  = $db->fscacheget();
        $userdata       = $cache_array["userdata"];
		$userclass_name = $cache_array["user_classname"];
		$city_name      = $cache_array["city_name"];
        $city_index     = $cache_array["city_index"];
		$job_name		= $cache_array["job_name"];
        $job_index		= $cache_array["job_index"];
		$job_type		= $cache_array["job_type"];
		$industry_name	= $cache_array["industry_name"];
        $city_parent    = $fscache_array["city_parent"];
        $job_parent     = $fscache_array["job_parent"];

		//是否属于分站下
		if($config['sy_web_site']=="1"){
			if($config[province]>0 && $config[province]!=""){
				$paramer[provinceid] = $config[province];
			}
			if($config['cityid']>0 && $config['cityid']!=""){
				$paramer['cityid']=$config['cityid'];
			}
			if($config['three_cityid']>0 && $config['three_cityid']!=""){
				$paramer['three_cityid']=$config['three_cityid'];
			}
			if($config['hyclass']>0 && $config['hyclass']!=""){
				$paramer['hy']=$config['hyclass'];
			}
		}

		
		$where = "a.`defaults`=1 and a.`state`=1 and a.`r_status`=1 AND a.`status`=1";

        //关注我公司的人才--条件
		if($paramer[where_uid]){
			$where .=" AND a.`uid` in (".$paramer['where_uid'].")";
		}
		//黑名单不能查看已拉黑的个人用户简历
		if($_COOKIE['uid']&&$_COOKIE['usertype']=="2"){
			$blacklist=$db->select_all("blacklist","`p_uid`='".$_COOKIE['uid']."'","c_uid");
			if(is_array($blacklist)&&$blacklist){
				foreach($blacklist as $v){
					$buid[]=$v['c_uid'];
				}
			    $where .=" AND a.`uid` NOT IN (".@implode(",",$buid).")";
			}
		}

        //置顶
		if($paramer[topdate]){
			$where .=" AND a.`top`=1 AND a.`topdate`>'".time()."'";
		}
		if($paramer[top]){
			$where .=" AND a.`top`=1 AND a.`topdate`>'".time()."'";
		}
        //身份认证
		if($paramer[idcard]){
			$where .=" AND a.`idcard_status`=1";
		}
		//优质人才推荐
		if($paramer[rec]){
			$where .=" AND a.`rec`=1";
		}
		//普通推荐
		if($paramer[recg]){
			$where .=" AND a.`rec_resume`=1";
		}
		//作品
		if($paramer[work]){
			$show=$db->select_all("resume_show","1 group by eid","`eid`");
			if(is_array($show)){
				foreach($show as $v){
					$eid[]=$v['eid'];
				}
			}
			$where .=" AND a.`id` in (".@implode(",",$eid).")";
		}
		//标签
		if($paramer[tag]){
		    $tagname=$userclass_name[$paramer[tag]];
			$oneYear    =   strtotime("-1 year");
			$tag=$db->select_all("resume","`def_job`>0 and `r_status`=1 and `status`=1 and `lastupdate` > '".$oneYear."' and FIND_IN_SET('".$tagname."',`tag`)","`def_job`");
			if(is_array($tag)){
				foreach($tag as $v){
					$tagid[]=$v['def_job'];
				}
			}
			$where .=" AND a.`id` in (".@implode(",",$tagid).")";
		}
		//更新时间区间
		if($paramer[uptime]){
			if($paramer[uptime]==1){
				$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
    			$where.=" AND a.`lastupdate`>$beginToday";
    		}else{
    			$time=time();
				$uptime = $time-($paramer[uptime]*86400);
				$where.=" AND a.`lastupdate`>$uptime";
    		}
		}else{
		    if($config[sy_datacycle]>0){
                // 后台-页面设置-数据周期
		        $uptime = strtotime('-'.$config[sy_datacycle].' day');
				$where.=" AND a.`lastupdate`>$uptime";
		    }
		}		
		//添加时间区间，即审核时间
		if($paramer[adtime]){
			$time=time();
			$adtime = $time-($paramer[adtime]*86400);
			$where.=" AND a.`status_time`>$adtime";
		}
		//是否有照片
		if($paramer[pic]=="1"){
			$where .=" AND a.`photo`<>'' AND a.`phototype`!=1 AND a.`defphoto`= 1";
		}
        //行业
		if($paramer['hy']!=""){
			$where .= " AND a.`hy` IN (".$paramer['hy'].")";
		}
        $job_col = $city_col = "";
        $cjwhere = "";
		if($paramer[threecityid]){
		    $city_col = "three_cityid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[threecityid]";
		}elseif($paramer[three_cityid]){
		    $city_col = "three_cityid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[three_cityid]";
		}elseif($paramer[cityid]){
		    $city_col = "cityid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[cityid]";
		}elseif($paramer[provinceid]){
		    $city_col = "provinceid";
		    $cjwhere .= " AND cj.`$city_col`= $paramer[provinceid]";
		}
        //城市区间,不建议执行该查询
		if($paramer[cityin]){
            if($city_parent[$paramer[cityin]]=='0'){
		        $city_col = "provinceid";
				$cjwhere .= " AND cj.`$city_col` = $paramer[cityin]";
			}elseif(in_array($city_parent[$paramer[cityin]],$city_index)){
		        $city_col = "cityid";
				$cjwhere .= " AND cj.`$city_col` = $paramer[cityin]";
			}elseif($city_parent[$paramer[cityin]]>0){
		        $city_col = "three_cityid";
				$cjwhere .= " AND cj.`$city_col` = $paramer[cityin]";
			}
		}
		
		if($paramer[jobpost]){
		    $job_col = "job_post";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[jobpost]";
		}elseif($paramer[job1son]){
		    $job_col = "job1_son";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job1son]";
		}elseif($paramer[job_post]){
		    $job_col = "job_post";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job_post]";
		}elseif($paramer[job1_son]){
		    $job_col = "job1_son";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job1_son]";
		}elseif($paramer[job1]){
		    $job_col = "job1";
		    $cjwhere .= " AND cj.`$job_col`= $paramer[job1]";
		}
        //职位区间,不建议执行该查询
		if($paramer[jobin]){
            if($job_parent[$paramer[jobin]]=='0'){
		        $job_col = "job1";
				$cjwhere .=" AND cj.`$job_col`= $paramer[jobin]";
			}elseif(in_array($job_parent[$paramer[jobin]],$job_index)){
		        $job_col = "job1_son";
				$cjwhere .=" AND cj.`$job_col`= $paramer[jobin]";
			}elseif($job_parent[$paramer[jobin]]>0){
		        $job_col = "job_post";
				$cjwhere .=" AND cj.`$job_col`= $paramer[jobin]";
			}
		}
		// 拼接唯一标识字段
		if($city_col || $job_col){
		    if($city_col && $job_col){
		        $cjwhere .= " AND cj.`{$city_col}_{$job_col}_num`= 1";
		    }elseif($city_col){
		        $cjwhere .= " AND cj.`{$city_col}_num`= 1";
		    }elseif($job_col){
		        $cjwhere .= " AND cj.`{$job_col}_num`= 1";
		    }
		}
		
		//工作经验
		if($paramer[exp]){
		    if (stripos($userclass_name[$paramer[exp]], "应届生") !== false){
                // 应届生特殊，只搜应届生
                $where .= " AND a.`exp`=".$paramer[exp];
            }else{
                $expArr  = $userdata['user_word'];
    			$expSort = 0;
    			$expIds  = array();
    			// 简历搜索，排序比搜索大的都符合条件。如搜“五年”，类别排序大于等于“五年”排序的（要排除不限）都符合
                foreach ($expArr as $k => $v) {
                    if ($v == $paramer[exp] && $userclass_name[$v] != "不限"){
                        $expSort = $k;
                        break;
                    }
                }
                foreach ($expArr as $k => $v) {
                    if ($k >= $expSort && $userclass_name[$v] != "不限"){
                        $expIds[] = $v;
                    }
                }
                if (!empty($expIds)) {
                	$where .= " AND a.`exp` in (".@implode(",",$expIds).")";
                }
            }
		}
		//学历
		if($paramer[edu]){

		    $eduArr  = $userdata['user_edu'];
			$eduSort = 0;
			$eduIds  = array();
			// 简历搜索，排序比搜索大的都符合条件。如搜“硕士”，类别排序大于等于“硕士”排序的（要排除不限）都符合
			foreach ($eduArr as $k => $v) {
			    if ($v == $paramer[edu] && $userclass_name[$v] != "不限"){
			        $eduSort = $k;
                    break;
			    }
			}
			foreach ($eduArr as $k => $v) {
			    if ($k >= $eduSort && $userclass_name[$v] != "不限"){
			        $eduIds[] = $v;
			    }
			}
            if (!empty($eduIds)) {
            	$where .= " AND a.`edu` in (".@implode(",",$eduIds).")";
            }
		}
		//性别
		if($paramer[sex]){
			$where .=" AND a.`sex`=$paramer[sex]";
		}
		//到岗时间
		if($paramer[report]){
			$where .=" AND a.`report`=$paramer[report]";
		}
		//工作性质
		if($paramer[type]){
			$where .= " AND a.`type`=$paramer[type]";
		}
		if($paramer[notid]){
			$where.= " and `id`<>$paramer[notid]";
		}
		//简历完整度
		
		if($paramer[integrity]){
		 	$integrityR	=	$arr_data["integrity_val"];
		 	
		 	$where.= " AND a.`integrity`>='".$integrityR[$paramer[integrity]]."'";

		}

		//关键字
		if($paramer[keyword]){
			$where1 = array();
			$where1[]="a.`name` LIKE '%$paramer[keyword]%'";
			$where1[]="a.`uname` LIKE '%$paramer[keyword]%'";
			
			//搜索工作经历 
			$workList = $db->select_all('resume_work',"`name` LIKE '%$paramer[keyword]%' OR `title` LIKE '%$paramer[keyword]%' OR `content` LIKE '%$paramer[keyword]%' ORDER BY id DESC limit 500","`eid`");
			if(!empty($workList)){
				$workId = array();
				foreach($workList as $value){
					$workId[] = $value['eid'];
				}
				$where1[]	=	"a.id IN (".implode(',',$workId).")";
			}
           
            $where.=" AND (".@implode(" or ",$where1).")";
		}
		//薪资待遇
		if($paramer[minsalary]&&$paramer[maxsalary]){
			$where.= " AND ((a.`minsalary`<=".intval($paramer[minsalary])." and a.`maxsalary`>=".intval($paramer[minsalary]).") 
						or (a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`>=".intval($paramer[maxsalary])."))";
		}elseif($paramer[minsalary]&&!$paramer[maxsalary]){
			$where.= " AND ((a.`minsalary`<=".intval($paramer[minsalary])." and a.`maxsalary`>=".intval($paramer[minsalary]).") 
						or (a.`minsalary`>=".intval($paramer[minsalary])." and a.`maxsalary`>=".intval($paramer[minsalary]).")
						or (a.`minsalary`!=0 and  a.`maxsalary`=0))";
		}elseif(!$paramer[minsalary]&&$paramer[maxsalary]){
			$where.= " AND ((a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`>=".intval($paramer[maxsalary]).")
						or (a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`<=".intval($paramer[maxsalary]).")  
						or (a.`minsalary`<=".intval($paramer[maxsalary])." and a.`maxsalary`=0) 
						or (a.`minsalary`=0 and a.`maxsalary`!=0)
						)";
		}
		//年龄
		if($paramer[minage]&&$paramer[maxage]){
			$mintime=date("Y-m-d",strtotime("-".$paramer[minage]." year"));
			$maxtime=date("Y-m-d",strtotime("-".$paramer[maxage]." year"));
			$where.= " AND a.`birthday`>= '".$maxtime."' and a.`birthday`<='".$mintime."'";
		}elseif($paramer[minage]&&!$paramer[maxage]){
			$mintime=date("Y-m-d",strtotime("-".$paramer[minage]." year"));
			$where.= " AND a.`birthday`<='".$mintime."'";
		}elseif(!$paramer[minage]&&$paramer[maxage]){
			$maxtime=date("Y-m-d",strtotime("-".$paramer[maxage]." year"));
			$where.= " AND a.`birthday`>='".$maxtime."'";
		}
        //排序字段默认为更新时间
		if($paramer[order] && $paramer[order]!="lastdate"){
			if($paramer[order]=='topdate'){
				$nowtime=time();
				$order = " ORDER BY if(a.`topdate`>$nowtime,a.`topdate`,a.`lastupdate`) ";
			}else{
				$order = " ORDER BY a.`".$paramer[order]."` ";
			}
		}else{
			$order = " ORDER BY a.`lastupdate` ";
		}
		//排序规则 默认为倒序
		$sort = $paramer[sort]?$paramer[sort]:"DESC";
		//查询条数
		if($paramer[limit]){
			$limit=" LIMIT ".$paramer[limit];
		}
		//自定义查询条件，默认取代上面任何参数直接使用该语句
		if($paramer[where]){
			$where = $paramer[where];
		}
        $pagewhere = "";$joinwhere = "";
        if($cjwhere){
            $pagewhere.=" ,`".$db_config[def]."resume_city_job_class`  cj ";
            $joinwhere .= " a.`id`=cj.`eid` " . $cjwhere;
        }
        
		if($paramer[ispage]){
            // 查询分页
			if($paramer["height_status"]){
				$limit = PageNav($paramer,$_GET,"resume_expect",$where,$Purl,"",$paramer[islt]?$paramer[islt]:"3",$_smarty_tpl,$pagewhere,$joinwhere);
			}else{
				$limit = PageNav($paramer,$_GET,"resume_expect",$where,$Purl,"","0",$_smarty_tpl,$pagewhere,$joinwhere);
			}
		}

		if($paramer[topdate] && $_GET[page]>1){
			$ulist = array();
		}else{
			
			$select="a.`id`,a.`uid`,a.`name`,a.`hy`,a.`job_classid`,a.`city_classid`,a.`jobstatus`,a.`type`,a.`report`,a.`lastupdate`,a.`rec`,a.`top`,a.`topdate`,a.`rec_resume`,a.`ctime`,a.`uname`,a.`idcard_status`,a.`minsalary`,a.`maxsalary`";
			if($pagewhere!=""){

				$sql = "select ".$select." from `".$db_config[def]."resume_expect` a ".$pagewhere." where ".$joinwhere." and ".$where.$order.$sort.$limit;
				$ulist=$db->DB_query_all($sql,"all");

			}else{
				$sql = "select ".$select." from `".$db_config[def]."resume_expect` a where ".$where.$order.$sort.$limit;
				$ulist=$db->DB_query_all($sql,"all");
			}
		}
        
		if(!empty($ulist) && is_array($ulist)){
			
			//如果存在top，则说明请求来自排行榜页面
			if($paramer['top']){
				$uids=$m_name=array();
				foreach($ulist as $k=>$v){
					$uids[]=$v[uid];
				}

				$member=$db->select_all($db_config[def]."member","`uid` in(".@implode(',',$uids).")","uid,username");
				foreach($member as $val){
					$m_name[$val[uid]]=$val['username'];
				}
			}
			$uid = $eid = array();
			foreach($ulist as $key=>$value){
				
				$uid[] = $value['uid'];
				$eid[] = $value['id'];
			}
			$eids = @implode(',',$eid);
			$uids = @implode(',',$uid);
            $resume=$db->select_all("resume","`uid` in(".$uids.")","uid,name,nametype,tag,sex,moblie_status,edu,exp,photo,phototype,defphoto,photo_status,birthday");
			foreach($resume as $v){
				$ruids[] = $v['uid'];
			}
			foreach($ulist as $k=>$v){
				if(!in_array($v['uid'],$ruids)){
					unset($ulist[$k]);
					continue;
				}
			}
			if($paramer[topdate]){
				$noids=array();
			}

			if($paramer[workexp] == 1){
				$eduList  = $db -> select_all("resume_edu","`eid` in(".$eids.")");
				if(!empty($eduList)){
					foreach($eduList as $key=>$value){
						$eduListNew[$value['eid']][] = $value;
					}
					foreach($eduListNew as $k=>$eduv){
						$edumin				=		0;
						$edumax				=		0;
						$edutitle			=	array();
						$education			=	array();
						foreach($eduv as $v){
							if($v['sdate']>0 && $edumin==0){
								$edumin		=		$v['sdate'];
							}elseif($edumin>$v['sdate']){
								$edumin		=		$v['sdate'];
							}
							if($v['edate']==0 ){
								$edumax		=		0;
							}elseif($edumax<$v['edate']){
								$edumax		=		$v['edate'];
							}
						
							$education[]	=		$userclass_name[$v['education']];

							$edutitle[]		=		$v['specialty'];
						}
						$return =array();
						$return['edumin']		=		date('Y-m',$edumin);
						$return['edumax']		=		$edumax  == 0 ?  '至今': date('Y',$edumax);
						$return['education']	=		@implode(',',$education);
						$return['eduspecialty']		=		@implode('、',$edutitle);
						
						$return['edu_time']		=		$return['edumin']."-".$return['edumax'];
						
						if($return['eduspecialty']){
						    $workexpList[$k]['edu_content']	=	$return['education'].'学历 · '.$return['eduspecialty'].' · 毕业于'.$return['edumax'].'年';
						}else{
						    $workexpList[$k]['edu_content']	=	$return['education'].'学历 · 毕业于'.$return['edumax'].'年';
						}
					}
					
				}

				$workList  = $db -> select_all("resume_work","`eid` in(".$eids.")");
				if(!empty($workList)){
					foreach($workList as $key=>$value){
						$workListNew[$value['eid']][] = $value;
					}
					foreach($workListNew as $k=>$workv){
						
						$whour     		=   0;
						$hour     	 	=   array();
						$time      		=   time();
						$workmin		=	0;
						$workmax		=	0;
						$worknum		=	count($workv);
						$wtitle			=	array();
						foreach($workv as $v){
							/* 计算每份工作时长(按月) */
							
							if($v['sdate']>0 && $workmin==0){
								$workmin		=		$v['sdate'];
							}elseif($workmin>$v['sdate']){
								$workmin		=		$v['sdate'];
							}
							
							if($v['edate']==0 ){
								$workmax		=		0;
							}elseif($workmax<$v['edate']){
								$workmax		=		$v['edate'];
							}
			
							$wtitle[]			=		$v['title'];
							
							$hour[]   			=		$workTime;
							$whour    			+=   	$workTime;
						}
						
						$workavg   =   ceil($whour/count($hour));
						$return = array();
						$return['worknum']		=		$worknum  > 0 ?  $worknum:0;
						$return['workavg']		=		$workavg  > 0 ?  $workavg:0;
						$return['workmin']		=		date('Y-m',$workmin);
						$return['workmax']		=		$workmax  == 0 ?  '至今': date('Y-m',$workmax);
						$return['worktit']		=		@implode(',',$wtitle);
						
						$return['work_time']	=		$return['workmin'].'-'.$return['workmax'];
						
						if($return['worktit']!=''){
							$workexpList[$k]['work_content']	=	' 参加过'.$return['worknum'].'份工作 · 涉及'.$return['worktit'].'等岗位';
						}else{
							$workexpList[$k]['work_content']	=	' 参加过'.$return['worknum'].'份工作';
						}
					}
				}
			}
			if ($_COOKIE['uid'] && $_COOKIE['usertype']==2){
			    $lookResume =   $db->select_all("look_resume","`com_id` = ".$_COOKIE['uid'], "`resume_id`");
			    if (!empty($lookResume)){
			        foreach($lookResume as $key=>$value){
						$lookResumeId[] = $value['resume_id'];
					}
			    }
			    if($paramer[isrck] == 1){
                $downList = $freedownList = array();
                    $downList = $db->select_all('down_resume', "`comid` = ".$_COOKIE['uid'] . " and `uid` in (".$uids.")", "`comid`,`eid`");
                    $freedownList = $db->select_all('freedown_resume', "`comid` = ".$_COOKIE['uid'] . " and `uid` in (".$uids.")", '`comid`,`eid`');
			    $vipetime = 0;
			    $vip_etime = $db->select_all('company', "`uid` = ".$_COOKIE['uid'], "`vipetime`");
			    $vipetime = $vip_etime[0][vipetime] ? $vip_etime[0][vipetime] : 0;
			    $userid_msg = $db->select_all('userid_msg', "`fid` = ".$_COOKIE['uid'] . " and `uid` in (".$uids.") and `isdel` = 9",'`uid`');
			    }
			}
			foreach($ulist as $k=>$v){
				if($paramer[topdate]){
					$noids[] = $v[id];
				}
				//筛除重复
				if($paramer[noid]=='1' && !empty($noids) && in_array($v['id'],$noids)){
					unset($ulist[$k]);
					continue;
				}
			    foreach($resume as $val){
			        if($v['uid']==$val['uid']){
			    		$ulist[$k]['edu_n']=$userclass_name[$val['edu']];
				        $ulist[$k]['exp_n']=$userclass_name[$val['exp']];
			            if($val['birthday']){
							
							$ageArr = explode('-',$val['birthday']);
						
							if(count($ageArr)==3){// 原来数据生日格式为年月日，如果是旧格式，改为年月
								
								$val['birthday'] = $ageArr[0].'-'.$ageArr[1];
								
							}
							
							$year = date("Y",strtotime($val['birthday']));
							$ulist[$k]['age'] =date("Y")-$year;
						}
						if($val['sex']==152){
							$val['sex']='1';
						}elseif ($val['sex']==153){
							$val['sex']='2';
						}
						$ulist[$k]['sex'] =$arr_data[sex][$val['sex']];
		                $ulist[$k]['phototype']=$val[phototype];
		                $photo=$icon="";
		                if($val['defphoto']==2){
		                	$photo=$val['photo'];
		                }else{
							if($config['user_pic']==1 || empty($config['user_pic'])){
				                if($val['photo'] && $val['photo_status']==0 && $val['phototype']!=1){
		            				$photo=$val['photo'];
		            			}else{
		            				if($val['sex']==1){
		            					$icon=$config['sy_member_icon'];
		            				}else{
		            					$icon=$config['sy_member_iconv'];
		            				}
		            			}
		            			
							}elseif($config['user_pic']==2){
								if($val['photo']&& $val['photo_status']==0){
									$photo=$val['photo'];
								}else{
									if($val['sex']==1){
										$icon=$config['sy_member_icon'];
									}else{
										$icon=$config['sy_member_iconv'];
									}
								}
							}elseif($config['user_pic']==3){
								if($val['sex']==1){
									$icon=$config['sy_member_icon'];
								}else{
									$icon=$config['sy_member_iconv'];
								}
							}
						}
						$ulist[$k]['photo']=checkpic($photo,$icon);
						if($val['tag']){
                            $ulist[$k]['tag']=explode(',',$val['tag']);
	                    }
                        $ulist[$k]['nametype']=$val[nametype];
                        $ulist[$k]['moblie_status']=$val[moblie_status];
                        //名称显示处理
						if($config['user_name']==1 || !$config['user_name']){
    						if($val['nametype']==3){
    						    if($val['sex']==1){
    						        $ulist[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."先生";
    						    }else{
    						        $ulist[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."女士";
    						    }
    						}elseif($val['nametype']==2){
    						    $ulist[$k]['username_n'] = "NO.".$v['id'];
    						}else{
    							$ulist[$k]['username_n'] = $val['name'];
    						}
						}elseif($config['user_name']==3){
							if($val['sex']==1){
								$ulist[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."先生";
							}else{
								$ulist[$k]['username_n'] = mb_substr($val['name'],0,1,'utf-8')."女士";
							}
						}elseif($config['user_name']==2){
							$ulist[$k]['username_n'] = "NO.".$v['id'];
						}elseif($config['user_name']==4){
							$ulist[$k]['username_n'] = $val['name'];
						}
                    }
                }
				//更新时间显示方式
				$time=$v['lastupdate'];
				//今天开始时间戳
				$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
				//昨天开始时间戳
				$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
				if($time>$beginYesterday && $time<$beginToday){
					$ulist[$k]['time'] = "昨天";
				}elseif($time>$beginToday){
					$ulist[$k]['time'] = lastupdateStyle($v['lastupdate']);
					$ulist[$k]['redtime'] =1;
				}else{
					$ulist[$k]['time'] = date("Y-m-d",$v['lastupdate']);
				} 
                
                // 前天
				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

				if($v['ctime']>$beforeYesterday){
					$ulist[$k]['newtime'] =1;
				}
				$ulist[$k]['user_jobstatus_n']=$userclass_name[$v['jobstatus']];
// 				$ulist[$k]['job_city_one']=$city_name[$v['provinceid']];
// 				$ulist[$k]['job_city_two']=$city_name[$v['cityid']];
// 				$ulist[$k]['job_city_three']=$city_name[$v['three_cityid']];
                $ulist[$k]['salary_n'] = salaryUnit($v['minsalary'], $v['maxsalary']);
				$ulist[$k]['report_n']=$userclass_name[$v['report']];
				$ulist[$k]['type_n']=$userclass_name[$v['type']];
				$ulist[$k]['lastupdate']=date("Y-m-d",$v['lastupdate']);
					
				$ulist[$k]['user_url']=Url("resume",array("c"=>"show","id"=>$v['id']),"1");
				$ulist[$k]["hy_info"]=$industry_name[$v['hy']];
				if($paramer['top']){
					$ulist[$k]['m_name']=$m_name[$v['uid']];
					$ulist[$k]['user_url']=Url("ask",array("c"=>"friend","a"=>"myquestion","uid"=>$v['uid']));
				}
				$ulist[$k]['work_content']=$workexpList[$v['id']]['work_content'];
				$ulist[$k]['edu_content']=$workexpList[$v['id']]['edu_content'];

				$kjob_classid=@explode(",",$v['job_classid']);
				$kjob_classid=array_unique($kjob_classid);	
				$jobname=array();
				if(is_array($kjob_classid)){
					foreach($kjob_classid as $val){
					    if($val!=''){
					        if($paramer['keyword']){
                               $jobname[]=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$job_name[$val]);
                            }else{
                               $jobname[]=$job_name[$val];
                            }
                        }
					}
				}
				//$ulist[$k]['job_post']=@implode(",",$jobname);
				$ulist[$k]['expectjob']=$jobname;
				$kcity_classid=@explode(",",$v['city_classid']);
				$kcity_classid=array_unique($kcity_classid);	
				$cityname=array();
				if(is_array($kcity_classid)){
					foreach($kcity_classid as $val){
					    if($val!=''){
					       
                            $cityname[]=$city_name[$val];
                            
                        }
					}
				}
                //$ulist[$k]['citylist']=@implode("/",$cityname);
				$ulist[$k]['expectcity']=$cityname;
				//截取标题
				if($paramer['post_len']){
					$postname[$k]=@implode(",",$jobname);
					$ulist[$k]['job_post_n']=mb_substr($postname[$k],0,$paramer[post_len],"utf-8");
				}
                if($paramer['city_len']){
					$scityname[$k]=@implode("/",$cityname);
					$ulist[$k]['city_name_n']=mb_substr($scityname[$k],0,$paramer[city_len],"utf-8");
				}
				//  是否浏览过
				$ulist[$k]['isLookEd'] = 0;
				if(in_array($v['id'], $lookResumeId)){
				    $ulist[$k]['isLookEd'] = 1;
				}
				if($paramer[isrck] == 1){
				//  是否已下载
				$ulist[$k]['islink'] = 0;
				foreach ($downList as $dv) {
                    if ($v['id'] == $dv['eid']) {
                        $ulist[$k]['islink'] = 1;
                    }
                }
                foreach ($freedownList as $dv) {
                    if ($v['id'] == $dv['eid']) {
                        if ($vipetime) {
                            $ulist[$k]['islink'] = 1;
                        }else{
                            $ulist[$k]['islink'] = 0;
                        }
                    }
                }
                //  是否已邀约
                $ulist[$k]['userid_msg'] = 0;
                foreach($userid_msg as $val){
                    if($v['uid']==$val['uid']){
                        $ulist[$k]['userid_msg'] = 1;
                        }
                    }
                }
			}
			foreach($ulist as $k=>$v){
               if($paramer['keyword']){
					$ulist[$k]['username_n']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$v['username_n']);
					$ulist[$k]['job_post']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$ulist[$k]['job_post']);
					$ulist[$k]['job_post_n']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$ulist[$k]['job_post_n']);
					$ulist[$k]['city_name_n']=str_replace($paramer['keyword'],"<font color=#FF6600 >".$paramer['keyword']."</font>",$ulist[$k]['city_name_n']);
				}
            }

			
			if($paramer['keyword']!=""&&!empty($ulist)){
				addkeywords('5',$paramer['keyword']);
			}
		}$ulist = $ulist; if (!is_array($ulist) && !is_object($ulist)) { settype($ulist, 'array');}
foreach ($ulist as $_smarty_tpl->tpl_vars['ulist']->key => $_smarty_tpl->tpl_vars['ulist']->value) {
$_smarty_tpl->tpl_vars['ulist']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['ulist']->key;
?>
                <li>
                    <div class="index_resume_user">
                        <a <?php if ($_smarty_tpl->tpl_vars['config']->value['com_search']==1&&!$_smarty_tpl->tpl_vars['uid']->value) {?> href="<?php echo smarty_function_url(array('m'=>'login'),$_smarty_tpl);?>
" <?php } elseif ($_smarty_tpl->tpl_vars['usertype']->value==1) {
if ($_smarty_tpl->tpl_vars['config']->value['sy_user_change']==1) {?>onclick="layer.msg('请先申请企业用户', 2, 8)" <?php } else { ?>onclick="layer.msg('只有企业用户才能查看', 2, 8)"<?php }?> <?php } else { ?>href="<?php echo $_smarty_tpl->tpl_vars['ulist']->value['user_url'];?>
" target="_blank" <?php }?> class="index_resume_username">
                        <img src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['ulist']->value['photo'];?>
" width="30" height="30">
                        <?php if (in_array($_smarty_tpl->tpl_vars['ulist']->value['id'],$_smarty_tpl->tpl_vars['eid']->value)) {?> <?php echo $_smarty_tpl->tpl_vars['ulist']->value['uname'];?>
 <?php } else { ?> <?php echo mb_substr($_smarty_tpl->tpl_vars['ulist']->value['username_n'],0,10,"utf-8");?>
 <?php }?>
                        </a>
                    </div>
                    <div class="index_resume_userinfo">
                        <?php echo $_smarty_tpl->tpl_vars['ulist']->value['sex'];?>
<i class="index_resume_userinfo_line">|</i><?php echo $_smarty_tpl->tpl_vars['ulist']->value['age'];?>
岁<i class="index_resume_userinfo_line">|</i><?php echo $_smarty_tpl->tpl_vars['ulist']->value['exp_n'];?>
经验<i class="index_resume_userinfo_line">|</i><?php echo $_smarty_tpl->tpl_vars['ulist']->value['edu_n'];?>
学历
                    </div>
                    <div class="index_resume_useryx">
                        意向：<span class="index_resume_useryx_n"><?php echo $_smarty_tpl->tpl_vars['ulist']->value['job_post_n'];?>
</span>
                    </div>
                </li>
                <?php } ?>
            </ul>
        </div>
        <div class="yunheader_60lookmore"><a href="<?php echo smarty_function_url(array('m'=>'resume','c'=>'search'),$_smarty_tpl);?>
">查看更多</a></div>
    </div>

    <div class="index_zl_box">
        <div class="yunheader_60_tit"><a href="<?php echo smarty_function_url(array('m'=>'article'),$_smarty_tpl);?>
" target="_blank" class="yunheader_60_tit_a"><i class="yunheader_60_tit_line"></i>职场资讯<i class="yunheader_60_tit_rline"></i></a></div>
        <div class="yunheader_60_tit_p" data-no='1'>您关心的职场头条</div>
        <div class="index_news_box60">
            <div class="index_news_box">
                <!--图文循环-->
                <?php  $_smarty_tpl->tpl_vars['plist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['plist']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;

		global $db,$db_config,$config;
		include PLUS_PATH.'/group.cache.php';$plist=	array();
		$rs		=	null;
		$nids	=	null;

		$paramer	=	array("item"=>"“plist“","type"=>"“t“","pic"=>"1","urlstatic"=>"1","t_len"=>"20","limit"=>"2","key"=>"“key“","name"=>"“newlistpic“","nocache"=>"")
;

		$ParamerArr	=	GetSmarty($paramer,$_GET,$_smarty_tpl);

		$paramer	=	$ParamerArr['arr'];

		$Purl		=	$ParamerArr['purl'];

		if($paramer[cache]){

			$Purl	=	"{{page}}.html";
		}

		global $ModuleName;

		if(!$Purl["m"]){
			$Purl["m"]=$ModuleName;
		}

		$where=1;

		$where .=" and (`starttime`<=".time()." or `starttime`=0 or `starttime` is null)";

        $where .=" and (`endtime`>".time()." or `endtime`=0 or `endtime` is null)";

		if($config['did']){

			$where	.=	" and (`did`='".$config['did']."' or `did`=-1)";
        }else{
			
			$where	.=	" and (`did`=-1 OR `did`=0 OR did='')";
		}

		include PLUS_PATH."/group.cache.php";
		if($paramer['nid']){
			$nid_s = @explode(',',$paramer[nid]);
			foreach($nid_s as $v){
				if($group_type[$v]){
					$paramer[nid] = $paramer[nid].",".@implode(',',$group_type[$v]);
				}
			}
		}

		if($paramer['nid']!="" && $paramer['nid']!=0){
			$where .=" AND `nid` in ($paramer[nid])";
			$nids = @explode(',',$paramer['nid']);
			$paramer['nid']=$paramer['nid'];
		}else if($paramer['rec']!=""){
			$nids=array();
			if(is_array($group_rec)){
				foreach($group_rec as $key=>$value){
					if($key<=2){
						$nids[]=$value;
					}
				}
				$paramer[nid]=@implode(',',$nids);
			}
		}

		if($paramer['type']){
			$type = str_replace("\"","",$paramer[type]);
			$type_arr =	@explode(",",$type);
			if(is_array($type_arr) && !empty($type_arr)){
				foreach($type_arr as $key=>$value){
					$where .=" AND FIND_IN_SET('".$value."',`describe`)";
					if(count($nids)>0){
						$picwhere .=" AND FIND_IN_SET('".$value."',`describe`)";
					}
				}
			}
		}

		//拼接补充SQL条件
		if($paramer['pic']!=""){
			$where .=" AND `newsphoto`<>''";
		}

		 //新闻搜索
		if($paramer['keyword']!=""){
			$where .=" AND `title` LIKE '%".$paramer[keyword]."%'";
		}

		//拼接查询条数
		if(intval($paramer['limit'])>0){
			$limit = intval($paramer['limit']);
			$limit = " limit ".$limit;
		}

		if($paramer['ispage']){
			if($Purl["m"]=="wap"){
				$limit = PageNav($paramer,$_GET,"news_base",$where,$Purl,"","6",$_smarty_tpl);
			}else{
				$limit = PageNav($paramer,$_GET,"news_base",$where,$Purl,"","5",$_smarty_tpl);
			}
		}

		//拼接字段排序
		if($paramer['order']!=""){
		
			$where .=" ORDER BY $paramer[order] DESC";
		}else{
		
		    $where .=" ORDER BY `starttime` DESC, `datetime` DESC";
		}

		//多类别新闻查找
		if(!intval($paramer['ispage']) && count($nids)>0){

			$nidArr = @explode(',',$paramer[nid]);
			$rsnids = array();
			if(is_array($group_type)){
				foreach($group_type as $key=>$value){
					if(in_array($key,$nidArr)){
						if(is_array($value)){
							foreach($value as $v){
								$rsnids[$v] = $key;
								$nidArr[] = $v;
							}
						}
					}
				}
			}

			$where = " `nid` IN (".@implode(',',$nidArr).")";

			if($config['did']){
				$where.=" and `did`='".$config['did']."'";
			}

			//查询带图新闻
			if($paramer['pic']){
				if(!$paramer['piclimit']){
					$piclimit = 1;
				}else{
					$piclimit = $paramer['piclimit'];
				}

				$db->query("set @f=0,@n=0");

				$query = $db->query("select * from (select id,title,color,datetime,starttime,description,newsphoto,@n:=if(@f=nid,@n:=@n+1,1) as aid,@f:=nid as nid from $db_config[def]news_base  WHERE ".$where." AND `newsphoto` <>''  order by nid asc,starttime desc) a where aid <=".$piclimit);

				$conque = $db->select_all("news_content","1 order by nbid desc".$limit);

				foreach($conque as $cv){
					$newcon[$cv[nbid]]=$cv;
				}
				while($rs = $db->fetch_array($query)){

					if($rsnids[$rs['nid']]){
						$rs['nid'] = $rsnids[$rs['nid']];
					}

					//处理标题长度
					if(intval($paramer[t_len])>0){
						$len = intval($paramer[t_len]);
						$rs[title_n] = $rs[title];
						$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
					}

					if($rs[color]){
						$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
					}

					//处理描述内容长度
					if(intval($paramer[d_len])>0){
						$len = intval($paramer[d_len]);
						$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
					}

					$rs['name'] = $group_name[$rs['nid']];

					//构建资讯静态链接
					if($config[sy_news_rewrite]=="2"){
						$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
					}else{
						$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
					}

					if(mb_substr($rs[newsphoto],0,4)=="http"){
						$rs["picurl"]=$rs[newsphoto];
					}else{
						if($rs['newsphoto']==""){
							$content=str_replace(array('"',"'"),array("",""),$newcon[$rs[id]]["content"]);
							preg_match_all("/<img[^>]+src=(.*?)\s[^>]+>/im",$content,$res);
							$str=str_replace("\\","",$res[1][0]);
							if($str){
								$rs[newsphoto]=".".$str;
							}
						}
						$nopic=$config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";

						$rs["picurl"] = checkpic($rs['newsphoto'],$nopic);

					}

					$rs[time]=date("Y-m-d",$rs[starttime]);
					$rs['starttime']=date("m-d",$rs[starttime]);
					if(count($plist[$rs['nid']]['pic'])<$piclimit){
					  $plist[$rs['nid']]['pic'][] = $rs;
					}
				}//end while
			}

			$db->query("set @f=0,@n=0");
			$query = $db->query("select * from (select id,title,datetime,starttime,color,description,newsphoto,@n:=if(@f=nid,@n:=@n+1,1) as aid,@f:=nid as nid from $db_config[def]news_base  WHERE ".$where." order by nid asc,starttime desc) a where aid <=$paramer[limit]");

			while($rs = $db->fetch_array($query)){
				if($rsnids[$rs['nid']]){
					$rs['nid'] = $rsnids[$rs['nid']];
				}

				//处理标题长度
				if(intval($paramer[t_len])>0){
					$len = intval($paramer[t_len]);
					$rs[title_n] = $rs[title];
					$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
				}

				if($rs[color]){
					$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
				}

				//处理描述内容长度
				if(intval($paramer[d_len])>0){
					$len = intval($paramer[d_len]);
					$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
				}

				//获取所属类别名称
				$rs['name'] = $group_name[$rs['nid']];

				//构建资讯静态链接
				if($config[sy_news_rewrite]=="2"){
					$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
				}else{
					$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
				}

				if(mb_substr($rs[newsphoto],0,4)=="http"){
					$rs["picurl"]=$rs[newsphoto];
				}else{
					if($rs['newsphoto']==""){
						$rs["picurl"] = $config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";
					}else{
						$rs["picurl"] = checkpic($rs['newsphoto']);
					}
				}

				$rs[time]=date("Y-m-d",$rs[starttime]);
				$rs[starttime]=date("m-d",$rs[starttime]);
				if(count($plist[$rs['nid']]['arclist'])<$paramer[limit]){
					$plist[$rs['nid']]['arclist'][] = $rs;
				}
			}//end while

		}//end if(!intval($paramer['ispage']) && count($nids)>0)
		else{
			$query = $db->query("SELECT * FROM `$db_config[def]news_base` WHERE ".$where.$limit);

			while($rs = $db->fetch_array($query)){
				//处理标题长度
				if(intval($paramer[t_len])>0){
					$len = intval($paramer[t_len]);
					$rs[title_n] = $rs[title];
					$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
				}

				if($rs[color]){
					$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
				}

				//处理描述内容长度
				if(intval($paramer[d_len])>0){
					$len = intval($paramer[d_len]);
					$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
				}

				//获取所属类别名称
				 $rs['name'] = $group_name[$rs['nid']];

				//构建资讯静态链接
				if($config[sy_news_rewrite]=="2"){
					$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
				}else{
					$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
				}

				if(mb_substr($rs[newsphoto],0,4)=="http"){
					$rs["picurl"]=$rs[newsphoto];
				}else{
					if($rs['newsphoto']==""){
						$rs["picurl"] = $config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";
					}else{
						$rs["picurl"] = checkpic($rs['newsphoto']);
					}
				}

				$rs[time]=date("Y-m-d",$rs[starttime]);
				$rs[starttime]=date("m-d",$rs[starttime]);
				$plist[] = $rs;
			}//end while
		}$plist = $plist; if (!is_array($plist) && !is_object($plist)) { settype($plist, 'array');}
foreach ($plist as $_smarty_tpl->tpl_vars['plist']->key => $_smarty_tpl->tpl_vars['plist']->value) {
$_smarty_tpl->tpl_vars['plist']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['plist']->key;
?>
                <div class="index_news_list">
                    <div class="index_news_list_img">
                        <img src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['plist']->value['picurl'];?>
" width="190" height="120" alt="<?php echo $_smarty_tpl->tpl_vars['plist']->value['title_n'];?>
">
                    </div>
                    <div class="index_news_list_info">
                        <div class="index_news_list_name">
                            <a href="<?php echo $_smarty_tpl->tpl_vars['plist']->value['url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['plist']->value['title_n'];?>
"><?php echo $_smarty_tpl->tpl_vars['plist']->value['title'];?>
</a>
                        </div>
                        <div class="index_news_list_lb"><?php echo $_smarty_tpl->tpl_vars['plist']->value['name'];?>
</div>
                        <div class="index_news_list_time"><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['plist']->value['datetime'],"%Y-%m-%d");?>
</div>
                    </div>
                </div>
                <?php } ?>
                <!--列表循环-->
                <ul class="index_news_list_list">
                    <?php  $_smarty_tpl->tpl_vars['nlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['nlist']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;

		global $db,$db_config,$config;
		include PLUS_PATH.'/group.cache.php';$nlist=	array();
		$rs		=	null;
		$nids	=	null;

		$paramer	=	array("item"=>"“nlist“","t_len"=>"20","limit"=>"14","key"=>"“key“","name"=>"“newlist“","nocache"=>"")
;

		$ParamerArr	=	GetSmarty($paramer,$_GET,$_smarty_tpl);

		$paramer	=	$ParamerArr['arr'];

		$Purl		=	$ParamerArr['purl'];

		if($paramer[cache]){

			$Purl	=	"{{page}}.html";
		}

		global $ModuleName;

		if(!$Purl["m"]){
			$Purl["m"]=$ModuleName;
		}

		$where=1;

		$where .=" and (`starttime`<=".time()." or `starttime`=0 or `starttime` is null)";

        $where .=" and (`endtime`>".time()." or `endtime`=0 or `endtime` is null)";

		if($config['did']){

			$where	.=	" and (`did`='".$config['did']."' or `did`=-1)";
        }else{
			
			$where	.=	" and (`did`=-1 OR `did`=0 OR did='')";
		}

		include PLUS_PATH."/group.cache.php";
		if($paramer['nid']){
			$nid_s = @explode(',',$paramer[nid]);
			foreach($nid_s as $v){
				if($group_type[$v]){
					$paramer[nid] = $paramer[nid].",".@implode(',',$group_type[$v]);
				}
			}
		}

		if($paramer['nid']!="" && $paramer['nid']!=0){
			$where .=" AND `nid` in ($paramer[nid])";
			$nids = @explode(',',$paramer['nid']);
			$paramer['nid']=$paramer['nid'];
		}else if($paramer['rec']!=""){
			$nids=array();
			if(is_array($group_rec)){
				foreach($group_rec as $key=>$value){
					if($key<=2){
						$nids[]=$value;
					}
				}
				$paramer[nid]=@implode(',',$nids);
			}
		}

		if($paramer['type']){
			$type = str_replace("\"","",$paramer[type]);
			$type_arr =	@explode(",",$type);
			if(is_array($type_arr) && !empty($type_arr)){
				foreach($type_arr as $key=>$value){
					$where .=" AND FIND_IN_SET('".$value."',`describe`)";
					if(count($nids)>0){
						$picwhere .=" AND FIND_IN_SET('".$value."',`describe`)";
					}
				}
			}
		}

		//拼接补充SQL条件
		if($paramer['pic']!=""){
			$where .=" AND `newsphoto`<>''";
		}

		 //新闻搜索
		if($paramer['keyword']!=""){
			$where .=" AND `title` LIKE '%".$paramer[keyword]."%'";
		}

		//拼接查询条数
		if(intval($paramer['limit'])>0){
			$limit = intval($paramer['limit']);
			$limit = " limit ".$limit;
		}

		if($paramer['ispage']){
			if($Purl["m"]=="wap"){
				$limit = PageNav($paramer,$_GET,"news_base",$where,$Purl,"","6",$_smarty_tpl);
			}else{
				$limit = PageNav($paramer,$_GET,"news_base",$where,$Purl,"","5",$_smarty_tpl);
			}
		}

		//拼接字段排序
		if($paramer['order']!=""){
		
			$where .=" ORDER BY $paramer[order] DESC";
		}else{
		
		    $where .=" ORDER BY `starttime` DESC, `datetime` DESC";
		}

		//多类别新闻查找
		if(!intval($paramer['ispage']) && count($nids)>0){

			$nidArr = @explode(',',$paramer[nid]);
			$rsnids = array();
			if(is_array($group_type)){
				foreach($group_type as $key=>$value){
					if(in_array($key,$nidArr)){
						if(is_array($value)){
							foreach($value as $v){
								$rsnids[$v] = $key;
								$nidArr[] = $v;
							}
						}
					}
				}
			}

			$where = " `nid` IN (".@implode(',',$nidArr).")";

			if($config['did']){
				$where.=" and `did`='".$config['did']."'";
			}

			//查询带图新闻
			if($paramer['pic']){
				if(!$paramer['piclimit']){
					$piclimit = 1;
				}else{
					$piclimit = $paramer['piclimit'];
				}

				$db->query("set @f=0,@n=0");

				$query = $db->query("select * from (select id,title,color,datetime,starttime,description,newsphoto,@n:=if(@f=nid,@n:=@n+1,1) as aid,@f:=nid as nid from $db_config[def]news_base  WHERE ".$where." AND `newsphoto` <>''  order by nid asc,starttime desc) a where aid <=".$piclimit);

				$conque = $db->select_all("news_content","1 order by nbid desc".$limit);

				foreach($conque as $cv){
					$newcon[$cv[nbid]]=$cv;
				}
				while($rs = $db->fetch_array($query)){

					if($rsnids[$rs['nid']]){
						$rs['nid'] = $rsnids[$rs['nid']];
					}

					//处理标题长度
					if(intval($paramer[t_len])>0){
						$len = intval($paramer[t_len]);
						$rs[title_n] = $rs[title];
						$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
					}

					if($rs[color]){
						$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
					}

					//处理描述内容长度
					if(intval($paramer[d_len])>0){
						$len = intval($paramer[d_len]);
						$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
					}

					$rs['name'] = $group_name[$rs['nid']];

					//构建资讯静态链接
					if($config[sy_news_rewrite]=="2"){
						$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
					}else{
						$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
					}

					if(mb_substr($rs[newsphoto],0,4)=="http"){
						$rs["picurl"]=$rs[newsphoto];
					}else{
						if($rs['newsphoto']==""){
							$content=str_replace(array('"',"'"),array("",""),$newcon[$rs[id]]["content"]);
							preg_match_all("/<img[^>]+src=(.*?)\s[^>]+>/im",$content,$res);
							$str=str_replace("\\","",$res[1][0]);
							if($str){
								$rs[newsphoto]=".".$str;
							}
						}
						$nopic=$config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";

						$rs["picurl"] = checkpic($rs['newsphoto'],$nopic);

					}

					$rs[time]=date("Y-m-d",$rs[starttime]);
					$rs['starttime']=date("m-d",$rs[starttime]);
					if(count($nlist[$rs['nid']]['pic'])<$piclimit){
					  $nlist[$rs['nid']]['pic'][] = $rs;
					}
				}//end while
			}

			$db->query("set @f=0,@n=0");
			$query = $db->query("select * from (select id,title,datetime,starttime,color,description,newsphoto,@n:=if(@f=nid,@n:=@n+1,1) as aid,@f:=nid as nid from $db_config[def]news_base  WHERE ".$where." order by nid asc,starttime desc) a where aid <=$paramer[limit]");

			while($rs = $db->fetch_array($query)){
				if($rsnids[$rs['nid']]){
					$rs['nid'] = $rsnids[$rs['nid']];
				}

				//处理标题长度
				if(intval($paramer[t_len])>0){
					$len = intval($paramer[t_len]);
					$rs[title_n] = $rs[title];
					$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
				}

				if($rs[color]){
					$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
				}

				//处理描述内容长度
				if(intval($paramer[d_len])>0){
					$len = intval($paramer[d_len]);
					$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
				}

				//获取所属类别名称
				$rs['name'] = $group_name[$rs['nid']];

				//构建资讯静态链接
				if($config[sy_news_rewrite]=="2"){
					$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
				}else{
					$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
				}

				if(mb_substr($rs[newsphoto],0,4)=="http"){
					$rs["picurl"]=$rs[newsphoto];
				}else{
					if($rs['newsphoto']==""){
						$rs["picurl"] = $config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";
					}else{
						$rs["picurl"] = checkpic($rs['newsphoto']);
					}
				}

				$rs[time]=date("Y-m-d",$rs[starttime]);
				$rs[starttime]=date("m-d",$rs[starttime]);
				if(count($nlist[$rs['nid']]['arclist'])<$paramer[limit]){
					$nlist[$rs['nid']]['arclist'][] = $rs;
				}
			}//end while

		}//end if(!intval($paramer['ispage']) && count($nids)>0)
		else{
			$query = $db->query("SELECT * FROM `$db_config[def]news_base` WHERE ".$where.$limit);

			while($rs = $db->fetch_array($query)){
				//处理标题长度
				if(intval($paramer[t_len])>0){
					$len = intval($paramer[t_len]);
					$rs[title_n] = $rs[title];
					$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
				}

				if($rs[color]){
					$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
				}

				//处理描述内容长度
				if(intval($paramer[d_len])>0){
					$len = intval($paramer[d_len]);
					$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
				}

				//获取所属类别名称
				 $rs['name'] = $group_name[$rs['nid']];

				//构建资讯静态链接
				if($config[sy_news_rewrite]=="2"){
					$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
				}else{
					$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
				}

				if(mb_substr($rs[newsphoto],0,4)=="http"){
					$rs["picurl"]=$rs[newsphoto];
				}else{
					if($rs['newsphoto']==""){
						$rs["picurl"] = $config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";
					}else{
						$rs["picurl"] = checkpic($rs['newsphoto']);
					}
				}

				$rs[time]=date("Y-m-d",$rs[starttime]);
				$rs[starttime]=date("m-d",$rs[starttime]);
				$nlist[] = $rs;
			}//end while
		}$nlist = $nlist; if (!is_array($nlist) && !is_object($nlist)) { settype($nlist, 'array');}
foreach ($nlist as $_smarty_tpl->tpl_vars['nlist']->key => $_smarty_tpl->tpl_vars['nlist']->value) {
$_smarty_tpl->tpl_vars['nlist']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['nlist']->key;
?>
                    <li>
                        <a href="<?php echo $_smarty_tpl->tpl_vars['nlist']->value['url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['nlist']->value['title_n'];?>
"><i class="index_news_list_icon"></i><?php echo $_smarty_tpl->tpl_vars['nlist']->value['title'];?>
</a>
                        <em><?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['nlist']->value['datetime'],"%Y-%m-%d");?>
</em>
                    </li>
                    <?php } ?>
                </ul>
            </div>

            <div class="index_hotnews">
                <ul>
                    <?php  $_smarty_tpl->tpl_vars['alist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['alist']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;

		global $db,$db_config,$config;
		include PLUS_PATH.'/group.cache.php';$alist=	array();
		$rs		=	null;
		$nids	=	null;

		$paramer	=	array("item"=>"“alist“","t_len"=>"15","type"=>"“indextj“","limit"=>"10","key"=>"“key“","name"=>"“newlisthot“","nocache"=>"")
;

		$ParamerArr	=	GetSmarty($paramer,$_GET,$_smarty_tpl);

		$paramer	=	$ParamerArr['arr'];

		$Purl		=	$ParamerArr['purl'];

		if($paramer[cache]){

			$Purl	=	"{{page}}.html";
		}

		global $ModuleName;

		if(!$Purl["m"]){
			$Purl["m"]=$ModuleName;
		}

		$where=1;

		$where .=" and (`starttime`<=".time()." or `starttime`=0 or `starttime` is null)";

        $where .=" and (`endtime`>".time()." or `endtime`=0 or `endtime` is null)";

		if($config['did']){

			$where	.=	" and (`did`='".$config['did']."' or `did`=-1)";
        }else{
			
			$where	.=	" and (`did`=-1 OR `did`=0 OR did='')";
		}

		include PLUS_PATH."/group.cache.php";
		if($paramer['nid']){
			$nid_s = @explode(',',$paramer[nid]);
			foreach($nid_s as $v){
				if($group_type[$v]){
					$paramer[nid] = $paramer[nid].",".@implode(',',$group_type[$v]);
				}
			}
		}

		if($paramer['nid']!="" && $paramer['nid']!=0){
			$where .=" AND `nid` in ($paramer[nid])";
			$nids = @explode(',',$paramer['nid']);
			$paramer['nid']=$paramer['nid'];
		}else if($paramer['rec']!=""){
			$nids=array();
			if(is_array($group_rec)){
				foreach($group_rec as $key=>$value){
					if($key<=2){
						$nids[]=$value;
					}
				}
				$paramer[nid]=@implode(',',$nids);
			}
		}

		if($paramer['type']){
			$type = str_replace("\"","",$paramer[type]);
			$type_arr =	@explode(",",$type);
			if(is_array($type_arr) && !empty($type_arr)){
				foreach($type_arr as $key=>$value){
					$where .=" AND FIND_IN_SET('".$value."',`describe`)";
					if(count($nids)>0){
						$picwhere .=" AND FIND_IN_SET('".$value."',`describe`)";
					}
				}
			}
		}

		//拼接补充SQL条件
		if($paramer['pic']!=""){
			$where .=" AND `newsphoto`<>''";
		}

		 //新闻搜索
		if($paramer['keyword']!=""){
			$where .=" AND `title` LIKE '%".$paramer[keyword]."%'";
		}

		//拼接查询条数
		if(intval($paramer['limit'])>0){
			$limit = intval($paramer['limit']);
			$limit = " limit ".$limit;
		}

		if($paramer['ispage']){
			if($Purl["m"]=="wap"){
				$limit = PageNav($paramer,$_GET,"news_base",$where,$Purl,"","6",$_smarty_tpl);
			}else{
				$limit = PageNav($paramer,$_GET,"news_base",$where,$Purl,"","5",$_smarty_tpl);
			}
		}

		//拼接字段排序
		if($paramer['order']!=""){
		
			$where .=" ORDER BY $paramer[order] DESC";
		}else{
		
		    $where .=" ORDER BY `starttime` DESC, `datetime` DESC";
		}

		//多类别新闻查找
		if(!intval($paramer['ispage']) && count($nids)>0){

			$nidArr = @explode(',',$paramer[nid]);
			$rsnids = array();
			if(is_array($group_type)){
				foreach($group_type as $key=>$value){
					if(in_array($key,$nidArr)){
						if(is_array($value)){
							foreach($value as $v){
								$rsnids[$v] = $key;
								$nidArr[] = $v;
							}
						}
					}
				}
			}

			$where = " `nid` IN (".@implode(',',$nidArr).")";

			if($config['did']){
				$where.=" and `did`='".$config['did']."'";
			}

			//查询带图新闻
			if($paramer['pic']){
				if(!$paramer['piclimit']){
					$piclimit = 1;
				}else{
					$piclimit = $paramer['piclimit'];
				}

				$db->query("set @f=0,@n=0");

				$query = $db->query("select * from (select id,title,color,datetime,starttime,description,newsphoto,@n:=if(@f=nid,@n:=@n+1,1) as aid,@f:=nid as nid from $db_config[def]news_base  WHERE ".$where." AND `newsphoto` <>''  order by nid asc,starttime desc) a where aid <=".$piclimit);

				$conque = $db->select_all("news_content","1 order by nbid desc".$limit);

				foreach($conque as $cv){
					$newcon[$cv[nbid]]=$cv;
				}
				while($rs = $db->fetch_array($query)){

					if($rsnids[$rs['nid']]){
						$rs['nid'] = $rsnids[$rs['nid']];
					}

					//处理标题长度
					if(intval($paramer[t_len])>0){
						$len = intval($paramer[t_len]);
						$rs[title_n] = $rs[title];
						$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
					}

					if($rs[color]){
						$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
					}

					//处理描述内容长度
					if(intval($paramer[d_len])>0){
						$len = intval($paramer[d_len]);
						$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
					}

					$rs['name'] = $group_name[$rs['nid']];

					//构建资讯静态链接
					if($config[sy_news_rewrite]=="2"){
						$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
					}else{
						$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
					}

					if(mb_substr($rs[newsphoto],0,4)=="http"){
						$rs["picurl"]=$rs[newsphoto];
					}else{
						if($rs['newsphoto']==""){
							$content=str_replace(array('"',"'"),array("",""),$newcon[$rs[id]]["content"]);
							preg_match_all("/<img[^>]+src=(.*?)\s[^>]+>/im",$content,$res);
							$str=str_replace("\\","",$res[1][0]);
							if($str){
								$rs[newsphoto]=".".$str;
							}
						}
						$nopic=$config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";

						$rs["picurl"] = checkpic($rs['newsphoto'],$nopic);

					}

					$rs[time]=date("Y-m-d",$rs[starttime]);
					$rs['starttime']=date("m-d",$rs[starttime]);
					if(count($alist[$rs['nid']]['pic'])<$piclimit){
					  $alist[$rs['nid']]['pic'][] = $rs;
					}
				}//end while
			}

			$db->query("set @f=0,@n=0");
			$query = $db->query("select * from (select id,title,datetime,starttime,color,description,newsphoto,@n:=if(@f=nid,@n:=@n+1,1) as aid,@f:=nid as nid from $db_config[def]news_base  WHERE ".$where." order by nid asc,starttime desc) a where aid <=$paramer[limit]");

			while($rs = $db->fetch_array($query)){
				if($rsnids[$rs['nid']]){
					$rs['nid'] = $rsnids[$rs['nid']];
				}

				//处理标题长度
				if(intval($paramer[t_len])>0){
					$len = intval($paramer[t_len]);
					$rs[title_n] = $rs[title];
					$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
				}

				if($rs[color]){
					$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
				}

				//处理描述内容长度
				if(intval($paramer[d_len])>0){
					$len = intval($paramer[d_len]);
					$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
				}

				//获取所属类别名称
				$rs['name'] = $group_name[$rs['nid']];

				//构建资讯静态链接
				if($config[sy_news_rewrite]=="2"){
					$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
				}else{
					$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
				}

				if(mb_substr($rs[newsphoto],0,4)=="http"){
					$rs["picurl"]=$rs[newsphoto];
				}else{
					if($rs['newsphoto']==""){
						$rs["picurl"] = $config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";
					}else{
						$rs["picurl"] = checkpic($rs['newsphoto']);
					}
				}

				$rs[time]=date("Y-m-d",$rs[starttime]);
				$rs[starttime]=date("m-d",$rs[starttime]);
				if(count($alist[$rs['nid']]['arclist'])<$paramer[limit]){
					$alist[$rs['nid']]['arclist'][] = $rs;
				}
			}//end while

		}//end if(!intval($paramer['ispage']) && count($nids)>0)
		else{
			$query = $db->query("SELECT * FROM `$db_config[def]news_base` WHERE ".$where.$limit);

			while($rs = $db->fetch_array($query)){
				//处理标题长度
				if(intval($paramer[t_len])>0){
					$len = intval($paramer[t_len]);
					$rs[title_n] = $rs[title];
					$rs[title] = mb_substr($rs[title],0,$len,"utf-8");
				}

				if($rs[color]){
					$rs[title] = "<font color='".$rs[color]."'>".$rs[title]."</font>";
				}

				//处理描述内容长度
				if(intval($paramer[d_len])>0){
					$len = intval($paramer[d_len]);
					$rs[description] = mb_substr($rs[description],0,$len,"utf-8");
				}

				//获取所属类别名称
				 $rs['name'] = $group_name[$rs['nid']];

				//构建资讯静态链接
				if($config[sy_news_rewrite]=="2"){
					$rs["url"]=$config['sy_weburl']."/news/".date("Ymd",$rs["datetime"])."/".$rs[id].".html";
				}else{
					$rs["url"] = Url("article",array("c"=>"show","id"=>$rs[id]),"1");
				}

				if(mb_substr($rs[newsphoto],0,4)=="http"){
					$rs["picurl"]=$rs[newsphoto];
				}else{
					if($rs['newsphoto']==""){
						$rs["picurl"] = $config[sy_weburl]."/app/template/".$config[style]."/images/nopic.gif";
					}else{
						$rs["picurl"] = checkpic($rs['newsphoto']);
					}
				}

				$rs[time]=date("Y-m-d",$rs[starttime]);
				$rs[starttime]=date("m-d",$rs[starttime]);
				$alist[] = $rs;
			}//end while
		}$alist = $alist; if (!is_array($alist) && !is_object($alist)) { settype($alist, 'array');}
foreach ($alist as $_smarty_tpl->tpl_vars['alist']->key => $_smarty_tpl->tpl_vars['alist']->value) {
$_smarty_tpl->tpl_vars['alist']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['alist']->key;
?>
                    <li>
                        <span class="index_hotnews_n <?php if ($_smarty_tpl->tpl_vars['key']->value<3) {?>hot<?php echo $_smarty_tpl->tpl_vars['key']->value+1;
}?>"><?php echo $_smarty_tpl->tpl_vars['key']->value+1;?>
</span>
                        <a href="<?php echo $_smarty_tpl->tpl_vars['alist']->value['url'];?>
" title="<?php echo $_smarty_tpl->tpl_vars['alist']->value['title_n'];?>
"><?php echo $_smarty_tpl->tpl_vars['alist']->value['title'];?>
</a>
                    </li>
                    <?php } ?>
                </ul>
            </div>
        </div>
        <div class="yunheader_60lookmore"><a href="<?php echo smarty_function_url(array('m'=>'article'),$_smarty_tpl);?>
">查看更多</a></div>
    </div>

    <!-- 友情链接-->
    <div class="index_zl_box">
        <div class="index_link_box fl">
            <div class="new_index_tit">
                <span class="new_index_tit_list new_index_tit_cur">友情链接<i class="new_index_tit_line"></i></span>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_linksq']==1) {?> <span class="new_index_tit_list" data-no="1"><a href="<?php echo smarty_function_url(array('m'=>'link'),$_smarty_tpl);?>
" target="_blank" class=" "> 申请链接</a></span> <?php }?>
                <a href="<?php echo smarty_function_url(array('m'=>'link'),$_smarty_tpl);?>
" target="_blank" class="new_index_tit_more">查看更多</a>
            </div>
            <div class=" ">
                <div class="index_link_box_banner">
                    <?php  $_smarty_tpl->tpl_vars['linklist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['linklist']->_loop = false;
global $config;
		//跨域名使用范围
		$domain='';
		if($config["cityid"]!="" || $config["hyclass"]!=""){
			include(PLUS_PATH."/domain_cache.php");
			$host_url=$_SERVER['HTTP_HOST'];
			foreach($site_domain as $v){
				if($v['host']==$host_url){
					$domain=$v['id'];
				}
			}
		}$tem_type = 2;
        include PLUS_PATH."/link.cache.php";
		if(is_array($link)){$linkList=array();
			$i=0;
			foreach($link as $key=>$value)
			{
				if($config["did"]!=0 && $value["did"]!=-1 && $config["did"]!=-1 && $config["did"]!=$value["did"])
				{
					continue;
				}elseif(is_numeric('2') && $value['tem_type']!='2' && $value['tem_type']!='1'){
					continue;

				}elseif((!is_numeric('2') || '2'=='1') && $value['tem_type']!='1'){

					continue;
				}elseif(!$config["did"] && $value["did"]>0){
					continue;
				}
				if(is_numeric('2') && $value['link_type']!='2')
				{
					continue;
				}
				if(is_numeric('') && intval('')<=$i)
				{
					break;
				}
				if($value['pic']&&$value['img_type']==1){
					$value[pic] = checkpic($value['pic']);
				}
				$linkList[] = $value;
				$i++;
			}
		}$linkList = $linkList; if (!is_array($linkList) && !is_object($linkList)) { settype($linkList, 'array');}
foreach ($linkList as $_smarty_tpl->tpl_vars['linklist']->key => $_smarty_tpl->tpl_vars['linklist']->value) {
$_smarty_tpl->tpl_vars['linklist']->_loop = true;
?>
                    <a href="<?php echo $_smarty_tpl->tpl_vars['linklist']->value['link_url'];?>
" target="_blank">
                        <img src='<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/images/lay-loding.png' lay-src="<?php echo $_smarty_tpl->tpl_vars['linklist']->value['pic'];?>
" alt="<?php echo $_smarty_tpl->tpl_vars['linklist']->value['link_name'];?>
" width="160" height="50"/>
                    </a>
                    <?php } ?>
                </div>
                <div class="index_link_box_p">
                    <?php  $_smarty_tpl->tpl_vars['linklist2'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['linklist2']->_loop = false;
global $config;
		//跨域名使用范围
		$domain='';
		if($config["cityid"]!="" || $config["hyclass"]!=""){
			include(PLUS_PATH."/domain_cache.php");
			$host_url=$_SERVER['HTTP_HOST'];
			foreach($site_domain as $v){
				if($v['host']==$host_url){
					$domain=$v['id'];
				}
			}
		}$tem_type = 2;
        include PLUS_PATH."/link.cache.php";
		if(is_array($link)){$linkList=array();
			$i=0;
			foreach($link as $key=>$value)
			{
				if($config["did"]!=0 && $value["did"]!=-1 && $config["did"]!=-1 && $config["did"]!=$value["did"])
				{
					continue;
				}elseif(is_numeric('2') && $value['tem_type']!='2' && $value['tem_type']!='1'){
					continue;

				}elseif((!is_numeric('2') || '2'=='1') && $value['tem_type']!='1'){

					continue;
				}elseif(!$config["did"] && $value["did"]>0){
					continue;
				}
				if(is_numeric('1') && $value['link_type']!='1')
				{
					continue;
				}
				if(is_numeric('') && intval('')<=$i)
				{
					break;
				}
				if($value['pic']&&$value['img_type']==1){
					$value[pic] = checkpic($value['pic']);
				}
				$linkList[] = $value;
				$i++;
			}
		}$linkList = $linkList; if (!is_array($linkList) && !is_object($linkList)) { settype($linkList, 'array');}
foreach ($linkList as $_smarty_tpl->tpl_vars['linklist2']->key => $_smarty_tpl->tpl_vars['linklist2']->value) {
$_smarty_tpl->tpl_vars['linklist2']->_loop = true;
?>
                    <span class="index_link_box_p_name"><a href="<?php echo $_smarty_tpl->tpl_vars['linklist2']->value['link_url'];?>
" target="_blank"><?php echo $_smarty_tpl->tpl_vars['linklist2']->value['link_name'];?>
</a></span>
                    <?php } ?>
                </div>
            </div>
        </div>
    </div>
</div>

<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_footer_fix']=='1') {?>
<div id="tip_bottom" class="tip_bottom  none">
    <div class="tip_bottom_cont">
        <div class="tip_bottom_bg"></div>
        <div class="tip_bottom_cont_c">
            <div class="tip_bottom_main">
                <div class="tip_fot_user"></div>
                <div class="tip_bottom_left">
                    <a href="javascript:void(0);" onclick="$('.tip_bottom').hide();" class="tip_bottom_close png"></a>
                    <div class="tip_bottom_ewm">
                        <div class="tip_bottom_ewm_bg">
                            <i class="tip_bottom_ewm_p_icon"></i>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wx_qcode'];?>
" width="90" height="90">
                        </div>
                        <div class="tip_bottom_ewm_p">手机也能找工作</div>
                    </div>
                    <div class="tip_bottom_leftbox">
                        <span class="tip_bottom_logo"><h2>海量职位 让求职更简单</h2></span>
                        <div class="tip_bottom_num "><span>0</span>+企业的共同选择</div>
                        <div class="tip_bottom_num "><span>0</span>+ 高薪职位任您挑选</div>
                    </div>
                    <?php if (!$_smarty_tpl->tpl_vars['uid']->value) {?>
                    <div class="tip_bottom_member">
                        <a href="<?php echo smarty_function_url(array('m'=>'login'),$_smarty_tpl);?>
" class="tip_bottom_login">登录</a>
                        <a href="<?php echo smarty_function_url(array('m'=>'register'),$_smarty_tpl);?>
" class="tip_bottom_reg">快速注册<i class="tip_bottom_reg_icon"></i></a>
                    </div>
                    <?php }?>
                </div>
            </div>
        </div>
    </div>
</div>
<?php }?>

<div id="bg"></div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/backtop.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


<div id='footer_ad'>
    <?php  $_smarty_tpl->tpl_vars['footer_ad'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['footer_ad']->_loop = false;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"10","item"=>"“footer_ad“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[10];if(is_array($add_arr) && !empty($add_arr)){
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['footer_ad']->key => $_smarty_tpl->tpl_vars['footer_ad']->value) {
$_smarty_tpl->tpl_vars['footer_ad']->_loop = true;
?>
    <div class="footer_ban" id="">
        <div class="baner_footer" id='bottom_ad_fl'>
            <span class="ban_close" onclick="colse_bottom()">关闭</span>
            <?php echo $_smarty_tpl->tpl_vars['footer_ad']->value['lay_html'];?>

        </div>
        <input type="hidden" value='1' id='bottom_ad_is_show'/>
    </div>
    <?php } ?>

    <?php  $_smarty_tpl->tpl_vars['left_ad'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['left_ad']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"11","key"=>"“key“","item"=>"“left_ad“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[11];if(is_array($add_arr) && !empty($add_arr)){
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
foreach ($AdArr as $_smarty_tpl->tpl_vars['left_ad']->key => $_smarty_tpl->tpl_vars['left_ad']->value) {
$_smarty_tpl->tpl_vars['left_ad']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['left_ad']->key;
?>
    <div class="duilian <?php if ($_smarty_tpl->tpl_vars['key']->value=='0') {?>duilian_left<?php } else { ?>duilian_right<?php }?>">
        <div class="duilian_con"><?php echo $_smarty_tpl->tpl_vars['left_ad']->value['html'];?>
</div>
        <div class="close_container">
            <div class="btn_close"></div>
        </div>
    </div>
    <?php } ?>
</div>


<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/jquery/jquery_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/js/index.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/js/reg_ajax.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/slides.jquery.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/layui.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/phpyun_layer.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/swiper/swiper-bundle.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 type="text/javascript">
    $(function () {
        if ($("#adContent").has('a').length > 0) {
            $("#yhq_tip").css('display', 'block');
			$('#adContent').on("click",function(){
				$("#yhq_tip").hide();
			});
			$('#yhq_tip_close').click(function () {
				$("#yhq_tip").hide();
			});
        }
		
        $('.index_enterprise_re ul li').mouseover(function () {
            $(this).find(".tips_job_info").show()
        })
        $('.index_enterprise_re ul li').mouseout(function () {
            $(this).find(".tips_job_info").hide()
        })
    });

    // 名企
    if ($('.swiper-slide').length > 2) {
        var slideLoop = true;
    } else {
        var slideLoop = false;
    }
    new Swiper('#newurgentCtrl', {
        slidesPerView: 3,
        spaceBetween: 1,
        autoplay: true,
        loop: slideLoop
    });

    layui.use(['carousel', 'flow'], function () { //layui 轮播  test1 test2
        var carousel = layui.carousel;
        var flow = layui.flow;
        flow.lazyimg();
        carousel.render({
            elem: '#test1',
            width: '675px',
            height: '315px'
        });

        carousel.render({
            elem: '#test2',
            width: '290px',
            height: '190px',
            indicator: 'none' //指示器属性；隐藏：none，容器内部：inside，容器外部：outside；
        });
    });

    //顶部伸展广告
    if ($("#js_ads_banner_top_slide a").length > 0) { //判断当前广告是否展开，如果没有，则执行下面代码
        var $slidebannertop = $("#js_ads_banner_top_slide"),
            $bannertop = null;
        if ($("#js_ads_banner_top a").length > 0) {
            $bannertop = $("#js_ads_banner_top");
        }
        setTimeout(function () {
            if ($bannertop) {
                $bannertop.slideUp(1000);
            }
            $slidebannertop.slideDown(1000);
        }, 1500); //1500毫秒(1.5秒)后，小广告收回，大广告伸开，执行时间都是1秒(1000毫秒)
        setTimeout(function () {
            $slidebannertop.slideUp(1000, function () {
                if ($bannertop) {
                    $bannertop.slideDown(1000);
                }
            });
			// 防止因自动滚动，造成下面的懒加载广告图没出来
			setTimeout(function(){
				document.body.scrollTop = 1;
				document.documentElement.scrollTop = 1;
			},1100);
        }, 3000); //3.0秒(3000毫秒)之后，大广告收回，小广告展开。
    }else if ($("#js_ads_banner_top a").length > 0) {
        // 只有小广告，直接显示
        $("#js_ads_banner_top").removeClass('none');
    }

    $(document).ready(function () {
        //首页登录框以及登录后显示各会员中心内容
        var loginIndex = '<?php echo smarty_function_url(array('m'=>'ajax','c'=>'DefaultLoginIndex'),$_smarty_tpl);?>
';
        $.post(loginIndex, {
            rand: Math.random()
        }, function (data) {
            $(".hp_login").html(data);
        });
        //热招人才->推荐简历
        $(document.body).on('mouseenter', '.ran-list li', function () {
            $(this).addClass('show').find('.moren').hide();
            $(this).find('.lr-show').show();
            $(this).siblings().removeClass('show').find('.moren').show();
            $(this).siblings().find('.lr-show').hide();
        });
        var a = '<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_footer_fix']=="1") {?>';
        $.get('<?php echo smarty_function_url(array('m'=>'ajax','c'=>'footertj'),$_smarty_tpl);?>
', {
            rand: Math.random()
        }, function (data) {
            $(".tip_bottom_left").html(data);
            if ($('#tip_bottom')) {
                $('#tip_bottom').show();
            }

        })
        var b = '<?php }?>';
    });


    $('.new_index_tit span').each(function () {
        $(this).click(function () {
            if (!$(this).attr("data-no")) {
                $(this).parent().find("span").removeClass("new_index_tit_cur")
                $(this).addClass("new_index_tit_cur")
            }
            var id = $(this).attr("data-id");
            $(this).parent().parent().find(".index_zw_item").hide()
            $("#" + id).show()
        })
    })

<?php echo '</script'; ?>
>
<!--下面为调用网站主题-->
<?php echo smarty_function_webspecial(array(),$_smarty_tpl);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/verify/index_verify_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }} ?>
