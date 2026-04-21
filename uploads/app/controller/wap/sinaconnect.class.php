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

class sinaconnect_controller extends common
{
    function actMsg($msgUrl, $msg, $status = 8)
    {
        $this->ACT_msg_wap($msgUrl, $msg, $status, 2, 5);
    }

    function getMsgUrl()
    {
        if (isMobileUser()) {
            if ($this->config['sy_wapdomain'] != "") {
                if ($this->config['sy_wapssl'] == '1') {

                    $protocol = 'https://';
                } else {

                    $protocol = 'http://';
                }
                if (strpos($this->config['sy_wapdomain'], 'http://') === false && strpos($this->config['sy_wapdomain'], 'https://') === false) {
                    $msgUrl = $protocol . $this->config['sy_wapdomain'];
                } else {
                    $msgUrl = $this->config['sy_wapdomain'];
                }

            } else {
                $msgUrl = $this->config['sy_weburl'] . '/wap/';
            }
        } else {
            $msgUrl = $this->config['sy_weburl'];
        }
        return $msgUrl;
    }

    function index_action()
    {

        $msgUrl =   $this->getMsgUrl();
        if ($this->config['sy_sinalogin'] != "1") {
            if ((int)$_GET['login'] == "1") {

                $this->actMsg($msgUrl, "对不起，新浪登录已关闭！");
            }
        }
        include_once(APP_PATH . 'api/weibo/saetv2.ex.class.php');
        define("WB_AKEY", $this->config['sy_sinaappid']);
        define("WB_SKEY", $this->config['sy_sinaappkey']);
        define("WB_CALLBACK_URL", Url('wap', array('c' => 'sinaconnect')));
        $o  =   new SaeTOAuthV2(WB_AKEY, WB_SKEY);
        if (isset($_GET['code'])) {

            $keys                   =   array();
            $keys['code']           =   $_GET['code'];
            $keys['redirect_uri']   =   WB_CALLBACK_URL;
            $token                  =   $o->getAccessToken('code', $keys);

            if ($token['access_token']) {

                $tokens     =   $token['access_token'];
                $tokenuid   =   $token['uid'];
                if ($tokenuid > 0) {

                    $UserinfoM  =   $this->MODEL('userinfo');

                    if ($this->uid != "" && $this->username != "") {

                        //已登录状态下 重新绑定账户
                        $UserinfoM->upInfo(array('sinaid' => $tokenuid), array('sinaid' => ''));
                        $UserinfoM->upInfo(array('uid' => $this->uid), array('sinaid' => $tokenuid));
                        $this->wapheader('member/index.php?c=binding');
                    }

                    $userinfo   =   $UserinfoM->getInfo(array('sinaid' => $tokenuid));

                    if (is_array($userinfo)) {
                        if ($userinfo['usertype'] > 0) {
                            if ($userinfo['status'] == '2' || $userinfo['status'] == '4') {
                                // 账号被锁定
                                header('Location: ' . Url('wap', array('c' => 'login', 'a' => 'loginlock', 'type' => 1)));
                                exit();
                            }
                            $UserinfoM->upInfo(array('uid' => $userinfo['uid']), array('login_date' => time()));

                            // 会员日志，记录手动登录
                            $LogM       =   $this->MODEL('log');
                            $logContent =   '账号登录：快捷登录';
                            $logDetail  =   'H5 新浪微博快捷登录';
                            $LogM->addMemberLog($userinfo['uid'], $userinfo['usertype'], $logContent, 32, 1, $logDetail);

                            $logtime    =   date("Ymd", $userinfo['login_date']);
                            $nowtime    =   date("Ymd", time());

                            if ($this->config['resume_sx'] == 1 && $userinfo['usertype'] == 1) {
                                $resumeM    =   $this->MODEL('resume');
                                $resume     =   $resumeM->getResumeInfo(array('uid' => $userinfo['uid']), array('field' => '`def_job`'));
                                if ($resume['def_job']) {
                                    $resumeM->upInfo(array('id' => $resume['def_job']), array('eData' => array('lastupdate' => time())));
                                    $resumeM->upResumeInfo(array('uid' => $userinfo['uid']), array('rData' => array('lastupdate' => time()), 'port' => 2));
                                }
                            }

                            if ($logtime != $nowtime) {

                                $this->MODEL('integral')->invtalCheck($userinfo['uid'], $userinfo['usertype'], "integral_login", "会员登录", 22);
                                //登录日志
                                $logdata['uid']         =   $userinfo['uid'];
                                $logdata['usertype']    =   $userinfo['usertype'];
                                $logdata['did']         =   $userinfo['did'];
                                $logdata['content']     =   'WAP微博登录';
                                $LogM->addLoginlog($logdata);
                                $ip    =  fun_ip_get();
                                $upLogin = array(
                                    'login_ip' => $ip,
                                    'login_date'=>time()
                                );
                                if ($userinfo['login_ip'] != $ip || $userinfo['login_address'] =='') {
                                    $upLogin['login_address'] = getIpAddress($ip);
                                }
                                $UserinfoM->upInfo(array('uid' => $userinfo['uid']),$upLogin);
                            }
                        }
                        if ($this->config['sy_uc_type'] == "uc_center") {

                            $this->obj->uc_open();
                            $user       =   uc_get_user($userinfo['username']);
                            $ucsynlogin =   uc_user_synlogin($user[0]);
                            $this->wapheader('member/');
                        } else {

                            $this->cookie->unset_cookie();
                            $this->cookie->add_cookie($userinfo['uid'], $userinfo['username'], $userinfo['salt'], $userinfo['email'], $userinfo['password'], $userinfo['usertype'], $this->config['sy_logintime'], $userinfo['did']);
                            $this->wapheader('member/');
                        }
                    } else {
                        session_start();

                        $_SESSION['sina']["openid"]     =   $tokenuid;
                        $_SESSION['sina']["tooken"]     =   $token['access_token'];
                        $_SESSION['sina']["logininfo"]  =   "您已登录新浪微博，请绑定您的帐户！";


                        $GetUrl =   "https://api.weibo.com/2/users/show.json?uid=" . $tokenuid . "&access_token=" . $token['access_token'];
                        $ch = curl_init();
                        curl_setopt($ch, CURLOPT_URL, $GetUrl);
                        curl_setopt($ch, CURLOPT_SSL_VERIFYHOST, 2);
                        curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
                        curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, FALSE);
                        $str = curl_exec($ch);
                        curl_close($ch);
                        $user = json_decode($str, true);

                        if ($user['name']) {

                            $_SESSION['sina']['nickname'] = $user['name'];
                            $_SESSION['sina']['pic'] = $user['avatar_hd'];
                        } else {

                            $this->actMsg($msgUrl, "用户信息获取失败，请重新登录新浪微博！");
                        }

                        $urlarr =   array('c' => 'sinaconnect', 'a' => 'sinabind');

                        if (isset($_GET['wxoauth'])) {

                            $urlarr['wxoauth'] = 1;
                        }
                        header("location:" . Url('wap', $urlarr));
                    }
                } else {

                    $this->actMsg($msgUrl, "新浪微博授权失败，请重新授权！");
                }
            }
        } else {

            $code_url = $o->getAuthorizeURL(WB_CALLBACK_URL);
            header("location:" . $code_url);
        }
    }

    function sinabind_action()
    {
        session_start();
        if ($_POST) {

            if ($_SESSION['sina']['openid']) {

                $data   =   array(
                    'openid'        =>  $_SESSION['sina']['openid'],
                    'source'        =>  10,
                    'moblie'        =>  $_POST['moblie'],
                    'moblie_code'   =>  $_POST['moblie_code'],
                    'code'          =>  $_POST['authcode'],
                    'port'          =>  2
                );
                $userinfoM  =   $this->MODEL('userinfo');
                $return     =   $userinfoM->fastReg($data, '', 'sinaweibo');

                if ($return['errcode'] == 9) {

                    $result['url']  =   Url("wap") . 'member/';
                } else {

                    $result['msg']  =   $return['msg'];
                }
            } else {

                $result['msg']      =   '微博登录信息已失效，请重新登录！';
            }
            echo json_encode($result);
        } else {

            $this->yunset("headertitle", "微博登录绑定");
            $this->yunset('sinalogin');
            $this->yuntpl(array('wap/sinabind'));
        }
    }

    function sendmsg_action()
    {

        $noticeM    =   $this->MODEL('notice');
        $result     =   $noticeM->jycheck($_POST['authcode'], '前台登录');
        if (!empty($result)) {
            $this->layer_msg($result['msg'], 9, 0, '', 2, $result['error']);
        }

        $moblie     =   $_POST['moblie'];
        $UserinfoM  =   $this->MODEL('userinfo');
        $userinfo   =   $UserinfoM->getInfo(array("moblie" => $moblie), array('field' => "`usertype`,`uid`"));

        if ($this->config['sy_reg_type'] == 2 && empty($userinfo)) {
            $result =   array(
                'error' =>  2,
                'msg'   =>  '请先注册账号',
                'url'   =>  Url('wap', array('c' => 'register'))
            );
        }else {
            $user = array(
                'uid' => $userinfo['uid'],
                'usertype' => $userinfo['usertype']
            );
            $result     =   $noticeM->sendCode($moblie, 'login', 2, $user, 6, 90, 'msg');
        }

        echo json_encode($result);
        exit();
    }
}

