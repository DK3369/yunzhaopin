<?php

class index_controller extends common{
	function index_action(){
		$msgM	=	$this->MODEL('email');
		$notice = 	$this->MODEL('notice');
		
		if($this->uid==""){
			
			$this->ACT_msg($this->config['sy_weburl'], yun_at('wap_js_00154'));
		
		}
		if($_POST['submit']){
			
			if($this->config['sy_reg_invite']>0){
				
				$where['uid']	=	$this->uid;
				$where['ctime']	=	array('>',strtotime(date('Y-m-d')));
				$where['title']	=	array('like%','invitereg_00009');
				
				$inviteregNum 	= 	$msgM->getEmsgNum($where);
				
				if($inviteregNum >= $this->config['sy_reg_invite']){
					$this->ACT_layer_msg('invitereg_00002',8,$_SERVER['HTTP_REFERER']);
					exit();
				}
			}
			
			$emailData['uid']	=	$this->uid;
			
			$_POST['content']	=	'invitereg_00008'.$this->config['sy_webname'].'invitereg_00001'.Url('register',array('uid'=>$this->uid));

			session_start();
			
			$authcode			=	md5(strtolower($_POST['authcode']));
			
			unset($_POST['authcode']);
			
			$_POST['email']		=	trim($_POST['email']);
			
			if($this->config['sy_email_set']!="1"){
				
				$this->ACT_layer_msg('invitereg_00003',8,$_SERVER['HTTP_REFERER']);
			}
			
			if($_POST['email']==""){
				
				$this->ACT_layer_msg('model_00077',8,$_SERVER['HTTP_REFERER']);
			} 
			if(CheckRegEmail($_POST['email'])==false){
				
				$this->ACT_layer_msg('model_00078',8,$_SERVER['HTTP_REFERER']);
			}
			if($_POST['content']==""){
				
				$this->ACT_layer_msg('member_com_00391',8,$_SERVER['HTTP_REFERER']);
			}
			if($authcode!=$_SESSION['authcode'] || empty($_SESSION['authcode'])){
				
				unset($_SESSION['authcode']);
				
				$this->ACT_layer_msg($_POST['authcode'].'model_00047'.$_SESSION['authcode'],8);
			} 
			
			//发送邮件并记录入库
			$emailData['email'] 	=	$_POST['email'];
			$emailData['subject']	= 	'invitereg_00009'.$this->config['sy_webname'];
			$emailData['content']	= 	$_POST['content'];
			
			$sendid					= 	$notice->sendEmail($emailData);

			if($sendid['status'] != -1){
				
				$this->ACT_layer_msg('invitereg_00006',9,$_SERVER['HTTP_REFERER']);
			}else{
				
				$this->ACT_layer_msg('model_00079',8,$_SERVER['HTTP_REFERER']);
			}
		}

		if($this->config['reg_moblie']){
			
			$type	=	2;
		}elseif($this->config['reg_email']){
			
			$type	=	3;
		}else{
			
			$type	=	1;
		}
		
		$reg_url	=	Url('register', array('uid'=>$this->uid), '1');
		
		$this->seo("invitereg");
		$this->yunset('reg_url', $reg_url);
		$this->yun_tpl(array('index'));
	}
}
?>