<?php

class passwd_controller extends user{
	//账号安全
	function index_action(){
		
		$UserinfoM	=	$this -> MODEL('userinfo');
		
		//修改密码
		if($_POST['submit']){
			 
			 $data   =   array(
               
			   'uid'          	 =>  intval($this->uid),
               
			   'usertype'     	 =>  $this->usertype,
                
				'oldpassword'  	 =>  trim($_POST['oldpassword']),
   
                'password'     	 =>  trim($_POST['password']),
                
				'repassword'   	 =>  trim($_POST['repassword'])
                
            );
			
			$info	=	$UserinfoM -> getInfo(array('uid'=> $this->uid));
			
			if (is_array($info)) {
				
				if($this->config['sy_uc_type']=="uc_center" &&$info['name_repeat']!="1"){
					
					$this->obj->uc_open();
					
					$ucresult	=	uc_user_edit($info['username'], $_POST['oldpassword'], $_POST['newpassword'], "","1");
					
					if($ucresult == -1){
						
						$this->ACT_layer_msg('member_user_00607', 8,"index.php?c=passwd");
					
					}elseif ($ucresult == 1){
						$err	=	$UserinfoM -> savePassword($data);
					}elseif ($ucresult == 0||$ucresult == -7){
						$err	=	array('msg'=>yun_at('member_com_00383'),'errcode'=>8);
					}elseif ($ucresult == -4){
						$err	=	array('msg'=>yun_at('member_com_00384'),'errcode'=>8);
					}elseif ($ucresult == -5){
						$err	=	array('msg'=>yun_at('member_com_00385'),'errcode'=>8);
					}elseif ($ucresult == -6){
						$err	=	array('msg'=>yun_at('member_com_00386'),'errcode'=>8);
					}elseif ($ucresult == -8){
						$err	=	array('msg'=>yun_at('member_com_00387'),'errcode'=>8);
					}
				
				}
					
				$err    =   $UserinfoM -> savePassword($data);
				
				
				
				
				
				if($err['errcode'] == '8'){ 
                    
					$this -> ACT_layer_msg($err['msg'], $err['errcode'], "index.php?c=passwd");
                
				}else{
                   $this -> cookie -> unset_cookie();
				   $this -> ACT_layer_msg($err['msg'], $err['errcode'], $this->config['sy_weburl'] . "/index.php?m=login");
                
				}
			
			}
		
		}
		
		//修改用户名
		if($_POST['submit2']){
			
			$data	=	array(
				
				'username'	=>  trim($_POST['username']),
				
				'password'	=>  trim($_POST['password']),
				
				'uid'		=>  intval($this->uid),
				
				'usertype'	=>  intval($this->usertype),
				
				'restname'	=>  '1'
            
			);
			if (!empty($data['username'])) {
				
				$err	=	$UserinfoM  -> saveUserName($data);
				
				if($err['errcode'] == '1'){
					
					$this->ACT_layer_msg('common_06062', 9 ,$this->config['sy_weburl']."/index.php?m=login");
					
				
				}else{
					
					$msg = $err['msg'] ? $err['msg'] : 'member_user_00603';

					$this->ACT_layer_msg($msg, 8 ,$_SERVER['HTTP_REFERER']);
				
				}
			
			}
		
		}
		
		$this->public_action();
		
		$this->user_tpl('passwd');
	
	}

}
?>