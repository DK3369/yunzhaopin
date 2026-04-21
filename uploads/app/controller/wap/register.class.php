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
class register_controller extends common
{

    function index_action()
    {

        if ($_COOKIE['wxid']) {
            $this->yunset('wxid', $_COOKIE['wxid']);
            $this->yunset('wxnickname', $_COOKIE['wxnickname']);
            $this->yunset('wxpic', $_COOKIE['wxpic']);
        }
        if ($this->config['reg_user_stop'] != 1) {//关闭会员注册

            $this->ACT_msg_wap('index.php', '网站已关闭注册！', 2, 5);

        } else {
            if ($this->uid) {
                $this->ACT_msg_wap('index.php', '您已经登录了！', 2, 5);
            }
            $type = $_GET['type'];
            if ($type) {
                switch ($type) {
                    case 1:
                        if ($this->config['reg_user'] != 1) {
                            $this->ACT_msg_wap('index.php', '用户名注册已关闭！', 2, 5);
                        }
                        break;
                    case 2:
                        if ($this->config['reg_moblie'] != 1) {
                            $this->ACT_msg_wap('index.php', '手机号注册已关闭！', 2, 5);
                        }
                        break;
                    case 3:
                        if ($this->config['reg_email'] != 1) {
                            $this->ACT_msg_wap('index.php', 'email注册已关闭！', 2, 5);
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
        }
        $this->get_moblie();

        $descM = $this->MODEL('description');
        $xieyi = $descM->getDes(array('id' => '5'), array('field' => 'content'));
        $this->yunset('xieyi', $xieyi);

        $yinsi = $descM->getDes(array('name' => array('like', '隐私政策')), array('field' => 'content'));
        $this->yunset('yinsi', $yinsi);

        if ($this->uid != '' && $this->username != '') {
            $this->logout(false);
        }
        if ((int)$_GET['uid']) {//邀请注册生成
            $time = time() + 3600;
            $this->cookie->setcookie('regcode', (int)$_GET['uid'], $time);
        }

        //注册提交
        if ($_POST) {
            session_start();

            $Member = $this->MODEL('userinfo');
            $data['usertype'] = $_POST['usertype'];
            $data['uid'] = $this->uid;
            $data['username'] = $_POST['username'];
            $data['codeid'] = $_POST['regway'];
            $data['moblie'] = $_POST['moblie'];
            $data['moblie_code'] = $_POST['moblie_code'];
            $data['code'] = $_POST['checkcode'];
            $data['email'] = $_POST['email'];
            $data['password'] = $_POST['password'];
            $data['source'] = 2;
            $data['did'] = $this->config['did'];
            $data['port'] = 2;
            $data['qfyuid'] = $_POST['qfyuid'];
            // 判断注册来源，第三方登录过来的，获取相应参数
            if (!empty($_COOKIE['reg_bind'])) {
                
                if ($_COOKIE['reg_bind'] == 1) {
                    
                    if ($_SESSION['wx']['openid']) {
                        $data['reg_weixin'] = array(
                            'openid' => $_SESSION['wx']['openid'],
                            'unionid' => $_SESSION['wx']['unionid'],
                        );
                    }
                } elseif ($_COOKIE['reg_bind'] == 3) {
                    
                    if ($_SESSION['sina']['openid']) {
                        $data['reg_sina'] = array(
                            'openid' => $_SESSION['sina']['openid']
                        );
                    }
                } elseif ($_COOKIE['reg_bind'] == 2) {
                    
                    if ($_SESSION['qq']['openid']) {
                        $data['reg_qq'] = array(
                            'openid' => $_SESSION['qq']['openid'],
                            'unionid' => $_SESSION['qq']['unionid']
                        );
                    }
                }
            }
            
            if ($this->config['sy_reg_type'] == 2){
                $data['reg_name']   =   $_POST['reg_name'];
                $data['reg_link']   =   $_POST['reg_link'];
                $data['reg_type']   =   $_POST['reg_type'];
            }

            $return = $Member->userRegSave($data);

            if ($return['errcode'] != 8) {
                if ($return['reg_type'] == 2){

                    $this->layer_msg($return['msg'], 9, 0, $return['url'], 2);
                }else {

                    $this->layer_msg($return['msg'], 9, 0, Url('wap', array('c' => 'register', 'a' => 'ident')), 2);
                }
            } else {
                $this->layer_msg($return['msg'], 9, 0, '', 2);
            }
        }

        $this->seo('register');
        // 判断注册来源，第三方登录过来的，获取相应参数
        if (!empty($_GET['bind'])){
            $this->cookie->setcookie("reg_bind", $_GET['bind'], time() + 86400);
        }
        
        if ($this->config['sy_reg_type'] == 2){
            if ($_GET['step'] == 2){

                $this->yunset('usertype', $_GET['usertype']);
                $this->yunset('backurl', Url('wap', array('c' => 'register')));
                $this->yunset('headertitle', '创建账户');
                $this->yuntpl(array('wap/reg_create'));
            }else{

                $this->yunset('headertitle', '选择注册身份');
                $this->yuntpl(array('wap/reg_new'));
            }
        }else{

            $this->yunset('headertitle', '选择注册类型');
            $this->yuntpl(array('wap/register'));
        }
    }

    function checkComName_action()
    {

        $registerM  =   $this->MODEL('register');
        $return     =   $registerM->checkRegFirst(array('c_name' => $_POST['c_name']));
        echo json_encode($return);
    }


    function ident_action()
    {

        if (!$this->uid) {

            header('Location: ' . Url('wap', array('c' => 'register')));
            exit();

        } elseif ($this->usertype) {
            $this->wapheader('member/');
            exit();
        }

        if ($_GET['usertype']) {

            $Member = $this->MODEL('userinfo');

            $info = $Member->upUserType(array('uid' => $this->uid, 'usertype' => $_GET['usertype'], 'iswap' => 1));

            if ($info['errcode'] == 1) {

                header('Location: ' . $info['url']);
                exit();

            } else {

                $this->ACT_msg_wap(Url('register', array('c' => 'ident')), $info['msg'], 2, 5);

            }

        }
        $this->seo('register');
        $this->yuntpl(array('wap/ident'));
    }

    function regemail($post)
    {
        if ($post['username']) {
            $username = $post['username'];
        } else {
            if ($post['moblie']) {
                $username = $post['moblie'];
            } else {
                $username = $post['email'];
            }
        }
        $notice = $this->MODEL('notice');
        if ($this->config['sy_email_set'] == '1') {
            $notice->sendEmailType(array('username' => $username, 'password' => $post['password'], 'email' => $post['email'], 'cname' => $username, 'type' => 'reg', 'uid' => $post['uid']));
        }
        if (checkMsgOpen($this->config)) {
            $notice->sendSMSType(array('username' => $username, 'password' => $post['password'], 'moblie' => $post['moblie'], 'type' => 'reg', 'uid' => $post['uid'], 'port' => '2'));
        }
    }

    function regok_action()
    {
        $this->yunset('headertitle', '会员注册');
        $this->seo('register');
        $this->yuntpl(array('wap/registerok'));
    }

    function ajaxreg_action()
    {
        //验证用户名、邮箱
        $post = array(
            'username' => $_POST['username'],
            'email' => $_POST['email'],
            'password' => $_POST['password']
        );
        $data = array(
            'post' => $post
        );
        $registerM = $this->MODEL('register');
        $return = $registerM->ajaxReg($data);
        echo json_encode($return);
    }

    function regmoblie_action()
    {
        $data = array(
            'moblie' => $_POST['moblie'],
        );
        $registerM = $this->MODEL('register');
        $return = $registerM->regMoblie($data);
        if ($return['errcode'] == 9) {
            echo json_encode($return['data']);
        } else {
            echo $return['errcode'];
        }
    }

    function regemail_action()
    {
        $data = array(
            'email' => $_POST['email'],
        );
        $registerM = $this->MODEL('register');
        $return = $registerM->regMoblie($data);
        if ($return['errcode'] == 9) {
            echo json_encode($return['data']);
        } else {
            echo $return['errcode'];
        }
    }

    function writtenoff_action()
    {
        $data = array(
            'zyuid' => $_POST['zyuid'],
            'pw' => $_POST['pw'],
            'mobile' => $_POST['mobile'],
            'email' => $_POST['email'],
        );
        $registerM = $this->MODEL('register');
        $return = $registerM->writtenOff($data);

        echo $return['errcode'];
    }

    /**
     *users:王旭
     *Data:2023/2/13
     *Time:16:07
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
        $data['source'] = 2;
        $data['did'] = $this->config['did'];
        $data['port'] = 2;
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
            }
        }else{
            $arr['msg']		=	$return['msg'];
            $arr['status']	=	8;
        }

        echo json_encode($arr);die;
    } 

     
}

?>