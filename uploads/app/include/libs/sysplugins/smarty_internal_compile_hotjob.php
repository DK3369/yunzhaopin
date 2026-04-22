<?php
class Smarty_Internal_Compile_Hotjob extends Smarty_Internal_CompileBase{
    public $required_attributes = array('item');
    public $optional_attributes = array('name', 'key', 'limit');
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

        $where=$_attr['where'];
        $ispage=$_attr['ispage'];
		$OutputStr='global $db,$db_config,$config;$paramer='.ArrayToString($_attr,true).';'.$name.'=array();
		
		$time = time();
		//处理传入参数，并且构造分页参数
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[\'arr\'];
		$Purl =  $ParamerArr[\'purl\'];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		//是否属于分站下
		if($config[sy_web_site]=="1"){
			$jobwheres="";
			if($config[province]>0 && $config[province]!=""){
				$jobwheres.=" and `provinceid`=\'$config[province]\'";
			}
			if($config[cityid]>0 && $config[cityid]!=""){
				$jobwheres.=" and `cityid`=\'$config[cityid]\'";
			}
			if($config[three_cityid]>0 && $config[three_cityid]!=""){
				$jobwheres.=" and `three_cityid`=\'$config[three_cityid]\'";
			}
			if($config[hyclass]>0 && $config[hyclass]!=""){
				$jobwheres.=" and `hy`=\'".$config[hyclass]."\'";
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
        if($paramer[\'limit\']){
			$limit=" limit ".$paramer[\'limit\'];
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

        if($config[\'hotcom_top\'] == 1){
            // 职位更新时间(职位修改时，会更新名企表lastupdate字段)
            $order = " ORDER BY `lastupdate` DESC ";
        }elseif($config[\'hotcom_top\'] == 2){
            // 随机
            $order = " ORDER BY rand() ";
        }else{
            // 后台手动设置
            $order = " ORDER BY `sort` DESC ";
        };
		//分页
		if($ispage){
			$limit = PageNav($paramer,$_GET,"hotjob",$where,$Purl,\'0\',$_smarty_tpl);;
		}
		$where.=$order;
        
        $Query = $db->query("SELECT * FROM $db_config[def]hotjob where ".$where.$limit); 
		while($rs = $db->fetch_array($Query)){
			'.$name.'[] = $rs;
			$ListId[] =  $rs[uid];
		}
        
		//是否需要查询对应职位
		$ComId  =   @implode(",",$ListId);
		$comList=   $db->select_all("company","`uid` IN ($ComId)","`shortname`,`uid`,`hy`,`mun`,`provinceid`,`cityid`,`three_cityid`, `r_status`");
		
		if($config[sy_datacycle_job]>0){
			    
		    $uptime =   strtotime(\'-\'.$config[sy_datacycle_job].\' day\');
		    $JobList=   $db->select_all("company_job","`uid` IN ($ComId) and state=1 and r_status=1 and status=0 and lastupdate > $uptime $jobwheres","`id`,`uid`,`name`");
	    }else{
	    
	        $JobList=   $db->select_all("company_job","`uid` IN ($ComId) and state=1 and r_status=1 and status=0 $jobwheres","`id`,`uid`,`name`");
	    }
		
		$statis=$db->select_all("company_statis","`uid` IN ($ComId)","`uid`,`comtpl`");
		if(is_array($ListId)){
		    
		    foreach('.$name.' as $key=>$value){
				foreach($comList as $v){
					if($v[\'uid\'] == $value[\'uid\']){
					    if($v[\'r_status\'] != 1){ 
					        unset('.$name.'[$key]);
					    }
					}
				}
			}
		    $JobIds =   array();
			//处理类别字段
			$cache_array = $db->cacheget();
			foreach('.$name.' as $key=>$value){
				'.$name.'[$key]["hot_pic"]=checkpic($value[hot_pic],$config[sy_unit_icon]);
				foreach($comList as $v){
				    
					if($value[\'uid\']==$v[\'uid\']){
						if($v[\'shortname\']){
							'.$name.'[$key]["username"]= $v[shortname];
						}
						'.$name.'[$key]["hy"]= $cache_array[industry_name][$v[hy]];
						'.$name.'[$key]["mun_n"]= $cache_array[comclass_name][$v[mun]];
						'.$name.'[$key]["job_city_one"]= $cache_array[city_name][$v[provinceid]];
						'.$name.'[$key]["job_city_two"]= $cache_array[city_name][$v[cityid]];
					}
				}
				$i=0;$j=0;
				'.$name.'[$key]["num"]=0;
				if(is_array($JobList)){
					foreach($JobList as $ke=>$va){ 
						if($value[uid]==$va[uid]){
							if($j<3){
								$hotjob_url = Url("job",array("c"=>"comapply","id"=>"$va[id]"),"1");
								$curl=  Url("company",array("c"=>"show","id"=>$value[uid]));
								$va[name] = mb_substr($va[name],0,8,"utf-8");
								'.$name.'[$key]["hotjob"].="<div class=\'index_mq_box_cont_showjoblist\'><a href=\"$hotjob_url\">".$va[name]."</a></div>";
						        $JobIds[] = $va[\'id\'];
							}else{
                                if($j==3){
                                    '.$name.'[$key]["hotjob"].="<div class=\'index_mq_box_cont_showjobmore\'><a href=\"$curl\">更多职位</a></div>";
							     }
							}
                            $j++;
						}
					}

					
					'.$name.'[$key]["job"].="<div class=\"area_left\"> ";
					
					foreach($JobList as $k=>$v){
						if($value[uid]==$v[uid] && $i<5){
							$job_url = Url("job",array("c"=>"comapply","id"=>"$v[id]"),"1");
							$v[name] = mb_substr($v[name],0,10,"utf-8");
							'.$name.'[$key]["job"].="<a href=\'".$job_url."\'>".$v[name]."</a>";
							$i++;
						}
						if($value[uid]==$v[uid]){
							'.$name.'[$key]["num"]='.$name.'[$key]["num"]+1;
						}
					}

					foreach($statis as $v){
						if($value[\'uid\']==$v[\'uid\']){
							if($v[\'comtpl\'] && $v[\'comtpl\']!="default"){
								$jobs_url = Url("company",array("c"=>"show","id"=>$value[uid]))."#job";
							}else{
								$jobs_url = Url("company",array("c"=>"show","id"=>$value[uid]));
							}
						}
					}
					$com_url = Url("company",array("c"=>"show","id"=>$value[uid]));
					'.$name.'[$key]["job"].="</div><div class=\"area_right\"><a href=\'".$com_url."\'>".$value["username"]."</a></div>";
					'.$name.'[$key]["url"]=$com_url;
				}
			}
			if(!empty($JobIds)){
			    //$db -> update_all("company_job", "`jobexpoure` = `jobexpoure` + 1", "`id` IN (".@implode(\',\',$JobIds).")");
			}
		}';
        //自定义标签 END
       // global $DiyTagOutputStr;
      //  $DiyTagOutputStr[]=$OutputStr;
        return SmartyOutputStr($this,$compiler,$_attr,'hotjob',$name,$OutputStr,$name);
    }
}
class Smarty_Internal_Compile_Hotjobelse extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);

        list($openTag, $nocache, $item, $key) = $this->closeTag($compiler, array('hotjob'));
        $this->openTag($compiler, 'hotjobelse', array('hotjobelse', $nocache, $item, $key));

        return "<?php }\nif (!\$_smarty_tpl->tpl_vars[$item]->_loop) {\n?>";
    }
}
class Smarty_Internal_Compile_Hotjobclose extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);
        if ($compiler->nocache) {
            $compiler->tag_nocache = true;
        }

        list($openTag, $compiler->nocache, $item, $key) = $this->closeTag($compiler, array('hotjob', 'hotjobelse'));

        return "<?php } ?>";
    }
}
