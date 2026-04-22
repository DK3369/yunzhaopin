<?php

class advice_controller extends common{
    /**
     * 意见反馈
     */
	function index_action(){
		if($this->uid){
			$userinfoM 	= $this->MODEL('userinfo');
			$meminfo 	= $userinfoM->getUserInfo(array('uid'=>$this->uid),array('usertype'=>$this->usertype));
			$username = $mobile = '';
			if($this->usertype==1){
				$username = $meminfo['name'];
				$mobile = $meminfo['telphone'];
			}else if($this->usertype==2){
				$username = $meminfo['name'];
				$mobile = $meminfo['linktel'];
			}
			$this->yunset('advice_name',$username);
			$this->yunset('advice_mobile',$mobile);
		}
		$this->seo('advice');
		$this->yunset('headertitle','意见反馈');
		$this->yuntpl(array('wap/advice'));
	}

    /**
     * 意见反馈-提交
     */
	function saveadd_action(){
		$data		=	array(
			'username'	=>	$_POST['username'],
			'infotype'	=>	$_POST['infotype'],
			'content'	=>	$_POST['content'],
			'mobile'	=>	$_POST['moblie'],
			'authcode'	=>	$_POST['authcode'],
			'advice_code'=>	$_POST['advice_code'],
			'utype'		=>	'wap'
		);
		$adviceM	=	$this->MODEL('advice');
		$return		=	$adviceM->addInfo($data);
		
		echo json_encode($return);die;
	}
    /**
     * 意见反馈-发送动态码
     */
	function sendmsg_action(){
		$noticeM	=	$this->MODEL('notice');
		$result		=	$noticeM->jycheck($_POST['authcode'],'意见反馈');
		if(!empty($result)){
			$this->layer_msg($result['msg'], 9, 0, '', 2, $result['error']);
		}
		$moblie		=	$_POST['moblie'];
		
		$result	=	$noticeM->sendCode($moblie, 'code', 1,array(), 6, 90, 'msg');
		echo json_encode($result);exit();
	}
}
?>