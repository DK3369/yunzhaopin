<?php



class wap_controller extends common
{

    public $resumeInfo  =   array();
    public $comInfo     =   array();
    

    function __construct($tpl, $db, $def = '', $model = 'index', $m = '')
    {

        parent::__construct($tpl, $db, $def, $model, $m);

        if ($this->usertype == 1) {


            $resumeM    =   $this->MODEL('resume');

            if ($this->config['user_resume_status'] == '1') {
                if (!in_array($_GET['c'], array('addresume', 'userLog', 'kresume', 'binding'))) {

                    $expectnum  =   $resumeM->getExpectNum(array('uid' => $this->uid));

                    if ($expectnum < 1) {

                        $this->yunset('header_title', yun_at('wap_user_00197'));
                        $this->yunset("remind", array('info' => yun_at('member_user_00615'), 'url' => 'index.php?c=addresume', 'btn' => yun_at('wap_user_00372')));
                        $this->yuntpl(array('wap/member/user/addresume'));
                    }
                }
            } else {

                $this->resumeInfo =   $resumeM->getResumeInfo(array('uid' => $this->uid));
                if (!$this->resumeInfo['uid']) {

                    $isActivUser    =   1;
                    $activuid       =   $this->uid;
                }
            }
        } elseif ($this->usertype == 2) {

            $this->yunset('todayStart', strtotime('today'));

            $CompanyM       =   $this->MODEL("company");
            $this->comInfo  =   $CompanyM->getInfo($this->uid, array('info' => '1', 'edit' => '1', 'logo' => '1', 'utype' => 'user'));
            $this->yunset('info', $this->comInfo);

            if (!in_array($_GET['c'], array('photo', 'info', 'userLog', 'ajaxCheckInfo', 'poi'))) {


                if ($this->config['com_enforce_info'] == 1) {
                    if (!$this->comInfo['info']['name'] || !$this->comInfo['info']['provinceid'] || !$this->comInfo['info']['linktel']) {

                        $this->yunset('header_title', yun_at('wap_00456'));
                        $this->yunset("remind", array('info' => yun_at('wap_user_00373'), 'url' => 'index.php?c=info', 'btn' => yun_at('wap_user_00117')));
                        $this->yuntpl(array('wap/member/com/info'));
                    }
                } elseif (!$this->comInfo['info']['uid']) {

                    $isActivUser    =   1;
                    $activuid       =   $this->uid;
                }
            }
        }


        if ($isActivUser == 1) {

            $userinfoM  =   $this->MODEL("userinfo");
            $userinfoM->activUser($activuid, $this->usertype);
        }

        include PLUS_PATH . 'tplmoblie.cache.php';
        $this->yunset('tplmoblie', $tplmoblie);
    }

    function waplayer_msg($msg, $url = '1', $tm = 2)
    {

        $msg    =   preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $msg));

        $layer_msg['msg']   =   $msg;
        $layer_msg['url']   =   $url;
        $layer_msg['tm']    =   $tm;

        $msg    =   json_encode($layer_msg);
        echo $msg;
        die();
    }
}

?>