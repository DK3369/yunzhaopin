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
class company_job_controller extends adminCommon
{
    function set_search($CacheList = array())
    {
        include(CONFIG_PATH . 'db.data.php');
        $source = $arr_data['source'];

        if (!$CacheList) {
            $cacheM = $this->MODEL('cache');
            $CacheList = $cacheM->GetCache(array('com', 'job', 'city'));
            $setArr = array('comdata' => $CacheList['comdata'], 'comclass_name' => $CacheList['comclass_name'], 'job_name' => $CacheList['job_name'], 'city_name' => $CacheList['city_name']);

        }
        $comdata = $CacheList['comdata'];
        $comclass_name = $CacheList['comclass_name'];
        foreach ($comdata['job_edu'] as $k => $v) {
            $edu[$v] = $comclass_name[$v];
        }
        foreach ($comdata['job_exp'] as $k => $v) {
            $exp[$v] = $comclass_name[$v];
        }
        $ratingM = $this->MODEL('rating');
        $rating = $ratingM->getList(array('category' => '1', 'orderby' => 'sort'), array('field' => '`id`,`name`'));
        if (!empty($rating)) {
            $ratingarr = array();
            foreach ($rating as $k => $v) {
                $ratingarr[$v['id']] = $v['name'];
            }
        }

        $search_list = array();
        $search_list['state'] = array('name' => '审核状态', 'value' => array('1' => '已审核', '4' => '未审核', '3' => '未通过', '2' => '已锁定'));
        $search_list['status'] = array('name' => '招聘状态', 'value' => array('1' => '已下架', '2' => '招聘中'));
        $search_list['jtype'] = array('name' => '职位类型', 'value' => array('urgent' => '紧急职位', 'xuanshang' => '置顶职位', 'rec' => '推荐职位'));
        $search_list['exp'] = array('name' => '工作经验', 'value' => $exp);
        $search_list['edu'] = array('name' => '学历要求', 'value' => $edu);
        $search_list['source'] = array('name' => '数据来源', 'value' => $source);
//        $search_list['adtime'] = array('name' => '发布时间', 'value' => array('1' => '今天', '3' => '最近三天', '7' => '最近七天', '15' => '最近半月', '30' => '最近一个月', '31' => '当月', '-1' => '昨天', '-7' => '本周'));
        $search_list['rating'] = array('name' => '会员等级', 'value' => $ratingarr);
        $search_list['openautho'] = array('name' => '开放权限', 'value' => array('1'=>'默认','2'=>'开放'));
        
        $search_list['is_depower'] = array('name' => '是否降权', 'value' => array('1'=>'是','2'=>'否'));
        return array('search_list' => $search_list);
    }

