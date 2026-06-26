<?php

class jfdk_model extends model{


    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {

        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }
	
	/**
	 * @desc   引用statis类，获取账户套餐数据信息
	 */
	private function getStatisInfo($uid, $data = array()) {
	    require_once ('statis.model.php');
	    $StatisM = new statis_model($this->db, $this->def);
	    return  $StatisM -> getInfo($uid , $data);
	}

    /**
     * @desc 套餐消耗明细
     * @param array $data
     * @return mixed
     */
    private function addStatisDetail($data = array())
    {
        require_once('statis.model.php');
        $statisM = new statis_model($this->db, $this->def);
        return $statisM->addStatisDetail($data);
    }
 
	
	/**
	 * @desc   积分支付
	 * @param  array $data
	 * @return array $return
	 */
	function dkBuy($data = array())
    {
        if ($data['uid']) {
            
            if($data['usertype']==2){
                $single_can = @explode(',', $this->config['com_single_can']);
            }

            if($data['server']!='vip' && $data['server']!='pack' && $data['server']!='autojob'){

                $serverCheck = $data['server'];
                if($data['server']=='sxpart'||$data['server']=='sxjob'){
                    $serverCheck = 'sxjob';
                }
                if($data['server']=='partrec'){
                    $serverCheck = 'jobrec';
                }
                if($serverCheck && !in_array($serverCheck,$single_can)){
                    return  array(
                        'error' => 1,
                        'msg'   => yun_at('common_00345')
                    );
                }
            }

            $serverStr  =   'vip,pack,issuejob,jobtop,jobrec,joburgent,sxjob,downresume,invite,zph,createson,chat,zphnet,spview';

            if (stripos($serverStr, $data['server']) !== false && stripos($this->config['sy_only_price'], $data['server']) !== false) {
                return array(
                    'error' => 1,
                    'msg' => yun_at('common_00915')
                );
            }

            if ($data['server'] == 'autojob') {
                
                $return = $this->buyAutoJob($data);
            } elseif ($data['server'] == 'jobtop') {
                
                $return = $this->buyZdJob($data);
            } elseif ($data['server'] == 'jobrec') {
                
                $return = $this->buyRecJob($data);
            } elseif ($data['server'] == 'joburgent') {
                
                $return = $this->buyUrgentJob($data);
            } elseif ($data['server'] == 'sxjob') {
                
                $return = $this->buyRefreshJob($data);
            }  elseif ($data['server'] == 'downresume') {
                
                $return = $this->downresume($data);
            } elseif ($data['server'] == 'issuejob') {
                
                $return = $this->buyIssueJob($data);
            } elseif ($data['server'] == 'invite') {
                
                $return = $this->buyInviteResume($data);
            } elseif ($data['server'] == 'pack') {
                
                $return = $this->buyPackOrder($data);
            } elseif ($data['server'] == 'vip') {
                
                $return = $this->buyVip($data);
            } elseif ($data['server'] == 'sxpart') {
                
                $return = $this->buyRefreshPart($data);
            } elseif ($data['server'] == 'partrec') {
                
                $return = $this->buyRecPart($data);
            } elseif ($data['server'] == 'zph') {
                
                $return = $this->buyZph($data);
                
            }
            if ($return['status'] == 1) {
                
                $status = 1;
                // 订单生成成功
                $return = array(
                    'error' => 0,
                    'msg'   => $return['msg']
                );
            } else {
                
                $status = 2;
                // 生成失败 返回具体原因
                $return = array(
                    'error' => 1,
                    'msg'   => $return['error'],
                    'url'   => $return['url']
                );
            }
        }else{
            $return = array(
                'error' => 1,
                'msg'   => yun_at('wap_00376')
            );
        }
        return $return;
    }

