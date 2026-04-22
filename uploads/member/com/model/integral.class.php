<?php

class integral_controller extends company{
	//如何获取积分
	function index_action(){
		$integralM	=	$this->MODEL('integral');
		$statusList	=	$integralM	->	integralMission(array('type'=>'com','uid'=>$this->uid,'usertype'=>$this->usertype));
		$this	->	yunset("statusList",$statusList);
        $this	->	public_action();
        $this	->	company_satic();
		$this	->	com_tpl('integral');
	}
	
}
?>