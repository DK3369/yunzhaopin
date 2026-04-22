<?php

class transfer_controller extends user{
	//基本信息
	function index_action(){
		
		$this->public_action();
		$this->user_tpl('transfer');
	}
	
	function save_action(){
		
		
		if($_POST['submit']){
			
			$transferM	=	$this -> MODEL('transfer');

			$return		=	$transferM -> setTransfer($this->uid,$_POST);
			
			if($return['errcode'] == '1'){
				$this->cookie->unset_cookie();
				$this->ACT_layer_msg('账户分离成功，请使用新账户登录！',9,Url('login'));

			}else{
				$this->ACT_layer_msg($return['msg'],8);
			}

		}
		
	}
	
}
?>