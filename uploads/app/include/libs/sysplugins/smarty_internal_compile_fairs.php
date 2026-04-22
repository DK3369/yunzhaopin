<?php
class Smarty_Internal_Compile_Fairs extends Smarty_Internal_CompileBase{
    public $required_attributes = array('item');
    public $optional_attributes = array('name', 'key', 'len', 'sort','limit', 'ispage','state','type');
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
        $OutputStr=''.$name.'=array();$time=time();$paramer='.ArrayToString($_attr,true).';
		global $db,$db_config,$config;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		$Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		$where = "1";
        //根据后台设置开启/隐藏
        $where .= " and `is_open`=1";
		if($config[\'did\']){
			$where.=" and `did`=\'".$config[\'did\']."\'";
		}
        if($paramer[state]==\'1\'){//尚未开始
			$where .=" AND unix_timestamp(`starttime`)>\'$time\'";
		}elseif($paramer[state]==\'2\'){//进行中
			$where .=" AND unix_timestamp(`starttime`)<\'$time\' AND unix_timestamp(`endtime`)>\'$time\'";
		}elseif($paramer[state]==\'3\'){//已结束
			$where .=" AND unix_timestamp(`endtime`)<\'$time\'";
		}
		//查询条数
		if($paramer[limit]){
			$limit=" LIMIT ".$paramer[limit];
		}else{
			$limit=" LIMIT 20";
		}
		if($paramer[ispage]){
            if(isset($paramer["type"])) {
               $Purl["type"] = $paramer["type"]; 
            }
			$limit = PageNav($paramer,$_GET,"zhaopinhui",$where,$Purl,"",6,$_smarty_tpl);
		}
        $where .= " ORDER BY";
        $select = "`id`,`starttime`,`endtime`,`title`,`is_themb`,`is_themb_wap`,`address`";
        if($paramer[state]){
            // 单类型获取，直接按开始时间排序
            $where .= " unix_timestamp(`starttime`)";
            if($paramer[sort]){
    			$where .= " $paramer[sort]";
    		}else{
    			$where .= " ASC ";
    		}
        }else{
            // 查询招聘列表，判断有没有未结束的招聘会，确定排序
            $zphlist  =  $db->DB_select_once("zhaopinhui","unix_timestamp(`endtime`)>\'$time\'","`id`");
            // 有未结束的，条件查询
            if(!empty($zphlist)){
                // 条件排序，进行中在最上面，接着是未开始，最后是已结束
                $select .= ",CASE WHEN unix_timestamp(`endtime`)>\'$time\' THEN unix_timestamp(`starttime`)";
        		$select .= " WHEN unix_timestamp(`endtime`)<\'$time\' THEN -1*unix_timestamp(`starttime`) END AS `zph_px`";
                $where .= " CASE";
                $where .= " WHEN unix_timestamp(`starttime`)<\'$time\' AND unix_timestamp(`endtime`)>\'$time\' THEN 0";
                $where .= " WHEN unix_timestamp(`starttime`)>\'$time\' THEN 1";
                $where .= " WHEN unix_timestamp(`endtime`)<\'$time\' THEN 2";
                $where .= " END, `zph_px` ASC";
            }else{
                // 全部结束。按时间开始时间倒序排列
                $where .= " unix_timestamp(`starttime`) DESC";
            }
        }
		'.$name.'=$db->select_all("zhaopinhui",$where.$limit, $select);
		if(is_array('.$name.')){
			foreach('.$name.' as $key=>$v){
				$array_zid[]=$v[id];
			}
            if(!empty($array_zid)){
                $uids = array();// 初始化参会企业uid数组
                $zphcom = array();// 招聘会参加企业信息
                $rows=$db->select_all("zhaopinhui_com","`zid` in (".implode(\',\',$array_zid).") and `status`=1","`uid`,`zid`,`jobid`");
                foreach($rows as $k => $va){
                    !in_array($va[uid], $uids) && array_push($uids, $va[uid]);
                    $zphcom[$va[zid]][$va[uid]][jids] = $va[jobid];
				}
				$comjobs = $db->select_all("company_job","`uid` in (".implode(\',\',$uids).") and `state`=1 and `status`=0 and `r_status`=1","`id`, `uid`, `zp_num`");
				$jobids = array();// 参会各企业所有在招岗位id
				$jid2zpnum = array();// 职位id转招聘人数
				foreach ($comjobs as $comjob){
				    $jobids[$comjob[uid]][] = $comjob[id];
				    $jid2zpnum[$comjob[id]] = $comjob[zp_num];
				}
				foreach ($zphcom as $zphid => $v) {
				    foreach ($v as $cuid => $vv) {
				        $jobcnt = $zpnum = 0;// 初始化企业参会职位数、招聘人数
				        if ($vv[jids]) {
				            $jobidsarr = array_unique(@explode(",",$vv[jids]));
				        } else {
				            $jobidsarr = isset($jobids[$cuid]) && $jobids[$cuid] ? $jobids[$cuid] : array();
				        }
				        if (isset($jobids[$cuid]) && $jobids[$cuid]) {
                            foreach ($jobidsarr as $jid) {
                                if (in_array($jid, $jobids[$cuid])){
                                    $jobcnt++;
                                    $zpnum += $jid2zpnum[$jid];
                                }
                            }
				        }
                        $zphcom[$zphid][$cuid][jobnum] = $jobcnt;// 企业参会职位数
                        $zphcom[$zphid][$cuid][zpnum] = $zpnum;// 企业参会招聘人数
                        unset($zphcom[$zphid][$cuid][jids]);
				    }
				}
            }
			$weekarray=array("日","一","二","三","四","五","六");
			foreach('.$name.' as $key=>$v){
                '.$name.'[$key]["comnum"]=0;
    			'.$name.'[$key]["jobnum"]=0;
                '.$name.'[$key]["zpnum"]=0;
                foreach ($zphcom as $zid => $vv) {
                    if ($v[id] == $zid) {
                        foreach ($vv as $val) {
                            '.$name.'[$key]["comnum"]++;
                            '.$name.'[$key]["jobnum"] += $val[jobnum];
                            '.$name.'[$key]["zpnum"] += $val[zpnum];
                        }
                    }
                }
				'.$name.'[$key]["starttime_n"]=date(\'Y\',strtotime($v[starttime])).\'年\'.date(\'n\',strtotime($v[starttime]))."月";
				'.$name.'[$key]["stime"]=strtotime($v[starttime])-time();
				'.$name.'[$key]["etime"]=strtotime($v[endtime])-time();
				if($paramer[len]){
					'.$name.'[$key]["title"]=mb_substr($v[\'title\'],0,$paramer[len],"utf-8");
				}
				'.$name.'[$key]["url"]=Url("zph",array("c"=>\'show\',"id"=>$v[\'id\']),"1");
                '.$name.'[$key]["week"] = $weekarray[date(\'w\',strtotime($v[\'starttime\']))];

				'.$name.'[$key][\'is_themb\'] = checkpic($v[\'is_themb\'],$config[\'sy_zph_icon\']);
                '.$name.'[$key][\'is_themb_wap\'] = checkpic($v[\'is_themb_wap\'],'.$name.'[$key][\'is_themb\']);
			}
		}';
        //自定义标签 END
        // global $DiyTagOutputStr;
        // $DiyTagOutputStr[]=$OutputStr;
        return SmartyOutputStr($this,$compiler,$_attr,'fairs',$name,$OutputStr,$name);
    }
}
class Smarty_Internal_Compile_Fairselse extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);

        list($openTag, $nocache, $item, $key) = $this->closeTag($compiler, array('fairs'));
        $this->openTag($compiler, 'fairselse', array('fairselse', $nocache, $item, $key));

        return "<?php }\nif (!\$_smarty_tpl->tpl_vars[$item]->_loop) {\n?>";
    }
}
class Smarty_Internal_Compile_Fairsclose extends Smarty_Internal_CompileBase{
    public function compile($args, $compiler, $parameter){
        $_attr = $this->getAttributes($compiler, $args);
        if ($compiler->nocache) {
            $compiler->tag_nocache = true;
        }

        list($openTag, $compiler->nocache, $item, $key) = $this->closeTag($compiler, array('fairs', 'fairselse'));

        return "<?php } ?>";
    }
}
