<?php

class sendresume_controller extends resume_controller{
	/**
	 * 发送简历
	 * 2019-06-15
	 */
	function index_action(){
		$id					=	intval($_GET['id']);
		if(!empty($id)){
			$resumeM		=	$this -> MODEL('resume');
			$user			=	$resumeM -> getInfoByEid(array(
				'eid' 		=>	$id,
				'uid' 		=>	$this -> uid,
				'usertype' 	=>	$this -> usertype
			));

			$JobM			=	$this -> MODEL('job');
			
			$time			=	strtotime("-14 day");
			$allnum			=	$JobM -> getYqmsNum(array(
				'uid' 		=>	$user['uid'],
				'eid' 		=>	$id,
				'datetime' 	=>	array('>', $time)
			));
			$replynum		=	$JobM -> getYqmsNum(array(
				'uid' 		=>	$user['uid'],
				'eid' 		=>	$id,
				'datetime' 	=>	array('>', $time),
				'is_browse' =>	array('>', 1)
			));

			$pre			=	round(($replynum/$allnum)*100);
			
			$this -> yunset('pre', $pre);
			$this -> yunset('Info', $user);
			
			$resumeCheck	=	$this->config['resume_open_check'] == '1' ? '1' : '2';
			$this->yunset('resumeCkeck',  $resumeCheck);
		}

		$this->yuntpl(array('resume/sendresume'));
    }
}
?>