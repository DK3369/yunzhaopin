<?php

class index_controller extends common{
	function index_action(){
		$this->seo("forgetpw");
		$this->yun_tpl(array('index'));
	}
	function half_replace($str,$encoding='utf-8'){
        
		$strlen     	= 	mb_strlen($str, 'utf-8');
		$firstStr     	= 	mb_substr($str, 0, 1, 'utf-8');
		$lastStr    	= 	mb_substr($str, -1, 1, 'utf-8');
		return $strlen 	== 	2 ? $firstStr . str_repeat('*', mb_strlen($str, 'utf-8') - 1) : $firstStr . str_repeat("*", $strlen - 2) . $lastStr;
        
	}
	function editpw_action(){
        $username	=	$_POST['username'];
        $uid		=	$_POST['uid'];
		$mobile		=	$_POST['mobile'];
		$email		=	$_POST['email'];
		$code		=	$_POST['code'];

        if($username!='' && $uid!=''){
			$userinfoM		=	$this->MODEL("userinfo");
			$companyM		=	$this->MODEL("company");
			$noticeM		=	$this->MODEL('notice');
			$pwmsg 	   		=   regPassWordComplex($_POST['password']);
			if($email!=''){
				$info 		= 	$userinfoM->getInfo(array('email'=>$email),array("field"=>"`uid`,`username`,`email`"));
				$check		=	$info['email'];
			}elseif($mobile!=''){
				$info 		= 	$userinfoM->getInfo(array('moblie'=>$mobile),array("field"=>"`uid`,`username`,`moblie`"));
				$check		=	$info['moblie'];
			}
			$cert 			= 	$companyM->getCertInfo(array("uid"=>$info['uid'],"type"=>"7","check"=>$check,'orderby'=>'ctime,desc'),array("field"=>"`uid`,`check2`,`ctime`,`id`"));
			
			$codeTime       =   $noticeM -> checkTime($cert['ctime']);
			
			if($uid != $cert['uid']){
				$res['msg']		=	yun_at('wap_00203');
			    $res['error']	=	'8';
			    echo yun_json_encode($res);die;
			
			}elseif (!$codeTime) {
			    
			    $res['msg']		=	yun_at('wap_00201');
			    $res['error']	=	'8';
			    echo yun_json_encode($res);die;
			    
			}else if(($code!=$cert['check2'])||(!$cert)){
				
				$res['msg']		=	yun_at('wap_00211');
				$res['error']	=	'8';
				echo yun_json_encode($res);die;
				
			}else if($pwmsg!=''){
				$res['msg']		=	$pwmsg;
				$res['error']	=	'8';
				echo yun_json_encode($res);die;
			}else{

				$info 		= 	$userinfoM->getInfo(array('uid'=>$uid),array("field"=>"`uid`,`username`,`email`,`moblie`,`name_repeat`"));
				
				if ($username==$info['username']){
					
					$password 		= 	$_POST['password'];
					
					if($this->config[sy_uc_type]=="uc_center" && $info['name_repeat']!="1"){
						
					    $this->obj->uc_open();
						uc_user_edit($info[username], "", $password, $info['email'],"0");
						
					}
					$userinfoM->upInfo(array("uid"=>$uid),array("password"=>$password));
					
					$res['msg']		=	yun_at('wap_01786');
					$res['error']	=	0;
					echo yun_json_encode($res);die;
				}else{
					$res['msg']		=	yun_at('wap_00209');
				}
			}
        }else{
            $res['msg']				=	yun_at('wap_00204');
        }
        echo yun_json_encode($res);die;
    }

    function sendCode_action()
    {
        $sendtype   =   $_POST['sendtype'];
        $noticeM    =   $this->MODEL('notice');
        $result     =   $noticeM->jycheck($_POST['code'], 'wap_js_00123');
        if (!empty($result)) {
            echo yun_json_encode(array('msg' => $result['msg'], 'error' => $result['error']));
            return;
        }
        if ($sendtype == 'mobile') {

            $sended =   $_POST['mobile'];
            $type   =   'msg';
        } elseif ($sendtype == 'email') {

            $sended =   $_POST['email'];
            $type   =   'email';
        }

        $result     =   $noticeM->sendCode($sended, 'getpass', 1, array(), 6, 120, $type);
        echo yun_json_encode($result);
        exit();
    }
	
	function checksendcode_action(){
		$moblie		=	$_POST['mobile'];
		$email		=	$_POST['email'];
		
		$userinfoM	=	$this->MODEL("userinfo");
		$companyM	=	$this->MODEL("company");
		$noticeM	=	$this->MODEL("notice");
		
		if($_POST['sendtype']=='email'){
			
		    $info 	= 	$userinfoM->getInfo(array('email'=>$email),array("field"=>"`uid`,`username`,`email`"));
			$check	=	$info['email'];
			
		}elseif($_POST['sendtype']=='mobile'){
			
		    $info 	= $userinfoM->getInfo(array('moblie'=>$moblie),array("field"=>"`uid`,`username`,`moblie`"));
			$check	=	$info['moblie'];
		}
		$cert		= 	$companyM->getCertInfo(array("uid"=>$info['uid'],"type"=>"7","check"=>$check,'orderby'=>'ctime,desc'),array("field"=>"`uid`,`check2`,`ctime`,`id`"));
		$codeTime   =   $noticeM -> checkTime($cert['ctime']);
		
		if (!$codeTime) {
		    
		    $res['msg']		=	yun_at('wap_00201');
		    $res['error']	=	'8';
		    echo yun_json_encode($res);die;
		    
		}else  if(($_POST['code']!=$cert['check2'])||(!$cert)){
		    $res['msg']		=	yun_at('wap_00211');
		    $res['error']	=	'8';
		    echo yun_json_encode($res);die;
		}
		$res['msg']			=	yun_at('wap_01784');
		$res['error']		=	0;
		$res['uid']			=	$info['uid'];
		$res['username']	=	$info['username'];
		echo yun_json_encode($res);die;
	}
	
	function checklink_action(){
	    $_POST		=	$this->post_trim($_POST);
	    $username	=	$_POST['username'];
		$userinfoM	=	$this->MODEL("userinfo");

		if(CheckRegUser($username)==false && CheckRegEmail($username)==false){
	        $data['msg']	=	yun_at('wap_00202');
	        $data['error']	=	'8';
	        echo yun_json_encode($data);die;
	    }

		$member 	= 	$userinfoM->getInfo(array('username'=>$_POST['username']), array('field' => '`uid`, `username`, `pid`'));
		
	    if(empty($member)){
			$data['msg']	=	yun_at('wap_01785');
			$data['error']	=	'8';
	        echo yun_json_encode($data);die;
		}

	    $shensu		=	$_POST['linkman'].'-'.$_POST['linkphone'].'-'.$_POST['linkemail'];
		
	    $nid 		= 	$userinfoM->upInfo(array('uid' => $member['uid']), array('appeal'=>$shensu,'appealtime'=>time(),'appealstate'=>'1'));
		
	    if($nid){
	        $data['error']	=	'0';
	        echo yun_json_encode($data);die;
	    }
	}
	
}
?>