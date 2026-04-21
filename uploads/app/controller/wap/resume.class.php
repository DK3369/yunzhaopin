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
class resume_controller extends common{

	public $userInfo	=	array();

	/**
	 * @desc 简历列表
	 * 2019-06-24
	 */
	function index_action()
	{

		$this -> get_moblie();
		$CacheM		=	$this -> MODEL('cache');
        $CacheArr	=	$CacheM -> GetCache(array('user','job','city','hy'));
        // 后台-页面设置-列表页区域默认设置。选择了一级城市
        if (!empty($this->config['sy_web_city_one'])) {
            $provinceid  =  $this->config['sy_web_city_one'];
            $CacheArr['city_index']  =  array($provinceid);
            $this->yunset('nocityall', 1);
            // 选择了二级城市
            if (!empty($this->config['sy_web_city_two'])) {
                $cityid  =  $this->config['sy_web_city_two'];
            }
            if (!isset($_GET['provinceid']) && !isset($_GET['cityid'])){
                if (isset($provinceid)){
                    $_GET['provinceid'] = $provinceid;
                }
                if (isset($cityid)){
                    $_GET['cityid'] = $cityid;
                }
            }
            if (isset($_GET['three_cityid'])) {
                unset($_GET['provinceid']);
                unset($_GET['cityid']);
            }
        }
        $this->yunset($CacheArr);
        
        $uptime		=	array(1 => '今天', 3 => '最近3天', 7 => '最近7天', 30 => '最近一个月', 90 => '最近三个月');
        $this -> yunset('uptime', $uptime);
		
        if (isset($_GET['ecity']) || isset($_GET['ejob'])){
            
            $pinyin  =  $CacheM->pinYin($_GET,array('city_index'=>$CacheArr['city_index'],'job_index'=>$CacheArr['job_index']));
            
            if (!empty($pinyin)){
                
                $_GET  =  array_merge($_GET,$pinyin);
            }
        }
		$searchurl  =  array();
		$searchUrlObj = array();
		foreach($_GET as $k=>$v){
			if($k!=""){
				$searchurl[]  =  $k.'='.$v;
				$searchUrlObj[$k]    = $v;
			}
		}
		
		if ($this->usertype == 2) {
    		//已下载
    		$downM    =	  $this -> MODEL('downresume');
    		$down     =	  $downM -> getSimpleList(array('comid' => $this->uid,'usertype'=>$this->usertype), array('field' => '`eid`'));
    		$eid      =   array();   
    		foreach ($down as $v){
    		    $eid[]	=	$v['eid'];
    		}
    		$this -> yunset('eid',$eid);

			$userInfoM		=	$this->MODEL('userinfo');

			$this->userInfo	=	$userInfoM -> getUserInfo(array('uid' => $this->uid), array('usertype' => $this->usertype));
			
 
			$this->yunset('userInfo', $this->userInfo);
			
		}
		
		$searchurl	=	@implode('&', $searchurl);
		$this -> yunset('searchurl', $searchurl);
		$this->yunset('searchUrlObj',json_encode($searchUrlObj));

		$cityChoosed = '';
        if ($_GET['three_cityid']){
            $_GET['threecityid']   =   $_GET['three_cityid'];
        }
        if($_GET['threecityid']){
            $cityChoosed = $_GET['threecityid'];
        }else if($_GET['cityid']){
            $cityChoosed = $_GET['cityid'];
        }else if($_GET['provinceid']){
            $cityChoosed = $_GET['provinceid'];
        }
        $this->yunset('cityChoosed', $cityChoosed);

        $jobChoosed = '';
        if ($_GET['job_post']){
            $_GET['jobpost']    =   $_GET['job_post'];
            unset($_GET['job_post']);
        }
        if ($_GET['job1_son']){
            $_GET['job1son']    =   $_GET['job1_son'];
            unset($_GET['job1_son']);
        }
        if($_GET['jobpost']){
            $jobChoosed = $_GET['jobpost'];
        }else if($_GET['job1_son']){
            $jobChoosed = $_GET['job1_son'];
        }else if($_GET['job1']){
            $jobChoosed = $_GET['job1'];
        }
        $this->yunset('jobChoosed', $jobChoosed);
        
		$this -> yunset('backurl', Url('wap'));
		
		include(CONFIG_PATH.'db.data.php');
		$this -> yunset('integrity_name',$arr_data['integrity_name']);

		if (count($searchurl) > 1){
		    $this->seo('user_search');
		}else{
		    $this->seo('user');
		}			
		 
		$companyM    =	  $this -> MODEL('company');
		$comnum     =	  $companyM -> getCompanyNum(array('uid' => $this->uid));
		$this->yunset("comnum", $comnum);
		
		$resumeM		=	$this -> MODEL('resume');
		$recnum = $resumeM->getExpectNum(array('rec_resume'=>1,'defaults'=>1,'state'=>1,'r_status'=>1,'status'=>1));
		$this->yunset("recnum", $recnum);

		$this -> yunset('headertitle', '找人才');
		$this -> yunset('topplaceholder', '请输入简历关键字,如：服务员...');
		$this -> yuntpl(array('wap/resume'));
	}

