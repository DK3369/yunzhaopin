<?php

class setname_controller extends user{
    //修改账户
	function index_action(){
		
		$UserinfoM  =   $this -> MODEL('userinfo');
	
		$data       =   array(
		  
			'username'      =>  trim($_POST['username']),
			'password'      =>  trim($_POST['password']),
			'uid'           =>  intval($this->uid),
			'usertype'      =>  intval($this->usertype),
			'restname'      =>  '1'
			
		);
		if (!empty($data['username'])) {
			
			$err	=   $UserinfoM  -> saveUserName($data);
			
			if($err['errcode'] != '1'){
				
				echo $err['msg'];die();
			}else{
				
				echo 1;die();
			}
		}
		
		$this->public_action();
		$this->user_tpl('setname');
	}
}
?>