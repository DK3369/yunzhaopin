<?php
/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

/**
 * 日志操作类
 */
class log_model extends model{

    public $opera = array(
        1 => '职位',
        2 => '简历',
        3 => '下载简历', // 下载简历（下载，删除）
        4 => '邀请面试', // 邀请面试（邀请，删除）
        5 => '收藏/关注', // 收藏/关注（收藏/关注，删除/取消）
        6 => '申请/报名/委托', // 申请/报名/委托
        7 => '基本信息',
        8 => '修改密码',
        9 => '兼职',
        10 => '猎头',

        11 => '修改账号',
        12 => '账号认证', // 账号认证（解绑）
        13 => '账号认证', // 账号认证（老版本）
        14 => '招聘会/专题',
        15 => '地图设置',
        16 => '图片操作', // 图片（横幅，环境，作品）
        17 => '积分兑换',
        18 => '系统信息',
        19 => '问答',
        20 => '讲师',

        21 => '课程',
        22 => '新闻',
        23 => '举报',
        24 => '优惠券',
        25 => '悬赏推荐',
        26 => '浏览/屏蔽',
        27 => '子账号增删 ',
        28 => '搜索器',
        29 => '项目任务',
        30 => '购买聊天',

        31 => '注销账号申请',
        32 => '账号登录' // 账号登录（新）
    );

    /**
     * 添加管理员日志
     * @param string $content
     * @param string $opera :
     * @param string $type
     * @param string $opera_id
     * @param array $adminlogin
     */
    public function addAdminLog($content, $opera = '', $type = '', $opera_id = '', $adminlogin = array())
    {

        if (((($_SESSION['auid'] && $_SESSION['ausername']) || ($_SESSION['wuid'] && $_SESSION['wusername'])) && $content) || !empty($adminlogin)) {

            if (!empty($adminlogin)) {

                $uid        =   $adminlogin['auid'];
                $username   =   $adminlogin['ausername'];
            } else {

                $uid        =   $_SESSION['auid'] ? $_SESSION['auid'] : $_SESSION['wuid'];
                $username   =   $_SESSION['ausername'] ? $_SESSION['ausername'] : $_SESSION['wusername'];
            }

            $data   =   array(
                'uid'       =>  $uid,
                'username'  =>  $username,
                'content'   =>  $content,
                'ctime'     =>  time(),
                'ip'        =>  fun_ip_get(),
                'opera'     =>  $opera,
                'type'      =>  $type,
                'opera_id'  =>  $opera_id,
                'did'       =>  $this->config['did']
            );

            $this->insert_into('admin_log', $data);
        }
    }

    /**
     * 查询管理员日志
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getAdminLog($whereData = array(), $data = array())
    {

        $log    =   $this->select_once('admin_log', $whereData);

        return $log;
    }

    /**
     * 查询管理员日志
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getAdminLogList($whereData = array(), $data = array())
    {

        $field  =   empty($data['field']) ? '*' : $data['field'];

        $List   =   $this->select_all('admin_log', $whereData, $field);

        return $List;
    }

    /**
     * 查询管理员日志数目
     * @param array $Where
     * @return array|bool|false|string|void
     */
    function getAdminLogNum($Where = array())
    {
        return $this->select_num('admin_log', $Where);
    }

