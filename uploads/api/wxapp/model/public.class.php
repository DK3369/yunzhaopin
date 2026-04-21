<?php
/*
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class public_controller extends wxapp_controller{
	function register_action(){
	    $data['sy_webname']				=	$this->config['sy_webname'];
	    $data['reg_moblie']				=	$this->config['reg_moblie'];
	    $data['reg_email']				=	$this->config['reg_email'];
	    $data['reg_user']				=	$this->config['reg_user'];
	    $data['reg_real_name_check']	=	$this->config['reg_real_name_check'];
	    $data['reg_passconfirm']		=	$this->config['reg_passconfirm'];
	    $data['sy_msg_regcode']			=	$this->config['sy_msg_regcode'];
	    $this->render_json(0,'ok',$data);
	}
	//职位多列选择器
	function multiJob_action(){
	    $cacheM		=	$this->MODEL('cache');
	    $cacheList	=	$cacheM->GetCache(array('job','jobfs'));
	    $job_index	=	$cacheList['job_index'];
	    $job_type	=	$cacheList['job_type'];
	    $job_name	=	$cacheList['job_name'];
	    $job_three  =   $cacheList['job_three'];
	    
	    $jobone	 =	$jobtwo	 =	$jobthree  =  array();
	    $jobtwoArr		=	array();
	    $jobthreetwoArr	=	array();
	    
	    if(is_array($job_index)){
	    	foreach($job_index as $k=>$v){
	    	    
	    	    if(!empty($job_index)){
					$jobtwoArr[$v][] 	   =  array('value'=>0,'label'=>'全部');//第二列 全部
					$jobthreetwoArr[$v][]  =  array(array());//用做 一级-全部-''
					
		    		if (is_array($job_type[$v])){
						
		            	foreach ($job_type[$v] as $ka=>$va){

							$jobtwoArr[$v][]	=	array('value'=>$va,'label'=>$job_name[$va]);

							$jobthreeArr		=	array();

							if (!empty($job_three)){
							    //只要有一个三级， 第三列都要有全部
							    $jobthreeArr[]  =	array('value'=>0,'label'=>'全部');
							    if (!empty($job_type[$va])){
							        foreach ($job_type[$va] as $key=>$val){
							            $jobthreeArr[]	=	array('value'=>$val,'label'=>$job_name[$val]);
							        }
							    }
							    $jobthreetwoArr[$v][]   =	$jobthreeArr;
							}
		                }
					}
				}
	            $jobone[]	=	array('value'=>$v,'label'=>$job_name[$v]);
	        }
	    }

	    $jobtwo	   =  array_values($jobtwoArr);
	    $jobthree  =  array_values($jobthreetwoArr);
	    
	    $data['jobone']	=	$jobone;
	    if(count($jobtwo)>0){
	        $data['jobtwo']=	$jobtwo;
	    }else{
	    	$data['jobtwo']=	array();
	    }
	    if (!empty($job_three)){
	        $data['jobthree']=	$jobthree;
	    }else{
	        $data['jobthree']  =  array();
	    }
	    
	    $this->render_json(0,'ok',$data);
	}
	
	//城市多列选择器
	function multiCity_action(){
	    $cacheM		=	$this->MODEL('cache');
	    $cacheList	=	$cacheM->GetCache(array('city','cityfs'));
	    $city_index	=	$cacheList['city_index'];
	    $city_type	=	$cacheList['city_type'];
	    $city_name	=	$cacheList['city_name'];
	    $city_three =   $cacheList['city_three'];
	    
	    $cityone	=	$citytwo	=	$citythree	=	array();
	    $citytwoArr		  =	 array();
	    $citythreetwoArr  =	 array();
	    
	    if(is_array($city_index)){

	    	foreach($city_index as $k=>$v){
	    		if(!empty($city_type)){
					$citytwoArr[$v][]       =  array('value'=>0,'label'=>'全部');//第二列 全部
		    		$citythreetwoArr[$v][]	=  array(array());//用做 一级-全部-''
		    		if (is_array($city_type[$v])){

	    				foreach ($city_type[$v] as $ka=>$va){

							$citytwoArr[$v][]	=	array('value'=>$va,'label'=>$city_name[$va]);
							
							$citythreeArr  =  array();
							if (!empty($city_three)){
							    //只要有一个三级， 第三列都要有全部
							    $citythreeArr[] 		 =	array('value'=>0,'label'=>'全部');
							    if (!empty($city_type[$va])){
							        foreach ($city_type[$va] as $key=>$val){
							            $citythreeArr[]  =	array('value'=>$val,'label'=>$city_name[$val]);
							        }
							    }
							    $citythreetwoArr[$v][]   =	$citythreeArr;
							}
		                }
					}
				}

	            $cityone[]	=	array('value'=>$v,'label'=>$city_name[$v]);
	        }
	    }

	    $citytwo	=	array_values($citytwoArr);
	    
	    $citythree	=	array_values($citythreetwoArr);
	    
	    $data['cityone']	=	$cityone;
	    if(count($citytwo)>0){
	        $data['citytwo']=	$citytwo;
	    }else{
	    	$data['citytwo']=	array();
	    }
	    if (!empty($city_three)){
	        $data['citythree']  =  $citythree;
	    }else{
	        $data['citythree']  =  array();
	    }
	    
	    $this->render_json(0,'ok',$data);
	}
	
	//列表页搜索取缓存
	function searchcity_action(){
	    include(PLUS_PATH.'city.cache.php');
	    include(PLUS_PATH.'cityparent.cache.php');
	    $data['city_index']	=	$city_index;
	    $data['city_type']	=	$city_type;
	    $data['city_name']	=	$city_name;
	    $data['city_parent']=	$city_parent;
	    $this->render_json(0,'ok',$data);
	}
	//列表页搜索取缓存
	function searchjob_action(){
	    include(PLUS_PATH.'job.cache.php');
	    include(PLUS_PATH.'jobparent.cache.php');
	    $data['job_index']	=	$job_index;
	    $data['job_type']	=	$job_type;
	    $data['job_name']	=	$job_name;
	    $data['job_parent']	=	$job_parent;
	    $this->render_json(0,'ok',$data);
	}
	
	function industry_action(){//获取行业类别
		include PLUS_PATH.'industry.cache.php';
		$id=array();$name=array();
		if(is_array($industry_index)){
		    $id[]	=	'0';
		    $name[]	=	'不限';
			foreach($industry_index as $k=>$v){
				$id[]		=	$v;
				$name[]		=	$industry_name[$v];
			}
		}
		$hy['id']			=	$id;
		$hy['name']			=	$name;
		$data['hy']			=	$hy;
		$this->render_json(0,'ok',$data);
	}
	function parts_action(){
	    
	    $cacheM  =  $this -> MODEL('cache');
	    $cache   =  $cacheM -> GetCache('part');
		
	    if(!empty($cache['part_sex'])){
	        $id[]	 =	0;
	        $name[]  =	'请选择性别';
	        foreach($cache['part_sex'] as $k=>$v){
	            $id[]	 =	$k;
	            $name[]  =	$v;
	        }
	    }
	    $sex['id']		 =	$id;
	    $sex['name']	 =	$name;
	    $partdata['sex']  =	$sex;
	    
	    $part_type  =  $cache['partdata']['part_type'];
	    if(!empty($part_type)){
		    $typeid[]	=	'0';
			$typename[]	=	'请选择兼职类型';
			foreach($part_type as $v){
				$typeid[]	=	$v;
				$typename[]	=	$cache['partclass_name'][$v];
			}
		}
		$type['id']		=	$typeid;
		$type['name']	=	$typename;
		$partdata['type']	=	$type;
		
		$part_salary_type  =  $cache['partdata']['part_salary_type'];
		if(!empty($part_salary_type)){
		    $salary_typeid[]	=	'0';
			$salary_typename[]	=	'请选择薪水类型';
			foreach($part_salary_type as $v){
				$salary_typeid[]	=	$v;
				$salary_typename[]	=	$cache['partclass_name'][$v];
			}
		}
		$salary_type['id']		=	$salary_typeid;
		$salary_type['name']	=	$salary_typename;
		$partdata['salary_type']	=	$salary_type;
		
		$part_billing_cycle  =  $cache['partdata']['part_billing_cycle'];
		if(!empty($part_billing_cycle)){
		    $billing_cycleid[]		=	'0';
			$billing_cyclename[]	=	'请选择结算周期';
			foreach($part_billing_cycle as $v){
				$billing_cycleid[]		=	$v;
				$billing_cyclename[]	=	$cache['partclass_name'][$v];
			}
		}
		$billing_cycle['id']	=	$billing_cycleid;
		$billing_cycle['name']	=	$billing_cyclename;
		$partdata['billing_cycle']	=	$billing_cycle;
		
		$partdata['timetype']  =  array(
		    'id'    =>  array(1,2),
		    'name'  =>  array('短期招聘', '长期招聘')
		);
		
		$data['partdata']  =  $partdata;
		
		if (!empty($_POST['uid']) && !empty($_POST['partid'])){
		    $partM	=	$this -> MODEL('part');
		    $row	=	$partM -> getInfo(array('id'=>$_POST['partid']),array('field'=>'`sex`,`salary_type`,`type`,`billing_cycle`,`edate`,`worktime`'));
		    $row  = $row['info'];
            foreach ($sex['id'] as $k=>$v){
		        if ($row && $row['sex']==$v){
		            $sexIndex = $k;
		        }
		    } 
		    foreach ($salary_type['id'] as $k=>$v){
		        if ($row && $row['salary_type']==$v){
		            $salarytypeIndex = $k;
		        }
		    } 
		    foreach ($type['id'] as $k=>$v){
		        if ($row && $row['type']==$v){
		            $typeIndex = $k;
		        }
		    }
		    foreach ($billing_cycle['id'] as $k=>$v){
		        if ($row && $row['billing_cycle']==$v){
		            $billingcycleIndex = $k;
		        }
		    }
		    if(empty($row['edate'])){
		        $timetypeIndex = 1;
		    }
		    $worktime  =  explode(',', $row['worktime']);
		}
		$data['sexIndex']			=  $sexIndex?$sexIndex:0;
		$data['salarytypeIndex']	=  $salarytypeIndex?$salarytypeIndex:0;
		$data['typeIndex']			=  $typeIndex?$typeIndex:0;
		$data['billingcycleIndex']	=  $billingcycleIndex?$billingcycleIndex:0;
		$data['timetypeIndex']	    =  $timetypeIndex?$timetypeIndex:0;
		
		$morning	=	array('0101','0201','0301','0401','0501','0601','0701');
		$noon		=	array('0102','0202','0302','0402','0502','0602','0702');
		$afternoon	=	array('0103','0203','0303','0403','0503','0603','0703');

		if(!empty($cache['part_morning'])){
		    foreach($cache['part_morning'] as $k=>$v){
				$mor[$k]['name']	=	$v;
				
				if (!empty($worktime)){
				    if (in_array($v, $worktime)){
				        $mor[$k]['checked']  =  true;
				    }else{
				        $mor[$k]['checked']  =  false;
				    }
				}else {
				    $mor[$k]['checked']  =  false;
				}
			}
		}
		$data['morning']=	$mor;
		if(!empty($cache['part_noon'])){
		    foreach($cache['part_noon'] as $k=>$v){
				$noo[$k]['name']	=	$v;
				if (!empty($worktime)){
				    if (in_array($v, $worktime)){
				        $noo[$k]['checked']  =  true;
				    }else{
				        $noo[$k]['checked']  =  false;
				    }
				}else {
				    $noo[$k]['checked']  =  false;
				}
			}
		}
		$data['noon']	=	$noo;
		if(!empty($cache['part_afternoon'])){
		    foreach($cache['part_afternoon'] as $k=>$v){
				$aft[$k]['name']	=	$v;
				if (!empty($worktime)){
				    if (in_array($v, $worktime)){
				        $aft[$k]['checked']  =  true;
				    }else{
				        $aft[$k]['checked']  =  false;
				    }
				}else {
				    $aft[$k]['checked']  =  false;
				}
			}
		}
		$data['night']	=	$aft;
		
		$this->render_json(0,'ok',$data);
	}
	
	function user_action(){//获取个人类别
	    
	    $cacheM  =  $this -> MODEL('cache');
	    $cache   =  $cacheM -> GetCache('user');
        include(CONFIG_PATH.'db.data.php');

	    $user_word  =  $cache['userdata']['user_word'];
	    if(!empty($user_word)){
		    $expid[]	=	'0';
			$expname[]	=	'请选择';
			foreach($user_word as $v){
				$expid[]	=	$v;
				$expname[]	=	$cache['userclass_name'][$v];
				$arrexp[]	=	array('id'=>$v,'name'=>$cache['userclass_name'][$v]);
			}
		}
		$exp['id']		=	$expid;
		$exp['name']	=	$expname;
		$userdata['exp']	=	$exp;
		$userdata['exp_arr']=	$arrexp;
		
		$user_edu  =  $cache['userdata']['user_edu'];
		if(!empty($user_edu)){
		    $eduid[]	=	'0';
			$eduname[]	=	'请选择';
			foreach($user_edu as $v){
				$eduid[]	=	$v;
				$eduname[]	=	$cache['userclass_name'][$v];
				$arredu[]	=	array('id'=>$v,'name'=>$cache['userclass_name'][$v]);
			}
		}
		$edu['id']		=	$eduid;
		$edu['name']	=	$eduname;
		$userdata['edu']	=	$edu;
		$userdata['edu_arr']=	$arredu;
		
		$user_ing  =  $cache['userdata']['user_ing'];
		if(!empty($user_ing)){
		    $ingid[]	=	'0';
			$ingname[]	=	'请选择熟练程度';
			foreach($user_ing as $v){
				$ingid[]	=	$v;
				$ingname[]	=	$cache['userclass_name'][$v];
			}
		}
		$ing['id']		=	$ingid;
		$ing['name']	=	$ingname;
		$userdata['ing']	=	$ing;
		
		$user_type  =  $cache['userdata']['user_type'];
		if(is_array($user_type)){
			foreach($user_type as $v){
				$typeid[]	=	$v;
				$typename[]	=	$cache['userclass_name'][$v];
				$arrutype[]	=	array('id'=>$v,'name'=>$cache['userclass_name'][$v]);
			}
		}
		$type['id']		=	$typeid;
		$type['name']	=	$typename;
		$userdata['type']	=	$type;
		$userdata['utype_arr']	=	$arrutype;
		
		$user_report  =  $cache['userdata']['user_report'];
		if(!empty($user_report)){
			foreach($user_report as $v){
				$reportid[]		=	$v;
				$reportname[]	=	$cache['userclass_name'][$v];
				$arrreport[]	=	array('id'=>$v,'name'=>$cache['userclass_name'][$v]);
			}
		}
		$report['id']	=	$reportid;
		$report['name']	=	$reportname;
		$userdata['report']		=	$report;
		$userdata['report_arr']	=	$arrreport;


		$user_integrity = $arr_data['integrity_name'];
		if(!empty($user_integrity)){
			foreach($user_integrity as $k=>$v){
				$integrityid[]		=	$k;
				$integrityname[]	=	$v;
				$arrintegrity[]		=	array('id'=>$k,'name'=>$v);
			}
		}
		$integrity['id']	=	$integrityid;
		$integrity['name']	=	$integrityname;
		$userdata['integrity']		=	$integrity;
		$userdata['integrity_arr']	=	$arrintegrity;
		
		$user_marriage  =  $cache['userdata']['user_marriage'];
		if(!empty($user_marriage)){
			$marriageid[]    =  '0';
			$marriagename[]  =  '请选择婚姻状况';
			foreach($user_marriage as $v){
				$marriageid[]    =  $v;
				$marriagename[]  =  $cache['userclass_name'][$v];
			}
		}
		$marriage['id']       =  $marriageid;
		$marriage['name']     =  $marriagename;
		$userdata['marriage'] =  $marriage;
		
		$user_jobstatus  =  $cache['userdata']['user_jobstatus'];
		if(!empty($user_jobstatus)){
			foreach($user_jobstatus as $v){
				$jobstatustid[]		=	$v;
				$jobstatusname[]	=	$cache['userclass_name'][$v];
				$arrjobstatus[]		=	array('id'=>$v,'name'=>$cache['userclass_name'][$v]);
			}
		}
		$jobstatus['id']	=	$jobstatustid;
		$jobstatus['name']	=	$jobstatusname;
		$userdata['jobstatus']	=	$jobstatus;
		$userdata['jobstatus_arr']	=	$arrjobstatus;
		
		if(!empty($cache['user_sex'])){
		    $id[]	 =	0;
		    $name[]  =	'请选择性别';
		    foreach($cache['user_sex'] as $k=>$v){
		        $id[]	 =	$k;
		        $name[]  =	$v;
		        $arrsex[]=	array('id'=>$k,'name'=>$v);
		    }
		}
        $sex['id']			=	$id;
        $sex['name']		=	$name;
        $userdata['sex']		=	$sex;
        $userdata['sex_arr']	=	$arrsex;
		$user_tag	=	$cache['userdata']['user_tag'];
		if(!empty($user_tag)){
			$tagid[]	=	'0';
			$tagname[]	=	'请选择个人标签';
			foreach($user_tag as $v){
				$tagid[]	=	$v;
				$tagname[]	=	$cache['userclass_name'][$v];
				$arrtag[]	=	array('id'=>$v,'name'=>$cache['userclass_name'][$v]);
			}
		}
		$tag['id']		=	$tagid;
		$tag['name']	=	$tagname;
		$userdata['tag']	=	$tag;
		$userdata['tag_arr']=	$arrtag;
		
		$userdata['uptime']=  array('1'=>'今天','3'=>'最近三天','7'=>'最近七天','30'=>'最近一个月','90'=>'最近三个月');
		$data['userdata']  =  $userdata;
		
		if($_POST['uid']){
		    $resumeM	=	$this -> MODEL('resume');
		    //addexpect页面
		    if ($_POST['eid'] && $_POST['expect']){
		        $row		=	$resumeM -> getExpect(array('id'=>$_POST['eid']),array('field'=>'`jobstatus`,`report`,`type`'));
		        foreach ($jobstatus['id'] as $k=>$v){
		            if ($row && $row['jobstatus']==$v){
		                $jobstatusIndex	=	$k;
		            }
		        } 
		        foreach ($report['id'] as $k=>$v){
		            if ($row && $row['report']==$v){
		                $reportIndex	=	$k;
		            }
		        } 
		        foreach ($type['id'] as $k=>$v){
		            if ($row && $row['type']==$v){
		                $typeIndex		=	$k;
		            }
		        }
		    }else{
		        //info页面
		        if ($_POST['info']){
		            $row	=	$resumeM -> getResumeInfo(array('uid'=>$_POST['uid']),array('field'=>'`sex`,`edu`,`exp`,`marriage`'));
		            foreach ($sex['id'] as $k=>$v){
		                if ($row && $row['sex']==$v){
		                    $sexIndex	=	$k;
		                }
		            }
		            foreach ($edu['id'] as $k=>$v){
		                if ($row && $row['edu']==$v){
		                    $eduIndex	=	$k;
		                }
		            }
		            foreach ($exp['id'] as $k=>$v){
		                if ($row && $row['exp']==$v){
		                    $expIndex	=	$k;
		                }
		            }
					foreach ($marriage['id'] as $k=>$v){
		                if ($row && $row['marriage']==$v){
		                    $marriageIndex	=	$k;
		                }
		            }
		        }
				if(!in_array($_POST['table'],array('expect','desc','cert','doc','edu','other','project','show','skill','training','work'))){

					unset($_POST['table']);
				}
		        if($_POST['table'] && $_POST['id']){
					
		            $id  	=	$_POST['id'];
		            
		            $table	=	'resume_'.$_POST['table'];
		            
		            $row	=	$resumeM -> getFb($table, $id);
		            if($_POST['table'] == 'edu'){
		                foreach ($edu['id'] as $k=>$v){
		                    if ($row && $row['education']==$v){
		                        $eduIndex	=	$k;
		                    }
		                }
		            }
		            if($_POST['table'] == 'skill'){
		                foreach ($ing['id'] as $k=>$v){
		                    if ($row && $row['ing']==$v){
		                        $ingIndex	=	$k;
		                    }
		                }
		            }
		        }
		    }
		}elseif ($_POST['tinyid'] && $_POST['tinyid']>0){
		    $row	=	$this -> MODEL('tiny') -> getResumeTinyInfo(array('id'=>$_POST['tinyid']),array('field'=>'`sex`,`exp`'));
		    foreach ($sex['id'] as $k=>$v){
		        if ($row && $row['sex']==$v){
		            $sexIndex	=	$k;
		        }
		    }
		    foreach ($exp['id'] as $k=>$v){
		        if ($row && $row['exp']==$v){
		            $expIndex	=	$k;
		        }
		    }
		}
		$data['jobstatusIndex']	=	$jobstatusIndex?$jobstatusIndex:0;
		$data['reportIndex']	=	$reportIndex?$reportIndex:0;
		$data['typeIndex']		=	$typeIndex?$typeIndex:0;
		$data['sexIndex']		=	$sexIndex?$sexIndex:0;
		$data['eduIndex']		=	$eduIndex?$eduIndex:0;
		$data['expIndex']		=	$expIndex?$expIndex:0;
		$data['marriageIndex']	=	$marriageIndex?$marriageIndex:0;
		$data['ingIndex']		=	$ingIndex?$ingIndex:0;
		$this->render_json(0,'ok',$data);
	}
	function com_action(){//获取企业类别
	    
	    $cacheM  =  $this -> MODEL('cache');
	    $cache   =  $cacheM -> GetCache(array('com','user'));
	    $job_pr  =  $cache['comdata']['job_pr'];
	    if(!empty($job_pr)){
		    $prid[]		=	'0';
			$prname[]	=	'请选择企业性质';
			foreach($job_pr as $v){
				$prid[]    =  $v;
				$prname[]  =  $cache['comclass_name'][$v];
				$arrpr[]   =  array('id'=>$v,'name'=>$cache['comclass_name'][$v]);
			}
		}
		$pr['id']			=	$prid;
		$pr['name']			=	$prname;
		$comdata['pr']		=	$pr;
		$comdata['pr_arr']	=	$arrpr;
		
		$job_mun  =  $cache['comdata']['job_mun'];
		if(!empty($job_mun)){
		    $munid[]	=  '0';
			$munname[]	=  '请选择企业规模';
			foreach($job_mun as $v){
				$munid[]	=	$v;
				$munname[]	=	$cache['comclass_name'][$v];
				$arrmun[]	=	array('id'=>$v,'name'=>$cache['comclass_name'][$v]);
			}
		}
		$mun['id']			=	$munid;
		$mun['name']		=	$munname;
		$comdata['mun']		=	$mun;
		$comdata['mun_arr']	=	$arrmun;
		
		$job_number  =  $cache['comdata']['job_number'];
		if(!empty($job_number)){
		    $numberid[]    =  '0';
			$numbername[]  =  '请选择招聘人数';
			foreach($job_number as $v){
				$numberid[]    =  $v;
				$numbername[]  =  $cache['comclass_name'][$v];
			}
		}
		$number['id']       =	$numberid;
		$number['name']     =	$numbername;
		$comdata['number']	=	$number;

		$job_exp  =  $cache['comdata']['job_exp'];
		if(!empty($job_exp)){
		 
			foreach($job_exp as $v){
				$expid[]	=	$v;
				$expname[]	=	$cache['comclass_name'][$v];
				$arrexp[]	=	array('id'=>$v,'name'=>$cache['comclass_name'][$v]);
			}
		}
		$exp['id']			=	$expid;
		$exp['name']		=	$expname;
		$comdata['exp']		=	$exp;
		$comdata['exp_arr']	=	$arrexp;
		
		$job_report  =  $cache['comdata']['job_report'];
		if(!empty($job_report)){
		    $reportid[]    =  '0';
			$reportname[]  =  '请选择到岗时间';
			foreach($job_report as $v){
				$reportid[]    =  $v;
				$reportname[]  =  $cache['comclass_name'][$v];
			}
		}
		$report['id']       =  $reportid;
		$report['name']     =  $reportname;
		$comdata['report']	=  $report;
		
		$job_age  =  $cache['comdata']['job_age'];
		if(!empty($job_age)){
		    $ageid[]	=	'0';
			$agename[]	=	'请选择年龄要求';
			foreach($job_age as $v){
				$ageid[]	=  $v;
				$agename[]	=  $cache['comclass_name'][$v];
			}
		}
		$age['id']       =  $ageid;
		$age['name']     =  $agename;
		$comdata['age']  =  $age;
		
		$job_edu  =  $cache['comdata']['job_edu'];
		if(!empty($job_edu)){
		 
			foreach($job_edu as $v){
				$eduid[]	=	$v;
				$eduname[]  =	$cache['comclass_name'][$v];
				$arredu[]	=	array('id'=>$v,'name'=>$cache['comclass_name'][$v]);
			}
		}
		$edu['id']			=	$eduid;
		$edu['name']		=	$eduname;
		$comdata['edu']		=	$edu;
		$comdata['edu_arr']	=	$arredu;
		
		$job_marriage  =  $cache['comdata']['job_marriage'];
		if(!empty($job_marriage)){
			$marriageid[]    =  '0';
			$marriagename[]  =  '请选择婚姻状况';
			foreach($job_marriage as $v){
				$marriageid[]    =  $v;
				$marriagename[]  =  $cache['comclass_name'][$v];
			}
		}
		$marriage['id']       =  $marriageid;
		$marriage['name']     =  $marriagename;
		$comdata['marriage']  =  $marriage;
		
		$comdata['salary']	=  $this->salaryArr(false,1);
		
		if(!empty($cache['com_sex'])){
			foreach($cache['com_sex'] as $k=>$v){
				$id[]	 =	$k;
				$name[]  =	$v;
				$arrsex[]=	array('id'=>$k,'name'=>$v);
			}
		}
		$sex['id']			=	$id;
		$sex['name']		=	$name;
		$comdata['sex']		=	$sex;
		$comdata['sex_arr']	=	$arrsex;
		
		if(!empty($cache['com_sex'])){
		    foreach($cache['com_sex'] as $k=>$v){
		        if($v != '男'){
		            $req_sex_id[] =	$k;
		            $req_sex_name[]  =	$v;
		        }
		    }
		}
		$sexreq['id']		 =	$req_sex_id;
		$sexreq['name']	     =	$req_sex_name;
		$comdata['sex_req']	 =	$sexreq;
		
		$comdata['uptime']	=	array('1'=>'今天','3'=>'最近三天','7'=>'最近七天','30'=>'最近一个月','90'=>'最近三个月');
		$job_lang  =  $cache['comdata']['job_lang'];
		if(!empty($job_lang)){
			foreach($job_lang as $v){
				$lang[]	=	array('id'=>$v,'name'=>$cache['comclass_name'][$v],'checked'=>false);
			}
		}

        $job_remark  =  $cache['comdata']['job_remark'];
        if(!empty($job_remark)){
            $remarkid[]		=	'0';
            $remarkname[]	=	'请选择简历状态';
            foreach($job_remark as $v){
                $remarkid[]    =  $v;
                $remarkname[]  =  $cache['comclass_name'][$v];
                $arrremark[]   =  array('id'=>$v,'name'=>$cache['comclass_name'][$v]);
            }
        }
        $remark['id']			=	$remarkid;
        $remark['name']			=	$remarkname;
        $comdata['remark']		=	$remark;
        $comdata['remark_arr']	=	$arrremark;

		//投递要求
		$user_word  =  $cache['userdata']['user_word'];
	    if(!empty($user_word)){
		    $expreqid[]			=	'0';
			$expreqname[]		=	'请选择工作经验';
			foreach($user_word as $v){
				$expreqid[]		=	$v;
				$expreqname[]	=	$cache['userclass_name'][$v];
			}
		}
		$expreq['id']		=	$expreqid;
		$expreq['name']		=	$expreqname;
		$comdata['exp_req']	=	$expreq;
		
		$user_edu  =  $cache['userdata']['user_edu'];
		if(!empty($user_edu)){
		    $edureqid[]			=	'0';
			$edureqname[]		=	'请选择学历';
			foreach($user_edu as $v){
				$edureqid[]		=	$v;
				$edureqname[]	=	$cache['userclass_name'][$v];
			}
		}
		$edureq['id']		=	$edureqid;
		$edureq['name']		=	$edureqname;
		$comdata['edu_req']	=	$edureq;
		// 企业福利
		$job_welfare  =  $cache['comdata']['job_welfare'];
		if(!empty($job_welfare)){
		    $arrwelfare[] = array('id'=>0,'name'=>'全部');
		    foreach($job_welfare as $v){
		        $arrwelfare[]	=	array('id'=>$v,'name'=>$cache['comclass_name'][$v]);
		    }
		}
		$comdata['job_welfare']	 =	$arrwelfare;
		
		
		if ($_POST['uid']){

			if($_POST['sid'] && $_POST['sid']>0){


		    	$row	=	$this -> MODEL('spview') -> getInfo(array('id'=>$_POST['sid'],'uid'=>$_POST['uid']));
				foreach ($edureq['id'] as $k=>$v){
	                if ($row && $row['edu']==$v){
	                    $edureqIndex	=	$k;
	                }
	            }
	            foreach ($expreq['id'] as $k=>$v){
	                if ($row && $row['exp']==$v){
	                    $expreqIndex	=	$k;
	                }
	            }
		    	
			}else if ($_POST['jobid'] && $_POST['jobid']>0){

		    	$row	=	$this -> MODEL('job') -> getInfo(array('id'=>$_POST['jobid']),array('field'=>'`number`,`report`,`age`,`lang`,`edu`,`exp`,`marriage`,`sex`,`exp_req`,`edu_req`,`sex_req`'));
		    	
		    	foreach ($sexreq['id'] as $k=>$v){
		    	    if ($row && $row['sex_req']==$v){
		    	        $sexreqIndex	=	$k;
		    	    }
		    	}
                foreach ($edureq['id'] as $k=>$v){
                    if ($row && $row['edu_req']==$v){
                        $edureqIndex	=	$k;
                    }
                }
		        foreach ($edureq['id'] as $k=>$v){
	                if ($row && $row['edu_req']==$v){
	                    $edureqIndex	=	$k;
	                }
	            }
	            foreach ($expreq['id'] as $k=>$v){
	                if ($row && $row['exp_req']==$v){
	                    $expreqIndex	=	$k;
	                }
	            }
		        $langidch  =  explode(',', $row['lang']);
				foreach($lang as $k=>$v){
					if(!empty($langidch)){
						if(in_array($v['id'],$langidch)){
							$lang[$k]['checked']=true;
						}else{
							$lang[$k]['checked']=false;
						}
					}
				}
				foreach ($number['id'] as $k=>$v){
		            if ($row && $row['number']==$v){
		                $numberIndex	=	$k;
		            }
		        }
		        foreach ($report['id'] as $k=>$v){
		            if ($row && $row['report']==$v){
		                $reportIndex	=	$k;
		            }
		        }
		        foreach ($age['id'] as $k=>$v){
		            if ($row && $row['age']==$v){
		                $ageIndex		=	$k;
		            }
		        }
		        foreach ($edu['id'] as $k=>$v){
		            if ($row && $row['edu']==$v){
		                $eduIndex		=	$k;
		            }
		        }
		        foreach ($exp['id'] as $k=>$v){
		            if ($row && $row['exp']==$v){
		                $expIndex		=	$k;
		            }
		        }
		        foreach ($sex['id'] as $k=>$v){
		            if ($row && $row['sex']==$v){
		                $sexIndex		=	$k;
		            }
		        }
		        foreach ($marriage['id'] as $k=>$v){
		            if ($row && $row['marriage']==$v){
		                $marriageIndex	=	$k;
		            }
		        }
		    }elseif (!$_POST['jobid'] || $_POST['jobid']==0){
		        $row	=	$this -> MODEL('company') -> getInfo($_POST['uid'],array('field'=>'`pr`,`mun`'));
		        foreach ($pr['id'] as $k=>$v){
		            if ($row && $row['pr']==$v){
		                $prIndex		=	$k;
		            }
		        }
		        foreach ($mun['id'] as $k=>$v){
		            if ($row && $row['mun']==$v){
		                $munIndex		=	$k;
		            }
		        }
                foreach ($remark['id'] as $k=>$v){
                    if ($row && $row['remark']==$v){
                        $remarkIndex	=	$k;
                    }
                }
		    }
		}
		$comdata['otherarr']	=	count($otherarr) ? $otherarr : array();
		$comdata['lang']		=	count($lang) ? $lang : array();
		$data['comdata']		=	$comdata;

		$data['remarkIndex']    =   $remarkIndex?$remarkIndex:0;
		$data['edureqIndex']	=	$edureqIndex?$edureqIndex:0;
		$data['expreqIndex']	=	$expreqIndex?$expreqIndex:0;
		$data['sexreqIndex']	=	$sexreqIndex?$sexreqIndex:0;
		$data['prIndex']		=	$prIndex?$prIndex:0;
		$data['munIndex']		=	$munIndex?$munIndex:0;
		$data['numberIndex']	=	$numberIndex?$numberIndex:0;
		$data['reportIndex']	=	$reportIndex?$reportIndex:0;
		$data['ageIndex']		=	$ageIndex?$ageIndex:0;
		$data['eduIndex']		=	$eduIndex?$eduIndex:0;
		$data['expIndex']		=	$expIndex?$expIndex:0;
		$data['marriageIndex']	=	$marriageIndex?$marriageIndex:0;
		$data['sexIndex']		=	$sexIndex?$sexIndex:0;
		
		$this->render_json(0,'ok',$data);
	}


	function keyword_action(){  //关键字展示
        include PLUS_PATH."keyword.cache.php";
        if(is_array($keyword)){
            foreach($keyword as $k=>$v){
                if($v['type']==$_POST['type']&&$v['tuijian']=='1' && count($key_name) < 12){// 热搜关键词最多展示12个
                    $key_name[]	=	$v['key_name'];
                }
            }
        }
        $data['keyword']    = $key_name;
        $this->render_json(0,'ok',$data['keyword']);
	}
	function group_action(){  //关键字类别
	    include(PLUS_PATH.'group.cache.php');
		$data['group_index']	=	$group_index;
	    $data['group_name']		=	$group_name;
	    $this->render_json(0,'ok',$data);
	}
	/**
	 * 查询分站数据
	 */
	function site_action(){
	    
	    include(PLUS_PATH."domain_cache.php");
	    $hy         =  array();
	    $domainarr  =  array();
	    $city_id    =  array();
	    foreach($site_domain as $k=>$v){
	        if($v['fz_type']=='1'){
	            if($v['three_cityid']>0){
	                $cityid=$v['three_cityid'];
	            }elseif($v['cityid']>0){
	                $cityid=$v['cityid'];
	            }else{
	                $cityid=$v['province'];
	            }
	            $domainarr[$cityid]=$v['id'];
	            $city_id[]=$cityid;
	        }elseif ($v['fz_type'] == '2'){
	            $hy[] = array(
	                'id'    =>  $v['id'],
	                'name'  =>  $v['webname']
	            );
	            $hydomain[]  =  $v['webname'];
	        }
	    }
	    $city  =  $site  =  array();
	    
	    if (!empty($city_id)){
	        
	        $sitecity  =  $this->obj->select_all("city_class",array('id'=>array('in',pylode(',', $city_id))),"`id`,`name`,`letter`");
	        
	        $city=array();
	        foreach($sitecity as $k=>$v){
	            $city[$k]['id']     = $domainarr[$v['id']];
	            $city[$k]['name']   = $v['name'];
	            $city[$k]['letter'] = $v['letter'];
	        }
	        $letter  =  array("A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z");
	        $larr    =  array();
	        foreach($letter as $key=>$val){
	            foreach($city as $v){
	                if($val==$v['letter']){
	                    $larr[$val][]=$v;
	                }
	            }
	        }
	        if (!empty($larr)){
	            $site  =  array();
	            foreach ($larr as $k=>$v){
	                $ldata  =  array();
	                foreach ($v as $vv){
	                    $ldata[]  =  $vv['name'];
	                }
	                $site[]  = array(
	                    'letter'  =>  $k,
	                    'data'    =>  $ldata
	                );
	            }
	        }
	    }
	    if (!empty($hydomain)){
	        $site[]  = array(
	            'letter'  =>  '行业分站',
	            'data'    =>  $hydomain
	        );
	    }
	    $data['city']  =  $city;
	    $data['site']  =  $site;
	    $data['hy']  =  $hy;
	    
	    $this->render_json(0, 'ok', $data);
	}
	/**
	 * 获取举报简历原因
	 */
	function getReportReason_action(){
	    
	    $cacheM  =  $this->MODEL('cache');
	    $cache   =  $cacheM -> GetCache(array('user'));
	    
	    $reason  =  array();
	    $user_reporting  =  $cache['userdata']['user_reporting'];
	    if(!empty($user_reporting)){
	        foreach($user_reporting as $v){
	            $reason[]  =  $cache['userclass_name'][$v];
	        }
	    }
	    $data['reason'] = $reason;
	    $this->render_json(0, 'ok', $data);
	}
    /**
     * 获取举报职位原因
     */
    function getjobReportReason_action(){

        $cacheM  =  $this->MODEL('cache');
        $cache   =  $cacheM -> GetCache(array('com'));

        $reason  =  array();
        $job_jobreport  =  $cache['comdata']['job_jobreport'];
        if(!empty($job_jobreport)){
            foreach($job_jobreport as $v){
                $reason[]  =  $cache['comclass_name'][$v];
            }
        }
        $data['reason'] = $reason;
        $this->render_json(0, 'ok', $data);
    }

	/**
     * 获取邀请注册模板列表
     */
    function getInviteRegHbList_action()
    {
        $whbM   =   $this->MODEL('whb');
        $list   =   $whbM->getWhbList(array('type' => 3, 'isopen' => 1, 'orderby' => 'sort,desc'), array('field' => 'id'));

        $this->render_json(0, 'ok', ['list' => $list]);
    }
}
?>