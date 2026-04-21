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

class transfer_model extends model
{
    /**
     * 账号分离，目前只有个人用户可以账号分离
     * @param $uid
     * @param array $data
     * @return mixed
     */
    public function setTransfer($uid, $data = array())
    {

        $uid    =   intval($uid);

        if ($uid && $data['oldpass'] && $data['username'] && $data['password']) {

            $data['username']   =   str_replace('！', '!', $data['username']);

            $msg                =   regUserNameComplex($data['username']);

            $pmsg               =   regPassWordComplex($data['password']);

            if ($msg != '') {

                $return['msg']  =   $msg;
            } elseif (CheckRegUser($data['username']) == false && CheckRegEmail($data['username']) == false) {

                $return['msg']  =   '新用户名不得包含特殊字符！';
            } else {

                $usernameNum    =   $this->select_num('member', array("username" => $data['username']));

                if ($usernameNum > 0) {

                    $return['msg']  =   '用户名已存在，请重新输入！';
                } elseif ($pmsg != '') {

                    $return['msg']  =   $pmsg;
                }
            }

            if (!isset($return['msg'])) {

                // 开始验证密码
                $mfiled     =   "`uid`,`username`,`password`,`salt`,`email`,`moblie`,`email_status`,`moblie_status`,`wxid`,`wxopenid`,`unionid`,`qqid`,`unionid`,`sinaid`";

                $memberInfo =   $this->select_once('member', array('uid' => $uid), $mfiled);

                if (!empty($memberInfo)) {

                    $res    =   passCheck($data['oldpass'], $memberInfo['salt'], $memberInfo['password']);

                    if ($res) {

                        // 判断是否需要解绑数据
                        if ($data['bdtype']) {

                            $jbType =   $data['bdtype'];
                            // 如果选择了解绑数据 那么之前数据迁移到对应的新账户名下 邮箱 QQ 微博 微信同理
                            foreach ($jbType as $key => $value) {

                                if ($value == 'moblie') {

                                    $companyData['moblie_status']   =   '0';
                                    $companyData['linktel']         =   '';

                                    $lxData['moblie_status']        =   '0';
                                    $lxData['moblie']               =   '';

                                    $memberData['moblie_status']    =   '0';
                                    $memberData['moblie']           =   '';

                                    $adata['moblie_status']         =   $memberInfo['moblie_status'];
                                    $adata['moblie']                =   $memberInfo['moblie'];
                                } elseif ($value == 'email') {

                                    $companyData['email_status']    =   '0';
                                    $companyData['linkmail']        =   '';

                                    $lxData['email_status']         =   '0';
                                    $lxData['email']                =   '';

                                    $memberData['email_status']     =   '0';
                                    $memberData['email']            =   '';

                                    $adata['email_status']          =   $memberInfo['email_status'];
                                    $adata['email']                 =   $memberInfo['email'];
                                } elseif ($value == 'qqid') {

                                    $memberData['qqunionid']        =   '';
                                    $memberData['qqid']             =   '';

                                    $adata['qqunionid']             =   $memberInfo['qqunionid'];
                                    $adata['qqid']                  =   $memberInfo['qqid'];
                                } elseif ($value == 'wxid') {

                                    $memberData['wxid']             =   '';
                                    $memberData['wxopenid']         =   '';
                                    $memberData['unionid']          =   '';

                                    $adata['wxid']                  =   $memberInfo['wxid'];
                                    $adata['wxopenid']              =   $memberInfo['wxopenid'];
                                    $adata['unionid']               =   $memberInfo['unionid'];
                                } elseif ($value == 'sinaid') {

                                    $memberData['sinaid']           =   '';
                                    $adata['sinaid']                =    $memberInfo['sinaid'];
                                }
                            }
                            $this->update_once('company', $companyData, array('uid' => $uid));

                        }

                        // 账号分离后，将原账号身份转换成其他身份，优先企业身份（不转换，原账号登录后还是个人身份，会造成误解）
                        $oldcom         =   $this->select_once('company', array('uid' => $uid), '`uid`');
                        if (!empty($oldcom)) {

                            $memberData['usertype'] = 2;
                        }
                        if (!empty($memberData)) {

                            $this->update_once('member', $memberData, array('uid' => $uid));
                        }

                        // 如果未解绑邮箱 手机数据 则需要清空新账户个人信息表中对应数据
                        if (!$jbType) {

                            $newResume['email']         = '';
                            $newResume['email_status']  = '0';
                            $newResume['telphone']      = '';
                            $newResume['moblie_status'] = '0';
                        } else {
                            if (!in_array('email', $jbType)) {
                                $newResume['email']         =   '';
                                $newResume['email_status']  =   '0';
                            }
                            if (!in_array('moblie', $jbType)) {
                                $newResume['telphone']      =   '';
                                $newResume['moblie_status'] =   '0';
                            }
                        }
                        if ($newResume) {
                            $this->update_once('resume', $newResume, array('uid' => $uid));
                        }

                        // 创建新账户
                        $salt = substr(uniqid(rand()), -6);
                        $pass = passCheck($data['password'], $salt);

                        $adata['username'] = $data['username'];
                        $adata['password'] = $pass;

                        $adata['did']       =   0;
                        $adata['status']    =    1;
                        $adata['salt']      =   $salt;
                        $adata['source']    =   21;
                        $adata['reg_date']  =   time();
                        $adata['reg_ip']    =   $memberInfo['reg_ip'];
                        $adata['usertype']  =   1;

                        $userid = $this->insert_into('member', $adata);

                        // 容错机制 防止因为某些情况下 未返回新增ID
                        if (!$userid) {
                            $user_id = $this->select_once('member', array("username" => $data['username']), 'uid');
                            $userid = $user_id['uid'];
                        }

                        // 将新账户UID 同步更新老数据 达成迁移效果
                        if ($userid) {

                            if (!empty($jbType) && in_array('moblie', $jbType)) {
                                $user_moblie = 1;
                            } else {
                                $user_moblie = 2;
                            }

                            $nid = $this->filterTableUid($userid, $uid, $user_moblie);

                            // 增加账号分离会员日志
                            require_once('log.model.php');
                            $LogM = new log_model($this->db, $this->def);

                            $LogM->addMemberLog($uid, 1, '账号分离成功，新账号用户名：' . $data['username']);
                            $LogM->addMemberLog($userid, 1, '账号分离成功，原账号用户名：' . $memberInfo['username']);

                            $return['msg'] = '账户分离成功！';
                            $return['errcode'] = 1;
                        } else {
                            $return['msg'] = '账户分离失败，请重试！';
                        }
                    } else {

                        $return['msg'] = '原账户密码错误！';
                    }
                } else {

                    $return['msg'] = '未找到原账号！';
                }
            }
        } else {
            $return['msg'] = '请正确填写相关信息！';
        }

        $return['errcode'] = $return['errcode'] ? $return['errcode'] : 0;

        return $return;
    }