    function index_action()
    {
        if (isset($_POST['fromCrmIndex']) && $_POST['fromCrmIndex'] == 1) {
            $comM = $this->MODEL('company');
            $coms = $comM->getChCompanyList(array('crm_uid' => array('=', intval($_SESSION['auid']))), array('field' => '`uid`'));
            $comuids = array();
            foreach ($coms as $ck => $cv) {
                $comuids[] = $cv['uid'];
            }
            $where['uid'] = array('in', pylode(',', $comuids));
        }
        if (isset($_POST['state']) && !empty($_POST['state'])) {
            $state = intval($_POST['state']);
            if ($state == 2) {
                $where['r_status'] = 2;
            } else {
                $where['state'] = $state == 4 ? 0 : $state;
            }
        }
        if (isset($_POST['openautho'])){
            $where['linkopen'] = intval($_POST['openautho']);
        }
        if (isset($_POST['status']) && !empty($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['status'] = $status == 2 ? 0 : $status;
        }
        if (isset($_POST['jtype']) && !empty($_POST['jtype'])) {
            $jtype = trim($_POST['jtype']);
            if ($jtype == 'rec') {
                $where['rec_time'] = array('>', time());
            } else if ($jtype == 'urgent') {
                $where['urgent_time'] = array('>', time());
            } else if ($jtype == 'xuanshang') {
                $where['xsdate'] = array('>', time());
            }
        }
        if (isset($_POST['edu']) && !empty($_POST['edu'])) {
            $where['edu'] = $_POST['edu'];
        }
        if (isset($_POST['exp']) && !empty($_POST['exp'])) {
            $where['exp'] = $_POST['exp'];
        }
        if (isset($_POST['source']) && !empty($_POST['source'])) {
            $where['source'] = intval($_POST['source']);
        }
//        if (isset($_POST['adtime']) && !empty($_POST['adtime'])) {
//            if ($_POST['adtime'] == '1') {
//                $where['sdate'] = array('>', strtotime('today'));
//            } else if ($_POST['adtime'] == '31') {
//                $month = get_this_month(time());
//                $where['PHPYUNBTWSTART_A'] = '';
//                $where['sdate'][] = array('>=', $month['start_month'], 'AND');
//                $where['sdate'][] = array('<=', $month['end_month'], 'AND');
//                $where['PHPYUNBTWEND_A'] = '';
//            } else if ($_POST['adtime'] == '-1') {
//                $sTime = mktime(0, 0, 0, date('m'), date('d'), date('Y')) - 86400;
//                $eTime = mktime(23, 59, 59, date('m'), date('d'), date('Y')) - 86400;
//                $where['PHPYUNBTWSTART_A'] = '';
//                $where['sdate'][] = array('>=', $sTime, '');
//                $where['sdate'][] = array('<=', $eTime, '');
//                $where['PHPYUNBTWEND_A'] = '';
//            } else if ($_POST['adtime'] == '-7') {
//                $sTime = strtotime(date('Y-m-d', strtotime("this week Monday", time())));
//                $eTime = strtotime(date('Y-m-d', strtotime("this week Sunday", time()))) + 24 * 3600 - 1;
//                $where['PHPYUNBTWSTART_A'] = '';
//                $where['sdate'][] = array('>=', $sTime, '');
//                $where['sdate'][] = array('<=', $eTime, '');
//                $where['PHPYUNBTWEND_A'] = '';
//            } else {
//                $where['sdate'] = array('>', strtotime('-' . intval($_POST['adtime']) . ' day'));
//            }
//        }
        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            if ($_POST['time_type'] == 'sdate') {
                $where['PHPYUNBTWSTART_C'] = '';
                $where['sdate'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
                $where['sdate'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
                $where['PHPYUNBTWEND_C'] = '';
            }

            if ($_POST['time_type'] == 'lastup') {
                $where['PHPYUNBTWSTART_C'] = '';
                $where['lastupdate'][] = array('>=', strtotime($_POST['times'][0] . ' 00:00:00'));
                $where['lastupdate'][] = array('<=', strtotime($_POST['times'][1] . ' 23:59:59'));
                $where['PHPYUNBTWEND_C'] = '';
            }
        }
        if (isset($_POST['rating']) && !empty($_POST['rating'])) {
            $where['rating'] = $_POST['rating'];
        }
        if (isset($_POST['job_class']) && !empty($_POST['job_class'])) {
            $where['PHPYUNBTWSTARTA'] = '';
            $where['job1'] = array('findin', $_POST['job_class']);
            $where['job1_son'] = array('findin', $_POST['job_class'], 'OR');
            $where['job_post'] = array('findin', $_POST['job_class'], 'OR');
            $where['PHPYUNBTWENDA'] = '';
        }
        if (isset($_POST['city_class']) && !empty($_POST['city_class'])) {
            $where['PHPYUNBTWSTARTB'] = '';
            $where['provinceid'] = array('findin', $_POST['city_class']);
            $where['cityid'] = array('findin', $_POST['city_class'], 'OR');
            $where['three_cityid'] = array('findin', $_POST['city_class'], 'OR');
            $where['PHPYUNBTWENDB'] = '';
        }
        if (isset($_POST['keyword']) && !empty($_POST['keyword'])) {
            $keywordStr = trim($_POST['keyword']);
            $typeStr = intval($_POST['type']);
            if ($typeStr == 1) {

                $where['PHPYUNBTWSTART_D'] = '';
                $where['com_name'] = array('like', $keywordStr);
                $where['name'] = array('like', $keywordStr, 'OR');
                $where['PHPYUNBTWEND_D'] = '';
            } elseif ($typeStr == 3) {
                $where['id'] = $keywordStr;
            } elseif ($typeStr == 4) {
                $where['add_ip'] = array('like', $keywordStr);
            }
        }
        if (isset($_POST['is_reserve']) && !empty($_POST['is_reserve'])) {
            $where['is_reserve'] = intval($_POST['is_reserve']);
        }
        if (isset($_POST['uid']) && !empty($_POST['uid'])) {
            $where['uid'] = intval($_POST['uid']);
        }
        
        if (!empty($_POST['is_depower'])) {
            $where['is_depower'] = intval($_POST['is_depower']);
        }
        if($_POST['add_crm']){
            if (intval($_POST['add_crm']) == -1) {
                $where['add_crm'] = array('<>', '0');
            } elseif (intval($_POST['add_crm']) == -2) {
                $where['add_crm'] = '0';
            } else {
                $where['add_crm'] = intval($_POST['add_crm']);
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company_job', $where, $page, array('limit' => $pageSize));
        if ($pages['total'] > 0) {
            if (isset($_POST['order']) && !empty($_POST['order'])) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'lastupdate,desc';
            }
            $where['limit'] = $pages['limit'];
            $JobM = $this->MODEL('job');
            $simple =   isset($_POST['simple']) ? $_POST['simple'] : 2;
            $ListJob = $JobM->getList($where, array('utype' => 'admin', 'cache' => '1', 'isurl' => 'yes', 'reserve' => 1, 'crm_user' => 1, 'simple' => $simple));
            unset($where['limit']);
            $_SESSION['jobXls'] = $where;
        }

        $rt['list'] = $ListJob['list'] ? $ListJob['list'] : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }

    /**
     *users:王旭
     *Data:2023/10/17
     *Time:11:36
     * 职位统计
     */
    function getJobStatist_action(){
        $MsgNum = $this->MODEL('msgNum');
        $numarr = json_decode($MsgNum->jobNum(), 1);
        $this->render_json(0, '', $numarr);
    }

    /**
     *users:王旭
     *Data:2023/10/17
     *Time:11:37
     *  海报信息
     */
    public function getHbData_action(){
        $WhbM = $this->MODEL('whb');
        $syJobHb = $WhbM->getWhbList(array('type' => 1, 'isopen' => '1'));
        $data['hbNum'] = count($syJobHb);
        $data['hb_isopen'] = $this->config['sy_haibao_isopen'];
        $this->render_json(0,'',$data);
    }

    /**
     *users:王旭
     *Data:2023/10/17
     *Time:11:37
     *  缓存信息
     */
    public function  getCacheData_action(){
        $CacheList = $this->MODEL('cache')->GetCache(array('job','hy','city','com'));
        $data['cache'] = $CacheList;
        $data['comdata'] = $CacheList['comdata'];
        $data['comclass_name'] = $CacheList['comclass_name'];
        $data['job_name'] = $CacheList['job_name'];
        $data['city_name'] = $CacheList['city_name'];
        $data['jionly'] = empty($CacheList['job_type']) ? 1 : 0;
        $data['job_types'] = $this->getJobTypeArr($CacheList);
        $data['city_types'] = $this->getCityTypeArr($CacheList);
        $data['curr_time'] = time();
        $sres = $this->set_search();
        $data['search_list'] = $sres['search_list'];
        $this->render_json(0,'',$data);
    }

    /**
     * 招聘/下架操作
     */
    function checkstate_action()
    {
        if ($_POST['id'] && $_POST['state']) {
            $_POST['state'] = $_POST['state'] == 2 ? 0 : $_POST['state'];
            $JobM = $this->MODEL('job');
            $id = intval($_POST['id']);
            $postData['status'] = intval($_POST['state']);
            $JobM->upInfo($postData, array('id' => $id));
            if ($_POST['state'] == 0) {
                $logMsg = "职位(ID" . $_POST['id'] . ")上架成功";
            } else {
                $logMsg = "职位(ID" . $_POST['id'] . ")下架成功";
            }
            $this->MODEL('log')->addAdminLog($logMsg);
        }
        $this->render_json(0, $logMsg);
    }

    // 	职位置顶
    function xuanshang_action()
    {
        $id = trim($_POST['pid']);
        $data = array(
            'top' => intval($_POST['s']),
            'days' => intval($_POST['days'])
        );
        $JobM = $this->MODEL('job');
        $arr = $JobM->addTopJob($id, $data);
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    //  职位推荐
    function recommend_action()
    {
        $id = trim($_POST['pid']);
        $data = array(
            'rec' => intval($_POST['s']),
            'days' => intval($_POST['days'])
        );
        $JobM = $this->MODEL('job');
        $arr = $JobM->addRecJob($id, $data);
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    //  职位紧急招聘
    function urgent_action()
    {
        $id = trim($_POST['pid']);
        $data = array(
            'urgent' => intval($_POST['s']),
            'days' => intval($_POST['days'])
        );
        $JobM = $this->MODEL('job');
        $arr = $JobM->addUrgentJob($id, $data);
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    //  职位审核
    function status_action()
    {
        $jobM = $this->MODEL('job');
        $statusData = array(
            'state' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody']),
            'single' => isset($_POST['single']) ? $_POST['single'] : '',
            'lock_status' => trim($_POST['lock_status']) // 新增需求 单个审核锁定企业也可以审核 新增lock_status 状态值，批量审核未传该值
        );
        $return = $jobM->statusJob($_POST['pid'], $statusData);
        if (isset($_POST['single'])) {
            if ($return['errcode'] == 9) {
                if ($_POST['atype'] == 1) {
                    // 仅保存
                    $this->admin_json(0, $return['msg']);
                } else {
                    // 下一个待审核职位
                    $jobM = $this->MODEL('job');
                    $row = $jobM->getInfo(array('state' => 0, 'orderby' => array('lastupdate,DESC')), array('field' => 'id'));
                    if (!empty($row)) {
                        $this->admin_json(0, $return['msg'], array('job' => $row));
                    } else {
                        $this->admin_json(0, $return['msg']);
                    }
                }
            } else {
                $this->render_json(1, $return['msg']);
            }
        } else {
            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    // 职位审核同步企业审核
    function cjobstatus_action()
    {
        if ($_POST) {
            $id = intval($_POST['pid']);
            $uid = intval($_POST['uid']);
            $status = intval($_POST['status']);
            $statusbody = trim($_POST['statusbody']);
            $jobM = $this->MODEL('job');
            $post = array(
                'uid' => $uid,
                'state' => $status,
                'statusbody' => $statusbody
            );
            $return = $jobM->status($id, $post);
            if (isset($_POST['single'])) {
                if ($return['errcode'] == 9) {
                    if ($_POST['atype'] == 1) {
                        // 仅保存
                        $this->admin_json(0, $return['msg']);
                    } else {
                        // 下一个待审核职位
                        $jobM = $this->MODEL('job');
                        $row = $jobM->getInfo(array('state' => 0, 'orderby' => array('lastupdate,DESC')), array('field' => 'id'));
                        if (!empty($row)) {
                            $this->admin_json(0, $return['msg'], array('job' => $row));
                        } else {
                            $this->admin_json(0, $return['msg']);
                        }
                    }
                } else {
                    $this->render_json(1, $return['msg']);
                }
            } else {
                if ($return['errcode'] == 9) {
                    $this->admin_json(0, $return['msg']);
                } else {
                    $this->render_json(1, $return['msg']);
                }
            }
        }
    }

    //  修改浏览量/曝光量
    function upjobhits_action()
    {
        $jobM = $this->MODEL('job');
        $return = $jobM->upJobHits($_POST['pid'], $_POST['jobhits'], $_POST['jobexpoure']);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    private function getCityTypeArr($CacheList){
        $citys = array();
        foreach ($CacheList['city_index'] as $fir_v) {
            $citys[$fir_v] = array('value' => $fir_v, 'label' => $CacheList['city_name'][$fir_v]);
            if ($CacheList['city_type'][$fir_v]) {// 二级城市
                $citys[$fir_v]['children'] = array();
                foreach ($CacheList['city_type'][$fir_v] as $sec_v) {
                    $citys[$fir_v]['children'][$sec_v] = array('value' => $sec_v, 'label' => $CacheList['city_name'][$sec_v]);
                    if ($CacheList['city_type'][$sec_v]) {// 三级城市
                        $citys[$fir_v]['children'][$sec_v]['children'] = array();
                        foreach ($CacheList['city_type'][$sec_v] as $thi_v) {
                            $citys[$fir_v]['children'][$sec_v]['children'][$thi_v] = array('value' => $thi_v, 'label' => $CacheList['city_name'][$thi_v]);
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
        return $citys;
    }

    private function getJobTypeArr($CacheList){
        $jobtypes = array();
        foreach ($CacheList['job_index'] as $fir_v) {
            $jobtypes[$fir_v] = array('value' => $fir_v, 'label' => $CacheList['job_name'][$fir_v]);
            if ($CacheList['job_type'][$fir_v]) {// 二级
                $jobtypes[$fir_v]['children'] = array();
                foreach ($CacheList['job_type'][$fir_v] as $sec_v) {
                    $jobtypes[$fir_v]['children'][$sec_v] = array('value' => $sec_v, 'label' => $CacheList['job_name'][$sec_v]);
                    if ($CacheList['job_type'][$sec_v]) {// 三级
                        $jobtypes[$fir_v]['children'][$sec_v]['children'] = array();
                        foreach ($CacheList['job_type'][$sec_v] as $thi_v) {
                            $jobtypes[$fir_v]['children'][$sec_v]['children'][$thi_v] = array('value' => $thi_v, 'label' => $CacheList['job_name'][$thi_v]);
                        }
                    }
                }
            }
        }
        $jobtypes = array_values($jobtypes);
        foreach ($jobtypes as $k => $v) {
            if ($v['children']) {
                $jobtypes[$k]['children'] = array_values($v['children']);
                foreach ($jobtypes[$k]['children'] as $kk => $vv) {
                    if ($vv['children']) {
                        $jobtypes[$k]['children'][$kk]['children'] = array_values($vv['children']);
                    }
                }
            }
        }
        return $jobtypes;
    }

    /**
     * @desc 后台 -- 会员 -- 企业 -- 企业管理 / 职位管理 -- 新增  /  修改
     */
    function add_action()
    {
        $cacheM = $this->MODEL('cache');
        $options = array('job', 'com', 'city', 'hy', 'user');
        $cache = $cacheM->GetCache($options);
        foreach ($cache['com_sex'] as $k => $v) {
            if ($v != '男') {
                $cache['com_sexreq'][$k] = $v;
            }
        }
        $rt['cache_com_sexreq'] = $cache['com_sexreq'];
        $rt['cache_userdata'] = $cache['userdata'];
        $rt['cache_userclassname'] = $cache['userclass_name'];
        $rt['cache'] = $cache;

        $JobM = $this->MODEL('job');
        $companyM = $this->MODEL('company');
        $addressM = $this->MODEL('address');
        if (isset($_POST['id']) && intval($_POST['id']) > 0) {
            $id = intval($_POST['id']);
            $info = $JobM->getInfo(array('id' => $id), array('add'=>'yes'));
            $all_welfare = $info['arraywelfare'] ? $info['arraywelfare'] : array();
            foreach ($cache['comdata']['job_welfare'] as $v) {
                !in_array($cache['comclass_name'][$v], $all_welfare) && array_push($all_welfare, $cache['comclass_name'][$v]);
            }
            $info['all_welfare'] = $all_welfare;
            $info['is_graduate'] = $info['is_graduate']?true:false;
            $rt['show'] = $info;
            $uid = $info['uid'];
        }
        if (intval($_POST['uid'])) {
            $uid = intval($_POST['uid']);
            $all_welfare = array();
            foreach ($cache['comdata']['job_welfare'] as $v) {
                !in_array($cache['comclass_name'][$v], $all_welfare) && array_push($all_welfare, $cache['comclass_name'][$v]);
            }
            $rt['default_welfare'] = $all_welfare;
            $rt['job_types'] = $this->getJobTypeArr($cache);
            $rt['city_types'] = $this->getCityTypeArr($cache);
        }
        $company = $companyM->getInfo($uid, array('field' => '`uid`,`r_status`,`linkman`,`linktel`,`linkphone`,`address`,`provinceid`,`cityid`,`three_cityid`,`x`,`y`,`welfare`'));
        $cityStr = $company['job_city_one'] . $company['job_city_two'] . $company['job_city_three'];
        if ($company['linktel']) {
            $company['linkmsg'] = $company['linkman'] . ' - ' . $company['linktel'] . ' - ' . $cityStr . ' - ' . $company['address'];
        } else if ($company['linkphone']) {
            $company['linkmsg'] = $company['linkman'] . ' - ' . $company['linkphone'] . ' - ' . $cityStr . ' - ' . $company['address'];
        } else {
            $company['linkmsg'] = $company['linkman'] . ' - ' . $cityStr . ' - ' . $company['address'];
        }
        $addressList = $addressM->getAddressList(array('uid' => $uid));
        $rt['company'] = $company;
        $rt['addressList'] = $addressList;
        $rt['uid'] = $uid;
        if ($_POST['save']) {
            $description = str_replace(array("&amp;", "background-color:#ffffff", "background-color:#fff", "white-space:nowrap;"), array("&", 'background-color:', 'background-color:', 'white-space:'), $_POST['content']);
            if ($_POST['link_id'] == -1) {
                $provinceid = $company['provinceid'];
                $cityid = $company['cityid'];
                $three_cityid = $company['three_cityid'];
                $x = $company['x'];
                $y = $company['y'];
            } else if (intval($_POST['link_id']) > 0) {
                $addressInfo = $addressM->getAddressInfo(array('id' => $_POST['link_id'], 'uid' => $uid));
                $provinceid = $addressInfo['provinceid'];
                $cityid = $addressInfo['cityid'];
                $three_cityid = $addressInfo['three_cityid'];
                $x = $addressInfo['x'];
                $y = $addressInfo['y'];
            }
            $post = array(
                'name' => $_POST['name'],
                'job1' => intval($_POST['job1']),
                'job1_son' => intval($_POST['job1_son']),
                'job_post' => intval($_POST['job_post']),
                'provinceid' => intval($provinceid),
                'cityid' => intval($cityid),
                'three_cityid' => intval($three_cityid),
                'x' => $x,
                'y' => $y,
                'link_id' => (int)$_POST['link_id'] > 0 ? $_POST['link_id'] : '',
                'is_link' => $_POST['is_link'] ? $_POST['is_link'] : 1,
                'is_message' => $_POST['is_message'] ? $_POST['is_message'] : 1,
                'is_email' => $_POST['is_email'] ? $_POST['is_email'] : 1,
                'minsalary' => intval($_POST['salary_type']) == 1 ? 0 : intval($_POST['minsalary']),
                'maxsalary' => intval($_POST['salary_type']) == 1 ? 0 : intval($_POST['maxsalary']),
                'description' => $description,
                'r_status' => $company['r_status'],
                'hy' => intval($_POST['hy']),
                'number' => intval($_POST['number']),
                'exp' => intval($_POST['exp']),
                'report' => intval($_POST['report']),
                'age' => intval($_POST['age']),
                'sex' => intval($_POST['sex']),
                'edu' => intval($_POST['edu']),
                'is_graduate' => $_POST['is_graduate']=='true'?1:0,
                'marriage' => intval($_POST['marriage']),
                'lang' => trim(pylode(',', $_POST['checked_lang'])),
                'welfare' => trim(implode(',', $_POST['checked_welfare'])),
                'state' => $company['r_status'] == 1 ? 1 : 0,
                'jobhits' => intval($_POST['jobhits']),
                'jobexpoure' => intval($_POST['jobexpoure']),
                'exp_req' => trim($_POST['exp_req']),
                'edu_req' => trim($_POST['edu_req']),
                'zp_num' => intval($_POST['zp_num']),
                'zp_minage' => intval($_POST['zp_minage']),
                'zp_maxage' => intval($_POST['zp_maxage']),
                'minage_req' => intval($_POST['minage_req']),
                'maxage_req' => intval($_POST['maxage_req']),
                'sex_req' => intval($_POST['sex_req']),
                'status' => intval($_POST['status']),
            );
            $data = array(
                'post' => $post,
                'id' => intval($_POST['id']),
                'uid' => $_POST['uid'],
                'utype' => 'admin'
            );
            $this->adminShell['is_crm'] == 1 && $data['add_crm'] = $this->adminShell['uid'];
            $return = $JobM->addJobInfo($data);
            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
        $rt['mapurl'] = $this->config['mapurl'];
        $rt['mapsecret'] = $this->config['map_secret'];
        $this->render_json(0, '', $rt);
    }

    // 转移类别
    function saveclass_action()
    {
        $JobM = $this->MODEL('job');
        if ($_POST['hy'] == '') {
            $this->render_json(1, '请选择行业类别！');
        }
        if ($_POST['job1'] == '') {
            $this->render_json(1, '请选择职位类别！');
        }
        $data['hy'] = $_POST['hy'];
        $data['job1'] = $_POST['job1'];
        $data['job1_son'] = $_POST['job1_son'];
        $data['job_post'] = $_POST['job_post'];
        $id = @explode(',', $_POST['jobid']);
        $listA = $JobM->getList(array('id' => array('in', pylode(',', $id))), array('cache' => '1', 'field' => 'id,uid,name'));
        $nid = $JobM->upInfo($data, array('id' => array('in', pylode(',', $id))));
        $job = $listA['list'];
        $cache = $listA['cache'];
        if ($job) {
            $msg = array();
            $uids = array();
            //  提取职位uid 和职位名称
            foreach ($job as $k => $v) {
                $uids[] = $v['uid'];
                $msg[$v['uid']][] = '您的职位<a href="comjobtpl,' . $v['id'] . '">《' . $v['name'] . '》</a>管理员已修改，行业类别为：' . $cache[industry_name][$_POST['hy']] . '，职位类别为：' . $cache[job_name][$_POST['job1']];
                if ($_POST['job1_son']) {
                    $msg[$v['uid']][] .= '' . $cache[job_name][$_POST['job1_son']];
                }
                if ($_POST['job_post']) {
                    $msg[$v['uid']][] .= '' . $cache[job_name][$_POST['job_post']];
                }
            }
            $sysmsgM = $this->MODEL('sysmsg');
            $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 2, 'content' => $msg));
        }
        if ($nid) {
            $this->admin_json(0, '职位类别(ID:' . $_POST['jobid'] . ')修改成功！');
        } else {
            $this->render_json(1, '修改失败！');
        }
    }

    // 删除职位
    function del_action()
    {
        $JobM = $this->Model('job');
        
        $delID = $_POST['del'];
        $numwhere['jobid'] = array('in', pylode(',', $delID));
        
        $addArr = $JobM->delJob($delID, array('utype' => 'admin'));
        if ($addArr['errcode'] == 9) {
            $this->admin_json(0, $addArr['msg']);
        } else {
            $this->render_json(1, $addArr['msg']);
        }
       
    }
    function refresh_action()
    {
        $JobM = $this->MODEL('job');
        $ids = @explode(',', $_POST['ids']);
        $result = $JobM->upInfo(array('lastupdate' => time()), array('id' => array('in', pylode(',', $ids))));
        if ($result) {
            $logM = $this->MODEL('log');
            $jobS = $logM->getJobBySxLog(array('id' => array('in', pylode(',', $ids))), array('type' => 1, 'field' => 'id,uid'));
            $vData = array();
            foreach ($jobS as $k => $v) {
                $vData[$k]['uid'] = $v['uid'];
                $vData[$k]['usertype'] = 2;
                $vData[$k]['jobid'] = $v['id'];
                $vData[$k]['type'] = 1;
                $vData[$k]['r_time'] = time();
                $vData[$k]['port'] = 5;
                $vData[$k]['ip'] = fun_ip_get();
                $vData[$k]['remark'] = '管理员操作：后台职位列表刷新职位';
            }
            $logM->addJobSxLogS($vData);
        }
        $this->admin_json(0, "职位(ID" . $_POST['ids'] . "刷新成功");
    }

    // 导出字段
    private function getFields()
    {
        // rtype 开头 简历字段 type 开头 个人信息字段
        $fieldsList = [
            'id' => '职位ID',
            'uid' => '企业UID',
            'name' => '职位名称',
            'hy' => '行业',
            'job1' => '一级类别',
            'job1_son' => '二级类别',
            'job_post' => '三级类别',
            'provinceid' => '省',
            'cityid' => '市',
            'three_cityid' => '县',
            'minsalary,maxsalary' => '薪水',
            'zp_num' => '招聘人数',
            'exp' => '工作经验',
            'report' => '到岗时间',
            'sex' => '性别要求',
            'edu' => '教育程度',
            'marriage' => '婚姻状况',
            'sdate' => '开始日期',
            'lastdate' => '更新时间',
            'zp_minage,zp_maxage' => '年龄要求',
            'lang' => '语言要求',
            'welfare' => '福利待遇',
            'com_name' => '公司名称',
            'pr' => '公司性质',
            'mun' => '企业规模'
        ];
        return $fieldsList;
    }

    // 导出职位列表数据
    function xls_action()
    {
        $where = $_SESSION['jobXls'] ? $_SESSION['jobXls'] : array('orderby' => 'id');
        if (!empty($_POST['type'])) {
            foreach ($_POST['type'] as $v) {
                if ($v == 'lastdate') {
                    $type[] = 'lastupdate';
                } else {
                    $type[] = $v;
                }
            }
            $field = @implode(',', $type);
        } else {
            $field = 'uid';
        }
        if ($_POST['pid']) {
            $ids = @explode(',', $_POST['pid']);
            $where['id'] = array('in', pylode(',', $ids));
        }
        if ($_POST['limit']) {
            $where['limit'] = intval($_POST['limit']);
        }
        $jobM = $this->MODEL('job');
        $listNew = $jobM->getList($where, array('cache' => 1, 'field' => $field));
        $jobs = $listNew['list'];
        $cache = $listNew['cache'];
        if (!empty($jobs)) {
            $fieldsList = $this->getFields();
            $fields = array_keys($fieldsList);
            $exportFields = explode(',', $field);
            foreach ($exportFields as $fval) {
                if (in_array($fval, $fields)) {
                    $export_type[] = $fval;
                }
            }
            include_once LIB_PATH . 'libs/PHPExcel.php';
            $objPHPExcel = new PHPExcel();
            $objPHPExcel->setActiveSheetIndex(0);
            $col = 'A';
            // 循环字段
            foreach ($export_type as $tval) {
                $width = 20;
                $objPHPExcel->getActiveSheet()->getColumnDimension($col)->setWidth($width); // 设置列宽
                $objPHPExcel->getActiveSheet()->setCellValue($col . '1', $fieldsList[$tval]); // 设置表头
                $col++;
            }
            foreach ($jobs as $key => $val) {
                $col = 'A';
                // 循环字段
                foreach ($export_type as $tval) {
                    if (in_array($tval, array('hy', 'exp', 'report', 'sex', 'edu', 'marriage', 'pr', 'mun'))) {
                        $text = $val['job_' . $tval];
                    } else if ($tval == 'job1') {
                        $text = $val['job_one_n'];
                    } else if ($tval == 'job1_son') {
                        $text = $val['job_two_n'];
                    } else if ($tval == 'job_post') {
                        $text = $val['job_three_n'];
                    } else if ($tval == 'provinceid') {
                        $text = $val['job_city_one'] ? $val['job_city_one'] : '';
                    } else if ($tval == 'cityid') {
                        $text = $val['job_city_two'] ? $val['job_city_two'] : '';
                    } else if ($tval == 'three_cityid') {
                        $text = $val['job_city_three'] ? $val['job_city_three'] : '';
                    } else if ($tval == 'minsalary,maxsalary') {
                        if ($val['minsalary'] && $val['maxsalary']){
                            $text = '￥' . $val['minsalary'] . '-' . $val['maxsalary'];
                        } else if ($val['minsalary']) {
                            $text = '￥' . $val['minsalary'] . '以上';
                        } else {
                            $text = '面议';
                        }
                    } else if ($tval == 'type') {
                        $text = $cache['comclass_name'][$val['type']];
                    } else if ($tval == 'sdate') {
                        $text = date('Y-m-d', $val[$tval]);
                    } else if ($tval == 'lastdate') {
                        $text = date('Y-m-d', $val['lastupdate']);
                    } else if ($tval == 'zp_minage,zp_maxage') {
                        if ($val['zp_minage'] && $val['zp_maxage']) {
                            $text = $val['zp_minage'] . '-' . $val['zp_maxage'] . '岁';
                        } else if ($val['zp_minage']) {
                            $text = $val['zp_minage'] . '岁以上';
                        }
                    } else if ($tval == 'lang') {
                        $text = $val['lang_n'] ? $val['lang_n'] : '';
                    } else {
                        $text = $val[$tval] ? $val[$tval] : '';
                    }
                    if (in_array($tval, array('id', 'uid'))) { // 数字转字符的字段
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
                'file_name' => '职位信息' . date('YmdHis') . '.xlsx'
            ];
            return $this->admin_json(0, "导出职位信息", $data);
        } else {
            $this->render_json(1, '暂无符合条件的职位数据');
        }
    }

    /* 职位匹配简历 */
    function matching_action()
    {

        if ($_POST['id']) {
            $id = intval($_POST['id']);
            $where = '1';
            $where .= ' and state=1';
            $where .= ' and status=1';
            $where .= ' and r_status=1';
            $where .= ' and defaults=1';
            $ResumeM = $this->MODEL('resume');
            $JobM = $this->MODEL('job');
            $jobinfo = $JobM->getInfo(array('id' => $id), array('field' => 'id,uid,job1,job1_son,job_post,provinceid,cityid,three_cityid'));
            $rt['comid'] = $jobinfo['uid'];
            if ($jobinfo) {
                if ($_POST['keyword']) {
                    $keyword = trim($_POST['keyword']);
                    $workWhere = array(
                        'name' => array('like', $keyword),
                        'title' => array('like', $keyword, 'OR'),
                    );
                    $work = $ResumeM->getResumeWorks($workWhere, 'eid');
                    if ($work) {
                        $eids = array();
                        foreach ($work as $v) {
                            $eids[] = $v['eid'];
                        }
                    }
                    $eduWhere = array(
                        'name' => array('like', $keyword),
                        'specialty' => array('like', $keyword, 'OR')
                    );
                    $edu = $ResumeM->getResumeEdus($eduWhere, 'eid');
                    if ($edu) {
                        $eids = array();
                        foreach ($edu as $v) {
                            $eids[] = $v['eid'];
                        }
                    }
                    $UserinfoM = $this->MODEL('userinfo');
                    $mwhere['description'] = array('like', $keyword);
                    if (!empty($mwhere)) {
                        $uidList = $UserinfoM->getUserInfoList($mwhere, array('usertype' => 1, 'field' => '`uid`'));
                        if (!empty($uidList)) {
                            foreach ($uidList as $uv) {
                                $mUids[] = $uv['uid'];
                            }
                        }
                    }
                    $where .= " and (a.name like '%$keyword%'";
                    if (!empty($mUids)) {
                        $where .= " or a.uid in (" . pylode(',', $mUids) . ")";
                    }
                    if (!empty($eids)) {
                        $where .= " or a.id in (" . pylode(',', $eids) . ")";
                    }
                    $where .= ")";
                }
                //  学历要求
                if ($_POST['edu']) {
                    $eduKey = $this->obj->select_once('userclass', array('variable' => 'user_edu'), "`id`");
                    $eduReq = $this->obj->select_once('userclass', array('id' => $_POST['edu']), "`sort`,`name`");
                    if ($eduReq['name'] != "不限") {
                        $eduArr = $this->obj->select_all('userclass', array('keyid' => $eduKey['id'], 'sort' => array('>=', $eduReq['sort'])), "`id`");
                        $eduIds = array();
                        foreach ($eduArr as $v) {
                            $eduIds[] = $v['id'];
                        }
                        $where .= " and a.edu in (" . pylode(',', $eduIds) . ") ";
                    }
                }
                //  工作经验
                if ($_POST['exp']) {
                    $expKey = $this->obj->select_once('userclass', array('variable' => 'user_word'), "`id`");
                    $expReq = $this->obj->select_once('userclass', array('id' => $_POST['exp']), "`sort`,`name`");
                    if (isset($expReq) && $expReq['name'] != "不限") {
                        $expArr = $this->obj->select_all('userclass', array('keyid' => $expKey['id'], 'sort' => array('>=', $expReq['sort'])), "`id`");
                        $expIds = array();
                        foreach ($expArr as $v) {
                            $expIds[] = $v['id'];
                        }
                        $where .= " and a.exp in (" . pylode(',', $expIds) . ") ";
                    }
                }
                //  工作城市
                if ($_POST['city_class']) {
                    $cityclass = explode(',', $_POST['city_class']);
                    $rt['cityArr'] = $cityclass;
                }
                //  工作职能
                if ($_POST['job_class']) {
                    $jobclass = explode(',', $_POST['job_class']);
                    $rt['jobArr'] = $jobclass;
                }
                //  简历标签
                if ($_POST['label']) {
                    $where .= ' and a.label=' . intval($_POST['label']);
                }
                //  简历备注
                if ($_POST['content']) {
                    $where .= " and a.content like '%" . trim($_POST['content']) . "%'";
                }

            }
            $record = $ResumeM->getResTsList(array('jobid' => $id), array('field' => '`eid`'));
            if (!empty($record)) {
                foreach ($record as $v) {
                    $eids[] = $v['eid'];
                }
                $where .= " and a.id not in (" . pylode(',', $eids) . ") ";
            }
            $noUids = array();
            $blackM = $this->MODEL('black');
            $black = $blackM->getBlackList(array('p_uid' => $jobinfo['uid']));
            if (!empty($black)) {
                foreach ($black as $v) {
                    $buids[] = $v['c_uid'];
                }
                if (!empty($buids)) {
                    $noUids = $buids;
                }
            }
            $applyList = $JobM->getSqJobList(array('job_id' => $jobinfo['id']), array('field' => '`uid`', 'utype' => 'simple'));
            if (!empty($applyList)) {
                $sqUids = array();
                foreach ($applyList as $v) {
                    $sqUids[] = $v['uid'];
                }
                if (!empty($sqUids)) {
                    $noUids = !empty($noUids) ? array_merge($noUids, $sqUids) : $sqUids;
                }
            }
            $yqList = $JobM->getYqmsList(array('fid' => $jobinfo['uid']), array('field' => '`uid`', 'utype' => 'simple'));
            if (!empty($yqList)) {
                $yqUids = array();
                foreach ($yqList as $v) {
                    $yqUids[] = $v['uid'];
                }
                if (!empty($yqUids)) {
                    $noUids = !empty($noUids) ? array_merge($noUids, $yqUids) : $yqUids;
                }
            }
            if (!empty($noUids)) {
                $where .= " and a.uid not in (" . pylode(',', $noUids) . ") ";
            }
            include(PLUS_PATH . 'city.cache.php');
            include(PLUS_PATH . 'cityparent.cache.php');
            include(PLUS_PATH . 'job.cache.php');
            include(PLUS_PATH . 'jobparent.cache.php');
            $city_job_class = '';
            if ($_POST['job_class'] || $_POST['city_class']) {
                $city_col = $job_col = '';
                $cjwhere = '';
                if ($_POST['job_class']) {
                    if ($job_parent[$_POST['job_class']] == '0') {
                        $job_col = "job1";
                        $cjwhere .= "$job_col = {$_POST['job_class']}";
                    } elseif (in_array($job_parent[$_POST['job_class']], $job_index)) {
                        $job_col = "job1_son";
                        $cjwhere .= "$job_col = {$_POST['job_class']}";
                    } elseif ($job_parent[$_POST['job_class']] > 0) {
                        $job_col = "job_post";
                        $cjwhere .= "$job_col = {$_POST['job_class']}";
                    }
                }
                if ($_POST['city_class']) {
                    $cjand = $cjwhere ? ' AND ' : '';
                    if ($city_parent[$_POST['city_class']] == '0') {
                        $city_col = "provinceid";
                        $cjwhere .= "{$cjand}$city_col = {$_POST['city_class']}";
                    } elseif (in_array($city_parent[$_POST['city_class']], $city_index)) {
                        $city_col = "cityid";
                        $cjwhere .= "{$cjand}$city_col = {$_POST['city_class']}";
                    } elseif ($city_parent[$_POST['city_class']] > 0) {
                        $city_col = "three_cityid";
                        $cjwhere .= "{$cjand}$city_col = {$_POST['city_class']}";
                    }
                }
                // 拼接唯一标识字段
                if ($city_col || $job_col) {
                    if ($city_col && $job_col) {
                        $cjwhere .= " AND {$city_col}_{$job_col}_num = 1";
                    } elseif ($city_col) {
                        $cjwhere .= " AND {$city_col}_num = 1";
                    } elseif ($job_col) {
                        $cjwhere .= " AND {$job_col}_num = 1";
                    }
                }
                $city_job_class = ",(select `eid` from `" . $this->def . "resume_city_job_class` where $cjwhere) cj";
                $where .= " and a.id = cj.eid";
            }
            $countSql = "select count(*) as num from `" . $this->def . "resume_expect` a{$city_job_class} where {$where}";
            $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
            $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
            //提取分页
            $pageM = $this->MODEL('page');
            $pages = $pageM->adminPageList('resume_expect', $where, $page, array('limit' => $pageSize, 'sql' => $countSql));
            $order = '';
            //分页数大于0的情况下 执行列表查询
            if ($pages['total'] > 0) {
                //limit order 只有在列表查询时才需要
                if ($_POST['order']) {
                    if ($_POST['t'] == 'time') {
                        $order .= "order by a.lastupdate " . $_POST['order'];
                    } else {
                        $order .= 'order by a.' . $_POST['t'] . ' ' . $_POST['order'];
                    }
                } else {
                    $order .= 'order by a.lastupdate desc';
                }
                $sql = "select a.* from `" . $this->def . "resume_expect` a{$city_job_class} where {$where} {$order} limit {$pages['limit'][0]},{$pages['limit'][1]}";
                $List = $ResumeM->getList(array(), array('cache' => 1, 'utype' => 'admin', 'sql' => $sql));
            }
            $rt['list'] = $List['list'] ? $List['list'] : array();
            $rt['total'] = intval($pages['total']);
            $rt['perPage'] = $pageSize;
            $rt['pageSizes'] = $pages['page_sizes'];
            $this->render_json(0, '', $rt);

        }
    }
    public function getResumeCacheData_action(){
        $cacheM = $this->MODEL('cache');
        $CacheList = $cacheM->GetCache(array('user', 'job', 'city'));
        $setArr = array(
            'userdata' => $CacheList['userdata'],
            'userclass_name' => $CacheList['userclass_name'],
            'job_name' => $CacheList['job_name'],
            'city_name' => $CacheList['city_name']
        );
        $rt['resume_cache'] = $setArr;
        $this->render_json(0, '', $rt);
    }
    function jobNum_action()
    {
        $MsgNum = $this->MODEL('msgNum');
        $rt = json_decode($MsgNum->jobNum(), 1);
        $this->render_json(0, '', $rt);
    }

    /**
     * @desc 企业微海报
     */
    function whb_action()
    {
        $WhbM = $this->MODEL('whb');
        $imgList = $WhbM->getWhbList(array('type' => 1, 'isopen' => '1', 'orderby' => 'sort,desc'));
        $this->render_json(0, '', array('comHb' => $imgList, 'hburl' => $this->config['sy_weburl'] . '/index.php?m=ajax&c=getJobHb'));
    }

    function getJobHtml_action()
    {
        if ($_POST['id']) {
            $wxpubtempM = $this->MODEL('wxpubtemp');
            $html = $wxpubtempM->getOneJob($_POST['id'], 'admin');
            $this->render_json(0, ' ', $html);
        }
    }

    function addTuiWenTask_action()
    {
        if ($_POST) {
            $wxPubM = $this->MODEL('wxpubtemp');
            $data = array(
                'jobid' => $_POST['twtask_jobid'],
                'content' => $_POST['twtask_content'],
                'urgent' => $_POST['twtask_urgent'],
                'wcmoments' => $_POST['twtask_wcmoments'],
                'gzh' => $_POST['twtask_gzh'],
                'type' => 1
            );
            $return = $wxPubM->addTwTask($data, array('auid' => $_SESSION['auid']));
            if ($return['code'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    // 职位单个审核，带职位详情
    function jobAudit_action()
    {
        $jobid = intval($_POST['id']);
        $JobM = $this->MODEL('job');
        $Info = $JobM->getInfo(array('id' => $jobid));
        $memberInfo = $this->obj->select_once('member', array('uid' => $Info['uid']), '`status`,`lock_info`,`reg_date`,`login_date`');
        $comInfo = $this->obj->select_once('company', array('uid' => $Info['uid']), '`linkman`, `linktel`,`linkphone`,`linkmail`, `address`, `crm_uid`,`rating_name`');
        if ($Info['is_link'] == 2) {
            $link = $this->obj->select_once('company_job_link', array('id' => $Info['link_id']));
            $Info['tel'] = $link['link_moblie'];
            $Info['phone'] = $link['link_phone'];
            $Info['address'] = $link['link_address'];
            $Info['linkman'] = $link['link_man'];
        } else {
            $Info['tel'] = $comInfo['linktel'];
            $Info['phone'] = $comInfo['linkphone'];
            $Info['address'] = $comInfo['address'];
            $Info['linkman'] = $comInfo['linkman'];
        }
        $Info['crm_salesman'] = '';
        $Info['rating_name'] = $comInfo['rating_name'];
        if ($comInfo['crm_uid']) {
            $crmuser = $this->obj->select_once('admin_user', array('uid' => $comInfo['crm_uid']), '`uid`,`name`');
            $Info['crm_salesman'] = $crmuser['name'] ? $crmuser['name'] : '';
        }
        $Info['c_status'] = $memberInfo['status'];
        $Info['statusbody'] = trim($memberInfo['lock_info']);
        $Info['reg_date_n'] = date('Y-m-d H:i:s', $memberInfo['reg_date']);
        $Info['login_date_n'] = $memberInfo['login_date'] > 0 ? date('Y-m-d H:i:s', $memberInfo['login_date']) : '';
        $Info['jobname'] = $Info['name'];
        $rt['info'] = $Info;
        // 待审核数量
        $snum = $JobM->getJobNum(array('state' => 0, 'id' => array('<>', $jobid)));
        $rt['snum'] = $snum;
        $this->render_json(0, '', $rt);
    }

    function reserveJob_action()
    {
        $JobM = $this->MODEL('job');
        $where = array(
            'is_reserve' => 1,
            'state' => 1,
            'status' => 0,
            'r_status' => 1
        );
        if (isset($_POST['keyword'])) {
            $keyStr = trim($_POST['keyword']);
            $typeStr = (int)$_POST['type'];
            if ($typeStr == 1) {
                $where['com_name'] = array('like', $keyStr);
            } elseif ($typeStr == 2) {
                $where['name'] = array('like', $keyStr);
            }
        }
        if (isset($_POST['uid']) && !empty($uid)) {
            $where['uid'] = $_POST['uid'];
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company_job', $where, $page, array('limit' => $pageSize));
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'lastupdate,desc';
            }
            $where['limit'] = $pages['limit'];
            $ListJob = $JobM->getList($where, array('utype' => 'admin', 'isurl' => 'yes', 'reserve' => 1));
        }
        $rt['list'] = $ListJob['list'] ? $ListJob['list'] : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }

    function closeReserve_action()
    {
        $JobM = $this->Model('job');
        $ids = $_POST['ids'];
        $return = $JobM->closeReserve(array('jobids' => $ids, 'utype' => 'admin'));
        if ($return['errcode'] == 9) {
            $this->MODEL('log')->addAdminLog("关闭职位（ID" . $ids . "）预约刷新");
        }
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function ajaxCloseReserve_action()
    {
        $JobM = $this->Model('job');
        $return = $JobM->closeReserve(array('auto' => 1));
        echo json_encode($return);
        die;
    }

    function upReserveJob_action()
    {
        $jobM = $this->MODEL('job');
        $data = array(
            'job_id' => $_POST['job_id'],
            'end_time' => strtotime($_POST['end_time']),
            'interval' => $_POST['interval'],
            'status' => $_POST['status'],
            's_time' => $_POST['s_time'],
            'e_time' => $_POST['e_time']
        );
        $return = $jobM->reserveUpJob($data, array('utype' => 'admin', 'uid' => $_POST['uid']));
        $return['errcode'] = $return['error'] == 1 ? 9 : 8;
        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /**
     * @desc 职位匹配简历投递
     */
    function applyJob_action()
    {
        $where = array(
            'eid' => intval($_POST['eid']),
            'uid' => intval($_POST['uid']),
            'job_id' => intval($_POST['id']),
            'com_id' => intval($_POST['comid'])
        );
        $jobM = $this->MODEL('job');
        $return = $jobM->applyJobByAdmin($where, array('auid' => $_SESSION['auid']));
        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function saveAddress_action()
    {
        $addressM = $this->MODEL('address');
        $linkData = array(
            'uid' => $_POST['uid'],
            'link_man' => $_POST['link_man'],
            'link_moblie' => $_POST['link_moblie'],
            'link_phone' => $_POST['link_phone'],
            'email' => $_POST['email'],
            'link_address' => $_POST['link_address'],
            'provinceid' => $_POST['provinceid'],
            'cityid' => $_POST['cityid'],
            'three_cityid' => $_POST['three_cityid'],
            'x' => $_POST['x'],
            'y' => $_POST['y']
        );
        $result = $addressM->saveAddress($linkData, array('utype' => 'admin'));
        $msg = $result ? '工作地址添加成功' : '工作地址添加失败';
        $link_id = $result;
        $errcode = $result ? 9 : 8;
        if ($result && (int)$_POST['is_link'] == 2) {
            $addressList = $addressM->getAddressList(array('uid' => $_POST['uid']));
            $comM = $this->MODEL('company');
            $this->comInfo = $comM->getInfo($_POST['uid'], array('info' => '1', 'edit' => '1', 'logo' => '1', 'utype' => 'user'));
            $defLink = array(
                'link_man' => $this->comInfo['linkman'],
                'link_moblie' => $this->comInfo['linktel'],
                'link_phone' => $this->comInfo['linkphone'],
                'email' => $this->comInfo['linkmail'],
                'city' => $this->comInfo['job_city_one'] . $this->comInfo['job_city_two'] . $this->comInfo['job_city_three'],
                'provinceid' => $this->comInfo['provinceid'],
                'cityid' => $this->comInfo['cityid'],
                'three_cityid' => $this->comInfo['three_cityid'],
                'address' => $this->comInfo['address'],
                'x' => $this->comInfo['x'],
                'y' => $this->comInfo['y']
            );
            $this->render_json($errcode == 9 ? 0 : 1, $msg, array('addressList' => $addressList, 'link_id' => $link_id));
        } else {
            $this->render_json($errcode == 9 ? 0 : 1, $msg);
        }
    }

    /**
     *users:王旭
     *Data:2023/2/4
     *Time:9:25
     *
     * 单职位联系方式选择性开放
     */
    function setlinkopen_action()
    {
        if (!empty($_POST['linkopen'])) {
            $JobM = $this->Model('job');
            $return = $JobM->setlinkopen(array('linkopen' => intval($_POST['linkopen'])), array('id' => intval($_POST['linkjobid'])));
            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    /**
     * 职位降权
     */
    function depower_action()
    {
        $jobM = $this->MODEL('job');
        if (empty($_POST['id']) || empty($_POST['is_depower'])) {
            $this->render_json(1, '参数错误');
        }
        $where['id'] = intval($_POST['id']);
        $data['is_depower'] = intval($_POST['is_depower']);
        $return = $jobM->upInfo($data, $where);
        $msg = $data['is_depower'] == 1 ? '降权' : '取消降权';
        if ($return) {
            $this->admin_json(0, "职位{$msg}（ID：{$where['id']}）操作成功");
        } else {
            $this->render_json(1, "职位{$msg}操作失败");
        }
    }
}