	function search_action(){
		$this -> index_action();
	}

	/**
	 * 简历详情
	 * 2019-06-21
	 */
	function show_action(){
		$this -> get_moblie();
		
		$resumeM					=	$this -> MODEL('resume');
		$JobM						=	$this -> MODEL('job');
		$companyM                   =   $this->MODEL('company');
		$getUid						=	intval($_GET['uid']);
		$getType					=	intval($_GET['type']);
		$getId						=	intval($_GET['id']);
		$returnUrl					=	Url('wap',array('c'=>'login'));
        if ($this->uid && $this->usertype == 2) {
            $companyInfo    =   $companyM->getCompanyInfo(array('uid' => $this->uid), array('field' => 'r_status'));
            if ($companyInfo['r_status'] != '1' && $companyInfo['r_status'] != '4') {
                $this->ACT_msg_wap(1, '账户正在审核中，无法查看！', 2, 5);
            }
        }
		//记录游客浏览量
		if($this->uid == '' && $this->config['sy_resume_visitors'] > 0){ 
		    
			if($_COOKIE['resumevisitors'] >= $this->config['sy_resume_visitors']){ 				
			     
			    $this->ACT_msg_wap($returnUrl,'游客用户，每天只能访问'.$this->config['sy_resume_visitors'].'份简历，请登录后继续访问！', 2, 5);
			}else{
			    
				if ($_COOKIE['resumevisitors']==''){
				
				    $resumevisitors	=	1;
				}else{
					
				    $resumevisitors	=	$_COOKIE['resumevisitors']+1;
				}
				$this -> cookie -> SetCookie('resumevisitors', $resumevisitors, strtotime(date("Y-m-d",strtotime("+1 day"))));
			}
		}
		
		if(!empty($getUid)){
			 			    
		    $def_job     =	$resumeM -> getResumeInfo(array('uid' => $getUid, 'r_status' => 1), array('field' => '`def_job`'));
			
			if(empty($def_job)){
			
			    $this->ACT_msg_wap(1,'没有找到该人才！', 2, 5);
			    
			}else if($def_job['def_job'] < 1){
			    
			    $this->ACT_msg_wap(1,'还没有创建简历！', 2, 5);
			}else if($def_job['def_job']){
			    
				$id      =	$def_job['def_job'];
			}
			 
 		}else{
			$id			     =	$getId;
		}

		$this->yunset('eid', $id);
		
		$resume_expect	     =	$resumeM -> getInfoByEid(array('eid' => $id, 'uid' => $this->uid, 'usertype' => $this->usertype));
		
		if(empty($resume_expect)){
		    $this->ACT_msg_wap(1,'没有找到该人才！', 2, 5);
		}

		//简历状态判断
		if($resume_expect['state'] == 0 && $resume_expect['uid'] != $this->uid){		    

		    $this->ACT_msg_wap(1,'简历正在审核中！', 2, 5);
			
		}elseif($resume_expect['r_status'] == 2 && $resume_expect['uid'] != $this->uid){
		    
		    $this->ACT_msg_wap(1,'简历暂被锁定，请稍后查看！', 2, 5);

		}elseif($resume_expect['state'] == 3 && $resume_expect['uid'] != $this->uid){

		    $this->ACT_msg_wap(1,'简历审核暂未通过！', 2, 5);
		}
		if ($this->config['com_search'] == 1 && !$this->uid){
            $this->ACT_msg_wap($returnUrl, '请先登录', 2, 5);
        }
		//个人用户无法查看简历
		if($this->config['sy_user_visit_resume'] == '0' && $this->usertype == 1 && $this->uid != $resume_expect['uid']){
			
		    $this->ACT_msg_wap($returnUrl,'个人用户无权限查看！', 2, 5);
		}

		if ($this->uid != $resume_expect['uid']) {
            // 检查简历隐私状态设置
            $canShow = true;
            if ($resume_expect['status'] == 2){
                // 简历关闭
                $canShow = false;
            }elseif ($resume_expect['status'] == 3){
                // 简历状态是投递企业可见
                $canShow = false;
                if (isset($resume_expect['userid_job'])){
                    // 已向企业投递简历，简历可以展示
                    $canShow = true;
                }
            }
            if (!$canShow){
                $this->ACT_msg_wap(1,'简历已设置不对外开放', 2, 5);
            }
        }

		// 查询黑名单
        $blackM             =   $this->MODEL('black'); 
        $blackInfo          =   $blackM -> getBlackInfo(array('p_uid' => $this->uid, 'c_uid'=> $resume_expect['uid']));
        
        if(!empty($blackInfo)){
            
            $this->ACT_msg_wap(1, '该用户已关闭简历!', 2, 5);
            
        }
		//人才收藏库
        $talent_pool				  =	  $resumeM -> getTalentNum(array('eid' => $id, 'cuid' => $this -> uid));

		//已邀请面试数量
		$userid_msg					  =	  $JobM -> getYqmsNum(array('fid' => $this->uid,'uid' => $resume_expect['uid'],'isdel'=>9));

		$resume_expect['talent_pool'] =	  $talent_pool;
		$resume_expect['userid_msg']  =	  $userid_msg;
		$resume_expect['euid']		  =	  $resume_expect['uid'];
		$resume_expect['rec_resume']  =	  $resume_expect['rec_resume'];
        
		$cData['uid']		=	$this->uid;
		$cData['usertype']	=	$this->usertype;
		$cData['eid']		=	intval($_GET['id']);
		$cData['ruid']		=	$resume_expect['uid'];
		$resumeCkeck		=	$resumeM->openResumeCheck($cData);
		$this ->yunset('resumeCkeck',$resumeCkeck);
		
		/* 模糊字段 */
		$this ->yunset('tj',$resume_expect['tj']);

		$data['resume_username']	=	$resume_expect['username_n'];//简历人姓名
		$data['resume_city']		=	$resume_expect['cityname'];//城市
		$data['resume_job']			=	$resume_expect['customjob'];//行业
		$this -> data				=	$data;
		$this -> seo('resume');
		
		if($_GET['down']){
		    if($this->usertype == '2'){
		      $this->yunset('backurl', Url('wap',array('c' => 'down'),'member'));
		    }
		}
        if ($this->usertype == 2 && isset($resume_expect['free_look'])) {

            $this->yunset('freenum', 1);
        }
		$this -> yunset('uid', $this->uid);
 		$this -> yunset('Info', $resume_expect);
  		$this -> yunset('headertitle', '个人简历');
		if($this->config['sy_h5_share']==1){
			$this -> yunset('shareurl', Url('wap',array('c'=>'resume','a'=>'share', 'id' => $id)));
		}else{
			$this->yunset("shareurl",Url('wap',array('c'=>'resume','a'=>'show','id'=>$id)));
		}
		$this -> yuntpl(array('wap/resume_show'));
	}