    // 积分抵扣，自动刷新
    function buyAutoJob($data)
    {
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
        
        $return     =   array();

        if ($this->config['com_integral_online'] != 2) {

            if ($data['jobautoids'] && ($data['days'] || $data['xdays'])) {

                $jobautoids =   pylode(',', @explode(',', $data['jobautoids']));

                // 判断自动刷新天数
                $autodays   =   intval($data['days']) > 0 ? intval($data['days']) : (intval($data['xdays']) > 1 ? intval($data['xdays']) : 1);
 
                if ($autodays > 0 && $jobautoids) {

                    // 判断职位ID真实性
                    $jobs   =   $this->select_all('company_job', array('uid' => $uid, 'id' => array( 'in', $jobautoids)), '`autotime`,`id`');

                    if (empty($jobs)) {

                        $return['error'] = yun_at('common_00925');
                    } else {

                        $jobnum     =   $this->select_num('company_job', array('uid' => $uid, 'id' => array('in', $jobautoids)));   // 计算自动刷新职位数量

                        
                        $price      =   $autodays * $jobnum * $this->config['job_auto'];
                        
                                                
                        $needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }

                        $statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`'));

                        if ($statis['integral'] >= $dkjf) {

                            // 积分抵扣，直接完成自动刷新购买
                            $autoJob=   $this->select_all('company_job', array('uid' => $uid, 'id' => array('in', $jobautoids)), '`autotime`,`id`');

                            if (! empty($autoJob)) {

                                foreach ($autoJob as $v) {

                                    if ($v['autotime'] >= time()) {

                                        $autotime = $v['autotime'] + $autodays * 86400;
                                    } else {

                                        $autotime = time() + $autodays * 86400;
                                    }

                                    $status =   $this->update_once('company_job', array('autotime' => $autotime), array('uid' => $uid, 'id' => $v['id']));
                                }

                                if ($status) {
                                    require_once ('integral.model.php');

                                    $integral   =   new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

                                    $integral->company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'] . 'common_06489', true, 2, 'integral', 12);

                                   	$return['status'] = '1';

                                    $return['msg'] = yun_at('common_06490');

                                    $logContent =   'common_06480'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'购买职位（ID：'.$jobautoids.'）自动刷新；自动刷新天数 + '.$autodays;
                                    $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);
                                }
                            }
                        } else {

                            if ($this->config['com_integral_online'] == 3) {

                                $return['error'] = $this->config['integral_pricename'] . 'wap_js_00136'.$this->config['integral_pricename'].'！';

                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {

                                $return['error'] = $this->config['integral_pricename'] . 'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {

                    $return['error'] = yun_at('common_00443');
                }
            } else {

                $return['error'] = yun_at('common_00700');
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }
	
	//积分抵扣，置顶职位
	function buyZdJob($data)
    {
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);

        $return     =   array();
        
        if ($this->config['com_integral_online'] != 2) {

            if ($data['zdjobid'] && ($data['days'] || $data['xdays'])) {

                $jobid  =   $data['zdjobid'];
                
                // 判断置顶天数
                $xsdays =   intval($data['days']) > 0 ? intval($data['days']) : (intval($data['xdays']) > 1 ? intval($data['xdays']) : 1);

                if ($xsdays > 0 && $jobid) {

                    // 判断职位ID真实性
                    $job    =   $this -> select_once('company_job', array('uid' => $uid, 'id' => $jobid));

                    if (empty($job)) {

                        $return['error'] = yun_at('common_00929');
                        
                    } else {
                        
                        
                        $price      =   $xsdays * $this->config['integral_job_top'];
                        
                      	$needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }

                        $statis =   $this -> select_once('company_statis', array('uid' => $uid), '`integral`');

                        if ($statis['integral'] >= $dkjf) {

                            $xsjob  =   $this->select_once('company_job', array('id' => $jobid), 'name,xsdate');

                            if (! empty($xsjob)) {
                                
                                if ($xsjob['xsdate'] > time()) {

                                    $xsdate = $xsjob['xsdate'] + $xsdays * 86400;
                                } else {

                                    $xsdate = strtotime('+' . $xsdays . ' day');
                                }

                                $status     =   $this->update_once('company_job', array('xsdate' => $xsdate), array('uid' => $uid, 'id' => $jobid));

                                if ($status) {

                                    require_once ('integral.model.php');

                                    $integral   =   new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

                                    $integral->company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'] . 'common_06491', true, 2, 'integral', 12);
                                    $return['status']   =   '1';

                                    $return['msg']      =   yun_at('common_06492');

                                    $logContent =   'common_06480'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'购买职位（ID：'.$jobid.'）置顶；置顶天数 + '.$xsdays;
                                    $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);
                                }
                            }
                        } else {

                            if ($this->config['com_integral_online'] == 3) {

                                $return['error'] = $this->config['integral_pricename'] . 'wap_js_00136'.$this->config['integral_pricename'].'！';

                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {

                                $return['error'] = $this->config['integral_pricename'] . 'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {

                    $return['error'] = yun_at('common_00488');
                }
            } else {

                $return['error'] = yun_at('common_00700');
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }

    // 积分抵扣，推荐职位
    function buyRecJob($data)
    {
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
        
        $return     =   array();
        
        if ($this->config['com_integral_online'] != 2) {

            if ($data['recjobid'] && ($data['days'] || $data['xdays'])) {

                $jobid      =   $data['recjobid'];

                // 判断推荐天数
                $recdays    =   intval($data['days']) > 0 ? intval($data['days']) : (intval($data['xdays']) > 1 ? intval($data['xdays']) : 1);
                 
                if ($recdays > 0 && $jobid) {

                    // 判断职位ID真实性
                    $job    =   $this -> select_once('company_job', array('uid' => $uid, 'id' => $jobid));

                    if (empty($job)) {

                        $return['error']    =   yun_at('common_00928');
                    } else {
                        
                        
                        $price      =   $recdays * $this->config['com_recjob'];
                       
                        $needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }
                        
                        $statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`'));

                        if ($statis['integral'] >= $dkjf) {

                            $recjob     =   $this -> select_once('company_job', array('id' => $jobid), '`name`,`rec_time`');

                            if (! empty($recjob)) {

                                if ($recjob['rec_time'] > time()) {

                                    $rec_time = $recjob['rec_time'] + $recdays * 86400;
                                } else {

                                    $rec_time = time() + $recdays * 86400;
                                }

                                $status =   $this->update_once('company_job', array('rec_time' => $rec_time, 'rec' => '1' ), array('uid' => $uid, 'id' => $jobid));

                                if ($status) {

                                    require_once ('integral.model.php');

                                    $integral   =   new integral_model($this->db, $this->def, array('uid' => $uid,  'username' => $username, 'usertype' => $usertype));

                                    $integral -> company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'].'common_06493', true, 2, 'integral', 12);
                                    
                                    $return['status']   =   '1';

                                    $return['msg']      =   yun_at('common_06494');

                                    $logContent =   'common_06480'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'购买职位（ID：'.$jobid.'）推荐；推荐天数 + '.$recdays;
                                    $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);
                                }
                            }
                        } else {

                            if ($this->config['com_integral_online'] == 3) {

                                $return['error'] = $this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';

                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {

                                $return['error'] = $this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {

                    $return['error'] = yun_at('common_00487');
                }
            } else {

                $return['error'] = yun_at('common_00700');
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }
    
    //积分抵扣，推荐兼职
    function buyRecPart($data)
    {
        
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
        
        $return     =   array();
        
        if ($this->config['com_integral_online'] != 2) {
            
            if ($data['recpartid'] && ($data['days'] || $data['xdays'])) {
                
                $partid     =   $data['recpartid'];
                
                // 判断推荐天数
                $recdays    =   intval($data['days']) > 0 ? intval($data['days']) : (intval($data['xdays']) > 1 ? intval($data['xdays']) : 1);
                
                if ($recdays > 0 && $partid) {
                    
                    // 判断职位ID真实性
                    $part   =   $this->select_once('partjob', array('uid' => $data['uid'], 'id' => $partid));
                    
                    if (empty($part)) {
                        
                        $return['error'] = yun_at('common_00928');
                        
                    } else {
                        
                        
                        $price      =   $recdays * $this->config['com_recjob'];
                        
                                                
                        $needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }
                        
                        $statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`'));
                        
                        if ($statis['integral'] >= $dkjf) {
                            
                            $recjob     =   $this->select_once('partjob', array('id' => $partid), '`name`,`rec_time`');
                            
                            if (! empty($recjob)) {
                                
                                if ($recjob['rec_time'] > time()) {
                                    
                                    $rec_time = $recjob['rec_time'] + $recdays * 86400;
                                } else {
                                    
                                    $rec_time = time() + $recdays * 86400;
                                }
                                
                                $status     =    $this->update_once('partjob', array('rec_time' => $rec_time), array('uid' => $uid, 'id' => $partid));
                                
                                if ($status) {
                                    
                                    require_once ('integral.model.php');
                                    
                                    $integral   =   new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));
                                    
                                    $integral->company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'] . 'common_06495', true, 2, 'integral', 12);
                                                                     
                                    $return['status']   =   '1';
                                    
                                    $return['msg']      =   yun_at('common_06496');

                                    $logContent =   'common_06480'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'购买兼职（ID：'.$partid.'）推荐；推荐天数 + '.$recdays;
                                    $this->addMemberLog($uid, $usertype, $logContent, 9, 4, $logDetail);
                                }
                            }
                        } else {
                            
                            if ($this->config['com_integral_online'] == 3) {
                                
                                $return['error'] = $this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';
                                
                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {
                                
                                $return['error'] = $this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {
                    
                    $return['error'] = yun_at('common_00487');
                }
            } else {
                
                $return['error'] = yun_at('common_00700');
            }
        } else {
            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }
    
	//积分抵扣，紧急招聘
	function buyUrgentJob($data)
    {
	    
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
        
        $return     =   array();
        
        if ($this->config['com_integral_online'] != 2 ) {

            if ($data['ujobid'] && ($data['days'] || $data['xdays'])) {

                $jobid  =   $data['ujobid'];

                // 判断紧急招聘天数
                $udays  =   intval($data['days']) > 0 ? intval($data['days']) : (intval($data['xdays']) > 1 ? intval($data['xdays']) : 1);
                
                if ($udays > 0 && $jobid) {

                    // 判断职位ID真实性
                    $job    =   $this -> select_once('company_job', array('uid' => $uid, 'id' => $jobid));

                    if (empty($job)) {

                        $return['error']    =   yun_at('common_01158');
                    } else {
                        
                        $price      =   $udays * $this->config['com_urgent'];
                                                
                        $needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }
                        
                        $statis =   $this -> getStatisInfo($uid, array('usertype' => $usertype, 'field'=>'`integral`'));

                         
                        if (intval($statis['integral']) >= $dkjf) {

                            $ujob   =   $this -> select_once('company_job', array('id' => $jobid), '`name`,`urgent_time`');

                            if (! empty($ujob)) {

                                if ($ujob['urgent_time'] > time()) {

                                    $urgent_time = $ujob['urgent_time'] + $udays * 86400;
                                } else {

                                    $urgent_time = strtotime('+' . $udays . ' day');
                                }

                                $status =   $this -> update_once('company_job', array('urgent_time' => $urgent_time, 'urgent' => '1'), array('uid' => $uid, 'id' => $jobid));

                                if ($status) {

                                    require_once ('integral.model.php');

                                    $integral   =   new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

                                    $integral -> company_invtal($uid,$usertype, $dkjf, false, $this->config['integral_pricename'] . 'common_06497', true, 2, 'integral', 12);
                                    
                                    
                                    $return['status'] = '1';

                                    $return['msg'] = yun_at('common_06498');

                                    $logContent =   'common_06480'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'购买职位（ID：'.$jobid.'）紧急招聘；紧急天数 + '.$udays;
                                    $this->addMemberLog($uid, $usertype, $logContent, 1, 4, $logDetail);
                                }
                            }
                        } else {

                            if ($this->config['com_integral_online'] == 3) {

                                $return['error'] = $this->config['integral_pricename'] . 'wap_js_00136'.$this->config['integral_pricename'].'！';

                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {

                                $return['error'] = $this->config['integral_pricename'] . 'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {

                    $return['error'] = yun_at('common_00544');
                }
            } else {

                $return['error'] = yun_at('common_00700');
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }
	
	//积分抵扣，购买会员
	function buyVip($data){
	    
	    $uid       =   intval($data['uid']);
	    $username  =   trim($data['username']);
	    $usertype  =   intval($data['usertype']);
		 
	    $return    =   array();
 
		if($this->config['com_integral_online']!=2){

			if($data['ratingid']){

				$id         =   intval($data['ratingid']);
				
				
				//判断套餐ID真实性
				$ratinginfo	=	$this -> select_once('company_rating',array('id'=>$id));
				
				$statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype));
			
				if(empty($ratinginfo)){
	
					$return['error']	=	yun_at('common_00594');

				}else {

									
					$needJf      =   $price * $this->config['integral_proportion'];
					
					if($needJf > intval($needJf)){
					    $dkjf    =   intval($needJf) + 1;
					}else{
					    $dkjf   =  intval($needJf);
					}
					
					$integral_dk  =   $dkjf;
					
					if($statis['integral'] >= $integral_dk){

						require_once('rating.model.php');

						$rating = new rating_model($this->db,$this->def,array('uid'=>$uid,'username'=>$username,'usertype'=>$usertype));

						if($usertype == 2){

							$value				=	$rating	->	ratingInfo($id, $uid);

							$return['status']	=   $this -> update_once('company_statis',$value,array('uid' => $uid));
              
							if ($return['status']) {
							    
							    $companydata     =	array(
							        'rating'	    =>	$value['rating'],
							        'rating_name'	=>	$value['rating_name'],
							        'vipetime'		=>	$value['vip_etime'],
							        'vipstime'		=>	$value['vip_stime']
							    );
							    
							    $this -> update_once('company', $companydata, array('uid' => $uid));
							}
							
							$this	->	update_once('company_job',array('rating' => $id),array('uid'=> $uid));
							
						}

						require_once('integral.model.php');

						$integral	=	new integral_model($this->db,$this->def,array('uid'=>$uid,'username'=>$username,'usertype'=>$usertype));

						$integral	->	company_invtal($uid, $usertype, $integral_dk, false, $this->config['integral_pricename'].'common_06499', true, 2, 'integral', 27);

						
						$return['status']	=	'1';
						$return['msg']		=	yun_at('common_06500');

                        if ($price > 0){

                            $logContent =   'common_06481'.$this->config['integral_pricename'].'member_user_00285';
                            $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'default_00090'.$ratinginfo['name'];
                        }else{

                            $logContent =   'common_06482'.$ratinginfo['name'];
                        }

                        $this->addMemberLog($uid, $usertype, $logContent, 88, 1, $logDetail);

					}else{

						if($this->config['com_integral_online']==3){

							$return['error']	=	$this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';

							$return['url']		=	$this->config['sy_weburl'].'/member/index.php?c=pay';

						}else{

							$return['error']	=	$this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
						}
					}
				}
			} else {

				$return['error'] = yun_at('common_00700');
			}
		}else{

			$return['error'] = yun_at('common_00804');
		}
		return $return;
	}

	//积分抵扣，购买增值套餐

	function buyPackOrder($data){

		$uid      =   intval($data['uid']);
		$usertype =   intval($data['usertype']);
		$username =   trim($data['username']);
		
		$return   =   array();
		
		if($this->config['com_integral_online']!=2){

			if($data['tcid']){

				$tid        =	intval($data['tcid']);
				
				
				if($tid){

					//判断套餐ID真实性
                    $tb_service =   'company_service_detail';
                    
                    $service	=	$this -> select_once($tb_service , array('id' => $tid));
                    
					if(empty($service)){

						$return['error']		=	yun_at('common_01064');

					}else {
                        
					    $statis	=	$this -> getStatisInfo($uid, array('usertype' => $usertype, 'field'=>'`integral`,`rating`,`vip_etime`'));
					    
					    if(!isVip($statis['vip_etime'])){

	
							$return['error'] =	yun_at('common_00574');
 
						}else{

							$rating			=	$this -> select_once('company_rating',array('id'=>$statis['rating']),'service_discount');//增值服务折扣

							if($rating['service_discount']){

								$discount	=	intval($rating['service_discount']);

								$price		=	$service['service_price'] * $discount * 0.01 ;

							}else{

								$price		=	$service['service_price'];

							}	
							
							$needJf     =   $price * $this->config['integral_proportion'];

                            if ($needJf > intval($needJf)) {

                                $dkjf   =   intval($needJf) + 1;
                            } else {

                                $dkjf   =   intval($needJf);
                            }
							
							if($statis['integral'] >= $dkjf){
                                
							    if($usertype == 2){
    								
							        $value		=	array(
    								
    								    'job_num'		=>	array('+', $service['job_num']?$service['job_num']:0),
    								    'breakjob_num'	=>	array('+', $service['breakjob_num']?$service['breakjob_num']:0),
    								    'down_resume'	=>	array('+', $service['resume']?$service['resume']:0),
    								    'invite_resume'	=>	array('+', $service['interview']?$service['interview']:0),
    								    'zph_num'	    =>	array('+', $service['zph_num']?$service['zph_num']:0),
    								    'top_num'	    =>	array('+', $service['top_num']?$service['top_num']:0),
    								    'rec_num'	    =>	array('+', $service['rec_num']?$service['rec_num']:0),
    								    'urgent_num'    =>	array('+', $service['urgent_num']?$service['urgent_num']:0)
     								);
    							
							        $status	=	$this	->	update_once('company_statis',$value,array('uid' => $uid));
							        
							    }

								if($status){

									require_once('integral.model.php');

									$integral	=	new integral_model($this->db,$this->def,array('uid'=>$uid,'username'=>$username,'usertype'=>$usertype));

									$integral	->	company_invtal($uid, $usertype, $dkjf,false,$this->config['integral_pricename'].'common_06501',true,2,'integral',12);
                                   
									
									$return['status']	=	'1';

									$return['msg']		=	yun_at('common_06502');

                                    $logContent =   'common_06483'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'common_06429';

                                    $this->addMemberLog($uid, $usertype, $logContent, 88, 1, $logDetail);
								}
							}else{

								if($this->config['com_integral_online']==3){

									$return['error']	=	$this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';
			
									$return['url']		=	$this->config['sy_weburl'].'/member/index.php?c=pay';
	
								}else{

									$return['error']	=	$this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
						
								}
							}
						}
					}
				}else{

					$return['error']	=	yun_at('common_00812');
				}
			} else {
			
				$return['error']	=	yun_at('common_00700');
			}
		}else{
		
			$return['error']	=	yun_at('common_00804');
		}
		return $return;
	}

	//积分抵扣，刷新职位

	function buyRefreshJob($data)
    {
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
        
        $return     =   array();
        if ($this->config['com_integral_online'] != 2) {

            if ($data['sxjobid']) {
                if($data['sxjobid'] == 'all'){
                    $sxjobids    =   array();
                    $jobwhere['uid']     =   $uid;
                    $jobwhere['state']      =   1;
                    $jobwhere['r_status']   =   array('<>',2);
                    $jobwhere['status']     =   array('<>',1);
                    $sxjobs   = $this->select_all('company_job', $jobwhere, '`id`');

                    foreach($sxjobs as $sk=>$sv){

                        $sxjobids[] = $sv['id'];

                    }
                    $sxjobid = pylode(',', $sxjobids);
                    
                }else{

                    $sxjobid = pylode(',', @explode(',', $data['sxjobid']));

                }
                

                if ($sxjobid) {

                    $statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`,`breakjob_num`'));
                    
                    $breakjob_num   =   intval($statis['breakjob_num']);
                    
                    // 判断职位ID真实性
                    $jobs   = $this->select_all('company_job', array('uid' => $uid, 'id' => array('in', $sxjobid)), '`id`,`name`');
                    
                    if (empty($jobs)) {

                        $return['error'] = yun_at('common_00927');
                    } else {

                        $jobnum     =   $this->select_num('company_job', array('uid' => $uid, 'id' => array('in', $sxjobid)));

                        // 优先扣除套餐

                        if ($breakjob_num) {

                            $jobnum = $jobnum - $breakjob_num;
                        }
                        
                        $price      =   $jobnum * $this->config['integral_jobefresh'];
                        
                        $needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }

                        if ($statis['integral'] >= $dkjf) {

                            // 积分抵扣，直接职位刷新
                            $status =   $this->update_once('company_job', array('lastupdate' => time()), array('id' => array('in', $sxjobid)));

                            $this->update_once('company', array('lastupdate' => time()), array('uid' => $uid));
                            $this->update_once('hot_job', array('lastupdate' => time()), array('uid' => $uid));

                            if ($breakjob_num) {

                                $this->update_once('company_statis', array('breakjob_num' => '0'), array('uid' => $uid));

                                $payDetail      =   $this->config['integral_pricename'].'common_06503'.$breakjob_num;
                                $this->addStatisDetail(array('uid' => $uid, 'type' => 2, 'num' => $breakjob_num, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                            }

                            if ($status) {

                                require_once ('integral.model.php');

                                $integral = new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

                                $integral->company_invtal($data['uid'], $data['usertype'], $dkjf, false, $this->config['integral_pricename'] . 'common_06504', true, 2, 'integral', 12);

                                if ($jobnum == 1) {

                                    $logContent =   'common_06484'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'刷新职位《'.$jobs[0]['job_name'].'》';
                                }else{

                                    $logContent =   'common_06484'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'common_01013'.$sxjobid.'）';
                                }

                                $this->addMemberLog($data['uid'], $data['usertype'], $logContent, 1, 4, $logDetail);

                                $return['status'] = '1';

                                $return['msg'] = yun_at('common_01530');
                            }
                        } else {

                            if ($this->config['com_integral_online'] == 3) {

                                $return['error'] = $this->config['integral_pricename'] . 'wap_js_00136'.$this->config['integral_pricename'].'！';

                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {

                                $return['error'] = $this->config['integral_pricename'] . 'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {

                    $return['error'] = yun_at('common_06463');
                }
            } else {

                $return['error'] = yun_at('common_00700');
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }

	// 积分抵扣，刷新兼职
	function buyRefreshPart($data)
    { 
	    
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
 
        $return     =   array();
        if ($this->config['com_integral_online'] != 2) {

            if ($data['sxpartid']) {

                $sxpartid   =   pylode(',', @explode(',', $data['sxpartid']));

                if ($sxpartid) {

                    $statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`,`breakjob_num`'));

                    $breakjob_num   =   intval($statis['breakjob_num']);

                    // 判断职位ID真实性
                    $parts  =   $this->select_all('partjob', array('uid' => $uid,'id' => array('in', $sxpartid)), '`id`,`name`');

                    if (empty($parts)) {

                        $return['error'] = yun_at('common_00927');
                    } else {

                        $partnum = $this->select_num('partjob', array('uid' => $uid,'id' => array('in', $sxpartid)));

                        // 优先扣除套餐
                        if ($breakjob_num) {

                            $partnum = $partnum - $breakjob_num;
                        }
                        
                        $price      =   $partnum * $this->config['integral_jobefresh'];
                        
						$needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }

                        if ($statis['integral'] >= $dkjf) {

                            // 积分抵扣，直接刷新兼职
                            $status =   $this->update_once('partjob', array('lastupdate' => time()), array('id' => array('in', $sxpartid)));
                            
                            if ($status) {
                                
                                if ($breakjob_num) {
                                    
                                    $this->update_once('company_statis', array('breakjob_num' => '0'), array('uid' => $uid));

                                    $payDetail      =   $this->config['integral_pricename'].'common_06503'.$breakjob_num;
                                    $this->addStatisDetail(array('uid' => $uid, 'type' => 2, 'num' => $breakjob_num, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                                }
                                $this->update_once('company', array('lastupdate' => time()), array('uid' => $uid));
                                $this->update_once('hot_job', array('lastupdate' => time()), array('uid' => $uid));
                                
                                require_once ('integral.model.php');
                                
                                $integral = new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));
                                
                                $integral->company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'].'common_06505', true, 2, 'integral', 12);

                                if ($partnum == 1) {

                                    $logContent =   'common_06484'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'刷新兼职职位《'.$parts[0]['job_name'].'》';
                                }else{

                                    $logContent =   'common_06484'.$this->config['integral_pricename'].'member_user_00285';
                                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'批量刷新兼职职位（ID：'.$sxpartid.'）';
                                }

                                $this->addMemberLog($uid, $data['usertype'], $logContent, 9, 4, $logDetail);
                                
                                $return['status'] = '1';
                                
                                $return['msg'] = yun_at('common_06466');
                            }
                            
                        } else {

                            if ($this->config['com_integral_online'] == 3) {

                                $return['error'] = $this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';

                                $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                            } else {

                                $return['error'] = $this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
                            }
                        }
                    }
                } else {

                    $return['error'] = yun_at('common_00918');
                }
            } else {

                $return['error'] = yun_at('common_00700');
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }
	
	
    
	 
    
	//积分抵扣，下载简历
	function downresume($data){
	     
        $uid	  =	  intval($data['uid']);
        $usertype =   intval($data['usertype']);
        $username =   trim($data['username']);
        $did      =   $data['did'] ? $data['did'] : $this -> config['did'];
        
        $return   =   array();
        
		require_once('integral.model.php');

		$integral 		= 		new integral_model($this->db,$this->def,array('uid'=>$uid,'username'=>$username,'usertype'=>$usertype));
		
		require_once ('resume.model.php');
		
		$resumeM		=		new resume_model($this->db, $this->def);
		
		if($this->config['com_integral_online']!=2){

			if($data['eid']){

				$eid = intval($data['eid']);

				if($eid){

				    $isDownresume   =   $this->select_once('down_resume', array('eid' => $eid, 'comid' => $uid,'usertype'=>$usertype));
				    
				    if (!empty($isDownresume)) {
				        
				        $return['msg']      =   yun_at('common_00471');
				        $return['status']   =   '1';
				        
				        return $return;
				    }
				    
					//判断简历ID真实性
				    $user       =  $this->select_once('resume_expect',array('id'=>$eid), '`id`,`uid`,`height_status`,`name`');
                   
				    $downdata   =   array();
                    
                    $downdata['eid']        =   $user['id'];
                    $downdata['uid']        =   $user['uid'];
                    $downdata['comid']      =   $uid;
                    $downdata['usertype']   =   $usertype;
                    $downdata['did']        =   $did;
                    $downdata['type']       =   $user['height_status'];
                    $downdata['downtime']   =   time();
                    
                    if(empty($user)){
                        
                        $return['error']    =   yun_at('common_06506');
                        
                    }else {
                        
                        $price      =   $resumeM -> setDayprice($eid);
                        
                        $needJf      =   $price * $this->config['integral_proportion'];
                        
                        if($needJf > intval($needJf)){
                            $dkjf    =   intval($needJf) + 1;
                        }else{
                            $dkjf   =  intval($needJf);
                        }
						
						$statis     =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`'));

						if($statis['integral'] >= $dkjf){

							//积分抵扣，直接下载简历
							$nid = $this -> insert_into('down_resume',$downdata);

							if($nid){

                                $integral->company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'] . 'common_06507', true, 2, 'integral', 12, $eid);
                                $this->update_once('resume_expect', array('dnum' => array('+', '1')), array('id' => $eid));

                                $logContent =   'common_06485'.$this->config['integral_pricename'].'member_user_00285';
                                $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'common_06508'.$user['name'].'）';

                                $this->addMemberLog($uid, $usertype, $logContent, 3, 1, $logDetail);

								$return['status']   =   '1';
								$return['msg']      =   yun_at('common_06509');
							}
							
						}else{

							if($this->config['com_integral_online']==3){

								$return['error']    =   $this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';

								$return['url']      =   $this->config['sy_weburl'].'/member/index.php?c=pay';
							
							}else{

								$return['error']    =   $this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
							}
						}
					}
				}
			} else {

				$return['error'] = yun_at('common_00861');
			}
		}else{

			$return['error'] = yun_at('common_00804');
		}
		return $return;
	}

	//积分抵扣，发布职位
	function buyIssueJob($data)
    {
        
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);
        
        $return     =   array();
        
        require_once ('statis.model.php');
        $StatisM    =   new statis_model($this->db, $this->def);

        require_once ('integral.model.php');
        $integral   =   new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));
        
        if ($this->config['com_integral_online'] != 2) {

            $price      =   $this->config['integral_job'];
            
            $needJf      =   $price * $this->config['integral_proportion'];
            
            if($needJf > intval($needJf)){
                $dkjf    =   intval($needJf) + 1;
            }else{
                $dkjf   =  intval($needJf);
            }
            
            $statis =   $this -> getStatisInfo($uid, array('usertype' => $usertype, 'field'=>'`integral`'));
            
            if ($statis['integral'] >= $dkjf) {
                
                $msg    =   'common_06222';
                // 积分抵扣，会员发布职位套餐加1
                $sValue =   array('job_num' => array('+', 1));
                
                $status =   $StatisM -> upInfo($sValue, array('uid' => $uid, 'usertype' => $usertype));
                
                if ($status) {
                    
                    $integral->company_invtal($uid, $usertype, $dkjf, false, $this->config['integral_pricename'].'抵扣，'.$msg, true, 2, 'integral', 12);
                    
                   	$return['status']   =   '1';
                    $return['msg']      =   $msg . 'admin_tool_00502';

                    $logContent =   'common_06486'.$this->config['integral_pricename'].'member_user_00285';
                    $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'common_06510';

                    $this->addMemberLog($uid, $usertype, $logContent, 1, 1, $logDetail);
                }
            } else {
                
                if ($this->config['com_integral_online'] == 3) {
                    
                    $return['error'] = $this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';
                    
                    $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                } else {
                    
                    $return['error'] = $this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
                }
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }

	//积分抵扣，邀请面试
	function buyInviteResume($data)
    {
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $username   =   trim($data['username']);

        $return     =   array();

        require_once ('statis.model.php');

        $StatisM    = new statis_model($this->db, $this->def);

        require_once ('integral.model.php');

        $integral   = new integral_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

        if ($this->config['com_integral_online'] != 2) {

            if (!$data['uid']) {
                
                $return['error'] = yun_at('common_00900');
            } else {
                
                $price      =   $this->config['integral_interview'];
                
				$needJf      =   $price * $this->config['integral_proportion'];
                
                if($needJf > intval($needJf)){
                    $dkjf    =   intval($needJf) + 1;
                }else{
                    $dkjf   =  intval($needJf);
                }
                
                $statis =   $this -> getStatisInfo($uid,array('usertype' => $usertype, 'field'=>'`integral`'));
                
                if ($statis['integral'] >= $dkjf) {
                    
                    $status = $StatisM -> upInfo(array('invite_resume' => array('+', 1)), array('uid' => $uid, 'usertype' => $usertype));
                    
                    if ($status) {
                        
                        $integral -> company_invtal($data['uid'], $data['usertype'], $dkjf, false, $this->config['integral_pricename'] . 'common_06511', true, 2, 'integral', 12);                                               
                        $return['status']   =   '1';
                        
                        $return['msg']      =   yun_at('common_06512');

                        $logContent =   'common_06487'.$this->config['integral_pricename'].'member_user_00285';
                        $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'common_00308';

                        $this->addMemberLog($data['uid'], $uid, $logContent, 4, 1, $logDetail);
                    }
                } else {
                    
                    if ($this->config['com_integral_online'] == 3) {
                        
                        $return['error'] = $this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';
                        
                        $return['url'] = $this->config['sy_weburl'] . '/member/index.php?c=pay';
                    } else {
                        
                        $return['error'] = $this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
                    }
                }
            }
        } else {

            $return['error'] = yun_at('common_00804');
        }
        return $return;
    }
 
    /**
     * 积分抵扣，报名招聘会
     */
	function buyZph($data = array()){
	    
	    $uid       =   intval($data['uid']);
	    $username  =   trim($data['username']);
	    $usertype  =   intval($data['usertype']);
	    
	    $return    =   array();
	    
	    require_once ('statis.model.php');
	    $StatisM   =   new statis_model($this->db, $this->def);
	    
	    require_once ('company.model.php');
	    $comM      =   new company_model($this->db, $this->def);
		
		require_once('integral.model.php');
        $integralM =   new integral_model($this->db,$this->def,array('uid'=>$uid,'username'=>$username,'usertype'=>$usertype));
		
		require_once('zph.model.php');
		$zphM      =   new zph_model($this->db,$this->def);
		
		
		if($this->config['com_integral_online']!=2){
		    
			if($data['zid'] && $data['bid']){
			    
			    $zid     =   $data['zid'] ? intval($data['zid']) : '';
			    $bid     =   $data['bid'] ? intval($data['bid']) : '';
			    
			    $com     =   $comM -> getInfo($uid, array('field' => '`name`'));
			    $zph     =   $zphM -> getInfo(array('id' => $zid));
			    
			    $zphcom  =   $zphM -> getZphComInfo(array('uid' => $uid, 'zid' => $zid));
			    
			    if ($zphcom && is_array($zphcom)) {
			        
			        if ($zphcom['status'] == 2) {
			            
			            $return['error'] = yun_at('common_06513');
			        } else {
			            
			            $return['error'] = yun_at('common_01124');
			        }
 			        
			    } else if (empty($zph)) {
			        
			        $return['error']     =	yun_at('common_00783');
			    }else{
			        
			        $space               =   $zphM -> getZphSpaceInfo(array('id' => $bid));
			        $sid                 =   $zphM -> getZphSpaceInfo(array('id' => $space['keyid']));
			        $zData               =   array();
			        
			        $zData['uid']        =   $uid;
			        $zData['com_name']   =   $com['name'];
			        $zData['zid']        =   $zid;
 			        $zData['ctime']      =   time();
			        $zData['status']     =   0;
			        $zData['sid']        =   $sid['keyid'];
			        $zData['cid']        =   $space['keyid'];
			        $zData['bid']        =   $bid;
 			        $zData['price']      =   $space['price'];
 			        // 参会职位处理
 			        if (!empty($data['jobid'])){
 			            // pc
 			            $zData['jobid']  =   $data['jobid'];
 			            
 			        }elseif (!empty($_COOKIE['zphjobid'])){
 			            // wap
 			            $zData['jobid']  =   $_COOKIE['zphjobid'];
 			        }
 			        
 			        $price      =   $space['price'] / $this->config['integral_proportion'];
 			         			        
 			        $needJf      =   $price * $this->config['integral_proportion'];
 			        
 			        if($needJf > intval($needJf)){
 			            $dkjf    =   intval($needJf) + 1;
 			        }else{
 			            $dkjf   =  intval($needJf);
 			        }
    			    
    			    $statis	             =   $StatisM -> getInfo($uid, array('usertype'=>$usertype, 'field' => '`integral`'));
    			    
    			    if($statis['integral'] >= $dkjf){
                        
    			        $status          =   $this->insert_into('zhaopinhui_com', $zData);
    			        
    			        if($status){
    			            
    			            $integralM -> company_invtal($uid, $usertype,$dkjf, false,$this->config['integral_pricename'].'common_06514',true,2,'integral');//积分操作记录    			               			           

                            $logContent =   'common_06488'.$this->config['integral_pricename'].'common_01991';
                            $logDetail  =   'member_user_00284'.$this->config['integral_pricename'].'报名招聘会（ID：'.$data['zid'].'common_06515'.$bid.'）';

                            $this->addMemberLog($uid, $usertype,$logContent,14,1, $logDetail);

    			            $return['status']  =   1;
    			            $return['msg']     =   yun_at('common_00714');
    			        }
    			        
    			    }else{
    			        
    			        if($this->config['com_integral_online']==3){
    			        
    			            $return['error'] 	= 	$this->config['integral_pricename'].'wap_js_00136'.$this->config['integral_pricename'].'！';
    			            $return['url'] 		= 	$this->config['sy_weburl'].'/member/index.php?c=pay';
    			            
    			        }else{
    			            
    			            $return['error'] 	= 	$this->config['integral_pricename'].'common_00969'.$this->config['integral_pricename'].'！';
    			        }
    			    }
			    }
			}else{
				$return['error']	=	yun_at('upgrade_00015'); 
			}
		}else{
			$return['error']	=	yun_at('common_00804');
		}
		return $return;
	}
		
}
?>