    /**
     * 迁移数据UID
     * @param $uid
     * @param $olduid
     * @param int $user_moblie
     * @return bool
     */
    private function filterTableUid($uid, $olduid, $user_moblie = 2)
    {

        // 只需要替换UID
        $tableUidList = array(
            'resume',
            'resume_cert',
            'resume_cityclass',
            'resume_doc',
            'resume_edu',
            'resume_expect',
            'resume_jobclass',
            'resume_other',
            'resume_project',
            'resume_show',
            'resume_skill',
            'resume_training',
            'resume_work',
            'resumeout',
            'user_resume',
            'userid_job',
            'userid_msg',
            'down_resume',
            'entrust',
            'fav_job',
            'look_job',
            'look_resume',
            'member_statis',
            'msg',
            'part_apply',
            'part_collect',
            'talent_pool',
            'user_entrust',
            'user_entrust_record'
        );

        foreach ($tableUidList as $value) {

            if ($value == 'resume_expect') {
                if ($user_moblie == 2){

                    //分离时如果没有选择手机号分离，分离出的个人账号下简历，简历状态改成未审核
                    $this->update_once($value, array('uid' => $uid, 'state' => 0), array('uid' => $olduid));
                }else if($user_moblie == 3){

                    //合并账号，简历状态改成不公开
                    $this->update_once($value, array('uid' => $uid, 'status' => 2), array('uid' => $olduid));
                }else{

                    $this->update_once($value, array('uid' => $uid), array('uid' => $olduid));
                }
            } else {
                $this->update_once($value, array('uid' => $uid), array('uid' => $olduid));
            }

        }
        // 需要根据usertype进行识别身份

        $tableUsertypeList = array(
            'atn',
            'change',
            'company_order',
            'evaluate_leave_message',
            'finder',
            'login_log',
            'member_log',
            'member_reg',
        );

        foreach ($tableUsertypeList as $value) {

            $this->update_once($value, array('uid' => $uid), array('uid' => $olduid, 'usertype' => 1));
        }

        $this->update_once('blacklist', array('c_uid' => $uid), array('c_uid' => $olduid, 'usertype' => 1));

        $this->update_once('company_pay', array('com_id' => $uid), array('com_id' => $olduid, 'usertype' => 1));

        $this->update_once('report', array('p_uid' => $uid), array('p_uid' => $olduid, 'usertype' => 1));

        $this->update_once('sysmsg', array('fa_uid' => $uid), array('fa_uid' => $olduid, 'usertype' => 1));

        $this->update_once('recommend', array('uid' => $uid), array('uid' => $olduid, 'rec_type' => 2));

        return $nid;
    }

