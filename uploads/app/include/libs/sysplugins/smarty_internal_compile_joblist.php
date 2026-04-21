<?php
class Smarty_Internal_Compile_Joblist extends Smarty_Internal_CompileBase{
    public $required_attributes = array('item');
    public $optional_attributes = array('name', 'key', 'limit', 'comlen', 'namelen', 'urgent', 'ispage', 'rec','report', 'hy', 'job1', 'job1_son', 'job_post', 'job1son', 'jobpost', 'jobids', 'pr', 'mun', 'provinceid', 'cityid', 'ltype', 'three_cityid','threecityid', 'type', 'edu', 'exp', 'sex', 'minsalary','maxsalary','keyword', 'sdate', 'cert', 'sdate', 'uptime', 'order', 'orderby', 'noid', 'nouid', 'jobwhere', 'bid', 'state','isshow','jobin','cityin','islt','noids','is_graduate','mun','welfare', 'com_id', 'depower','istop');
    public $shorttag_order = array('from', 'item', 'key', 'name');
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);

        $from = $_attr['from'];
        $item = $_attr['item'];
        $name = $_attr['item'];
        $name=str_replace('\'','',$name);
        $name=$name?$name:'List';$name='$'.$name;
        if (!strncmp("\$_smarty_tpl->tpl_vars[$item]", $from, strlen($item) + 24)) {
            $compiler->trigger_template_error("item variable {$item} may not be the same variable as at 'from'", $compiler->lex->taglineno);
        }

        //自定义标签 START
        $OutputStr='global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer='.ArrayToString($_attr,true).';
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
			$where .= " AND `uid` = \'$paramer[com_id]\'";
			// 单查某企业下职位，排除城市、行业类别（排除分站）参数
            if(isset($paramer[provinceid])){unset($paramer[provinceid]);}
            if(isset($paramer[cityid])){unset($paramer[cityid]);}
            if(isset($paramer[three_cityid])){unset($paramer[three_cityid]);}
            if(isset($paramer[hy])){unset($paramer[hy]);}
		}
		
		if (!empty($paramer[depower])) {
		    trim($paramer[depower]) != \'all\' && $where .= " AND `is_depower` = $paramer[depower]"; // all为降权和非降权均查询
		} else {
		    $where .= " AND `is_depower` = 2"; // 默认查询未降权的职位
		}

		//是否推荐职位
		if($paramer[rec]){
			
			$where.=" AND `rec_time`>=".time();
			
		}
		//企业认证条件
		if($paramer[\'cert\']){
			
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
			$where.= " and `status`=\'1\'";
		}else{
			$where.= " and `status`=\'0\'";
		}
		//公司体制
		if($paramer[pr]){
			$where .= " AND `pr` =$paramer[pr]";
		}
		//公司行业分类
		if($paramer[\'hy\']){
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
		if($paramer[\'jobwhere\']){
			$where .=" and ".$paramer[\'jobwhere\'];
		}
		//职位分类综合查询
		if($paramer[\'jobids\']){
			$where.= " AND (`job1` = \'$paramer[jobids]\' OR `job1_son`= \'$paramer[jobids]\' OR `job_post`=\'$paramer[jobids]\')";
		}
		//职位分类区间,不建议执行该查询
		if($paramer[\'jobin\']){
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
		if($paramer[\'cityid\']){
			$where .= " AND (`cityid` IN ($paramer[cityid]))";
		}
		//城市三级子类
		if($paramer[\'three_cityid\']){
			$where .= " AND (`three_cityid` IN ($paramer[three_cityid]))";
		}
		if($paramer[\'threecityid\']){
			$where .= " AND (`three_cityid` IN ($paramer[threecityid]))";
		}
		if($paramer[\'cityin\']){
			$where .= " AND `three_cityid` IN ($paramer[cityin])";
		}
		//学历
		if($paramer[edu]){
            $eduArr  = $comdata[\'job_edu\'];
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
            $expArr  = $comdata[\'job_exp\'];
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
            $where .=" AND `welfare` LIKE \'%".$welfarename."%\' ";
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
				$beginToday = strtotime(\'today\');
				$where.=" AND lastupdate>$beginToday";
			}else{
				$time=time();
				$uptime = $time-($paramer[uptime]*86400);
				$where.=" AND lastupdate>$uptime";
			}
		}else{
		    if($config[sy_datacycle_job]>0){	
                // 后台-页面设置-数据周期	        
				$uptime = strtotime(\'-\'.$config[sy_datacycle_job].\' day\');
				$where.=" AND lastupdate>$uptime";
		    }
		}		
		//按类似公司名称,不建议进行大数据量操作
		if($paramer[comname]){
			$where.=" AND `com_name` LIKE \'%".$paramer[comname]."%\'";
		}
		//按公司归属地,只适合查询一级城市分类
		if($paramer[com_pro]){
			$where.=" AND `com_provinceid` =\'".$paramer[com_pro]."\'";
		}
		// 关键字匹配
		if($paramer[keyword]){
		    $comuids    =   $db->select_all("company","`name` LIKE \'%".$paramer[\'keyword\']."%\' OR `shortname` LIKE \'%".$paramer[\'keyword\']."%\'","`uid`");
		    $cuidArr    =   array();
		    foreach($comuids as $v){
				$cuidArr[]=$v[\'uid\'];
			}
            $where1     =   array();
			$where1[]   =   "`name` LIKE \'%".$paramer[keyword]."%\'";
			if($config[\'job_full_text_search\'] == 1){
			    $where1[]   =   "`description` LIKE \'%".$paramer[keyword]."%\'";
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
					$class[]= "(provinceid = \'".$value."\' or cityid = \'".$value."\' or three_cityid = \'".$value."\')";
				}
				$where1[]=@implode(" or ",$class);
			}
			if($config[\'job_full_text_search\'] == 1){
                $jobClassId =   array();
                foreach($job_name as $k=>$v){
                    if(strpos($v,$paramer[keyword])!==false){
                        $jobClassId[]=$k;
                    }
                }
                if(!empty($jobClassId)){
                    $class = array();
                    foreach($jobClassId as $value){
                    
                        $class[]= "(job1_son = \'".$value."\' or job_post = \'".$value."\')";
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
			    $isZhiding = ($paramer[job1] || $paramer[job1_son] || $paramer[job1son] || $paramer[job_post] || $paramer[jobpost] || $paramer[\'jobwhere\'] || $paramer[\'jobids\'] || $paramer[\'jobin\'] || $paramer["job"] || $paramer[keyword]) ? true : false;
			}
			
			if($isZhiding){
			    $where.="  and `xsdate`>\'".time()."\'";			
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
			    $where.="  and `xsdate`>\'".time()."\'";			
			}else{
			    $where.=" AND false";
			}
        }
		//自定义查询条件，默认取代上面任何参数直接使用该语句
		if($paramer[where]){
			$where = $paramer[where];
		}

		//查询条数
		$limit = \'\';
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
				$order = " ORDER BY ".str_replace("\'","",$paramer[order])."  ";
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
		
		'.$name.' = $db->select_all("company_job",$where.$limit);

		if(is_array('.$name.') && !empty('.$name.')){
			$comuid=$jobid=array();
			foreach('.$name.' as $key=>$value){
				if(in_array($value[\'uid\'],$comuid)==false){$comuid[] = $value[\'uid\'];}
				if(in_array($value[\'id\'],$jobid)==false){$jobid[] = $value[\'id\'];} 
			}
			$comuids = @implode(\',\',$comuid);
			$jobids = @implode(\',\',$jobid);
			//减少曝光量统计维度 只有列表才统计
			if($paramer[ispage]){
				$db->update_all("company_job", "`jobexpoure` = `jobexpoure` + 1", "`id` in ($jobids)");
			}
			

			if($comuids){
				$r_uids=$db->select_all("company","`uid` IN (".$comuids.")","`uid`,`hy`,`shortname`,`welfare`,`hotstart`,`hottime`,`fact_status`");
				if(is_array($r_uids)){
					foreach($r_uids as $key=>$value){
						if($value[shortname]){
    						$value[\'shortname_n\'] = $value[shortname];
    					}
						if($value[\'hotstart\']<=time() && $value[\'hottime\']>=time()){
							$value[\'hotlogo\'] = 1;
						}
                        $value[\'hy_n\'] = $industry_name[$value[hy]];
						$r_uid[$value[\'uid\']] = $value;
					}
				}
			}
			
 			if($paramer[bid]){
				$noids=array();
			}	
			if ($_COOKIE[\'uid\'] && $_COOKIE[\'usertype\']==1){
			    $lookJob =   $db->select_all("look_job","`uid` = ".$_COOKIE[\'uid\'], "`jobid`");
			    if (!empty($lookJob)){
			        foreach($lookJob as $key=>$value){
						$lookJobIdArr[] = $value[\'jobid\'];
					}
			    }
			}
			foreach('.$name.' as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value[\'id\'],$noids)){
					unset('.$name.'[$key]);
					continue;
				}else{
					'.$name.'[$key] = $db->array_action($value,$cache_array);
					'.$name.'[$key][stime] = date("Y-m-d",$value[sdate]);
					'.$name.'[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data[\'sex\'][$value[\'sex\']]){
						'.$name.'[$key][sex_n]=$arr_data[\'sex\'][$value[\'sex\']];
					}
					'.$name.'[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					'.$name.'[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value[\'uid\']][shortname]){
						'.$name.'[$key][com_name] =$r_uid[$value[\'uid\']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        '.$name.'[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        '.$name.'[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    '.$name.'[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     '.$name.'[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    '.$name.'[$key][job_number] = "";
					}else{
					    '.$name.'[$key][job_number] = $value[zp_num]." 人";
					}			
                    '.$name.'[$key][hotlogo] = $r_uid[$value[\'uid\']][hotlogo];
                    '.$name.'[$key][hy_n] = $r_uid[$value[\'uid\']][hy_n];
                    '.$name.'[$key][fact_status] = $r_uid[$value[\'uid\']][fact_status];
					'.$name.'[$key][logo] = checkpic($value[\'com_logo\'],$config[\'sy_unit_icon\']);
					'.$name.'[$key][pr_n] = $comclass_name[$value[pr]];
					'.$name.'[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value[\'lastupdate\'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date(\'m\'),date(\'d\'),date(\'Y\'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date(\'m\'),date(\'d\')-1,date(\'Y\'));
					
					if($time>$beginYesterday && $time<$beginToday){
						'.$name.'[$key][\'time\'] ="昨天";
					}elseif($time>$beginToday){	
						'.$name.'[$key][\'time\'] = '.$name.'[$key][\'lastupdate\'];
						'.$name.'[$key][\'redtime\'] =1;
					}else{
						'.$name.'[$key][\'time\'] = date("Y-m-d",$value[\'lastupdate\']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date(\'m\'),date(\'d\')-2,date(\'Y\'));

					if($value[\'sdate\']>$beforeYesterday){
						'.$name.'[$key][\'newtime\'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(\' \', \'\',$value[welfare]);
						$welfareList = @explode(\',\',trim($value[welfare]));

						if(!empty($welfareList)){
							'.$name.'[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value[\'uid\']][welfare]){
						$welfareList = @explode(\',\',trim($r_uid[$value[\'uid\']][welfare]));
						'.$name.'[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value[\'uid\']][shortname]){
							'.$name.'[$key][com_n] = mb_substr($r_uid[$value[\'uid\']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							'.$name.'[$key][com_n] = mb_substr($value[\'com_name\'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value[\'rec_time\']>time()){
							'.$name.'[$key][name_n] = "<font color=\'red\'>".mb_substr($value[\'name\'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							'.$name.'[$key][name_n] = mb_substr($value[\'name\'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value[\'rec_time\']>time()){
							'.$name.'[$key][\'name_n\'] = "<font color=\'red\'>".$value[\'name\']."</font>";
						}else{
							'.$name.'[$key][name_n] = $value[\'name\'];
						}
					}
					//构建职位伪静态URL
					'.$name.'[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					'.$name.'[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							'.$name.'[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								'.$name.'[$key][ratlogo] = checkpic($v[com_pic]);
							}
							'.$name.'[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						'.$name.'[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",'.$name.'[$key][name_n]);
						'.$name.'[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",'.$name.'[$key][com_n]);
						'.$name.'[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						'.$name.'[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    '.$name.'[$key][\'isLookEd\'] = 0;
                    if(in_array($value[\'id\'], $lookJobIdArr)){
                        '.$name.'[$key][\'isLookEd\'] = 1;
                    }
				}
			}
			if(is_array('.$name.')){
				if($paramer[keyword]!=""&&!empty('.$name.')){
					addkeywords(\'3\',$paramer[keyword]);
				}
			}
		}';
        //global $DiyTagOutputStr;
        //$DiyTagOutputStr[]=$OutputStr;
        return SmartyOutputStr($this,$compiler,$_attr,'joblist',$name,$OutputStr,$name);
    }
}
class Smarty_Internal_Compile_Joblistelse extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);

        list($openTag, $nocache, $item, $key) = $this->closeTag($compiler, array('joblist'));
        $this->openTag($compiler, 'joblistelse', array('joblistelse', $nocache, $item, $key));

        return "<?php }\nif (!\$_smarty_tpl->tpl_vars[$item]->_loop) {\n?>";
    }
}
class Smarty_Internal_Compile_Joblistclose extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);
        if ($compiler->nocache) {
            $compiler->tag_nocache = true;
        }

        list($openTag, $compiler->nocache, $item, $key) = $this->closeTag($compiler, array('joblist', 'joblistelse'));

        return "<?php } ?>";
    }
}
