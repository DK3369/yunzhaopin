<?php

class likejob_controller extends user{
	//匹配职位
	function index_action(){
		$this->public_action();
    $resumeM	=	$this->MODEL('resume');
		$data	=	array(
			'id'=>(int)$_GET['id'],
			'uid'=>$this->uid,
		);
	
		$list		=	$resumeM->likeJob($data);

		$this->yunset("job",$list);
		$this->user_tpl('likejob');
	}
}
?>