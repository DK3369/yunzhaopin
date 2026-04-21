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
class index_controller extends com_controller{
    // 企业会员中心首页
    function index_action()
    {
        $info      	=  $this->comInfo;
        $userinfoM  =  $this -> MODEL('userinfo');
		$jobM		=  $this -> MODEL('job');
		$orderM  	=  $this -> MODEL('companyorder');
        $msgNumM   	=  $this -> MODEL('msgNum');
        $msg       	=  $msgNumM -> getmsgNum($this->member['uid'],2,array('from' => 'wxapp'));

        $suid		=  $this->member['uid'];
        $statis   	=  $this->company_statis($suid);

        if ($this->config['com_enforce_info'] == 1) {
            if (empty($this->comInfo['name']) || empty($this->comInfo['address']) || empty($this->comInfo['pr'])) {
 
             	$return['remind'] = 1;               
            }
        }

        if (empty($info['name'])){
            $info['name']   =  $this->member['username'];
        }
        $member	=	$userinfoM->getInfo(array('uid'=> $this->member['uid']));
        if ($info['r_status'] == 3){
            $info['lock_info'] = $member['lock_info'];
        }
        $return['msg']     	=  $msg;
        $return['info']    	=  $info;
        $return['statis']  	=  $statis;
        $date				=  date('Ymd');
		$reg				=  $userinfoM -> getMemberregInfo(array('uid'=>$this->member['uid'],'usertype'=>$this->member['usertype'],'date'=>$date));
		$signstate			=  $reg['id'] ? 1 : 0;
		$return['signstate']=  $signstate;
		
		$return['nofkorder']	=  $orderM	->	getCompanyOrderNum(array('uid'=>$this->member['uid'],'usertype' =>$this->member['usertype'],'order_state'=>'1'));
		// 在招职位数量
		$jobwhere['uid']		=   $this->member['uid'];
		$jobwhere['state']		=	1;
		$jobwhere['status']		=	0;
        $normal_job_num         =   $jobM -> getJobNum($jobwhere);;
		$return['jobnums']		=	$normal_job_num;
		// 在招职位中，今天未刷新的数量
		$jobwhere['lastupdate'] =   array('<', strtotime('today'));
		$return['noRefreshNum'] =   $jobM -> getJobNum($jobwhere);
		
		$webtel 				= 	$this->config['sy_comwebtel']?$this->config['sy_comwebtel']:$this->config['sy_freewebtel'];

		$return['iosfk']		=	$this->config['sy_iospay'] ;
		$return['webtel']		=	$webtel;
		$return['worktime']		=	!empty($this->config['sy_worktime']) ? $this->config['sy_worktime'] : '';
		// 客户服务
		$adminM            		=   $this -> MODEL('admin');
		$guweninfo          	=   $adminM -> getAdminUser(array('uid' => $info['crm_uid']));
	    $guweninfo['photo_n']	 =  checkpic($guweninfo['photo'], $this->config['sy_guwen']);
	    $guweninfo['ewm_n']  	 =  checkpic($guweninfo['ewm']);
	    $guweninfo['freewebtel'] =  !empty($guweninfo['moblie']) ? $guweninfo['moblie'] : $webtel;
		$return['guweninfo']	=	$guweninfo ;
        
		$return['config']  =  array(

		    'part'     =>  isset($this->config['sy_part_web']) ? $this->config['sy_part_web'] : 2,
		    'zph'      =>  isset($this->config['sy_zph_web']) ? $this->config['sy_zph_web'] : 2,
		    'special'  =>  isset($this->config['sy_special_web']) ? $this->config['sy_special_web'] : 2
		);
		// 强制关注公众号
		if(isset($this->config['com_gzgzh']) && $this->config['com_gzgzh'] == 1){
		    $return['gzhurl'] = Url('wap', array('c'=>'ajax','a'=>'gzhqrcode','token'=>$this->member['gzhtoken']));
		    $return['config']['com_gzgzh'] = 1;
		}else{
		    $return['config']['com_gzgzh'] = 0;
		}
		$return['qqdt']     =  !empty($this->config['sy_qqdt']) ? $this->config['sy_qqdt'] : 2;
		$return['wxlogin']  =  !empty($this->config['sy_app_wxlogin']) ? $this->config['sy_app_wxlogin'] : 2;
		$return['qqlogin']  =  !empty($this->config['sy_app_qqlogin']) ? $this->config['sy_app_qqlogin'] : 2;

        $WhbM       =   $this->MODEL('whb');
        $maxNum     =   $normal_job_num > 6 ? 6 : $normal_job_num;
        $syComHb    =   $WhbM->getWhbList(array('type' => 2, 'isopen' => 1, 'num' => $maxNum ));
        $return['hbNum']    =   count($syComHb);
        $return['ishb']     =  $this->config['sy_haibao_isopen'] == 1 ? true : false;
        
        $thisyear = strtotime(date('Y').'-01-01');

		$return['yrshow'] = 0;
		$return['yrbtn'] = 0;
        if($member['reg_date']<$thisyear && $this->config['sy_yearreport_isopen']==1){
        	
        	$return['yrshow'] = 1;
        	
        	$return['yrbtn'] = 1;

        	$return['yrTip'] = checkpic($this->config['sy_yearreport_tip']);
        }
        $return['paused']   =   $this->comInfo['r_status'] == '4' ? 1 : 2;
        $comM = $this -> MODEL('company');
        $ratinginfo = $comM->getRatingInfo(array('rating' => $statis['rating'], 'field' => 'service_price'));
        $sjtip = 1;// 展示升级套餐提醒
        if ($statis['rating'] > 0){
            $comM = $this -> MODEL('company');
            $ratinginfo = $comM->getRatingInfo(array('rating' => $statis['rating'], 'field' => 'service_price'));
            if ($ratinginfo['service_price'] > 100 && isVip($statis['vip_etime'])) {
                $sjtip = 0;
            }
        }
        $return['sjtip'] = $sjtip;
        $this->render_json(0, 'ok', $return);
    }
	//签到，TODO:会员中心
	function sign_action(){
		
		$userinfoM		=	$this -> MODEL('userinfo');
		$date			=	date("Ymd");
		$member			=	$userinfoM -> getInfo(array('uid'=>$this->member['uid'],'usertype'=>$this->member['usertype']),array('field'=>"`signday`,`signdays`"));
		$lastreg		=	$userinfoM -> getMemberregInfo(array('uid'=>$this->member['uid'],'usertype'=>$this->member['usertype'],'orderby'=>'id,desc'));
		$lastregdate	=	date("Ymd",$lastreg['ctime']);
		if($lastregdate!=$date){
			$yesterday	=	date("Ymd",strtotime("-1 day"));
			if($lastregdate==$yesterday&&intval(date("d"))>1){
				if($member['signday']>=5){
					$integral	=	$this->config['integral_signin']*2;
				}else{
					$integral	=	$this->config['integral_signin'];
				}
				$signday	=	$member['signday']+1;
				$msg		=	'连续签到'.$signday."天";
			}else{
				$signday	=	'1';
				$integral	=	$this->config['integral_signin'];
				$msg		=	'第一次签到';
			}
			$arr	=	array();
			$nid	=	$userinfoM -> addMemberreg(array('uid'=>$this->member['uid'],'usertype'=>$this->member['usertype'],'date'=>$date,"ctime"=>time(),'ip'=>fun_ip_get()));
			if($nid){
			    $IntegralM		=	$this -> MODEL('integral');
				$IntegralM->company_invtal($this->member['uid'],$this->member['usertype'],$integral,true,$msg,true,2,'integral',19);		
				$userinfoM -> upInfo(array('uid'=>$this->member['uid']),array('signday'=>$signday,'signdays'=>array('+','1')));			
				$data['msg']	=	'签到成功！+'.$integral.$this->config['integral_pricename'];
				$data['error']	=	1;				
			}else{
				$data['msg']	=	'签到失败！';
				$data['error']	=	2;
			}
		}else{
			$data['msg']	=	'签到失败！';
			$data['error']	=	2;
		}
		$this->render_json($data['error'],$data['msg'],$data);
	}
	/**
	 * 检测职位发布数量
	 */
	function addCheck_action()
	{

	    if ($this->comInfo['r_status'] == '4'){
            $this->render_json(-1, '当前账户会员权益已暂停，请联系客服开启服务~');
        }

	    $jobM    =  $this->MODEL('job');
	    $provider = isset($_POST['provider'])?$_POST['provider']:'';
	    $need    =  $jobM -> getAddJobNeedInfo($this->member['uid'], 1, '', 1,array('provider'=>$provider));
	    
	    if(!empty($need['wxapp'])){
	        
	        $this->render_json(0, 'ok', array('need'=>$need['wxapp']));
	    }
	    
	    $result  =  $this->day_check($this->member['uid'], 'jobnum');
	    
	    if ($result['status'] == 1){
	        $suid	 =	$this->member['uid'];
	        $statis  =  $this -> company_statis($suid);

	        if ($statis['job_num'] == 0) {
                $return['msg']  =   '套餐已用完 , 立即升级VIP？';
            }else{
				if($statis['addjobnum']==0){
		        	$return['msg']='套餐已用完 , 立即升级VIP？';
				}elseif($statis['addjobnum']==2){
				    $return['msg']='当前会员套餐可上架职位数已达上限，新发布职位将无法直接上架哦~';
				}
            }
	        
			$return['job_num']  =   $statis['job_num'];
            $return['num']      =   $statis['addjobnum'];

	        $this->render_json(0, 'ok',$return);
	        
	    }else{
	        
	        $this->render_json(-1, $result['msg']);
	    }
	}

}