	/**
	 * 简历详情
	 * 分享简历
	 * 2019-06-21
	 */
	function share_action(){
		$this -> get_moblie();
		
		$id							=	intval($_GET['id']);
		$ResumeM					=	$this -> MODEL('resume');
		$user						=	$ResumeM -> getInfoByEid(array('eid' => $id, 'uid' => $this -> uid, 'usertype' => $this -> usertype));
		$this -> yunset('Info', $user);
		$cData['uid']		=	$this->uid;
		$cData['usertype']	=	$this->usertype;
		$cData['eid']		=	intval($_GET['id']);
		$cData['ruid']		=	$user['uid'];
		$resumeCkeck		=	$ResumeM->openResumeCheck($cData);
		$this ->yunset('resumeCkeck',$resumeCkeck);
		/* 模糊字段 */
		$this ->yunset('tj',$user['tj']);
		$data['resume_username']	=	$user['username_n'];
		$data['resume_city']		=	$user['city_one'].','.$user['city_two'];
		$this -> data				=	$data;
		$this -> seo('resume_share');
		$this -> yunset('resume_style', $this->config['sy_weburl'].'/app/template/wap/resume');
		$this -> yuntpl(array('wap/resume/index'));
	}
	
	/**
	 * 简历详情
	 * 面试邀请
	 * 2019-06-21
	 */
	function invite_action(){
		$this -> get_moblie();
		$uid					=	intval($_GET['uid']);
		$fuid					=	$this->uid;
		
		if(!empty($uid)){
			$resumeM			=	$this -> MODEL('resume');
			$user_resume			=	$resumeM -> getResumeInfo(array('uid' => $uid, 'r_status' => 1), array('field' => '`def_job`'));
			$userrows	     	=	$resumeM -> getInfoByEid(array('eid' => $user_resume['def_job'], 'uid' => $this->uid, 'usertype' => $this->usertype));
			
			$data               =   array();
			$data['name']		=	$userrows['m_status']==1 ? $userrows['name'] : $userrows['username_n'];
			$data['uid']		=	$userrows['uid'];
			$data['photo']		=	$userrows['photo'];
			
			$expect				=	$resumeM -> getExpect(array('uid' => $uid, 'defaults' => 1), array('field' => '`id`, `job_classid`', 'needCache' => 1));
			$data['id']			=	$expect['id'];
			$data['jobname']	=	$expect['jobnameArr'];
			$this -> yunset('usermsg', $data);

			//企业联系信息(默认联系方式)
			$comM				=	$this -> MODEL('company');
			$company 			=	$comM -> getInfo($fuid, array('field' => '`linktel`, `linkphone`, `linkman`, `address`'));
 			
			$jobid				=	intval($_GET['jobid']);
			$jobM 				=	$this -> MODEL('job');
			
			if(!empty($jobid)){
				
			    $job			=	$jobM -> getInfo(array('uid' =>	$fuid, 'status' => 0, 'state' => 1, 'id' => $jobid));				
					
				if(!empty($job)){
					
					if($job['is_link'] == 1){
					    
						$job['link_man']      =	  $company['linkman'];
						$job['link_moblie']   =	  $company['linktel'] ? $company['linktel'] : $company['linkphone'];
						$job['address']       =	  $company['address'];
					}elseif($job['is_link'] == 2){
					    
					    $job_link	=	$jobM -> getComJobLinkInfo(array(
					        'uid' => $fuid,
                            'PHPYUNBTWSTARTA' => 1,
                            'id' => $job['link_id'],
                            'jobid'=> array('=', $jobid, 'or'),
                            'PHPYUNBTWENDA' => 1
                        ), array('field' => '`link_man`, `link_moblie`,`link_address`'));
						$job['link_man']      =	  $job_link['link_man'];
						$job['link_moblie']   =	  $job_link['link_moblie'];
						$job['address']       =	  $job_link['link_address'];
					}
					
				}else{
				    
					$job                       =   $ltjobM -> getInfo(array('id' => $jobid, 'status' => 1,'z_status' => 0, 'uid' => $fuid));
					$job['name']               =   $job['job_name'];
					$job['link_man']           =   $company['linkman'];
					$job['link_moblie']        =   $company['linktel']?$company['linktel']:$company['linkphone'];
					$job['address']            =   $company['address'];
				}
				
				
			}else{
                $job['link_man']      =	  $company['linkman'];
				$job['link_moblie']   =	  $company['linktel'] ? $company['linktel'] : $company['linkphone'];
				$job['address']       =	  $company['address'];
            }
     
            $this -> yunset('job', $job);
			//公司旗下职位信息（包含职位联系方式）
			$joblistA    =   $jobM -> getList(array('uid' => $fuid, 'status' => 0, 'state' => 1, 'r_status' => 1), array('link'=>'yes', 'field' => '`id`, `name`, `is_link`'));
			$joblist     =   $joblistA['list'];    
			$this -> yunset('joblist', $joblist);
			

			//邀请模板
            $yqmbM  =   $this->MODEL('yqmb');
            $ymlist = $yqmbM  ->getList(array('uid'=>$this->uid,'status'=>1));
            $ymnum  = $yqmbM  ->getNum(array('uid'=>$this->uid));
            $ymcan  = $ymnum<$this->config['com_yqmb_num'] ? true : false; 
            
            $this->yunset('ymlist', $ymlist);
            $this->yunset('ymcan', $ymcan);
            
 			$this -> seo('invite_resume');
		}
		
		if($_GET['invite']){
		
		    $this->yunset('backurl', Url('wap',array('c' => 'look_resume'),'member'));
		}
		$this -> yunset('headertitle', '面试邀请');
		$this -> yuntpl(array('wap/invite'));
	}

