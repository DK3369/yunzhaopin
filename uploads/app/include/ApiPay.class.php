<?php
/*
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

/**
 * @desc 合并所有支付API 支付成功逻辑处理函数（支付宝、财付通、微信、wap支付宝）
 * @method 支付处理函数
 * @param 回调参数
 * @param dingdan   系统订单ID
 * @param total_fee 支付金额
 *
 * @author 孔舒程 2015-12-08
 */

class ApiPay extends common
{

    function payAll($dingdan, $total_fee = '', $paytype = '')
    {


        //支付回调参数验证数据合法性
        if (!preg_match('/^[0-9]+$/', $dingdan)) {
            die;
        }

        //查询订单是否真实存在
        $orderM =   $this->MODEL('companyorder');
        $order  =   $orderM->getInfo(array('order_id' => $dingdan));

        $userinfoM  =   $this->MODEL('userinfo');
        $resumeM    =   $this->MODEL('resume');
        $comM       =   $this->MODEL('company');
        $jobM       =   $this->MODEL('job');
        $ratingM    =   $this->MODEL('rating');
        $partM      =   $this->MODEL('part');
        $downM      =   $this->MODEL('downresume');
        $logM       =   $this->MODEL('log');
        $statisM    =   $this->MODEL('statis');
        $integralM  =   $this->MODEL('integral');
        $warningM   =   $this->MODEL('warning');

        //判断订单状态是否未处理，只处理未付款的
        if ($order['order_state'] == '1' && $order['id']) {

            $uid        =   intval($order['uid']);
            $ratingId   =   intval($order['rating']);
            $orderid    =   $order['order_id'];
            $type       =   intval($order['type']);

            $tvalue     =   array();
            $usertype   =   intval($order['usertype']);

            $member     =   $userinfoM->getInfo(array('uid' => $uid), array('field' => '`username`,`usertype`,`wxid`'));

            $usertype_n =   '';

            if ($usertype == 1) {

                $marr       =   $resumeM->getResumeInfo(array('uid' => $uid), array('field' => '`name`,`email`,`telphone` as `moblie`'));
                $usertype_n =   '个人用户';

            } else if ($usertype == 2) {

                $tvalue['all_pay']  =   array('+', $order['order_price']);

                $marr       =   $comM->getInfo($uid, array('field' => '`name`,`crm_uid`,`linkmail` as `email`, `linktel` as moblie'));
                $usertype_n =   '企业用户';

            }

            // 短信、邮件通知参数
            $emaildata              =   array();
            $emaildata['type']      =   'recharge';
            $emaildata['username']  =   $member['username'];
            $emaildata['name']      =   $marr['name'];
            $emaildata['price']     =   $order['order_price'];
            $emaildata['time']      =   date("Y-m-d H:i:s");
            $emaildata['email']     =   $marr['email'];
            $emaildata['moblie']    =   $marr['moblie'];
            $emaildata['port']      =   $order['port'];

            //  微信通知参数
            $sendInfo               =   array();
            $sendInfo['wxid']       =   $member['wxid'];
            $sendInfo['first']      =   '您有一笔订单支付成功！';
            $sendInfo['username']   =   $member['username'];
            $sendInfo['order']      =   $orderid;
            $sendInfo['price']      =   $order['order_price'];
            $sendInfo['time']       =   date('Y-m-d H:i:s');
            $sendInfo['uid']        =   $uid;
            $sendInfo['usertype']   =   $member['usertype'];

            switch ($paytype) {

                case 'alipay':
                    $sendInfo['paytype'] = '支付宝支付';
                    break;
                case 'wapalipay':
                    $sendInfo['paytype'] = '支付宝手机支付';
                    break;
                case 'tenpay':
                    $sendInfo['paytype'] = '财付通支付';
                    break;
                case 'baidu':
                    $sendInfo['paytype'] = '百度闪付';
                    break;
                default :
                    $sendInfo['paytype'] = '其他支付';
                    break;
            }

            if ($type == 1 && $ratingId && $usertype != 1) {    //购买会员


                $value  =   $ratingM->ratingInfo($ratingId, $uid);
                $nid    =   $statisM->upInfo($value, array('uid' => $uid, 'usertype' => $usertype));

                if ($nid) {

                    if (isset($value['integral'])) {

                        $addJF  =   $value['integral'][1];
                        $integralM->insert_company_pay($addJF, 2, $uid, $order['usertype'], '开通会员赠送积分', 1, 2, true);
                    }

                    $order_info =   unserialize($order['order_info']);

                    if ($order_info['vip_integral'] && $order['integral']) { // 充值积分购买会员

                        $tvalue['integral'] =   array('+', $order['integral']);

                        $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                        $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，购买会员', 1, 2, true);
                        $statisM->upInfo(array('integral' => array('-', $order_info['vip_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                        $integralM->insert_company_pay($order_info['vip_integral'], 2, $uid, $order['usertype'], "购买会员，扣除" . $this->config['integral_pricename'], 1, 2, false);
                        $warningM->warning(4, $uid);
                    }

                    $logContent = $sendInfo['paytype'] . $value['rating_name'] . '购买成功，价格:' . $order['order_price'] . '元，' . '订单编号：' . $order['id'];
                    $logContent .= $order['order_dkjf'] > 0 ? '，使用抵扣' . $this->config['integral_pricename'] . $order['order_dkjf'] : '';
                    $logM->addMemberLog($uid, $usertype, '财务订单：购买会员', 88, 1, $logContent);
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '购买：'.$value['rating_name'];
            } else if ($type == 2 && $order['integral']) {  //充值积分

                $tvalue['integral'] =   array('+', $order['integral']);

                $nid                =   $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));

                if ($nid) {
                    $warningM->warning(4, $uid);
                    $logContent =   $sendInfo['paytype'] . '充值' . $this->config['integral_pricename'] . $order['integral'] . '成功';
                    $logContent .=  $order['order_dkjf'] > 0 ? '，使用抵扣' . $this->config['integral_pricename'] . $order['order_dkjf'] : '';
                    $logM->addMemberLog($uid, $usertype, '财务订单：充值' . $this->config['integral_pricename'], 88, 1, $logContent);
                }
                $sendMail           =   1;
                $sendInfo['info']   =   '充值'.$this->config['integral_pricename'].'：'.$order['integral'];
            } else if ($type == 5) {    //购买增值包

                $row = $ratingM->getComSerDetailInfo($ratingId);
                $rinfo=$ratingM->getComServiceInfo(array('id'=>$row['type']),array('field'=>'`name`'));

                $value['job_num'] = array('+', intval($row['job_num']));
                $value['breakjob_num'] = array('+', intval($row['breakjob_num']));
                $value['down_resume'] = array('+', intval($row['resume']));
                $value['invite_resume'] = array('+', intval($row['interview']));
                $value['zph_num'] = array('+', intval($row['zph_num']));
                $value['top_num'] = array('+', intval($row['top_num']));
                $value['rec_num'] = array('+', intval($row['rec_num']));
                $value['urgent_num'] = array('+', intval($row['urgent_num']));

                $nid    =   $statisM->upInfo($value, array('uid' => $uid, 'usertype' => $usertype));

                if ($nid) {

                    $order_info = unserialize($order['order_info']);

                    if ($order_info['pack_integral'] && $order['integral']) { // 充值积分购买增值服务

                        $tvalue['integral'] = array('+', $order['integral']);

                        $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                        $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，购买增值服务', 1, 2, true);
                        $statisM->upInfo(array('integral' => array('-', $order_info['pack_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                        $integralM->insert_company_pay($order_info['pack_integral'], 2, $uid, $order['usertype'], "购买增值服务，扣除" . $this->config['integral_pricename'], 1, 2, false);
                        $warningM->warning(4, $uid);
                    }
                    $logContent = $sendInfo['paytype'] . '增值服务购买成功';
                    $logContent .= $order['order_dkjf'] > 0 ? '，使用抵扣' . $this->config['integral_pricename'] . $order['order_dkjf'] : '';
                    $logM->addMemberLog($uid, $usertype, '财务订单：购买增值服务', 88, 1, $logContent);
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '购买增值包：'.$rinfo['name'];
            } else if ($order['type'] == 10) {    //职位置顶
                /**
                 * 购买置顶职位，付款成功后续操作，
                 * @var jobid days price
                 */
                $order_info =   unserialize($order['order_info']);

                $jobname    =   '';

                if ($order_info['jobid']) {

                    //查询该职位
                    $xsJob  =   $jobM->getInfo(array('id' => intval($order_info['jobid'])), array('field' => '`id`,`name`,`xsdate`'));

                    if (!empty($xsJob)) {

                        $jobname=   '，置顶职位' . $xsJob['name'] . $order_info['days'] . '天';
                        $xsdate =   $xsJob['xsdate'] > time() ? array('+', $order_info['days'] * 86400) : strtotime('+' . $order_info['days'] . ' day');

                        $nid    =   $jobM->upInfo(array('xsdate' => $xsdate), array('uid' => $uid, 'id' => intval($order_info['jobid'])));

                        if ($order_info['jobzd_integral'] && $order['integral']) { // 充值积分购买职位置顶

                            $tvalue['integral'] =   array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，购买职位置顶', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['jobzd_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['jobzd_integral'], 2, $uid, $order['usertype'], "职位置顶，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $logM->addMemberLog($uid, $usertype, '财务订单：购买职位置顶', 88, 1, $sendInfo['paytype'].'购买职位置顶'.$jobname);
                    }
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '职位置顶';
                $wxtempMsg          =   $usertype_n.$marr['name'].$sendInfo['paytype'].'购买职位置顶'.$jobname."，实际支付".$order['order_price']."元,订单编号：".$order['id'];
            } else if ($type == 11) {   //职位紧急
                /**
                 * 购买紧急招聘，付款成功后续操作，
                 * @var jobid days price
                 */
                $order_info =   unserialize($order['order_info']);

                $jobname    =   '';

                if ($order_info['jobid']) {

                    //查询该职位
                    $uJob   =   $jobM->getInfo(array('id' => intval($order_info['jobid'])), array('where' => array('uid' => $uid), 'field' => '`id`,`name`, `urgent_time`'));

                    if (!empty($uJob)) {

                        $jobname    =   '，设置职位'.$uJob['name'].'紧急'.$order_info['days'].'天';
                        $urgent_time=   $uJob['urgent_time'] > time() ? array('+', $order_info['days'] * 86400) : strtotime('+' . $order_info['days'] . ' day');

                        $nid        =   $jobM->upInfo(array('urgent_time' => $urgent_time, 'urgent' => '1'), array('uid' => $uid, 'id' => intval($order_info['jobid'])));

                        if ($order_info['joburgent_integral'] && $order['integral']) { // 充值积分购买紧急招聘

                            $tvalue['integral'] = array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '购买职位紧急招聘', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['joburgent_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['joburgent_integral'], 2, $uid, $order['usertype'], "职位紧急招聘，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $logM->addMemberLog($uid, $usertype, '财务订单：购买职位紧急招聘', 88, 1, $sendInfo['paytype'].'购买职位紧急招聘'.$jobname);
                    }
                }
                $sendMail           =   1;
                $sendInfo['info']   =   '职位紧急';
                $wxtempMsg          =   $usertype_n.$marr['name'].$sendInfo['paytype'].'购买职位紧急'.$jobname."，实际支付".$order['order_price']."元,订单编号：".$order['id'];
            } else if ($type == 12) {   //职位推荐

                /**
                 * 购买职位推荐，付款成功后续操作，
                 * @var jobid days price
                 */
                $order_info =   unserialize($order['order_info']);

                $jobname    =   '';

                if ($order_info['jobid']) {

                    //查询该职位
                    $recJob =   $jobM->getInfo(array('id' => intval($order_info['jobid']), 'uid' => $uid), array('field' => '`id`,`name`,`rec_time`'));

                    if (!empty($recJob)) {

                        $jobname    =   '，推荐职位：' . $recJob['name'] . $order_info['days'] . '天';
                        $rec_time   =   $recJob['rec_time'] > time() ? array('+', $order_info['days']) : strtotime('+' . $order_info['days'] . ' day');

                        $nid        =   $jobM->upInfo(array('rec_time' => $rec_time, 'rec' => '1'), array('uid' => $uid, 'id' => intval($order_info['jobid'])));

                        if ($order_info['jobrec_integral'] && $order['integral']) { // 充值积分购买职位推荐

                            $tvalue['integral'] = array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，购买职位推荐', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['jobrec_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['jobrec_integral'], 2, $uid, $order['usertype'], "职位推荐，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $logM->addMemberLog($uid, $usertype, '财务订单：购买职位推荐', 88, 1, $sendInfo['paytype'].'购买职位推荐'.$jobname);
                    }
                }
                $sendMail           =   1;
                $sendInfo['info']   =   '职位推荐';
                $wxtempMsg          =   $usertype_n.$marr['name'].$sendInfo['paytype'].'购买职位推荐'.$jobname."，实际支付".$order['order_price']."元,订单编号：".$order['id'];
            } else if ($type == 13) {   //职位自动刷新
                /**
                 * 购买自动刷新，付款成功后续操作，
                 * @var jobautoids days price
                 */
                $order_info =   unserialize($order['order_info']);

                if ($order_info['jobid']) {

                    //查询该职位
                    $ListA  =   $jobM->getList(array('uid' => $uid, 'id' => array('in', $order_info['jobid'])), array('field' => '`id`,`autotime`'));
                    $autoJob=   $ListA['list'];

                    if (!empty($autoJob)) {

                        $lastautotime   =   0;
                        foreach ($autoJob as $v) {

                            $autotime   =   $v['autotime'] >= time() ? array('+', $order_info['days'] * 86400) : strtotime('+' . $order_info['days'] . ' day');

                            if ($autotime > $lastautotime) {

                                $lastautotime   =   $autotime;
                            }

                            $nid    =   $jobM->upInfo(array('autotime' => $autotime), array('uid' => $uid, 'id' => $v['id']));
                        }

                        if ($order_info['auto_integral'] && $order['integral']) { // 充值积分购买自动刷新

                            $tvalue['integral'] =   array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => 2));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, 2, "充值" . $this->config['integral_pricename'] . '，购买职位自动刷新', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['auto_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['auto_integral'], 2, $uid, $order['usertype'], "职位自动刷新，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $logM->addMemberLog($uid, $usertype, '财务订单：职位自动刷新', 88, 1, $sendInfo['paytype'].'购买职位自动刷新');
                    }
                }
                $sendMail           =   1;
                $sendInfo['info']   =   '职位刷新';
            } else if ($type == 14) {   //简历置顶

                /**
                 * 购买简历置顶，付款成功后续操作，
                 * @var resumeid days price
                 */
                $order_info =   unserialize($order['order_info']);

                $orderMsg   =   '';

                if ($order_info['resumeid']) {

                    //查询该简历
                    $zdResume   =   $resumeM->getExpect(array('id' => intval($order_info['resumeid']), 'uid' => $uid), array('field' => '`id`,`name`,`topdate`'));

                    if (!empty($zdResume)) {

                        $topdate    =   $zdResume['topdate'] > time() ? array('+', $order_info['days'] * 86400) : strtotime('+' . $order_info['days'] . ' day');
                        $nid        =   $resumeM->upInfo(array('id' => intval($order_info['resumeid'])), array('eData' => array('topdate' => $topdate, 'top' => '1')));
                        $orderMsg   =   '，置顶简历' . $zdResume['name'] . $order_info['days'] . '天';
                        $logM->addMemberLog($uid, $usertype, '财务订单：购买简历置顶', 88, 1, $sendInfo['paytype'].'购买简历置顶'.$orderMsg);
                    }
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '简历置顶';
            }  else if ($type == 16) {   //职位刷新

                $order_info =   unserialize($order['order_info']);

                if ($order_info['jobid']) {

                    //查询该职位
                    $sxJob  =   $jobM->getList(array('uid' => $uid, 'id' => array('in', $order_info['jobid'])), array('field' => '`lastupdate`,`id`'));

                    if (!empty($sxJob)) {

                        $nid    =   $jobM->upInfo(array('lastupdate' => time()), array('id' => array('in', pylode(',', explode(',', $order_info['jobid'])))));

                        if ($order_info['sxjob_integral'] && $order['integral']) {

                            $tvalue['integral'] = array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '， 购买职位刷新', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['sxjob_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['sxjob_integral'], 2, $uid, $order['usertype'], "职位刷新，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $comM->upInfo($uid, '', array('lastupdate' => time()));
                        $logM->addMemberLog($uid, $usertype, '财务订单：购买职位刷新', 88, 1, $sendInfo['paytype']."购买职位刷新");
                    }
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '职位刷新';
            } else if ($type == 17) {   //兼职刷新

                $order_info = unserialize($order['order_info']);

                if ($order_info['jobid']) {

                    $sxPart =   $partM->getList(array('uid' => $uid, 'id' => array('in', $order_info['jobid'])), array('field' => '`id`,`lastupdate`'));

                    if (!empty($sxPart)) {

                        $nid    =   $partM->upInfo(array('lastupdate' => time()), array('id' => array('in', pylode(',', explode(',', $order_info['jobid'])))));

                        if ($order_info['sxpart_integral'] && $order['integral']) {

                            $tvalue['integral'] = array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '， 购买兼职刷新', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['sxpart_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['sxpart_integral'], 2, $uid, $order['usertype'], "兼职刷新，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $logM->addMemberLog($uid, $usertype, '财务订单：购买兼职刷新', 88, 1, $sendInfo['paytype'].'购买兼职刷新');
                    }
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '兼职刷新';
            } else if ($type == 19) {   //下载简历

                $order_info =   unserialize($order['order_info']);

                if ($order_info['eid']) {

                    $eid    =   intval($order_info['eid']);
                    $expect =   $resumeM->getExpect(array('id' => $eid), array('field' => '`id`,`uid`,`name`,`uname`,`height_status`'));

                    if ($expect) {

                        $dData  =   array(

                            'eid' => intval($expect['id']),
                            'uid' => intval($expect['uid']),
                            'comid' => intval($order_info['uid']),
                            'usertype' => intval($usertype),
                            'type' => intval($expect['height_status']),
                            'downtime' => time(),
                            'did' => $this->config['did']
                        );

                        $isDown =   $downM->getDownResumeInfo(array('eid' => $eid, 'comid' => $order_info['uid'], 'usertype' => $usertype));

                        if (empty($isDown)) {

                            $orderMsg   =   '，下载简历：' . $expect['name'];

                            $nid        =   $downM->addInfo($dData);

                            $resumeM->upInfo(array('id' => $eid), array('eData' => array('dnum' => array('+', '1'))));

                            if ($order_info['resume_integral'] && $order['integral']) { // 充值积分下载简历

                                $tvalue['integral'] = array('+', $order['integral']);

                                $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                                $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，下载简历', 1, 2, true);
                                $statisM->upInfo(array('integral' => array('-', $order_info['resume_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                                $integralM->insert_company_pay($order_info['resume_integral'], 2, $uid, $order['usertype'], "下载简历，扣除" . $this->config['integral_pricename'], 1, 2, false);
                                $warningM->warning(4, $uid);
                            }

                            $logM->addMemberLog($uid, $usertype, '财务订单：购买下载简历：', 88, 1, $sendInfo['paytype'] . '购买简历下载'.$orderMsg);

                            $sendMail           =   1;
                            $sendInfo['info']   =   '下载简历';
                        } else {

                            $this->obj->update_once('company_order', array('order_state' => '4', 'order_remark' => '简历（ID:' . $expect['id'] . '）您已经下载过，关闭无效交易订单！'), array('id' => $order['id']));
                        }
                    }
                }
            } else if ($type == 20) {   //上架职位

                $msg        =   '购买上架职位数';
                $order_info =   unserialize($order['order_info']);

                if ($usertype == 2) {
                    $jobnum =   array('+', 1);
                }

                $nid        =   $statisM->upInfo(array('job_num' => $jobnum), array('uid' => $uid, 'usertype' => $usertype));

                if ($order_info['issue_integral'] && $order['integral']) { // 充值积分购买上架职位

                    $tvalue['integral'] = array('+', $order['integral']);

                    $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                    $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，' . $msg, 1, 2, true);
                    $statisM->upInfo(array('integral' => array('-', $order_info['issue_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                    $integralM->insert_company_pay($order_info['issue_integral'], 2, $uid, $order['usertype'], $msg . "，扣除" . $this->config['integral_pricename'], 1, 2, false);
                    $warningM->warning(4, $uid);
                }

                $logM->addMemberLog($uid, $usertype, '财务订单：职位上架套餐：', 88, 1, $sendInfo['paytype'].$msg);

                $sendMail           =   1;
                $sendInfo['info']   =   $msg;
            }   else if ($type == 23) {   //面试邀请

                $order_info =   unserialize($order['order_info']);

                if ($usertype == 2) {
                    $inviteNum  =   array('+', 1);
                }

                $nid    =   $statisM->upInfo(array('invite_resume' => $inviteNum), array('uid' => $uid, 'usertype' => $usertype));

                if ($order_info['invite_integral'] && $order['integral']) {

                    $tvalue['integral'] = array('+', $order['integral']);

                    $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                    $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，购买面试邀请', 1, 2, true);
                    $statisM->upInfo(array('integral' => array('-', $order_info['invite_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                    $integralM->insert_company_pay($order_info['invite_integral'], 2, $uid, $order['usertype'], "面试邀请，扣除" . $this->config['integral_pricename'], 1, 2, false);
                    $warningM->warning(4, $uid);
                }

                $logM->addMemberLog($uid, $usertype, '财务订单：职位上架套餐：', 88, 1, $sendInfo['paytype'].'购买面试邀请次数');

                $sendMail           =   1;
                $sendInfo['info']   =   '购买面试邀请';
            } else if ($type == 24) {   //兼职推荐

                $order_info =   unserialize($order['order_info']);

                if ($order_info['jobid']) {

                    $partA  =   $partM->getInfo(array('id' => intval($order_info['jobid'])), array('field' => '`id`,`name`,`rec_time`'));
                    $recJob =   $partA['info'];

                    if (!empty($recJob)) {

                        $rec_time   =   $recJob['rec_time'] > time() ? array('+', $order_info['days'] * 86400) : strtotime('+' . $order_info['days'] . ' day');

                        $nid        =   $partM->upInfo(array('rec_time' => $rec_time), array('id' => intval($order_info['jobid']), 'uid' => $uid));

                        $orderMsg   =   '，推荐兼职'.$recJob['name'].$order_info['days'].'天';

                        if ($order_info['recpart_integral'] && $order['integral']) {

                            $tvalue['integral'] = array('+', $order['integral']);

                            $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，购买兼职推荐', 1, 2, true);
                            $statisM->upInfo(array('integral' => array('-', $order_info['recpart_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                            $integralM->insert_company_pay($order_info['recpart_integral'], 2, $uid, $order['usertype'], "推荐兼职，扣除" . $this->config['integral_pricename'], 1, 2, false);
                            $warningM->warning(4, $uid);
                        }

                        $logM->addMemberLog($uid, $usertype, '财务订单：购买职位推荐', 88, 1, $sendInfo['paytype'] . '购买推荐套餐；'.$orderMsg);
                    }
                }
                $sendMail           =   1;
                $sendInfo['info']   =   '兼职推荐';
            } else if ($type == 25) {   //店铺招聘

                if ($order['once_id']) {

                    $onceM  =   $this->MODEL('once');
                    $once   =   $onceM->getOnceInfo(array('id' => intval($order['once_id'])), array('field' => '`pay`'));

                    if (!empty($once)) {

                        $nid    =   $onceM->upOnce(array('pay' => '2'), array('id' => intval($order['once_id'])));
                    }
                    $orderMsg   =   '，店铺id('.$order['once_id'].')';
                }
            } else if ($type == 26) {   //购买广告位

                if ($order['order_id']) {

                    $adM    =   $this->MODEL('ad');
                    $ad     =   $adM->getAdOrderInfo(array('order_id' => $orderid), array('field' => '`aid`,`price`'));
                    if (!empty($ad)) {

                        $orderMsg   =   '，广告位id(' . $ad['aid'] . ')';
                        $nid        =   $adM->upOrderAd(array('order_id' => $orderid), array('order_state' => '2'));

                        $logM->addMemberLog($uid, $usertype, '财务订单：购买广告位', 88, 1, $sendInfo['paytype'].'购买广告位'.$orderMsg);
                    }
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '购买广告位';
            } else if ($type == 28) {   //招聘会报名

                $order_info =   unserialize($order['order_info']);

                if ($order_info['zid']) {

                    $zid    =   intval($order_info['zid']);
                    $bmData =   array(
                        'comid' => $order_info['uid'],
                        'zphid' => $zid,
                        'bid' => $order_info['bid'],
                        'sid' => $order_info['sid'],
                        'cid' => $order_info['cid'],
                        'ctime' => time(),
                        'status' => 0,
                        'price' => $order['order_price'],
                        'com_name' => $order_info['com_name'],
                        'jobid' => $order_info['jobid']
                    );
                    $zphM   =   $this->MODEL('zph');
                    $zphCom =   $zphM->getZphComInfo(array('uid' => $uid, 'zid' => $zid, 'sid' => $order_info['sid'], 'cid' => $order_info['cid'], 'bid' => $order_info['bid']));

                    if (empty($zphCom)) {

                        $nid    =   $zphM->addZCom($bmData);

                        if ($nid) {

                            if ($order_info['zph_integral'] && $order['integral']) {

                                $tvalue['integral'] = array('+', $order['integral']);

                                $statisM->upInfo($tvalue, array('uid' => $uid, 'usertype' => $usertype));
                                $integralM->insert_company_pay($order['integral'], 2, $uid, $order['usertype'], "充值" . $this->config['integral_pricename'] . '，预定招聘会', 1, 2, true);
                                $statisM->upInfo(array('integral' => array('-', $order_info['zph_integral'])), array('uid' => $uid, 'usertype' => $usertype));
                                $integralM->insert_company_pay($order_info['zph_integral'], 2, $uid, $order['usertype'], "预定招聘会，扣除" . $this->config['integral_pricename'], 1, 2, false);
                                $warningM->warning(4, $uid);
                            }

                            $orderMsg   =   '，招聘会id('.$zid.')';

                            $logM->addMemberLog($uid, $usertype, '财务订单：报名招聘会：', 88, 1, $sendInfo['paytype'].'购买报名招聘会次数'.$orderMsg);
                        }
                    }
                }

                $sendMail           =   1;
                $sendInfo['info']   =   '报名招聘会';
            }

            if ($nid) {

                $order_remark = !empty($order['order_remark']) ? $order['order_remark'] . '。' : '';
                $order_remark .= $sendInfo['paytype'];
                $orderM->upInfo($order['id'], array('order_state' => '2', 'order_remark' => $order_remark));

                if ($sendMail == 1) {
                    $notice = $this->MODEL('notice');
                    $notice->sendEmailType($emaildata);
                    $notice->sendSMSType($emaildata);
                }
                if ($order['type'] == '2') {

                    $integralM  =   $this->MODEL('integral');
                    $integralM->insert_company_pay($order['integral'], 2, $order['uid'], $order['usertype'], "购买" . $this->config['integral_pricename'], 1, 2, true);
                }
                return 2;
            }
        } else {

            return $order['order_state'];
        }
    }

    function getOrder($id)
    {
        if (!preg_match('/^[0-9]+$/', $id)) {

            return array();
        } else {

            $orderM =   $this->MODEL('companyorder');
            $order  =   $orderM->getInfo(array('id' => $id));
            return $order;
        }
    }
}

?>