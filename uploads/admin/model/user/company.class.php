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
class company_controller extends adminCommon
{
    function set_search(){
        $ratingM = $this->MODEL('rating');
        $rating = $ratingM->getList(array( 'category' => '1', 'orderby' => 'sort' ), array('field'=>'`id`,`name`'));
        if(!empty($rating)){
            $ratingarr = array();
            foreach($rating as $k => $v){
                $ratingarr[$v['id']] = $v['name'];
            }
        }
        include(CONFIG_PATH.'db.data.php');
        $source = $arr_data['source'];
        $cacheM = $this->MODEL('cache');
        $CacheList = $cacheM->GetCache(array('city'));
        $setArr	= array('city_name' => $CacheList['city_name']);
        $this->yunset($setArr);
        $status = array('1'=>'已审核', '2'=>'已锁定', '3'=>'未通过', '4'=>'未审核','5'=>'已暂停');
        $edtime = array('1'=>'7天内', '2'=>'一个月内', '3'=>'半年内', '4'=>'一年内', '5'=>'已到期','6'=>'当月');
        $isrec = array('1'=>'是', '2'=>'否', '3'=>'已到期');
        $isgw = array('-1'=>'已分配', '-2'=>'未分配');
        //顾问
        $adminM = $this->MODEL('admin');
        if($this->config['did']>0){
            $gwhere['did'] = $this->config['did'];
        }
        $gwhere['is_crm'] = '1';
        $gwList = $adminM->getList($gwhere);
        if ($gwList) {
            foreach ($gwList as $gwVal) {
                $isgw[$gwVal['uid']] = $gwVal['name'] ? $gwVal['name'] : $gwVal['username'];
            }
        }

        $search_list = array();
        $search_list['rating'] = array('name' => '会员等级', 'value' => $ratingarr);
        $search_list['time'] = array('name' => '到期时间', 'value' => $edtime);
        $search_list['status'] = array('name' => '审核状态', 'value' => $status);
        $search_list['source'] = array('name' => '数据来源', 'value' => $source);
        $search_list['rec'] = array('name' => '知名企业', 'value' => $isrec);
        $search_list['gw'] = array('name' => '企业顾问', 'value' => $isgw);
        $search_list['has_job'] = array('name' => '拥有职位', 'value' => array('1' => '是', '2' => '否'));
        $search_list['fact_status'] = array('name' => '实地核验', 'value' => array('1' => '是', '2' => '否'));
        $search_list['map_status'] = array('name' => '设置地图', 'value' => array('1' => '是', '2' => '否'));
        return array('source' => $source, 'ratingarr' => $ratingarr, 'search_list' => $search_list);
    }