    /**
     * 添加会员日志
     * @param string $uid
     * @param string $usertype
     * @param string $content
     * @param string $opera
     * 1-职位，2-简历，3-下载简历（下载，删除），4-邀请面试（邀请，删除），5-收藏/关注（收藏/关注，删除/取消），6-申请/报名/委托，7-基本信息，8-修改密码，9-兼职，10-猎头
     *
     * 11-修改账号，12-账号认证（解绑），13-账号认证（老版本），14-招聘会/专题，15-地图设置，16-图片（横幅，环境，作品），17-积分兑换，18-系统信息，19-问答，20-讲师
     *
     * 21-课程，22-新闻，23-举报，24-优惠券，25-悬赏推荐，26-浏览/屏蔽，27-子账号增删 ,28-搜索器，29-项目任务，30-购买聊天
     *
     * 31-注销账号申请，32-账号登录（新）
     *
     * @param string $type :1-增加，2-修改，3-删除, 4-刷新
     *
     * @param string $detail | 日志明细
     */
    public function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {

        if ($uid && $usertype && $content) {

            $data   =   array(
                'uid'       =>  $uid,
                'usertype'  =>  $usertype,
                'content'   =>  $content,
                'opera'     =>  $opera,
                'type'      =>  $type,
                'ip'        =>  fun_ip_get(),
                'ctime'     =>  time()
            );
            $member         =   $this->select_once('member', array('uid' => $uid), '`did`');
            $data['did']    =   $member['did'];

            $nid    =   $this->insert_into('member_log', $data);

            if (!empty($detail)) {

                $this->insert_into('member_log_detail', array('log_id' => $nid, 'detail' => $detail));
            }
        }
    }

    /**
     * 查询会员日志数目
     * @param array $Where
     * @return array|bool|false|string|void
     */
    function getMemberLogNum($Where = array())
    {
        return $this->select_num('member_log', $Where);
    }

    /**
     * 获取member_log 列表
     *
     * @param $whereData    查询条件
     * @param array $data   自定义处理数组
     * @return array|bool|false|string|void
     */
    public function getMemlogList($whereData, $data = array())
    {

        $data['field']  =   empty($data['field']) ? '*' : $data['field'];
        $List           =   $this->select_all('member_log', $whereData, $data['field']);

        if ($data['utype'] == 'admin') {

            $List       =   $this->getDataList($List);
        }
        return $List;
    }

    /**
     * @desc 删除member_log日志 $delId 日志id
     *
     * @param array $whereData
     * @param array $data
     * @return mixed
     */
    public function delMemlog($whereData = array(), $data = array())
    {

        $return['layertype']    =   0;

        if (!empty($whereData)) {

            if (!empty($whereData['id']) && $whereData['id'][0] == 'in') {
                $return['layertype']    =   1;
            }

            if ($data['norecycle'] == '1') {    //	数据库清理，不插入回收站

                $return['id']   =   $this->delete_all('member_log', $whereData, $data['limit'], '', '1');
            } else {

                $return['id']   =   $this->delete_all('member_log', $whereData, '');
            }

            $return['msg']      =   '会员日志';
            $return['errcode']  =   $return['id'] ? '9' : '8';
            $return['msg']      =   $return['id'] ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';
        } else {

            $return['msg']      =   '请选择您要删除的会员日志！';
            $return['errcode']  =   8;
        }

        return $return;
    }

    /**
     * @desc 获取login_log 列表
     *
     * @param $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getLoginlogList($whereData, $data = array())
    {

        $data['field']  =   empty($data['field']) ? '*' : $data['field'];
        $List           =   $this->select_all('login_log', $whereData, $data['field']);

        if ($data['utype'] == 'admin') {

            $List   =   $this->getDataList($List);
        }
        return $List;
    }

    public function getLoginlogNum($Where){
        return $this->select_num('login_log', $Where);
    }

    /**
     * @desc 添加login_log
     * @param $addData
     * @param array $data
     * @return bool|int
     */
    public function addLoginlog($addData, $data = array())
    {

        if (!empty($addData)) {

            $addData['ip']          =   fun_ip_get();
            $addData['ctime']       =   time();
            $addData['remoteport']  =   $_SERVER['REMOTE_PORT'];

            if (empty($addData['content'])) {

                $addData['content'] =   $this->LoginType($data);
            }
            if ($addData['usertype'] == 1){
                $this->joinZphnetUser(array('uid'=>$addData['uid'],'usertype'=>1));
                $this->update_once('resume', array('login_date' => time()), array('uid' => $addData['uid']));
            }else if ($addData['usertype'] == 2){

                $this->update_once('company', array('login_date' => time()), array('uid' => $addData['uid']));
            }

            if (isset($data['continue']) && $data['continue'] == 1) {
                include_once('integral.model.php');
                $integralM  =   new integral_model($this->db, $this->def);
                $integralM->invtalCheck($addData['uid'], $addData['usertype'], 'integral_login', '会员登录', 22);
            }

            return  $this->insert_into('login_log', $addData);
        }
	}

