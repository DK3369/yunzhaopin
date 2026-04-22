<?php

class index_controller extends common{
	function index_action(){

		if($_POST['submit']){
		
			$linkM		=	$this  -> MODEL('link');
			if($_POST['phototype']==1){
				
				if($_FILES['file']['tmp_name']){

				    $UploadM = $this->MODEL('upload');

	                $upArr    =  array(
	                  'file'  =>  $_FILES['file'],
	                  'dir'   =>  'link'
	                );

	                $uploadM  =  $this->MODEL('upload');

	                $pics      =  $uploadM->newUpload($upArr);
	              
	                if (!empty($pics['msg'])){

	                  $this->ACT_layer_msg($pics['msg'],8);

	                }elseif (!empty($pics['picurl'])){

	                  $pic   =   $pics['picurl'];

	                }
	            }
				
			}else{
				$pic		=	$_POST['uplocadpic'];
			}
				
			$post	=	array(
				'did'			=>	$this->userdid ? $this->userdid : 0,
				'link_name'		=>	trim($_POST['title']),
				'link_url'		=>	$_POST['url'],
				'link_type'		=>	$_POST['type'],
				'tem_type'		=>	1,
				'img_type'		=>	$_POST['phototype'],
				'link_sorting'	=>	$_POST['sorting'],
				'link_state'	=>	0,
				'pic'			=>	$pic,
			);
			$data	=	array(
				'post'		=>	$post,
				'id'		=>	$_POST['id'],
				'authcode'	=>	$_POST['authcode'],
				'utype'		=>	'index'
			);
			
			$return	=	$linkM -> addInfo($data);
			
			$this->ACT_layer_msg($return['msg'],$return['errcode'],$_SERVER['HTTP_REFERER']);
		}
		$this->seo("friend");
		$this->yun_tpl(array('index'));
	}
}
?>