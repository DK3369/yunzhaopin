<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:38
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:13513709069e8962e767541-05297207%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'ed28312d08afd3f9a6fbb091949c995d52521546' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/index.htm',
      1 => 1709532472,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '13513709069e8962e767541-05297207',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'config' => 0,
    'uid' => 0,
    'expect' => 0,
    'tplmoblie' => 0,
    'username' => 0,
    'lunbo' => 0,
    'navlist' => 0,
    'key' => 0,
    'v' => 0,
    'annum' => 0,
    'alist' => 0,
    'resume_yhnum' => 0,
    'hotclass' => 0,
    'item' => 0,
    'hotjoblist' => 0,
    'zdjob' => 0,
    'njk' => 0,
    'newjob' => 0,
    'urgjob' => 0,
    'ujk' => 0,
    'recjob' => 0,
    'rjk' => 0,
    'kfurl' => 0,
    'isweixin' => 0,
    'bannerFlag' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8962e7b1613_02619770',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8962e7b1613_02619770')) {function content_69e8962e7b1613_02619770($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>


<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/swiper/swiper.min.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet"/>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/swiper/swiper.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<div class="" id="app">
    
    <!-- 页面头部部分 -->

    <!--隐藏简历提示-->
    <?php if ($_smarty_tpl->tpl_vars['uid']->value&&$_smarty_tpl->tpl_vars['expect']->value['status']=='2') {?>
    <div id="privacyCtrl" class="hide_tip">
        <div class="resume_hint_eye_n">
            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/conceal_1.png" alt="" width="100%" height="100%">
        </div>
        <i class="resume_hint_word_color"> 简历已隐藏，</i>
       <i class="resume_hint_word_black"> 企业无法主动发现你</i>
        <a href="javascript:void(0);" onclick="privacy();" class="hide_tip_qx">取消隐藏</a>
        <i class="hide_tip_gb"></i>
    </div>
    <?php }?>

    <div class="clear"></div>

    <div class="yunTop">
        <div class="yunlogobox">
            <?php if ($_smarty_tpl->tpl_vars['tplmoblie']->value['logo']==2) {?>
                <div class="header_p_z"> <?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
</div>
            <?php } else { ?>
                <img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wap_logo'];?>
" alt=" " title=" " class="yunlogo">
            <?php }?>
			<!--
            <?php if (!$_smarty_tpl->tpl_vars['username']->value&&$_GET['c']=='') {?>
                <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
" class="yunTopUp_right"> 发布简历</a>
            <?php } else { ?>
                <a href="<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
member" id="memberclick" class="yunTopUp_right">用户中心</a>
            <?php }?>-->
        </div>

        <div class="index_newedition_search_box">
            <div class="index_newedition_searchbg">
                <div class="index_newedition_search_c">

                    <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_web_site']=='1') {?>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'site'),$_smarty_tpl);?>
" class="index_newedition_search_city"><?php if ($_smarty_tpl->tpl_vars['config']->value['cityname']) {
echo $_smarty_tpl->tpl_vars['config']->value['cityname'];
} else {
echo $_smarty_tpl->tpl_vars['config']->value['sy_indexcity'];
}?></a>
                    <?php }?>

                    <span class="index_newedition_search_p searchnew" style="<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_web_site']=='1') {?>width:75%<?php } else { ?>width:95%<?php }?>">搜职位/公司</span>
                    <span class="index_newedition_searchbth"> </span>
                </div>
            </div>
        </div>
    </div>

    <div class="clear"></div>

    <!-- 页面主体部分 -->
    <div class="index_body">
        <!-- 轮播图部分 -->
        <div class="banner">
            <!-- 轮播图 -->
            <div class="roll">
                <div class="swiper-container" id="imgswiper" style="transform:translate3d(0,0,0);overflow:hidden;">
                    <div class="swiper-wrapper" >
                        <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"50","item"=>"\"lunbo\"","key"=>"“key“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[50];if(is_array($add_arr) && !empty($add_arr)){
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
                        <div class="swiper-slide" style="transform:translate3d(0,0,0);">
                            <a href="<?php echo $_smarty_tpl->tpl_vars['lunbo']->value['src'];?>
">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['lunbo']->value['pic'];?>
" style="border-radius: 6px;width:100%;height:3.2rem"/>
                            </a>
                        </div>
                        <?php } ?>
                    </div>
                </div>
            </div>

            <div class="job">
                <!--金刚位 -->
                <div class="swiper-container navbox_jgw" id="navswiper">
                    <div class="swiper-wrapper">
                        <div class="swiper-slide">
                            <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['navlist']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                            <?php if ($_smarty_tpl->tpl_vars['key']->value>0&&$_smarty_tpl->tpl_vars['key']->value%4==0) {?>
                        </div>
                        <div class="swiper-slide">
                            <?php }?>
                            <a href="<?php echo $_smarty_tpl->tpl_vars['v']->value['url_n'];?>
">
                                <div class="full-time">
                                    <div class="full-time-logo">
                                        <img src="<?php echo $_smarty_tpl->tpl_vars['v']->value['pic_n'];?>
