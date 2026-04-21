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
class warning_model extends model
{

    /**
     * @desc 预警提醒
     * @param int $type : 1-发布职位 2-简历下载 3-发布简历 4-充值 5-短信 6-投递 7-浏览简历 8-浏览职位 9-拨号 12-注册账号同一IP 13-发布职位同一IP 14-创建简历同一IP
     * @param null $uid
	 * @param null $ip
     */
    function warning($type, $uid = null, $ip = null)
    {

        $today = strtotime('today');


        if ($type == 1 && $this->config['warning_addjob'] > 0) {    // 发布职位

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {

                $num1 = $this->select_num('company_job', array('uid' => $uid, 'sdate' => array('>', $today)));
                $num2 = $this->select_num('partjob', array('uid' => $uid, 'sdate' => array('>', $today)));

                $num = $num1 + $num2;
                if ($num >= $this->config['warning_addjob']) {

                    $this->send_warning($type, $uid);
                }
            }
        } elseif ($type == 2 && $this->config['warning_downresume'] > 0) {  // 简历下载

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $num = $this->select_num('down_resume', array('comid' => $uid, 'downtime' => array('>', $today)));

                if ($num >= $this->config['warning_downresume']) {

                    $this->send_warning($type, $uid);
                }
            }
        } elseif ($type == 3 && $this->config['warning_addresume'] > 0) {   // 简历发布

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $num = $this->select_num('resume_expect', array('uid' => $uid, 'ctime' => array('>', $today)));

                if ($num >= $this->config['warning_addresume']) {

                    $this->send_warning($type, $uid);
                }
            }
        } elseif ($type == 4 && $this->config['warning_recharge'] > 0) {    // 充值

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $order = $this->select_all('company_order', array('uid' => $uid, 'order_state' => 2, 'integral' => array('>', 0), 'order_time' => array('>', $today)), 'sum(`integral`) as `total`');

                if (!empty($order)) {

                    $price = ceil($order[0]['total'] / $this->config['integral_proportion']);

                    if ($price >= $this->config['warning_recharge']) {

                        $this->send_warning($type, $uid);
                    }
                }
            }
        } else if ($type == 5 && $this->config['sy_hour_msgnum'] > 0) { // 短信预警

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $time = time() - 3600;
                $num = $this->select_num('moblie_msg', array('ctime' => array('>', $time)));
                $msg = "系统一小时内已发送" . $num . "条短信！";

                if ($num >= $this->config['sy_hour_msgnum']) {

                    $this->send_warning($type, $uid, $msg);
                }
            }
        } else if ($type == 6) {    //每天简历投递

            if ($this->config['warning_sendresume'] > 0) {   //  限制投递数量
                $isWaring = $this->isWarning($type, $uid);
                if (!$isWaring) {
                    $num = $this->select_num('userid_job', array('uid' => $uid, 'datetime' => array('>', $today)));

                    if ($num >= $this->config['warning_sendresume']) {

                        $this->send_warning($type, $uid);
                    }
                }
            }

            if ($this->config['warning_sqjob'] > 0) {   //  限制投递类别
                $isWaring = $this->isWarning($type, $uid);

                if (!$isWaring) {

                    $sqList = $this->select_all('userid_job', array('uid' => $uid, 'datetime' => array('>', $today), 'limit' => 500), 'job_id');

                    $sqJobIds = array();
                    foreach ($sqList as $sqk => $sqv) {
                        $sqJobIds[] = $sqv['job_id'];
                    }

                    $sqJobList = $this->select_all('company_job', array('id' => array('in', pylode(',', $sqJobIds))), 'job1');
                    $jobClassIdArr = array();
                    foreach ($sqJobList as $jk => $jv) {
                        if (!in_array($jv['job1'], $jobClassIdArr)) {
                            $jobClassIdArr[] = $jv['job1'];
                        }
                    }

                    if (count($jobClassIdArr) >= $this->config['warning_sqjob']) {

                        $this->send_warning($type, $uid, '', 1);
                    }
                }
            }
        } else if ($type == 7 && $this->config['warning_lookresume'] > 0) {    //  浏览简历

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $num    =   $this->select_num('look_resume', array('com_id' => $uid, 'datetime' => array('>', $today)));

                if ($num >= $this->config['warning_lookresume']) {

                    $this->send_warning($type, $uid);
                }
            }
        } else if ($type == 8 && $this->config['warning_lookjob'] > 0) {    //  浏览职位

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $num    =   $this->select_num('look_job', array('uid' => $uid, 'datetime' => array('>', $today)));

                if ($num >= $this->config['warning_lookjob']) {

                    $this->send_warning($type, $uid);
                }
            }
        } else if ($type == 9 && $this->config['warning_teljob'] > 0) {    //  求职者拨号

            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $num    =   $this->select_num('job_tellog', array('uid' => $uid, 'ctime' => array('>', $today)));

                if ($num >= $this->config['warning_teljob']) {

                    $this->send_warning($type, $uid);
                }
            }
        } else if ($type == 12 && $this->config['warning_reg_ip'] > 0){ //注册账号同一IP
            $isWaring = $this->isWarning($type, $uid);

            if (!$isWaring) {
                $num    =   $this->select_num('member', array('reg_ip' => $ip,'reg_date' => array('>', $today)));

                if ($num >= $this->config['warning_reg_ip']) {

                    $this->send_warning($type, $uid);
                }
            }
        }
    }

    private function isWarning($type, $uid)
    {

        $result = false;
        if ($type == 5) {

            $isWarning = $this->select_once('warning', array('type' => $type, 'ctime' => array('>', strtotime('-1 hours'))));
        } else {

            $isWarning = $this->select_once('warning', array('type' => $type, 'uid' => $uid, 'ctime' => array('>', strtotime('today'))));
        }
        if (!empty($isWarning)) {

            $result = true;
        }
        return $result;
    }

    /**
     * @desc  预警提醒
     * @param int $type
     * @param null $uid
     * @param string $emailcoment
     * @param int $way
     */
    function send_warning($type, $uid = null, $emailcoment = null, $way=0)
    {

        

        $member     = $this->select_once('member', array('uid' => $uid), '`email`,`username`,`usertype`');
        $this->insert_into('warning',array('usertype' => $member['usertype'], 'type' => $type, 'uid' => $uid, 'status' => 2, 'ctime' => time(),'content'=>$contentV));

        $email      =   $member['email'];
        $username   =   trim($member['username']);
        $usertype   =   intval($member['usertype']);

        include_once('resume.model.php');
        $resumeM    =   new resume_model($this->db, $this->def);

        require_once('notice.model.php');
        $notice     =   new notice_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

        require_once('cookie.model.php');
        $cookie     =   new cookie_model($this->db, $this->def, array('uid' => $uid, 'username' => $username, 'usertype' => $usertype));

        if ($type == 1) { // 发布职位

            $emailcoment    =   '用户：【' . $username . '】发布职位超出规定数目，请检查是否有问题';

            if ($this->config['warning_addjob_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '发布职位超出规定数目'), array('uid' => $uid));

                $this->update_once('company', array('r_status' => 2), array('uid' => $uid));

                $this->update_once('company_job', array('r_status' => 2), array('uid' => $uid));
                $this->update_once('partjob', array('r_status' => 2), array('uid' => $uid));

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '发布职位超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天发布职位限制)';
            }
        } elseif ($type == 2) { // 下载简历

            $emailcoment    =   '用户：【' . $username . '】下载简历超出规定数目，请检查是否有问题';

            if ($this->config['warning_downresume_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '下载简历超出规定数目'), array('uid' => $uid));

                $this->update_once('company', array('r_status' => 2), array('uid' => $uid));

                $this->update_once('company_job', array('r_status' => 2), array('uid' => $uid));
                $this->update_once('partjob', array('r_status' => 2), array('uid' => $uid));

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '下载简历超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天下载简历限制)';
            }
        } elseif ($type == 3) { // 简历发布

            $emailcoment    =   '用户：【' . $username . '】简历发布超出规定数目，请检查是否有问题';

            if ($this->config['warning_addresume_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '简历发布超出规定数目'), array('uid' => $uid));
                $this->update_once('resume', array('r_status' => 2), array('uid' => $uid));

                $resumeM->setExpectState(array('state' => 3, 'r_status' => 2, 'statusbody' => '简历发布超出规定数目'), array('uid' => $uid));

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '简历发布超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天发布简历限制)';
            }
        } elseif ($type == 4) { // 充值

            $emailcoment    =   '用户：【' . $username . '】充值超出规定金额，请检查是否有问题';

            if ($this->config['warning_recharge_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '充值超出规定金额'), array('uid' => $uid));

                if ($usertype == 1) {

                    $this->update_once('resume', array('r_status' => 2), array('uid' => $uid));
                    $resumeM->setExpectState(array('state' => '0', 'r_status' => 2), array('uid' => $uid));
                } else if ($usertype == 2) {

                    $this->update_once('company', array('r_status' => 2), array('uid' => $uid));
                    $this->update_once('company_job', array('r_status' => 2), array('uid' => $uid));
                    $this->update_once('partjob', array('r_status' => 2), array('uid' => $uid));
                }

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '充值超出规定金额', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天充值金额限制)';
            }
        } else if ($type == 5 && $this->config['warning_closemsg_type'] == 1) {

            $this->update_once('admin_config', array('config' => 2, 'name' => 'sy_msg_isopen'));

        } else if ($type == 6) { // 简历投递

            if ($way == 1){

                $emailcoment    =   '用户：【'.$username.'】简历投递超出规定分类，请检查是否有问题';

                if ($this->config['warning_sqjob_type'] == 3){
                    $this->update_once('member', array('status' => 2, 'lock_info' => '简历投递超出限定'), array('uid' => $uid));
                    $this->update_once('resume', array('r_status' => 2), array('uid' => $uid));

                    $resumeM->setExpectState(array('state' => 3, 'r_status' => 2), array('uid' => $uid));

                    $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '简历投递超出限定', 'type' => 'lock'));
                    $cookie->unset_cookie();
                    $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天简历投递超出限制)';
                }
            }else{

                $emailcoment    =   '用户：【'.$username.'】简历投递超出规定数目，请检查是否有问题';

                if ($this->config['warning_sendresume_type'] == 3){
                    $this->update_once('member', array('status' => 2, 'lock_info' => '简历投递超出限定'), array('uid' => $uid));
                    $this->update_once('resume', array('r_status' => 2), array('uid' => $uid));

                    $resumeM->setExpectState(array('state' => 3, 'r_status' => 2), array('uid' => $uid));

                    $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '简历投递超出限定', 'type' => 'lock'), '');
                    $cookie->unset_cookie();
                    $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天简历投递超出限制)';
                }
            }
        } else if ($type == 7){ //  浏览简历

            $emailcoment    =   '用户：【' . $username . '】浏览简历超出规定数目，请检查是否有问题';

            if ($this->config['warning_lookresume_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '浏览简历超出规定数目'), array('uid' => $uid));

                $this->update_once('company', array('r_status' => 2), array('uid' => $uid));

                $this->update_once('company_job', array('r_status' => 2), array('uid' => $uid));
                $this->update_once('partjob', array('r_status' => 2), array('uid' => $uid));

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '浏览简历超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天浏览简历限制)';
            }
        }  else if ($type == 8){ //  浏览职位

            $emailcoment    =   '用户：【' . $username . '】浏览职位超出规定数目，请检查是否有问题';

            if ($this->config['warning_lookjob_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '浏览职位超出规定数目'), array('uid' => $uid));
                $this->update_once('resume', array('r_status' => 2), array('uid' => $uid));

                $resumeM->setExpectState(array('state' => 3, 'r_status' => 2, 'statusbody' => '浏览职位超出规定数目'), array('uid' => $uid));

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '浏览职位超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天浏览职位限制)';
            }
        } else if ($type == 9){ //  拨号限制

            $emailcoment    =   '用户：【' . $username . '】拨号次数超出规定数目，请检查是否有问题';

            if ($this->config['warning_teljob_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '拨号次数超出规定数目'), array('uid' => $uid));
                $this->update_once('resume', array('r_status' => 2), array('uid' => $uid));

                $resumeM->setExpectState(array('state' => 3, 'r_status' => 2, 'statusbody' => '拨号次数超出规定数目'), array('uid' => $uid));

                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '拨号次数超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-每天拨号次数限制)';
            }
        } else if ($type == 12){ //  注册账号同一IP

            $emailcoment    =   '用户：【' . $username . '】您注册账号过多，请检查是否有问题';

            if ($this->config['warning_reg_ip_type'] == 1) { // 锁定帐号

                $this->update_once('member', array('status' => 2, 'lock_info' => '同一IP每天注册账号超出规定数目'), array('uid' => $uid));
                $notice->sendEmailType(array('email' => $email, 'uid' => $uid, 'username' => $username, 'lock_info' => '同一IP每天注册账号超出规定数目', 'type' => 'lock'));
                $cookie->unset_cookie();

                $logContent =   '账户锁定(ID:' . $uid . '。原因：预警-同一IP每天注册账号次数限制)';
            }
        }
        // 发送邮件提醒管理员
        $emailData  =   array('email' => $this->config['sy_webemail'], 'subject' => '预警提醒', 'content' => $emailcoment);
        $notice->sendEmail($emailData);

        if (!empty($logContent)) {
            // 锁定账号的记录管理员日志
            $data   =   array('uid' => 0, 'username' => '预警', 'content' => $logContent, 'ctime' => time(), 'ip' => fun_ip_get(), 'did' => $this->config['did']);
            $this->insert_into('admin_log', $data);
        }
    }
}
?>