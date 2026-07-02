<?php



class users_member_controller extends adminCommon
{
    //会员/个人

    /**
     * 个人用户列表高级搜索功能
     */
    private function set_search(&$source, &$search_list)
    {
        /* @var $arr_data */
        include(CONFIG_PATH . 'db.data.php');
        $source = $arr_data['source'];
        $search_list[] = array('param' => 'source', 'name' => 'admin_yunying_00139', 'value' => $source);
        $search_list[] = array('param' => 'status', 'name' => 'member_user_00181', 'value' => array('1' => 'admin_user_00149', '2' => 'admin_user_00150'));
        $search_list[] = array('param' => 'def_job', 'name' => 'admin_user_company_00294', 'value' => array('1' => '是', '2' => '否'));

        return compact('source', 'search_list');
    }

    /**
     * 会员-个人-个人用户列表：全部个人
     */
    function index_action()
    {


        $month = get_this_month(time());
        $userInfoM = $this->MODEL('userinfo');
        $resumeM = $this->MODEL('resume');
        $where = $memberWhere = array();
        //判断是否有简历
        if (isset($_POST['def_job']) && !empty($_POST['def_job'])) {
            if ($_POST['def_job'] == '1') {
                $where['def_job'] = array('>', 0);
            } else {
                $where['def_job'] = 0;
            }
        }
        //条件查询 ：搜索查询
        if ($_POST['keyword']) {
            $keytype = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);
            if ($keytype == 1) {
                $memberWhere['username'] = array('like', $keyword);
            } elseif ($keytype == 2) {
                $where['name'] = array('like', $keyword);
            } elseif ($keytype == 3) {
                $where['telphone'] = array('like', $keyword);
            } elseif ($keytype == 4) {
                $where['email'] = array('like', $keyword);
            } elseif ($keytype == 5) {
                $where['uid'][] = array('=', $keyword);
            } elseif ($keytype == 6) {
                $memberWhere['login_ip'] = array('like', $keyword);
            }
        }
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            $where['r_status'] = $status;
        }


        /* 新增，弹框自带搜索条件（数据看板弹窗） */
        if (isset($_POST['login_days']) && !empty($_POST['login_days'])) {

            $loginDays = intval($_POST['login_days']);

            if ($loginDays == -1) {   //  昨日

                $sdate  =   strtotime('today') - 86400;
                $edate  =   strtotime(date('Y-m-d'));
            } else if ($loginDays == 1) {

                $sdate  =   strtotime(date('Y-m-d'));
                $edate  =   time();
            } else if ($loginDays == 2) {

                $sdate  =   strtotime('-1 week');
                $edate  =   time();
            } else if ($loginDays == 3) {

                $sdate  =   strtotime('-1 month');
                $edate  =   time();
            } else if ($loginDays == 4) {

                $sdate  =   strtotime('-6 month');
                $edate  =   time();
            } else if ($loginDays == 5) {

                $sdate  =   strtotime('-12 month');
                $edate  =   time();
            }

            $where['PHPYUNBTWSTART_1C'] =   '';
            $where['login_date'][]      =   array('>=', $sdate);
            $where['login_date'][]      =   array('<=', $edate);
            $where['PHPYUNBTWEND_1C']   =   '';
        }
        if (isset($_POST['login_time']) && !empty($_POST['login_time'])) {

            $timeArr    =   $_POST['login_time'];
            $timeStart  =   date('Y-m-d', $timeArr[0] / 1000);
            $timeEnd    =   date('Y-m-d', $timeArr[1] / 1000);
            $sdate      =   strtotime($timeStart." 00:00:00");
            $edate      =   strtotime($timeEnd." 23:59:59");

            $where['PHPYUNBTWSTART_2C'] =   '';
            $where['login_date'][]      =   array('>=', $sdate);
            $where['login_date'][]      =   array('<=', $edate);
            $where['PHPYUNBTWEND_2C']   =   '';
        }

        if(is_array($_POST['times']) && count($_POST['times']) == 2){
            if($_POST['time_type'] == 'adtime'){
                $memberWhere['PHPYUNBTWSTART_C'] = '';
                $memberWhere['reg_date'][] = array('>=', strtotime($_POST['times'][0]. ' 00:00:00'));
                $memberWhere['reg_date'][] = array('<=', strtotime($_POST['times'][1]. ' 23:59:59'));
                $memberWhere['PHPYUNBTWEND_C'] = '';
            }
            if($_POST['time_type'] == 'lotime'){
                $where['PHPYUNBTWSTART_C'] = '';
                $where['login_date'][] = array('>=', strtotime($_POST['times'][0]. ' 00:00:00'));
                $where['login_date'][] = array('<=', strtotime($_POST['times'][1]. ' 23:59:59'));
                $where['PHPYUNBTWEND_C'] = '';
            }
        }

        /* 新增，弹框自带搜索条件（数据看板弹窗） */
        if (isset($_POST['reg_days']) && !empty($_POST['reg_days'])) {

            $regDays = intval($_POST['reg_days']);

            if ($regDays == -1) {   //  昨日

                $sdate  =   strtotime('today') - 86400;
                $edate  =   strtotime(date('Y-m-d'));
            } else if ($regDays == 1) {

                $sdate  =   strtotime(date('Y-m-d'));
                $edate  =   time();
            } else if ($regDays == 2) {

                $sdate  =   strtotime('-1 week');
                $edate  =   time();
            } else if ($regDays == 3) {

                $sdate  =   strtotime('-1 month');
                $edate  =   time();
            } else if ($regDays == 4) {

                $sdate  =   strtotime('-6 month');
                $edate  =   time();
            } else if ($regDays == 5) {

                $sdate  =   strtotime('-12 month');
                $edate  =   time();
            }

            $memberWhere['PHPYUNBTWSTART_1C']    =   '';
            $memberWhere['reg_date'][]           =   array('>=', $sdate);
            $memberWhere['reg_date'][]           =   array('<=', $edate);
            $memberWhere['PHPYUNBTWEND_1C']      =   '';
        }
        if (isset($_POST['reg_time']) && !empty($_POST['reg_time'])) {

            $timeArr    =   $_POST['reg_time'];
            $timeStart  =   date('Y-m-d', $timeArr[0] / 1000);
            $timeEnd    =   date('Y-m-d', $timeArr[1] / 1000);
            $sdate      =   strtotime($timeStart." 00:00:00");
            $edate      =   strtotime($timeEnd." 23:59:59");

            $memberWhere['PHPYUNBTWSTART_2C']    =   '';
            $memberWhere['reg_date'][]           =   array('>=', $sdate);
            $memberWhere['reg_date'][]           =   array('<=', $edate);
            $memberWhere['PHPYUNBTWEND_2C']      =   '';
        }

        if (isset($_POST['source']) && !empty($_POST['source'])) {
            $memberWhere['source'] = intval($_POST['source']);
        }
        if (isset($memberWhere) && !empty($memberWhere)) {
            $memberUids = array();
            $memberList = $userInfoM->getList($memberWhere, array('field' => '`uid`'));
            foreach ($memberList as $val) {
                $memberUids[] = $val['uid'];
            }
            $where['uid'] = array('in', pylode(',', $memberUids));
        }

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('resume', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        if ($pages['total'] > 0) {
            if (!empty($_POST['order'])) {
                if ($_POST['t'] == 'time') {
                    $where['orderby'] = 'lastupdate,' . $_POST['order'];
                } else {
                    $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
                }
            } else {
                $where['orderby'] = 'uid';
            }
            $where['limit'] = $pages['limit'];
            $list = $resumeM->getResumeList($where, array('utype' => 'admin'));
            foreach ($list as $key => $val) {
                $list[$key]['wxBindmsg'] = $this->wxBindState($val);
            }
        }
        $this->render_json(0, 'ok', compact( 'list', 'total', 'page_sizes', 'limit', 'page','accept'));
    }
    public function  getConfigData_action(){
        $source = $search_list = array();
        $this->set_search($source, $search_list);
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');
        $domainList = !empty($domain['Dname']) ? (object)$domain['Dname'] : (object)[];
        $this->render_json(0,'',compact('search_list','source','domainList'));
    }

    /**
     * 会员-个人-个人用户列表: 全部个人（页面统计数量）
     */
    function userNum_action()
    {
        $MsgNum = $this->MODEL('msgNum');
        echo $MsgNum->userNum();
    }

    /**
     * 会员-个人-个人用户列表:添加会员保存
     */
    function add_action()
    {
        if(isset($_POST['add']) && $_POST['add']){
            $this->render_json(0);
        }else{
            if ($_POST['username'] == '' || mb_strlen($_POST['username']) < 2 || mb_strlen($_POST['username']) > 16) {
                $this->render_json(1, yun_at('admin_user_00084'));
            } elseif ($_POST['password'] == '' || mb_strlen($_POST['password']) < 6 || mb_strlen($_POST['password']) > 20) {
                $this->render_json(1, yun_at('admin_user_00085'));
            } elseif ($_POST['moblie'] == '') {
                $this->render_json(1, yun_at('admin_01285'));
            }

            $userinfoM = $this->MODEL('userinfo');

            $result = $userinfoM->addMemberCheck($_POST);

            if ($result['msg']) {
                $this->render_json(1, $result['msg']);
            }

            if ($this->config['sy_uc_type'] == 'uc_center') {
                $this->obj->uc_open();

                $user = uc_get_user($_POST['username']);

                if (is_array($user)) {
                    $this->render_json(1, yun_at('common_01400'));
                }
            }
            $time = time();
            $ip = fun_ip_get();
            $pass = $_POST['password'];

            if ($this->config['sy_uc_type'] == 'uc_center') {
                $result = uc_user_register($_POST['username'], $pass, $_POST['email']);

                if ($result < 0) {
                    switch ($result) {
                        case '-1' :
                            $data['msg'] = yun_at('common_01387');
                            break;
                        case '-2' :
                            $data['msg'] = yun_at('common_00858');
                            break;
                        case '-3' :
                            $data['msg'] = yun_at('common_01288');
                            break;
                        case '-4' :
                            $data['msg'] = yun_at('common_00834');
                            break;
                        case '-5' :
                            $data['msg'] = yun_at('common_00762');
                            break;
                        case '-6' :
                            $data['msg'] = yun_at('common_00673');
                            break;
                    }

                    $this->render_json(1, $data['msg']);
                } else {
                    list($uid, $username, $email, $password, $salt) = uc_get_user($_POST['username']);
                }
            } else {
                $salt = substr(uniqid(rand()), -6);
                $password = passCheck($pass, $salt);
            }
            $mdata = array(
                'username' => $_POST['username'],
                'password' => $password,
                'usertype' => 1,
                'salt' => $salt,
                'moblie' => $_POST['moblie'],
                'email' => $_POST['email'],
                'reg_date' => $time,
                'reg_ip' => $ip,
                'status' => 1
            );
            $udata = array(
                'email' => $_POST['email'],
                'telphone' => $_POST['moblie'],
                'r_status' => 1
            );

            $nid = $userinfoM->addInfo(array('mdata' => $mdata, 'udata' => $udata));

            if ($nid > 0) {
                $this->admin_json(0, yun_t('admin_model_00106', array('id' => $nid)));
            } else {
                $this->render_json(1, yun_at('admin_user_00076'));
            }
        }

    }

    /**
     * 会员-个人-个人用户列表: 修改页面
     */
    function edit_action()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $uid = intval($_POST['uid']);

        $userinfoM = $this->MODEL('userinfo');

        $member = $userinfoM->getInfo(array('uid' => $uid));

        $resumeM = $this->MODEL('resume');

        $resume = $resumeM->getResumeInfo(array('uid' => $uid), array('logo' => 2));

        $resume = !empty($resume) ? $resume : '';

        $return = $resumeM->getInfo(array(
            'uid' => $uid,
            'eid' => $resume['def_job'],
            'tb' => 'all',
            'needCache' => 1
        ));

        $expectData = array(
            'uid' => $uid,
            'expect' => $return['expect'],
            'edu' => $return['edu'],
            'other' => $return['other'],
            'project' => $return['project'],
            'skill' => $return['skill'],
            'training' => $return['training'],
            'work' => $return['work'],
            'salary' => $resumeM->salary()
        );

        $user_sex = $return['cache']['user_sex'];
        $userdata = $return['cache']['userdata'];
        $userclass_name = $return['cache']['userclass_name'];
        $industry_index = $return['cache']['industry_index'];
        $industry_name = $return['cache']['industry_name'];

        $this->render_json(0, 'ok', compact('member', 'resume', 'expectData', 'user_sex', 'userdata', 'userclass_name', 'industry_index', 'industry_name'));
    }

    /**
     * 会员-个人-个人用户列表: 修改保存
     */
    function editSave_action()
    {
        $_POST = $this->post_trim($_POST);

        if (empty($_POST) || empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $uid = intval($_POST['uid']);

        $mData = array(
            'moblie' => $_POST['moblie'],
            'email' => $_POST['email'],
        );

        if ($_FILES['file']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['file'],
                'dir' => 'user'
            );
        }
        $uploadM = $this->MODEL('upload');
        if (!empty($upArr)) {
            $pic = $uploadM->newUpload($upArr);
        }
        if (!empty($pic['picurl'])) {
            $picture = $pic['picurl'];
        }
        $rData = array(
            'name' => $_POST['name'],
            'sex' => $_POST['sex'],
            'birthday' => $_POST['birthday'],
            'exp' => $_POST['exp'],
            'edu' => $_POST['edu'],
            'telphone' => $_POST['telphone'],
            'email' => $_POST['email'],
            'domicile' => $_POST['domicile'],
            'living' => $_POST['living'],
            'marriage' => $_POST['marriage'],
            'height' => $_POST['height'],
            'nationality' => $_POST['nationality'],
            'weight' => $_POST['weight'],
            'idcard' => $_POST['idcard'],
            'address' => $_POST['address'],
            'homepage' => $_POST['homepage'],
            'qq' => $_POST['qq'],
            'description' => $_POST['description']
        );
        !empty($picture) && $rData['wxewm'] = $picture;

        $resumeM = $this->MODEL('resume');
		
		$resume 	= 	$resumeM->getResumeInfo(array('uid'=>$uid));
		if(!$resume['photo'] || ($resume['defphoto']==2 && $rData['sex']!=$resume['sex'])){
			$logo   =   $resumeM->deflogo($rData);
			if($logo!=''){
				$rData['photo'] 		= $logo;
				$rData['defphoto'] 		= 2;
				$rData['photo_status'] 	= 0;
			}
		}

        //修改个人基本信息
        $return = $resumeM->upResumeInfo(array('uid' => $uid), array('mData' => $mData, 'rData' => $rData, 'utype' => 'admin'));

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    // 个人认证总入口，入口整合，利于权限控制
    function usercert_action()
    {
        if (isset($_POST['batchfirm'])) {
            $this->batchfirm();
        } elseif (isset($_POST['email'])) {
            $this->emailstatus();
        } elseif (isset($_POST['moblie'])) {
            $this->mobliestatus();
        } else {
            $this->userStatus();
        }
    }

    /**
     * 列表-邮箱认证
     */
    function emailstatus()
    {
        $ResumeM = $this->MODEL('resume');
        $UserinfoM = $this->MODEL('userinfo');

        if ($_POST['email'] == "") {
            $this->render_json(1, yun_at('wap_01119'));
        } elseif (CheckRegEmail($_POST['email']) == false) {
            $this->render_json(1, yun_at('wap_js_00120'));
        }

        $uid = $_POST['uid'];
        $status = $_POST['estatus'];
        $email = $_POST['email'];

        $rInfo = $ResumeM->getResumeInfo(array('uid' => $uid), array('field' => '`email`, `email_status`'));

        if ($rInfo) {
            if ($rInfo['email'] == $email && $rInfo['email_status'] == 1) {
                $this->render_json(0, yun_at('admin_user_00080'));
            }

            $rdata = array('email_status' => $status, 'email' => $email);
            $nid = $ResumeM->upResumeInfo(array('uid' => $uid), array('rData' => $rdata));

            if ($nid) {
                $data = array('email' => $email, 'email_status' => $status);
                $UserinfoM->upInfo(array('uid' => $uid), $data);

                $this->obj->update_once('member', array('email' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'email' => $email));
                $this->obj->update_once('resume', array('email' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'email' => $email));
                $this->obj->update_once('company', array('linkmail' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'linkemail' => $email));
                $this->obj->update_once('lt_info', array('email' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'email' => $email));
                $this->obj->update_once('px_train', array('linkmail' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'linkmail' => $email));

                $msg = 'admin_01286' . $email;
                if (!empty($rInfo['email']) && $rInfo['email'] != $email) {

                    $msg .= 'admin_user_00091' . $rInfo['email'];
                }

                $this->MODEL('log')->addAdminLog(yun_t('admin_model_00108', array('uid' => $uid, 'email' => $email)));

                $this->admin_json(0, yun_t('admin_model_00107', array('uid' => $uid, 'detail' => $msg)));
            } else {
                $this->render_json(1, yun_at('admin_user_00089'));
            }
        } else {
            $this->render_json(1, yun_at('admin_user_00086'));
        }
    }

    /**
     * 列表-手机认证
     */
    function mobliestatus()
    {
        $_POST = $this->post_trim($_POST);

        $ResumeM = $this->MODEL('resume');
        $UserinfoM = $this->MODEL('userinfo');

        if ($_POST['moblie'] == "") {
            $this->render_json(1, yun_at('wap_user_00274'));
        } elseif (CheckMobile($_POST['moblie']) == false) {
            $this->render_json(1, yun_at('wap_user_00039'));
        }

        $uid = $_POST['uid'];
        $status = $_POST['mstatus'];

        $phone = $_POST['moblie'];

        $rInfo = $ResumeM->getResumeInfo(array('uid' => $uid), array('field' => '`telphone`,`moblie_status`'));

        if (!empty($rInfo)) {
            if ($rInfo['telphone'] == $phone && $rInfo['moblie_status'] == 1) {
                $this->render_json(0, yun_at('admin_user_00078'));
            }

            $data = array('moblie_status' => $status, 'telphone' => $phone);
            $nid = $ResumeM->upResumeInfo(array('uid' => $uid), array('rData' => $data));

            if ($nid) {
                $memberData = array('moblie_status' => $status, 'moblie' => $phone);

                $msg = 'admin_01287' . $phone;
                if (!empty($rInfo['telphone'])) {
                    $msg .= 'admin_user_00090' . $rInfo['telphone'];
                }
                // 获取用户信息，用来判断旧手机号和用户名是否一致
                $member = $UserinfoM->getInfo(array('uid' => $uid), array('field' => 'username,moblie'));
                if ($member['username'] == $member['moblie']) {

                    $memberData['username'] = $phone;
                    $msg .= 'admin_user_00081' . $phone;
                }
                // 有账号的用户名与新手机号一致的，将用户名改成新手机号
                $omb = $UserinfoM->getInfo(array('username' => $phone), array('field' => 'uid'));
                if (!empty($omb)) {
                    // 如果现有数据中，存在用户名是这个手机号的，要修改
                    $UserinfoM->upInfo(array('uid' => $omb['uid']), array('username' => $phone . '_s'));

                    $logDetail = 'admin_user_00077' . $uid . 'admin_user_00075' . $omb['uid'] . 'common_06620' . $phone . ' → ' . $phone . '_s）';

                    $logM = $this->MODEL('log');
                    $logM->addAdminLog($logDetail);
                }
                $UserinfoM->upInfo(array('uid' => $uid), $memberData);

                $this->obj->update_once('member', array('moblie' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'moblie' => $phone));
                $this->obj->update_once('resume', array('telphone' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'telphone' => $phone));
                $this->obj->update_once('company', array('linktel' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'linktel' => $phone));
                $this->obj->update_once('lt_info', array('moblie' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'moblie' => $phone));
                $this->obj->update_once('pr_train', array('linktel' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'linktel' => $phone));

                $this->admin_json(0, yun_t('admin_model_00109', array('uid' => $uid, 'detail' => $msg)));
            } else {
                $this->render_json(1, yun_at('admin_user_00087'));
            }
        } else {
            $this->render_json(1, yun_at('admin_user_00086'));
        }
    }

    /**
     * 列表底部-批量认证
     */
    function batchfirm()
    {
        $UserinfoM = $this->MODEL('userinfo');
        $ResumeM = $this->MODEL('resume');

        $msg = array();

        if (empty($_POST['type']) || !is_array($_POST['type'])) {
            $this->render_json(1, yun_at('admin_01288'));
        }
        if (!isset($_POST['status'])) {
            $this->render_json(1, yun_at('admin_01289'));
        }
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('member_com_00320'));
        }

        $type = $_POST['type'];
        $status = intval($_POST['status']);
        $ids = pylode(',', $_POST['uid']);
        $where['uid'] = array('in', $ids);

        $rows = $ResumeM->getResumeList($where, array('field' => '`uid`,`telphone`,`email`,`idcard_pic`,`idcard`,`email_status`,`moblie_status`,`idcard_status`'));

        if (is_array($rows) && $rows) {
            if (in_array('email', $type)) {
                array_push($msg, 'member_user_00282');

                foreach ($rows as $val) {
//                    if ($val['email'] || $val['email_status'] == 1) {
                        $emailuids[] = $val['uid'];
//                    }
                }

                $emaildata = array('email_status' => $status);
                $emailwhere['uid'] = array('in', pylode(',', $emailuids));

                $UserinfoM->upInfo($emailwhere, $emaildata);
                $ResumeM->upResumeInfo($emailwhere, array('rData' => $emaildata));
            }

            if (in_array('moblie', $type)) {
                array_push($msg, 'member_user_00163');

                foreach ($rows as $val) {
//                    if ($val['telphone'] || $val['moblie_status'] == 1) {
                        $telphoneuids[] = $val['uid'];
//                    }
                }

                $mobliedata = array('moblie_status' => $status);
                $mobliewhere['uid'] = array('in', pylode(',', $telphoneuids));

                $UserinfoM->upInfo($mobliewhere, $mobliedata);
                $ResumeM->upResumeInfo($mobliewhere, array('rData' => $mobliedata));
            }

            if (in_array('idcard', $type)) {
                array_push($msg, 'member_com_00014');

                foreach ($rows as $val) {
//                    if ($val['idcard_pic'] && $val['idcard'] || $val['idcard_status'] == 1) {
                        $idcarduids[] = $val['uid'];
//                    }
                }

                $idcarddata = array('idcard_status' => $status);
                $idcardwhere['uid'] = array('in', pylode(',', $idcarduids));

                $ResumeM->upResumeInfo($idcardwhere, array('rData' => $idcarddata));
            }

            $ty = $status == 1 ? yun_at('wap_user_00128') : yun_at('admin_user_00300');

            $this->admin_json(0, yun_t('admin_model_00110', array('items' => implode(',', $msg), 'status' => $ty, 'ids' => $ids)));
        }
    }

    /**
     * 列表: 认证审核
     */
    function userStatus()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $resumeM = $this->MODEL('resume');

        $post = array(
            'idcard_status' => intval($_POST['r_status']),
            'statusbody' => trim($_POST['statusbody'])
        );

        $return = $resumeM->statusCert($_POST['uid'], array('post' => $post));

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /**
     * 会员-个人-个人用户列表:全部个人（设置分站）
     */
    function checksitedid_action()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $did = intval($_POST['did']);

        $uid = pylode(',', $_POST['uid']);

        if (empty($uid)) {
            $this->render_json(1, yun_at('admin_user_00079'));
        }

        $siteM = $this->MODEL('site');

        $didData = array('did' => $did);

        $Table = array(
            'company_cert',
            'company_msg',
            'company_order',
            'look_job',
            'member',
            'member_statis',
            'resume',
            'resume_expect',
            'user_entrust',
            'userid_job'
        );

        $siteM->updDid(array('report'), array('p_uid' => array('in', $uid)), $didData);

        $siteM->updDid(array('company_pay'), array('com_id' => array('in', $uid)), $didData);

        $siteM->updDid($Table, array('uid' => array('in', $uid)), $didData);

        $this->admin_json(0, yun_t('admin_model_00111', array('ids' => $uid)));
    }

    /**
     * 账户信息-保存
     */
    function saveUser_action()
    {
        $_POST = $this->post_trim($_POST);

        if (empty($_POST['uid']) || empty($_POST['username']) || empty($_POST['status'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $userInfoM = $this->MODEL('userinfo');

        $uid = intval($_POST['uid']);
        $data = array(
            'username' => $_POST['username'],
            'password' => $_POST['password'],
            'status' => $_POST['status'],
            'lock_info' => $_POST['lock_info']
        );

        $result = $userInfoM->addMemberCheck($data, $uid, 'admin');

        if (!empty($result['msg'])) {
            $this->render_json(1, $result['msg']);
        } else {
            $return = $userInfoM->upInfo(array('uid' => $uid), $data);
            if ($return) {
                $this->admin_json(0, 'admin_user_00083');
            } else {
                $this->render_json(1, yun_at('admin_user_00082'));
            }
        }
    }

    /**
     * 个人账户合并，搜所目标企业名称；
     */
    function searchCom_action()
    {
        $companyList = array();

        if (isset($_POST['com_name']) && $_POST['com_name'] !== '') {
            $comM = $this->MODEL('company');
            $name = $this->post_trim($_POST['com_name']);

            $list = $comM->getList(array('name' => array('like', $name)), array('field' => '`uid`,`name`'));

            $com = $list['list'];

            if (is_array($com) && !empty($com)) {
                foreach ($com as $val) {
                    $companyList[] = array('uid' => $val['uid'], 'name' => $val['name'],);
                }
            }
        }

        $this->render_json(0, 'ok', compact('companyList'));
    }

    // 账户合并
    function merge_action()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $transferM = $this->MODEL('transfer');

        $data = array(
            'uid' => $_POST['uid'],
            'com_uid' => $_POST['com_uid'],
            'mobile' => $_POST['mobile'],
            'email' => $_POST['email'],
            'QQ' => $_POST['QQ'],
            'wx' => $_POST['wx'],
            'sina' => $_POST['sina']
        );

        $return = $transferM->mergeData($data);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /**
     * 重置密码
     */
    function reset_pw_action()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $userinfoM = $this->MODEL('userinfo');

        $uid = intval($_POST['uid']);
        $userinfoM->upInfo(array('uid' => $uid), array('password' => '123456'));

        $this->admin_json(0, yun_t('admin_model_00112', array('uid' => $uid)));
    }

    /**
     * 会员-个人-个人用户列表: 全部个人（删除）
     */
    function del_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $userinfoM = $this->MODEL('userinfo');

        $return = $userinfoM->delInfo($_POST['del'], 1, $_POST['delAccount']);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /**
     * 会员  -  : 解绑日志
     */
    function writtenOffLog_action()
    {
        $where['opera'] = 12;

        $where['PHPYUNBTWSTART_A'] = '';
        $where['content'][] = array('like', 'wap_user_00138');
        $where['content'][] = array('like', 'wap_js_00065', 'OR');
        $where['content'][] = array('like', 'common_02028', 'OR');
        $where['PHPYUNBTWEND_A'] = '';

        $where['usertype'] = 1; // TODO 建议各写各的，统一走这里权限会导致加载不了

        if ($_POST['keyword']) {
            $type = intval($_POST['type']);
            $keyword = trim($_POST['keyword']);

            if ($type == 1) {
                $userInfoM = $this->MODEL('userinfo');
                $member = $userInfoM->getList(array('username' => array('like', $keyword)), array('field' => '`uid`'));
                if ($member) {
                    $muids = array();
                    foreach ($member as $val) {
                        $muids[] = $val['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $muids));
                }else{
                    $where['uid'] = -1;
                }
            } elseif ($type == 2) {
                $where['content'] = array('like', $keyword);
            }
        }

        if ($_POST['time_start'] != "" && $_POST['time_end'] != "") {
            $where['PHPYUNBTWSTART'] = '';
            $where['ctime'][] = array('>=', strtotime($_POST['time_start']));
            $where['ctime'][] = array('<=', strtotime($_POST['time_end'] . ' 23:59:59'));
            $where['PHPYUNBTWEND'] = '';
        }

        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('member_log', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];
        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id';
            }
            $where['limit'] = $pages['limit'];

            $logM = $this->MODEL('log');
            $list = $logM->getMemlogList($where, array('utype' => 'admin'));
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    /**
     * 会员 - ： 解绑日志删除
     */
    function delwflog_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $del = $_POST['del'];
        $ids = pylode(',', $del);
        if (is_array($_POST['del'])) {
            $where = array('id' => array('in', $ids));
        } else {
            if (trim($_POST['del']) == 'all') {
                $ids = 'all';
                $where = array('usertype' => 1, 'opera' => '12');
            } else {
                $where = array('id' => $ids);
            }
        }

        $logM = $this->MODEL('log');

        $return = $logM->delMemlog($where);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $ids == 'all' ? yun_t('admin_01290') : yun_t('admin_model_00113', array('ids' => $ids)));
        } else {
            $this->render_json(1, yun_at('admin_01292'));
        }
    }

    // 会员日志-个人日志
    function log_action()
    {
        $logM = $this->MODEL('log');
        $memberM = $this->MODEL('userinfo');

        $where['usertype'] = 1;
        if (isset($_POST['uid'])) {
            $where['uid'] = intval($_POST['uid']);
        }

        $keywordStr = trim($_POST['keyword']);

        if (!empty($keywordStr)) {
            if ($_POST['type'] == 1) {
                $member = $memberM->getList(array('username' => array('like', $keywordStr)), array('field' => '`uid`,`username`'));
                foreach ($member as $v) {
                    $uid[] = $v['uid'];
                }
                $where['uid'] = array('in', pylode(",", $uid));
            } elseif ($_POST['type'] == 3) {
                $where['uid'] = $keywordStr;
            }
        }

        $contentStr = trim($_POST['content']);

        if (!empty($contentStr)) {
            $logDetailList = $this->obj->select_all('member_log_detail', array('detail' => array('like', $contentStr)), 'log_id');
            if (!empty($logDetailList)) {
                $logIds = array();
                foreach ($logDetailList as $lk => $lv) {

                    $logIds[] = $lv['log_id'];
                }

                $where['PHPYUNBTWSTART_A'] = '';
                $where['id'] = array('in', pylode(',', $logIds), '');
                $where['content'] = array('like', $contentStr, 'OR');
                $where['PHPYUNBTWEND_A'] = '';
            } else {
                $where['content'] = array('like', $contentStr);
            }
        }

        if (!empty($_POST['end'])) {
            if ($_POST['end'] == '1') {
                $where['ctime'] = array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {
                $where['ctime'] = array('>=', strtotime('-' . (int)$_POST['end'] . 'day'));
            }
        }

        if (!empty($_POST['operas'])) {
            $operaStr = intval($_POST['operas']);

            $operaSql = array(
                '1' => array('name' => array('wap_user_00154')),
                '2' => array('name' => array('common_01951', 'wap_com_00428', 'common_02021')),
                '3' => array('name' => array('wap_00070')),
                '4' => array('name' => array('common_02040')),
                '5' => array('name' => array('wap_00379', 'common_01949', 'member_user_00242')),
                '6' => array('name' => array('wap_00574', 'common_01991', 'common_01982', 'common_01971')),
                '7' => array('name' => array('wap_00456')),
                '8' => array('name' => array('member_user_00226')),
                '9' => array('name' => array('wap_user_00220')),
                '11' => array('name' => array('admin_user_00140', 'common_02035')),
                '12' => array('name' => array('member_com_00093', 'common_02028', 'member_user_00234', 'member_user_00236', 'common_02034', 'admin_user_00171', 'member_user_00235')),
                '14' => array('name' => array('member_com_00293', 'common_01937')),
                '15' => array('name' => array('wap_00317', 'common_01954')),
                '16' => array('name' => array('wap_js_00081', 'member_user_00161', 'LOGO', 'common_02012', 'default_00092', 'admin_tool_00428', 'common_01886', 'member_com_00077')),
                '17' => array('name' => array('admin_yunying_00117', 'wap_user_00008'), 'realId' => 17),
                '18' => array('name' => array('common_01967', 'common_01965', 'common_02015', 'wap_user_00363')),
                '19' => array('name' => array('wap_user_00223')),
                '22' => array('name' => array('admin_tool_00428')),
                '23' => array('name' => array('wap_com_00350')),
                '25' => array('name' => array('wap_com_00357', 'admin_user_company_00379')),
                '26' => array('name' => array('wap_user_00221', 'member_user_00044')),
                '29' => array('name' => array('common_02046')),
                '88' => array('name' => array('common_02029'))
            );

            if (array_key_exists($operaStr, $operaSql)) {
                if (count($operaSql[$operaStr]['name']) == 1) {
                    $where['content'] = array('like', $operaSql[$operaStr]['name'][0]);
                } else {
                    $where['PHPYUNBTWSTART'] = '';
                    foreach ($operaSql[$operaStr]['name'] as $oV) {
                        $where['content'][] = array('like', $oV, 'OR');
                    }
                    $where['PHPYUNBTWEND'] = '';
                }
            } elseif (!empty($operasStr)) {
                $where['opera'] = $operaStr;
            }
        }

        if (isset($_POST['parrs'])) {
            $where['type'] = intval($_POST['parrs']);
        }

        if ($_POST['time_start'] != "" && $_POST['time_end'] != "") {
            $where['ctime'][] = array('>=', strtotime($_POST['time_start']));
            $where['ctime'][] = array('<=', strtotime($_POST['time_end'] . " 23:59:59"));
        }

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('member_log', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pages['limit'];

            $list = $logM->getMemlogList($where, array('utype' => 'admin'));
            $weekArr = array('wap_00014', 'wap_00008', 'wap_00010', 'wap_00009', 'wap_00013', 'wap_00011', 'wap_00012');
            foreach($list as $key => $val) {
                $list[$key]['date_n'] = date('Y-m-d', $val['ctime']);
                $list[$key]['time_n'] = date('H:i', $val['ctime']);
                $list[$key]['week'] = $list[$key]['date_n'] == date('Y-m-d') ? 'common_01940' : $weekArr[date('w', $val['ctime'])];
            }
        }



        $this->render_json(0, 'ok', compact('search_list', 'list', 'total', 'page_sizes', 'limit', 'page', 'last_page'));
    }
    public function getSearchData_action(){
        $opera = array('88' => 'admin_user_00157', '2' => 'wap_com_00428', '6' => 'wap_00574', '5' => 'wap_user_00193', '7' => 'wap_00456', '11' => 'admin_user_00152', '8' => 'member_user_00226', '12' => 'member_com_00093', '16' => 'wap_js_00081', '17' => 'common_06524', '18' => 'wap_user_00365', '19' => 'wap_user_00223', '23' => 'wap_com_00350', '25' => 'admin_user_00154', '26' => 'wap_user_00221');
        $search_list[] = array('param' => 'operas', 'name' => 'admin_user_00155', 'value' => $opera);

        $parr = array('1' => 'admin_user_00156', '2' => 'wap_js_00073', '3' => 'wap_js_00077', '4' => 'wap_user_00334');
        $search_list[] = array('param' => 'parrs', 'name' => 'wap_com_00030', 'value' => $parr);

        $ad_time = array('1' => 'common_01940', '3' => 'admin_user_00179', '7' => 'admin_user_00178', '15' => 'admin_user_00180', '30' => 'admin_user_00175');
        $search_list[] = array("param" => "end", "name" => 'member_user_00241', "value" => $ad_time);
        $this->render_json(0, 'ok', compact('search_list'));
    }
    // 会员日志删除
    function logDel_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $logM = $this->MODEL('log');

        $del = $_POST['del'];
        $ids = pylode(',', $del);
        if (is_array($del)) {
            $where['id'] = array('in', $ids);
        } else {
            if (trim($_POST['del']) == 'all') {
                $ids = 'all';
                $where['usertype'] = 1;
            } else {
                $where['id'] = $ids;
            }
        }

        $return = $logM->delMemlog($where);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $ids == 'all' ? yun_t('admin_01293') : yun_t('admin_model_00114', array('ids' => $ids)));
        } else {
            $this->render_json(1, yun_at('admin_01294'));
        }
    }

    /**
     * 职位申请记录
     */
    function jobSqLog_action()
    {
        $JobM = $this->MODEL('job');

        isset($_POST['uid']) && $where['uid'] = $_POST['uid'];
        isset($_POST['eid']) && $where['eid'] = $_POST['eid'];

        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('userid_job', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pages['limit'];

            $list = $JobM->getSqJobList($where, array('utype' => 'admin'));

            foreach ($list as $key => $val) {
                $list[$key]['company_show'] = $val['com_url'];
                $list[$key]['job_comapply'] = $val['job_url'];
            }
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    /**
     * 面试邀请记录
     */
    function yqmsLog_action()
    {
        $JobM = $this->MODEL('job');

        $where['uid'] = $_POST['uid'];

        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('userid_msg', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }
            $where['limit'] = $pages['limit'];

            $list = $JobM->getYqmsList($where, array('utype' => 'admin'));

            foreach ($list as $key => $val) {
                $list[$key]['company_show'] = Url('company', array('c' => 'show', 'id' => $val['fid'], 'look' => 'admin'));
                $list[$key]['job_comapply'] = Url('job', array('c' => 'comapply', 'id' => $val['jobid'], 'look' => 'admin'));
            }
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    /**
     * 积分管理
     */
    function payLog_action()
    {
        $orderM = $this->MODEL('companyorder');

        $where['com_id'] = $_POST['uid'];
        // $where['usertype'] = 1;

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company_pay', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = array();
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('id,desc');
            }

            $where['limit'] = $pages['limit'];

            $list = $orderM->getPayList($where);
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    /**
     * 会员-个人-个人用户列表: 全部个人（点用户名跳转会员中心）
     */
    function Imitate_action()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, yun_at('wap_com_00228'));
        }

        $userinfoM = $this->MODEL('userinfo');

        $member = $userinfoM->getInfo(array('uid' => intval($_POST['uid'])), array('field' => '`uid`,`username`,`salt`,`email`,`password`,`usertype`,`did`'));
        if (empty($member)) {
            $this->render_json(1, yun_at('wap_00341'));
        }

        $this->cookie->unset_cookie($_SESSION['auid']);

        $this->cookie->add_cookie($member['uid'], $member['username'], $member['salt'], $member['email'], $member['password'], 1, $this->config['sy_logintime'], $member['did'], '1');

        $logM = $this->MODEL('log');

        $content = yun_t('admin_model_00138', array('admin' => $_SESSION['ausername'], 'uid' => $member['uid']));

        $logM->addAdminLog($content);

        $this->render_json(0, 'ok', array('url' => $this->config['sy_weburl'] . '/member'));
    }
}