    /**
     * 求职者登录自动参会
     * @param array $data
     */
    private function joinZphnetUser($data = array())
    {

        if ($data['uid']) {

            // 个人用户 能在列表正常展示的简历，才统计
            $expect =   $this->select_once('resume_expect', array('uid' => $data['uid'], 'defaults' => 1, 'state' => '1', 'status' => '1', 'r_status' => '1'), '`id`');
            if (empty($expect)) {
                return;
            }
            $zid    =   $zphlist    =   array();
            $time   =   time();
            $wherezph['is_open']    =   1;
            $wherezph['endtime']    =   array('unixtime', '>', $time);
            $list   =   $this->select_all('zphnet', $wherezph, 'id,endtime');

            if ($list) {
                foreach ($list as $k => $v) {
                    $zid[]  =   $v['id'];
                }
                if (!empty($zid)) {

                    $rows   =   $this->select_all('zphnet_user', array('zid' => array('in', pylode(',', $zid)), 'uid' => $data['uid'], 'usertype' => $data['usertype']));
                    if (!empty($rows)) {
                        foreach ($list as $k => $v) {
                            foreach ($rows as $ke => $va) {
                                if (!in_array($va['zid'], $zid)) {

                                    $zphlist[]  =   $v;
                                }
                            }
                        }
                    } else {

                        $zphlist    =   $list;
                    }
                    if (!empty($zphlist)) {
                        foreach ($zphlist as $k => $v) {
                            if (strtotime($v['endtime']) >= $time) {
                                $data['ctime']  =   $time;
                                $data['zid']    =   $v['id'];
                                $this->insert_into('zphnet_user', $data);
                            }
                        }
                    }
                }
            }
        }
    }

    /**
     * 处理登录日志中，登录来源
     * @param array $data
     * @return string
     */
	public function LoginType($data = array())
	{

	    $content    =   '';

	    if (isset($data['provider'])){
	        if ($data['provider'] == 'app'){
	            if (isset($data['type'])){

	                if ($data['type'] == 'weixin'){

	                    $content    =   'app内微信登录成功';
	                }elseif ($data['type'] == 'qq'){

	                    $content    =   'app内QQ登录成功';
	                }elseif ($data['type'] == 'qq'){

	                    $content    =   'app内新浪微博登录成功';
	                }
	            }else{

	                $content        =   'app登录成功';
	            }
	        }elseif ($data['provider'] == 'weixin'){

	            $content    =   '微信小程序登录成功';
	        }elseif ($data['provider'] == 'baidu'){

	            $content    =   '百度小程序登录成功';
	        }elseif ($data['provider'] == 'sinaweibo'){

	            $content    =   '新浪微博登录成功';
	        }elseif ($data['provider'] == 'toutiao'){

	            $content    =   '抖音小程序登录成功';
	        }
	    }elseif ((isset($data['source']) && $data['source'] == 2) || isset($data['wap'])){

	        $content    =   'WAP登录成功';
	    }else{

	        $content    =   'PC登录成功';
	    }
	    return $content;
	}

	public function getLoginLog($where = array())
	{
	    $return  =  $this -> select_once('login_log', $where);
	    return  $return;
	}

