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
class user extends common{
	
	function __construct($tpl,$db,$def='',$model='index',$m='') {

		$this->common($tpl,$db,$def,$model,$m);
		$this->member_satic();
		//判断是否强制创建简历(等同于企业强制完善基本资料)
		$resumeM    =  $this->MODEL('resume');

		if($this->config['user_resume_status']=='1'){
			 
			if(!in_array($_GET['c'],array('binding','expect','log')) && !in_array($_GET['act'],array('logout','add'))){
				
				$expectnum  =  $resumeM	->	getExpectNum(array('uid'=>$this->uid));
				
				if($expectnum < 1){
					
					$remindInfo['url']		=	'index.php?c=expect&act=add';
					$remindInfo['title']	=	'请先创建一份简历！';
					$remindInfo['msg']		=	'一份完整的简历是您成功求职的良好起点！';

					$this   ->  yunset('isremind',1);
					$this	->	yunset('remindInfo',$remindInfo);

					$this   ->  user_tpl('expect_add');
				}
			}
		}else{
			$resume	=	$resumeM -> getResumeInfo(array('uid'=>$this->uid));
				
			if(!$resume['uid']){
			
				//容错机制，前期强制完善资料，后期开放，防止部分数据无uid 又可以直接操作会员中心
				$userinfoM    =   $this->MODEL("userinfo");
				$userinfoM -> activUser($this->uid,1);
			}
		}	
	}
	
	function public_action()
	{
	    $resumeM	=	$this->MODEL('resume');
	    $resume		=	$resumeM -> getResumeInfo(array('uid'=>$this->uid),array('logo'=>1));
		$expectnum	=	$resumeM ->	getExpectNum(array('uid'=>$this->uid));
		
		$this -> yunset('expectnum',$expectnum);
   		$this -> yunset('resume',$resume);
   		$userInfo   = $this->MODEL('userinfo');
   		$userStatue = $userInfo -> getUserInfo(array('uid'=>$this->uid),array('usertype'=>2));
        $this->yunset('userStatue',$userStatue);

        //未查看面试邀请
        $JobM    =	$this->MODEL('job');
        $msgnum  =	$JobM->getYqmsNum(array('uid'=>$this->uid,'is_browse'=>1,'isdel'=>9));
        $this -> yunset("msgnum", $msgnum);

        $member =   $userInfo->getInfo(array('uid' => $this->uid), array('setname' => '1'));
        if ($member['subscribe'] != 1 && !empty($member['wxid'])){
            $wxM    =   $this->MODEL('weixin');
            $wxUser =   $wxM->getWxUser($member['wxid']);
            $member['subscribe']    =   $wxUser['subscribe'];
            $this->obj->update_once('member', array('subscribe' => $wxUser['subscribe']), array('uid' => $this->uid));
        }
        $this->yunset("member", $member);
        
		if($_GET['c']==''){
		    // 首页
		    if($member['wxid']=='' && $member['unionid']=='' && $this->config['wx_popWin']=='1'){
		        
		        $this -> yunset("wxbindshow",1);
		        $this -> cookie -> SetCookie("wxbd",'1',time() + 86400);
		    }
			$this->yunset('left',1);
      
		}elseif(in_array($_GET['c'], array('expect','resume','import','look','privacy','resumeout','show','likejob','resumetpl'))){
			
     		$this->yunset('left',2);
			
		}elseif(in_array($_GET['c'], array('commsg','subscribe','finder','favorite','look_job','job','invite','atn','partapply','partcollect','rebates','comment'))){
			
			$this->yunset('left',3);
      
		}elseif(in_array($_GET['c'], array('paylist','paylog','integral','integral_reduce','reward_list','pay','payment'))||$_GET['m']=='invitereg' || in_array($_GET['act'], array('loglist','withdrawlist','withdraw','change','changelist'))){
			
		    if ($_GET['act'] == 'withdraw' && empty($member['wxid'])){
                $this -> ACT_msg("index.php?c=binding","请先绑定微信！");
		    }
		    
			$this->yunset('left',5);
      
		}elseif(in_array($_GET['c'], array('uppic','info','setname','passwd','binding','sysnews','transfer'))){ 
			
			$this->yunset('left',6);
		}
		return $resume;
	}
	//会员统计信息调用
	function member_satic(){
    
		$StatisM                =	$this->MODEL('statis');
		$JobM                   =	$this->MODEL('job');
		
		$where['uid']           =	$this->uid;
		$statis                 =	$StatisM->getInfo($this->uid,array('usertype'=>'1'));
		$sqwhere['uid'] 		= 	$this->uid;
		$sqwhere['isdel'] 		= 	9;
		$sq_nums				=	$JobM->getSqJobNum($sqwhere);
		$statis['sq_jobnum']    =	$sq_nums;

		$fav_jobnum             =	$JobM->getFavJobNum($where);
		$statis['fav_jobnum']	=	$fav_jobnum;
		
		$this->yunset('statis',$statis);
		
		return $statis;
	}
	function user_tpl($tpl){
		$this->yuntpl(array('member/user/'.$tpl));
	}

	//退出
	function logout_action(){
		$this->logout();
	}
}
?>