" alt="" style="width: 100%;">
                                    </div>
                                    <div class="full-time-word"><?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
</div>
                                </div>
                            </a>
                            <?php } ?>
                        </div>
                    </div>
                    <div class="swiper-pagination navbox_fyq"></div>
                </div>
            </div>
            <?php if ($_smarty_tpl->tpl_vars['annum']->value) {?>
            <div class="inform">
                <div class="inform-trumpet">
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'announcement'),$_smarty_tpl);?>
"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/home_icon_notice.png" alt="" style="width: 100%;"></a>
                </div>
                <div class="swiper-container " id="ggswiper" style="margin-top:0.08rem">
                    <div class="swiper-wrapper">
                        <?php  $_smarty_tpl->tpl_vars['alist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['alist']->_loop = false;
$alist=array();$time=time();$paramer=array("limit"=>"10","item"=>"“alist“","t_len"=>"40","nocache"=>"")
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

		$alist=$db->select_all("admin_announcement",$where.$limit);
		if(is_array($alist)){
			foreach($alist as $key=>$value){
				//截取标题
				if($paramer[t_len]){
					$alist[$key][title_n] = mb_substr($value['title'],0,$paramer[t_len],"utf-8");
				}
				$alist[$key][time]=date("Y-m-d",$value[startime]);
				$alist[$key][url] = Url("announcement",array("id"=>$value[id]),"1");
			}
		}$alist = $alist; if (!is_array($alist) && !is_object($alist)) { settype($alist, 'array');}
foreach ($alist as $_smarty_tpl->tpl_vars['alist']->key => $_smarty_tpl->tpl_vars['alist']->value) {
$_smarty_tpl->tpl_vars['alist']->_loop = true;
?>
                        <div class="swiper-slide">
                            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'announcement','id'=>$_smarty_tpl->tpl_vars['alist']->value['id']),$_smarty_tpl);?>
" style="color:#666;height:0.64rem;"><i class="inform-word conceal_word" ><?php echo $_smarty_tpl->tpl_vars['alist']->value['title_n'];?>
</i></a>
                        </div>
                        <?php } ?>
                    </div>
                </div>
            </div>
            <?php }?>
        </div>

        <!-- 简历优化提示 -->
        <?php if ($_smarty_tpl->tpl_vars['resume_yhnum']->value>0) {?>
        <div id="resume_yh" class="optimize_tip_box none">
            <div class="optimize_tip">
                <i class="optimize_tipicon"></i>
                <i class="optimize_tipgbicon" onclick="resume_yhhide();"></i>
                <div class="optimize_name">您的简历有<?php echo $_smarty_tpl->tpl_vars['resume_yhnum']->value;?>
个可优化项</div>
                <div class="optimize_p">处理后可大幅提升求职成功率</div>
                <a class="optimize_tip_bth" href="<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
member/index.php?c=resume&eid=<?php echo $_smarty_tpl->tpl_vars['expect']->value['id'];?>
">去处理</a>
            </div>
        </div>
        <?php }?>
          <!-- 登陆注册 -->
		  <?php if (!$_smarty_tpl->tpl_vars['username']->value&&$_GET['c']=='') {?>
		   <div class="indexlogin_bth">
			   
			   <div class="indexlogin_bth_c">
		   <div class="indexlogin_list">
			     <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
"  class="indexlogin_listc indexlogin_listcr">
			   <i class="indexlogin_icon"></i>
			   <div class="indexlogin_name">发布简历	  </div>
			   <div class="indexlogin_p">找喜欢的工作	  </div>
		  </a>
		  </div>
		     <div class="indexlogin_list">
				    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
" class="indexlogin_listc indexlogin_listcl">
				   <i class="indexlogin_icon indexlogin_icon2"></i>
		  <div class="indexlogin_name">发布职位	  </div>
		  <div class="indexlogin_p">招优秀人才	  </div>  </a> </div>
		 
		   </div>
		   </div>

              
            <?php } else { ?>
             
            <?php }?>
			<!--职位关键字-->
			<?php if ($_smarty_tpl->tpl_vars['hotclass']->value) {?>			
	        <div class="new_mq"> 
			     <i class="new_mq_name">热门职位</i>
				<a class="new_mq_more" href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job'),$_smarty_tpl);?>
">更多 ></a> 
	           <div class="index_jobtagbox">
                <?php  $_smarty_tpl->tpl_vars['item'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['item']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['hotclass']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['item']->key => $_smarty_tpl->tpl_vars['item']->value) {
$_smarty_tpl->tpl_vars['item']->_loop = true;
?>                
    			<div class="index_jobtaglist"><a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','job1'=>$_smarty_tpl->tpl_vars['item']->value['job1'],'job1son'=>$_smarty_tpl->tpl_vars['item']->value['job1_son'],'jobpost'=>$_smarty_tpl->tpl_vars['item']->value['job_post']),$_smarty_tpl);?>
"><span class="index_jobtag_n"><?php echo $_smarty_tpl->tpl_vars['item']->value['name'];?>
</span></a></div>
    			<?php } ?>
    			 </div>
		    </div>
            <?php }?>
			<!--职位关键字 end-->
		     <!-- 名企招聘 new-->
			<div class="new_mq ">
			     <i class="new_mq_name">名企招聘</i>
			     <a class="new_mq_more" href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'company','rec'=>1),$_smarty_tpl);?>
