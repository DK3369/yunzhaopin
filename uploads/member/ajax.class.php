<?php

/*
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class ajax_controller extends common
{

    /**
     *功能需求：添加简历置顶开启条件
     *内容描述：当网站设置开启简历中工作简历，教育经历，项目经历进行验证
     */
    function top_resume_action()
    {

        $data       =   array(
            'eid'   =>  $_POST['eid'],
            'uid'   =>  $this->uid
        );

        $resumeM    =   $this->MODEL('resume');

        $return     =   $resumeM->topResumeCheck($data);

        echo json_encode($return);

        die;
    }

    /**
     * @desc 订单付款类型修改（仅限会员中心操作）
     */
    function order_type_action()
    {

        if ($this->uid && $this->username) {

            $orderM =   $this->MODEL('companyorder');
            $oid    =   intval($_POST['oid']);
            $nid    =   $orderM->upInfo($oid, array('order_type' => $_POST['paytype'], 'port' => '1'), $this->uid);
            if ($nid) {

                $this->MODEL('log')->addMemberLog($this->uid, $this->usertype, "财务订单：修改订单（ID".$oid."）付款类型", 88, 2);
            }
            
            echo $nid ? 1 : 2;
        }
    }

    /**
     * @desc 邮箱认证,发送邮件（会员中心）
     */
    function emailcert_action()
    {

        $CookieM    =   $this->MODEL('cookie');

        $ComapnyM   =   $this->MODEL('company');

        $CookieM->SetCookie('delay', '', time() - 60);

        session_start();

        if (md5(strtolower($_POST['authcode'])) != $_SESSION['authcode'] || empty($_SESSION['authcode'])) {

            echo 4;
            die;
        }

        $data       =   array(

            'usertype'  =>  $this->usertype,
            'did'       =>  $this->userid,
            'email'     =>  $_POST['email']
        );

        $errCode    =   $ComapnyM->sendCertEmail(array('uid' => $this->uid, 'type' => '1'), $data);

        echo $errCode;
        die;
    }


    /**
     * @desc 手机认证,发送短信（会员中心）
     */
    function mobliecert_action()
    {

        $noticeM    =   $this->MODEL('notice');

        $result     =   $noticeM->jycheck($_POST['pcode'], '');

        if (!empty($result)) {

            $this->layer_msg($result['msg'], 9, 0, '', 2, $result['error']);
        }

        if (!$this->uid || !$this->username) {

            $this->layer_msg('请先登录', 9, 0, '', 2, 110);
        } else {

            $shell  =   $this->GET_user_shell($this->uid, $_COOKIE['shell']);

            if (!is_array($shell)) {

                $this->layer_msg('登录有误', 9, 0, '', 2, 111);
            }

            $moblie =   $_POST['str'];

            $user   =   array(
                'uid'       =>  $this->uid,
                'usertype'  =>  $this->usertype
            );

            $result =   $noticeM->sendCode($moblie, 'cert', 1, $user);

            $logM       =   $this->MODEL('log');
            $logContent =   '账号认证：发送手机认证验证码';
            $logDetail  =   '手机认证，发送短信验证码；认证手机号码：'.$moblie;
            $logM->addMemberLog($user['uid'], $user['usertype'], $logContent, 12, 1, $logDetail);

            echo json_encode($result);
            exit();
        }
    }

    /**
     * @desc 获取参与招聘会的公司（会员中心）
     */
    function getzphcom_action()
    {

        $jobM   =   $this->MODEL('job');
        $zphM   =   $this->MODEL('zph');

        $_GET['jobid']  =   pylode(',', @explode(',', $_GET['jobid']));

        $jobwhere       =   array(
            'id'        =>  array('in', $_GET['jobid']),
            'uid'       =>  $this->uid,
            'r_status'  =>  1,
            'status'    =>  0
        );

        $listA  =   $jobM->getList($jobwhere, array('field' => "`name`"));
        $row    =   $listA['list'];

        $space  =   $zphM->getZphSpaceList("");
        $zph    =   $zphM->getInfo(array('id' => intval($_GET['zid'])), array('field' => '`title`,`address`,`starttime`,`endtime`'));
        $com    =   $zphM->getZphComInfo(array('zid' => intval($_GET['zid']), 'uid' => $this->uid));

        foreach ($row as $v) {

            $data[] = $v['name'];
        }

        $spaces =   array();

        foreach ($space as $val) {
            $spaces[$val['id']] = $val['name'];
        }

        $cname  =   @implode('、', $data);

        $arr    =   array();

        $arr['status']      =   1;
        $arr['content']     =   $cname;
        $arr['title']       =   $zph['title'];
        $arr['address']     =   $zph['address'];
        $arr['starttime']   =   $zph['starttime'];
        $arr['endtime']     =   $zph['endtime'];

        if ($spaces[$com['sid']]) {

            $arr['sid']     =   $spaces[$com['sid']];
        } else {

            $arr['sid']     =   '无';
        }
        $arr['bid']         =   $spaces[$com['bid']];
        $arr['cid']         =   $spaces[$com['cid']];

        echo json_encode($arr);
    }

    
    /**
     * @desc 搜索器获取职位类别
     */
    function getjoblist_action()
    {

        include(PLUS_PATH . "job.cache.php");
        if (is_array($_POST[id])) {

            $jobid  =   $_POST[id][0];
        } else {

            $jobid  =   $_POST[id];
        }

        $data   =   "<option value=''>请选择</option>";

        if (isset($job_type) && isset($job_name) &&is_array($job_type[$jobid])) {
            foreach ($job_type[$jobid] as $v) {
                $data .= "<option value='$v'>" . $job_name[$v] . "</option>";
            }
        }
        echo $data;
    }

    /**
     * @desc 搜索器获取城市类别
     */
    function getcitylist_action()
    {

        if ($_POST['type'] == 'province') {

            $self   =   'cityid';
            $son    =   'three_cityid';
        } else {

            $self   =   'three_cityid';
            $son    =   '';
        }
        include(PLUS_PATH . "city.cache.php");
        $data       =   '';
        if (isset($city_type) && isset($city_name) && is_array($city_type[$_POST['id']])) {
            foreach ($city_type[$_POST['id']] as $v) {
                $data .= "<li><a href=\"javascript:void(0);\" onclick=\"getcitylist('" . $v . "','" . $self . "','" . $city_name[$v] . "','" . $son . "');\">" . $city_name[$v] . "</a></li>";
            }
        }
        echo $data;
        die;
    }

    /**
     * @desc 签到
     */
    function sign_action()
    {
        // 防止重复签到判断
        session_start();
        if (!isset($_SESSION['qiandao'])){
            $_SESSION['qiandao'] = 1;
        }else{
            $arr['type'] = -2;
            echo json_encode($arr);
            die;
        }

        $IntegralM  =   $this->MODEL('integral');
        $userinfoM  =   $this->MODEL('userinfo');

        $date       =   date("Ymd");

        $member     =   $userinfoM->getInfo(array('uid' => $this->uid, 'usertype' => $_COOKIE['usertype']), array('field' => "`signday`,`signdays`"));
        $lastreg    =   $userinfoM->getMemberregInfo(array('uid' => $this->uid, 'usertype' => $_COOKIE['usertype'], 'orderby' => 'id,desc'));

        $lastregdate=   date("Ymd", $lastreg['ctime']);

        if ($lastregdate != $date) {

            $yesterday          =   date("Ymd", strtotime("-1 day"));

            if ($lastregdate == $yesterday && intval(date("d")) > 1) {
                if ($member['signday'] >= 5) {

                    $integral   =   $this->config['integral_signin'] * 2;
                } else {

                    $integral   =   $this->config['integral_signin'];
                }

                $signday        =   $member['signday'] + 1;
                $msg            =   '连续签到' . $signday . "天";
            } else {

                $signday        =   '1';
                $integral       =   $this->config['integral_signin'];
                $msg            =   '第一次签到';
            }

            $arr                =   array();

            $nid                =   $userinfoM->addMemberreg(array("uid" => $this->uid, "usertype" => $_COOKIE['usertype'], 'date' => $date, "ctime" => time(), 'ip' => fun_ip_get()));

            if (isset($nid)) {

                $IntegralM->company_invtal($this->uid, $this->usertype, $integral, true, $msg, true, 2, 'integral');
                $userinfoM->upInfo(array('uid' => $this->uid), array('signday' => $signday, 'signdays' => array('+', '1')));
                $arr['type']    =   date("j");
                // 签到成功，请除session
                unset($_SESSION['qiandao']);
            } else {

                $arr['type']    =   -2;
            }
            $arr['integral']    =   $integral . $this->config['integral_pricename'];
            $arr['signday']     =   $signday;
            $arr['signdays']    =   $member['signdays'] + 1;

            echo json_encode($arr);
            die;
        }
    }

    /**
     * @desc 会员中心批量阅读
     */
    public function ajaxReadsys_action()
    {
        if ($_POST['ids']) {

            $sysM   =   $this->MODEL('sysmsg');
            $result =   $sysM->upInfo(array('id' => array('in', pylode(',', $_POST['ids']))), array('remind_status' => '1'));
            echo json_encode($result);
            die;
        }
    }

    /**
     * @desc 会员中心，系统消息全部标记为已读
     */
    public function ajaxRreadSysAll_action()
    {
        if ($_POST) {

            $sysM   =   $this->MODEL('sysmsg');
            $result =   $sysM->upInfo(array('fa_uid' => $this->uid, 'usertype' => $this->usertype, 'remind_status' => '0'), array('remind_status' => '1'));

            echo $result;
            die;
        }
    }
    function poi_action(){
        
        echo getPoi($_POST);
    }
}

?>