    /**
     * @desc 后台 企业列表
     */
    function index_action(){
        $ComM = $this->MODEL('company');
        $where = array();
        $mwhere = array();
        $month = get_this_month(time());
        //判断是否有职位
        if(isset($_POST['has_job'])){
            if($_POST['has_job'] == '1'){
                $where['jobtime'] = array('>',0);// 最近一次发布/更新职位时间
            }elseif($_POST['has_job'] == '2'){
                $where['jobtime'] = 0;
            }
        }
        if($_POST['fact_status']){
            if($_POST['fact_status'] == '1'){
                $where['fact_status'] = 1;
            }elseif($_POST['fact_status'] == '2'){
                $where['fact_status'] = 0;
            }
        }
        if ($_POST['map_status']) {
            if ($_POST['map_status'] == '1') {
                $where['x'] = array('<>', '');
                $where['y'] = array('<>', '');
            } elseif ($_POST['map_status'] == '2') {
                $where['PHPYUNBTWSTART_D'] = '';
                $where['x'] = array('=', '');
                $where['y'] = array('=', '', 'OR');
                $where['PHPYUNBTWEND_D'] = '';
            }
        }
        if ($_POST['keyword']) {
            $keywordStr = trim($_POST['keyword']);
            $typeStr = intval($_POST['type']);
            if (!empty($keywordStr) && $typeStr == 1) {
                $where['PHPYUNBTWSTART_C'] = '';
                $where['name'] = array('like', $keywordStr);
                $where['shortname'] = array('like', $keywordStr, 'OR');
                $where['PHPYUNBTWEND_C'] = '';
            } elseif (!empty($keywordStr) && $typeStr == 2) {
                $mwhere['username'] = array('like', $keywordStr);
            } elseif (!empty($keywordStr) && $typeStr == 3) {
                $where['linkman'] = array('like', $keywordStr);
            } elseif (!empty($keywordStr) && $typeStr == 4) {
                $where['linktel'] = array('like', $keywordStr);
            } elseif (!empty($keywordStr) && $typeStr == 5) {
                $where['linkmail'] = array('like', $keywordStr);
            } elseif (!empty($keywordStr) && $typeStr == 6) {
                $where['uid'][] = array('=', $keywordStr);
            } elseif (!empty($keywordStr) && $typeStr == 7) {
                $mwhere['login_ip'] = array('like', $keywordStr);
            } elseif (!empty($keywordStr) && $typeStr == 8) {
                $where['address'] = array('like', $keywordStr);
            }
        }
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            if($status == 4){
                $where['r_status'] = 0;
            }else if($status == 5){
                $where['r_status'] = 4;
            }else{
                $where['r_status'] = $status;
            }
        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            if ($_POST['time_type'] == 'lotime') {
                $where['PHPYUNBTWSTART_C'] = '';
                $where['login_date'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
                $where['login_date'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
                $where['PHPYUNBTWEND_C'] = '';
            }
            if ($_POST['time_type'] == 'adtime') {
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
                $mwhere['reg_date'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            }
        }

        if($_POST['city_class']){
            $where['PHPYUNBTWSTARTB'] = '';
            $where['provinceid'] = array('findin', $_POST['city_class']);
            $where['cityid'] = array('findin', $_POST['city_class'], 'OR');
            $where['three_cityid'] = array('findin', $_POST['city_class'], 'OR');
            $where['PHPYUNBTWENDB']  = '';
        }
        if($_POST['source']){
            $mwhere['source'] = $_POST['source'];
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

            $mwhere['PHPYUNBTWSTART_1C']    =   '';
            $mwhere['reg_date'][]           =   array('>=', $sdate);
            $mwhere['reg_date'][]           =   array('<=', $edate);
            $mwhere['PHPYUNBTWEND_1C']      =   '';
        }
        if (isset($_POST['reg_time']) && !empty($_POST['reg_time'])) {

            $timeArr    =   $_POST['reg_time'];
            $timeStart  =   date('Y-m-d', $timeArr[0] / 1000);
            $timeEnd    =   date('Y-m-d', $timeArr[1] / 1000);
            $sdate      =   strtotime($timeStart." 00:00:00");
            $edate      =   strtotime($timeEnd." 23:59:59");

            $mwhere['PHPYUNBTWSTART_2C']    =   '';
            $mwhere['reg_date'][]           =   array('>=', $sdate);
            $mwhere['reg_date'][]           =   array('<=', $edate);
            $mwhere['PHPYUNBTWEND_2C']      =   '';
        }

        $mUids = array();
        $UserinfoM = $this->MODEL('userinfo');
        if(!empty($mwhere)){
            $uidList = $UserinfoM->getList($mwhere, array('field' => '`uid`'));
            if(!empty($uidList)){
                foreach($uidList as $uv){
                    $mUids[] = $uv['uid'];
                }
            }else{
                $mUids = array(0);
            }
            $where['uid'][] = array('in', pylode(',',$mUids));
        }
        if($_POST['rating']){
            $where['rating'] = $_POST['rating'];
        }
        $toDay = strtotime(date('Y-m-d'));
        if($_POST['time']){
            $time = intval($_POST['time']);
            if($time == 5){
                $where['PHPYUNBTWSTART_A'] = '';
                $where['vipetime'][] = array('>', '0', 'AND');
                $where['vipetime'][] = array('<', $toDay, 'AND');
                $where['PHPYUNBTWEND_A'] = '';
            }else if ($time == 6){
                $where['PHPYUNBTWSTART_A'] = '';
                $where['vipetime'][] = array('>=', $month['start_month'], 'AND');
                $where['vipetime'][] = array('<=', $month['end_month'], 'AND');
                $where['PHPYUNBTWEND_A'] = '';
            }else{
                if($time == 1){
                    $num = '+7 day';
                }elseif($time == 2 ){
                    $num = '+1 month';
                }elseif($time == 3){
                    $num = '+6 month';
                }elseif($time == 4){
                    $num = '+1 year';
                }
                $where['PHPYUNBTWSTART_B'] = '';
                $where['vipetime'][] = array('>', time(), 'AND');
                $where['vipetime'][] = array('<', strtotime($num), 'AND');
                $where['PHPYUNBTWEND_B'] = '';
            }
        }
        if($_POST['rec']){
            $rec = intval($_POST['rec']);
            if($rec == 1){
                $where['rec'] = '1';
                $where['hottime'] = array('>', time());
            }elseif ($rec == 2){
                $where['rec'] = '0';
            }elseif ($rec == 3){
                $where['rec'] = '1';
                $where['hottime'] = array('<', time());
            }
        }
        if($_POST['gw']){
            if (intval($_POST['gw']) == -1) {
                $where['crm_uid'] = array('<>', '0');
            } elseif (intval($_POST['gw']) == -2) {
                $where['crm_uid'] = '0';
            } else {
                $where['crm_uid'] = intval($_POST['gw']);
            }
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

        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company', $where, $page, array('limit' => $pageSize));
        //分页数大于0的情况下 执行列表查询
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'] . ',' . $_POST['order'];
            }else if($_POST['time'] == '5'){
                $where['orderby'] =	array('vipetime,desc','uid,desc');
            }else if($_POST['time']){
                $where['orderby'] =	array('vipetime,asc');
            }else if($_POST['lotime']){
                $where['orderby'] =	array('login_date,desc');
            }else{
                $where['orderby'] =	'uid,desc';
            }
            $where['limit'] = $pages['limit'];
            $ListNew = $ComM->getList($where,array('utype' => 'admin'));
            foreach ($ListNew['list'] as $key => $val){
                $ListNew['list'][$key]['wxBindmsg'] = $this->wxBindState($val);
            }
            unset($where['limit']);
        }
        $rt['list'] = $ListNew['list'] ? $ListNew['list'] : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $_SESSION['comXls'] = $where;
        $this->render_json(0, '', $rt);
    }

    public function getCache_action(){
        //顾问
        $adminM = $this->MODEL('admin');
        if($this->config['did']>0){
            $gwhere['did'] = $this->config['did'];
        }
        $gwhere['is_crm'] =	'1';
        $gwInfo = $adminM->getList($gwhere);
        //来源
        $sres = $this->set_search();
        $source = $sres['source'];
        //会员信息
        $ratingarr = $sres['ratingarr'];
        //搜索数据
        $search_list = $sres['search_list'];
        //海报信息
        $hbArr = [1,2,3,4,5,6,7,8,9,10];
        $hbBgA = array();
        foreach ($hbArr as $v) {
            $hbBgA[] = $this->config['sy_weburl'].'/data/upload/whb/logo/'.$v.'.png';
        }
        //配置信息
        $config = array();
        $config['com_social_credit'] = $this->config['com_social_credit'];
        $config['com_cert_owner'] = $this->config['com_cert_owner'];
        $config['com_cert_wt'] = $this->config['com_cert_wt'];
        $config['com_cert_other'] = $this->config['com_cert_other'];
        $config['com_free_status'] = $this->config['com_free_status'];

        $config['pricename'] = $this->config['integral_pricename'];

        $config['today_etime'] = strtotime(date('Y-m-d 23:59:59'));
        $config['today'] = strtotime(date('Y-m-d'));

        //分站
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');

        $data = array('gwinfo'=>$gwInfo,
            'source'=>$source,
            'ratingarr'=>$ratingarr,
            'search_list'=>$search_list,
            'hbBgA' => $hbBgA,
            'config'=>$config,
            'dname'=>(object)$domain['Dname'],
            'mapurl' => $this->config['mapurl'],
            'mapsecret' => $this->config['map_secret'],
        );
        $this->render_json(0,'',$data);
    }

    /**
     * @desc   后台企业列表 --  添加企业  -- 提交表单
     */
    function add_action(){
        $cacheM = $this->MODEL('cache');
        $options = array('com','city','hy');
        $cache = $cacheM->GetCache($options);
        $citys = array();
        foreach ($cache['city_index'] as $fir_v) {
            $citys[$fir_v] = array('value' => $fir_v, 'label' => $cache['city_name'][$fir_v]);
            if ($cache['city_type'][$fir_v]) {// 二级城市
                $citys[$fir_v]['children'] = array();
                foreach ($cache['city_type'][$fir_v] as $sec_v) {
                    $citys[$fir_v]['children'][$sec_v] = array('value' => $sec_v, 'label' => $cache['city_name'][$sec_v]);
                    if ($cache['city_type'][$sec_v]) {// 三级城市
                        $citys[$fir_v]['children'][$sec_v]['children'] = array();
                        foreach ($cache['city_type'][$sec_v] as $thi_v) {
                            $citys[$fir_v]['children'][$sec_v]['children'][$thi_v] = array('value' => $thi_v, 'label' => $cache['city_name'][$thi_v]);
                        }
                    }
                }
            }
        }
        $citys = array_values($citys);
        foreach ($citys as $k => $v) {
            if ($v['children']) {
                $citys[$k]['children'] = array_values($v['children']);
                foreach ($citys[$k]['children'] as $kk => $vv) {
                    if ($vv['children']) {
                        $citys[$k]['children'][$kk]['children'] = array_values($vv['children']);
                    }
                }
            }
        }
        $cache['cities'] = $citys;
        $rt['cache'] = $cache;
        $noticeM = $this->MODEL('notice');
        if ($_POST['submit']) {
            if ($_POST['username'] == '' || mb_strlen($_POST['username']) < 2 || mb_strlen($_POST['username']) > 16) {
                $this->render_json(1, '用户名格式错误');
            } elseif ($_POST['password'] == '' || mb_strlen($_POST['password']) < 6 || mb_strlen($_POST['password']) > 20) {
                $this->render_json(1, '密码格式错误');
            }
            $mPost = array(
                'username' => trim($_POST['username']),
                'companyName' => trim($_POST['name']),
                'moblie' => trim($_POST['moblie']),
                'email' => trim($_POST['email'])
            );
            $userinfoM = $this->MODEL('userinfo');
            $result = $userinfoM->addMemberCheck($mPost);
            if ($result['msg']){
                $this->render_json(1, $result['msg']);
            }
            if($this->config['sy_uc_type'] == 'uc_center'){
                $this->obj->uc_open();
                $user = uc_get_user($_POST['username']);
                if(is_array($user)){
                    $this->render_json(1, '该会员已经存在！');
                }
            }
            $time = time();
            $ip = fun_ip_get();
            $pass = $_POST['password'];
            if($this->config['sy_uc_type'] == 'uc_center'){
                $uid = uc_user_register($_POST['username'], $pass, $_POST['email']);
                if($uid < 0){
                    switch($uid){
                        case '-1' : $data['msg'] = '用户名不合法!';
                            break;
                        case '-2' : $data['msg'] = '包含不允许注册的词语!';
                            break;
                        case '-3' : $data['msg'] = '用户名已经存在!';
                            break;
                        case '-4' : $data['msg'] = 'Email 格式有误!';
                            break;
                        case '-5' : $data['msg'] = 'Email 不允许注册!';
                            break;
                        case '-6' : $data['msg'] = '该 Email 已经被注册!';
                            break;
                    }
                    $this->render_json(1, $data['msg']);
                }else{
                    list($uid, $username, $email, $password, $salt) = uc_get_user($_POST['username'], $pass);
                }
            }else{
                $pwdRes = $userinfoM->generatePwd(array('password' => $pass));
                $salt = $pwdRes['salt'];
                $password = $pwdRes['pwd'];
            }
            $mdata = array(
                'username' => trim($_POST['username']),
                'password' => $password,
                'usertype' => 2,
                'salt' => $salt,
                'address' => $_POST['address'],
                'moblie' => $_POST['moblie'],
                'email' => $_POST['email'],
                'reg_date' => $time,
                'reg_ip' => $ip,
                'status'=> 1
            );
            if($_POST['areacode'] && $_POST['telphone']){
                $_POST['phone'] = $_POST['areacode'] . '-' . $_POST['telphone'];
                if($_POST['exten']){
                    $_POST['phone'] .= '-'.$_POST['exten'];
                }
            }
            $udata = array(
                'name' => $_POST['name'],
                'shortname' => $_POST['shortname'],
                'hy' => $_POST['hy'],
                'pr' => $_POST['pr'],
                'mun' => $_POST['mun'],
                'provinceid' => $_POST['provinceid'],
                'cityid' => $_POST['cityid'],
                'three_cityid' => $_POST['three_cityid'],
                'address' => $_POST['address'],
                'x' => $_POST['x'],
                'y' => $_POST['y'],
                'linkman' => $_POST['linkman'],
                'linktel' => $_POST['moblie'],
                'linkphone' => $_POST['phone'],
                'linkmail' => $_POST['email'],
                'content' => $_POST['content'],
                'lastupdate' => time(),
                'r_status' => 1
            );
            $sdata = array(
                'integral' => intval($_POST['integral']),
                'rating' => intval($_POST['rating_name'])
            );
            $nid = $userinfoM->addInfo(array('mdata' => $mdata,'udata' => $udata, 'sdata' => $sdata));
            if($nid > 0){
                if($_POST['sendemail'] && $_POST['email']){
                    $noticeM->sendEmailType(array('name' => $mdata['username'], 'username' => $mdata['username'],'password' => $_POST['password'], 'email' => $_POST['email'], 'type' => 'reg', 'uid' => $nid));
                }
                if($_POST['sendmsg'] && $_POST['moblie']){
                    if(checkMsgOpen($this->config)){
                        $noticeM->sendSMSType(array('name' => $mdata['username'], 'username' => $mdata['username'],'password' => $_POST['password'], 'moblie' => $_POST['moblie'], 'type' => 'reg', 'uid' => $nid, 'port' => 1));
                    }
                }
                $this->admin_json(0, '企业会员(ID:'.$nid.')添加成功！');
            }else{
                $this->render_json(1, '企业会员添加失败，请重试！');
            }
        }
        $rt['com_rating'] = $this->config['com_rating'];
        if(empty($cache['city_type'])){
            $rt['cionly'] = 1;
        }
        $rt['mapurl'] = $this->config['mapurl'];
        $rt['mapsecret'] = $this->config['map_secret'];
        $this->render_json(0, '', $rt);
    }
    /**
     * 检测用户名重复性
     */
    function checkUsername_action(){
        $userinfoM = $this->MODEL('userinfo');
        $result = $userinfoM->addMemberCheck(array('username'=>trim($_POST['username'])));
        $this->render_json($result['error'] ? 1 : 0, $result['msg']);
    }

    /**
     * @desc 企业列表 修改操作
     */
    function edit_action(){
        if($_POST['id']){
            $uid = intval($_POST['id']);
            $ComM = $this->MODEL('company');
            $row = $ComM->getInfo($uid, array('edit' => 1));
            $UserinfoM = $this->MODEL('userinfo');
            $com_info = $UserinfoM->getInfo(array('uid' => $uid));
            $StatisM = $this->MODEL('statis');
            $statis = $StatisM->getInfo($uid, array('usertype'=>2));
            $ratingM = $this->MODEL('rating');
            $rating_list = $ratingM->getList(array('category' => '1'), array('field'=>'`id`,`name`'));
            $cacheM = $this->MODEL('cache');
            $options = array('com','city','hy');
            $cache = $cacheM->GetCache($options);
            $citys = array();
            foreach ($cache['city_index'] as $fir_v) {
                $citys[$fir_v] = array('value' => $fir_v, 'label' => $cache['city_name'][$fir_v]);
                if ($cache['city_type'][$fir_v]) {// 二级城市
                    $citys[$fir_v]['children'] = array();
                    foreach ($cache['city_type'][$fir_v] as $sec_v) {
                        $citys[$fir_v]['children'][$sec_v] = array('value' => $sec_v, 'label' => $cache['city_name'][$sec_v]);
                        if ($cache['city_type'][$sec_v]) {// 三级城市
                            $citys[$fir_v]['children'][$sec_v]['children'] = array();
                            foreach ($cache['city_type'][$sec_v] as $thi_v) {
                                $citys[$fir_v]['children'][$sec_v]['children'][$thi_v] = array('value' => $thi_v, 'label' => $cache['city_name'][$thi_v]);
                            }
                        }
                    }
                }
            }
            $citys = array_values($citys);
            foreach ($citys as $k => $v) {
                if ($v['children']) {
                    $citys[$k]['children'] = array_values($v['children']);
                    foreach ($citys[$k]['children'] as $kk => $vv) {
                        if ($vv['children']) {
                            $citys[$k]['children'][$kk]['children'] = array_values($vv['children']);
                        }
                    }
                }
            }
            $cache['cities'] = $citys;
            $rt['cache'] = $cache;
            $rt['statis'] = $statis;
            $rt['com_info'] = $com_info;
            $rt['rating_list'] = $rating_list;
            $all_welfare = $row['arraywelfare'] ? $row['arraywelfare'] : array();
            foreach ($cache['comdata']['job_welfare'] as $v) {
                !in_array($cache['comclass_name'][$v], $all_welfare) && array_push($all_welfare, $cache['comclass_name'][$v]);
            }
            $row['all_welfare'] = $all_welfare;
            $rt['row'] = $row;
            $rt['city_name'] = $cache['city_name'];
        }
        $this->render_json(0, '', $rt);
    }

    /**
     * @desc   后台企业列表 --  修改 -- 基本资料 -- 提交表单
     */
    function comeditsave_action(){
        $_POST = $this->post_trim($_POST);
        $info = $_POST;
        $isAjax = isset($_POST['is_ajax']) ? true : false;
        $picture = '';
        $companyM =	$this->MODEL('company');
        // 新上传图片文件处理
        foreach ($_FILES['comqcode'] as $nk => $nv) {
            foreach ($nv as $nkk => $nvv) {
                $qcodepic_file[$nkk][$nk] = $nvv;
            }
        }
        if($_FILES['comqcode']['tmp_name']){
            $upArr = array(
                'file' => $qcodepic_file[0],
                'dir' => 'company'
            );
            $uploadM = $this->MODEL('upload');
            if(!empty($upArr)){
                $pic = $uploadM->newUpload($upArr);
                if(!empty($pic['picurl'])){
                    $picture = $pic['picurl'];
                }
            }
        }
        $comData = array(
            'name' => $info['name'],
            'shortname' => $info['shortname'],
            'hy' => $info['hy'],
            'pr' => $info['pr'],
            'mun' => $info['mun'],
            'linkman' => $info['linkman'],
            'linktel' => $info['linktel'],
            'linkphone' => $info['linkphone'],
            'linkmail' => $info['linkmail'],
            'address' => $info['address'],
            'moneytype' => $info['moneytype'],
            'money' => $info['money'],
            'linkqq' => $info['linkqq'],
            'website' => $info['website'],
            'provinceid' =>	$info['provinceid'],
            'cityid' => $info['cityid'],
            'three_cityid' => $info['three_cityid'],
            'content' => str_replace(array('&amp;','background-color:#ffffff','background-color:#fff','white-space:nowrap;'), array('&','background-color:','background-color:','white-space:'), $info['content']),
            'busstops' => $info['busstops'],
            'welfare' => $info['checked_welfare'],
            'lastupdate' => time(),
//            'admin_remark' => $_POST['admin_remark'],
            'comqcode' => $picture,
            'x' => $info['x'],
            'y' => $info['y']
        );
        if (isset($info['r_status'])) {
            $comData['r_status'] = $info['r_status'];
        }
        if (isset($info['infostatus'])){
            $comData['infostatus'] = $info['infostatus'];
        }
        if (isset($info['sdate'])){
            $comData['sdate'] = $info['sdate'];
        }
        if (isset($info['linkjob'])){
            $comData['linkjob'] = $info['linkjob'];
        }
        $row = $companyM->getInfo($info['uid'], array('edit' => 1));
        if($row['comqcode'] && $picture == ""){
            $comData['comqcode'] = $row['comqcode'];
        }
        //	处理账号信息
        $mData = array(
            'email' => $info['linkmail'],
            'moblie' => $info['linktel'],
            'address' => trim($info['address']),
        );
        // 处理company_statis中的字段
        $return = $companyM->setCompany(array('uid'=>intval($info['uid'])),array('mData'=>$mData,'comData'=>$comData,/*'sData'=>$sData,*/'utype'=>'admin'));
        delfiledir('../data/upload/tel/'.$info['uid']);
        if ($isAjax) {
            echo json_encode(array('error' => $return['errcode'], 'msg' => $return['msg']));die;
        } else {
            if ($return['errcode'] == 8){
                $this->render_json(1, $return['msg']);
            }else{
                $this->admin_json(0, $return['msg']);
            }
        }
    }

    /**
     * @desc 后台企业列表 --  修改 -- 会员套餐 -- 提交表单
     */
    function saveRating_action(){
        if($_POST){
            $cuid =	intval($_POST['uid']);
            $comM =	$this->MODEL('company');
            $sData = array(
                'rating' => $_POST['ratingid'],
                'vip_etime' => $_POST['vip_etime'] ? strtotime($_POST['vip_etime']) : 0,
                'job_num' => intval($_POST['job_num']),
                'breakjob_num' => intval($_POST['breakjob_num']),
                'down_resume' => intval($_POST['down_resume']),
                'invite_resume' => intval($_POST['invite_resume']),
                'zph_num' => intval($_POST['zph_num']),
                'top_num' => intval($_POST['top_num']),
                'urgent_num' =>	intval($_POST['urgent_num']),
                'rec_num' => intval($_POST['rec_num']),
            );
            $return = $comM->setStatisInfo($cuid, $sData);
            if ($return['errcode'] == 8){
                $this->render_json(1, $return['msg']);

            }else{
                $this->admin_json(0, $return['msg'].'(企业列表-修改信息-修改会员信息，ID：'.$cuid.')');
            }
        }

    }
    /**
     * 账户信息-保存
     */
    function saveUser_action()
    {
        $_POST = $this->post_trim($_POST);

        if (empty($_POST['uid']) || empty($_POST['username']) || empty($_POST['status'])) {
            $this->render_json(1, '参数错误');
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
                $this->admin_json(0, '账户信息修改成功');
            } else {
                $this->render_json(1, '账户信息修改失败');
            }
        }
    }
    /**
     * @desc   后台企业列表 -- 会员审核
     */
    function status_action(){
        $userinfoM = $this->MODEL('userinfo');
        $post = array(
            'status' => intval($_POST['status']),
            'lock_info' => trim($_POST['statusbody']),
            'lock_status' => isset($_POST['lock_status']) ? intval($_POST['lock_status']) : '',
            'single' => isset($_POST['single']) ? $_POST['single'] : '',
        );
        $uids = @explode(',', $_POST['uid']);
        $return = $userinfoM->status(array('uid' => array('in', pylode(',', $uids)), 'usertype' => 2), array('post' => $post));
        if (isset($_POST['single'])){
            if ($return['errcode'] == 9){
                if($_POST['atype'] == 1){
                    // 仅保存
                    $this->render_json(0, $return['msg']);
                }else{
                    // 下一个待审核企业
                    $ComM = $this->MODEL('company');
                    $row = $ComM->getCompanyInfo(array('r_status'=>0,'orderby'=>array('uid,desc')), array('field'=>'uid'));
                    if (!empty($row)){
                        $this->render_json(0, $return['msg'], array('uid' => $row['uid']));
                    }else{
                        $this->render_json(0, $return['msg']);
                    }
                }
            }else{
                $this->render_json(1, $return['msg']);
            }
        }else{
            if($return['errcode']==9){
                $this->admin_json(0, $return['msg']);
            }else{
                $this->render_json(1, $return['msg']);
            }
        }
    }

