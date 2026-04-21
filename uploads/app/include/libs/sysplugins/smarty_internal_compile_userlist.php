<?php
class Smarty_Internal_Compile_Userlist extends Smarty_Internal_CompileBase{
	public $required_attributes = array('item');
	public $optional_attributes = array('notid','name', 'key', 'post_len', 'limit', 'city_len', 'salary', 'minsalary', 'maxsalary', 'minage', 'maxage', 'idcard', 'edu', 'order', 'work', 'exp', 'sex','birthday', 'keyword', 'hy', 'provinceid', 'report', 'cityid', 'three_cityid', 'threecityid', 'adtime', 'pic', 'typeids', 'type', 'job1_son', 'job_post', 'job1son', 'jobpost', 'uptime', 'ispage', 'recg','where_uid', 'height_status', 'rec', 't_len' ,'top','job_classid','islt','job1','isshow','cityin','jobin','where','topdate','noid','tag','workexp','integrity', 'rec_resume','isrck');
	public $shorttag_order = array('from', 'item', 'key', 'name');
	public function compile($args, $compiler, $parameter){
		$_attr = $this->getAttributes($compiler, $args);

		$from = $_attr['from'];
		$item = $_attr['item'];
		$name = $_attr['item'];
		$name=str_replace('\'','',$name);
		$name=$name?$name:'list';$name='$'.$name;
		if (!strncmp("\$_smarty_tpl->tpl_vars[$item]", $from, strlen($item) + 24)) {
			$compiler->trigger_template_error("item variable {$item} may not be the same variable as at 'from'", $compiler->lex->taglineno);
		}

		//自定义标签START
		$OutputStr=''.$name.'=array();global $db,$db_config,$config;
		if(is_array($_GET)){
			foreach($_GET as $key=>$value){
				if($value==\'0\'){
					unset($_GET[$key]);
				}
			}
		}
		$paramer='.ArrayToString($_attr,true).';
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
		if($config[\'sy_web_site\']=="1"){
			if($config[province]>0 && $config[province]!=""){
				$paramer[provinceid] = $config[province];
			}
			if($config[\'cityid\']>0 && $config[\'cityid\']!=""){
				$paramer[\'cityid\']=$config[\'cityid\'];
			}
			if($config[\'three_cityid\']>0 && $config[\'three_cityid\']!=""){
				$paramer[\'three_cityid\']=$config[\'three_cityid\'];
			}
			if($config[\'hyclass\']>0 && $config[\'hyclass\']!=""){
				$paramer[\'hy\']=$config[\'hyclass\'];
			}
		}

		
		$where = "a.`defaults`=1 and a.`state`=1 and a.`r_status`=1 AND a.`status`=1";

        //关注我公司的人才--条件
		if($paramer[where_uid]){
			$where .=" AND a.`uid` in (".$paramer[\'where_uid\'].")";
		}
		//黑名单不能查看已拉黑的个人用户简历
		if($_COOKIE[\'uid\']&&$_COOKIE[\'usertype\']=="2"){
			$blacklist=$db->select_all("blacklist","`p_uid`=\'".$_COOKIE[\'uid\']."\'","c_uid");
			if(is_array($blacklist)&&$blacklist){
				foreach($blacklist as $v){
					$buid[]=$v[\'c_uid\'];
				}
			    $where .=" AND a.`uid` NOT IN (".@implode(",",$buid).")";
			}
		}

        //置顶
		if($paramer[topdate]){
			$where .=" AND a.`top`=1 AND a.`topdate`>\'".time()."\'";
		}
		if($paramer[top]){
			$where .=" AND a.`top`=1 AND a.`topdate`>\'".time()."\'";
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
					$eid[]=$v[\'eid\'];
				}
			}
			$where .=" AND a.`id` in (".@implode(",",$eid).")";
		}
		//标签
		if($paramer[tag]){
		    $tagname=$userclass_name[$paramer[tag]];
			$oneYear    =   strtotime("-1 year");
			$tag=$db->select_all("resume","`def_job`>0 and `r_status`=1 and `status`=1 and `lastupdate` > \'".$oneYear."\' and FIND_IN_SET(\'".$tagname."\',`tag`)","`def_job`");
			if(is_array($tag)){
				foreach($tag as $v){
					$tagid[]=$v[\'def_job\'];
				}
			}
			$where .=" AND a.`id` in (".@implode(",",$tagid).")";
		}
		//更新时间区间
		if($paramer[uptime]){
			if($paramer[uptime]==1){
				$beginToday=mktime(0,0,0,date(\'m\'),date(\'d\'),date(\'Y\'));
    			$where.=" AND a.`lastupdate`>$beginToday";
    		}else{
    			$time=time();
				$uptime = $time-($paramer[uptime]*86400);
				$where.=" AND a.`lastupdate`>$uptime";
    		}
		}else{
		    if($config[sy_datacycle]>0){
                // 后台-页面设置-数据周期
		        $uptime = strtotime(\'-\'.$config[sy_datacycle].\' day\');
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
			$where .=" AND a.`photo`<>\'\' AND a.`phototype`!=1 AND a.`defphoto`= 1";
		}
        //行业
		if($paramer[\'hy\']!=""){
			$where .= " AND a.`hy` IN (".$paramer[\'hy\'].")";
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
            if($city_parent[$paramer[cityin]]==\'0\'){
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
            if($job_parent[$paramer[jobin]]==\'0\'){
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
                $expArr  = $userdata[\'user_word\'];
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

		    $eduArr  = $userdata[\'user_edu\'];
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
		 	
		 	$where.= " AND a.`integrity`>=\'".$integrityR[$paramer[integrity]]."\'";

		}

		//关键字
		if($paramer[keyword]){
			$where1 = array();
			$where1[]="a.`name` LIKE \'%$paramer[keyword]%\'";
			$where1[]="a.`uname` LIKE \'%$paramer[keyword]%\'";
			
			//搜索工作经历 
			$workList = $db->select_all(\'resume_work\',"`name` LIKE \'%$paramer[keyword]%\' OR `title` LIKE \'%$paramer[keyword]%\' OR `content` LIKE \'%$paramer[keyword]%\' ORDER BY id DESC limit 500","`eid`");
			if(!empty($workList)){
				$workId = array();
				foreach($workList as $value){
					$workId[] = $value[\'eid\'];
				}
				$where1[]	=	"a.id IN (".implode(\',\',$workId).")";
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
			$where.= " AND a.`birthday`>= \'".$maxtime."\' and a.`birthday`<=\'".$mintime."\'";
		}elseif($paramer[minage]&&!$paramer[maxage]){
			$mintime=date("Y-m-d",strtotime("-".$paramer[minage]." year"));
			$where.= " AND a.`birthday`<=\'".$mintime."\'";
		}elseif(!$paramer[minage]&&$paramer[maxage]){
			$maxtime=date("Y-m-d",strtotime("-".$paramer[maxage]." year"));
			$where.= " AND a.`birthday`>=\'".$maxtime."\'";
		}
        //排序字段默认为更新时间
		if($paramer[order] && $paramer[order]!="lastdate"){
			if($paramer[order]==\'topdate\'){
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
			'.$name.' = array();
		}else{
			
			$select="a.`id`,a.`uid`,a.`name`,a.`hy`,a.`job_classid`,a.`city_classid`,a.`jobstatus`,a.`type`,a.`report`,a.`lastupdate`,a.`rec`,a.`top`,a.`topdate`,a.`rec_resume`,a.`ctime`,a.`uname`,a.`idcard_status`,a.`minsalary`,a.`maxsalary`";
			if($pagewhere!=""){

				$sql = "select ".$select." from `".$db_config[def]."resume_expect` a ".$pagewhere." where ".$joinwhere." and ".$where.$order.$sort.$limit;
				'.$name.'=$db->DB_query_all($sql,"all");

			}else{
				$sql = "select ".$select." from `".$db_config[def]."resume_expect` a where ".$where.$order.$sort.$limit;
				'.$name.'=$db->DB_query_all($sql,"all");
			}
		}
        
		if(!empty('.$name.') && is_array('.$name.')){
			
			//如果存在top，则说明请求来自排行榜页面
			if($paramer[\'top\']){
				$uids=$m_name=array();
				foreach('.$name.' as $k=>$v){
					$uids[]=$v[uid];
				}

				$member=$db->select_all($db_config[def]."member","`uid` in(".@implode(\',\',$uids).")","uid,username");
				foreach($member as $val){
					$m_name[$val[uid]]=$val[\'username\'];
				}
			}
			$uid = $eid = array();
			foreach('.$name.' as $key=>$value){
				
				$uid[] = $value[\'uid\'];
				$eid[] = $value[\'id\'];
			}
			$eids = @implode(\',\',$eid);
			$uids = @implode(\',\',$uid);
            $resume=$db->select_all("resume","`uid` in(".$uids.")","uid,name,nametype,tag,sex,moblie_status,edu,exp,photo,phototype,defphoto,photo_status,birthday");
			foreach($resume as $v){
				$ruids[] = $v[\'uid\'];
			}
			foreach('.$name.' as $k=>$v){
				if(!in_array($v[\'uid\'],$ruids)){
					unset('.$name.'[$k]);
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
						$eduListNew[$value[\'eid\']][] = $value;
					}
					foreach($eduListNew as $k=>$eduv){
						$edumin				=		0;
						$edumax				=		0;
						$edutitle			=	array();
						$education			=	array();
						foreach($eduv as $v){
							if($v[\'sdate\']>0 && $edumin==0){
								$edumin		=		$v[\'sdate\'];
							}elseif($edumin>$v[\'sdate\']){
								$edumin		=		$v[\'sdate\'];
							}
							if($v[\'edate\']==0 ){
								$edumax		=		0;
							}elseif($edumax<$v[\'edate\']){
								$edumax		=		$v[\'edate\'];
							}
						
							$education[]	=		$userclass_name[$v[\'education\']];

							$edutitle[]		=		$v[\'specialty\'];
						}
						$return =array();
						$return[\'edumin\']		=		date(\'Y-m\',$edumin);
						$return[\'edumax\']		=		$edumax  == 0 ?  \'至今\': date(\'Y\',$edumax);
						$return[\'education\']	=		@implode(\',\',$education);
						$return[\'eduspecialty\']		=		@implode(\'、\',$edutitle);
						
						$return[\'edu_time\']		=		$return[\'edumin\']."-".$return[\'edumax\'];
						
						if($return[\'eduspecialty\']){
						    $workexpList[$k][\'edu_content\']	=	$return[\'education\'].\'学历 · \'.$return[\'eduspecialty\'].\' · 毕业于\'.$return[\'edumax\'].\'年\';
						}else{
						    $workexpList[$k][\'edu_content\']	=	$return[\'education\'].\'学历 · 毕业于\'.$return[\'edumax\'].\'年\';
						}
					}
					
				}

				$workList  = $db -> select_all("resume_work","`eid` in(".$eids.")");
				if(!empty($workList)){
					foreach($workList as $key=>$value){
						$workListNew[$value[\'eid\']][] = $value;
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
							
							if($v[\'sdate\']>0 && $workmin==0){
								$workmin		=		$v[\'sdate\'];
							}elseif($workmin>$v[\'sdate\']){
								$workmin		=		$v[\'sdate\'];
							}
							
							if($v[\'edate\']==0 ){
								$workmax		=		0;
							}elseif($workmax<$v[\'edate\']){
								$workmax		=		$v[\'edate\'];
							}
			
							$wtitle[]			=		$v[\'title\'];
							
							$hour[]   			=		$workTime;
							$whour    			+=   	$workTime;
						}
						
						$workavg   =   ceil($whour/count($hour));
						$return = array();
						$return[\'worknum\']		=		$worknum  > 0 ?  $worknum:0;
						$return[\'workavg\']		=		$workavg  > 0 ?  $workavg:0;
						$return[\'workmin\']		=		date(\'Y-m\',$workmin);
						$return[\'workmax\']		=		$workmax  == 0 ?  \'至今\': date(\'Y-m\',$workmax);
						$return[\'worktit\']		=		@implode(\',\',$wtitle);
						
						$return[\'work_time\']	=		$return[\'workmin\'].\'-\'.$return[\'workmax\'];
						
						if($return[\'worktit\']!=\'\'){
							$workexpList[$k][\'work_content\']	=	\' 参加过\'.$return[\'worknum\'].\'份工作 · 涉及\'.$return[\'worktit\'].\'等岗位\';
						}else{
							$workexpList[$k][\'work_content\']	=	\' 参加过\'.$return[\'worknum\'].\'份工作\';
						}
					}
				}
			}
			if ($_COOKIE[\'uid\'] && $_COOKIE[\'usertype\']==2){
			    $lookResume =   $db->select_all("look_resume","`com_id` = ".$_COOKIE[\'uid\'], "`resume_id`");
			    if (!empty($lookResume)){
			        foreach($lookResume as $key=>$value){
						$lookResumeId[] = $value[\'resume_id\'];
					}
			    }
			    if($paramer[isrck] == 1){
                $downList = $freedownList = array();
                    $downList = $db->select_all(\'down_resume\', "`comid` = ".$_COOKIE[\'uid\'] . " and `uid` in (".$uids.")", "`comid`,`eid`");
                    $freedownList = $db->select_all(\'freedown_resume\', "`comid` = ".$_COOKIE[\'uid\'] . " and `uid` in (".$uids.")", \'`comid`,`eid`\');
			    $vipetime = 0;
			    $vip_etime = $db->select_all(\'company\', "`uid` = ".$_COOKIE[\'uid\'], "`vipetime`");
			    $vipetime = $vip_etime[0][vipetime] ? $vip_etime[0][vipetime] : 0;
			    $userid_msg = $db->select_all(\'userid_msg\', "`fid` = ".$_COOKIE[\'uid\'] . " and `uid` in (".$uids.") and `isdel` = 9",\'`uid`\');
			    }
			}
			foreach('.$name.' as $k=>$v){
				if($paramer[topdate]){
					$noids[] = $v[id];
				}
				//筛除重复
				if($paramer[noid]==\'1\' && !empty($noids) && in_array($v[\'id\'],$noids)){
					unset('.$name.'[$k]);
					continue;
				}
			    foreach($resume as $val){
			        if($v[\'uid\']==$val[\'uid\']){
			    		'.$name.'[$k][\'edu_n\']=$userclass_name[$val[\'edu\']];
				        '.$name.'[$k][\'exp_n\']=$userclass_name[$val[\'exp\']];
			            if($val[\'birthday\']){
							
							$ageArr = explode(\'-\',$val[\'birthday\']);
						
							if(count($ageArr)==3){// 原来数据生日格式为年月日，如果是旧格式，改为年月
								
								$val[\'birthday\'] = $ageArr[0].\'-\'.$ageArr[1];
								
							}
							
							$year = date("Y",strtotime($val[\'birthday\']));
							'.$name.'[$k][\'age\'] =date("Y")-$year;
						}
						if($val[\'sex\']==152){
							$val[\'sex\']=\'1\';
						}elseif ($val[\'sex\']==153){
							$val[\'sex\']=\'2\';
						}
						'.$name.'[$k][\'sex\'] =$arr_data[sex][$val[\'sex\']];
		                '.$name.'[$k][\'phototype\']=$val[phototype];
		                $photo=$icon="";
		                if($val[\'defphoto\']==2){
		                	$photo=$val[\'photo\'];
		                }else{
							if($config[\'user_pic\']==1 || empty($config[\'user_pic\'])){
				                if($val[\'photo\'] && $val[\'photo_status\']==0 && $val[\'phototype\']!=1){
		            				$photo=$val[\'photo\'];
		            			}else{
		            				if($val[\'sex\']==1){
		            					$icon=$config[\'sy_member_icon\'];
		            				}else{
		            					$icon=$config[\'sy_member_iconv\'];
		            				}
		            			}
		            			
							}elseif($config[\'user_pic\']==2){
								if($val[\'photo\']&& $val[\'photo_status\']==0){
									$photo=$val[\'photo\'];
								}else{
									if($val[\'sex\']==1){
										$icon=$config[\'sy_member_icon\'];
									}else{
										$icon=$config[\'sy_member_iconv\'];
									}
								}
							}elseif($config[\'user_pic\']==3){
								if($val[\'sex\']==1){
									$icon=$config[\'sy_member_icon\'];
								}else{
									$icon=$config[\'sy_member_iconv\'];
								}
							}
						}
						'.$name.'[$k][\'photo\']=checkpic($photo,$icon);
						if($val[\'tag\']){
                            '.$name.'[$k][\'tag\']=explode(\',\',$val[\'tag\']);
	                    }
                        '.$name.'[$k][\'nametype\']=$val[nametype];
                        '.$name.'[$k][\'moblie_status\']=$val[moblie_status];
                        //名称显示处理
						if($config[\'user_name\']==1 || !$config[\'user_name\']){
    						if($val[\'nametype\']==3){
    						    if($val[\'sex\']==1){
    						        '.$name.'[$k][\'username_n\'] = mb_substr($val[\'name\'],0,1,\'utf-8\')."先生";
    						    }else{
    						        '.$name.'[$k][\'username_n\'] = mb_substr($val[\'name\'],0,1,\'utf-8\')."女士";
    						    }
    						}elseif($val[\'nametype\']==2){
    						    '.$name.'[$k][\'username_n\'] = "NO.".$v[\'id\'];
    						}else{
    							'.$name.'[$k][\'username_n\'] = $val[\'name\'];
    						}
						}elseif($config[\'user_name\']==3){
							if($val[\'sex\']==1){
								'.$name.'[$k][\'username_n\'] = mb_substr($val[\'name\'],0,1,\'utf-8\')."先生";
							}else{
								'.$name.'[$k][\'username_n\'] = mb_substr($val[\'name\'],0,1,\'utf-8\')."女士";
							}
						}elseif($config[\'user_name\']==2){
							'.$name.'[$k][\'username_n\'] = "NO.".$v[\'id\'];
						}elseif($config[\'user_name\']==4){
							'.$name.'[$k][\'username_n\'] = $val[\'name\'];
						}
                    }
                }
				//更新时间显示方式
				$time=$v[\'lastupdate\'];
				//今天开始时间戳
				$beginToday=mktime(0,0,0,date(\'m\'),date(\'d\'),date(\'Y\'));
				//昨天开始时间戳
				$beginYesterday=mktime(0,0,0,date(\'m\'),date(\'d\')-1,date(\'Y\'));
				if($time>$beginYesterday && $time<$beginToday){
					'.$name.'[$k][\'time\'] = "昨天";
				}elseif($time>$beginToday){
					'.$name.'[$k][\'time\'] = lastupdateStyle($v[\'lastupdate\']);
					'.$name.'[$k][\'redtime\'] =1;
				}else{
					'.$name.'[$k][\'time\'] = date("Y-m-d",$v[\'lastupdate\']);
				} 
                
                // 前天
				$beforeYesterday=mktime(0,0,0,date(\'m\'),date(\'d\')-2,date(\'Y\'));

				if($v[\'ctime\']>$beforeYesterday){
					'.$name.'[$k][\'newtime\'] =1;
				}
				'.$name.'[$k][\'user_jobstatus_n\']=$userclass_name[$v[\'jobstatus\']];
// 				'.$name.'[$k][\'job_city_one\']=$city_name[$v[\'provinceid\']];
// 				'.$name.'[$k][\'job_city_two\']=$city_name[$v[\'cityid\']];
// 				'.$name.'[$k][\'job_city_three\']=$city_name[$v[\'three_cityid\']];
                '.$name.'[$k][\'salary_n\'] = salaryUnit($v[\'minsalary\'], $v[\'maxsalary\']);
				'.$name.'[$k][\'report_n\']=$userclass_name[$v[\'report\']];
				'.$name.'[$k][\'type_n\']=$userclass_name[$v[\'type\']];
				'.$name.'[$k][\'lastupdate\']=date("Y-m-d",$v[\'lastupdate\']);
					
				'.$name.'[$k][\'user_url\']=Url("resume",array("c"=>"show","id"=>$v[\'id\']),"1");
				'.$name.'[$k]["hy_info"]=$industry_name[$v[\'hy\']];
				if($paramer[\'top\']){
					'.$name.'[$k][\'m_name\']=$m_name[$v[\'uid\']];
					'.$name.'[$k][\'user_url\']=Url("ask",array("c"=>"friend","a"=>"myquestion","uid"=>$v[\'uid\']));
				}
				'.$name.'[$k][\'work_content\']=$workexpList[$v[\'id\']][\'work_content\'];
				'.$name.'[$k][\'edu_content\']=$workexpList[$v[\'id\']][\'edu_content\'];

				$kjob_classid=@explode(",",$v[\'job_classid\']);
				$kjob_classid=array_unique($kjob_classid);	
				$jobname=array();
				if(is_array($kjob_classid)){
					foreach($kjob_classid as $val){
					    if($val!=\'\'){
					        if($paramer[\'keyword\']){
                               $jobname[]=str_replace($paramer[\'keyword\'],"<font color=#FF6600 >".$paramer[\'keyword\']."</font>",$job_name[$val]);
                            }else{
                               $jobname[]=$job_name[$val];
                            }
                        }
					}
				}
				//'.$name.'[$k][\'job_post\']=@implode(",",$jobname);
				'.$name.'[$k][\'expectjob\']=$jobname;
				$kcity_classid=@explode(",",$v[\'city_classid\']);
				$kcity_classid=array_unique($kcity_classid);	
				$cityname=array();
				if(is_array($kcity_classid)){
					foreach($kcity_classid as $val){
					    if($val!=\'\'){
					       
                            $cityname[]=$city_name[$val];
                            
                        }
					}
				}
                //'.$name.'[$k][\'citylist\']=@implode("/",$cityname);
				'.$name.'[$k][\'expectcity\']=$cityname;
				//截取标题
				if($paramer[\'post_len\']){
					$postname[$k]=@implode(",",$jobname);
					'.$name.'[$k][\'job_post_n\']=mb_substr($postname[$k],0,$paramer[post_len],"utf-8");
				}
                if($paramer[\'city_len\']){
					$scityname[$k]=@implode("/",$cityname);
					'.$name.'[$k][\'city_name_n\']=mb_substr($scityname[$k],0,$paramer[city_len],"utf-8");
				}
				//  是否浏览过
				'.$name.'[$k][\'isLookEd\'] = 0;
				if(in_array($v[\'id\'], $lookResumeId)){
				    '.$name.'[$k][\'isLookEd\'] = 1;
				}
				if($paramer[isrck] == 1){
				//  是否已下载
				'.$name.'[$k][\'islink\'] = 0;
				foreach ($downList as $dv) {
                    if ($v[\'id\'] == $dv[\'eid\']) {
                        '.$name.'[$k][\'islink\'] = 1;
                    }
                }
                foreach ($freedownList as $dv) {
                    if ($v[\'id\'] == $dv[\'eid\']) {
                        if ($vipetime) {
                            '.$name.'[$k][\'islink\'] = 1;
                        }else{
                            '.$name.'[$k][\'islink\'] = 0;
                        }
                    }
                }
                //  是否已邀约
                '.$name.'[$k][\'userid_msg\'] = 0;
                foreach($userid_msg as $val){
                    if($v[\'uid\']==$val[\'uid\']){
                        '.$name.'[$k][\'userid_msg\'] = 1;
                        }
                    }
                }
			}
			foreach('.$name.' as $k=>$v){
               if($paramer[\'keyword\']){
					'.$name.'[$k][\'username_n\']=str_replace($paramer[\'keyword\'],"<font color=#FF6600 >".$paramer[\'keyword\']."</font>",$v[\'username_n\']);
					'.$name.'[$k][\'job_post\']=str_replace($paramer[\'keyword\'],"<font color=#FF6600 >".$paramer[\'keyword\']."</font>",'.$name.'[$k][\'job_post\']);
					'.$name.'[$k][\'job_post_n\']=str_replace($paramer[\'keyword\'],"<font color=#FF6600 >".$paramer[\'keyword\']."</font>",'.$name.'[$k][\'job_post_n\']);
					'.$name.'[$k][\'city_name_n\']=str_replace($paramer[\'keyword\'],"<font color=#FF6600 >".$paramer[\'keyword\']."</font>",'.$name.'[$k][\'city_name_n\']);
				}
            }

			
			if($paramer[\'keyword\']!=""&&!empty('.$name.')){
				addkeywords(\'5\',$paramer[\'keyword\']);
			}
		}';
		//自定义标签 END
		//global $DiyTagOutputStr;
		//$DiyTagOutputStr[]=$OutputStr;
		return SmartyOutputStr($this,$compiler,$_attr,'userlist',$name,$OutputStr,$name);
	}
}
class Smarty_Internal_Compile_Userlistelse extends Smarty_Internal_CompileBase{
	public function compile($args, $compiler, $parameter){
		$_attr = $this->getAttributes($compiler, $args);

		list($openTag, $nocache, $item, $key) = $this->closeTag($compiler, array('userlist'));
		$this->openTag($compiler, 'userlistelse', array('userlistelse', $nocache, $item, $key));

		return "<?php }\nif (!\$_smarty_tpl->tpl_vars[$item]->_loop) {\n?>";
	}
}
class Smarty_Internal_Compile_Userlistclose extends Smarty_Internal_CompileBase{
	public function compile($args, $compiler, $parameter){
		$_attr = $this->getAttributes($compiler, $args);
		if ($compiler->nocache) {
			$compiler->tag_nocache = true;
		}

		list($openTag, $compiler->nocache, $item, $key) = $this->closeTag($compiler, array('userlist', 'userlistelse'));

		return "<?php } ?>";
	}
}
