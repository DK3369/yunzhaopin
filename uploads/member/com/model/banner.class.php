<?php

class banner_controller extends company{
	/*
	查看横幅页面
	**/
	function index_action(){
		$where['uid']	=	$this->uid;
		
		$banner		=	$this -> MODEL('company') -> getBannerInfo('',array('where'=>$where));
		$syspic  	=   checkpic($this->config['sy_banner']);
		$this->yunset("banner",$banner); 
		$this->yunset("syspic",$syspic); 
		$this->public_action();
		$this->com_tpl("banner");
		
	}
	/*
	上传横幅
	**/
	function save_action(){
		
		$companyM		=	$this -> MODEL('company');
		
		$data			=	array(

			'file'		=>	$_FILES['file'],

			'uid'		=>	$this->uid,

			'usertype'	=>	$this->usertype

		);
		
		if($_POST['save']){

			$data['type']='add';

		}

		if($_POST['update']){

			$data['type']='update';

		}

		$return			 =	$companyM	->	setBanner($data);

		$this			->	ACT_layer_msg($return['msg'],$return['errcode'],$return['url']);
		
	 
		
	}
}
?>