    // 新审核，增加企业详情
    function companyAudit_action(){
        $uid = intval($_POST['uid']);
        $ComM = $this->MODEL('company');
        $userinfoM = $this->MODEL('userinfo');
        $Info = $ComM->getInfo($uid, array('edit' => 1,'jlist' => 1));
        $member = $userinfoM->getInfo(array('uid' => $uid),array('field' => 'lock_info,status,login_address,login_ip,moblie_address,reg_date,login_date'));
        $Info['statusbody'] = trim($member['lock_info']);
        $Info['member_status'] = $member['status'];
        $Info['login_ip'] = $member['login_ip'];
        $Info['login_address'] = $member['login_address'];
        $Info['moblie_address'] = $member['moblie_address'];
        $Info['reg_date_n'] = $member['reg_date'] ? date('Y-m-d H:i', $member['reg_date']) : '';
        $Info['login_date_n'] = $member['login_date'] ? date('Y-m-d H:i', $member['login_date']) : '';
        $rt['Info'] = $Info;
        // 待审核数量
        $snum = $ComM->getCompanyNum(array('r_status' => 0, 'uid' => array('<>', $uid)));
        $rt['snum'] = $snum;
        $StatisM = $this->MODEL('statis');
        $statis = $StatisM->getInfo($uid, array('usertype'=>2));
        $ratingM = $this->MODEL('rating');
        $rating_list = $ratingM->getList(array('category' => '1'), array('field'=>'`id`,`name`'));
        $cacheM = $this->MODEL('cache');
        $options = array('com','city','hy');
        $cache = $cacheM->GetCache($options);
        $rt['cache'] = $cache;
        $rt['statis'] = $statis;
        $rt['rating_list'] = $rating_list;
        $this->render_json(0, '', $rt);
    }