    /**
     * @desc 删除login_log日志
     * @param $delId    日志id
     * @param $where
     * @return mixed
     */
    public function delLoginlog($delId,$where)
    {
		$return['layertype']	=	0;

		if(!empty($delId)){
			if(is_array($delId)){
				$delId					=	pylode(',', $delId);

				$return['layertype']	=	1;
			}
			
			$return['id']				=	$this -> delete_all("login_log", array('id' => array('in',$delId)),"");
			
			$return['msg']				=	'会员登录日志(ID:'.pylode(',', $delId).')';
			$return['errcode']			=	$return['id'] ? '9' :'8';
			$return['msg']				=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
		}elseif($where){
			$return['id']				=	$this -> delete_all("login_log", $where, "");
			
			$return['msg']				=	'会员登录日志';
			$return['errcode']			=	$return['id'] ? '9' :'8';
			$return['msg']				=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
		}else{
			$return['msg']				=	'请选择您要删除的会员登录日志！';
			$return['errcode']			=	8;
		}
		return	$return; 
	}

    /**
     * 会员日志后台列表数据处理
     * @param $List
     * @return mixed
     */
    private function getDataList($List)
    {

        $uids       =   array();
        $idArr      =   array();
        foreach($List as $v){
            $idArr[]=   $v['id'];

            if (!in_array($v['uid'], $uids)){
                $uids[] =   $v['uid'];
            }
        }

        $member     =   $this->select_all('member', array('uid' => array('in', pylode(',', $uids))), '`uid`,`username`,`pid`');

        $subUidToUid=   array();
        foreach ($member as $v) {
            if ($v['pid'] > 0) {
                !in_array($v['pid'],$uids) && array_push($uids, $v['pid']);
                $subUidToUid[$v['uid']] = $v['pid'];
            }
        }
        $resume     =   $this->select_all('resume', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`,`def_job`');
        $company    =   $this->select_all('company', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`');

        $ltinfo     =   $this->select_all('lt_info', array('uid' => array('in', pylode(',', $uids))), '`uid`,`realname`');
        $pxtrain    =   $this->select_all('px_train', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`');

        $gqinfo     =   $this->select_all('gq_info', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`');

        $logDetail  =   $this->select_all('member_log_detail', array('log_id' => array('in', pylode(',', $idArr))));

        foreach($List as $k => $v){

            $List[$k]['sub_n']  =   '';

            if (!empty($logDetail)){
                foreach ($logDetail as $lk => $lv) {
                    if ($lv['log_id'] == $v['id']){

                        $List[$k]['sub_n']  =   $lv['detail'];
                    }
                }
            }

            $List[$k]['pid']    =   0;
            foreach($member as $val){
                if($val['uid']==$v['uid']){

                    $List[$k]['username']  =  $val['username'];
                    $List[$k]['pid'] = $val['pid'];
                }
            }
            foreach($resume as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['rname']  =  $val['name'];
                    $List[$k]['eid']    =  $val['def_job'];
                }
            }
            foreach($company as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['comname']  =  $val['name'];
                }else if (isset($subUidToUid[$v['uid']]) && $subUidToUid[$v['uid']] == $val['uid']) {// 子账户登陆uid转主账户uid
                    $List[$k]['comname']  =  $val['name'] . ' 企业(子账号)';
                }
            }
            foreach($gqinfo as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['gqname']  =  $val['name'];
                }
            }
            foreach($ltinfo as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['ltname']  =  $val['realname'];
                }
            }
            foreach($pxtrain as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['pxname']  =  $val['name'];
                }
            }

			!empty($v['ctime']) && $List[$k]['ctime_n'] = date('Y-m-d H:i', $v['ctime']);
			$List[$k]['opera_n'] = $this->opera[$v['opera']];
            if ($v['usertype'] == 2) {
                if ($List[$k]['pid'] > 0) {
                    $List[$k]['comp_url'] = Url('company', array('c' => 'show', 'id' => $List[$k]['pid'], 'look' => 'admin'));
                }
                $List[$k]['com_url'] = Url('company', array('c' => 'show', 'id' => $List[$k]['uid'], 'look' => 'admin'));
            }
	    }
        return $List;
    }

    /**
     * @desc
     * @param array $data
     */
    public function addJobSxLogS($data = array())
    {
        if (!empty($data)) {

            $this->DB_insert_multi('job_refresh_log', $data);
        }
    }

    /**
     * @desc 职位刷新日志
     * @param array $data
     * @param int $free | 1-免费刷新；2-非免费刷新
     * @param int $free_num
     */
    public function addJobSxLog($data = array(), $free = 2, $free_num = 0)
    {

        $vData   =   array(

            'uid'       =>  $data['uid'],
            'usertype'  =>  $data['usertype'],
            'jobid'     =>  $data['jobid'],
            'type'      =>  $data['type'],
            'ip'        =>  fun_ip_get(),
            'r_time'    =>  isset($data['r_time']) ? $data['r_time'] : time(),
            'port'      =>  isset($data['port']) ? $data['port'] : 1,
            'remark'    =>  $data['remark'],
            'free'      =>  $free,
            'free_num'  =>  $free_num
        );

        $this->insert_into('job_refresh_log', $vData);
    }

    /**
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getJobBySxLog($whereData = array(), $data = array())
    {

        $field      =   isset($data['field']) ? $data['field'] : '*';

        $tableName  =   'company_job';  //  默认查询全职数据

        if (isset($data['type'])){
            if ($data['type'] == 1){

                $tableName  =   'company_job';
            } elseif ($data['type'] == 2){

                $tableName  =   'partjob';
            } elseif ($data['type'] == 3){

                $tableName  =   'lt_job';
            }
        }
        $List    =   $this->select_all($tableName, $whereData, $field);

        return $List;
	}

    public function getSxJobLogNum($whereData = array())
    {
        return $this->select_num('job_refresh_log', $whereData);
    }

    public function getSxJobLogList($whereData = array(), $data = array())
    {

        $field  =   isset($data['field']) ? $data['field'] : '*';

        $List   =   $this->select_all('job_refresh_log', $whereData, $field);

        $jobIds =   array();
        foreach ($List as $k => $v) {

            if (!in_array($v['jobid'], $jobIds)){

                $jobIds[]   =   $v['jobid'];
            }
        }

        $tableName      =   'company_job';
        $jobField       =   'id,name,com_name';

        if ($whereData['type'] == '1'){

            $tableName  =   'company_job';
            $jobField   =   'id,name,com_name';
        } elseif ($whereData['type'] == '2'){

            $tableName  =   'partjob';
            $jobField   =   'id,name,com_name';
        } elseif ($whereData['type'] == '3'){

            $tableName  =   'lt_job';
            $jobField   =   'id,job_name as name,com_name, usertype';
        }

        $jobList        =   $this->select_all($tableName, array('id' => array('in', pylode(',', $jobIds))), $jobField);

        include_once CONFIG_PATH.'db.data.php';
        $portArr        =   $arr_data['port'];
        foreach ($List as $k => $v) {

            if ($whereData['type'] == '1') {

                $List[$k]['joburl']     =  Url('job', array('c' => 'comapply', 'id' => $v['jobid'], 'look' => 'admin'));
            } elseif ($whereData['type'] == '2') {

                $List[$k]['joburl']     =  Url('part', array('c' => 'show', 'id' => $v['jobid'], 'look' => 'admin'));
            }

            $List[$k]['port_n']     =   $portArr[$v['port']];
            if (in_array($whereData['type'], array(1, 2))) {
                $comurl_arr = array('c' => 'show', 'id' => $v['uid']);
                if ($data['utype'] == 'admin') {
                    $comurl_arr['look'] = 'admin';
                }
                $List[$k]['comurl'] = Url('company', $comurl_arr);
            }

            $List[$k]['r_time_n']   =   date('Y-m-d H:i', $v['r_time']);

            if ($v['type'] == 1){

                $List[$k]['type_n'] =   '普通职位';
            } elseif ($v['type'] == 2){

                $List[$k]['type_n'] =   '兼职';
            } elseif ($v['type'] == 3){

                $List[$k]['type_n'] =   '高级职位';
            }

            foreach ($jobList as $jk => $jv) {

                if ($v['jobid'] == $jv['id']){

                    $List[$k]['job_name']   =   $jv['name'];
                    $List[$k]['com_name']   =   $jv['com_name'];
                }
            }
        }

        return $List;
    }

    /**
     * @desc 删除刷新职位日志
     * @param $del
     * @param array $data
     * @return mixed
     */
    public function delSxJobLog($del, $data = array())
    {

        $return['layertype'] = 0;

        if (is_array($del)) {

            $where['id']    =   array('in', pylode(',', $del));
            $return['layertype'] = 1;
        } else {

            $where['id']    =   $del;
        }

        $result             =   $this->delete_all('job_refresh_log', $where, '');

        $return['msg']      =   '职位刷新日志';
        $return['errcode']  =   $result ? '9' : '8';
        $return['msg']      =   $result ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';
        return $return;
    }



    /**
     * @desc
     * @param array $data
     */
    public function addResumeSxLogS($data = array())
    {
        if (!empty($data)) {

            $this->DB_insert_multi('resume_refresh_log', $data);
        }
    }

    /**
     * @desc 简历刷新日志
     * @param array $data
     */
    public function addResumeSxLog($data = array())
    {

        $vData   =   array(
            'uid'       =>  $data['uid'],
            'resume_id' =>  $data['resume_id'],
            'r_time'    =>  isset($data['r_rime']) ? $data['r_rime'] : time(),
            'port'      =>  isset($data['port']) ? $data['port'] : 1,
            'ip'        =>  isset($data['ip']) ? $data['ip']  : fun_ip_get(),
        );
        $this->insert_into('resume_refresh_log', $vData);
    }

    /**
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getResumeBySxLog($whereData = array(), $data = array())
    {

        $field      =   isset($data['field']) ? $data['field'] : '*';

        $tableName  =   'resume_expect';

        $List    =   $this->select_all($tableName, $whereData, $field);

        return $List;
    }

    /**
     * 简历刷新日志 查询数量
     * @param $whereData
     * @return int
     */
    public function getSxResumeLogNum($whereData = array())
    {
        return intval($this->select_num('resume_refresh_log', $whereData));
    }

    /**
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getSxResumeLogList($whereData = array(), $data = array())
    {

        $field  =   isset($data['field']) ? $data['field'] : '*';

        $List   =   $this->select_all('resume_refresh_log', $whereData, $field);

        $resumeIds  =   array();
        foreach ($List as $k => $v) {

            if (!in_array($v['resume_id'], $resumeIds)) {

                $resumeIds[]    =   $v['resume_id'];
            }
        }

        $resumeList     =   $this->select_all('resume_expect', array('id' => array('in', pylode(',', $resumeIds))), 'id,name,uname,uid');

        include_once CONFIG_PATH.'db.data.php';
        $portArr        =   $arr_data['port'];
        foreach ($List as $k => $v) {

            $List[$k]['port_n']     =   $portArr[$v['port']];

            $List[$k]['r_time_n']   =   date('Y-m-d H:i', $v['r_time']);

            foreach ($resumeList as $rk => $rv) {

                if ($v['resume_id'] == $rv['id']){

                    $List[$k]['r_name'] =   $rv['name'];
                    $List[$k]['u_name'] =   $rv['uname'];
                }
            }
        }

        return $List;
    }

    /**
     * @desc 删除刷新简历日志
     * @param $del
     * @param array $data
     * @return mixed
     */
    public function delSxResumeLog($del, $data = array())
    {

        $return['layertype'] = 0;

        if (is_array($del)) {

            $where['id']    =   array('in', pylode(',', $del));
            $return['layertype'] = 1;
        } else {

            $where['id']    =   $del;
        }

        $result             =   $this->delete_all('resume_refresh_log', $where, '');

        $return['msg']      =   '简历刷新日志';
        $return['errcode']  =   $result ? '9' : '8';
        $return['msg']      =   $result ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';
        return $return;
    }
}

?>