">更多 ></a>
			  	<div class="new_mq_new_show">
			 		<div class="swiper-container navbox_jgw" id="mqswiper" style="height:3.64rem">
			 		    <div class="swiper-wrapper">
			 		        <div class="swiper-slide">
								<?php  $_smarty_tpl->tpl_vars['hotjoblist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['hotjoblist']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$paramer=array("item"=>"“hotjoblist“","key"=>"“key“","limit"=>"12","nocache"=>"")
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
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['hotjoblist']->key;
?>
			 		            <?php if ($_smarty_tpl->tpl_vars['key']->value>0&&$_smarty_tpl->tpl_vars['key']->value%4==0) {?>
			 		        </div>
			 		        <div class="swiper-slide">
			 		            <?php }?>
			 		            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'company','a'=>'show','id'=>$_smarty_tpl->tpl_vars['hotjoblist']->value['uid']),$_smarty_tpl);?>
" class="mqnewlist">
			 		                <div>
										<div class="mqnew">
			 		                    <div class="mqnewimg">
											<img src="<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['hot_pic'];?>
" alt="" style="width: 100%;">
										</div>
										<div class="mqnew_comname">
											<?php echo mb_substr($_smarty_tpl->tpl_vars['hotjoblist']->value['username'],0,12,'utf-8');?>

										</div> 
									<div class="mqnew_comjob">
										<?php echo $_smarty_tpl->tpl_vars['hotjoblist']->value['num'];?>
个岗位
									</div></div></div>
			 		            </a>
			 		           <?php } ?>
			 		        </div>
			 		    </div>
			 		</div>
			  	</div> 
			  </div>
        <!-- 广告 -->
        <?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"503","item"=>"\"lunbo\"","key"=>"“key“","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[503];if(is_array($add_arr) && !empty($add_arr)){
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
        <div class="zd_banner"><?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