    // 企业暂停
    function suspend_action(){
        $userinfoM = $this->MODEL('userinfo');

        $res = $this->MODEL('company')->jugdeSuspend(array('uid'=>$_POST['uid']));

        if ($res['errcode'] != 9) {
            $this->render_json(1, $res['msg']);
        }

        $post = array(
            'status' => 4,
        );
        $uids = @explode(',', $_POST['uid']);
        $return = $userinfoM->status(array('uid' => array('in', pylode(',', $uids)), 'usertype' => 2), array('post' => $post));
        if ($return['errcode'] == 9) {
            $this->MODEL('company')->setComStaticSub($_POST['uid']);
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * @desc   企业列表 删除
     */
    function del_action(){
        $uid = $_POST['del'];
        $userinfoM = $this->MODEL('userinfo');
        $return = $userinfoM->delInfo($uid, 2, $_POST['delAccount']);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /**
     * @desc 后台企业列表  --  修改会员等级  返回当前会员套餐数据
     */
    function getstatis_action(){
        $statisM = $this->MODEL('statis');
        $ComM = $this->MODEL('company');
        if($_POST['uid']){
            $uid = intval($_POST['uid']);
            $rating	= $statisM->getInfo($uid, array('usertype' => '2'));

            $hotjob	= $ComM->getHotJob(array('uid'=>$uid));
            if($hotjob['time_start'] <= time() && $hotjob['time_end'] >= time()){
                $rating['hotjob'] = 1;
            }else{
                $rating['hotjob'] = 2;
            }
            if($rating['vip_etime']>0){
                $rating['vipetime'] = date('Y-m-d',$rating['vip_etime']);
            }else{
                $rating['vipetime'] = '不限';
            }
            $rating['rating'] = intval($rating['rating'])>0?$rating['rating']:'';
            $this->render_json(0, '', $rating);
        }
    }

    /**
     * @desc 后台企业列表  --  修改会员等级  -- 弹出框  --选择会员等级返回套餐时间
     */
    function getrating_action(){
        $ratingM = $this->MODEL('rating');
        if($_POST['id']){
            $id = intval($_POST['id']);
            $uid = intval($_POST['uid']);
            $rating = $ratingM->changeRatingInfo($id,$uid);
            if($rating['vip_etime'] > 0){
                $rating['oldetime'] = $rating['vip_etime'];
                $rating['vipetime'] = date('Y-m-d',$rating['vip_etime']);
            }else{
                $rating['oldetime'] = 0;
                $rating['vipetime'] = '不限';
            }

            if($rating['max_time'] > 0){
                $rating['max_time'] = $rating['max_time'];
                $rating['max_time_n'] = date('Y-m-d',$rating['max_time']);
            }else{
                $rating['max_time'] = 0;
                $rating['max_time_n'] = '不限';
            }

            $this->render_json(0, '', $rating);
        }
    }

    /**
     * @desc 后台企业列表  --  修改会员等级  -- 弹出框  --确认修改会员等级
     */

    function uprating_action()
    {
        $rid = intval($_POST['rating']);
        $uid = intval($_POST['ratuid']);
        $ratingM = $this->MODEL('rating');
        $rating_info = $ratingM->getInfo(array('id' => $rid));
        if (empty($rid)) {
            $this->render_json(1, '请选择会员等级！');
        } else if ($uid) {
            $statisM = $this->MODEL('statis');
            $statis = $statisM->getInfo($uid, array('usertype' => 2));
            $companyM = $this->MODEL('company');

			$company	=	$companyM->getInfo($uid);

			if ($company['r_status'] == '4'){
                $this->render_json(8, '请先解除暂停，再进行套餐调整！');
			}

            if($rating_info['service_time'] == 0){
                $_POST['vip_etime'] = 0;
            }else{
                $_POST['vip_etime'] = strtotime(date('Y-m-d 23:59:59', strtotime($_POST['vipetime'])));
            }
            if(!empty($_POST['max_time']) && $_POST['vip_etime']>0){
                $_POST['max_time'] = strtotime(date('Y-m-d 23:59:59', strtotime($_POST['max_time'])));
            }else{
                $_POST['max_time'] = 0;
            }

            if($_POST['vip_etime']>0 && $_POST['max_time']>0 && $_POST['max_time']<$_POST['vip_etime']){
                $this->render_json(1, '最长有效期不能短于会员到期时间');
            }

            $data = array();
            $sData = array('rating','rating_name','integral','vip_etime','job_num','breakjob_num','down_resume','invite_resume','zph_num','top_num','urgent_num','rec_num','suspend_num','max_time');
            $msg = array();
            foreach ($_POST as $key => $value) {
                if(in_array($key, $sData)){
                    $value = $value ? $value : 0;
                    $data[$key] = $value;
                    if($key == 'integral' && $value != $statis[$key]){
                        $msg[] = " 会员".$this->config['integral_pricename']."：".$statis[$key]." -> ".$value;
                    }else if($key == 'vip_etime' && $value != $statis[$key]){
                        $vEtime	= $value ? date('Y-m-d', $value) : '不限';
                        $msg[] = " 会员到期时间：".$statis['vip_etime_n']." -> ".$vEtime;
                    }else if($key == 'max_time' && $value != $statis[$key]){
                        $maxtime = $value ? date('Y-m-d', $value) : '不限';
                        $msg[] = " 会员最长有效期：".$statis['max_time_n']." -> ".$maxtime;
                    }else if($key == 'suspend_num' && $value != $statis[$key]){
                        $msg[] = " 可暂停次数：".$statis[$key]." -> ".$value;
                    }else if($key == 'job_num' && $value != $statis[$key]){
                        $msg[] = " 发布职位数：".$statis[$key]." -> ".$value;
                    }else if($key == 'breakjob_num' && $value != $statis[$key]){
                        $msg[] = " 刷新职位数：".$statis[$key]." -> ".$value;
                    }else if($key == 'down_resume' && $value != $statis[$key]){
                        $msg[] = " 下载简历数：".$statis[$key]." -> ".$value;
                    }else if($key == 'invite_resume' && $value != $statis[$key]){
                        $msg[] = " 邀请面试数：".$statis[$key]." -> ".$value;
                    }else if($key == 'zph_num' && $value != $statis[$key]){
                        $msg[] = " 招聘会报名：".$statis[$key]." -> ".$value;
                    }else if($key == 'top_num' && $value != $statis[$key]){
                        $msg[] = " 职位置顶数：".$statis[$key]." -> ".$value;
                    }else if($key == 'urgent_num' && $value != $statis[$key]){
                        $msg[] = " 紧急招聘数：".$statis[$key]." -> ".$value;
                    }else if($key == 'rec_num' && $value != $statis[$key]){
                        $msg[] = " 职位推荐数：".$statis[$key]." -> ".$value;
                    }
                }
            }
            if(!empty($msg)){
                $msgContent	= @implode('，', $msg);
            }
            $data['rating_type'] = $rating_info['type'];
            $data['vip_stime'] = $statis['vip_stime'] ? $statis['vip_stime'] : time();

            if ($_POST['hotjob']) {
                if (empty($company['name'])) {
                    $this->render_json(1, '请先完善企业资料！');
                }
            }
            if ($statis['rating'] != $_POST['rating'] || ($statis['rating'] == $_POST['rating'] && !isVip($statis['vip_etime']) && (int) $_POST['addday'] == 0)) {
                $data['vip_stime'] = time();
                if (!empty($rating_info['integral_buy'])) {
                    if (isset($data['integral']) && $data['integral'] !== '') {
                        $data['integral'] = intval($rating_info['integral_buy']) + intval($data['integral']);
                    } else {
                        $data['integral'] = array('+', intval($rating_info['integral_buy']));
                    }
                }
                $statisM->upInfo($data, array('uid' => $uid, 'usertype' => 2, 'adminedit' => '1', 'info' => $rating_info));
                $sName = !isVip($statis['vip_etime']) ? '过期会员' : $statis['rating_name'];
                $mContent = '会员等级：'.$sName.' -> '.$rating_info['name'];
                $msgContent	= $msgContent ? $mContent.'，'.$msgContent : $mContent;
            } else {
                unset($data['rating']);
                $statisM->upInfo($data, array('uid' => $uid, 'usertype' => 2));
            }
            if ($_POST['hotjob']) {
                $hotData = array(
                    'uid' => $uid,
                    'username' => $company['name'],
                    'rating' => $data['rating_name'],
                    'time_start' => $data['vip_stime'] ? $data['vip_stime'] : time(),
                    'time_end' => $data['vip_etime'] == 0 ? strtotime("2029-12-30") : $data['vip_etime'],
                    'rating_id' => $data['rating'],
                    'did' => $company['did']
                );
                $hotjob = $companyM->getHotJob(array('uid' => $uid));
                if(!$hotjob['hot_pic']){
                    $hotData['hot_pic'] = $company['logo'];
                }
                if (!empty($hotjob)) {
                    $hotData['sort'] = $hotjob['sort'];
                    $companyM->upHotJob($uid, $hotData);
                } else {
                    $hotData['uid'] = $uid;
                    $hotData['username'] = $company['name'];
                    $hotData['did'] = $company['did'];
                    $companyM->addHotJob($hotData);
                }
            } else {
                if ($company['rec'] == 1) {
                    $companyM->delHotJob($uid);
                }
            }
            $companyData = array(
                'rating' => $_POST['rating'],
                'rating_name' => $_POST['rating_name'],
                'vipetime' => $_POST['vip_etime']
            );
            if ($data['vip_stime']) {
                $companyData['vipstime'] = $data['vip_stime'];
            }
            if ($_POST['hotjob']) {
                $companyData['hotstart'] = $data['vip_stime'];
                $companyData['hottime'] = $data['vip_etime'] == 0 ? strtotime("2029-12-30") : $data['vip_etime'];
            } else {
                $companyData['hotstart'] = 0;
                $companyData['hottime'] = 0;
            }
            $companyM->upInfo($uid, '', $companyData);
            $content = '企业列表修改会员等级(ID:' . $uid . ')；';
            $logContent = isset($msgContent) ? $content.$msgContent : $content.'未修改会员等级、套餐';
            $logM =	$this->MODEL('log');
            $logM->addAdminLog($logContent);
            $this->admin_json(0, '企业(ID:' . $uid . ')修改成功！');
        } else {
            $this->render_json(1, '非法操作！');
        }
    }
    function setupcom_action(){
        $companyM = $this->MODEL('company');
        $data = array(
            'uid' => intval($_POST['uid']),
            'addzttime' => $_POST['addzttime'],
            'from' => 'admin'
        );
        $res = $companyM->setupcom($data);
        $this->admin_json(0, '', $res);
    }

    /**
     * @desc 分配顾问
     */
    function checkguwen_action(){
        $ComM = $this->MODEL('company');
        $adminM = $this->MODEL('admin');
        $gid = intval($_POST['gid']);
        $comid = trim($_POST['comid']);
        $uid = @explode(',', $comid);
        $uids = pylode(',', $uid);
        $crmUser = $adminM->getAdminUser(array( 'uid' => $gid ));
        if (!is_array($crmUser)) {
            $this->render_json(1, '请选择分配顾问！');
        }
        $gData = array('crm_uid' => $gid, 'crm_time' => time(), 'crm_source' => 5);
        $whereData = array('crm_uid' => $gid, 'uid' => $uids);
        $arr = $ComM->setComGw($gData, $whereData);
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    /**
     * @desc 会员企业列表，点击企业用户名成，跳转企业会员中心
     *
     * @param  $_POST['type']，跳转招聘统计分析页面
     */
    function Imitate_action(){
        $userinfoM = $this->MODEL('userinfo');
        $member = $userinfoM->getInfo(array('uid'=> intval($_POST['uid'])),array('field'=>'`uid`,`username`,`salt`,`email`,`password`,`usertype`,`did`'));
        $this->cookie->unset_cookie($_SESSION['auid']);
        $this->cookie->add_cookie($member['uid'],$member['username'],$member['salt'],$member['email'],$member['password'],2,$this->config['sy_logintime'],$member['did'],'1');
        $typeStr = trim($_POST['type']);
        $url = '';
        if(!empty($typeStr)){
            if ($typeStr == 'job') {
                $url = 'index.php?c='.$typeStr;
            }else{
                $url = 'index.php?c='.$typeStr;
            }
        }
        $logM = $this->MODEL('log');
        $content = '管理员'.$_SESSION['ausername'].'登录企业账户(ID:'.$member['uid'].')';
        $logM->addAdminLog($content);
        $this->render_json(0, '', $this->config['sy_weburl'].'/member/'.$url);
    }

    /**
     * @desc  导出数据检测
     */
    function export_check_action(){
        $where = $_SESSION['comXls'] ? $_SESSION['comXls'] : array('orderby' => 'uid');
        if(!empty($_POST['type'])){
            foreach($_POST['type'] as $v){
                if($v == 'lastdate'){
                    $type[]  =  'lastupdate';
                }else if ($v == 'money'){
                    $type[] = 'money';
                    $type[] = 'moneytype';
                }else if ($v == 'crm_salesman'){
                    $type[] = 'crm_uid';
                }else if ($v != 'rating' && $v != 'vip_stime' && $v != 'vip_etime'){
                    $type[] = $v;
                }
            }
            !in_array('uid', $type) && array_unshift($type, 'uid');
            //array_unshift($type, 'uid');
            $field = @implode(',', $type);
        }else{
            $field = 'uid';
        }
        if($_POST['uid']){
            $uids = @explode(',', $_POST['uid']);
            $where['uid'] = array('in', pylode(',', $uids));
            $_SESSION['comXlsIds'] = $where['uid'];// 记录勾选的id条件
        } else {
            unset($_SESSION['comXlsIds']);
        }
        if($_POST['limit']){
            $where['limit'] = intval($_POST['limit']);
            $_SESSION['comXlsLimit'] = intval($_POST['limit']);// 记录输入的条数限制
        } else {
            unset($_SESSION['comXlsLimit']);
        }
        $_SESSION['xlsFields'] = $field;
        $_SESSION['postTypes'] = $_POST['type'];
        $ComM = $this->MODEL('company');
        $companyNum = $ComM->getCompanyNum($where);
        if (intval($companyNum) > 0){
            !is_null($_COOKIE['companyXls']) && $this->cookie->SetCookie("companyXls",true,time() - 86400);// 手动控制上一次下载成功的cookie过期
            $this->render_json(0, '', array('field' => $_SESSION['xlsFields']));
        } else {
            $this->render_json(1, '暂无符合条件的企业数据！');
        }
    }

    // 导出字段
    private function getFields()
    {
        // rtype 开头 简历字段 type 开头 个人信息字段
        $fieldsList = [
            'uid' => '企业UID',
            'name' => '企业名称',
            'hy' => '从事行业',
            'pr' => '企业性质',
            'rating' => '会员等级',
            'provinceid' => '省',
            'cityid' => '市',
            'mun' => '规模',
            'sdate' => '创办时间',
            'money' => '注册资金',
            'address' => '公司地址',
            'linkman' => '联系人',
            'linkqq' => '所属职位',
            'type_email' => '联系QQ',
            'linkphone' => '固定电话',
            'linktel' => '联系手机',
            'linkmail' => '邮件',
            'website' => '网址',
            'rec' => '知名企业',
            'lastupdate' => '更新时间',
            'vip_stime' => '会员开始时间',
            'vip_etime' => '会员到期时间',
            'crm_salesman' => '当前业务员',
        ];
        return $fieldsList;
    }

    /**
     * @desc  导出企业数据
     */
    function xls_action(){
        $where = $_SESSION['comXls'] ? $_SESSION['comXls'] : array('orderby' => 'uid');
        if (isset($_SESSION['comXlsIds']) && $_SESSION['comXlsIds']) {
            $where['uid'] = $_SESSION['comXlsIds'];// 勾选的id条件
        }
        if (isset($_SESSION['comXlsLimit']) && $_SESSION['comXlsLimit']) {
            $where['limit'] = $_SESSION['comXlsLimit'];// 导出条数限制
        }
        $this->cookie->SetCookie("companyXls",true,time() + 60);// 前台获取到该cookie值后关闭页面加载动画
        $field = $_SESSION['xlsFields'] ? $_SESSION['xlsFields'] : 'uid';
        $ComM = $this->MODEL('company');
        $listNew = $ComM->getList($where,array('statis' => 1,'cache' => 1,'field' => $field));
        $list = $listNew['list'];
        if (!empty($list)){
            $fieldsList = $this->getFields();
			if (in_array('rating', $_SESSION['postTypes'])){
                $field .= ',rating';
            }
            if (in_array('vip_stime', $_SESSION['postTypes'])){
                $field .= ',vip_stime';
            }
            if (in_array('vip_etime', $_SESSION['postTypes'])){
                $field .= ',vip_etime';
            }
            if (in_array('crm_salesman', $_SESSION['postTypes'])){
                $field .= ',crm_salesman';
            }
            $fields = array_keys($fieldsList);
            $exportFields = explode(',', $field);
            foreach ($exportFields as $fval) {
                if (in_array($fval, $fields)) {
                    $type[] = $fval;
                }
            }
            include_once LIB_PATH . 'libs/PHPExcel.php';
            $objPHPExcel = new PHPExcel();
            $objPHPExcel->setActiveSheetIndex(0);
            $col = 'A';
            // 循环字段
            foreach ($type as $tval) {
                $width = 20;
                $objPHPExcel->getActiveSheet()->getColumnDimension($col)->setWidth($width); // 设置列宽
                $objPHPExcel->getActiveSheet()->setCellValue($col . '1', $fieldsList[$tval]); // 设置表头
                $col++;
            }
            foreach ($list as $key => $val) {
                $col = 'A';
                // 循环字段
                foreach ($type as $tval) {
                    if (in_array($tval, array('hy', 'pr', 'mun'))) {
                        $text = $val[$tval . '_n'];
                    } else if (in_array($tval, array('vip_stime', 'vip_etime', 'lastupdate'))) {
                        $text = date('Y-m-d', $val[$tval]);
                    } else if ($tval == 'provinceid') {
                        $text = $val['job_city_one'] ? $val['job_city_one'] : '';
                    } else if ($tval == 'cityid') {
                        $text = $val['job_city_two'] ? $val['job_city_two'] : '';
                    } else if ($tval == 'money') {
                        $text = $val['money'];
                        if ($val['moneytype'] == 1) {
                            $text .= '人民币';
                        } else if ($val['moneytype'] == 2) {
                            $text .= '美元';
                        }
                    } else if ($tval == 'rec') {
                        $text = $val['rec'] == 1 ? '是' : '否';
                    } else {
                        $text = $val[$tval] ? $val[$tval] : '';
                    }
                    if (in_array($tval, array('linktel', 'uid'))) { // 数字转字符的字段
                        $objPHPExcel->getActiveSheet()->setCellValueExplicit($col . ($key + 2), $text, PHPExcel_Cell_DataType::TYPE_STRING); // 转类型
                    } else {
                        $objPHPExcel->getActiveSheet()->setCellValue($col . ($key + 2), $text); // 动态表格内容
                    }
                    $col++;
                }
            }
            $objWriter = PHPExcel_IOFactory::createWriter($objPHPExcel, 'Excel2007');
            ob_start();
            $objWriter->save('php://output');
            $xlsData = ob_get_contents();
            ob_end_clean();
            $data = [
                'file' => base64_encode($xlsData),
                'file_name' => '公司信息' . date('YmdHis') . '.xlsx'
            ];
            return $this->admin_json(0, "导出公司信息", $data);
        } else {
            $this->render_json(1, '暂无符合条件的企业数据');
        }
    }

    /**
     * @desc  企业列表  分站设置
     */
    function checksitedid_action(){
        $uid = trim($_POST['uid']);
        $did = intval($_POST['did']);
        if(empty($uid)){
            $this->render_json(0, '参数不全请重试！');
        }
        $uids =	@explode(',', $_POST['uid']);
        $uid = pylode(',', $uids);
        if(empty($uid)){
            $this->render_json(0, '请正确选择需分配用户！');
        }
        $siteM = $this->MODEL('site');
        $didData = array('did' => $did);
        $Table = array(
            'member',
            'company',
            'company_statis',
            'company_job',
            'company_cert',
            'company_news',
            'company_order',
            'company_product',
            'partjob',
            'hotjob'
        );
        $siteM->updDid(array('report'), array('p_uid' => array('in', $uid), 'usertype' => 2), $didData);
        $siteM->updDid(array('userid_msg'), array('fid' => array('in', $uid)), $didData);
        $siteM->updDid(array('company_pay', 'look_resume'), array('com_id' => array('in', $uid), 'usertype' => 2), $didData);
        $siteM->updDid(array('down_resume'), array('comid' => array('in', $uid), 'usertype' => 2), $didData);
        $siteM->updDid(array('ad_order'), array('comid' => array('in', $uid)), $didData);
        $siteM->updDid($Table,array('uid' => array('in', $uid)), $didData);
        $this->admin_json(0, '会员(ID:'.$_POST['uid'].')分配站点成功！');
    }

    /**
     * @desc 后台    - 企业列表  - 更多操作
     */
    function getinfo_action(){
        if($_POST['comid']){
            $comid = intval($_POST['comid']);
            $ComM = $this->MODEL('company');
            $info = $ComM->getInfo($comid,array('ywy'=>1));
            $uid = intval($info['uid']);
            $UsernfoM = $this->MODEL('userinfo');
            $member = $UsernfoM->getInfo(array('uid'=> $uid), array('field'=>'username, reg_ip,status,lock_info,reg_date,source,wxid,wxopenid'));
            $statisM = $this->MODEL('statis');
            $statis = $statisM->getInfo($uid, array('usertype' => '2', 'field' => 'rating'));
            $yyzz = $ComM->getCertInfo(array('uid' => $uid, 'type' => '3'), array('field' => '`check`'));
            if ($info['crm_name']){
                $info['adviser'] = $info['crm_name'];
            }else{
                $info['adviser'] = null;
            }
            $info['wxid'] = $member['wxid'];
            $info['wxopenid'] = $member['wxopenid'];
            
            $info['username'] = $member['username'];
            $info['reg_ip'] = $member['reg_ip'];
            $info['status'] = $member['status'];
            $info['lock_info'] = $member['lock_info'];
            $info['rating'] = $statis['rating'];
            $info['yyzzurl'] = $yyzz['old_check'];
            $info['logo_n'] = checkpic($info['logo'],$this->config['sy_unit_icon']);
            include(CONFIG_PATH.'db.data.php');
            $info['source_n'] = $member['source'] ? $arr_data['source'][$member['source']] : '';
            $info['reg_date_n'] = $member['reg_date'] ? date('Y-m-d H:i:s', $member['reg_date']) : '';
            $info['login_date_n'] = $info['login_date'] ? date('Y-m-d H:i:s', $info['login_date']) : '';
            if ($info['linktel']){
                $info['phone'] = $info['linktel'];
            }else{
                $info['phone'] = $info['linkphone'];
            }
            if($info['zt_time']){
                $info['zt_days'] = intval((time() - $info['zt_time'])/86400);
            }
            $comOrderM = $this->MODEL('companyorder');
            $integralNum = $comOrderM->getCompanyPayNum(array('com_id' => $uid, 'type' => '1' ,'usertype' => '2'));
            $info['integralNum'] = $integralNum;
            $orderNum = $comOrderM->getCompanyOrderNum(array('uid'=>$uid , 'usertype' => 2));
            $info['orderNum'] = $orderNum;
            $downResumeM = $this->MODEL('downresume');
            $downNum = $downResumeM->getDownNum(array('comid' => $uid, 'usertype' => 2));
            $info['downNum'] = $downNum;
            $jobM = $this->MODEL('job');
            $applyNum = $jobM->getSqJobNum(array('com_id' => $uid));
            $info['applyNum'] = $applyNum;
            $inviteNum = $jobM->getYqmsNum(array('fid' => $uid));
            $info['inviteNum'] = $inviteNum;
            $showNum = $ComM->getComShowNum(array('uid' => $uid));
            $info['showNum'] = $showNum;
            $jobNum = $jobM->getJobNum(array('uid' => $uid));
            $info['jobNum'] = $jobNum;
//            $comacM	= $this->MODEL('companyaccount');
//            $info['sonsNum'] = $comacM->getNum(array('comid' => $uid));

            //提取分站内容
            $cacheM	= $this->MODEL('cache');
            $domain = $cacheM->GetCache('domain');
            if (!$info['did']) {
                $info['did'] = '0';
            }
            $info['did_name'] = $domain['Dname'][$info['did']];
            $info['vipetime_n'] = $info['vipetime'] ? date('Y-m-d', $info['vipetime']) : '';
            $info['package'] = $info['package'] ? explode(',', $info['package']) : [];
            $this->render_json(0, '', $info);
        }
    }

    /**
     * @desc 会员-企业-企业用户列表:（重置密码）
     */
    function reset_companypassword_action(){
        $userinfoM = $this->MODEL('userinfo');
        $userinfoM->upInfo(array('uid' => intval($_POST['uid'])), array('password' => '123456'));
        $this->admin_json(0, '会员(ID:' . $_POST['uid'] . ')重置密码成功');
    }

    /**
     * @desc 会员-企业-企业列表:（统计数量）
     */
    function companyNum_action(){
        $MsgNum = $this->MODEL('msgNum');
        $rt = json_decode($MsgNum->companyNum(), 1);
        $this->render_json(0, '', $rt);
    }

    /**
     * @desc 企业模板
     */
    function mcomtpl_action(){
        $tplM = $this -> MODEL('tpl');
        $comid = intval($_POST['comid']);
        $where = array();
        $where['status'] = '1';
        $where['PHPYUNBTWSTART_A'] = '';
        $where['service_uid'][] = array('=', '0','OR');
        $where['service_uid'][] = array('findin', $comid, 'OR');
        $where['PHPYUNBTWEND_A'] = '';
        $where['orderby'] = 'id,desc';
        $list = $tplM->getComtplList($where);
        foreach ($list as $k => $v) {
            $list[$k]['preview_url'] = Url('company',array('c'=>'show','id'=>$comid, 'style' => $v['url']));
        }
        $rt['list'] = $list;
        $statisM = $this->MODEL('statis');
        $statis = $statisM->getInfo($comid, array('usertype'=>'2', 'field'=>'comtpl'));
        $rt['statis'] = $statis;
        $this->render_json(0, '', $rt);
    }

    /**
     * @desc 设置企业模板
     */
    function msettpl_action(){
        $uid = intval($_POST['comid']);
        $tplM = $this->MODEL('tpl');
        $id = intval($_POST['id']);
        $tpl = $tplM->getComtpl(array('id' => $id), array('field' => 'url'));
        $statisM = $this->MODEL('statis');
        $nid = $statisM->upInfo(array('comtpl'=>$tpl['url']), array('uid' => $uid, 'usertype' => '2'));
        if ($nid){
            $sysmsgM = $this->MODEL('sysmsg');
            $sysmsgM->addInfo(array('uid' => $uid, 'usertype'=>2, 'content' => '管理员为您设置企业模板：<a href="comtpl,'.$uid.'">'.trim($tpl['url'].'</a>')));
            $this->admin_json(0, '设置成功！');
        }else{
            $this->render_json(1, '设置失败！');
        }
    }

    /**
     * @desc 职位匹配简历推送
     */
    function directrecom_action(){
        $where = array(
            'eid' => intval($_POST['eid']),
            'jobid' => intval($_POST['id']),
            'comid' => intval($_POST['comid'])
        );
        $userEntrustM = $this->MODEL('userEntrust');
        $return = $userEntrustM->sendRecord($where);
        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /**
     * 企业认证总入口，入口整合，利于权限控制
     */
    function comcert_action()
    {
        if (isset($_POST['sbody'])) {
            $this->sbody();
        } elseif (isset($_POST['batchfirm'])) {
            $this->batchfirm();
        } elseif (isset($_POST['comemail'])) {
            $this->emaillock();
        } elseif (isset($_POST['comlinktel'])) {
            $this->phonelock();
        } elseif (isset($_POST['acwxbind'])) {
            $this->acwxbind();
        } else {
            $this->comStatus();
        }
    }

    /**
     * 列表-邮箱认证
     */
    function emaillock()
    {
        $CompanyM = $this->MODEL('company');
        $UserinfoM = $this->MODEL('userinfo');
        if ($_POST['comemail'] == "") {
            $this->render_json(1, "请填写邮箱");
        } elseif (CheckRegEmail($_POST['comemail']) == false) {
            $this->render_json(1, "邮箱格式错误");
        }
        $uid = $_POST['uid'];
        $status = $_POST['estatus'];
        $email = $_POST['comemail'];
        $comInfo = $CompanyM->getInfo($uid, array('field' => '`linkmail`, `email_status`'));
        if ($comInfo) {
            if ($comInfo['linkmail'] == $email && $comInfo['email_status'] == 1){
                $this->render_json(1, "邮箱未变更，无需调整！");
            }
            $data = array('email_status' => $status, 'linkmail' => $email);
            $nid = $CompanyM->upInfo($uid, '', $data);
            if ($nid) {
                $emaildata = array('email' => $email, 'email_status' => $status);
                $UserinfoM->upInfo(array('uid' => $uid), $emaildata);
                $this->obj->update_once('member', array('email' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'email' => $email));
                $this->obj->update_once('company', array('linkmail' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'linkemail' => $email));
                $this->obj->update_once('resume', array('email' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'email' => $email));
                $this->obj->update_once('lt_info', array('email' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'email' => $email));
                $this->obj->update_once('px_train', array('linkmail' => '', 'email_status' => 0), array('uid' => array('<>', $uid), 'linkmail' => $email));
                $msg = '新邮箱：' . $email;
                if (!empty($comInfo['linkmail']) && $comInfo['linkmail'] != $email) {
                    $msg .= '，原邮箱：' . $comInfo['linkmail'];
                }
                $this->MODEL('log')->addAdminLog("企业会员(ID".$_POST['uid'].")认证邮箱【".$email."】");
                $this->admin_json(0, "邮箱认证成功(用户ID：" . $uid . "，" . $msg . ")");
            } else {
                $this->render_json(1, "邮箱认证失败");
            }
        } else {
            $this->render_json(1, "当前数据错误");
        }
    }

    /**
     * 列表-手机认证
     */
    function phonelock()
    {
        $_POST = $this->post_trim($_POST);
        $CompanyM = $this->MODEL('company');
        $UserinfoM = $this->MODEL('userinfo');
        if ($_POST['comlinktel'] == "") {
            $this->render_json(1, "请填写手机号码");
        } elseif (CheckMobile($_POST['comlinktel']) == false) {
            $this->render_json(1, "手机号码格式错误");
        }
        $uid = $_POST['uid'];
        $status = $_POST['mstatus'];
        $moblie = $_POST['comlinktel'];
        $comInfo = $CompanyM->getInfo($uid, array('field' => '`linktel`,`moblie_status`'));
        if (!empty($comInfo)) {
            if ($comInfo['linktel'] == $moblie && $comInfo['moblie_status'] == 1){
                $this->render_json(1, "手机号未变更，无需调整！");
            }
            $data = array('moblie_status' => $status, 'linktel' => $moblie);
            $nid = $CompanyM->upInfo($uid, '', $data);
            if ($nid) {
                $mobliedata = array('moblie' => $moblie, 'moblie_status' => $status);
                $msg = '新手机号：' . $moblie;
                if (!empty($comInfo['linktel']) && $comInfo['linktel'] != $moblie) {
                    $msg .= '，原手机号：' . $comInfo['linktel'];
                }
                // 获取用户信息，用来判断旧手机号和用户名是否一致
                $member =   $UserinfoM->getInfo(array('uid' => $uid), array('field'=>'username,moblie'));
                if ($member['username'] == $member['moblie']) {

                    $mobliedata['username'] = $moblie;
                    $msg .= '，同步调整用户名为'.$moblie;
                }
                // 有账号的用户名与新手机号一致的，将用户名改成新手机号
                $omb = $UserinfoM->getInfo(array('username' => $moblie), array('field'=>'uid'));
                if (!empty($omb)) {
                    // 如果现有数据中，存在用户名是这个手机号的，要修改
                    $UserinfoM->upInfo(array('uid' => $omb['uid']), array('username' => $moblie . '_s'));
                    $logDetail = '账号修改：账号（UID：'.$uid.'）认证手机号，因本账号用户名与该手机号相同，调整本账号（ID：'.$omb['uid'].'）用户名（'.$moblie.' → '.$moblie.'_s）';
                    $logM = $this->MODEL('log');
                    $logM -> addAdminLog($logDetail);
                }
                $UserinfoM->upInfo(array('uid' => $uid), $mobliedata);
                $this->obj->update_once('member', array('moblie' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'moblie' => $moblie));
                $this->obj->update_once('resume', array('telphone' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'telphone' => $moblie));
                $this->obj->update_once('company', array('linktel' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'linktel' => $moblie));
                $this->obj->update_once('lt_info', array('moblie' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'moblie' => $moblie));
                $this->obj->update_once('pr_train', array('linktel' => '', 'moblie_status' => 0), array('uid' => array('<>', $uid), 'linktel' => $moblie));
                $this->admin_json(0, "手机认证成功(用户ID：" . $uid . "，" . $msg . ")");
            } else {
                $this->render_json(1, "手机认证失败");
            }
        } else {
            $this->render_json(1, "当前数据错误");
        }
    }

    /**
     * 列表底部-批量认证
     */
    function batchfirm()
    {
        $CompanyM = $this->MODEL('company');
        $UserinfoM = $this->MODEL('userinfo');
        $status = $_POST['plstatus'];
        $msg = array();
        if ($_POST['comname_email'] == "" && $_POST['comname_moblie'] == "" && $_POST['comname_yyzz'] == "") {
            $this->render_json(1, "请选择认证类型");
        }
        if ($_POST['uid'] == "") {
            $this->render_json(1, "非法操作");
        }
        if ($status == "") {
            $this->render_json(1, "请选择认证状态");
        }
        if ($_POST['comname_email'] || $_POST['comname_moblie']) {
            $where['uid'] = array('in', pylode(',', $_POST['uid']));
            $rows = $CompanyM->getChCompanyList($where, array('field' => '`uid`,`linktel`,`linkmail`,`moblie_status`,`email_status`'));
            if (is_array($rows) && $rows) {
                if ($_POST['comname_email']) {
                    array_push($msg, '邮箱');
                    foreach ($rows as $val) {
                        if ($val['linkmail'] || $val['email_status'] == 1) {
                            $emailuid[] = $val['uid'];
                        }
                    }
                    $emaildata = array('email_status' => $status);
                    $emailwhere['uid'] = array('in', pylode(',', $emailuid));
                    $UserinfoM->upInfo($emailwhere, $emaildata);
                    $CompanyM->upInfo($emailuid, '', $emaildata);
                }
                if ($_POST['comname_moblie']) {
                    array_push($msg, '手机');
                    foreach ($rows as $val) {
                        if ($val['linktel'] || $val['moblie_status'] == 1) {
                            $moblieuid[] = $val['uid'];
                        }
                    }
                    $mobliedata = array('moblie_status' => $status);
                    $mobliewhere['uid'] = array('in', pylode(',', $moblieuid));
                    $UserinfoM->upInfo($mobliewhere, $mobliedata);
                    $CompanyM->upInfo($moblieuid, '', $mobliedata);
                }
            }
        }
        if ($_POST['comname_yyzz']) {
            //企业资质
            array_push($msg, '企业资质');
            if ($status != 0) {
                //已认证
                $yyzzwhere['uid'] = array('in', pylode(',', $_POST['uid']));
                $yyzzwhere['type'] = 3;
                $yyzz = $CompanyM->getCertList($yyzzwhere, array('field' => '`uid`,`check`,`owner_cert`,`wt_cert`,`other_cert`'));
                if (is_array($yyzz) && $yyzz) {
                    foreach ($yyzz as $val) {
                        $pass = true;
                        if (!$val['check']) {
                            $pass = false;
                        }
                        if ($this->config['com_cert_owner'] == 1 && !$val['owner_cert']) {
                            $pass = false;
                        }
                        if ($this->config['com_cert_wt'] == 1 && !$val['wt_cert']) {
                            $pass = false;
                        }
                        if ($pass) {
                            $checkuid[] = $val['uid'];
                        }
                    }
                }
            } else {
                $checkuid[] = $_POST['uid'];
            }
            $yyzzkdata = array('yyzz_status' => $status);
            $CompanyM->upInfo($checkuid, '', $yyzzkdata);
            $checkdata = array('status' => $status);
            $checwhere['uid'] = array('in', pylode(',', $checkuid));
            $checwhere['type'] = 3;
            $CompanyM->upCertInfo($checwhere, $checkdata, array('utype' => 'admin'));
        }
        $ty = $status = 1 ? '已认证' : '待认证';
        $this->admin_json(0, '(企业列表)' . implode(',', $msg) . '批量设置' . $ty . '成功(ID:' . pylode(',', $_POST['uid']) . ')');
    }

    /**
     * 列表-认证审核
     */
    function comStatus()
    {
        $companyorder = $this->MODEL('companyorder');
        $companyM = $this->MODEL('company');
        if (empty($_POST['r_status'])) {
            $this->render_json(1, '请选审核状态！');
        }else{
            $status = intval($_POST['r_status']);
        }
        if ($_POST['uid']) {
            $uid = $_POST['uid'];
            if ($status != 1) {
                $yyzz_status = 2;
            } else {
                $yyzz_status = 1;
                // 如果是“审核通过”，判断之前是否有过“审核通过的记录”，没有则增加企业资质审核通过的积分（只有第一次审核通过才加积分）
                if (!empty($uid) && is_array($uid)) {
                    $comids = @explode(',', $uid);
                    $paywhere['com_id'] = array('in', pylode(',', $comids));
                    $paywhere['pay_remark'] = '认证企业资质';
                    $companypay = $companyorder->getPayList($paywhere, array('field' => 'com_id'));
                    foreach ($companypay as $k => $v) {
                        if (in_array($v, $uid)) {
                            unset($uid[$k]);
                        }
                    }
                    foreach ($uid as $v) {
                        $this->MODEL('integral')->invtalCheck($v, 2, 'integral_comcert', '认证企业资质', 21);
                    }
                } elseif ($uid != '') {
                    $paywhere['com_id'] = $uid;
                    $paywhere['pay_remark'] = '认证企业资质';
                    $num = $companyorder->getCompanyPayNum($paywhere);
                    if ($num < 1) {
                        $this->MODEL('integral')->invtalCheck($uid, 2, 'integral_comcert', '认证企业资质', 21);
                    }
                }
            }
            $companyData = array('yyzz_status' => $yyzz_status);
            if($_POST['name']){
                $companyData['name'] = $_POST['name'];
            }
            $companyM->upInfo($uid, '', $companyData);
            $companycertData = array('status' => $status, 'statusbody' => $_POST['statusbody']);
            $id = $companyM->upCertInfo(array('type' => '3', 'uid' => array('in', pylode(',', $uid))), $companycertData, array('utype' => 'admin'));
            if($_POST['name'] && $uid && !is_array($uid)){

                $companyM->comNameSync($uid);
            }
            // 职位免审核开启，管理勾选同步审核职位，未审核职位同步审核成功
            $jobM = $this->MODEL('job');
            if ($this->config['com_free_status'] == '1' && $_POST['job_status']) {
                $jobM->upInfo(array('state' => '1', 'r_status' => 1), array('state' => '0', 'uid' => array('in', pylode(',', $uid))));
            }
            $jobM->upInfo(array('yyzz_status' => $yyzz_status), array('uid' => array('in', pylode(',', $uid))));
            $ComA = $companyM->getList(array('uid' => array('in', pylode(',', $uid))), array('field' => 'uid,name,linkmail'));
            $company = $ComA['list'];
            if ($this->config['sy_email_set'] == '1') {
                if (is_array($company)) {
                    $notice = $this->MODEL('notice');
                    foreach ($company as $v) {
                        if ($this->config['sy_email_comcert'] == '1' && $status > 0) {
                            if ($status == '1') {
                                $certinfo = '企业资质审核通过！';
                            } else {
                                $certinfo = '企业资质审核未通过！';
                            }
                            $notice->sendEmailType(array('email' => $v['linkmail'], 'certinfo' => $certinfo, 'comname' => $v['name'], 'uid' => $v['uid'], 'name' => $v['name'], 'type' => 'comcert'));
                        }
                    }
                }
            }
            /* 消息前缀 */
            foreach ($company as $v) {
                $uids[] =   $v['uid'];
                /* 处理审核信息 */
                if ($status == 2) {
                    $statusInfo = '很遗憾 , 贵公司企业资质未能通过审核';
                    if ($_POST['statusbody']) {
                        $statusInfo .= ' , 原因：' . $_POST['statusbody'];
                    }
                    $msg[$v['uid']] = $statusInfo;
                } elseif ($status == 1) {
                    $msg[$v['uid']] = '贵公司企业资质审核通过，招聘人才更轻松！';
                }
            }
            //发送系统通知
            $sysmsgM = $this->MODEL('sysmsg');
            $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 2, 'content' => $msg));
            if ($id) {
                $this->admin_json(0, '企业资质审核(UID:' . $uid . ')设置成功！');
            } else {
                $this->render_json(1, '设置失败！');
            }
        } else {
            $this->render_json(1, '非法操作！');
        }
    }

    /**
     * 列表-认证审核弹窗: 查询审核原因
     */
    function sbody()
    {
        $CompanyM = $this->MODEL('company');
        $info = $CompanyM->getCertInfo(array('type' => 3, 'uid' => intval($_POST['uid'])), array('field' => 'statusbody'));
        $this->render_json(0, '', array('sbody' => $info['statusbody']));
    }

    /**
     * @desc 企业微海报
     */
    function mwhb_action()
    {
        $WhbM = $this->MODEL('whb');
        $comHb = $WhbM->getWhbList(array('type' => 2, 'isopen' => '1'), array('only' => 1));
        $this->render_json(0, '', array('comHb' => $comHb, 'hburl' => $this->config['sy_weburl'] . '/index.php?m=ajax&c=getComHb'));
    }

    /**
     * 列表-企业扫码绑定
     */
    function acwxbind(){
        $time = time();
        $randStr = $time.rand(1000,9999);
        $cookie = $this->MODEL('cookie');
        $cookie->setCookie('acwxbind' ,$randStr, $time+3600);
        $WxM = $this->MODEL('weixin');
        $qrcode = $WxM->applyWxQrcode($randStr, 'acwxbind', $_POST['comid']);
        if(!$qrcode){
            $this->render_json(1, '二维码获取失败');
        }else{
            $this->render_json(0, 'ok', array('code_url' => $qrcode));
        }
    }
    // 检测二维码扫码绑定情况
    function getacbindstatus_action(){
        if(isset($_COOKIE['acwxbind'])){
            $WxM = $this->MODEL('weixin');
            $result = $WxM->getWxLoginStatus($_COOKIE['acwxbind'], $_POST['comid']);
            if($result['status'] == 1){
                if (!empty($result['member'])){
                    $this->admin_json(0, '扫码绑定成功');
                }else{
                    $this->render_json(1, '扫码绑定失败');
                }
            }
        }
    }

    function setLogo_action(){
        if ($_POST){
            $comM = $this->MODEL('company');
            $result = $comM->setLogoByAdmin(array('logo' => $_POST['logo'], 'logo_status' => 0), array('uid' => $_POST['uid']));
            if ($result) {
                $this->admin_json(0, 'LOGO设置成功（企业UID: '.$_POST['uid'].'）');
            } else {
                $this->render_json(1, '设置失败');
            }
        }
    }

    //获取LOGO（后台生成LOGO或预览LOGO调用）
    function adminLogoHb_action()
    {
        $whbM = $this->MODEL('whb');
        if ($_POST['out']){
            $data = array('text' => $_POST['name'], 'hb' => $_POST['hb']);
            $data['out'] = 1;
            ob_start();
            $whbM->getLogoHb($data);
            $pic = ob_get_contents();
            ob_end_clean();
            if ($pic) {
                $this->render_json(0, '', $pic);
            } else {
                $this->render_json(1, '生成LOGO出错');
            }
        } else {
            $data = array('text' => $_GET['name'], 'hb' => $_GET['hb']);
            $whbM->getLogoHb($data);
        }
    }

    // 校验企业名称
    function checkName_action(){
        $companyName = trim($_POST['companyName']);
        $userInfoM = $this->MODEL('userinfo');
        $check = $userInfoM->addMemberCheck(array('companyName' => $companyName));
        echo $check['msg'];
        die;
    }

    public function checkComName_action()
    {
        $comM = $this->MODEL('company');
        $coms = array();
        if (!empty($_POST['companyName'])) {
            $coms = $comM->getKhList(array('name' => array('like', $_POST['companyName'])));
        }
        if ($coms) {
            $rt[] = array('value' => '已存在企业 (业务员)');
            foreach ($coms as $v) {
                $crmname = $v['crm_uid'] > 0 ? $v['crm_name'] : '未分配';
                $rt[] = array('value' => $v['name'] . ' (' . $crmname . ')');
            }
        }
        $this->render_json(0, '', $rt);
    }
    // 弹窗预览企业详情
    function compreview_action(){
        $ComM = $this->MODEL('company');
        $Info = $ComM->getInfo(intval($_POST['uid']));
        $this->yunset('Info', $Info);
        $this->render_json(0, '', $Info);
    }

    function addTuiWenTask_action()
    {
        if ($_POST) {
            $wxPubM = $this->MODEL('wxpubtemp');
            $data = array(
                'cuid' => $_POST['twtask_uid'],
                'content' => $_POST['twtask_content'],
                'urgent' => $_POST['twtask_urgent'],
                'wcmoments' => $_POST['twtask_wcmoments'],
                'gzh' => $_POST['twtask_gzh'],
                'type' => 2
            );
            $return = $wxPubM->addTwTask($data, array('auid' => $_SESSION['auid']));
            if ($return['code'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    function statisDetail_action()
    {
        $where['uid'] =	$_POST['uid'];
        if (isset($_POST['type']) && !empty($_POST['type'])){
            $where['type'] = $_POST['type'];
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company_statis_detail', $where, $page, array('limit' => $pageSize));
        if ($pages['total'] > 0) {
            $where['orderby'] =	'id';
            $where['limit'] = $pages['limit'];
            $statisM = $this->MODEL('statis');
            $rows = $statisM->getStatisDetail($where);
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $typeArr = array('1' => '上架|发布 职位', '2' => '刷新职位', '3' => '下载简历', '4' => '邀请面试', '5' => '职位推荐', '6' => '紧急招聘', '7' => '职位置顶', '8' => '招聘会报名');
        $search_list = array();
        $search_list[] = array('param' => 'type', 'name' => '套餐类目', 'value' => $typeArr);
        $rt['search_list'] = $search_list;
        $this->render_json(0, '', $rt);
    }

    function delStatisDetail_action()
    {
        $delID = isset($_POST['del']) ? $_POST['del'] : $_POST['id'];
        $statisM = $this->MODEL('statis');
        $return = $statisM->delStatisDetail($delID, array('utype' => 'admin'));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function bindPackage_action()
    {
        if ($_POST) {
            $comM = $this->MODEL('company');
            $result	= $comM->upInfo($_POST['uid'], array(), array('package' => pylode(',', $_POST['package'])));
            if ($result){
                $this->MODEL('log')->addAdminLog('企业（UID：' . $_POST['uid'].'）绑定会员套餐（ID：' . $_POST['package'] . '）成功');
                $this->render_json(0, '操作成功');
            }else{
                $this->render_json(1, '操作失败');
            }
        }
    }

    /**
     * 企业用户  -  : 解绑日志
     */
    function writtenOffLog_action()
    {
        $where['opera'] = 12;
        $where['PHPYUNBTWSTART_A'] = '';
        $where['content'][] = array('like', '解除绑定');
        $where['content'][] = array('like', '解绑', 'OR');
        $where['content'][] = array('like', '解除', 'OR');
        $where['PHPYUNBTWEND_A'] = '';
        $where['usertype'] = 2; // TODO 建议各写各的，统一走这里权限会导致加载不了
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
            $where['PHPYUNBTWSTART_B'] = '';
            $where['ctime'][] = array('>=', strtotime($_POST['time_start']));
            $where['ctime'][] = array('<=', strtotime($_POST['time_end'] . ' 23:59:59'));
            $where['PHPYUNBTWEND_B'] = '';
        }
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('member_log', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id';
            }
            $where['limit'] = $pages['limit'];

        }
        $logM = $this->MODEL('log');
        $list = $logM->getMemlogList($where, array('utype' => 'admin'));
        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    /**
     * 企业用户 - ： 解绑日志删除
     */
    function delwflog_action()
    {
        if (empty($_POST['del'])) {
            $this->render_json(1, '参数错误');
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
            $this->admin_json(0, $ids == 'all' ? '解绑记录清空成功' : '解绑记录（ID:' . $ids . '）删除成功');
        } else {
            $this->render_json(1, '解绑记录删除失败');
        }
    }

    // 企业日志
    function log_action()
    {
        $logM = $this->MODEL('log');
        $memberM = $this->MODEL('userinfo');
        if ($_POST['utype']) {
            $where['usertype'] = trim($_POST['utype']);
        } else {
            $where['usertype']  =   1;
        }
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
            if (!empty($logDetailList)){
                $logIds = array();
                foreach ($logDetailList as $lk => $lv){
                    $logIds[] = $lv['log_id'];
                }
                $where['PHPYUNBTWSTART_A']  =   '';
                $where['id'] = array('in', pylode(',', $logIds), '');
                $where['content'] = array('like', $contentStr, 'OR');
                $where['PHPYUNBTWEND_A'] = '';
            }else{
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
        if (!empty($_POST['operas'])){
            $operaStr = intval($_POST['operas']);
            $operaSql = array(
                '1' => array('name' => array('职位')),
                '2' => array('name' => array('创建','简历', '经历')),
                '3' => array('name' => array('下载')),
                '4'	=> array('name' => array('邀请')),
                '5'	=> array('name' => array('收藏', '关注', '备注')),
                '6'	=> array('name' => array('申请', '报名', '应聘', '委托')),
                '7'	=> array('name' => array('基本信息')),
                '8'	=> array('name' => array('修改密码')),
                '9'	=> array('name' => array('兼职')),
                '11' =>	array('name' => array('用户名', '身份')),
                '12' =>	array('name' => array('账号认证', '解除', '绑定', '验证', '资质', '执照', '认证')),
                '14' =>	array('name' => array('招聘会', '专题')),
                '15' =>	array('name' => array('地图', '助力')),
                '16' =>	array('name' => array('图片', '头像', 'LOGO', '环境', '产品', '新闻', '二维码', '横幅')),
                '17' =>	array('name' => array('兑换', '积分'), 'realId' => 17),
                '18' =>	array('name' => array('回复', '咨询', '留言', '系统消息')),
                '19' =>	array('name' => array('问答')),
                '22' =>	array('name' => array('新闻')),
                '23' =>	array('name' => array('举报')),
                '25' =>	array('name' => array('悬赏', '推送')),
                '26' =>	array('name' => array('浏览', '黑名单')),
                '29' =>	array('name' => array('项目')),
                '88' =>	array('name' => array('订单'))
            );
            if (array_key_exists($operaStr, $operaSql)) {
                if (count($operaSql[$operaStr]['name']) == 1){
                    $where['content'] = array('like', $operaSql[$operaStr]['name'][0]);
                }else{
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
        if (isset($_POST['parrs']) && $_POST['parrs']) {
            $where['type'] = intval($_POST['parrs']);
        }
        if (!empty($_POST['time'])) {
            $time = $_POST['time'];
            $time_begin = $time[0] ? date('Y-m-d', $time[0]/1000) : date('Y-m-d', strtotime('-30 days'));
            $time_end = $time[1] ? date('Y-m-d', $time[1]/1000) : date('Y-m-d');
            $where['ctime'][] = array('>=', strtotime($time_begin . "00:00:00"));
            $where['ctime'][] = array('<=', strtotime($time_end . "23:59:59"));
        }
        $pageM = $this->MODEL('page');
        $page = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pages = $pageM->adminPageList('member_log',$where,$page,array('limit' => $pageSize));
        if(!$pages['total']){
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'].','.$_POST['order'];
        } else {
            $where['orderby'] = array('id,desc');
        }
        $where['limit'] = $pages['limit'];
        $List = $logM->getMemlogList($where, array('utype' => 'admin'));
        foreach ($List as &$value){
            $value['ctime_ymd'] = $value['ctime']?date('Y-m-d H:i:s',$value['ctime']):'';
        }
        $this->render_json(0,'',['data'=>$List,'total'=>(int)$pages['total'],'pageSizes'=>$pages['page_sizes']]);
    }

    function delLog_action()
    {
        $logM = $this->MODEL('log');
        if ($_POST['del'] == 'allcom') {
            $where['usertype']  =   2;
            $logM->delMemlog($where);
            $this->layer_msg('已清空企业日志！', 9, 0, $_SERVER['HTTP_REFERER']);
        } elseif ($_POST['del']) {
            $del = $_POST['del'];
            if (is_array($del)) {
                $where['id'] = array('in', pylode(',', $del));
            } else {
                $where['id'] = $del;
            }
            $return = $logM->delMemlog($where);
            if ($return['errcode'] == 9){
                $this->render_json(0,$return['msg']);
            }else{
                $this->render_json(1,$return['msg']);
            }
        }
    }
    function savefact_action(){

        $companyM =   $this->MODEL('company');
        
        $_POST = $this->post_trim($_POST);
        
        if (!$_POST['uid']){
            $this->render_json('1','参数错误，请重试！');
        }
        
        $newpic = array();

        if ($_FILES) {

            $fileArray = array();

            foreach ($_FILES['newpic'] as $nk => $nv) {
                foreach ($nv as $nkk => $nvv) {
                    $fileArray[$nkk][$nk] = $nvv;
                }
            }

            $UploadM = $this->MODEL('upload');
            
            foreach ($fileArray as $img) {
                
                $upArr = array(
                    'file' => $img,
                    'dir' => 'comfact',
                );
                $result = $UploadM->newUpload($upArr);
                if (!empty($result['msg'])) {
                    $this->render_json(8, $result['msg']);
                } else if (!empty($result['picurl'])) {
                    $newpic[] = $result['picurl'];
                }
                
            }
        }
        
        if(!empty($newpic)){
            foreach ($newpic as $v) {
                $factV = array(
                    'uid'=>$_POST['uid'],
                    'picurl'=>$v,
                    'ctime'=>time()
                );
                $companyM->addFactPic($factV);
            }
        }
        
        if(!empty($_POST['fact_delid'])){
            $companyM->delFactPic(array('id'=>array('in',pylode(',',$_POST['fact_delid']))));
        }

        $companyM->upInfo($_POST['uid'],array(),array('fact_status'=>$_POST['fact_status']));

        $this->admin_json('0',"实地核验保存成功！");
        
    }
}