    /**
     * 浏览历史记录
     */
	function history_action(){
	    
	    if ($_POST['eid'] && $this->usertype == 2) {

            $eid = intval($_POST['eid']);

            $resumeM = $this->MODEL('resume');
            $resume_expect = $resumeM->getExpect(array('id' => $eid), array('field' => '`uid`'));

            $lookM = $this->MODEL('lookresume');
            $lookM->browseResume(array('euid' => $resume_expect['uid'], 'uid' => $this->uid, 'usertype' => $this->usertype, 'did' => $this->config['did'], 'eid' => $eid));
	    }
	}
	/**
	 * 举报简历检查
	 */
	function report_action(){
	    $eid  =  intval($_POST['eid']);
	    if(!empty($eid)){
	        $Where = array(
	            'uid' => $this->uid,
	            'usertype' => $this->usertype,
	            'eid' => $eid
	        );
	        
	        $resumeM  =  $this->MODEL('resume');
	        $jlres	  =  $resumeM->openResumeCheck($Where);
	        if($jlres != 1){
	            echo 3;die;
	        }
	        $reportM	=	$this -> MODEL('report');
	        $report		=	$reportM -> getNum(array('eid' => $eid, 'p_uid' => $this -> uid));
	        if($report > 0){
	            echo 1; die;
	        }else{
	            echo 2; die;
	        }
	    }
	}
	/**
	 * 微信内上拉加载
	 */
	function ajaxLoad_action(){
	    
	    $param = array();
	    $searchurl = explode('&', $_POST['searchurl']);
	    foreach ($searchurl as $v){
	        $p = explode('=', $v);
	        $param[$p[0]] = $p[1];
	    }
	    $param['page'] = $_POST['page'];
	    $param['limit'] = 20;
	    $resumeM  =  $this->MODEL('resume');

        if ($_COOKIE['did'] > 0){
            if ($_COOKIE['fz_type'] == 1) {

                if ($_COOKIE['province'] > 0) {
                    $param['provinceid'] = $_COOKIE['province'];
                }
                if ($_COOKIE['cityid'] > 0) {
                    $param['cityid'] = $_COOKIE['cityid'];
                }
                if ($_COOKIE['three_cityid'] > 0) {
                    $param['three_cityid'] = $_COOKIE['three_cityid'];
                }
            } else if ($_COOKIE['fz_type'] == 2) {

                if ($_COOKIE['hyclass'] > 0){

                    $param['hy']    =   $_COOKIE['hyclass'];
                }
            }
        }
        
        $param['three_cityid']  =   $param['threecityid'];
        $param['job_post']      =   $param['jobpost'];
        $param['job1_son']      =   $param['job1son'];
	    
	    $resumerows  =  $resumeM->getList(
	        array(),
	        array(
	            'search'            =>  $param,
	            'withResumeField'   =>  'uid,name,nametype,tag,sex,moblie_status,edu,exp,photo,resume_photo,phototype,birthday,photo_status,def_job',
	            'downresume_where'  =>  array('comid'=>$this->uid,'usertype'=>$this->usertype),
                'look_where'        =>  array('com_id'=>$this->uid,'usertype'=>$this->usertype),
	            'workexp'           =>  1,
	            'url'               =>  1
	        )
	        );
	    
	    $data['list']  =  !empty($resumerows['list']) ? $resumerows['list'] : array();

	    echo json_encode($data);die;
	}
}
?>