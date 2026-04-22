<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:45
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/job.htm" */ ?>
<?php /*%%SmartyHeaderCode:174108473469e89635af25a4-49123804%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '786923ac1019bd1e2b0abc4289bed91f21c4ddf9' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/job.htm',
      1 => 1703143958,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '174108473469e89635af25a4-49123804',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'backurl' => 0,
    'wap_style' => 0,
    'searchurl' => 0,
    'config' => 0,
    'city_name' => 0,
    'job_name' => 0,
    'lunbo' => 0,
    'isweixin' => 0,
    'zd_list' => 0,
    'waflist' => 0,
    'job_list' => 0,
    'total' => 0,
    'blist' => 0,
    'plusstyle' => 0,
    'pagelink' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89635b47805_93159132',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89635b47805_93159132')) {function content_69e89635b47805_93159132($_smarty_tpl) {?><?php if (!is_callable('smarty_function_searchurl')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.searchurl.php';
if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


<div id="app">
    <div class="job_header">
        <a class="job_header_left" href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack()<?php }?>">
            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" style="width: 100%;" onclick="">
        </a>
        <div class="job_header_center">
            <input type="text" readonly class="searchnew" value="<?php if ($_GET['keyword']) {
echo $_GET['keyword'];
}?>" placeholder="<?php if ($_GET['keyword']) {
echo $_GET['keyword'];
} else { ?>搜索职位/公司<?php }?>">
        </div>
        <input type="hidden" id="searchurl" value="<?php echo $_smarty_tpl->tpl_vars['searchurl']->value;?>
" />
    </div>
    <div class="job_header_nav resumeAdeFlex">
        <div class="job_header_nav_left category">
            <ul>
                <li class="<?php if (!$_GET['urgent']&&!$_GET['mapjob']) {?>active<?php }?>">
                    <a href="<?php echo smarty_function_searchurl(array('m'=>'wap','c'=>'job','untype'=>'urgent,mapjob'),$_smarty_tpl);?>
">最新</a>
                </li>
                <li class="<?php if ($_GET['urgent']) {?>active<?php }?>">
                    <a href="<?php echo smarty_function_searchurl(array('m'=>'wap','c'=>'job','urgent'=>1,'untype'=>'mapjob'),$_smarty_tpl);?>
">急聘</a>
                </li>
                <li class="<?php if ($_GET['mapjob']) {?>active<?php }?>">
                    <a href="<?php echo smarty_function_searchurl(array('m'=>'wap','c'=>'job','mapjob'=>1,'untype'=>'urgent'),$_smarty_tpl);?>
">附近</a>
                </li>
            </ul>
        </div>
        <div class="job_header_nav_right">
            <ul>
                <li onclick="areashow();">
                    <?php if ($_smarty_tpl->tpl_vars['config']->value['three_cityid']&&$_smarty_tpl->tpl_vars['config']->value['sy_web_site']=='1') {?>
						<?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['config']->value['three_cityid']];?>

                    <?php } elseif ($_smarty_tpl->tpl_vars['config']->value['cityid']&&$_smarty_tpl->tpl_vars['config']->value['sy_web_site']=='1') {?>
						<?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['config']->value['cityid']];?>

                    <?php } else { ?>
						<?php if ($_smarty_tpl->tpl_vars['city_name']->value[$_GET['cityid']]||$_smarty_tpl->tpl_vars['city_name']->value[$_GET['provinceid']]) {?>
							<?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_GET['cityid']];
echo $_smarty_tpl->tpl_vars['city_name']->value[$_GET['provinceid']];?>

						<?php } elseif ($_smarty_tpl->tpl_vars['city_name']->value[$_GET['threecityid']]) {?>
							<?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_GET['threecityid']];?>

						<?php } else { ?>
							区域
						<?php }?>
                    <?php }?>
                    <i class="nav_right_open"></i>
                </li>
                <li onclick="jobshow();"><?php if ($_GET['job1']) {
echo $_smarty_tpl->tpl_vars['job_name']->value[$_GET['job1']];
} elseif ($_GET['job1son']) {
echo $_smarty_tpl->tpl_vars['job_name']->value[$_GET['job1son']];
} elseif ($_GET['jobpost']) {
echo $_smarty_tpl->tpl_vars['job_name']->value[$_GET['jobpost']];
} else { ?>职能<?php }?>
                    <i class="nav_right_open"></i>
                </li>
                <li onclick="jobmoreShow();">筛选
                    <i class="nav_right_open"></i>
                </li>
            </ul>
        </div>
    </div>
    <div class="main_part" style="padding-top: 2.4rem;">
		<!--广告-->
		<?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"504","item"=>"\"lunbo\"","key"=>"“key“","random"=>"1","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[504];if(is_array($add_arr) && !empty($add_arr)){
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
		<div class="jobzd_banner"><?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
</div>
		<?php } ?>
		<!--广告-->
        <?php if (($_smarty_tpl->tpl_vars['isweixin']->value&&$_smarty_tpl->tpl_vars['config']->value['sy_wxwap_list']==2)||$_GET['mapjob']) {?>
        <div id="listdiv" class="cont active"></div>
        <div id="pageLoading" class="lodbox none"><span class="lodbox_p"><i class="lodbox_iocn"></i>正在加载...</span></div>
        <div id="pageNoMore" class="lodbox none"> -没有更多了-</div>
        <?php } else { ?>
        <div id="listdiv" class="cont active">
            <?php if ($_GET['page']<2) {?> 
			<?php  $_smarty_tpl->tpl_vars['zd_list'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['zd_list']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("namelen"=>"10","comlen"=>"30","welfare"=>"“auto.welfare“","job1"=>"“auto.job1“","job1son"=>"“auto.job1son“","jobpost"=>"“auto.jobpost“","report"=>"“auto.report“","hy"=>"“auto.hy“","jobids"=>"“auto.jobids“","pr"=>"“auto.pr“","mun"=>"“auto.mun“","provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","threecityid"=>"“auto.threecityid“","type"=>"“auto.type“","edu"=>"“auto.edu“","exp"=>"“auto.exp“","sex"=>"“auto.sex“","minsalary"=>"“auto.minsalary“","maxsalary"=>"“auto.maxsalary“","keyword"=>"“auto.keyword“","cert"=>"“auto.cert“","urgent"=>"“auto.urgent“","rec"=>"“auto.rec“","bid"=>"1","uptime"=>"“auto.uptime“","key"=>"“key“","item"=>"“zd_list“","name"=>"“zdlist1“","nocache"=>"")
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
		
		$zd_list = $db->select_all("company_job",$where.$limit);

		if(is_array($zd_list) && !empty($zd_list)){
			$comuid=$jobid=array();
			foreach($zd_list as $key=>$value){
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
			foreach($zd_list as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($zd_list[$key]);
					continue;
				}else{
					$zd_list[$key] = $db->array_action($value,$cache_array);
					$zd_list[$key][stime] = date("Y-m-d",$value[sdate]);
					$zd_list[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$zd_list[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$zd_list[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$zd_list[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$zd_list[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $zd_list[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $zd_list[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $zd_list[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $zd_list[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $zd_list[$key][job_number] = "";
					}else{
					    $zd_list[$key][job_number] = $value[zp_num]." 人";
					}			
                    $zd_list[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $zd_list[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $zd_list[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$zd_list[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$zd_list[$key][pr_n] = $comclass_name[$value[pr]];
					$zd_list[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$zd_list[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$zd_list[$key]['time'] = $zd_list[$key]['lastupdate'];
						$zd_list[$key]['redtime'] =1;
					}else{
						$zd_list[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$zd_list[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$zd_list[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$zd_list[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$zd_list[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$zd_list[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$zd_list[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$zd_list[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$zd_list[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$zd_list[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$zd_list[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$zd_list[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$zd_list[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$zd_list[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$zd_list[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$zd_list[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$zd_list[$key][name_n]);
						$zd_list[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$zd_list[$key][com_n]);
						$zd_list[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$zd_list[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $zd_list[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $zd_list[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($zd_list)){
				if($paramer[keyword]!=""&&!empty($zd_list)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$zd_list = $zd_list; if (!is_array($zd_list) && !is_object($zd_list)) { settype($zd_list, 'array');}
foreach ($zd_list as $_smarty_tpl->tpl_vars['zd_list']->key => $_smarty_tpl->tpl_vars['zd_list']->value) {
$_smarty_tpl->tpl_vars['zd_list']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['zd_list']->key;
?> <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['zd_list']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['zd_list']->value['name'];?>
">
                <div class="tab_card">
                    <!--实地已核验-->
                     <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['fact_status']=='1') {?>
                     <div class="ptyhybox">
                     <div class="ptyhy">
                     <i class="ptyhy_icon"></i>实地核验</div></div> 
                     <?php }?><!--实地已核验-->
                    <div class="tab_card_top">
                        <div class="tab_card_job">
                            <i class="tab_card_job_name"><?php echo $_smarty_tpl->tpl_vars['zd_list']->value['name_n'];?>
</i>
                            <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['newtime']==1) {?><i class="tab_card_new">new</i><?php }?>
                        </div>
                        <i class="tab_card_pay"><?php echo $_smarty_tpl->tpl_vars['zd_list']->value['job_salary'];?>
</i>
                    </div>
                    <div class="newjob_info">
                        <span class="">
                            <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['job_city_three']) {?>
                            <?php echo $_smarty_tpl->tpl_vars['zd_list']->value['job_city_three'];?>

                            <?php } elseif ($_smarty_tpl->tpl_vars['zd_list']->value['job_city_two']) {?>
                            <?php echo $_smarty_tpl->tpl_vars['zd_list']->value['job_city_two'];?>

                            <?php } else { ?>
                            <?php echo $_smarty_tpl->tpl_vars['zd_list']->value['job_city_one'];?>

                            <?php }?>
                        </span>
                        <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['job_exp']) {?>
                        <i class="newjob_info_line"></i><span class=""><?php echo $_smarty_tpl->tpl_vars['zd_list']->value['job_exp'];?>
经验</span>
                        <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['job_edu']) {?><i class="newjob_info_line"></i>
                        <span class=""><?php echo $_smarty_tpl->tpl_vars['zd_list']->value['job_edu'];?>
学历</span>
                        <?php }?>
                        <span class="newjob_fw">
                            <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['rec']=='1'&&$_smarty_tpl->tpl_vars['zd_list']->value['rec_time']>time()) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_recommend.png" alt="">
                            <?php }?>
                            <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['urgent_time']>time()) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jp.png" alt="">
                            <?php }?>
                        </span>
                    </div>
                    <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['welfarename']) {?>
                    <div class="welfare"><?php  $_smarty_tpl->tpl_vars['waflist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['waflist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['zd_list']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['waflist']->key => $_smarty_tpl->tpl_vars['waflist']->value) {
$_smarty_tpl->tpl_vars['waflist']->_loop = true;
?>
                        <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['waflist']->value;?>
</span><?php } ?>
                    </div>
                    <?php }?>
                    <div class="tab_card_bottom">
                        <div class="card_bottom_logo">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['zd_list']->value['logo'];?>
" alt="" style="width: 100%;">
                        </div>
                        <i class="card_bottom_word"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['zd_list']->value['com_name']),0,14,'utf-8');?>
</i>
						<?php if ($_smarty_tpl->tpl_vars['zd_list']->value['ratlogo']!=''&&$_smarty_tpl->tpl_vars['zd_list']->value['ratlogo']!="0") {
if ($_smarty_tpl->tpl_vars['zd_list']->value['hotlogo']==1) {?> <img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/template/wap/images/mq.png" alt="名企" class="png" width="14" style="vertical-align:middle"><?php }?><img src="<?php echo $_smarty_tpl->tpl_vars['zd_list']->value['ratlogo'];?>
" style="vertical-align:middle; margin-left:3px;" width="14" height="14" /> <?php }?> <?php if ($_smarty_tpl->tpl_vars['zd_list']->value['yyzz_status']=='1') {?>
						<i class="job_qy_rz_icon"></i> <?php }?>
                        <div>
                            <i class="zdnow">
                                置顶
                            </i>
                        </div>
                    </div>
                </div>
                </a>
                <?php } ?>
                <?php }?>
                <?php  $_smarty_tpl->tpl_vars['job_list'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['job_list']->_loop = false;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("noids"=>"1","namelen"=>"10","comlen"=>"15","ispage"=>"1","welfare"=>"“auto.welfare“","job1"=>"“auto.job1“","job1son"=>"“auto.job1son“","jobpost"=>"“auto.jobpost“","hy"=>"“auto.hy“","pr"=>"“auto.pr“","mun"=>"“auto.mun“","provinceid"=>"“auto.provinceid“","threecityid"=>"“auto.threecityid“","cityid"=>"“auto.cityid“","type"=>"“auto.type“","edu"=>"“auto.edu“","exp"=>"“auto.exp“","sex"=>"“auto.sex“","minsalary"=>"“auto.minsalary“","maxsalary"=>"“auto.maxsalary“","keyword"=>"“auto.keyword“","rec"=>"“auto.rec“","urgent"=>"“auto.urgent“","uptime"=>"“auto.uptime“","limit"=>"20","item"=>"“job_list“","islt"=>"4","nocache"=>"")
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
                <div class="tab_card">
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['job_list']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['name'];?>
">
                        <?php if ($_smarty_tpl->tpl_vars['job_list']->value['fact_status']=='1') {?> <div class="ptyhybox">
                            <div class="ptyhy">
                            <i class="ptyhy_icon"></i>实地核验</div></div>  <?php }?><!--实地已核验-->
                        <div class="tab_card">
                            <div class="tab_card_top">
                                <div class="tab_card_job">
                                    <i class="tab_card_job_name"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['name_n'];?>
</i>
                                    <?php if ($_smarty_tpl->tpl_vars['job_list']->value['newtime']==1) {?><i class="tab_card_new">new</i><?php }?>
                                </div>
                                <i class="tab_card_pay"><?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_salary'];?>
</i>
                            </div>
                            <div class="newjob_info">
                                <span class="">
                                    <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_city_three']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_city_three'];?>

                                    <?php } elseif ($_smarty_tpl->tpl_vars['job_list']->value['job_city_two']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_city_two'];?>

                                    <?php } else { ?>
                                    <?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_city_one'];?>

                                    <?php }?>
                                </span>
                                <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_exp']) {?>
                                <i class="newjob_info_line"></i><span class=""><?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_exp'];?>
经验</span>
                                <?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['job_list']->value['job_edu']) {?><i class="newjob_info_line"></i>
                                <span class=""><?php echo $_smarty_tpl->tpl_vars['job_list']->value['job_edu'];?>
学历</span>
                                <?php }?>
                                <span class="newjob_fw">
                                    <?php if ($_smarty_tpl->tpl_vars['job_list']->value['rec']=='1'&&$_smarty_tpl->tpl_vars['job_list']->value['rec_time']>time()) {?>
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_recommend.png" alt="">
                                    <?php }?>
                                    <?php if ($_smarty_tpl->tpl_vars['job_list']->value['urgent_time']>time()) {?>
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jp.png" alt="">
                                    <?php }?>
                                </span>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['job_list']->value['welfarename']) {?>
                            <div class="welfare"><?php  $_smarty_tpl->tpl_vars['waflist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['waflist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['job_list']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['waflist']->key => $_smarty_tpl->tpl_vars['waflist']->value) {
$_smarty_tpl->tpl_vars['waflist']->_loop = true;
?>
                                <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['waflist']->value;?>
</span><?php } ?>
                            </div>
                            <?php }?>
                            <div class="tab_card_bottom">
                                <div class="card_bottom_logo">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['logo'];?>
" alt="" style="width: 100%;">
                                </div>
                                <div class="card_bottom_word"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['job_list']->value['com_name']),0,14,'utf-8');?>
</div>
                                <?php if ($_smarty_tpl->tpl_vars['job_list']->value['ratlogo']!=''&&$_smarty_tpl->tpl_vars['job_list']->value['ratlogo']!="0") {
if ($_smarty_tpl->tpl_vars['job_list']->value['hotlogo']==1) {?> <img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/template/wap/images/mq.png" alt="名企" class="png" width="14" style="vertical-align:middle"><?php }?><img src="<?php echo $_smarty_tpl->tpl_vars['job_list']->value['ratlogo'];?>
" style="vertical-align:middle; margin-left:3px;" width="14" height="14" /> <?php }?> <?php if ($_smarty_tpl->tpl_vars['job_list']->value['yyzz_status']=='1') {?>
                                <i class="job_qy_rz_icon"></i> <?php }?>
                                <i class="card_bottom_logo_word">
                                    <?php if ($_smarty_tpl->tpl_vars['job_list']->value['time']=='今天'||$_smarty_tpl->tpl_vars['job_list']->value['time']=='昨天'||$_smarty_tpl->tpl_vars['job_list']->value['redtime']=='1') {?>
                                    <?php echo $_smarty_tpl->tpl_vars['job_list']->value['time'];?>

                                    <?php } else { ?>
                                    <?php echo $_smarty_tpl->tpl_vars['job_list']->value['time'];?>

                                    <?php }?>
                                </i>
                            </div>
                        </div>
                    </a>
                </div>
                <?php } ?>
        </div>
		<?php if ($_smarty_tpl->tpl_vars['total']->value<=0) {?> 
			<?php if ($_GET['keyword']!='') {?> 
			<div class="wap_member_no">没有搜索到职位</div>
			<?php } else { ?>
			<div class="wap_member_no">很抱歉,这个星球没有职位呢！</div>
			<?php }?>
			<div class="wap_member_no_submit">
				<a class="wap_mb_no_sr" href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job'),$_smarty_tpl);?>
">重新搜索</a>
			</div>
		<?php } else { ?>
            <?php if ($_smarty_tpl->tpl_vars['total']->value>20) {?>
    		<van-pagination v-model="currentPage" :total-items="total" :items-per-page="perpage" force-ellipses @change="pageChange" show-page-size="3" />
    		<?php }?>
		<?php }?>
        <!--为你推荐 样式需要添加-->
        <?php if ($_smarty_tpl->tpl_vars['total']->value<=0&&!$_GET['rec']) {?> <section id="muirecjob" class=" ">
            <div class="newjob_tj ">
                <span class=" ">- 为你推荐职位 -</span>
            </div>
            <?php  $_smarty_tpl->tpl_vars['blist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['blist']->_loop = false;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("namelen"=>"15","comlen"=>"19","rec"=>"1","limit"=>"10","item"=>"“blist“","nocache"=>"")
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
		
		$blist = $db->select_all("company_job",$where.$limit);

		if(is_array($blist) && !empty($blist)){
			$comuid=$jobid=array();
			foreach($blist as $key=>$value){
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
			foreach($blist as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($blist[$key]);
					continue;
				}else{
					$blist[$key] = $db->array_action($value,$cache_array);
					$blist[$key][stime] = date("Y-m-d",$value[sdate]);
					$blist[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$blist[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$blist[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$blist[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$blist[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $blist[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $blist[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $blist[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $blist[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $blist[$key][job_number] = "";
					}else{
					    $blist[$key][job_number] = $value[zp_num]." 人";
					}			
                    $blist[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $blist[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $blist[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$blist[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$blist[$key][pr_n] = $comclass_name[$value[pr]];
					$blist[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$blist[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$blist[$key]['time'] = $blist[$key]['lastupdate'];
						$blist[$key]['redtime'] =1;
					}else{
						$blist[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$blist[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$blist[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$blist[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$blist[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$blist[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$blist[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$blist[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$blist[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$blist[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$blist[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$blist[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$blist[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$blist[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$blist[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$blist[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$blist[$key][name_n]);
						$blist[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$blist[$key][com_n]);
						$blist[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$blist[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $blist[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $blist[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($blist)){
				if($paramer[keyword]!=""&&!empty($blist)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$blist = $blist; if (!is_array($blist) && !is_object($blist)) { settype($blist, 'array');}
foreach ($blist as $_smarty_tpl->tpl_vars['blist']->key => $_smarty_tpl->tpl_vars['blist']->value) {
$_smarty_tpl->tpl_vars['blist']->_loop = true;
?>
            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['blist']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['blist']->value['name_n'];?>
">
                <div class="tab_card">
                    <?php if ($_smarty_tpl->tpl_vars['blist']->value['fact_status']=='1') {?>
            <div class="ptyhybox">
            <div class="ptyhy">
                    <i class="ptyhy_icon"></i>实地核验 </div> </div> <?php }?><!--实地已核验-->
                    <div class="tab_card_top">
                        <div class="tab_card_job"><i class="tab_card_job_name"><?php echo $_smarty_tpl->tpl_vars['blist']->value['name_n'];?>
 </i></div>
                        <i class="tab_card_pay"><?php if ($_smarty_tpl->tpl_vars['blist']->value['job_salary']!='面议') {
}
echo $_smarty_tpl->tpl_vars['blist']->value['job_salary'];?>
</i>
                    </div>
                    <div class="newjob_info">
                        <span>
                            <?php if ($_smarty_tpl->tpl_vars['blist']->value['job_city_three']) {?>
                            <?php echo $_smarty_tpl->tpl_vars['blist']->value['job_city_three'];?>

                            <?php } elseif ($_smarty_tpl->tpl_vars['blist']->value['job_city_two']) {?>
                            <?php echo $_smarty_tpl->tpl_vars['blist']->value['job_city_two'];?>

                            <?php } else { ?>
                            <?php echo $_smarty_tpl->tpl_vars['blist']->value['job_city_one'];?>

                            <?php }?>
                        </span> <i class="newjob_info_line"></i><span><?php echo $_smarty_tpl->tpl_vars['blist']->value['job_exp'];?>
经验</span> <i class="newjob_info_line"></i> <span><?php echo $_smarty_tpl->tpl_vars['blist']->value['job_edu'];?>
学历</span>
                        <span class="newjob_fw">
                            <?php if ($_smarty_tpl->tpl_vars['blist']->value['rec']=='1'&&$_smarty_tpl->tpl_vars['blist']->value['rec_time']>time()) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/icon_recommend.png" alt="">
                            <?php }?>
                            <?php if ($_smarty_tpl->tpl_vars['blist']->value['urgent_time']>time()) {?>
                            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/jp.png" alt="">
                            <?php }?>
                        </span>
                    </div>
                    <?php if ($_smarty_tpl->tpl_vars['blist']->value['welfarename']) {?>
                    <div class="welfare">
                        <?php  $_smarty_tpl->tpl_vars['waflist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['waflist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['blist']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['waflist']->key => $_smarty_tpl->tpl_vars['waflist']->value) {
$_smarty_tpl->tpl_vars['waflist']->_loop = true;
?>
                        <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['waflist']->value;?>
</span>
                        <?php } ?>
                    </div>
                    <?php } else { ?>
                    <?php }?>
                    <div class="tab_card_bottom">
                        <div class="card_bottom_logo">
                            <img src="<?php echo $_smarty_tpl->tpl_vars['blist']->value['logo'];?>
" style="width: 100%;">
                        </div>
                        <i class="card_bottom_word"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['blist']->value['com_name']),0,20,'utf-8');?>
 </i>
                        <?php if ($_smarty_tpl->tpl_vars['blist']->value['ratlogo']!=''&&$_smarty_tpl->tpl_vars['blist']->value['ratlogo']!="0") {?>
                        <img src="<?php echo $_smarty_tpl->tpl_vars['blist']->value['ratlogo'];?>
" style="vertical-align:middle" width="14" height="14" /> <?php }?>
                        <?php if ($_smarty_tpl->tpl_vars['blist']->value['yyzz_status']=='1') {?>
                        <i class="job_qy_rz_icon"></i>
                        <?php }?>
                        <div>
                            <i class="card_bottom_logo_word">
                                <?php if ($_smarty_tpl->tpl_vars['job_list']->value['time']=='今天'||$_smarty_tpl->tpl_vars['job_list']->value['time']=='昨天'||$_smarty_tpl->tpl_vars['job_list']->value['redtime']=='1') {?>
                                <?php echo $_smarty_tpl->tpl_vars['job_list']->value['time'];?>

                                <?php } else { ?>
                                <?php echo $_smarty_tpl->tpl_vars['job_list']->value['time'];?>

                                <?php }?></i>
                        </div>
                    </div>
                </div>
            </a>
            <div class="job_list_box">
                <div class="yunwap_jobcom">
                    <div class="yunwap_jobcom_name">
                        <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'company','a'=>'show','id'=>$_smarty_tpl->tpl_vars['blist']->value['uid']),$_smarty_tpl);?>
" title="<?php echo preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['blist']->value['com_name']);?>
">
                        </a>
                    </div>
                </div>
            </div>
            <?php } ?>
            </section>
            <?php }?>
            <?php }?>
    </div>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['plusstyle']->value;?>
/city.cache.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['plusstyle']->value;?>
/job.cache.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/category.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_city_search.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 type="text/javascript">
var wapurl = "<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
";
var mapjob = '<?php echo $_GET['mapjob'];?>
';
var currentPage = parseInt('<?php echo $_GET['page'];?>
'),
    total = parseInt('<?php echo $_smarty_tpl->tpl_vars['total']->value;?>
'),
    pagelink = '<?php echo $_smarty_tpl->tpl_vars['pagelink']->value;?>
';

var jobData = jobCategory();


var joblistvue = new Vue({
    el: '#app',
    data: {
        //分页相关
        currentPage: currentPage ? currentPage : 1,
        total: total,
        perpage: 20,
    },
    methods: {
        pageChange: function(e) {
            var pageurl = pagelink.replace('{{page}}', e);
            location.href = pageurl;
        }
    }
})
	
<?php echo '</script'; ?>
>

<?php if (($_smarty_tpl->tpl_vars['isweixin']->value&&$_smarty_tpl->tpl_vars['config']->value['sy_wxwap_list']==2)||$_GET['mapjob']) {?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/joblist_vue.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/publictwo.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/search_new.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
