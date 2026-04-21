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
class users_resume_controller extends adminCommon
{
    // 设置高级搜索功能
    function set_search()
    {
        $cacheM		=	$this -> MODEL('cache');
        $CacheList	=	$cacheM -> GetCache(array('user', 'job', 'city'));

        $userdata       =   $CacheList['userdata'];
        $userclass_name =   $CacheList['userclass_name'];

        foreach($userdata['user_type'] as $k=>$v){
            $type[$v]   =   $userclass_name[$v];
        }

        foreach($userdata['user_edu'] as $k=>$v){
            $edu[$v]    =   $userclass_name[$v];
        }

        foreach($userdata['user_word'] as $k=>$v){
            $exp[$v]    =   $userclass_name[$v];
        }

        foreach($userdata['user_report'] as $k=>$v){
            $report[$v] =   $userclass_name[$v];
        }
        foreach($userdata['user_marriage'] as $k=>$v){
            $marriage[$v] =   $userclass_name[$v];
        }

        /* @var $arr_data */
        include(CONFIG_PATH.'db.data.php');

        $source		=    $arr_data['source'];
        $status		=	array('1'=>'已审核','2'=>'已锁定','3'=>'未通过','4'=>'未审核');
        $service	=	array('1'=>'置顶','2'=>'推荐');
        $integrity  =	$arr_data['integrity_name'];
        $salary     =   array('2000_4000' => '2000-4000', '4000_6000' => '4000-6000', '6000_8000' => '6000-8000', '8000_10000' => '8000-10000', '10000' => '10000以上');
        $age        =   array('16_20' => '16-20岁', '21_30' => '21-30岁', '31_40' => '31-40岁', '41_50' => '41-50岁', '50' => '50岁以上');
        $sex        =   $arr_data['sex'];
        $isRemark	=	array('1'=>'是','2'=>'否');

        $search_list[]	=	array('param'=>'status','name'=>'审核状态','value'=>$status);
        $search_list[]	=	array('param'=>'source','name'=>'数据来源','value'=>$source);
        $search_list[]	=	array('param'=>'service','name'=>'简历类型','value'=>$service);
        $search_list[]	=	array('param'=>'type','name'=>'工作性质','value'=>$type);
        $search_list[]	=	array('param'=>'salary','name'=>'薪资','value'=>$salary);
        $search_list[]	=	array('param'=>'age','name'=>'年龄','value'=>$age);
        $search_list[]	=	array('param'=>'sex','name'=>'性别','value'=>$sex);
        $search_list[]	=	array('param'=>'marriage','name'=>'婚姻状况','value'=>$marriage);
        $search_list[]	=	array('param'=>'remark','name'=>'是否备注','value'=>$isRemark);

        // 其它条件
        $search_list[]	=	array('param'=>'edu','name'=>'学历要求','value'=>$edu);
        $search_list[]	=	array('param'=>'exp','name'=>'工作经验','value'=>$exp);
        $search_list[]	=	array('param'=>'report','name'=>'到岗时间','value'=>$report);
        $search_list[]	=	array('param'=>'integrity','name'=>'完整度','value'=>$integrity);

        return compact('source', 'search_list');
    }
    /**
     * 会员-个人-简历管理
     */
    function index_action()
    {
        $resumeM = $this->MODEL('resume');

        $where = '1';

        /* @var $arr_data */
        include(CONFIG_PATH . 'db.data.php');

        //搜索类型和搜索关键字
        if ($_POST['keyword']) {
            $keytype = intval($_POST['keytype']);
            $keyword = trim($_POST['keyword']);

            if ($keytype == 1) {
                $where .= " and a.name like '%$keyword%'";
            } elseif ($keytype == 2) {
                $where .= " and a.uname like '%$keyword%'";
            } elseif ($keytype == 3) {
                $where .= " and a.id = $keyword";
            } elseif ($keytype == 4) {
                $mUids = array();

                $UserinfoM = $this->MODEL('userinfo');
                $mwhere['username'] = array('like', $keyword);
                if (!empty($mwhere)) {
                    $uidList = $UserinfoM->getList($mwhere, array('field' => '`uid`'));

                    if (!empty($uidList)) {
                        foreach ($uidList as $uv) {
                            $mUids[] = $uv['uid'];
                        }
                    }

                    $where .= " and a.uid in (" . pylode(',', $mUids) . ")";
                }
            } elseif ($keytype == 5) {
                $mUids = array();
                $UserinfoM = $this->MODEL('userinfo');
                $mwhere['telphone'] = array('like', $keyword);
                if (!empty($mwhere)) {
                    $uidList = $UserinfoM->getUserInfoList($mwhere, array('usertype' => 1, 'field' => '`uid`'));
                    if (!empty($uidList)) {
                        foreach ($uidList as $uv) {
                            $mUids[] = $uv['uid'];
                        }
                    }
                    $where .= " and a.uid in (" . pylode(',', $mUids) . ")";
                }
            } elseif ($keytype == 6) {                      //  教育经历
                $eduWhere = array(
                    'name' => array('like', $keyword),
                    'title' => array('like', $keyword, 'OR'),
                    'specialty' => array('like', $keyword, 'OR')
                );

                $edu = $resumeM->getResumeEdus($eduWhere, 'eid');

                $eids = array();
                foreach ($edu as $v) {
                    $eids[] = $v['eid'];
                }

                $where .= " and a.id in (" . pylode(',', $eids) . ")";

            } elseif ($keytype == 7) {                      //  工作经历
                $workWhere = array(
                    'name' => array('like', $keyword),
                    'title' => array('like', $keyword, 'OR'),
                    'content' => array('like', $keyword, 'OR')
                );

                $work = $resumeM->getResumeWorks($workWhere, 'eid');

                $eids = array();
                foreach ($work as $v) {
                    $eids[] = $v['eid'];
                }

                $where .= " and a.id in (" . pylode(',', $eids) . ")";
            } elseif ($keytype == 8) {                      //  项目经历
                $proWhere = array(
                    'name' => array('like', $keyword),
                    'title' => array('like', $keyword, 'OR'),
                    'content' => array('like', $keyword, 'OR')
                );

                $work = $resumeM->getResumeProjects($proWhere, 'eid');
                $eids = array();
                foreach ($work as $v) {
                    $eids[] = $v['eid'];
                }
                $where .= " and a.id in (" . pylode(',', $eids) . ")";
            } elseif ($keytype == 9) {                      //  培训经历
                $trainWhere = array(
                    'name' => array('like', $keyword),
                    'title' => array('like', $keyword, 'OR'),
                    'content' => array('like', $keyword, 'OR')
                );
                $work = $resumeM->getResumeTrains($trainWhere, 'eid');
                $eids = array();
                foreach ($work as $v) {
                    $eids[] = $v['eid'];
                }
                $where .= " and a.id in (" . pylode(',', $eids) . ")";
            } elseif ($keytype == 10) {                      //  职业技能
                $skillWhere = array(
                    'name' => array('like', $keyword),
                    'title' => array('like', $keyword, 'OR'),
                    'content' => array('like', $keyword, 'OR')
                );
                $work = $resumeM->getResumeSkills($skillWhere, 'eid');
                $eids = array();
                foreach ($work as $v) {
                    $eids[] = $v['eid'];
                }
                $where .= " and a.id in (" . pylode(',', $eids) . ")";
            } elseif ($keytype == 11) {
                $where .= " and a.add_ip like '%$keyword%'";
            }
        }

        //审核状态
        if ($_POST['status']) {
            $status = intval($_POST['status']);
            if ($status == 2) {
                $where .= " and a.r_status = 2";
            } else {
                $where .= " and a.state = " . ($status == 4 ? 0 : $status);
            }
        }

        //来源
        if ($_POST['source']) {
            $where .= " and a.source = " . intval($_POST['source']);
        }

        if (is_array($_POST['times']) && count($_POST['times']) == 2) {
            //创建时间
            if ($_POST['time_type'] == 'adtime') {
                $where .= " and a.ctime >= " . strtotime($_POST['times'][0] . ' 00:00:00') . " and a.ctime <= " . strtotime($_POST['times'][1] . ' 23:59:59');
            }
            //更新时间
            if ($_POST['time_type'] == 'uptime') {
                $where .= " and a.lastupdate >= " . strtotime($_POST['times'][0] . ' 00:00:00') . " and a.lastupdate <= " . strtotime($_POST['times'][1] . ' 23:59:59');
            }
        }

        //工作性质
        if ($_POST['type']) {
            $where .= " and a.type = " . intval($_POST['type']);
        }

        //学历要求
        if ($_POST['edu']) {
            $where .= " and a.`edu` = " . $_POST['edu'];
        }

        //工作经验
        if ($_POST['exp']) {
            $where .= " and a.`exp` = " . $_POST['exp'];
        }
        //到岗时间
        if ($_POST['report']) {
            $where .= " and a.report = " . intval($_POST['report']);
        }

        //  性别
        if ($_POST['sex']) {
            $where .= " and a.sex = " . intval($_POST['sex']);
        }

        //简历完整度
        if ($_POST['integrity']) {
            if ($_POST['integrity'] == 55) {
                $where .= " and a.integrity = 55";
            } else if ($_POST['integrity'] == 65) {
                $where .= " and a.integrity = 65";
            } else {
                $integrity_val = $arr_data['integrity_val'];
                $where .= " and a.integrity >= " . $integrity_val[$_POST['integrity']];
            }
        }

        //薪资待遇
        if (!empty($_POST['salary'])) {
            $salary = explode('_', $_POST['salary']);
            if ($salary[0] && $salary[1]) {
                $where .= " AND ((a.`minsalary`<=" . intval($salary[0]) . " and a.`maxsalary`>=" . intval($salary[0]) . ")
                 or (a.`minsalary`<=" . intval($salary[1]) . " and a.`maxsalary`>=" . intval($salary[1]) . "))";
            } elseif ($salary[0] && !$salary[1]) {
                $where .= " AND ((a.`minsalary`<=" . intval($salary[0]) . " and a.`maxsalary`>=" . intval($salary[0]) . ") 
						or (a.`minsalary`>=" . intval($salary[0]) . " and a.`maxsalary`>=" . intval($salary[0]) . ")
						or (a.`minsalary`!=0 and  a.`maxsalary`=0))";
            } elseif (!$salary[0] && $salary[1]) {
                $where .= " AND ((a.`minsalary`<=" . intval($salary[1]) . " and a.`maxsalary`>=" . intval($salary[1]) . ")
						or (a.`minsalary`<=" . intval($salary[1]) . " and a.`maxsalary`<=" . intval($salary[1]) . ")  
						or (a.`minsalary`<=" . intval($salary[1]) . " and a.`maxsalary`=0) 
						or (a.`minsalary`=0 and a.`maxsalary`!=0)
						)";
            }
        }

        //年龄
        if (!empty($_POST['age'])) {
            $age = explode('_', $_POST['age']);
            if ($age[0] && $age[1]) {
                $mintime = date("Y-m-d", strtotime("-" . $age[0] . " year"));
                $maxtime = date("Y-m-d", strtotime("-" . $age[1] . " year"));
                $where .= " AND a.`birthday`>= '" . $maxtime . "' and a.`birthday`<='" . $mintime . "'";
            } elseif ($age[0] && !$age[1]) {
                $mintime = date("Y-m-d", strtotime("-" . $age[0] . " year"));
                $where .= " AND a.`birthday`<='" . $mintime . "'";
            } elseif (!$age[0] && $age[1]) {
                $maxtime = date("Y-m-d", strtotime("-" . $age[1] . " year"));
                $where .= " AND a.`birthday`>='" . $maxtime . "'";
            }
        }

        if ($_POST['remark'] == 1) {
            $where .= " and (a.content IS NOT NULL or a.label > 0)";
        } elseif ($_POST['remark'] == 2) {
            $where .= " and a.content IS NULL and a.label IS NULL";
        }

        //简历类型
        if ($_POST['service']) {
            $service = intval($_POST['service']);
            if ($service == 1) {
                //置顶
                $where .= " and a.top = 1";
                $where .= " and a.topdate > " . time();
            } elseif ($service == 2) {
                //推荐
                $where .= " and a.rec_resume = 1";
            }
        }
        if ($_POST['teen'] == 1) {
            $datetime = strtotime('-16 years');
            $where .= " and UNIX_TIMESTAMP(a.`birthday`) >" . $datetime;
        }

        $city_job_class = '';
        if ($_POST['job_class'] || $_POST['city_class']) {
            /* @var $city_index */
            include_once(PLUS_PATH . 'city.cache.php');
            /* @var $city_parent */
            include_once(PLUS_PATH . 'cityparent.cache.php');
            /* @var $job_index */
            include_once(PLUS_PATH . 'job.cache.php');
            /* @var $job_parent */
            include_once(PLUS_PATH . 'jobparent.cache.php');
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
            $city_job_class = ",(select `eid` from `".$this->def."resume_city_job_class` where $cjwhere) cj";
            $where .= " and a.id = cj.eid";
        }

        //  婚姻
        if ($_POST['marriage']) {
            $marriageSql = ",(select `uid`,`marriage` from `" . $this->def . "resume` where `marriage` = " . $_POST['marriage'] . ") as ms";
            $where .= " and a.uid = ms.uid";
        }

        $countSql = "select count(*) as num from `" . $this->def . "resume_expect` a{$city_job_class}{$marriageSql} where {$where}";

        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('resume_expect', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true, 'sql' => $countSql));
        extract($pages);
        $limit = $pages['limit'][1];

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
            $sql = "select a.* from `" . $this->def . "resume_expect` a{$city_job_class}{$marriageSql} where {$where} {$order} limit {$pages['limit'][0]},{$pages['limit'][1]}";
            $List = $resumeM->getList(array(), array('cache' => 1, 'utype' => 'admin', 'sql' => $sql));

            foreach ($List['list'] as $key => $val) {
                $List['list'][$key]['integrity'] = intval($val['integrity']);
            }
        }
        $list = !empty($List['list']) ? $List['list'] : array();
        // 查无数据的搜索条件也记入session，导出时根据此条件查无数据，提示暂无数据
        //处理导出需要的where条件
        $_SESSION['resumeXls']  =   array(
            'where'             =>  $where,
            'order'             =>  $order,
            'city_job_class'    =>  $city_job_class,
            'marriageSql'       =>  $marriageSql
        );



        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }
    public function getConfig_action(){
        //高级搜索
        extract($this->set_search());

        $exportType = $this->getFields();
        $this->render_json(0, 'ok', compact('source','search_list','exportType'));
    }
    /**
     * 会员-个人-简历管理: 添加简历页面
     */
    public function addResume_action()
    {

    }
    /**
     * 会员-个人-简历管理: 添加简历保存
     */
    public function add_action(){
        if(isset($_POST['add']) && $_POST['add']){
            if ($_POST['add'] == 2){
                $cacheM = $this->MODEL('cache');

                $cache = $cacheM->GetCache('user');

                $resume = '';
                if ($_POST['uid']) {
                    $where['uid'] = intval($_POST['uid']);
                    $resumeM = $this->MODEL('resume');
                    $resume = $resumeM->getResumeInfo($where);
                }

                $user_sex = $cache['user_sex'];
                $userdata = $cache['userdata'];
                $userclass_name = $cache['userclass_name'];
                $this->render_json(0, 'ok', compact('resume', 'user_sex', 'userdata', 'userclass_name'));
            }else{
                $this->render_json(0);
            }
        }

        $resumeM  =  $this->MODEL('resume');

        $_POST    =  $this->post_trim($_POST);

        if($_POST['uid']){
            $uid    =  intval($_POST['uid']);

            $mData = array(
                'email'  => $_POST['email'],
                'moblie' => $_POST['moblie']
            );

            $rData = array(
                'name'        => 	$_POST['resume_name'],
                'sex'         => 	$_POST['sex'],
                'birthday'    => 	$_POST['birthday'],
                'living'      => 	$_POST['living'],
                'edu'         => 	$_POST['edu'],
                'exp'         => 	$_POST['exp'],
                'telphone'    => 	$_POST['moblie'],
                'email'       => 	$_POST['email'],
                'description' => 	$_POST['description'],
            );

            $result  =  $resumeM -> upResumeInfo(array('uid'=>$uid),array('mData'=>$mData,'rData'=>$rData));

            if ($result['id']){
                $this->admin_json(0, '下一步：填写求职意向', array('uid' => $uid));
            }elseif ($result['msg']){
                $this->render_json(1, $result['msg']);
            }else{
                $this->render_json(1, '保存失败，请重试');
            }
        }else{
            if($this->config['sy_uc_type']=='uc_center'){
                $this-> obj-> uc_open();

                $user = uc_get_user($_POST['username']);

                if ($user){
                    $this->render_json(1, '该会员已经存在');
                }
            }
            $userinfoM = $this->MODEL('userinfo');
            //检测用户名、手机号、邮箱是否已被注册
            $checkData = array(
                'username' => $_POST['username'],
                'moblie'   => $_POST['moblie'],
                'email'    => $_POST['email'],
            );
            $memberCheck = $userinfoM->addMemberCheck($checkData);

            if ($memberCheck['msg']){
                $this->render_json(1, $memberCheck['msg']);
            }

            $ip    =  fun_ip_get();
            $time  =  time();
            $pass  =  $_POST['password'];

            if($this->config['sy_uc_type']=='uc_center'){

                $uid  =  uc_user_register($_POST['username'],$pass,$_POST['email']);

                if($uid < 0){

                    switch($uid){
                        case "-1" : $data['msg']='用户名不合法!';
                            break;
                        case "-2" : $data['msg']='包含不允许注册的词语!';
                            break;
                        case "-3" : $data['msg']='用户名已经存在!';
                            break;
                        case "-4" : $data['msg']='Email 格式有误!';
                            break;
                        case "-5" : $data['msg']='Email 不允许注册!';
                            break;
                        case "-6" : $data['msg']='该 Email 已经被注册!';
                            break;
                    }
                    $this->render_json(1, $data['msg']);
                }else{
                    list($uid,$username,$email,$password,$salt)=uc_get_user($_POST['username'],$pass);
                }
            }else{
                $salt      =  substr(uniqid(rand()), -6);

                $password  =  passCheck($pass,$salt);
            }

            $mdata = array(
                'username'  =>  $_POST['username'],
                'password'  =>  $password,
                'usertype'  =>  1,
                'salt'      =>  $salt,
                'moblie'    =>  $_POST['moblie'],
                'email'     =>  $_POST['email'],
                'reg_date'  =>  $time,
                'reg_ip'    =>  $ip,
                'status'    =>  1
            );
            $udata = array(
                'name'         =>  $_POST['resume_name'],
                'sex'          =>  $_POST['sex'],
                'edu'          =>  $_POST['edu'],
                'exp'          =>  $_POST['exp'],
                'birthday'     =>  $_POST['birthday'],
                'email'        =>  $_POST['email'],
                'telphone'     =>  $_POST['moblie'],
                'description'  =>  $_POST['description'],
                'living'       =>  $_POST['living'],
                'r_status'	   =>  1
            );

            $nid  =  $userinfoM -> addInfo(array('mdata'=>$mdata,'udata'=>$udata,'sdata'=>array('integral'=>0)));

            if($nid > 0){
                $this->admin_json(0, '下一步：填写求职意向', array('uid' => $nid));
            }else{
                $this->render_json(1, '会员添加失败，请重试');
            }
        }
    }
    /**
     * 会员-个人-简历管理: 修改简历页面
     */
    public function editResume_action(){
        $uid = intval($_POST['uid']);

        $expectWhere = array('uid' => $uid, 'tb' => 'all', 'needCache' => 1);
        if ($_POST['id']) {
            $expectWhere['eid'] = intval($_POST['id']);
        }
        $resumeM = $this->MODEL('resume');

        $return = $resumeM->getInfo($expectWhere);

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

        $where['uid'] = $uid;

        $resume = $resumeM->getResumeInfo($where, array('logo' => 2));

        $user_sex = $return['cache']['user_sex'];
        $userdata = $return['cache']['userdata'];
        $userclass_name = $return['cache']['userclass_name'];
        $industry_index = $return['cache']['industry_index'];
        $industry_name = $return['cache']['industry_name'];

        $this->render_json(0, 'ok', compact('resume', 'expectData', 'user_sex', 'userdata', 'userclass_name', 'industry_index', 'industry_name', 'snum'));
    }
    /**
     * 会员-个人-简历管理: 保存求职意向
     */
    public function saveExpect_action(){
        $eid = (int)$_POST['eid'];
        $uid = (int)$_POST['uid'];
        $time = time();

        $resumeM = $this->MODEL('resume');

        if ($eid == '') {
            $field = 'uid,name,edu,exp,sex,birthday,idcard_status,status,photo,phototype,r_status';

            $resumewhere['uid'] = $uid;

            $resume = $resumeM->getResumeInfo($resumewhere, array('field' => $field));

            $expectDate = array(
                'height_status' => 0,
                'r_status' => $resume['r_status'],
                'state' => 1, // 管理员添加的简历，默认是已审核状态 20220706 hl
                'integrity' => 55,
                'lastupdate' => $time,
                'ctime' => $time,
                'name' => $_POST['name'],
                'hy' => $_POST['hy'],
                'job_classid' => $_POST['job_classid'],
                'minsalary' => $_POST['minsalary'],
                'maxsalary' => $_POST['maxsalary'],
                'city_classid' => $_POST['city_classid'],
                'type' => $_POST['type'],
                'report' => $_POST['report'],
                'jobstatus' => $_POST['jobstatus'],
                'uid' => $resume['uid'],
                'edu' => $resume['edu'],
                'exp' => $resume['exp'],
                'uname' => $resume['name'],
                'sex' => $resume['sex'],
                'birthday' => $resume['birthday'],
                'idcard_status' => $resume['idcard_status'],
                'status' => $resume['status'],
                'photo' => $resume['photo'],
                'phototype' => $resume['phototype']
            );
            $return = $resumeM->addInfo(array('uid' => $uid, 'eData' => $expectDate, 'utype' => 'admin'));
            $eid = $return['id'];
            $msg = "简历(ID:{$eid})添加";
        } else {
            $expectDate = array(
                'name' => $_POST['name'],
                'hy' => $_POST['hy'],
                'job_classid' => $_POST['job_classid'],
                'minsalary' => $_POST['minsalary'],
                'maxsalary' => $_POST['maxsalary'],
                'city_classid' => $_POST['city_classid'],
                'type' => $_POST['type'],
                'report' => $_POST['report'],
                'jobstatus' => $_POST['jobstatus'],
                'lastupdate' => $time
            );

            $return = $resumeM->upInfo(array('id' => $eid), array('eData' => $expectDate, 'utype' => 'admin'));
            $msg = "求职意向(ID:{$eid})修改";
        }

        if ($return['id']) {
            $this->admin_json(0, $msg . '成功', compact('eid'));
        } else {
            $this->render_json(1, $msg . '失败');
        }
    }
    /**
     * TODO 会员-个人-简历管理: 保存个人优势
     */
    public function saveTag_action()
    {
        if (empty($_POST['uid'])) {
            $this->render_json(1, '参数错误');
        }
        if (is_array($_POST['tag']) && count($_POST['tag']) > 5) {
            $this->render_json(2, '标签最多选择5个');
        }
        if (strlen($_POST['description']) < 1) {
            $this->render_json(3, '请输入自我评价');
        }

        $uid = (int)$_POST['uid'];

        $resumeM = $this->MODEL('resume');

        $tagStr = '';
        if (!empty($_POST['tag']) && is_array($_POST['tag'])) {
            $tag = array_unique($_POST['tag']);
            foreach ($tag as $value) {
                $tagLen = mb_strlen($value);
                if ($tagLen >= 2 && $tagLen <= 8) {
                    $tagList[] = $value;
                }
                if (count($tagList) >= 5) {
                    break;
                }
            }
            $tagStr = implode(',', $tagList);
        }

        $rData = array(
            'tag' => $tagStr,
            'description' => $_POST['description'],
            'lastupdate' => time()
        );

        $return = $resumeM->upResumeInfo(array('uid' => $uid), array('rData' => $rData));

        if ($return['id']) {
            $this->admin_json(0, '简历(UID:' . $uid . ')个人优势保存成功');
        } else {
            $this->render_json(1, '简历个人优势保存失败');
        }
    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（工作经历处理）
     */
    public function work_action(){
        $eid = intval($_POST['eid']);

        $id = intval($_POST['id']);

        $uid = intval($_POST['uid']);

        $resumeM = $this->MODEL('resume');

        $post = array(
            'uid' => $uid,
            'eid' => $eid,
            'name' => $_POST['name'],
            'sdate' => strtotime($_POST['sdate']),
            'edate' => strtotime($_POST['edate']),
            'title' => $_POST['title'],
            'content' => $_POST['content']
        );

        if (!$id) {
            $return = $resumeM->addResumeWork($post, array('utype' => 'admin'));
            $id = $return['id'];
        } else {
            $return = $resumeM->addResumeWork($post, array('where' => array('id' => $id), 'utype' => 'admin'));
        }

        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg'], compact('id'));
        } else {
            $this->render_json(1, $return['msg'], compact('id'));
        }
    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（教育经历处理）
     */
    public function edu_action()
    {
        $eid = intval($_POST['eid']);

        $id = intval($_POST['id']);

        $uid = intval($_POST['uid']);

        $resumeM = $this->MODEL('resume');

        $post = array(
            'uid' => $uid,
            'eid' => $eid,
            'name' => $_POST['name'],
            'sdate' => strtotime($_POST['sdate']),
            'edate' => strtotime($_POST['edate']),
            'title' => $_POST['title'],
            'education' => $_POST['education'],
            'specialty' => $_POST['specialty']
        );

        if (!$id) {
            $return = $resumeM->addResumeEdu($post, array('utype' => 'admin'));
            $id = $return['id'];
        } else {
            $return = $resumeM->addResumeEdu($post, array('where' => array('id' => $id), 'utype' => 'admin'));
        }

        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg'], compact('id'));
        } else {
            $this->render_json(1, $return['msg'], compact('id'));
        }

    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（培训经历处理）
     */
    public function training_action(){
        $eid = intval($_POST['eid']);

        $id = intval($_POST['id']);

        $uid = intval($_POST['uid']);

        $resumeM = $this->MODEL('resume');

        $post = array(
            'uid' => $uid,
            'eid' => $eid,
            'name' => $_POST['name'],
            'sdate' => strtotime($_POST['sdate']),
            'edate' => strtotime($_POST['edate']),
            'title' => $_POST['title'],
            'content' => $_POST['content']
        );

        if (!$id) {
            $return = $resumeM->addResumeTrain($post, array('utype' => 'admin'));
            $id = $return['id'];
        } else {
            $return = $resumeM->addResumeTrain($post, array('where' => array('id' => $id), 'utype' => 'admin'));
        }

        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg'], compact('id'));
        } else {
            $this->render_json(1, $return['msg'], compact('id'));
        }

    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（职业技能处理）
     */
    public function skill_action(){
        $eid   =   intval($_POST['eid']);

        $id    =   intval($_POST['id']);

        $uid   =   intval($_POST['uid']);

        $resumeM   =   $this -> MODEL('resume');
        $post = array(
            'uid'		=>  $uid,
            'eid'		=>  $eid,
            'name'		=>  $_POST['name'],
            'ing'		=>  $_POST['ing'],
            'file'		=>  $_FILES['file'],
            'longtime'	=>	$_POST['longtime']
        );

        if(!$id){
            $return  =  $resumeM -> addResumeSkill($post,array('utype'=>'admin'));
            $id      =  $return['id'];
        }else{
            $return  =  $resumeM -> addResumeSkill($post,array('where'=>array('id'=>$id),'utype'=>'admin'));
        }

        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg'], compact('id'));
        } else {
            $this->render_json(1, $return['msg'], compact('id'));
        }
    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（项目经历处理）
     */
    public function project_action(){
        $eid = intval($_POST['eid']);

        $id = intval($_POST['id']);

        $uid = intval($_POST['uid']);

        $resumeM = $this->MODEL('resume');

        $post = array(
            'uid' => $uid,
            'eid' => $eid,
            'name' => $_POST['name'],
            'sdate' => strtotime($_POST['sdate']),
            'edate' => strtotime($_POST['edate']),
            'title' => $_POST['title'],
            'content' => $_POST['content']
        );

        if (!$id) {
            $return = $resumeM->addResumeProject($post, array('utype' => 'admin'));
            $id = $return['id'];
        } else {
            $return = $resumeM->addResumeProject($post, array('where' => array('id' => $id), 'utype' => 'admin'));
        }

        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg'], compact('id'));
        } else {
            $this->render_json(1, $return['msg'], compact('id'));
        }
    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（其他描述处理）
     */
    public function other_action(){
        $eid = intval($_POST['eid']);

        $id = intval($_POST['id']);

        $uid = intval($_POST['uid']);

        $resumeM = $this->MODEL('resume');

        $post = array(
            'uid' => $uid,
            'eid' => $eid,
            'name' => $_POST['name'],
            'content' => $_POST['content']
        );

        if (!$id) {
            $return = $resumeM->addResumeOther($post, array('utype' => 'admin'));
            $id = $return['id'];
        } else {
            $return = $resumeM->addResumeOther($post, array('where' => array('id' => $id), 'utype' => 'admin'));
        }

        if ($return['errcode'] == 9) {
            $this->render_json(0, $return['msg'], compact('id'));
        } else {
            $this->render_json(1, $return['msg'], compact('id'));
        }

    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（简历附表详情）
     */
    function getResumeFb_action(){

        $resumeM   =   $this -> MODEL('resume');

        $id    =   intval($_POST['id']);

        $table =   'resume_'.$_POST['type'];

        $info  =   $resumeM -> getFb($table,$id);

        echo json_encode($info);die;
    }
    /**
     * TODO 会员-个人-简历管理: 修改简历页面（简历详情页附表删除）
     */
    function delResumeFb_action(){
        $table = trim($_POST['table']);

        $tables = array('skill', 'work', 'project', 'edu', 'training', 'other');

        if (in_array($table, $tables)) {
            $id = (int)$_POST['id'];
            $eid = (int)$_POST['eid'];
            $uid = (int)$_POST['uid'];

            $resumeM = $this->MODEL('resume');

            $return = $resumeM->delFb($table, array('id' => $id, 'eid' => $eid, 'uid' => $uid), array('utype' => 'admin'));

            if (is_array($return)) {
                if ($return['errcode'] == 9) {
                    $this->render_json(0, $return['msg']);
                } else {
                    $this->render_json(1, $return['msg']);
                }
            } else {
                $this->render_json(0, '删除成功');
            }
        } else {
            $this->render_json(1, '系统异常');
        }
    }
    /**
     * 会员-个人-简历管理: 审核简历
     */
    function status_action(){
        $resumeM = $this->MODEL('resume');

        $post = array(
            'state' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody'])
        );

        if (!empty($_POST['content'])) {
            $data = array(
                'id' => $_POST['id'],
                'uid' => $_POST['uid'],
                'content' => $_POST['content'],
                'auid' => $_SESSION['auid']
            );
            $resumeM->label($data);
        }

        $return = $resumeM->statusResume($_POST['id'], array('post' => $post));

        if (isset($_POST['single'])) {
            if ($return['errcode'] == 9) {
                if ($_POST['atype'] == 1) {
                    // 仅保存
                    $this->admin_json(0, $return['msg']);
                } else {
                    // 下一个待审核简历
                    $resumeM = $this->MODEL('resume');
                    $row = $resumeM->getExpect(array('state' => 0,'r_status'=>array('<>',2), 'orderby' => array('lastupdate,DESC')), array('field' => 'id'));
                    if (!empty($row)) {
                        $this->admin_json(0, $return['msg'], array('next_id' => $row['id']));
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
    /**
     * TODO 会员-个人-简历管理: 同步审核用户
     */
    function resumestatus_action()
    {
        if (empty($_POST['id']) || empty($_POST['uid']) || empty($_POST['status'])) {
            $this->render_json(1, '参数错误');
        }

        $id = intval($_POST['id']);
        $uid = intval($_POST['uid']);
        $status = intval($_POST['status']);
        $statusbody = trim($_POST['statusbody']);

        $resumeM = $this->MODEL('resume');

        $post = array(
            'uid' => $uid,
            'lock_status' => !empty($_POST['lock_status']) ? $_POST['lock_status'] : 0,
            'state' => $status,
            'statusbody' => $statusbody
        );

        if (!empty($_POST['content'])) {
            $data = array(
                'id' => $_POST['id'],
                'uid' => $_POST['uid'],
                'content' => $_POST['content'],
                'auid' => $_SESSION['auid']
            );
            $resumeM->label($data);
        }

        $return = $resumeM->status($id, $post);

        if (isset($_POST['single'])) {
            if ($return['errcode'] == 9) {
                if ($_POST['atype'] == 1) {
                    // 仅保存
                    $this->admin_json(0, $return['msg']);
                } else {
                    // 下一个待审核简历
                    $resumeM = $this->MODEL('resume');
                    $row = $resumeM->getExpect(array('state' => 0,'r_status'=>array('<>',2), 'orderby' => array('lastupdate,DESC')), array('field' => 'id'));
                    if (!empty($row)) {
                        $this->admin_json(0, $return['msg'], array('next_id' => $row['id']));
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
    /**
     * 会员-个人-简历管理: 刷新简历
     */
    function refresh_action()
    {
        if (empty($_POST['id']) && empty($_POST['ids'])) {
            $this->render_json(1, '参数错误');
        }

        if ($_POST['id']) {
            $id = intval($_POST['id']);
        } elseif ($_POST['ids']) {
            $id = $_POST['ids'];
        }
        $resumeM = $this->MODEL('resume');

        $return = $resumeM->refreshResume($id);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * 会员-个人-简历管理: 推荐简历
     */
    function rec_action()
    {
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $id = is_array($_POST['id']) ? $_POST['id'] : intval($_POST['id']);

        $rec = $_POST['rec'];
        $resumeM = $this->MODEL('resume');

        $return = $resumeM->recResume($id, $rec);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * 会员-个人-简历管理: 置顶简历
     */
    function top_action(){
        if (empty($_POST['id'])) {
            $this->render_json(1, '参数错误');
        }

        $id = $_POST['id'];

        if ($_POST['s']) {
            $post = array(
                'top' => 0,
                'topdate' => 0
            );
        } else {
            $post = array(
                'top' => 1,
                'topdate' => strtotime('+' . intval($_POST['addday']) . ' day')
            );
        }
        $resumeM = $this->MODEL('resume');

        $return = $resumeM->topResume($id, $post);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * 会员-个人-简历管理: 简历状态
     */
    function cstatus_action()
    {
        if (empty($_POST['uid']) || empty($_POST['status'])) {
            $this->render_json(1, '参数错误');
        }

        $uid = (int)$_POST['uid'];

        $post = array(
            'status' => (int)$_POST['status']
        );

        $resumeM = $this->MODEL('resume');

        $return = $resumeM->cstatus($uid, $post);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * 会员-个人-简历管理: 跟进/备注
     */
    function label_action()
    {
        $resumeM = $this->MODEL('resume');
        $data = array(
            'id' => $_POST['id'],
            'uid' => $_POST['uid'],
            'label' => $_POST['label'],
            'content' => $_POST['content'],
            'auid' => $_SESSION['auid']
        );
        $return = $resumeM->label($data);

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * 会员-个人-简历管理: 删除简历
     */
    function delResume_action(){
        if (empty($_POST['del'])) {
            $this->render_json(1, '参数错误');
        }

        $resumeM = $this->MODEL('resume');

        $return = $resumeM->delResume($_POST['del'], array('utype' => 'admin', 'delAccount' => $_POST['delAccount']));

        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
    /**
     * 会员-个人-简历管理: 检测用户名重复性
     */
    function checkUsername_action(){
        $userinfoM	=	$this -> MODEL('userinfo');

        $result     =   $userinfoM -> addMemberCheck(array('username'=>trim($_POST['username'])));

        if (!empty($result['error'])) {
            $this->render_json(1, $result['msg']); // 异常
        } else {
            $this->render_json(0); // 成功
        }
    }
    /**
     * TODO 会员-个人-简历管理: 数据统计查询
     */
    function resumeNum_action(){

        $MsgNum = $this -> MODEL('msgNum');

        echo $MsgNum -> resumeNum();
    }

    // 导出字段
    private function getFields()
    {
        // rtype 开头 简历字段 type 开头 个人信息字段
        $fieldsList = [
            'rtype_id' => '简历ID',
            'rtype_name' => '简历名称',
            'rtype_uid' => '用户ID',
            'rtype_uname' => '姓名',
            'rtype_sex' => '性别',
            'rtype_birthday' => '生日',
            'type_marriage' => '婚姻',
            'type_height' => '身高',
            'type_nationality' => '民族',
            'type_weight' => '体重',
            'type_idcard' => '身份证',
            'type_telphone' => '手机',
            'type_telhome' => '座机',
            'type_email' => '邮件',
            'rtype_edu' => '教育程度',
            'type_homepage' => '个人主页',
            'type_address' => '详细地址',
            'rtype_exp' => '工作经验',
            'type_domicile' => '户籍',
            'type_living' => '现居住地',
            'type_description' => '个人说明',
            'rtype_hy' => '意向行业',
            'rtype_job_classid' => '意向职位',
            'rtype_city_classid' => '城市',
            'rtype_minsalary,maxsalary' => '薪水',
            'rtype_type' => '工作性质',
            'rtype_report' => '到岗时间',
            'rtype_lastdate' => '更新时间'
        ];

        return $fieldsList;
    }

    /**
     * 导出数据检测
     */
    function export_check_action(){

        if (empty($_POST['type']) || !is_array($_POST['type'])) {
            $this->render_json(1, '参数错误');
        }

        $city_job_class = '';
        $where = '1';
        $sessionData = $_SESSION['resumeXls'] ? $_SESSION['resumeXls'] : array();
        if($sessionData){
            $city_job_class = $sessionData['city_job_class'];
            $marriageSql = $sessionData['marriageSql'];
            $where = $sessionData['where'];
            $order = $sessionData['order'];
        }else{
            $order = 'order by id';
        }

        $typeList = $rtypeList = array();
        foreach ($_POST['type'] as $tval) {
            if (strpos($tval, 'rtype') === 0) {
                $rtypeList[] = str_replace('rtype_', '', $tval);
            } elseif (strpos($tval, 'type') === 0) {
                $typeList[] = str_replace('type_', '', $tval);
            }
        }

        if(!empty($rtypeList)){
            foreach($rtypeList as $v){
                if($v == 'lastdate'){
                    $rtype[]  =  'a.lastupdate';
                }else{
                    $rtype[]  =  'a.'.$v;
                }
            }
            $rfield  =  @implode(',',$rtype).',a.uid';
        }else{
            $rfield  =  'a.uid';
        }
        $limit = '';
        if($_POST['limit']){
            $limit = "limit " . intval($_POST['limit']);
        }
        if($_POST['section']){
            $num=explode(',',$_POST['section']);
            $startNum = $num[0]-1;
            $endNum = $num[1];
            $limit = "limit {$startNum},$endNum";
        }
        if($_POST['ids']){
            // 传id参数，直接按id查，不需要考虑其他条件
            $where = 'a.id in ('. pylode(',', $_POST['ids']) .')';
            $sql = "select {$rfield} from `".$this->def."resume_expect` a where {$where} {$order} {$limit}";
        }else{
            $sql = "select {$rfield} from `".$this->def."resume_expect` a{$city_job_class}{$marriageSql} where {$where} {$order} {$limit}";
        }
        $_SESSION['resumeXlsSql'] = $sql;
        $_SESSION['resumeXlsPostRTypes'] = $rtypeList;
        $resumeM  =  $this->MODEL('resume');
        $resumeNum = $resumeM->getExpectNum($where);
        if (intval($resumeNum) > 0){
            if(!empty($typeList)){
                if(in_array('uid',$typeList)){
                    $field  =  @implode(',',$typeList);
                }else{
                    $field  =  @implode(',',$typeList).',uid';
                }
            } else {
                $field = 'uid';
            }
            $_SESSION['resumeXlsFields'] = $field;
            $_SESSION['resumeXlsPostTypes'] = $typeList;

            $this->render_json(0);
        } else {
            $this->render_json(1, '暂无符合条件的简历数据');
        }
    }

    /**
     * 会员-个人-简历管理: 导出简历
     */
    function xls_action(){
        $sql = $_SESSION['resumeXlsSql'];

        $resumeM  =  $this -> MODEL('resume');
        $listNew = $resumeM->getList(array(), array('cache' => 1, 'utype' => 'admin', 'sql' => $sql));
        $expects  =  $listNew['list'];
        $rTypes = $_SESSION['resumeXlsPostRTypes'];
        if (!empty($expects)){
            $types = $_SESSION['resumeXlsPostTypes'];
            if(!empty($types)){
                foreach($expects as $v){
                    $uids[]  =  $v['uid'];
                }
                $field = $_SESSION['resumeXlsFields'];
                $resume = $resumeM->getResumeList(array('uid'=>array('in',pylode(',', $uids))),array('field'=>$field));
            }
            foreach ($expects as $k=>$v){
                if(is_array($resume)){
                    foreach($resume as $val){
                        if($v['uid']==$val['uid']){
                            $expects[$k]['reusme'] = $val;
                            $expects[$k]['marriage_n'] = $listNew['cache']['userclass_name'][$val['marriage']];
                        }
                    }
                }
            }

            $fieldsList = $this->getFields();
            $fields = array_keys($fieldsList);

            foreach ($_POST['type'] as $fval) {
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

            if ($expects) {
                foreach ($expects as $key => $val) {
                    $col = 'A';
                    // 循环字段
                    foreach ($type as $tval) {
                        if (strpos($tval, 'rtype') === 0) { // 简历信息
                            $typeNew = str_replace('rtype_', '', $tval);
                            $text = $val[$typeNew];
                            if (in_array($typeNew, array('sex', 'edu', 'exp', 'hy', 'lastupdate', 'type', 'report'))) { // 需要取格式化的字段
                                $text = $val[$typeNew . '_n'];
                            } elseif ($typeNew == 'job_classid') {
                                $text = $val['job_classname'];
                            } elseif ($typeNew == 'city_classid') {
                                $text = $val['city_classname'];
                            } elseif ($typeNew == 'minsalary,maxsalary') {
                                $text = $val['salary'];
                            } elseif ($typeNew == 'lastdate') {
                                $text = $val['lastupdate_n'];
                            }
                        } elseif (strpos($tval, 'type') === 0) { // 个人信息
                            $typeNew = str_replace('type_', '', $tval);
                            if ($typeNew == 'marriage') {
                                $text = $val[$typeNew . '_n'];
                            } else {
                                $text = $val['reusme'][$typeNew];
                            }
                        }

                        if (in_array($typeNew, array('idcard', 'telphone'))) { // 数字转字符的字段
                            $objPHPExcel->getActiveSheet()->setCellValueExplicit($col . ($key + 2), $text, PHPExcel_Cell_DataType::TYPE_STRING); // 转类型
                        } else {
                            $objPHPExcel->getActiveSheet()->setCellValue($col . ($key + 2), $text); // 动态表格内容
                        }
                        $col++;
                    }
                }
            }

            $objWriter = PHPExcel_IOFactory::createWriter($objPHPExcel, 'Excel2007');

            ob_start();
            $objWriter->save('php://output');
            $xlsData = ob_get_contents();
            ob_end_clean();
            $data = [
                'file' => base64_encode($xlsData),
                'file_name' => '简历信息' . date('YmdHis') . '.xlsx'
            ];
            return $this->admin_json(0, "导出简历信息", $data);
        } else {
            $this->render_json(1, '暂无符合条件的简历数据');
        }
    }

    // 获取编辑需要的缓存
    public function getCache_action()
    {
        $cacheData = $this->MODEL('cache')->GetCache(array('user'));

        $userdata = $cacheData['userdata'];
        $userclass_name = $cacheData['userclass_name'];

        $this->render_json(0, 'ok', compact('userdata', 'userclass_name'));
    }

    // 简历单个审核，带简历详情
    function resumeAudit_action(){
        $resumeM = $this->MODEL('resume');
        $info = $resumeM->getInfoByEid(array('eid' => $_POST['id']));

        $cacheM = $this->MODEL('cache');
        $cacheList = $cacheM->GetCache('user');
        $this->yunset($cacheList);

        //简历数据
        $return = $resumeM->getInfo(array('uid' => $info['uid'], 'eid' => $_POST['id'], 'tb' => 'all', 'needCache' => 1));
        $expectData = array(
            'uid' => $info['uid'],
            'expect' => $return['expect'],
            'edu' => $return['edu'],
            'other' => $return['other'],
            'project' => $return['project'],
            'skill' => $return['skill'],
            'training' => $return['training'],
            'work' => $return['work'],
            'salary' => $resumeM->salary()
        );

        $where['uid'] = $info['uid'];

        $resume = $resumeM->getResumeInfo($where, array('logo' => 2));
        //简历数据end

        $user_sex = $return['cache']['user_sex'];
        $userdata = $return['cache']['userdata'];
        $userclass_name = $return['cache']['userclass_name'];
        $industry_index = $return['cache']['industry_index'];
        $industry_name = $return['cache']['industry_name'];

        // 待审核数量
        $snum = $resumeM->getExpectNum(array('state' => 0, 'id' => array('<>', $_POST['id'])));

        $this->render_json(0, 'ok', compact('info', 'resume', 'expectData', 'user_sex', 'userdata', 'userclass_name', 'industry_index', 'industry_name', 'snum'));
    }

//region 简历预览
    function resumePreview_action()
    {
        $resumeM = $this->MODEL('resume');

        if (!empty($_POST['id'])) {// 简历ID
            $Info = $resumeM->getInfoByEid(array('eid' => $_POST['id']));
        } elseif (!empty($_POST['uid'])) {// 用户ID
            $where['uid'] = $_POST['uid'];
            $resumeinfo = $resumeM->getResumeInfo($where, array('logo' => 2));
            $Info = $resumeM->getInfoByEid(array('eid' => $resumeinfo['def_job']));
        }
        $result = array();
        $result['Info'] = $Info;


        $cacheM = $this->MODEL('cache');
        $cacheList = $cacheM->GetCache('user');
        $result['cacheList'] = $cacheList;

        //简历数据
        $return = $resumeM->getInfo(array('uid' => $Info['uid'], 'eid' => $Info['id'], 'tb' => 'all', 'needCache' => 1));
        $setarr = array(
            'expect' => $return['expect'],//求职意向
            'edu' => $return['edu'],
            'other' => $return['other'],
            'project' => $return['project'],
            'skill' => $return['skill'],
            'training' => $return['training'],
            'work' => $return['work'],
            'industry_index' => $return['cache']['industry_index'],
            'industry_name' => $return['cache']['industry_name'],
            'userdata' => $return['cache']['userdata'],
            'userclass_name' => $return['cache']['userclass_name'],
            'user_sex' => $return['cache']['user_sex'],
        );

        if (empty($return['cache']['city_type'])) {
            $result['cionly'] = 1;
        }
        if (empty($return['cache']['job_type'])) {
            $result['jionly'] = 1;
        }

        if (empty($_POST['uid'])) {
            $where['uid'] = $Info['uid'];
            $resumeinfo = $resumeM->getResumeInfo($where, array('logo' => 2));
        }
        if ($_POST['fromc'] == 'matching') {
            $result['matching'] = $_POST['fromc'];
        }
        $result = array_merge($result, $setarr);
        $result['resumeinfo'] = $resumeinfo;
        //简历数据end

        $this->render_json(0, '', $result);
    }
//endregion
}