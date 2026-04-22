<?php

class integral_reduce_controller extends company{
	//如何获取积分
	function index_action(){
        $this->public_action();
		$this->com_tpl('integral_reduce');
	}
}
?>