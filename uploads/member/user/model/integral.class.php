<?php

class integral_controller extends user{
	//获取积分规则
	function index_action(){
		$integralM	=	$this->MODEL('integral');
		$resumeM	=	$this->MODEL('resume');
		
		$statusList	=	$integralM	->	integralMission(array('type'=>'member','uid'=>$this->uid,'usertype'=>$this->usertype));
		
		$expectnum	=	$resumeM->getExpectNum(array('uid'=>$this->uid));
		
		$this->public_action();
		$this->yunset("expectnum",$expectnum);
		$this->yunset("statusList",$statusList);
		$this->user_tpl('integral');
	}
	
}
?>