</div>
        <?php } ?>
						
        <!-- tab栏切换部分 -->
        <div id="yunvue" class="tab none">
            <van-tabs color="#2778F8" @click="chooseTab">
                <van-tab title="最新">
					<?php  $_smarty_tpl->tpl_vars['zdjob'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['zdjob']->_loop = false;
 $_smarty_tpl->tpl_vars['njk'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","key"=>"“njk“","item"=>"“zdjob“","istop"=>"1","nocache"=>"")
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
		
		$zdjob = $db->select_all("company_job",$where.$limit);

		if(is_array($zdjob) && !empty($zdjob)){
			$comuid=$jobid=array();
			foreach($zdjob as $key=>$value){
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
			foreach($zdjob as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($zdjob[$key]);
					continue;
				}else{
					$zdjob[$key] = $db->array_action($value,$cache_array);
					$zdjob[$key][stime] = date("Y-m-d",$value[sdate]);
					$zdjob[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$zdjob[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$zdjob[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$zdjob[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$zdjob[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $zdjob[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $zdjob[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $zdjob[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $zdjob[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $zdjob[$key][job_number] = "";
					}else{
					    $zdjob[$key][job_number] = $value[zp_num]." 人";
					}			
                    $zdjob[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $zdjob[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $zdjob[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$zdjob[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$zdjob[$key][pr_n] = $comclass_name[$value[pr]];
					$zdjob[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$zdjob[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$zdjob[$key]['time'] = $zdjob[$key]['lastupdate'];
						$zdjob[$key]['redtime'] =1;
					}else{
						$zdjob[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$zdjob[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$zdjob[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$zdjob[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$zdjob[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$zdjob[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$zdjob[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$zdjob[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$zdjob[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$zdjob[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$zdjob[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$zdjob[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$zdjob[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$zdjob[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$zdjob[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$zdjob[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$zdjob[$key][name_n]);
						$zdjob[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$zdjob[$key][com_n]);
						$zdjob[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$zdjob[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $zdjob[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $zdjob[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($zdjob)){
				if($paramer[keyword]!=""&&!empty($zdjob)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$zdjob = $zdjob; if (!is_array($zdjob) && !is_object($zdjob)) { settype($zdjob, 'array');}
foreach ($zdjob as $_smarty_tpl->tpl_vars['zdjob']->key => $_smarty_tpl->tpl_vars['zdjob']->value) {
$_smarty_tpl->tpl_vars['zdjob']->_loop = true;
 $_smarty_tpl->tpl_vars['njk']->value = $_smarty_tpl->tpl_vars['zdjob']->key;
?>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['zdjob']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['zdjob']->value['name'];?>
">

                        <div class="table-card" style="<?php if ($_smarty_tpl->tpl_vars['njk']->value==0) {?>margin-top: .3rem;<?php }?>">
                            <div class="card_post">
                                <i class="table-card-word"><?php echo $_smarty_tpl->tpl_vars['zdjob']->value['name'];?>
</i>
                                <i class="table-card-salary"><?php echo $_smarty_tpl->tpl_vars['zdjob']->value['job_salary'];?>
</i>
                            </div>
                            <div class="table-card-require">
                                <i class="requir-area">
                                    <?php if ($_smarty_tpl->tpl_vars['zdjob']->value['job_city_three']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['zdjob']->value['job_city_three'];?>

                                    <?php } elseif ($_smarty_tpl->tpl_vars['zdjob']->value['job_city_two']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['zdjob']->value['job_city_two'];?>

                                    <?php } else { ?>
                                    <?php echo $_smarty_tpl->tpl_vars['zdjob']->value['job_city_one'];?>

                                    <?php }?>
                                </i>
                                <i class="requir_area_parting_line"></i>
                                <?php if ($_smarty_tpl->tpl_vars['zdjob']->value['job_edu']) {?><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['zdjob']->value['job_edu'];?>
学历</i><?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['zdjob']->value['job_exp']) {?> <i class="requir_area_parting_line"></i><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['zdjob']->value['job_exp'];?>
经验</i><?php }?>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['zdjob']->value['welfarename']) {?>
                            <div class="welfare">
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['zdjob']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</span>
                                <?php } ?>
                            </div>
                            <?php }?>
                            <div class="index_company">
                                <i class="index_company-logo">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['zdjob']->value['logo'];?>
" alt="" style="width: 100%;">
                                </i>
                                <i class="index_company-name"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['zdjob']->value['com_name']),0,20,"utf-8");?>
</i>
                                <i class="index_company-status">置顶</i>
                            </div>
                        </div>
                    </a>
                    <?php } ?>
                    <?php  $_smarty_tpl->tpl_vars['newjob'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['newjob']->_loop = false;
 $_smarty_tpl->tpl_vars['njk'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("noids"=>"1","provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","limit"=>"15","key"=>"“njk“","item"=>"“newjob“","nocache"=>"")
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
		
		$newjob = $db->select_all("company_job",$where.$limit);

		if(is_array($newjob) && !empty($newjob)){
			$comuid=$jobid=array();
			foreach($newjob as $key=>$value){
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
			foreach($newjob as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($newjob[$key]);
					continue;
				}else{
					$newjob[$key] = $db->array_action($value,$cache_array);
					$newjob[$key][stime] = date("Y-m-d",$value[sdate]);
					$newjob[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$newjob[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$newjob[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$newjob[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$newjob[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $newjob[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $newjob[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $newjob[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $newjob[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $newjob[$key][job_number] = "";
					}else{
					    $newjob[$key][job_number] = $value[zp_num]." 人";
					}			
                    $newjob[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $newjob[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $newjob[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$newjob[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$newjob[$key][pr_n] = $comclass_name[$value[pr]];
					$newjob[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$newjob[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$newjob[$key]['time'] = $newjob[$key]['lastupdate'];
						$newjob[$key]['redtime'] =1;
					}else{
						$newjob[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$newjob[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$newjob[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$newjob[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$newjob[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$newjob[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$newjob[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$newjob[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$newjob[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$newjob[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$newjob[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$newjob[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$newjob[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$newjob[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$newjob[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$newjob[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$newjob[$key][name_n]);
						$newjob[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$newjob[$key][com_n]);
						$newjob[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$newjob[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $newjob[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $newjob[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($newjob)){
				if($paramer[keyword]!=""&&!empty($newjob)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$newjob = $newjob; if (!is_array($newjob) && !is_object($newjob)) { settype($newjob, 'array');}
foreach ($newjob as $_smarty_tpl->tpl_vars['newjob']->key => $_smarty_tpl->tpl_vars['newjob']->value) {
$_smarty_tpl->tpl_vars['newjob']->_loop = true;
 $_smarty_tpl->tpl_vars['njk']->value = $_smarty_tpl->tpl_vars['newjob']->key;
?>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['newjob']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['newjob']->value['name'];?>
">

                        <div class="table-card" style="<?php if ($_smarty_tpl->tpl_vars['njk']->value==0) {?>margin-top: .3rem;<?php }?>">
                            <div class="card_post">
                                <i class="table-card-word"><?php echo $_smarty_tpl->tpl_vars['newjob']->value['name'];?>
</i>
                                <i class="table-card-salary"><?php echo $_smarty_tpl->tpl_vars['newjob']->value['job_salary'];?>
</i>
                            </div>
                            <div class="table-card-require">
                                <i class="requir-area">
                                    <?php if ($_smarty_tpl->tpl_vars['newjob']->value['job_city_three']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['newjob']->value['job_city_three'];?>

                                    <?php } elseif ($_smarty_tpl->tpl_vars['newjob']->value['job_city_two']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['newjob']->value['job_city_two'];?>

                                    <?php } else { ?>
                                    <?php echo $_smarty_tpl->tpl_vars['newjob']->value['job_city_one'];?>

                                    <?php }?>
                                </i>
                                <i class="requir_area_parting_line"></i>
                                <?php if ($_smarty_tpl->tpl_vars['newjob']->value['job_edu']) {?><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['newjob']->value['job_edu'];?>
学历</i><?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['newjob']->value['job_exp']) {?> <i class="requir_area_parting_line"></i><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['newjob']->value['job_exp'];?>
经验</i><?php }?>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['newjob']->value['welfarename']) {?>
                            <div class="welfare">
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['newjob']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</span>
                                <?php } ?>
                            </div>
                            <?php }?>
                            <div class="index_company">
                                <i class="index_company-logo">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['newjob']->value['logo'];?>
" alt="" style="width: 100%;">
                                </i>
                                <i class="index_company-name"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['newjob']->value['com_name']),0,20,"utf-8");?>
</i>
                                <i class="index_company-status"><?php echo $_smarty_tpl->tpl_vars['newjob']->value['time'];?>
</i>
                            </div>
                        </div>
                    </a>
                    <?php } ?>
                </van-tab>

                <van-tab title="紧急">
                    <?php  $_smarty_tpl->tpl_vars['urgjob'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['urgjob']->_loop = false;
 $_smarty_tpl->tpl_vars['ujk'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","limit"=>"15","key"=>"“ujk“","item"=>"“urgjob“","urgent"=>"1","nocache"=>"")
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
		
		$urgjob = $db->select_all("company_job",$where.$limit);

		if(is_array($urgjob) && !empty($urgjob)){
			$comuid=$jobid=array();
			foreach($urgjob as $key=>$value){
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
			foreach($urgjob as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($urgjob[$key]);
					continue;
				}else{
					$urgjob[$key] = $db->array_action($value,$cache_array);
					$urgjob[$key][stime] = date("Y-m-d",$value[sdate]);
					$urgjob[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$urgjob[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$urgjob[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$urgjob[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$urgjob[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $urgjob[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $urgjob[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $urgjob[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $urgjob[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $urgjob[$key][job_number] = "";
					}else{
					    $urgjob[$key][job_number] = $value[zp_num]." 人";
					}			
                    $urgjob[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $urgjob[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $urgjob[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$urgjob[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$urgjob[$key][pr_n] = $comclass_name[$value[pr]];
					$urgjob[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$urgjob[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$urgjob[$key]['time'] = $urgjob[$key]['lastupdate'];
						$urgjob[$key]['redtime'] =1;
					}else{
						$urgjob[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$urgjob[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$urgjob[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$urgjob[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$urgjob[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$urgjob[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$urgjob[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$urgjob[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$urgjob[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$urgjob[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$urgjob[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$urgjob[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$urgjob[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$urgjob[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$urgjob[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$urgjob[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$urgjob[$key][name_n]);
						$urgjob[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$urgjob[$key][com_n]);
						$urgjob[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$urgjob[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $urgjob[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $urgjob[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($urgjob)){
				if($paramer[keyword]!=""&&!empty($urgjob)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$urgjob = $urgjob; if (!is_array($urgjob) && !is_object($urgjob)) { settype($urgjob, 'array');}
foreach ($urgjob as $_smarty_tpl->tpl_vars['urgjob']->key => $_smarty_tpl->tpl_vars['urgjob']->value) {
$_smarty_tpl->tpl_vars['urgjob']->_loop = true;
 $_smarty_tpl->tpl_vars['ujk']->value = $_smarty_tpl->tpl_vars['urgjob']->key;
?>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['urgjob']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['urgjob']->value['name'];?>
">

                        <div class="table-card" style="<?php if ($_smarty_tpl->tpl_vars['ujk']->value==0) {?>margin-top: .3rem;<?php }?>">
                            <div class="card_post">
                                <i class="table-card-word"><?php echo $_smarty_tpl->tpl_vars['urgjob']->value['name'];?>
</i>
                                <i class="table-card-salary"><?php echo $_smarty_tpl->tpl_vars['urgjob']->value['job_salary'];?>
</i>
                            </div>
                            <div class="table-card-require">
                                <i class="requir-area">
                                    <?php if ($_smarty_tpl->tpl_vars['urgjob']->value['job_city_three']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['urgjob']->value['job_city_three'];?>

                                    <?php } elseif ($_smarty_tpl->tpl_vars['urgjob']->value['job_city_two']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['urgjob']->value['job_city_two'];?>

                                    <?php } else { ?>
                                    <?php echo $_smarty_tpl->tpl_vars['urgjob']->value['job_city_one'];?>

                                    <?php }?>
                                </i>
                                <i class="requir_area_parting_line"></i>
                                <?php if ($_smarty_tpl->tpl_vars['urgjob']->value['job_edu']) {?><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['urgjob']->value['job_edu'];?>
学历</i><?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['urgjob']->value['job_exp']) {?>  <i class="requir_area_parting_line"></i><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['urgjob']->value['job_exp'];?>
经验</i><?php }?>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['urgjob']->value['welfarename']) {?>
                            <div class="welfare">
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['urgjob']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</span>
                                <?php } ?>
                            </div>
                            <?php }?>
                            <div class="index_company">
                                <i class="index_company-logo">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['urgjob']->value['logo'];?>
" alt="" style="width: 100%;">
                                </i>
                                <i class="index_company-name"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['urgjob']->value['com_name']),0,20,"utf-8");?>
</i>
                                <i class="index_company-status">
                                    <?php echo $_smarty_tpl->tpl_vars['urgjob']->value['time'];?>

                                </i>
                            </div>
                        </div>
                    </a>
                    <?php } ?>
                </van-tab>

                <van-tab title="推荐">
                    <?php  $_smarty_tpl->tpl_vars['recjob'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['recjob']->_loop = false;
 $_smarty_tpl->tpl_vars['rjk'] = new Smarty_Variable;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("provinceid"=>"“auto.provinceid“","cityid"=>"“auto.cityid“","limit"=>"15","key"=>"“rjk“","item"=>"“recjob“","rec"=>"1","nocache"=>"")
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
		
		$recjob = $db->select_all("company_job",$where.$limit);

		if(is_array($recjob) && !empty($recjob)){
			$comuid=$jobid=array();
			foreach($recjob as $key=>$value){
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
			foreach($recjob as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($recjob[$key]);
					continue;
				}else{
					$recjob[$key] = $db->array_action($value,$cache_array);
					$recjob[$key][stime] = date("Y-m-d",$value[sdate]);
					$recjob[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$recjob[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$recjob[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$recjob[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$recjob[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $recjob[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $recjob[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $recjob[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $recjob[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $recjob[$key][job_number] = "";
					}else{
					    $recjob[$key][job_number] = $value[zp_num]." 人";
					}			
                    $recjob[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $recjob[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $recjob[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$recjob[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$recjob[$key][pr_n] = $comclass_name[$value[pr]];
					$recjob[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$recjob[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$recjob[$key]['time'] = $recjob[$key]['lastupdate'];
						$recjob[$key]['redtime'] =1;
					}else{
						$recjob[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$recjob[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$recjob[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$recjob[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$recjob[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$recjob[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$recjob[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$recjob[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$recjob[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$recjob[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$recjob[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$recjob[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$recjob[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$recjob[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$recjob[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$recjob[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$recjob[$key][name_n]);
						$recjob[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$recjob[$key][com_n]);
						$recjob[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$recjob[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $recjob[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $recjob[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($recjob)){
				if($paramer[keyword]!=""&&!empty($recjob)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$recjob = $recjob; if (!is_array($recjob) && !is_object($recjob)) { settype($recjob, 'array');}
foreach ($recjob as $_smarty_tpl->tpl_vars['recjob']->key => $_smarty_tpl->tpl_vars['recjob']->value) {
$_smarty_tpl->tpl_vars['recjob']->_loop = true;
 $_smarty_tpl->tpl_vars['rjk']->value = $_smarty_tpl->tpl_vars['recjob']->key;
?>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job','a'=>'comapply','id'=>$_smarty_tpl->tpl_vars['recjob']->value['id']),$_smarty_tpl);?>
" title="<?php echo $_smarty_tpl->tpl_vars['recjob']->value['name'];?>
">

                        <div class="table-card" style="<?php if ($_smarty_tpl->tpl_vars['rjk']->value==0) {?>margin-top: .3rem;<?php }?>">
                            <div class="card_post">
                                <i class="table-card-word"><?php echo $_smarty_tpl->tpl_vars['recjob']->value['name'];?>
</i>
                                <i class="table-card-salary"><?php echo $_smarty_tpl->tpl_vars['recjob']->value['job_salary'];?>
</i>
                            </div>
                            <div class="table-card-require">
                                <i class="requir-area">
                                    <?php if ($_smarty_tpl->tpl_vars['recjob']->value['job_city_three']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['recjob']->value['job_city_three'];?>

                                    <?php } elseif ($_smarty_tpl->tpl_vars['recjob']->value['job_city_two']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['recjob']->value['job_city_two'];?>

                                    <?php } else { ?>
                                    <?php echo $_smarty_tpl->tpl_vars['recjob']->value['job_city_one'];?>

                                    <?php }?>
                                </i>
                                <i class="requir_area_parting_line"></i>
                                <?php if ($_smarty_tpl->tpl_vars['recjob']->value['job_edu']) {?><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['recjob']->value['job_edu'];?>
学历</i><?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['recjob']->value['job_exp']) {?>  <i class="requir_area_parting_line"></i><i class="requir-area"><?php echo $_smarty_tpl->tpl_vars['recjob']->value['job_exp'];?>
经验</i><?php }?>
                            </div>
                            <?php if ($_smarty_tpl->tpl_vars['recjob']->value['welfarename']) {?>
                            <div class="welfare">
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['recjob']->value['welfarename']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <span class="welfare_n"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</span>
                                <?php } ?>
                            </div>
                            <?php }?>
                            <div class="index_company">
                                <i class="index_company-logo">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['recjob']->value['logo'];?>
" alt="" style="width: 100%;">
                                </i>
                                <i class="index_company-name"><?php echo mb_substr(preg_replace('!<[^>]*?>!', ' ', $_smarty_tpl->tpl_vars['recjob']->value['com_name']),0,20,"utf-8");?>
</i>
                                <i class="index_company-status">
                                    <?php echo $_smarty_tpl->tpl_vars['recjob']->value['time'];?>

                                </i>
                            </div>
                        </div>
                    </a>
                    <?php } ?>
                </van-tab>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
                <van-tab title="附近">
                    <div v-if="skeletonLoading">
                        <div class="map_job_list" v-for="(item,skeletonKey) in skeletonLen" :key="skeletonKey">
                            <div class="map_job_list_box">
                                <van-skeleton :row-width="['100%', '50%', '100%']" :row="3"></van-skeleton>
                            </div>
                        </div>
                    </div>
                    <div v-else>
                        <div v-if="nearbyJob">
                            <div v-for="(item,nkey) in nearbyJobList" :key="nkey" class="map_job_list">
                                <div class="map_job_list_box">
                                    <div class="map_job_top">
                                        <div class="neighbouring_top">
                                            <div class="map_job_topname">
                                                <a :href="item.joburl">{{item.name}}</a>
                                            </div>
                                            <span class="map_job_xz">{{item.salary_n}}</span>
                                        </div>
                                        <div class="map_job_list_welfare">
                                            <ul>
                                                <li v-if="item.job_city_three">{{item.job_city_three}}</li>
                                                <li v-else-if="item.job_city_two">{{item.job_city_two}}</li>
                                                <li v-else>{{item.job_city_one}}</li>
                                                <i class="requir_area_parting_line"></i>
                                                <li v-if="item.job_edu">{{item.job_edu}}学历</li>
                                                <i class="requir_area_parting_line"></i>
                                                <li v-if="item.job_exp">{{item.job_exp}}经验</li>
                                            </ul>
                                        </div>
                                        <div v-if="item.welfare" class="welfare">
                                        <span v-for="(witem,wkey) in item.welfare" :key="wkey" class="welfare_n">
                                            {{witem}}
                                        </span>
                                        </div>
                                    </div>
                                    <div class="com_map">
                                        <div class="map_job_com">
                                            <a :href="item.comurl">
                                                <div class="map_job_com_logo">
                                                    <img :src="item.logo" alt="" width="100%" height="100%">
                                                </div>
                                                <div class="map_job_com_name">{{item.com_name}}</div>
                                            </a>
                                        </div>
                                        <div class="com_map_name">
                                            <a :href="item.addressurl">
                                                <div class="com_map_name_address">{{item.address}}</div>
                                                <div class="com_map_distance">{{item.dis}}</div>
                                            </a>
                                        </div>

                                    </div>
                                </div>
                            </div>
                        </div>
                        <div v-else class="no_data">
                            <div class="no_data_img">
                                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/home_emptygraph.png" alt=""style="width: 100%;">
                            </div>
                            <i class="no_data_word">当前没有职位数据哦~</i>
                        </div>
                    </div>
                </van-tab>
				<?php }?>
            </van-tabs>
			<!--弹窗广告-->
            <van-popup v-model="adBanner" position="center" :style="{ width:'90%',background:'none'}" closeable>
				<?php  $_smarty_tpl->tpl_vars["lunbo"] = new Smarty_Variable; $_smarty_tpl->tpl_vars["lunbo"]->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
global $db,$db_config,$config;$AdArr=array();$paramer=array();$attr=array("classid"=>"502","item"=>"\"lunbo\"","key"=>"“key“","random"=>"1","nocache"=>"")
;
			include(PLUS_PATH.'pimg_cache.php');$add_arr = $ad_label[502];if(is_array($add_arr) && !empty($add_arr)){
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
				<div class="zd_userbanner"><?php echo $_smarty_tpl->tpl_vars['lunbo']->value['html'];?>
</div>
				<?php } ?>
            </van-popup>
			<!--弹窗广告-->
        </div>
    </div>
    <div class="yun_newedition_jobmore"><a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'job'),$_smarty_tpl);?>
">查看更多</a></div>

    <div class="yun_newedition_footer">
        <div class="">
            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'contact'),$_smarty_tpl);?>
">联系我们</a>
            <span class="yun_newedition_footer_line">|</span>
            <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_app_open']==1) {?>
            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'appdown'),$_smarty_tpl);?>
">下载APP</a>
            <span class="yun_newedition_footer_line">|</span>
            <?php }?>
            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'advice'),$_smarty_tpl);?>
">意见反馈</a>
            <span class="yun_newedition_footer_line">|</span>
            <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'about'),$_smarty_tpl);?>
">关于我们</a>
        </div>
    </div>
	<?php if (!empty($_smarty_tpl->tpl_vars['kfurl']->value)&&$_smarty_tpl->tpl_vars['isweixin']->value) {?>
	<!--企业微信浮动客服-->
    <a href="<?php echo $_smarty_tpl->tpl_vars['kfurl']->value;?>
" class="zxkf"> </a>
    <?php }?>
</div>

<style>
    .van-tabs__nav {
        background-color: #f4f4f4;
        font-size: 0.8rem;
    }

    .van-tab__text {
        font-size: 0.426666rem;
    }

    .van-tab--active {
        font-weight: bold;
    }

    .van-skeleton__row:not(:first-child) {
        margin-top: 22px;
    }
</style>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
<?php echo '<script'; ?>
 type="text/javascript">
    window._AMapSecurityConfig = {
        securityJsCode:'<?php echo $_smarty_tpl->tpl_vars['config']->value['map_secret'];?>
'
    }
<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 type="text/javascript" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['mapurl'];?>
"><?php echo '</script'; ?>
>
<?php }?>
<?php echo '<script'; ?>
>
    // 轮播图
    new Swiper('#imgswiper', {
        direction: 'horizontal',
        autoplay: {
            disableOnInteraction: false
        },
        loop: true
    });
    new Swiper('#gundswiper', {
        direction: 'horizontal',
        autoplay: {
            disableOnInteraction: false
        },
        loop: true,
        speed: 2000
    });
    // 金刚位
    new Swiper('#navswiper', {
        direction: 'horizontal',
        pagination: {
            el: '.swiper-pagination'
        }
    });
    // 公告
    new Swiper('#ggswiper', {
        direction: 'vertical',
        autoplay: {
            disableOnInteraction: false
        },
        loop: true,
		height: 40,
        autoHeight: true
    });
	// 名企
	new Swiper('#mqswiper', {
	    direction: 'horizontal',
	    autoplay: {
	        disableOnInteraction: false,
	    },
	    loop: true,
		speed: 1000
	});
    var sy_web_site = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_web_site'];?>
';
    var sy_gotocity = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_gotocity'];?>
';

    var yunvue = new Vue({
        el: '#yunvue',
        data() {
            return {
                skeletonLoading: true,
                skeletonLen: new Array(15).fill(''),
                nearbyJob: false,
                nearbyJobList: [],
                x: '',
                y: '',
                adBanner:false,               
            };
        },
        created() {
			$('#yunvue').css('display','block');

            if(sy_web_site=='1' && sy_gotocity=='1'){
                if(window.localStorage.getItem('wapToDomainAuto')!=1){
                    this.todomain();
                };
            }else{
                window.localStorage.removeItem('wapToDomainAuto');
            }

            this.adShow();
        },
        methods: {
            todomain:function(){
                this.getCurrentLoaction().then(this.getCityDomain);
            },

            getCityDomain: function () {

                let that = this;
                let paramer = {
                    x: this.x,
                    y: this.y
                }
                
                $.post(wapurl+'index.php?c=index&a=getCityDomain', paramer, function (data) {

                    if(data.error==1 || data.error==0){
                        window.localStorage.setItem("wapToDomainAuto", "1");
                    }
                    
                    if (data.error==1) {
                        showConfirm(`您现在正在${data.citydomain.cityname},是否切换到该站点？`,function(){
                            that.setsite(data.citydomain.curcityid);
                        },'取消','确定',function(){
                            showToast('您也可以通过点击左上角切换站点');
                        });
                    }else if(data.error==2){
                        window.localStorage.removeItem('wapToDomainAuto');
                    }
                    
                }, 'json');
            },
            chooseTab: function(name, title){
                let that = this;
                if (name==3 && title=='附近'){

                    this.getCurrentLoaction().then(this.getNearbyJob)
                                             .catch(this.getNearbyJob);
                }
            },
            getCurrentLoaction: function () {
                
                let that = this;
                
                return new Promise((resolve, reject)=>{
                    AMap.plugin('AMap.Geolocation', function() {
                       var geolocation = new AMap.Geolocation();
                    
                      geolocation.getCurrentPosition(function(status,result){
                            if(status=='complete' && result.info == 'SUCCESS'){
                                var position = result.position;
                                that.x = position.lng;
                                that.y = position.lat;
                                resolve();
                            }else{
                                console.log('获取定位异常');
                                console.log(result);
                                reject();
                            }
                      });
                    });
                })
            },
            getNearbyJob: function () {
                let that = this;
                let paramer = {
                    x: that.x,
                    y: that.y,
                    limit: 15
                }
                $.post(wapurl+'index.php?c=map&a=joblist', paramer, function (data) {
                    if (data.list.length > 0) {
                        that.nearbyJobList = data.list
                        that.nearbyJob = true;
                    }
                    that.skeletonLoading = false;
                }, 'json');
            },
            adShow: function () { // 首页弹出广告
				if($('.zd_userbanner').length > 0){
					var bannerFlag = "<?php echo $_smarty_tpl->tpl_vars['bannerFlag']->value;?>
";
					if (bannerFlag) {
						this.adBanner = false;
					} else{
						this.adBanner = true;
					}
				}
            },
			setsite:function(id){
            	$.post(wapurl+'index.php?c=site&a=domain',{id:id},function(data){
					window.location.href=wapurl;
				});
            }
        }
    });
	
    function privacy(){
        var paramer = {                   
            status: 1,
        };
        showLoading('设置中...');
        $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'privacy','c'=>'up'),$_smarty_tpl);?>
', paramer, function(data){
            hideLoading();
            location.reload();
        },'json');
        
    }
    
<?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/search_new.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }} ?>
