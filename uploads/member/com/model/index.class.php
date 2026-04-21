<?php
/* *
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/
class index_controller extends company{
	function index_action(){
		
		$this -> public_action();
	
		$statis			=	$this->company_satic();
		$jobM			=	$this->MODEL('job');
		$userinfoM		=	$this->MODEL('userinfo');
		$downresumeM	=	$this->MODEL('downresume');
		$companyorderM	=	$this->MODEL('companyorder');
		$MsgM			=	$this -> MODEL('msg');
		

		//收到的简历
		$des_resume		=	$jobM->getSqJobNum(array('com_id'=>$this->uid,'isdel'=>9,'type'=>array('<>',3)));
		$this->yunset('des_resume',$des_resume);
		
		//求职者咨询
		$msgnum			=	$MsgM -> getMsgNum(array('job_uid'=>$this->uid,'status'=>1,'del_status'=>0));
		$this->yunset('msgnum',$msgnum);

        //谁看过我
        $look_jobnum	=	$jobM -> getLookJobNum(array('com_id'=>$this->uid,'com_status'=>0), array('usertype' => $this->usertype));
        $this->yunset('look_jobnum',$look_jobnum);
		
		//浏览量
        $comM           =   $this->MODEL('company');
        $hitsExporue    =   $comM->getHitsExpoure($this->uid);
		$this->yunset(array('hitsNum' => $hitsExporue['hits'], 'expoureNum' => $hitsExporue['expoure']));

		//收到未查看的简历
		$de_resume		=	$jobM->getSqJobNum(array('com_id'=>$this->uid,'isdel'=>9,'is_browse'=>'1','type'=>array('<>',3)));
        $this->yunset('de_resume',$de_resume);
		
		//下载简历
		// $down_resume	=	$downresumeM->getDownNum(array('comid'=>$this->uid,'usertype'=>$this->usertype,'isdel'=>9));
		// $this->yunset('down_resume',$down_resume);
	    // 对我感兴趣
	    $atnM  =  $this->MODEL('atn');
	    $atn   =  $atnM->getantnNum(array('sc_uid'=>$this->uid,'sc_usertype'=>2));
	    $this->yunset('atn',$atn);
		//刷新职位数量
		if((int)$statis['vip_etime'] == 0){

			$breakWhere	=	array(
				'uid'	=>  $this -> uid,
				'opera'	=>  1,
				'type'	=>  4,
				'ctime'	=>	array('>=', $statis['vip_stime'])
			);
		}else{
			$breakWhere 	=	array(
				'uid'                 =>  $this -> uid,
				'opera'               =>  1,
				'type'                =>  4,
				'PHPYUNBTWSTART_A'    =>  '',
				'ctime'               =>  array(
					'0'   =>  array('>=', $statis['vip_stime'], 'AND'),
					'1'   =>  array('<=', $statis['vip_etime'], 'AND')
				),
				'PHPYUNBTWEND_A'      =>  ''
			);
		}
		$breakjobNums	=	$this -> MODEL('log') -> getMemberLogNum($breakWhere);
		$this->yunset('breakjobNums', $breakjobNums);

		//正常职位数,判断是否弹出刷新职位
        $jobNumWhere = array('uid' => $this->uid, 'state' => '1' , 'r_status' => 1, 'status' => 0);
		$normal_job_num	=	$jobM -> getJobNum($jobNumWhere);
		$this->yunset('normal_job_num',$normal_job_num);

        // 急聘职位数量
        $jp_job_num	=	$jobM -> getJobNum(array_merge($jobNumWhere, array('urgent' => 1, 'urgent_time' => array('>', time()))));
        $this->yunset('jp_job_num',$jp_job_num);
        // 置顶职位数量
        $top_job_num	=	$jobM -> getJobNum(array_merge($jobNumWhere, array('xsdate' => array('>', time()))));
        $this->yunset('top_job_num',$top_job_num);
        // 推荐职位数量
        $rec_job_num	=	$jobM -> getJobNum(array_merge($jobNumWhere, array('rec' => 1, 'rec_time' => array('>', time()))));
        $this->yunset('rec_job_num',$rec_job_num);


        if ($statis['rating_type'] == 1) {

            $jobNum     =   $this->obj->select_num('company_job', array('uid' => $this->uid, 'status' => 0));
            $partNum    =   $this->obj->select_num('partjob', array('uid' => $this->uid, 'status' => 0));
            $zzNum      =   $jobNum + $partNum;
            $JobNum     =   $statis['job_num'] - $zzNum;
            $JobNum     =   $JobNum > 0 ? $JobNum : 0;
            $this->yunset('JobNum', $JobNum);
        }elseif ($statis['rating_type'] == 2){

            $jobNum     =   $this->obj->select_num('company_job', array('uid' => $this->uid, 'sdate' => array('>', strtotime('today'))));
            $partNum    =   $this->obj->select_num('partjob', array('uid' => $this->uid, 'addtime' => array('>', strtotime('today'))));
            $zzNum      =   $jobNum + $partNum;
            $JobNum     =   $statis['job_num'] - $zzNum;
            $JobNum     =   $JobNum > 0 ? $JobNum : 0;
            $this->yunset('JobNum', $JobNum);
        }
		
		//今日未刷新职位,判断是否弹出刷新职位
		$un_refreshjob_num		=	$jobM -> getJobNum(array('uid' => $this->uid,'lastupdate' => array('<' , strtotime('today')),'state'=>'1','r_status'=>1,'status'=>0));
		$this->yunset('un_refreshjob_num',$un_refreshjob_num);
	
		//获取职位id、name
        $jobwhere['uid']		=   $this->uid;
		$jobwhere['state']		=	1;
		$jobwhere['r_status']	=	1;
		$jobwhere['status']		=	0;
		$jobs	=	$this->obj->select_all('company_job',$jobwhere,'`id`,`name`');
		$this->yunset('jobs', $jobs);

		if($jobs && is_array($jobs)){//获取职位ID
		 
			foreach($jobs as $key=>$v){
				
				$ids[]			=	$v['id'];
				if ($key<3){
				    $jobnames[]	=	$v['name'];
				}
			}
			$jobids 			=	"".pylode(",",$ids)."";
			$jobnames 			=	"".@implode(",",$jobnames)."";
			if (count($jobs)>3){
			    $jobnames		.=	"等，<span style='color:blue'>共".count($jobs)."个职位</span>。";
			}
			$this->yunset('jobids',$jobids);
			$this->yunset('jobnames',$jobnames);
		}

        $member	=	$userinfoM->getInfo(array('uid'=> $this->uid),array('field'=>'`login_date`,`reg_date`,`status`,`subscribe`,`wxid`,`unionid`,`lock_info`'));
        if ($member['subscribe'] != 1 && !empty($member['wxid'])){
            $wxM    =   $this->MODEL('weixin');
            $wxUser =   $wxM->getWxUser($member['wxid']);
            $member['subscribe']    =   $wxUser['subscribe'];
            $this->obj->update_once('member', array('subscribe' => $wxUser['subscribe']), array('uid' => $this->uid));
        }
        $this->yunset('member',$member);
		
		if($statis['rating']>0){
			//获取会员图标
			$company_rating	=	$this->MODEL('rating')->getInfo(array('id'=>$statis['rating']));
      		$this->yunset('company_rating',$company_rating);
		}
    
		//浏览记录
		$look_resume	=	$this->MODEL('lookresume')->getLookNum(array('com_id'=>$this->uid,'com_status'=>'0'));
		$this->yunset('look_resume',$look_resume);
		//未付款订单
		$paying			=   $companyorderM -> getCompanyOrderNum(array('uid' => $this->uid,'usertype' => $this->usertype,'order_state' => '1'));
		$this->yunset('paying',$paying);
		
		//企业资质认证查询
		$yyzz			=   $this->MODEL('company')->getCertInfo(array('uid' => $this -> uid, 'type' => 3), array('field' => '`status`'));
		$this->yunset('yyzz', $yyzz);
		
		
		$this->cookie->SetCookie('jobrefresh','1',(strtotime('today') + 86400));
		//判断微信绑定情况
		if($member['wxid']==''&&$member['unionid']=='' && $this->config['wx_author']=='1'){
		    $this->yunset('qrcode', 1);
		}
		$this->cookie->SetCookie('gzh','1',(strtotime('today') + 86400));
				
		$company	=	$this->comInfo['info'];

		if($company['hy']== ''){
			
			if($_COOKIE['indextip']=='1'){
				$indextip = 0;
			}else{
				$this->cookie->SetCookie('indextip','1',(strtotime('today') + 86400));
				$indextip = 1;
			}
			
			$this->yunset('indextip',$indextip);
		}else{

			$this->cookie->SetCookie('indextip','',(strtotime('today') - 86400));

		}

		$this->yunset('company', $this->comInfo['info']);


        $WhbM       =   $this->MODEL('whb');
        $syComHb    =   $WhbM->getWhbList(array('type' => 2, 'isopen' => 1), array('only' => 1));
        $this->yunset('hbNum', count($syComHb));
        $this->yunset('comHb', $syComHb);

        if(!empty($syComHb)){
            $hbids  =   array();
            foreach ($syComHb as $hk => $hv) {
                $hbids[] = $hv['id'];
            }
            $this->yunset('hbids', $hbids);
        }
        $this->yunset('hb_uid', $this->uid);
        // HR年度报告弹窗显示条件
        $thisyear = strtotime(date('Y').'-01-01');

        

        if($member['reg_date']<$thisyear && $this->config['sy_yearreport_isopen']==1){
        	if($_COOKIE['yearreport']=='1'){
				$yearreport = 0;
			}else{
				$this->cookie->SetCookie('yearreport','1',(strtotime('today') + 86400*30));
				$yearreport = 1;
			}
        	if($yearreport){
        		$this->yunset('yrshow',1);
        	}
        	$this->yunset('yrbtn',1);
        }
        $this->yunset('sy_yearreport_tip',checkpic($this->config['sy_yearreport_tip']));
        
        
 		$this->com_tpl('index');
	}

	
	function resumeajax_action(){
		$jobM		=	$this->MODEL('job');
		$resumeM	=	$this->MODEL('resume');
		
		
        $jobwhere['com_id']		=   $this->uid;
		$jobwhere['state']		=	1;
		$jobwhere['r_status']	=	1;
		$jobwhere['status']		=	0;
		
	    $joblist  =	 $jobM->getList($jobwhere,array('field'=>'`job1_son`,`job_post`,``three_cityid``,`cityid`,`provinceid`'));
	    $joblist  =  $joblist['list'];
	    $cityclass = $jobclass = array();
	    if(is_array($joblist) && !empty($joblist)){
	        foreach($joblist as $v){
	            if(!empty($v['three_cityid'])){
	        		$cityclass[]  = $v['three_cityid'];
	        	}else if (!empty($v['cityid'])){
	                $cityclass[]  = $v['cityid'];
	            }else if (!empty($v['provinceid'])){
	                $cityclass[]  = $v['provinceid'];
	            }

	            if(!empty($v['job_post'])){
	                $jobclass[] = $v['job_post'];
	            }else if(!empty($v['job1_son'])){
	                $jobclass[] = $v['job1_son'];
	            }
	        }
	    }
	    // 按企业发布职位的城市类别来查询
	    if (!empty($cityclass)){
	        $cityclass = array_unique($cityclass);
	        $whereSql['PHPYUNBTWSTART_city']  =  '';
	        foreach ($cityclass as $k=>$v){
	        	if($k>0){
	        		$whereSql['city_classid'][]  =  array('findin',$v,'or');
	        	}else{
	        		$whereSql['city_classid'][]  =  array('findin',$v);
	        	}
	        }
	        $whereSql['PHPYUNBTWEND_city']    =  '';
	    }
	    // 按企业发布职位的工作类别来查询
	    if (!empty($jobclass)){
	        $jobclass = array_unique($jobclass);
	        $whereSql['PHPYUNBTWSTART_job']  =  '';
	        foreach ($jobclass as $k=>$v){
	        	if($k>0){
	            	$whereSql['job_classid'][]  =  array('findin',$v,'or');
	            }else{
	            	$whereSql['job_classid'][]  =  array('findin',$v);
	            }
	        }
	        $whereSql['PHPYUNBTWEND_job']    =  '';
	    }
	    $blackM		=	$this->MODEL('black');
	    $blacklist	=	$blackM->getBlackList(array('p_uid'=>$this->uid),array('field'=>'`c_uid`'));
	    if(is_array($blacklist) && !empty($blacklist)){
	        foreach($blacklist as $v){
	            $bids[]=$v['c_uid'];
	        }
	        
	        $nwhereSql['uid']		=	$whereSql['uid'] 			=	array('notin',pylode(',',$bids)) ;
	    }
		$nwhereSql['uname']			=	$whereSql['uname']			=	array('<>','');
		$nwhereSql['status']		=	$whereSql['status']			=	1;
		$nwhereSql['r_status']		=	$whereSql['r_status']		=	1;
		$nwhereSql['state']			=	$whereSql['state']			=	1;
		$nwhereSql['defaults']		=	$whereSql['defaults']		=	1;
		$nwhereSql['orderby']		=	$whereSql['orderby']		=	'lastupdate,desc';
		$nwhereSql['limit']			=	$whereSql['limit']			=	10;
		// 容错查询，防止按企业发布职位的工作类别来查询，查不到数据
		$nwhereSql['job_classid']	=	array('<>','');
		
	    $resumes 		= 	$resumeM->getList($whereSql);
	    
		$resume			=	$resumes['list'];
		
	    $list			=	array();
	    if ($resume){
	        foreach ($resume as $v){
	            $uids[]	=	$v['uid'];
	            $resumeIdArr[] = $v['id'];
	        }
	        if ($uids){
	            $user 	= 	$resumeM->getResumeList(array('uid'=>array('in',pylode(',',$uids))),array('field'=>'`uid`,`name`,`nametype`,`sex`,`photo`,`defphoto`, `phototype`,`photo_status`,`def_job`,`birthday`'));
	        }

            // 查询今日被查看次数
            //$lookList = $this->obj->select_all('look_resume', array(
            //    'resume_id' => array('in', pylode(',', $resumeIdArr)),
            //    'datetime' => array('>=', strtotime('today')),
            //    'com_status' => '0',
            //    'groupby' => 'resume_id'
            //), 'resume_id,count(resume_id) num');
	        //$lookNumArr = $lookList ? array_column($lookList, 'num', 'resume_id') : array();

	        foreach ($resume as $k=>$v){
	            $list[$k]['username_n']='';
	            foreach ($user as $val){
	                if ($v['uid']==$val['uid']){
	                    $list[$k]['username_n'] 	=	$val['name_n'];
						$list[$k]['photo']			=	$val['photo'];
	                }
	            }
	            // 2023年9月13日08:44:04 注释
	            // $list[$k]['resumeurl']				=	Url('resume',array('c'=>'show','id'=>$v['id']));
                $list[$k]['id']                     =   $v['id'];
                $list[$k]['uid']                    =   $v['uid'];
                $list[$k]['age_n']                  =   $v['age_n'] ? $v['age_n'] . '岁' : '';
                $list[$k]['sex_n']                  =   $v['sex_n'];
	            $list[$k]['edu_n']					=	$v['edu_n']?$v['edu_n'].'学历':'';
	            $list[$k]['exp_n']					=	$v['exp_n']?$v['exp_n'].'经验':'';
                $list[$k]['status']                 =   $v['status'];
	            
	            $jobname							=	@explode(',', $v['job_classname']);
	            $list[$k]['jobname']				=	$jobname['0'];
				
	            $cityname							=	@explode(',', $v['city_classname']);
	            $list[$k]['cityname']				=	$cityname['0'];

                // 拼接今日被查看次数
                //$list[$k]['looknum']				=	isset($lookNumArr[$v['id']]) ? $lookNumArr[$v['id']] : 0;
	        }
	    }
	    $data['list']=$list;
	    echo json_encode($data);die;
	}
	function logout_action(){

		$this->logout();

	}
}
?>