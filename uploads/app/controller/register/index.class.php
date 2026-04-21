<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class index_controller extends common
{

    function index_action()
    {

        if ($this->config['reg_user_stop'] != 1) {//关闭会员注册

            $this->yun_tpl(array('stopreg'));
        } else {

            if ($this->uid) {

                $this->ACT_msg($this->config['sy_weburl'], "您已经登录了！");
            }
            $type   =   $_GET['type'];
            if ($type) {
                switch ($type) {
                    case 1:
                        if ($this->config['reg_user'] != 1) {
                            $this->ACT_msg("index.php", "用户名注册已关闭！");
                        }
                        break;
                    case 2:
                        if ($this->config['reg_moblie'] != 1) {
                            $this->ACT_msg("index.php", "手机号注册已关闭！");
                        }
                        break;
                    case 3:
                        if ($this->config['reg_email'] != 1) {
                            $this->ACT_msg("index.php", "email注册已关闭！");
                        }
                    default:
                        break;
                }
            } else {
                if ($this->config['reg_moblie']) {

                    $type = '2';
                } else if ($this->config['reg_email']) {

                    $type = '3';
                } else {

                    $type = '1';
                }
            }

            $this->yunset('type', $type);

            if ((int)$_GET['uid']) {//邀请注册生成

                $this->cookie->setcookie('regcode', (int)$_GET['uid'], time() + 3600);
            }
            // 判断注册来源，第三方登录过来的，获取相应参数
            if (!empty($_GET['bind'])){
                $this->yunset('bind', $_GET['bind']);
            }
            if ($this->config['sy_reg_type'] == 2){

                if ($_GET['step'] == 2){

                    $this->yunset('usertype', $_GET['usertype']);
                    $this->seo("register");
                    $this->yun_tpl(array('reg_create'));
                }else {

                    $this->seo("register");
                    $this->yun_tpl(array('reg_new'));
                }
            }else{

                $this->seo("register");
                $this->yun_tpl(array('index'));
            }
        }
    }

    function checkComName_action()
    {

        $registerM  =   $this->MODEL('register');
        $return     =   $registerM->checkRegFirst(array('c_name' => $_POST['c_name']));
        echo json_encode($return);
    }

    function ok_action()
    {
        if ($this->uid) {
            header("location:" . $this->config['sy_weburl'] . '/member');
        }
        if ((int)$_GET['type'] == 1) {

            $title = $this->config['sy_webname'] . "- 注册成功";
        } elseif ((int)$_GET['type'] == 2) {

            $title = $this->config['sy_webname'] . "- 帐号被锁定";
        } elseif ((int)$_GET['type'] == 3) {

            $title = $this->config['sy_webname'] . "- 审核未通过";
        } elseif ((int)$_GET['type'] == 4) {

            $title = $this->config['sy_webname'] . "- 邮件认证成功";
        } elseif ((int)$_GET['type'] == 5) {

            $title = $this->config['sy_webname'] . "- 未审核";
        } elseif ((int)$_GET['type'] == 6) {

            $title = $this->config['sy_webname'] . "- 订阅";
        } else {

            header("location:" . $this->config['sy_weburl']);
        }
        if (isset($title)) {

            $this->yunset('title', $title);
        }

        $this->seo('register');
        $this->yun_tpl(array('ok'));
    }

    function ajaxreg_action()
    {
        $post   =   array();

        if (!empty($_POST['username'])) {
            $post['username'] = $_POST['username'];
        }
        if (!empty($_POST['email'])) {
            $post['email'] = $_POST['email'];
        }
        if (!empty($_POST['realname'])) {
            $post['realname'] = $_POST['realname'];
        }
        if (!empty($_POST['password'])) {
            $post['password'] = $_POST['password'];
        }
        $data       =   array('post' => $post);
        $registerM  =   $this->MODEL("register");
        $return     =   $registerM->ajaxReg($data);

        echo json_encode($return);
    }

	
	function regmoblie_action(){
		$data	=	array(
			'moblie'	=>	$_POST['moblie'],
		);
		$registerM	=	$this->MODEL("register");
		$return 	= 	$registerM->regMoblie($data);
		if($return['errcode']==9){
			echo json_encode($return['data']);
		}else{
			echo $return['errcode'];
		}
	}

    function regemail_action()
    {
        $data   =   array(
            'email' => $_POST['email'],
        );

        $registerM  =   $this->MODEL("register");
        $return     =   $registerM->regMoblie($data);

        if ($return['errcode'] == 9) {

            echo json_encode($return['data']);
        } else {

            echo $return['errcode'];
        }
    }
	
	
	function writtenOff_action(){
		
		$data	=	array(
			'code'		=>	$_POST['code'],
			'zyuid'		=>	$_POST['zyuid'],
			'pw'		=>	$_POST['pw'],
			'mobile'	=>	$_POST['mobile'],
			'email'		=>	$_POST['email'],
			
		);
		$registerM	=	$this->MODEL("register");
		$return 	= 	$registerM->writtenOff($data);
		
		echo $return['errcode'];
	}

	function regsave_action()
    {

		$Member	=	$this->MODEL('userinfo');
			
		$data['uid']			=	$this->uid;
		$data['username']		=	$_POST['username'];
		$data['codeid']			=	$_POST['codeid'];
		$data['moblie']			=	$_POST['moblie'];
		$data['moblie_code']	=	$_POST['moblie_code'];
		$data['code']			=	$_POST['authcode'];
		$data['name']			=	$_POST['name'];
		$data['email']			=	$_POST['email'];
		$data['password']		=	$_POST['password'];
		$data['source']			=	1;
		$data['did']			=	$this->config['did'];
		$data['port']			=	1;
		// 判断注册来源，第三方登录过来的，获取相应参数
		if (!empty($_POST['reg_bind'])){
		    
		    if ($_POST['reg_bind'] == '2'){
		        
		        session_start();
		        
		        if ($_SESSION['qq']['openid']) {
		            
		            $data['reg_qq'] = array(
		                
		                'openid'    => $_SESSION['qq']['openid'],
		                'unionid'   => $_SESSION['qq']['unionid'],
		            );
		        }
		    }elseif ($_POST['reg_bind'] == '1'){
		        if ($_COOKIE['wxloginid']) {
		            
		            $weixinM    =   $this->MODEL('weixin');
		            $qrcode     =   $weixinM->getWxQrcode(array('wxloginid' => $_COOKIE['wxloginid'], 'status' => 1));
		            
		            $data['reg_weixin'] =   array(
		                
		                'openid'    =>  $qrcode['wxid'],
		                'unionid'   =>  $qrcode['unionid']
		            );
		        }
		    }elseif ($_POST['reg_bind'] == '3'){
		        
		        session_start();
		        
		        if ($_SESSION['sina']['openid']) {
		            
		            $data['reg_sina']   =   array(
		                'openid' => $_SESSION['sina']['openid']
		            );
		        }
		    }
		}
		if ($this->config['sy_reg_type'] == 2){

		    $data['reg_name']   =   $_POST['reg_name'];
		    $data['reg_link']   =   $_POST['reg_link'];
		    $data['reg_type']   =   $_POST['reg_type'];
        }

		$return	=	$Member->userRegSave($data);
		if($return['errcode']!=8){
		    if ($return['reg_type'] == 2){

                $arr['status']  =   1;
                $arr['msg']     =   $return['msg'];
                $arr['url']     =   $return['url'];
            }else {

                $arr['status']  =   $return['errcode'];
                $arr['msg']     =   $return['msg'];
                $arr['url']     =   Url('register', array('c' => 'ident'));
            }
		}else{
			$arr['msg']		=	$return['msg'];
			$arr['status']	=	8;
		}

		echo json_encode($arr);die;
	}


    function ident_action()
    {

        if (!$this->uid) {

            header('Location: ' . Url('register'));
            exit();
        } elseif ($this->usertype) {

            header('Location: ' . Url('member'));
            exit();
        }

        if ($_GET['usertype']) {

            $Member =   $this->MODEL('userinfo');
            $info   =   $Member->upUserType(array('uid' => $this->uid, 'usertype' => $_GET['usertype']));

            if ($info['errcode'] == 1) {

                header('Location: ' . $info['url']);
                exit();
            } else {

                $this->ACT_msg(Url('register', array('c' => 'ident')), $info['msg']);
            }
        }
        $this->seo("register");
        $this->yun_tpl(array('ident'));
    }
	/**
     *users:王旭
     *Data:2023/2/13
     *Time:16:06
     */
    function jobregsave_action()
    {

        $Member	=	$this->MODEL('userinfo');
        $salt   =  substr(uniqid(rand()),-6);
        $data['uid']			=	$this->uid;
        $data['username']		=	$_POST['moblie'];
        $data['password']		=	$salt;
        $data['moblie_status']	=	1;
        $data['usertype']		=	1;
        $data['moblie']			=	$_POST['moblie'];
        $data['moblie_code']	=	$_POST['moblie_code'];
        $data['name']			=	$_POST['name'];
        $data['source']			=	1;
        $data['did']			=	$this->config['did'];
        $data['port']			=	1;
        $this->config['sy_reg_type'] = 2;
        if ($this->config['sy_reg_type'] == 2){

            $data['reg_name']   =   $_POST['name'];
            $data['reg_link']   =   $_POST['moblie'];
            $data['reg_type']   =   1;
        }

        $return	=	$Member->userRegSave($data);
        if($return['errcode']!=8){
            if ($return['reg_type'] == 2){

                $arr['status']  =   1;
                $arr['msg']     =   $return['msg'];
                $arr['url']     =   $return['url'];
            }else {

                $arr['status']  =   $return['errcode'];
                $arr['msg']     =   $return['msg'];
                $arr['url']     =   Url('register', array('c' => 'ident'));
            }
        }else{
            $arr['msg']		=	$return['msg'];
            $arr['status']	=	8;
        }

        echo json_encode($arr);die;
    }

}
?>