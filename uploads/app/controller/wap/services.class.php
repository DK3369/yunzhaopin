<?php

class services_controller extends common{
	function index_action(){
		$this->get_moblie();
		$M=$this->MODEL('article');
		$row=$M->GetDescriptionOnce(array('id'=>'5'),array('field'=>'content'));
		$this->yunset("row",$row);
		$this->yunset("headertitle","服务协议"); 
		$this->yuntpl(array('wap/services'));
	}	
}
?>