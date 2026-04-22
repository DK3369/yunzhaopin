<?php

class integral_reduce_controller extends user{
	//积分消费规则
	function index_action(){
		$this->public_action();
		$this->user_tpl('integral_reduce');
	}
}
?>