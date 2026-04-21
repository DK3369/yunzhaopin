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

class wxconnect_controller extends common
{

    function index_action()
    {
        if ($this->config['wx_rz'] != "1") {

            if ((int)$_GET['login'] == "1") {

                $this->ACT_msg_wap(Url("wap") . 'member/', $msg = "对不起，微信绑定已关闭！", 2, 5);
            } else {

                $this->ACT_msg_wap(Url("wap"), $msg = "对不起，微信登录已关闭！", 2, 5);
            }
        }
        $this->seo('wxlogin');

        $app_id     =   $this->config['wx_appid'];
        $app_secret =   $this->config['wx_appsecret'];
        $my_url     =   $this->config['sy_wapdomain'] . "/index.php?c=wxconnect";

        if (isset($_GET['wxoauth'])) {
            $my_url .=  '&wxoauth=1';
        }
        $code       =   $_GET['code'];

        session_start();

        if (empty($code)) {

            $_SESSION['wx']['state']    =   md5(uniqid(rand(), TRUE));

            $dialog_url =   "https://open.weixin.qq.com/connect/oauth2/authorize?appid=" . $app_id . "&redirect_uri=" . urlencode($my_url) . "&response_type=code&scope=snsapi_userinfo&state=" . $_SESSION['wx']['state'] . "#wechat_redirect";

            header("location:" . $dialog_url);
            die;
        }

        if ($_GET['state'] == $_SESSION['wx']['state']) {

            $token_url  =   "https://api.weixin.qq.com/sns/oauth2/access_token?appid=" . $app_id . "&secret=" . $app_secret . "&code=" . $code . "&grant_type=authorization_code";

            if (function_exists('curl_init')) {

                $ch =   curl_init();
                curl_setopt($ch, CURLOPT_URL, $token_url);
                curl_setopt($ch, CURLOPT_SSL_VERIFYHOST, 2);
                curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
                curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, FALSE);
                $response   =   curl_exec($ch);
                curl_close($ch);

                $user   =   json_decode($response, true);

                if (!empty($user['openid'])) {

                    $userinfoM      =   $this->MODEL("userinfo");

                    $where['wxid']  =   $user['openid'];
                    if (!empty($user['unionid'])) {
                        $where['unionid']   =   array('=', $user['unionid'], 'OR');
                    }

                    $userinfo       =   $userinfoM->getInfo($where);

                    if (is_array($userinfo) && !empty($userinfo)) {
                        if ($this->uid) {
                            
                            $userinfoM->upInfo(array('uid' => $this->uid), array('wxid' => $user['openid'], 'unionid' => $user['unionid'], 'wxbindtime'=>time()));

                            if ($userinfo['usertype'] > 0) {
								$this->MODEL('integral')->invtalCheck($userinfo['uid'], $userinfo['usertype'], "integral_bind_wx", "微信扫码绑定", 28);
                                // 会员日志，记录手动绑定微信
                                $LogM       =   $this->MODEL('log');
                                $logContent =   '账号认证：绑定微信';
                                $logDetail  =   'H5 微信绑定';
                                $LogM->addMemberLog($userinfo['uid'], $userinfo['usertype'], $logContent, 12, 1, $logDetail);
                            }
                            
                            $this->wapheader('member/index.php?c=binding');
                        } else {

                            $userinfoM->upInfo(array('uid' => $userinfo['uid']), array('login_date' => time(), 'wxid' => $user['openid'], 'unionid' => $user['unionid'], 'wxbindtime'=>time()));

                            if ($userinfo['usertype'] > 0) {
                                if ($userinfo['status'] == '2' || $userinfo['status'] == '4') {
                                    // 账号被锁定
                                    header("location:" . Url('wap', array('c' => 'login', 'a' => 'loginlock', 'type' => 1)));
                                    exit();
                                }
                                // 会员日志，记录手动登录
                                $LogM       =   $this->MODEL('log');
                                $logContent =   '账号登录：快捷登录';
                                $logDetail  =   'H5 微信快捷登录';
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
                                    $logdata['content']     =   'WAP微信登录';
                                    $LogM->addLoginlog($logdata);
                                    $ip    =  fun_ip_get();
                                    $upLogin = array(
                                        'login_ip' => $ip,
                                        'login_date'=>time()
                                    );
                                    if ($userinfo['login_ip'] != $ip || $userinfo['login_address'] =='') {
                                        $upLogin['login_address'] = getIpAddress($ip);
                                    }

                                    $userinfoM->upInfo(array('uid' => $userinfo['uid']),$upLogin);
                                }
                            }

                            if ($this->config['sy_uc_type'] == "uc_center") {

                                $this->obj->uc_open();

                                $uc_user    =   uc_get_user($userinfo['username']);
                                $ucsynlogin =   uc_user_synlogin($uc_user[0]);
                                $msg        =   '登录成功！';
                                $this->wapheader('member/');
                            } else {

                                $this->cookie->add_cookie($userinfo['uid'], $userinfo['username'], $userinfo['salt'], $userinfo['email'], $userinfo['password'], $userinfo['usertype'], $this->config['sy_logintime'], $userinfo['did']);
                                $this->wapheader('member/');
                            }
                        }

                    } else {
                        $_SESSION['wx']["openid"]       =   $user['openid'];
                        $_SESSION['wx']["unionid"]      =   $user['unionid'];
                        $_SESSION['wx']["tooken"]       =   $user['access_token'];
                        $_SESSION['wx']["logininfo"]    =   "您已登录微信，请绑定您的帐户！";

                        if ($this->uid) {

                            // 会员中心微信绑定
                            $uwhere['wxid']             =   $user['openid'];

                            if (!empty($user['unionid'])) {

                                $uwhere['unionid']      =   array('=', $user['unionid'], 'OR');
                            }

                            $userinfoM->upInfo($uwhere, array('wxopenid' => '', 'wxid' => '', 'unionid' => ''));

                            $userinfoM->upInfo(array('uid' => $this->uid), array('wxid' => $user['openid'], 'unionid' => $user['unionid'],'wxbindtime'=>time()));

                            if ($this->usertype) {
								$this->MODEL('integral')->invtalCheck($this->uid, $this->usertype, "integral_bind_wx", "微信扫码绑定", 28);
                                // 会员日志，记录手动绑定微信
                                $LogM       =   $this->MODEL('log');
                                $logContent =   '账号认证：绑定微信';
                                $logDetail  =   'H5微信绑定';
                                $LogM->addMemberLog($this->uid, $this->usertype, $logContent, 12, 1, $logDetail);
                            }

                            $this->wapheader('member/index.php?c=binding');
                        } else {

                            $nick_url   =   'https://api.weixin.qq.com/sns/userinfo?access_token=' . $user['access_token'] . '&openid=' . $user['openid'] . '&lang=zh_CN';

                            $ch         =   curl_init();
                            curl_setopt($ch, CURLOPT_URL, $nick_url);
                            curl_setopt($ch, CURLOPT_SSL_VERIFYHOST, 2);
                            curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
                            curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, FALSE);
                            $response   =   curl_exec($ch);
                            curl_close($ch);

                            $user       =   json_decode($response, true);

                            if ($user['nickname']) {

                                $_SESSION['wx']['nickname'] =   $user['nickname'];
                                $_SESSION['wx']['pic']      =   $user['headimgurl'];

                                $urlarr =   array('c' => 'wxconnect', 'a' => 'wxbind');

                                if (isset($_GET['wxoauth'])) {

                                    $urlarr['wxoauth']  =   1;
                                }
                                header("location:" . Url('wap', $urlarr));
                            } else {

                                $this->ACT_msg(Url("wap"), "用户信息获取失败，请重新登录！", 8);
                            }
                        }
                    }
                }
            }
        } else {

            $this->wapheader('member/');
        }
    }

    function wxbind_action()
    {

        session_start();

        if ($_POST) {
            if ($_SESSION['wx']['openid']) {

                $data   =   array(
                    'openid'        =>  $_SESSION['wx']['openid'],
                    'unionid'       =>  $_SESSION['wx']['unionid'],
                    'source'        =>  4,
                    'moblie'        =>  $_POST['moblie'],
                    'moblie_code'   =>  $_POST['moblie_code'],
                    'code'          =>  $_POST['authcode'],
                    'port'          =>  2
                );
                $userinfoM  =   $this->MODEL('userinfo');
                $return     =   $userinfoM->fastReg($data, '', 'weixin');

                if ($return['errcode'] == 9) {

                    $result['url']  =   Url("wap") . 'member/';
                } else {

                    $result['msg']  =   $return['msg'];
                }
            } else {

                $result['msg']      =   '微信登录信息已失效，请重新登录！';
            }
            echo json_encode($result);
        } else {

            $this->yunset('headertitle', '微信登录绑定');
            $this->yunset('backurl', Url('wap', array('c' => 'login')));
            $this->yuntpl(array('wap/wxbind'));
        }
    }

    //登录短信验证码发送
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

?>