    /**
     * 账户合并
     * @param array $data
     * @return mixed
     */
    public function mergeData($data = array())
    {

        $uid        =   intval($data['uid']);       //  个人账户UID
        $com_uid    =   intval($data['com_uid']);   //  企业账户UID

        $mobile     =   intval($data['mobile']);    //  1-企业手机  2-个人手机
        $email      =   intval($data['email']);     //  1-企业邮箱  2-个人邮箱
        $QQ         =   intval($data['QQ']);        //  1-企业Q Q  2-个人Q Q
        $wx         =   intval($data['wx']);        //  1-企业微信  2-个人微信
        $sina       =   intval($data['sina']);      //  1-企业微博  2-个人微博

        /**
         * 判断身份真实性
         */
        $mFiled     =   "`uid`,`email`,`moblie`,`email_status`,`moblie_status`,`wxid`,`wxopenid`,`unionid`,`qqid`,`unionid`,`sinaid`";

        $com        =   $this->select_once('member', array('uid' => $com_uid, 'usertype' => 2), $mFiled);
        if (empty($com)){
            $return['msg']  =   '目标企业账户信息不存在';
        }
        $user       =   $this->select_once('member', array('uid' => $uid), $mFiled);
        if (empty($user)){
            $return['msg']  =   '当前个人用户账户信息不存在';
        }

        /**
         *  判断个人是否有其他身份：企业
         */
        $existCom   =   $this->select_num('company', array('uid' => $uid));
        
        if ((int)$existCom > 0 ){

            $return['msg']  =   '当前个人用户存在其他身份，不可进行合并';
        }

        /**
         * 判断企业是否存在个人身份
         */
        $existNum   =   $this->select_num('resume', array('uid' => $com_uid));
        if ((int)$existNum > 0){

            $return['msg']  =   '目标企业用户已存在个人身份，不可进行合并';
        }

        if (!isset($return['msg'])) {

            $rData['uid']                       =   $com_uid;

            /**
             * 执行数据合并
             */
            if ($mobile == 1) {

                $rData['telphone']              =   $com['moblie'];
                $rData['moblie_status']         =   $com['moblie_status'];
            } elseif ($mobile == 2) {

                $memberData['moblie']           =   $user['moblie'];
                $memberData['moblie_status']    =   $user['moblie_status'];

                $comData['linktel']             =   $user['moblie'];
                $comData['moblie_status']       =   $user['moblie_status'];


            }

            if ($email == 1) {

                $rData['email']                 =   $com['email'];
                $rData['email_status']          =   $com['email_status'];
            } elseif ($email == 2) {

                $memberData['email']            =   $user['email'];
                $memberData['email_status']     =   $user['email_status'];

                $comData['linkmail']            =   $user['email'];
                $comData['email_status']        =   $user['email_status'];

            }

            if ($QQ == 2) {

                $memberData['qqid']             =   $user['qqid'];
                $memberData['qqunionid']        =   $user['qqunionid'];
            }

            if ($wx == 2) {

                $memberData['wxid']             =   $user['qqid'];
                $memberData['wxopenid']         =   $user['qqunionid'];
                $memberData['unionid']          =   $user['unionid'];
            }

            if ($sina == 2) {

                $memberData['sinaid']           =   $user['sinaid'];
            }

            if (isset($memberData)){

                $this->update_once('member', $memberData, array('uid' => $com_uid));
            }
            if (isset($comData)){

                $this->update_once('company', $comData, array('uid' => $com_uid));
            }


            if (isset($rData)){
                $this->update_once('resume', $rData, array('uid' => $uid));
            }

            //  原有个人账号 相关表UID 替换为 合并后企业账号UID
            $result =   $this->filterTableUid($com_uid, $uid, 3);

            if ($result){

                //  原有个人账号 member 数据删除
                $this->delete_all('member', array('uid' => $uid));

                // 增加账号分离会员日志
                require_once('log.model.php');
                $LogM               =   new log_model($this->db, $this->def);
                $content            =   "合并账户：个人（UID：".$uid."） 合并到企业（UID：".$com_uid."）";
                $LogM->addAdminLog($content);

                $return['msg']      =   '账户合并成功！';
                $return['errcode']  =   9;

            }else{

                $return['msg']      =   '账户合并失败，请重试！';
            }
        }

        $return['errcode']          =   $return['errcode'] ? $return['errcode'] : 8;

        return $return;
    }

}

?>