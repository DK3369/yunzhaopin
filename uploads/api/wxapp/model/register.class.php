<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class register_controller extends wxapp_controller
{

    function index_action()
    {

		if (!empty($_POST['uid']) && !empty($_POST['token'])){
		    
			$member     =  $this->yzToken($_POST['uid'], $_POST['token']);
			if(!empty($member['uid'])){
				$error	=   2;
				$msg	=	'您已登录！';
                $this->render_json($error, $msg, '');
			}
		}
		if (isset($_POST['version']) && $_POST['version']=='6.1'){
		    // 6.1版开始对注册、登录传的相关参数进行加密，这里需要解密
		    $username   = $this->decryptRequest($_POST['username']);
		    $password   = $this->decryptRequest($_POST['password']);
		    if ($_POST['type'] == 1){
		        // 用户名注册
		        $data['username']       =  $username;
		        $data['moblie']			=  $_POST['moblie'];
		        $data['moblie_code']	=  $_POST['mobliecode'];
		    }elseif ($_POST['type'] == 2){
		        // 手机号注册
		        $data['moblie']         =  $username;
		        $data['moblie_code']	=  $_POST['mobliecode'];
		    }elseif ($_POST['type'] == 3){
		        // 邮箱注册
		        $data['email']          =  $username;
		        $data['moblie']			=  $_POST['moblie'];
		        $data['moblie_code']	=  $_POST['mobliecode'];
		    }
		    $data['password']  =  $password;
		}else{
		    $data['username']     =  $_POST['username'];
		    $data['moblie']		  =	 $_POST['moblie'];
		    $data['moblie_code']  =	 $_POST['mobliecode'];
		    $data['email']		  =	 $_POST['email'];
		    $data['password']	  =	 $_POST['password'];
		}
		
		$data['usertype']		=	$_POST['usertype'];
		$data['uid']			=	$member['uid'];
		$data['codeid']			=	$_POST['type'];
		$data['source']			=	$_POST['source'];
		$data['wxapp']			=	1;
		$data['clientid']       =   $_POST['clientid'];
		$data['deviceToken']    =   $_POST['deviceToken'];
		$data['port']			=	$this->plat == 'mini' ? '3' : '4';
		$data['provider']       =   $_POST['provider'];
        // 目前从wxlogin到注册页面，会携带第三方登录参数。微信小程序和百度小程序，都是直接使用login页面携带的openid。
        if (!empty($_POST['loginType'])){
            if ($_POST['provider'] == 'app'){
                if($_POST['loginType'] == 'qq'){
                    $data['qqid']   =   $_POST['openid'];
                    if (!empty($_POST['unionid']) && $_POST['unionid'] != 'undefinded'){
                        $data['qqunionid']=   $_POST['unionid'];
                    }
                }
            }
        }
            
        if ($this->config['sy_reg_type'] == 2){
            $data['reg_name']   =   $_POST['regName'];
            $data['reg_link']   =   $_POST['regLink'];
            $data['reg_type']   =   $_POST['regType'];
        }

		if (!empty($_POST['did'])){
		    $data['did']        =   $_POST['did'];
		}
		//  小程序邀请注册
        if ((int)$_POST['fromUser'] > 0){

            $data['fromUser']   =   $_POST['fromUser'];
        }
        
        $userinfoM				=	$this->MODEL('userinfo');
		$return 				=	$userinfoM->userRegSave($data);
		if($return['errcode']!=8){
			$error	=	1;
		}else{
			$error	=	2;
		}
		$this -> render_json($error,$return['msg'],$return['user']);
    }

    function checkComName_action()
    {

        $registerM  =   $this->MODEL('register');
        $return     =   $registerM->checkRegFirst(array('c_name' => $_POST['c_name']));

        $this->render_json($return['errcode'], '', array());
    }

    function ident_action()
    {

        if (!empty($_POST['uid']) && !empty($_POST['token'])) {

            $member =   $this->yzToken($_POST['uid'], $_POST['token']);
        } else {

            $error  =   2;
            $msg    =   '参数错误！';
            $this->render_json($error, $msg, '');
        }

        if (empty($_POST['usertype'])) {

            $error  =   2;
            $msg    =   '请选择用户类型！';
        } else {

            $UserInfoM  =   $this->MODEL('userinfo');
            $info       =   $UserInfoM->upUserType(array('uid' => $member['uid'], 'usertype' => $_POST['usertype'], 'wxapp' => 1, 'provider' => $_POST['provider']));

            if ($info['errcode'] == 1) {

                $error  =   1;
                $msg    =   '';
            } else {

                $error  =   2;
                $msg    =   $info['msg'];
            }
        }
        $this-> render_json($error,$msg,$info);
	}
    function regmoblie_action(){
        $data	=	array(
            'moblie'	=>	$_POST['moblie'],
        );
        $registerM	=	$this->MODEL('register');
        $return 	= 	$registerM->regMoblie($data);

        $this -> render_json($return['errcode'],$return['msg'],$return['data']);
    }
    function regemail_action(){
        $data	=	array(
            'email'	=>	$_POST['email'],
        );
        $registerM	=	$this->MODEL('register');
        $return 	= 	$registerM->regMoblie($data);
        $this -> render_json($return['errcode'],$return['msg'],$return['data']);
    }
    function writtenoff_action(){
        $data	=	array(
            'zyuid'		=>	$_POST['zyuid'],
            'pw'		=>	$_POST['pw'],
            'mobile'	=>	$_POST['mobile'],
            'email'		=>	$_POST['email'],
        );
        $registerM	=	$this->MODEL('register');
        $return 	= 	$registerM->writtenOff($data);

        $this -> render_json($return['errcode'],$return['msg'],'');
    }
	function regcheck_action(){
		//验证用户名、邮箱
		$post	=	array(
			'username'	=>	$_POST['username'],
			'email'		=>	$_POST['email'],
			'password'	=>	$_POST['password']
		);
		$data	=	array(
			'post'	=>	$post
		);
		$registerM	=	$this->MODEL("register");
		$return 	= 	$registerM->ajaxReg($data);
		if($return['errcode']==0){
			$errcode	=	1;
		}else{
			$errcode	=	2;
		}
		$this -> render_json($errcode,$return['msg'],'');
	}

	/* 注册发送手机号 */
    function regcode_action()
    {
        $this->regcode($this->plat == 'mini' ? '3' : '4');	// 短信发送端口$port : 3-小程序  4-APP
    }

    /* 注册发送手机号-快速投递 */
    function regcodeks_action()
    {
        $this->regcode($this->plat == 'mini' ? '9' : '10');	// 短信发送端口$port : 9-小程序快速投递,10-APP快速投递
    }

	function regcode($port)
	{
	    $moblie		= 	trim($_POST['moblie']);
	    
	    $this->checkMcsdk($moblie);
	    
	    $arr['regtype'] = 'reg';
		$registerM	=	$this->MODEL("register");
		$noticeM 	= 	$this->MODEL('notice');

	    $data	=	array(
	        'moblie'	=>	$moblie
		);
		
		$return 	= 	$registerM->regMoblie($data);
		
		if($return['errcode']==0){
			$result 	= 	$noticeM->sendCode($moblie, 'regcode', $port);
			if($result['error']==1){
				$errcode	=	1;
			}else{
				$errcode	=	2;
				$msg		=	$result['msg'];
			}
		}else{
		    $errcode  =  2;
			if($return['errcode']==9){
			    if(isset($_POST['type']) && $_POST['type'] == 'wxreg'){
			        $arr['regtype'] = 'wxreg';
			        // 微信绑定里，已存在账号的，可以直接用手机短信绑定
			        if ($return['data']['usertype'] == 1 || $return['data']['usertype'] == 2){
			            $result  =  $noticeM->sendCode($moblie, 'login', $port, $return['data'], 6, 90, 'msg');
			            if($result['error']==1){
			                $errcode	=	1;
			            }else{
			                $errcode	=	2;
			                $msg		=	$result['msg'];
			            }
			        }
			    }else{
			        $msg    =   '该手机号已被使用';
			    }
			}else{
				$msg		=	'该手机号已被禁止使用';
			}
		}
		$this -> render_json($errcode,$msg,$arr);
	}
	
	function getCon_action(){
		if($this->config['reg_moblie']){				
			$type = '2';
		}else if($this->config['reg_email']){
			$type = '3';
		}else{
			$type = '1';
		}
		$data	=	array(
			'type'					=>	$type,
			'reg_email'				=>	$this->config['reg_email'],
			'reg_moblie'			=>	$this->config['reg_moblie'],
			'reg_user'				=>	$this->config['reg_user'],
			'reg_passconfirm'		=>	$this->config['reg_passconfirm'],
			'code_kind'				=>	$this->config['code_kind'],
			'reg_real_name_check'	=>	$this->config['reg_real_name_check'],
			'reg_passconfirm'		=>	$this->config['reg_passconfirm'],
			
			'reg_namemaxlen' 		=>  $this->config['sy_reg_namemaxlen'],
            'reg_nameminlen'		=>  $this->config['sy_reg_nameminlen'],
       		'reg_name_sp'    		=>  $this->config['reg_name_sp'],
            'reg_name_zm'    		=>  $this->config['reg_name_zm'],
            'reg_name_num'   		=>  $this->config['reg_name_num'],
            'reg_name_han'   		=>  $this->config['reg_name_han'],

            'reg_pw_sp'    			=>  $this->config['reg_pw_sp'],
            'reg_pw_zm'    			=>  $this->config['reg_pw_zm'],
            'reg_pw_num'   			=>  $this->config['reg_pw_num'],
            'sy_freewebtel'         =>  $this->config['sy_freewebtel'],
		    'sy_reg_type'           =>  isset($this->config['sy_reg_type']) ? $this->config['sy_reg_type'] : 1
		);
		$this -> render_json(1,'',$data);
	}
}
?>