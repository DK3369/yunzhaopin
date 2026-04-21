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

class yingxiao_tuiguang_controller extends adminCommon
{
    // 邮件推广
    function index_action()
    {
        $emailM = $this->MODEL('email');

        $anniversary = $emailM->getEmsgOnce(array('title' => '网站周年', 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $todaydue = $emailM->getEmsgOnce(array('title' => array('like', '会员套餐还有1天将过期'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $sevenduew['title'][] = array('like', '会员套餐还有');
        $sevenduew['title'][] = array('like', '天将过期');
        $sevenduew['orderby'] = 'ctime,desc';

        $sevendue = $emailM->getEmsgOnce($sevenduew, array('field' => '`ctime`'));

        $useradd = $emailM->getEmsgOnce(array('title' => array('like', '未发布简历'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $userup = $emailM->getEmsgOnce(array('title' => array('like', '今天简历刷新'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $addjob = $emailM->getEmsgOnce(array('title' => array('like', '未发布职位'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $upjob = $emailM->getEmsgOnce(array('title' => array('like', '未刷新职位'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $this->render_json(0, 'ok', array(
            'anniversary' => $anniversary,
            'todaydue' => $todaydue,
            'sevendue' => $sevendue,
            'useradd' => $useradd,
            'userup' => $userup,
            'addjob' => $addjob,
            'upjob' => $upjob
        ));
    }

    // 短信推广
    function msgtg_action()
    {
        $mobliemsgM = $this->MODEL('mobliemsg');

        $anniversary = $mobliemsgM->getInfo(array('content' => array('like', '周年庆'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $todaydue = $mobliemsgM->getInfo(array('content' => array('like', '将于1天后到期'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $sevendue = $mobliemsgM->getInfo(array('content' => array('like', '将于7天后到期'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $useradd = $mobliemsgM->getInfo(array('content' => array('like', '未发布简历'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $userup = $mobliemsgM->getInfo(array('content' => array('like', '未刷新简历'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $addjob = $mobliemsgM->getInfo(array('content' => array('like', '未发布职位'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $upjob = $mobliemsgM->getInfo(array('content' => array('like', '未刷新职位'), 'orderby' => 'ctime,desc'), array('field' => '`ctime`'));

        $this->render_json(0, 'ok', array(
            'anniversary' => $anniversary,
            'todaydue' => $todaydue,
            'sevendue' => $sevendue,
            'useradd' => $useradd,
            'userup' => $userup,
            'addjob' => $addjob,
            'upjob' => $upjob
        ));
    }

    // 周年数据获取
    function getBirthday_action()
    {
        $jobM = $this->MODEL('job');

        $statisM = $this->MODEL('statis');

        $resumeM = $this->MODEL('resume');

        $companyM = $this->MODEL('company');

        $userinfoM = $this->MODEL('userinfo');

        $todayduewh['vip_etime'][] = array('>', time());

        $todayduewh['vip_etime'][] = array('<', strtotime('+1 day'));

        $todaydue = $statisM->getList($todayduewh, array('field' => 'uid', 'usertype' => '2'));
        foreach ($todaydue as $v) {
            $todayuid[] = $v['uid'];
        }
        $sevenduewh['vip_etime'][] = array('>', time());

        $sevenduewh['vip_etime'][] = array('<', strtotime('+7 day'));

        $sevendue = $statisM->getList($sevenduewh, array('field' => 'uid', 'usertype' => '2'));
        foreach ($sevendue as $v) {
            $sevenuid[] = $v['uid'];
        }
        /*
         * 当前日期前第七天注册
         */
        $regStart = strtotime(date('Y-m-d 00:00:00', time() - 7 * 86400));
        $regEnd = strtotime(date('Y-m-d 23:59:59', time() - 7 * 86400));

        $regWhere['PHPYUNBTWSTART_A'] = '';
        $regWhere['reg_date'][] = array('>=', $regStart);
        $regWhere['reg_date'][] = array('<=', $regEnd);
        $regWhere['PHPYUNBTWEND_A'] = '';
        $regs = $userinfoM->getList($regWhere, array('field' => 'uid'));
        foreach ($regs as $k => $v) {
            $uids[] = $v['uid'];// 当前日期前第七天注册的uid
        }

        $upjobA = $jobM->getList(array('lastupdate' => array('<', strtotime('-7 day')), 'r_status' => array('<>', '2'), 'groupby' => '`uid`', 'orderby' => 'lastupdate,desc'), array('field' => '`uid`'));
        $upjob = $upjobA['list'];
        $upuid = array();
        foreach ($upjob as $v) {
            $upuid[$v['uid']] = $v['uid'];
        }

        $upjobArr = $jobM->getList(array('lastupdate' => array('>=', strtotime('-7 day')), 'uid' => array('in', pylode(',', $upuid))), array('field' => '`uid`'));
        $upjobs = $upjobArr['list'];
        $upuids = array();
        foreach ($upjobs as $uv) {
            $upuids[$uv['uid']] = $uv['uid'];
        }
        $newUpuids = array_diff($upuid, $upuids);

        if ($_POST['type'] == 'email') {

            $num['anniversary_e'] = $userinfoM->getMemberNum(array('email' => array('<>', ''), 'status' => '1'));

            $num['todaydue_e'] = $companyM->getCompanyNum(array('email_status' => '1', 'linkmail' => array('<>', ''), 'r_status' => array('<>', '2'), 'uid' => array('in', pylode(',', $todayuid))));

            $num['sevendue_e'] = $companyM->getCompanyNum(array('email_status' => '1', 'linkmail' => array('<>', ''), 'r_status' => array('<>', '2'), 'uid' => array('in', pylode(',', $sevenuid))));

            $num['useradd_e'] = $resumeM->getResumeNum(array('email_status' => '1', 'r_status' => 1, 'def_job' => '0', 'resumetime' => '0', 'email' => array('<>', ''), 'uid' => array('in', pylode(',', $uids))));

            $num['userup_e'] = $resumeM->getResumeNum(array('email_status' => '1', 'def_job' => array('>', '0'), 'r_status' => 1, 'email' => array('<>', ''), 'lastupdate' => array('<', strtotime('-7 day'))));
            if ($uids) {// 当日前第七天有新注册的用户
                $addjob_ewh['email_status'] = '1';

                $addjob_ewh['jobtime'] = 0;
                $addjob_ewh['uid'] = array('in', pylode(',', $uids));

                $addjob_ewh['r_status'] = array('<>', '2');

                $addjob_ewh['linkmail'] = array('<>', '');
                $num['addjob_e'] = $companyM->getCompanyNum($addjob_ewh);
            } else {
                $num['addjob_e'] = 0;
            }
            $num['upjob_e'] = $companyM->getCompanyNum(array('email_status' => '1', 'linkmail' => array('<>', ''), 'r_status' => array('<>', '2'), 'uid' => array('in', pylode(',', $newUpuids))));
        } else {

            $num['anniversary_m'] = $userinfoM->getMemberNum(array('moblie' => array('<>', ''), 'status' => '1'));

            $num['todaydue_m'] = $companyM->getCompanyNum(array('moblie_status' => '1', 'linktel' => array('<>', ''), 'r_status' => array('<>', '2'), 'uid' => array('in', pylode(',', $todayuid))));

            $num['sevendue_m'] = $companyM->getCompanyNum(array('moblie_status' => '1', 'linktel' => array('<>', ''), 'r_status' => array('<>', '2'), 'uid' => array('in', pylode(',', $sevenuid))));

            $num['useradd_m'] = $resumeM->getResumeNum(array('moblie_status' => '1', 'r_status' => 1, 'def_job' => '0', 'resumetime' => '0', 'telphone' => array('<>', ''), 'uid' => array('in', pylode(',', $uids))));

            $num['userup_m'] = $resumeM->getResumeNum(array('moblie_status' => '1', 'def_job' => array('>', '0'), 'r_status' => 1, 'telphone' => array('<>', ''), 'lastupdate' => array('<', strtotime('-7 day'))));
            if ($uids) {// 当日前第七天有新注册的用户
                $addjob_mwh['moblie_status'] = '1';
                $addjob_mwh['jobtime'] = 0;
                $addjob_mwh['uid'] = array('in', pylode(',', $uids));
                $addjob_mwh['r_status'] = array('<>', '2');
                $addjob_mwh['linktel'] = array('<>', '');
                $num['addjob_m'] = $companyM->getCompanyNum($addjob_mwh);
            } else {
                $num['addjob_m'] = 0;
            }
            $num['upjob_m'] = $companyM->getCompanyNum(array('moblie_status' => '1', 'linktel' => array('<>', ''), 'r_status' => array('<>', '2'), 'uid' => array('in', pylode(',', $newUpuids))));
        }

        $this->render_json(0, 'ok', $num);
    }

    // 导出
    function xls_action()
    {
        $jobM = $this->MODEL('job');
        $statisM = $this->MODEL('statis');
        $resumeM = $this->MODEL('resume');
        $companyM = $this->MODEL('company');
        $userinfoM = $this->MODEL('userinfo');

        $userinfo = array();

        $xls_type = $_POST['xls_type'];

        if ($_POST['outtype'] == 'birthday') {
            // 废弃生日通知
            // if($xls_type=='email'){
            // 	$rows	=	$resumeM->getResumeList(array('birthday'=>array('dateformat','%m%d'),'email'=>array('<>',''), 'email_status'=>'1'),array('field'=>'`email`,`uid`,`birthday`,`name`'));
            // }else{
            // 	$rows	=	$resumeM->getResumeList(array('birthday'=>array('dateformat','%m%d'),'telphone'=>array('<>',''), 'moblie_status'=>'1'),array('field'=>'`uid`,`birthday`,`name`,`telphone` as moblie'));
            // }
            // if($rows&&is_array($rows)){
            //
            //     foreach($rows as $k=>$v){
            //
            //         $userinfo[$v['uid']]=$v;
            //
            //     }
            //
            // }
        } else if ($_POST['outtype'] == 'anniversary') {
            if ($xls_type == 'email') {
                $rows = $userinfoM->getList(array('email' => array('<>', ''), 'status' => '1'), array('field' => '`uid`,`username`,`email`'));
            } else {
                $rows = $userinfoM->getList(array('moblie' => array('<>', ''), 'status' => '1'), array('field' => '`uid`,`username`,`moblie`'));
            }
            if ($rows && is_array($rows)) {
                foreach ($rows as $k => $v) {
                    $userinfo[$v['uid']] = $v;
                }
            }
        } else if ($_POST['outtype'] == 'todaydue' || $_POST['outtype'] == 'sevendue') {
            $dayslimit = $_POST['outtype'] == 'sevendue' ? 7 : 1;

            $statiswh['vip_etime'][] = array('>', time());

            $statiswh['vip_etime'][] = array('<', strtotime('+' . $dayslimit . ' day'));

            $comstatis = $statisM->getList($statiswh, array('field' => '`uid`,`vip_etime`,`rating_name`', 'usertype' => '2'));
            if (is_array($comstatis)) {
                foreach ($comstatis as $key => $value) {
                    $uid[] = $value['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uid));

                $where['name'] = array('<>', '');

                if ($xls_type == 'email') {
                    $where['linkmail'] = array('<>', '');

                    $where['email_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linkmail` as email'));
                } else {
                    $where['linktel'] = array('<>', '');

                    $where['moblie_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linktel` as moblie'));
                }

            }
            if ($companys && is_array($companys)) {
                foreach ($companys as $k => $v) {
                    $userinfo[$v['uid']] = $v;
                }
            }
        } else if ($_POST['outtype'] == 'useradd') {
            $dayslimit = 7;

            $regs = $userinfoM->getList(array('reg_date' => array('<', strtotime('-' . $dayslimit . ' day')), 'usertype' => '1'), array('field' => 'uid,reg_date,username'));
            foreach ($regs as $k => $v) {
                $uids[] = $v['uid'];
            }
            $where['r_status'] = 1;

            $where['def_job'] = '0';

            $where['resumetime'] = '0';

            $where['uid'] = array('in', pylode(',', $uids));

            if ($xls_type == 'email') {
                $where['email'] = array('<>', '');

                $where['email_status'] = '1';

                $users = $resumeM->getResumeList($where, array('field' => '`uid`,`name`,`email`,`resumetime`'));
            } else {
                $where['telphone'] = array('<>', '');

                $where['moblie_status'] = '1';

                $users = $resumeM->getResumeList($where, array('field' => '`uid`,`name`,`resumetime`,`telphone` as `moblie`'));
            }
            foreach ($users as $k => $v) {

                $userinfo[$v['uid']] = $v;
            }
        } else if ($_POST['outtype'] == 'userup') {
            $where['def_job'] = array('>', '0');

            $where['r_status'] = 1;

            $where['lastupdate'] = array('<', strtotime('-7 day'));

            if ($xls_type == 'email') {
                $where['email'] = array('<>', '');

                $where['email_status'] = '1';

                $resumes = $resumeM->getResumeList($where, array('field' => 'distinct `uid`,`name`,`email`,`lastupdate`'));
            } else {
                $where['telphone'] = array('<>', '');

                $where['moblie_status'] = '1';

                $resumes = $resumeM->getResumeList($where, array('field' => 'distinct `uid`,`name`,`lastupdate`,`telphone` as `moblie`'));
            }
            foreach ($resumes as $k => $v) {
                $userinfo[$v['uid']] = $v;
            }
        } else if ($_POST['outtype'] == 'addjob') {
            $where['jobtime'][] = array('>', '1');

            $where['jobtime'][] = array('<', strtotime('-7 day'));

            $where['r_status'] = array('<>', '2');

            if ($xls_type == 'email') {
                $where['linkmail'] = array('<>', '');

                $where['email_status'] = '1';

                $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`jobtime`,`linkmail` as `email`,`linktel` as `moblie`'));
            } else {
                $where['linktel'] = array('<>', '');

                $where['moblie_status'] = '1';

                $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`jobtime`,`linktel` as `moblie`'));
            }
            foreach ($companys as $k => $v) {
                $userinfo[$v['uid']] = $v;
            }
        } else if ($_POST['outtype'] == 'upjob') {
            $comjobsA = $jobM->getList(array('lastupdate' => array('<', strtotime('-7 day')), 'r_status' => array('<>', '2'), 'groupby' => 'uid', 'orderby' => 'lastupdate,desc'), array('field' => '`uid`,`lastupdate`'));
            $comjobs = $comjobsA['list'];
            $comids = array();
            foreach ($comjobs as $k => $v) {
                $comids[$v['uid']] = $v['uid'];
            }

            $comjobsArr = $jobM->getList(array('lastupdate' => array('>=', strtotime('-7 day')), 'uid' => array('in', pylode(',', $comids))), array('field' => '`uid`,`lastupdate`'));
            $comJobs = $comjobsArr['list'];
            $comidss = array();
            foreach ($comJobs as $ck => $cv) {
                $comidss[$cv['uid']] = $cv['uid'];
            }

            $newComids = array_diff($comids, $comidss);

            $where['name'] = array('<>', '');

            $where['uid'] = array('in', pylode(',', $newComids));

            if ($xls_type == 'email') {
                $where['linkmail'] = array('<>', '');

                $where['email_status'] = '1';

                $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linkmail` as `email`'));
            } else {
                $where['linktel'] = array('<>', '');

                $where['moblie_status'] = '1';

                $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linktel` as `moblie`'));
            }

            foreach ($companys as $k => $v) {
                $userinfo[$v['uid']] = $v;
            }
        }

        if (!empty($userinfo)) {
            $list = $userinfo;

            include_once LIB_PATH . 'libs/PHPExcel.php';
            $objPHPExcel = new PHPExcel();

            $objPHPExcel->setActiveSheetIndex(0);

            $col = 'A'; // 初始列
            $row = 1; // 初始行
            $width = 20;
            $objPHPExcel->getActiveSheet()->getColumnDimension($col)->setWidth($width); // 设置列宽

            foreach ($list as $key => $val) {
                $text = $xls_type == 'email' ? $val['email'] : $val['moblie'];
                $objPHPExcel->getActiveSheet()->setCellValueExplicit($col . ($row++), $text, PHPExcel_Cell_DataType::TYPE_STRING); // 转类型
            }

            $objWriter = PHPExcel_IOFactory::createWriter($objPHPExcel, 'Excel2007');

            ob_start();
            $objWriter->save('php://output');
            $xlsData = ob_get_contents();
            ob_end_clean();
            $data = [
                'file' => base64_encode($xlsData),
                'file_name' => ($xls_type == 'email' ? '推广邮箱' : '推广手机号') . date('YmdHis') . '.xlsx'
            ];
            return $this->admin_json(0, $xls_type == 'email' ? '推广营销导出邮箱信息' : '推广营销导出手机号信息', $data);
        } else {
            $this->render_json(1, '没有可以导出的邮箱信息');
        }
    }

    // 标记已发送
    function finish_action()
    {
        $noticeM = $this->MODEL('notice');

        $msgdata = $emaildata = array();

        $emaildata['ctime'] = time();
        $emaildata['del'] = 1;
        $emaildata['state'] = 0;
        $msgdata['ctime'] = time();
        $msgdata['del'] = 1;
        $msgdata['state'] = 0;

        if ($_POST['type'] == 'birthday') {
            // 废弃生日通知
            // $emaildata['title'] = '生日提醒';
            // $msgdata['content'] = '生日提醒';
        } elseif ($_POST['type'] == 'anniversary') {
            $emaildata['title'] = '网站周年';
            $msgdata['content'] = '周年庆';
        } elseif ($_POST['type'] == 'todaydue') {
            $emaildata['title'] = '会员套餐还有1天将过期';
            $msgdata['content'] = '将于1天后到期';
        } elseif ($_POST['type'] == 'sevendue') {
            $emaildata['title'] = '会员套餐还有x天将过期';
            $msgdata['content'] = '将于7天后到期';
        } elseif ($_POST['type'] == 'useradd') {
            $emaildata['title'] = '未发布简历';
            $msgdata['content'] = '未发布简历';
        } elseif ($_POST['type'] == 'userup') {
            $emaildata['title'] = '今天简历刷新';
            $msgdata['content'] = '未刷新简历';
        } elseif ($_POST['type'] == 'addjob') {
            $emaildata['title'] = '未发布职位';
            $msgdata['content'] = '未发布职位';
        } elseif ($_POST['type'] == 'upjob') {
            $emaildata['title'] = '未刷新职位';
            $msgdata['content'] = '未刷新职位';
        }

        if ($_POST['xls_type'] == 'email') {
            $id = $noticeM->insertEmail($emaildata);
        } else {
            $id = $noticeM->insertMsg($msgdata);
        }

        if ($id) {
            $this->admin_json(0, '邮件推广标记已发送');
        } else {
            $this->render_json(1);
        }
    }

    // 发送
    function sendPromotion_action()
    {
        $jobM = $this->MODEL('job');
        $statisM = $this->MODEL('statis');
        $resumeM = $this->MODEL('resume');
        $companyM = $this->MODEL('company');
        $userinfoM = $this->MODEL('userinfo');

        $type = intval($_POST['type']);
        $emailtype = intval($_POST['emailtype']);
        $emailtpl = intval($_POST['emailtpl']);
        $dayslimit = intval($_POST['dayslimit']);
        $sort = intval($_POST['sort']);

        if ($sort) {
            if ($this->config['sy_email_set'] != "1") {
                $this->render_json(1, '还没有配置邮箱，请联系管理员');
            }
        } else {
            if (!checkMsgOpen($this->config)) {
                $this->render_json(1, '还没有配置短信，请联系管理员');
            }
        }

        $emailarr = $user = $com = $lt = $userinfo = array();
        $members = $users = $companys = $uids = $useremail = $comemail = $ltemail = $tpls = array();

        if ($emailtype == '1') {
            // 废弃生日通知
            // if(($this->config['sy_email_birthday']!=1 && $sort == '1') || ($this->config['sy_msg_birthday']!=1 && !$sort)){
            //     $this->render_json(1, '请先开启生日提醒');
            // }
            //
            // if($type=='1'){
            //     if($sort){
            //         $users	=	$resumeM->getResumeList(array('birthday'=>array('dateformat','%m%d'),'email'=>array('<>',''), 'email_status'=>'1'),array('field'=>'`email`,`uid`,`birthday`,`name`'));
            //     }else{
            //         $users	=	$resumeM->getResumeList(array('birthday'=>array('dateformat','%m%d'),'telphone'=>array('<>',''), 'moblie_status'=>'1'),array('field'=>'`uid`,`birthday`,`name`,`telphone` as moblie'));
            //     }
            //
            //     if($users&&is_array($users)){
            //         foreach($users as $k=>$v){
            //             $userinfo[$v['uid']]=$v;
            //         }
            //     }
            // }
        } else if ($emailtype == '2') {
            if (($this->config['sy_email_webbirthday'] != 1 && $sort == '1') || ($this->config['sy_msg_webbirthday'] != 1 && !$sort)) {
                $this->render_json(1, '请先开启周年提醒');
            }
            if ($type == '2') {
                if ($sort) {
                    $members = $userinfoM->getList(array('email' => array('<>', ''), 'status' => '1'), array('field' => '`uid`,`username`,`email`'));
                } else {
                    $members = $userinfoM->getList(array('moblie' => array('<>', ''), 'status' => '1'), array('field' => '`uid`,`username`,`moblie`'));
                }
                if ($members && is_array($members)) {
                    foreach ($members as $k => $v) {
                        $userinfo[$v['uid']] = $v;
                    }
                }
            }
        } else if ($emailtype == '3') {
            if (($this->config['sy_email_vipmr'] != 1 && $sort == '1') || ($this->config['sy_msg_vipmr'] != 1 && !$sort)) {
                $this->render_json(1, '请先开启会员到期提醒');
            }
            $statiswh['vip_etime'][] = array('>', time());

            $statiswh['vip_etime'][] = array('<', strtotime('+' . $dayslimit . ' day'));

            $comstatis = $statisM->getList($statiswh, array('field' => '`uid`,`vip_etime`,`rating_name`', 'usertype' => '2'));
            if (is_array($comstatis)) {
                foreach ($comstatis as $key => $value) {
                    $uid[] = $value['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uid));

                $where['name'] = array('<>', '');

                if ($sort) {
                    $where['linkmail'] = array('<>', '');

                    $where['email_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linkmail` as email'));
                } else {
                    $where['linktel'] = array('<>', '');

                    $where['moblie_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linktel` as moblie'));
                }
                foreach ($companys as $key => $value) {
                    foreach ($comstatis as $k => $v) {
                        if ($value['uid'] == $v['uid']) {
                            $companys[$key]['vip_etime'] = $v['vip_etime'];
                            $companys[$key]['rating_name'] = $v['rating_name'];
                        }
                    }
                }
            }
            if ($companys && is_array($companys)) {
                foreach ($companys as $k => $v) {
                    $v['day'] = $dayslimit;
                    $userinfo[$v['uid']] = $v;
                }
            }
        } else if ($emailtype == '4') {
            if (($this->config['sy_email_useradd'] != 1 && $sort == '1') || ($this->config['sy_msg_useradd'] != 1 && !$sort)) {
                $this->render_json(1, '请先开启未发布简历提醒');
            }
            if ($type == '1') {
                /*
                 * 当前日期前第七天注册
                 */
                $regStart = strtotime(date('Y-m-d 00:00:00', time() - 7 * 86400));
                $regEnd = strtotime(date('Y-m-d 23:59:59', time() - 7 * 86400));
                $regWhere['usertype'] = 1;
                $regWhere['PHPYUNBTWSTART_A'] = '';
                $regWhere['reg_date'][] = array('>=', $regStart);
                $regWhere['reg_date'][] = array('<=', $regEnd);
                $regWhere['PHPYUNBTWEND_A'] = '';
                $regs = $userinfoM->getList($regWhere, array('field' => 'uid,reg_date,username'));
                $regdate = $user = array();
                foreach ($regs as $k => $v) {
                    $uids[] = $v['uid'];
                    $regdate[$v['uid']] = $v['reg_date'];
                    $user[$v['uid']] = $v['username'];
                }
                $where['r_status'] = 1;

                $where['def_job'] = '0';

                $where['resumetime'] = '0';

                $where['uid'] = array('in', pylode(',', $uids));

                if ($sort) {
                    $where['email'] = array('<>', '');

                    $where['email_status'] = '1';

                    $users = $resumeM->getResumeList($where, array('field' => '`uid`,`name`,`email`,`resumetime`'));
                } else {
                    $where['telphone'] = array('<>', '');

                    $where['moblie_status'] = '1';

                    $users = $resumeM->getResumeList($where, array('field' => '`uid`,`name`,`resumetime`,`telphone` as `moblie`'));
                }
                foreach ($users as $k => $v) {
                    if ($v['name']) {

                        $v['name'] = $user[$v['uid']];
                    }
                    $userinfo[$v['uid']] = $v;
                    $userinfo[$v['uid']]['day'] = $dayslimit;
                    $userinfo[$v['uid']]['reg_date'] = $regdate[$v['uid']];
                }
            }
        } else if ($emailtype == '5') {
            if (($this->config['sy_email_userup'] != 1 && $sort == '1') || ($this->config['sy_msg_userup'] != 1 && !$sort)) {
                $this->render_json(1, '请先开启未刷新简历提醒');
            }

            if ($type == '1') {
                $where['def_job'] = array('>', '0');

                $where['r_status'] = 1;

                $where['lastupdate'] = array('<', strtotime('-7 day'));

                if ($sort) {
                    $where['email'] = array('<>', '');

                    $where['email_status'] = '1';

                    $resumes = $resumeM->getResumeList($where, array('field' => 'distinct `uid`,`name`,`email`,`lastupdate`'));
                } else {
                    $where['telphone'] = array('<>', '');

                    $where['moblie_status'] = '1';

                    $resumes = $resumeM->getResumeList($where, array('field' => 'distinct `uid`,`name`,`lastupdate`,`telphone` as `moblie`'));
                }
                foreach ($resumes as $k => $v) {

                    $userinfo[$v['uid']] = $v;
                    $userinfo[$v['uid']]['day'] = $dayslimit;
                }
            }

        } else if ($emailtype == '6') {
            if (($this->config['sy_email_addjob'] != 1 && $sort == '1') || ($this->config['sy_msg_addjob'] != 1 && !$sort)) {
                $this->render_json(1, '请先开启未发布职位提醒');
            }

            if ($type == '2') {
                /*
                 * 当前日期前第七天注册
                 */
                $regStart = strtotime(date('Y-m-d 00:00:00', time() - 7 * 86400));
                $regEnd = strtotime(date('Y-m-d 23:59:59', time() - 7 * 86400));
                $regWhere['usertype'] = 1;
                $regWhere['PHPYUNBTWSTART_A'] = '';
                $regWhere['reg_date'][] = array('>=', $regStart);
                $regWhere['reg_date'][] = array('<=', $regEnd);
                $regWhere['PHPYUNBTWEND_A'] = '';
                $regs = $userinfoM->getList($regWhere, array('field' => 'uid'));
                foreach ($regs as $k => $v) {
                    $uids[] = $v['uid'];
                }
                $where['jobtime'] = '0';
                $where['uid'] = array('in', pylode(',', $uids));
                $where['r_status'] = array('<>', '2');

                if ($sort) {
                    $where['linkmail'] = array('<>', '');

                    $where['email_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`jobtime`,`linkmail` as `email`,`linktel` as `moblie`'));
                } else {
                    $where['linktel'] = array('<>', '');

                    $where['moblie_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`jobtime`,`linktel` as `moblie`'));
                }
                foreach ($companys as $k => $v) {
                    $userinfo[$v['uid']] = $v;
                    $userinfo[$v['uid']]['day'] = $dayslimit;
                }
            }
        } else if ($emailtype == '7') {
            if (($this->config['sy_email_upjob'] != 1 && $sort == '1') || ($this->config['sy_msg_upjob'] != 1 && !$sort)) {
                $this->render_json(1, '请先开启未刷新职位提醒');
            }
            if ($type == '2') {
                $comjobsA = $jobM->getList(array('lastupdate' => array('<', strtotime('-7 day')), 'r_status' => array('<>', '2'), 'groupby' => 'uid', 'orderby' => 'lastupdate,desc'), array('field' => '`uid`,`lastupdate`'));
                $comjobs = $comjobsA['list'];
                $comids = array();
                foreach ($comjobs as $k => $v) {
                    $comids[$v['uid']] = $v['uid'];
                }

                $comjobsArr = $jobM->getList(array('lastupdate' => array('>=', strtotime('-7 day')), 'uid' => array('in', pylode(',', $comids))), array('field' => '`uid`,`lastupdate`'));
                $comJobs = $comjobsArr['list'];
                $comidss = array();
                foreach ($comJobs as $ck => $cv) {
                    $comidss[$cv['uid']] = $cv['uid'];
                }

                $newComids = array_diff($comids, $comidss);

                $where['name'] = array('<>', '');

                $where['uid'] = array('in', pylode(',', $newComids));

                if ($sort) {
                    $where['linkmail'] = array('<>', '');

                    $where['email_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linkmail` as `email`'));
                } else {
                    $where['linktel'] = array('<>', '');

                    $where['moblie_status'] = '1';

                    $companys = $companyM->getChCompanyList($where, array('field' => '`uid`,`name`,`linktel` as `moblie`'));
                }
                foreach ($companys as $k => $v) {
                    foreach ($comjobs as $val) {
                        if ($v['uid'] == $val['uid']) {
                            $companys[$k]['lastupdate'] = $val['lastupdate'];
                        }
                    }
                }
                foreach ($companys as $k => $v) {
                    $userinfo[$v['uid']] = $v;
                    $userinfo[$v['uid']]['day'] = $dayslimit;
                }
            }
        }

        if ($emailtpl == '1') {
            $useremail = array_unique($useremail);
            $comemail = array_unique($comemail);
            $ltemail = array_unique($ltemail);
            set_time_limit(1000);
            if (count($userinfo) > 500) {
                if ($sort) {
                    $msg = '数量过多，第三方发送服务器将会影响，部分邮件无法发送。建议找专业的群发软件！';
                } else {
                    $msg = '数量过多，第三方发送服务器将会影响，部分短信无法发送。建议找专业的群发软件！';
                }
                $this->render_json(1, $msg);
            }
        }

        foreach ($userinfo as $key => $value) {
            $data[] = $this->shjobmsg($value, $emailtype, $sort);
        }

        if ($data != "") {
            if ($_POST['action'] == 'sendEmailMsg') {
                $pagesize = intval($_POST['pagelimit']);
                $sendok = intval($_POST['sendok']);
                $sendno = intval($_POST['sendno']);

                $result = $this->sendEmailMsg($pagesize, $sendok, $sendno, $data);
                if ($result) {

                    $toSize = $pagesize * $result['page'];

                    if (count($data) > $toSize) {
                        $npage = $result['page'];
                        $spage = $npage * $pagesize + 1;
                        $topage = ($npage + 1) * $pagesize;

                        $name = $spage . "-" . $topage;
                        $this->get_return(3, $npage, "正在发送" . $name . "份数据", $result['sendok'], $result['sendno']);
                    } else {
                        $this->get_return(0, 0, "发送成功:" . $result['sendok'] . ",失败:" . $result['sendno']);
                    }
                }
            }
        }
    }

    function shjobmsg($info, $type, $sort)
    {
        $data = array();
        $tpltype = array(
            // '1'	=>	'birthday',
            '2' => 'webbirthday',
            '3' => 'vipmr',
            '4' => 'useradd',
            '5' => 'userup',
            '6' => 'addjob',
            '7' => 'upjob'
        );
        $data['type'] = $tpltype[$type];
        if ($data['type'] != "") {
            if ($type == '1') {
                // 废弃生日通知
                // $data['uid']		=	$info['uid'];
                // $data['name']		=	$info['name'];
                // if($sort){
                //     $data['email']	=	$info['email'];
                // }else{
                //     $data['moblie']	=	$info['moblie'];
                // }
                // $data['username']	=	$info['name'];
                // $data['date']		=	$info['birthday'];
                // $data['year']		=	date("Y")-date("Y",strtotime($info['birthday']));
            } elseif ($type == '2') {
                $data['uid'] = $info['uid'];
                $data['name'] = $info['username'];
                $data['username'] = $info['username'];
                if ($sort) {
                    $data['email'] = $info['email'];
                } else {
                    $data['moblie'] = $info['moblie'];
                }
            } elseif ($type == '3') {
                $data['uid'] = $info['uid'];
                $data['name'] = $info['name'];
                if ($sort) {
                    $data['email'] = $info['email'];
                } else {
                    $data['moblie'] = $info['moblie'];
                }
                $data['ratingname'] = $info['rating_name'];
                $data['date'] = date("Y-m-d", $info['vip_etime']);//会员到期日期
                $data['day'] = ceil(($info['vip_etime'] - time()) / (60 * 60 * 24));//剩余几天到期（向下取整）
            } elseif ($type == '4') {
                if ($info['username_n'] == '') {
                    $userarr = $this->MODEL('userinfo')->getInfo(array('uid' => $info['uid']), array('field' => '`username`'));
                    $data['username'] = $userarr['username'];
                } else {
                    $data['username'] = $info['username_n'];
                }
                $data['uid'] = $info['uid'];
                $data['name'] = $info['name_n'];
                if ($sort) {
                    $data['email'] = $info['email'];
                } else {
                    $data['moblie'] = $info['moblie'];
                }
                $data['date'] = date("Y-m-d", $info['reg_date']);
                $data['day'] = $info['day'];
            } elseif ($type == '5') {
                if ($info['username_n'] == '') {
                    $userarr = $this->MODEL('userinfo')->getInfo(array('uid' => $info['uid']), array('field' => '`username`'));
                    $data['username'] = $userarr['username'];
                } else {
                    $data['username'] = $info['username_n'];
                }
                $data['uid'] = $info['uid'];
                $data['name'] = $info['name_n'];
                if ($sort) {
                    $data['email'] = $info['email'];
                } else {
                    $data['moblie'] = $info['moblie'];
                }
                $data['date'] = date("Y-m-d", $info['lastupdate']);
                $data['day'] = $info['day'];
            } elseif ($type == '6') {
                $data['uid'] = $info['uid'];
                $data['name'] = $info['name'];
                if ($sort) {
                    $data['email'] = $info['email'];
                } else {
                    $data['moblie'] = $info['moblie'];
                }
                if ($info['name']) {
                    $data['username'] = $info['name'];
                } else {
                    $data['username'] = $info['username'];
                }
                $data['date'] = date("Y-m-d", $info['jobtime']);
                $data['day'] = $info['day'];
            } elseif ($type == '7') {
                $data['uid'] = $info['uid'];
                $data['name'] = $info['name'];
                if ($sort) {
                    $data['email'] = $info['email'];
                } else {
                    $data['moblie'] = $info['moblie'];
                }
                if ($info['name']) {
                    $data['username'] = $info['name'];
                } else {
                    $data['username'] = $info['username'];
                }
                $data['date'] = date("Y-m-d", $info['lastupdate']);
                $data['day'] = $info['day'];
            }
            return $data;
        }
    }

    function sendEmailMsg($pagesize, $sendok, $sendno, $data)
    {

        $errorMsg = '';
        $notice = $this->MODEL('notice');

        if ($_POST['value'] == '0') {

            foreach ($data as $key => $value) {

                if ($key < $pagesize) {
                    if ($value['email']) {
                        $retval = $notice->sendEmailType($value);
                    }
                    if ($value['moblie']) {
                        $value['port'] = '5';
                        $retval = $notice->sendSMSType($value);
                    }

                    if ($retval['status'] != -1) {
                        $sendok++;
                    } else {
                        $sendno++;
                    }
                }
            }

            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = 1;

        } else {

            $page = $_POST['value'];
            $start = $page * $pagesize;
            $end = ($page + 1) * $pagesize;

            foreach ($data as $key => $value) {

                if ($key >= $start && $key < $end) {
                    if ($value['email']) {
                        $retval = $notice->sendEmailType($value);
                    }
                    if ($value['moblie']) {
                        $value['port'] = '5';
                        $retval = $notice->sendSMSType($value);
                    }
                    if ($retval['status'] != -1) {
                        $sendok++;
                    } else {
                        $sendno++;
                    }
                }
            }
            $page = $page + 1;
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = $page;
        }

        return $result;
    }

    //简历推广
    function resume_action()
    {
        $this->render_json(0, 'ok', array(
            'sy_webname' => $this->config['sy_webname'],
        ));
    }

    // 简历推广-获取企业类型对应的可发送数量
    function getcom_action()
    {
        $jobM = $this->MODEL('job');

        $statisM = $this->MODEL('statis');

        $companyM = $this->MODEL('company');

        $userinfoM = $this->MODEL('userinfo');

        $com = (int)$_POST['com'];
        $time = time();
        if ($com == 1) {
            $row = $statisM->getList(array('vip_etime' => array('>=', strtotime('today'))), array('field' => 'uid', 'usertype' => '2'));
        } elseif ($com == 2) {
            $rowA = $jobM->getList(array('lastupdate' => array('>', strtotime('-7 day', $time)), 'r_status' => array('<>', '2')), array('field' => '`uid`'));
            $row = $rowA['list'];
        } elseif ($com == 3) {
            $row = $userinfoM->getList(array('reg_date' => array('>', strtotime('-3 day', $time)), 'usertype' => '2'), array('field' => 'uid'));
        }
        $uids = array();
        foreach ($row as $r) {
            $uids [] = $r['uid'];
        }
        $uids = array_unique($uids);

        $num = 0;
        if (count($uids) > 0) {
            $where['uid'] = array('in', pylode(',', $uids));

            $where['name'] = array('<>', '');

            if ($_POST['msgType'] == 2) {
                $where['linktel'] = array('<>', '');

                $where['moblie_status'] = '1';
            } else {
                $where['linkmail'] = array('<>', '');

                $where['email_status'] = '1';
            }
            $num = $companyM->getCompanyNum($where);
        }

        $this->render_json(0, 'ok', $num);
    }

    // 简历推广发送
    function sendresume_action()
    {
        extract($_POST);
        $stype = intval($stype);

        if ($stype == 1) {

            $company = $this->gsresume($resume, $com, $sendnum, $num);
            if ($company && is_array($company)) {
                $pagesize = intval($_POST['pagelimit']);
                $sendok = intval($_POST['sendok']);
                $sendno = intval($_POST['sendno'] ? $_POST['sendno'] : 0);
                $value = intval($_POST['value']);

                $result = $this->sendResumeETG($company, $email_title, $pagesize, $value, $sendok, $sendno);

                if ($result) {
                    $toSize = $pagesize * $result['page'];

                    if (count($company) > $toSize) {
                        $npage = $result['page'];
                        $spage = $npage * $pagesize + 1;
                        $topage = ($npage + 1) * $pagesize;
                        $name = $spage . "-" . $topage;

                        $this->get_return(3, $npage, "正在发送" . $name . "份邮件", $result['sendok'], $result['sendno']);
                    } else {
                        $this->get_return(0, 0, "发送成功:" . $result['sendok'] . ",失败:" . $result['sendno']);
                    }
                }
            }
        } else {
            if (!checkMsgOpen($this->config)) {
                $this->render_json(2, '还没有配置短信');
            }
            $company = $this->getsendcom($com, $sendnum, 2);

            if ($company && is_array($company)) {

                $pagesize = intval($pagelimit);
                $sendok = intval($sendok);
                $sendno = intval($sendno ? $sendno : 0);
                $value = intval($value);

                $result = $this->sendResumeMTG($company, $content, $pagesize, $value, $sendok, $sendno);

                if ($result) {
                    $toSize = $pagesize * $result['page'];

                    if (count($company) > $toSize) {
                        $npage = $result['page'];
                        $spage = $npage * $pagesize + 1;
                        $topage = ($npage + 1) * $pagesize;
                        $name = $spage . "-" . $topage;

                        $this->get_return(3, $npage, "正在发送" . $name . "条信息", $result['sendok'], $result['sendno']);
                    } else {
                        $this->get_return(0, 0, "发送成功:" . $result['sendok'] . ",失败:" . $result['sendno']);
                    }
                }
            }
        }
    }

    function sendResumeETG($company, $title, $pagesize, $value, $sendok, $sendno)
    {

        $notice = $this->MODEL('notice');

        if ($value == 0) {
            foreach ($company as $key => $val) {

                if ($key < $pagesize) {
                    if ($val['html'] != '') {
                        $emailData['email'] = $val['linkmail'];
                        $emailData['subject'] = $title;
                        $emailData['content'] = $val['html'];
                        $emailData['uid'] = $val['uid'];
                        $emailData['name'] = $val['name'];
                        $emailData['cname'] = "admin";
                        $sendid = $notice->sendEmail($emailData);

                        if ($sendid['status'] != -1) {
                            $sendok++;
                        } else {
                            $sendno++;
                        }
                    }
                }
            }

            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = 1;

        } else {

            $page = $_POST['value'];
            $start = $page * $pagesize;
            $end = ($page + 1) * $pagesize;

            foreach ($company as $key => $val) {

                if ($key >= $start && $key < $end) {
                    if ($val['html'] != '') {
                        $emailData['email'] = $val['linkmail'];
                        $emailData['subject'] = $title;
                        $emailData['content'] = $val['html'];
                        $emailData['uid'] = $val['uid'];
                        $emailData['name'] = $val['name'];
                        $emailData['cname'] = "admin";
                        $sendid = $notice->sendEmail($emailData);

                        if ($sendid['status'] != -1) {
                            $sendok++;
                        } else {
                            $sendno++;
                        }
                    }
                }
            }
            $page = $page + 1;
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = $page;
        }

        return $result;
    }

    function sendResumeMTG($company, $content, $pagesize, $value, $sendok, $sendno)
    {

        $notice = $this->MODEL('notice');

        if ($value == 0) {
            foreach ($company as $key => $val) {
                if ($key < $pagesize) {
                    $msgData['moblie'] = $val['linktel'];
                    $msgData['content'] = $content;
                    $msgData['uid'] = $val['uid'];
                    $msgData['name'] = $val['name'];
                    $msgData['cname'] = "admin";
                    $msgData['port'] = '5';
                    $sendid = $notice->sendSMS($msgData);
                    if ($sendid['status'] != -1) {
                        $sendok++;
                    } else {
                        $sendno++;
                    }
                }
            }

            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = 1;

        } else {

            $page = $_POST['value'];
            $start = $page * $pagesize;
            $end = ($page + 1) * $pagesize;

            foreach ($company as $key => $val) {

                if ($key >= $start && $key < $end) {
                    $msgData['moblie'] = $val['linktel'];
                    $msgData['content'] = $content;
                    $msgData['uid'] = $val['uid'];
                    $msgData['name'] = $val['name'];
                    $msgData['cname'] = "admin";
                    $msgData['port'] = '5';
                    $sendid = $notice->sendSMS($msgData);
                    if ($sendid['status'] != -1) {
                        $sendok++;
                    } else {
                        $sendno++;
                    }
                }
            }
            $page = $page + 1;
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = $page;
        }

        return $result;
    }

    function gsresume($resume, $com, $sendnum, $num)
    {
        $resumeM = $this->MODEL('resume');
        $company = $this->getsendcom($com, $sendnum);
        $cacheM = $this->MODEL('cache');
        $cacheList = $cacheM->GetCache('user');
        if ($company && is_array($company)) {
            foreach ($company as $v) {
                $hyid[$v['uid']]['hy'] = $v['hy'];
                $hyid[$v['uid']]['cityid'] = $v['cityid'];
            }
        }
        if ($hyid && is_array($hyid)) {
            include(CONFIG_PATH . "db.data.php");
            include PLUS_PATH . "/user.cache.php";
            foreach ($hyid as $k => $v) {
                $where['hy'] = $v['hy'];

                $where['status'] = array('<>', '2');

                $where['r_status'] = 1;

                $where['job_classid'] = array('<>', '');

                $where['defaults'] = '1';

                $where['orderby'] = 'lastupdate,desc';

                $where['limit'] = $num;

                if ($resume == 1) {
                    $where['height_status'] = '2';

                    $expect = $resumeM->getSimpleList($where);
                } elseif ($resume == 2) {
                    $expect = $resumeM->getSimpleList($where);
                } elseif ($resume == 3) {
                    $where['whour'] = array('>', '12');

                    $where['exp'] = array('>', '18');

                    $expect = $resumeM->getSimpleList($where);
                }

                if ($expect && is_array($expect)) {

                    $html = '<table width="800"border="0" style="border:1px solid #ddd" cellpadding="5" cellspacing="0" id="rtable">';
                    $html .= '<tr ><td colspan="6"><div style="float:left;padding:10px;"><img src="' . $this->config['sy_weburl'] . '/' . $this->config['sy_logo'] . '"></div><div style="float:left; padding-left:100px; line-height: 100px;"> 网站联系电话:' . $this->config['sy_freewebtel'] . '</div> <div style=" float:right; padding-right:10px;    text-align: center;"><div><img src="' . $this->config['sy_weburl'] . '/' . $this->config['sy_wx_qcode'] . '" width="80"height="80"></div>微信公众号二维码</div></td></tr>';
                    $html .= '<tr style="background:#f8f8f8; font-weight:bold"><td height="26">姓名</td><td>年龄</td><td>学历</td><td>工作经验</td> <td>性别</td> <td width="80" align="center">操作</td></tr>';


                    foreach ($expect as $k => $v) {

                        $a = date('Y', strtotime($v['birthday']));
                        $age = date("Y") - $a;
                        if ($v['height_status'] == 2) {
                            $url = Url('resume', array('c' => 'show', 'id' => $v['id'], 'type' => 2));
                        } else {
                            $url = Url('resume', array('c' => 'show', 'id' => $v['id']));
                        }
                        $html .= '<tr><td height="26" style="border-bottom:1px solid #ddd"><b><font color="#0033FF">' . $v['uname'] . '</font></b></td><td style="border-bottom:1px solid #ddd">' . $age . '</td><td style="border-bottom:1px solid #ddd">' . $userclass_name[$v['edu']] . '</td><td style="border-bottom:1px solid #ddd">' . $userclass_name[$v['exp']] . '</td> <td style="border-bottom:1px solid #ddd">' . $cacheList['user_sex'][$v['sex']] . '</td><td align="center" style="border-bottom:1px solid #ddd"><a href="' . $url . '" style="background:#f60; padding:2px 14px;color:#fff; display:inline-block">查看</a></td></tr>';
                    }
                    $html .= '</table>';
                    $table[$k] = $html;
                }
            }
            foreach ($company as $k => $v) {

                foreach ($table as $key => $val) {
                    $company[$k]['html'] = $val;
                }
            }

            return $company;
        }
    }

    function getsendcom($com, $sendnum, $type = 1)
    {
        $jobM = $this->MODEL('job');

        $statisM = $this->MODEL('statis');

        $companyM = $this->MODEL('company');

        $userinfoM = $this->MODEL('userinfo');

        if ($com == 1) {
            $scom = $statisM->getList(array('vip_etime' => array('>=', strtotime('today'))), array('field' => 'uid', 'usertype' => '2'));
        } elseif ($com == 2) {
            $scomA = $jobM->getList(array('lastupdate' => array('>', strtotime('-7 day')), 'r_status' => array('<>', '2')), array('field' => "distinct `uid`"));
            $scom = $scomA['list'];
        } elseif ($com == 3) {
            $scom = $userinfoM->getList(array('reg_date' => array('>', strtotime('-3 day')), 'usertype' => '2'), array('field' => 'uid'));
        }
        if ($scom && is_array($scom)) {
            foreach ($scom as $v) {
                $comid[] = $v['uid'];
            }

            $where['r_status'] = array('<>', '2');

            $where['uid'] = array('in', pylode(',', $comid));

            $where['orderby'] = 'lastupdate,desc';

            if ($sendnum) {
                $where['limit'] = $sendnum;
            }
            if ($type == 1) {
                $where['linkmail'] = array('<>', '');

                $where['email_status'] = '1';

                $company = $companyM->getChCompanyList($where, array('field' => 'uid,name,hy,linkmail,cityid'));
            } else {
                $where['linktel'] = array('<>', '');

                $where['moblie_status'] = '1';

                $company = $companyM->getChCompanyList($where, array('field' => 'uid,name,hy,linktel,cityid'));
            }
        }
        return $company;
    }

    // 职位推广
    function job_action()
    {
        $this->render_json(0, 'ok', array(
            'sy_webname' => $this->config['sy_webname'],
        ));
    }

    // 职位推广-获取选择用户数量
    function getuser_action()
    {
        $resumeM = $this->MODEL('resume');

        $userinfoM = $this->MODEL('userinfo');

        $user = (int)$_POST['user'];
        if ($user == 1) {
            $row = $resumeM->getSimpleList(array('lastupdate' => array('>', strtotime('-7 day')), 'status' => 1, 'r_status' => 1, 'defaults' => '1'), array('field' => '`uid`'));
        } elseif ($user == 2) {
            $row = $userinfoM->getList(array('reg_date' => array('>', strtotime('-3 day')), 'usertype' => '1'), array('field' => '`uid`'));
        }
        $uids = array();
        foreach ($row as $v) {
            $uids [] = $v['uid'];
        }

        $num = 0;
        if (count($uids) > 0) {
            $where['uid'] = array('in', pylode(',', $uids));
            if ($_POST['msgType'] == 1) {
                $where['email'] = array('<>', '');
                $where['email_status'] = '1';
            } else {
                $where['telphone'] = array('<>', '');
                $where['moblie_status'] = '1';
            }
            $num = $resumeM->getResumeNum($where);
        }

        $this->render_json(0, 'ok', $num);
    }

    // 职位推广-获取选择职位数量
    function getjob_action()
    {
        $jobM = $this->MODEL('job');

        $job = (int)$_POST['job'];

        $num = 0;
        if ($job == 2) {
            $num = $jobM->getJobNum(array('rec_time' => array('>', time()), 'state' => '1'));
        } elseif ($job == 3) {
            $num = $jobM->getJobNum(array('urgent_time' => array('>', time()), 'state' => '1'));
        }

        $this->render_json(0, 'ok', $num);
    }

    // 职位推广-发送职位
    function sendjob_action()
    {
        extract($_POST);
        $stype = intval($stype);

        if ($stype == 1) {
            $resume = $this->gsjob($job, $user, $sendnum, $num);

            if ($resume && is_array($resume)) {
                $pagesize = intval($_POST['pagelimit']);
                $sendok = intval($_POST['sendok']);
                $sendno = intval($_POST['sendno'] ? $_POST['sendno'] : 0);
                $value = intval($_POST['value']);

                $result = $this->sendJobETG($resume, $email_title, $pagesize, $value, $sendok, $sendno);

                if ($result) {
                    $toSize = $pagesize * $result['page'];

                    if (count($resume) > $toSize) {
                        $npage = $result['page'];
                        $spage = $npage * $pagesize + 1;
                        $topage = ($npage + 1) * $pagesize;
                        $name = $spage . "-" . $topage;

                        $this->get_return(3, $npage, "正在发送" . $name . "份邮件", $result['sendok'], $result['sendno']);
                    } else {
                        $this->get_return(0, 0, "发送成功:" . $result['sendok'] . ",失败:" . $result['sendno']);
                    }
                }
            }
        } else {
            if (!checkMsgOpen($this->config)) {
                $this->render_json(2, '还没有配置短信');
            }
            $resume = $this->getsenduser($_POST['user'], $_POST['sendnum'], 2);

            if ($resume && is_array($resume)) {
                $pagesize = intval($pagelimit);
                $sendok = intval($sendok);
                $sendno = intval($sendno ? $sendno : 0);
                $value = intval($value);

                $result = $this->sendJobMTG($resume, $content, $pagesize, $value, $sendok, $sendno);

                if ($result) {
                    $toSize = $pagesize * $result['page'];

                    if (count($resume) > $toSize) {
                        $npage = $result['page'];
                        $spage = $npage * $pagesize + 1;
                        $topage = ($npage + 1) * $pagesize;
                        $name = $spage . "-" . $topage;

                        $this->get_return(3, $npage, "正在发送" . $name . "条信息", $result['sendok'], $result['sendno']);
                    } else {
                        $this->get_return(0, 0, "发送成功:" . $result['sendok'] . ",失败:" . $result['sendno']);
                    }
                }
            }
        }
    }

    function gsjob($job, $user, $sendnum, $num)
    {
        $jobM = $this->MODEL('job');
        $resumeM = $this->MODEL('resume');
        $resume = $this->getsenduser($user, $sendnum);
        $cacheM = $this->MODEL('cache');
        $cacheList = $cacheM->GetCache('user');
        if ($resume && is_array($resume)) {
            foreach ($resume as $v) {
                $uid[] = $v['uid'];
            }
            $expect = $resumeM->getSimpleList(array('uid' => array('in', pylode(',', $uid)), 'defaults' => '1'), array('field' => 'uid,hy,city_classid'));
            foreach ($expect as $v) {
                $hyid[$v['uid']]['hy'] = $v['hy'];
                $hyid[$v['uid']]['cityid'] = $v['city_classid'];
            }
        }
        if ($hyid && is_array($hyid)) {
            include(CONFIG_PATH . "db.data.php");
            include PLUS_PATH . "/com.cache.php";
            include PLUS_PATH . "/city.cache.php";
            foreach ($hyid as $k => $v) {
                $where['hy'] = $v['hy'];

                $where['cityid'] = array('in', $v['cityid']);

                $where['state'] = '1';

                $where['orderby'] = 'lastupdate,desc';

                $where['limit'] = $num;

                if ($job == 1) {
                    $comjobA = $jobM->getList($where);
                    $comjob = $comjobA['list'];
                } elseif ($job == 2) {
                    $where['rec_time'] = array('>', time());

                    $comjobA = $jobM->getList($where);
                    $comjob = $comjobA['list'];
                } elseif ($job == 3) {
                    $where['urgent_time'] = array('>', time());

                    $comjobA = $jobM->getList($where);
                    $comjob = $comjobA['list'];
                }
                if ($comjob && is_array($comjob)) {
                    $html = '<table width="800"border="0" style="border:1px solid #ddd" cellpadding="5" cellspacing="0" id="rtable">';
                    $html .= '<tr ><td colspan="6"><div style="float:left;padding:10px;"><img src="' . $this->config['sy_weburl'] . '/' . $this->config['sy_logo'] . '"></div><div style="float:left; padding-left:100px; line-height: 100px;"> 网站联系电话:' . $this->config['sy_freewebtel'] . '</div> <div style=" float:right; padding-right:10px;    text-align: center;"><div><img src="' . $this->config['sy_weburl'] . '/' . $this->config['sy_wx_qcode'] . '" width="80"height="80"></div>微信公众号二维码</div></td></tr>';
                    $html .= '<tr style="background:#f8f8f8; font-weight:bold"><td height="26">职位</td><td>工作地点</td><td>薪资</td><td>学历要求</td><td>工作经验</td> <td>性别</td> <td width="80" align="center">操作</td></tr>';
                    foreach ($comjob as $v) {
                        $url = Url('job', array('c' => 'comapply', 'id' => $v['id']));
                        $salary = salaryUnit($v['minsalary'], $v['maxsalary']);
                        $html .= '<tr><td height="26" style="border-bottom:1px solid #ddd"><b><font color="#0033FF">' . mb_substr($v['name'], "0", "12", "utf-8") . '</font></b></td><td style="border-bottom:1px solid #ddd">' . $city_name[$v['cityid']] . '</td><td style="border-bottom:1px solid #ddd">' . $salary . '</td><td style="border-bottom:1px solid #ddd">' . $comclass_name[$v['edu']] . '</td><td style="border-bottom:1px solid #ddd">' . $comclass_name[$v['exp']] . '</td> <td style="border-bottom:1px solid #ddd">' . $cacheList['user_sex'][$v['sex']] . '</td><td align="center" style="border-bottom:1px solid #ddd"><a href="' . $url . '" style="background:#f60; padding:2px 14px;color:#fff; display:inline-block">查看</a></td></tr>';
                    }
                    $html .= '</table>';
                    $table[$k] = $html;
                }
            }

            foreach ($resume as $k => $v) {
                foreach ($table as $key => $val) {
                    if ($v['uid'] == $key) {
                        $resume[$k]['html'] = $val;
                    }
                }
            }

            return $resume;
        }
    }

    function getsenduser($user, $sendnum, $type = 1)
    {
        $resumeM = $this->MODEL('resume');

        $userinfoM = $this->MODEL('userinfo');
        if ($user == 1) {
            $suser = $resumeM->getSimpleList(array('lastupdate' => array('>', strtotime('-7 day')), 'status' => array('<>', '2'), 'r_status' => array('<>', '2'), 'job_classid' => array('<>', ''), 'defaults' => '1'), array('field' => '`uid`'));
        } elseif ($user == 2) {
            $suser = $userinfoM->getList(array('reg_date' => array('>', strtotime('-3 day')), 'usertype' => '1'), array('field' => '`uid`'));
        }
        if ($suser && is_array($suser)) {
            foreach ($suser as $v) {
                $userid[] = $v['uid'];
            }

            $where['r_status'] = 1;

            $where['uid'] = array('in', pylode(',', $userid));

            $where['orderby'] = 'lastupdate,desc';

            if ($sendnum) {
                $where['limit'] = $sendnum;
            }
            if ($type == 1) {
                $where['email'] = array('<>', '');

                $where['email_status'] = '1';

                $resume = $resumeM->getResumeList($where, array('field' => "uid,name,email"));
            } else {
                $where['telphone'] = array('<>', '');

                $where['moblie_status'] = '1';

                $resume = $resumeM->getResumeList($where, array('filed' => "uid,name,telphone"));
            }
        }
        return $resume;
    }

    function sendJobETG($resume, $title, $pagesize, $value, $sendok, $sendno)
    {

        $notice = $this->MODEL('notice');

        if ($value == 0) {
            foreach ($resume as $key => $val) {

                if ($key < $pagesize) {
                    if ($val['html'] != '') {
                        $emailData['email'] = $val['email'];
                        $emailData['subject'] = $title;
                        $emailData['content'] = $val['html'];
                        $emailData['uid'] = $val['uid'];
                        $emailData['name'] = $val['name'];
                        $emailData['cname'] = "admin";
                        $sendid = $notice->sendEmail($emailData);
                        if ($sendid['status'] != -1) {
                            $sendok++;
                        } else {
                            $sendno++;
                        }
                    }
                }
            }

            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = 1;

        } else {

            $page = $_POST['value'];
            $start = $page * $pagesize;
            $end = ($page + 1) * $pagesize;

            foreach ($resume as $key => $val) {

                if ($key >= $start && $key < $end) {
                    if ($val['html'] != '') {
                        $emailData['email'] = $val['email'];
                        $emailData['subject'] = $title;
                        $emailData['content'] = $val['html'];
                        $emailData['uid'] = $val['uid'];
                        $emailData['name'] = $val['name'];
                        $emailData['cname'] = "admin";
                        $sendid = $notice->sendEmail($emailData);

                        if ($sendid['status'] != -1) {
                            $sendok++;
                        } else {
                            $sendno++;
                        }
                    }
                }
            }
            $page = $page + 1;
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = $page;
        }

        return $result;
    }

    function sendJobMTG($resume, $content, $pagesize, $value, $sendok, $sendno)
    {

        $notice = $this->MODEL('notice');

        if ($value == 0) {
            foreach ($resume as $key => $value) {
                if ($key < $pagesize) {
                    $msgData['moblie'] = $value['telphone'];
                    $msgData['content'] = $content;
                    $msgData['uid'] = $value['uid'];
                    $msgData['name'] = $value['name'];
                    $msgData['cname'] = "admin";
                    $msgData['port'] = '5';
                    $sendid = $notice->sendSMS($msgData);
                    if ($sendid['status'] != -1) {
                        $sendok++;
                    } else {
                        $sendno++;
                    }
                }
            }

            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = 1;

        } else {

            $page = $_POST['value'];
            $start = $page * $pagesize;
            $end = ($page + 1) * $pagesize;

            foreach ($resume as $key => $value) {

                if ($key >= $start && $key < $end) {
                    $msgData['moblie'] = $value['telphone'];
                    $msgData['content'] = $content;
                    $msgData['uid'] = $value['uid'];
                    $msgData['name'] = $value['name'];
                    $msgData['cname'] = "admin";
                    $msgData['port'] = '5';
                    $sendid = $notice->sendSMS($msgData);
                    if ($sendid['status'] != -1) {
                        $sendok++;
                    } else {
                        $sendno++;
                    }
                }
            }
            $page = $page + 1;
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = $page;
        }

        return $result;
    }

    // 自定义邮件-发送
    function send_action()
    {
        $UserInfoM = $this->MODEL('userinfo');

        if ($_POST['email_title'] == '' || $_POST['content'] == '') {
            $this->render_json(2, '邮件标题均不能为空');
        }

        $emailarr = $user = $com = $lt = $px = $userinfo = array();

        if ($_POST['utype'] != '5') {
            //自定义用户
            $userrows = $UserInfoM->getList(array('usertype' => $_POST['utype']), array('field' => 'email,`uid`,`usertype`'));
        } else if ($_POST['utype'] == '5') {
            $email_user = @explode(',', $_POST['email_user']);
            $email_user = array_filter($email_user);
            foreach ($email_user as $v) {
                if (CheckRegEmail($v)) {
                    $earr[] = $v;
                }
            }
            if (!empty($earr)) {
                $where['email'] = array('in', "\"" . @implode('","', $earr) . "\"");
                $userrows = $UserInfoM->getList($where, array('field' => '`email`,`uid`,`usertype`'));
            }
        }
        if (is_array($userrows) && $userrows) {
            foreach ($userrows as $v) {
                if ($v['usertype'] == '1') {
                    $user[] = $v['uid'];
                }
                if ($v['usertype'] == '2') {
                    $com[] = $v['uid'];
                }
                if ($v['usertype'] == '3') {
                    $lt[] = $v['uid'];
                }
                if ($v['usertype'] == '4') {
                    $px[] = $v['uid'];
                }

                $emailarr[$v['uid']] = $v["email"];
            }

            if ($user && is_array($user)) {
                $where['uid'] = array('in', pylode(',', $user));
            }
            if ($com && is_array($com)) {
                $where['uid'] = array('in', pylode(',', $com));
            }
            if ($lt && is_array($lt)) {
                $where['uid'] = array('in', pylode(',', $lt));
            }
            if ($px && is_array($px)) {
                $where['uid'] = array('in', pylode(',', $px));
            }
            $List = $UserInfoM->getUserList($where);

            foreach ($List as $v) {
                $userinfo[$v['uid']] = $v['name'];
            }
        }

        if (!count($emailarr)) {
            $this->render_json(2, '没有符合条件的邮箱，请先检查');
        } else {
            set_time_limit(10000);

            $pagesize = intval($_POST['pagelimit']);
            $sendok = intval($_POST['sendok']);
            $sendno = intval($_POST['sendno'] ? $_POST['sendno'] : 0);
            $value = intval($_POST['value']);

            //分批次发送
            $result = $this->send_email($emailarr, $_POST['email_title'], $_POST['content'], $userinfo, $pagesize, $sendok, $sendno, $value);

            if ($result) {
                $toSize = $pagesize * $result['page'];

                if (count($emailarr) > $toSize) {
                    $npage = $result['page'];
                    $spage = $npage * $pagesize + 1;
                    $topage = ($npage + 1) * $pagesize;
                    $name = $spage . "-" . $topage;

                    $this->get_return(3, $result['page'], "正在发送" . $name . "封邮件", $result['sendok'], $result['sendno']);

                } else {
                    $this->get_return(0, $result['page'], "发送成功:" . $result['sendok'] . ",失败:" . $result['sendno']);
                }
            }
        }
    }

    function send_email($email = array(), $emailtitle = "", $emailcoment = "", $userinfo = array(), $pagesize, $sendok, $sendno, $value)
    {

        $notice = $this->MODEL('notice');
        $sendok = intval($sendok);
        $sendno = intval($sendno);

        if ($value == '0') {
            $i = 1;
            foreach ($email as $key => $v) {

                if ($i <= $pagesize) {
                    $emailData['email'] = $v;
                    $emailData['subject'] = $emailtitle;
                    $emailData['content'] = stripslashes($emailcoment);
                    $emailData['uid'] = $key;
                    $emailData['name'] = $userinfo[$key];
                    $emailData['cname'] = "系统";
                    if ($v) {
                        $sendid = $notice->sendEmail($emailData);
                    }
                    if ($sendid['status'] != -1) {
                        $state = 1;
                        $sendok++;
                    } else {
                        $state = 0;
                        $sendno++;
                    }
                }
                $i++;
            }
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = 1;

        } else {
            $page = $value;
            $start = $page * $pagesize;
            $end = ($page + 1) * $pagesize;

            $i = 1;

            foreach ($email as $key => $v) {

                if ($i > $start && $i <= $end) {

                    $emailData['email'] = $v;
                    $emailData['subject'] = $emailtitle;
                    $emailData['content'] = stripslashes($emailcoment);
                    $emailData['uid'] = $key;
                    $emailData['name'] = $userinfo[$key]['name'];
                    $emailData['cname'] = "系统";

                    if ($v) {
                        $sendid = $notice->sendEmail($emailData);
                    }
                    if ($sendid['status'] != -1) {
                        $state = 1;
                        $sendok++;
                    } else {
                        $state = 0;
                        $sendno++;
                    }
                }
                $i++;
            }

            $page = $page + 1;
            $result['sendok'] = $sendok;
            $result['sendno'] = $sendno;
            $result['page'] = $page;

        }
        return $result;
    }

    //自定义短信-发送
    function msgsave_action()
    {
        $userinfoM = $this->MODEL('userinfo');

        if (!checkMsgOpen($this->config)) {
            $this->render_json(2, '还没有配置短信');
        }
        if (trim($_POST['content']) == '') {
            $this->render_json(2, '请输入短信内容');
        }
        if ($_POST['userarr'] == '' && $_POST['utype'] == '5') {
            $this->render_json(2, '手机号码不能为空');
        }
        if ($_POST['utype'] == 5) {
            $where = array('moblie' => array('in', $_POST['userarr']));
            $total = $userinfoM->getMemberNum($where);
        } else {
            $where = array('usertype' => $_POST['utype'], 'moblie' => array('<>', ''));
            $total = $userinfoM->getMemberNum($where);
        }
        if ($total > 0) {
            //循环次数
            $page = intval($_POST['page']);
            $size = 50;
            $num = ceil($total / $size);

            $sendok = $sendno = 0;

            $where['limit'] = array(($size * ($page - 1)), $size);

            $userrows = $userinfoM->getList($where, array('field' => '`moblie`,`uid`,`usertype`'));

            $notice = $this->MODEL('notice');

            foreach ($userrows as $k => $v) {
                $msgData['mobile'] = $v['moblie'];
                $msgData['content'] = trim($_POST['content']);
                $msgData['uid'] = $v['uid'];
                $msgData['cname'] = '系统';
                $msgData['port'] = '5';

                $sendid = $notice->sendSMS($msgData);
                if ($sendid['status'] != -1) {
                    $sendok++;
                } else {
                    $sendno++;
                }
            }
            $msg = '';
            if ($page < $num) {
                $msg = '发送中...';
                $status = 3;
            } else {
                $status = 0;
            }
            $msg .= '已发送短信成功：' . $sendok . '条';
            if ($sendno) {
                $msg .= ',失败：' . $sendno . '条';
            }

            $this->render_json($status, $msg);
        } else {
            $this->render_json(2, '没有符合条件号码，请先检查');
        }
    }

    // 公用提示
    function get_return($status, $value, $msg, $sendok = 0, $sendno = 0)
    {
        $data['value'] = $value;
        $data['sendok'] = $sendok;
        $data['sendno'] = $sendno;

        $this->render_json($status, $msg, $data);
    }
}

?>