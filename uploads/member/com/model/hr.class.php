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

class hr_controller extends company
{

    function index_action()
    {
        $statis =   $this->public_action();
        $remarkM=	$this->MODEL('remark');
        $downM	=	$this->MODEL('downresume');
        $JobM   =   $this->MODEL('job');
        $ResumeM=   $this->MODEL('resume');
		// 查询所有收到的简历，用于统计数量
        $dclNum = $yckNum = $bhsNum = $wjtNum = $dtzNum = $yrzNum = 0;
        $where  =   array(
            'com_id'    =>  $this->uid,
            'type'      =>  array('<>', 3),
            'isdel'     =>  9
        );
        if ($_GET['jobid']) {
            $jobid      =   @explode('-', $_GET['jobid']);
            $jobid['1'] =   !array_key_exists('1', $jobid) ? 1 : $jobid['1'];
            $where['job_id']    =   $jobid['0'];
            $where['type']      =   $jobid['1'];
            $urlarr['jobid']    =   $_GET['jobid'];
        }
        // 查询简历eid
        if ($_GET['resumetype'] || $_GET['exp'] || $_GET['edu'] || $_GET['sex'] || $_GET['uptime'] || $_GET['keyword']) {
            $sqJob  =   $JobM->getSqJobList($where, array('field' => 'eid','id','utype'=>'simple'));
            if (!empty($sqJob) && is_array($sqJob)) {
                $sqEid                  =   array();
                foreach ($sqJob as $v) {
                    $sqEid[]            =   $v['eid'];
                }
            }
            if ($_GET['resumetype']) {
                $resumeType             =   intval($_GET['resumetype']);
                $rWhere['height_status']=   $resumeType == 2 ? 2 : 0;
                $urlarr['resumetype']   =   $resumeType;
            }
            if ($_GET['exp']) {
                $resumeExp              =   intval($_GET['exp']);
                $rWhere['exp']          =   $resumeExp;
                $urlarr['exp']          =   $resumeExp;
            }
            if ($_GET['edu']) {
                $resumeEdu              =   intval($_GET['edu']);
                $rWhere['edu']          =   $resumeEdu;
                $urlarr['edu']          =   $resumeEdu;
            }
            if ($_GET['sex']) {
                $resumeSex              =   intval($_GET['sex']);
                $rWhere['sex']          =   $resumeSex;
                $urlarr['sex']          =   $resumeSex;
            }
            if (isset($_GET['uptime'])&&$_GET['uptime']) {
                $resumeUptime           =   intval($_GET['uptime']);
                if ($resumeUptime == 1) {
                    $rWhere['lastupdate']   =   array('>', strtotime('today'));
                } else {
                    $rWhere['lastupdate']   =   array('>', strtotime('-' . $resumeUptime . ' day'));
                }
                $urlarr['uptime']           =   $resumeUptime;
            }
            if (!empty($rWhere)) {
                $rWhere['id']       =   array('in', pylode(',', $sqEid));
                $rWhere['r_status'] =   1;
                $resumeList            =   $ResumeM->getSimpleList($rWhere, array('field' => '`id`'));
                if (!empty($resumeList) && is_array($resumeList)) {
                    $reid       =   array();
                    foreach ($resumeList as $v) {
                        $reid[] =   $v['id'];
                    }
                }
                $where['eid']   =   array('in', pylode(',', $reid));
            }
        }
        if (trim($_GET['keyword'])) {
            $resume =   $ResumeM->getResumeList(array('name' => array('like', trim($_GET['keyword'])), 'r_status' => 1), array('field' => "`uid`"));
            if (!empty($resume) && is_array($resume)) {
                $uid        =   array();
                foreach ($resume as $v) {
                    $uid[]  =   $v['uid'];
                }
            }
            $where['uid']       =   array('in', pylode(',', $uid));
            $urlarr['keyword']  =   trim($_GET['keyword']);
        }
        if ($_GET['state']) {
            $where['is_browse'] =   intval($_GET['state']);
            $urlarr['state']    =   $_GET['state'];
        }
        if (isset($_GET['rstate']) && $_GET['rstate']!='') {
            $where['resume_state']  =   intval($_GET['rstate']);
            $urlarr['rstate']       =   $_GET['rstate'];
        }
        //分页链接
        $urlarr['c']    =   $_GET['c'];
        $urlarr['page'] =   '{{page}}';
        $pageurl        =   Url('member', $urlarr);
        //提取分页
        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->pageList('userid_job', $where, $pageurl, $_GET['page']);
        if ($pages['total'] > 0) {
            if ($_GET['order']) {
                $where['orderby']   =   $_GET['t'] . ',' . $_GET['order'];
                $urlarr['order']    =   $_GET['order'];
                $urlarr['t']        =   $_GET['t'];
            } else {
                $where['orderby']   =   array('datetime,desc', 'is_browse,asc');
            }
            $where['limit']         =   $pages['limit'];
            $rows   =   $JobM->getSqJobList($where, array('uid' => $this->uid, 'usertype' => $this->usertype, 'is_link' => 'yes','statis'=>$statis, 'hr' => 1)); // s_num 申请状态数目
        }
        $jobArr =   $JobM -> getList(array('uid' => $this->uid), array('field' => '`id`,`name`'));
        $job    =   $jobArr['list'];
        foreach ($job as $key => $val) {
            $job[$key]['type']  =   1;
        }
        $JobList=   $job;
        if ($JobList && is_array($JobList) && $jobid['0']) {
            foreach ($JobList as $val) {
                if ($jobid['0'] == $val['id'] && $jobid['1'] == $val['type']) {
                    $current    =   $val;
                }
            }
        }
        $allrows = $JobM->getSqJobList(array('com_id'=>$this->uid,'type'=>array('<>', 3), 'isdel' => 9), array('uid'=>$this->uid, 'usertype'=>$this->usertype,'utype'=>'simple'));
        foreach ($allrows as $v) {
            // 统计投递简历各种状态数量
            if ($v['is_browse'] == 1) {
                $dclNum++;
            } elseif ($v['is_browse'] == 2) {
                $yckNum++;
            }elseif ($v['is_browse'] == 3) {
                $dtzNum++;
            }elseif ($v['is_browse'] == 4) {
                $bhsNum++;
            } elseif ($v['is_browse'] == 5) {
                $wjtNum++;
            }elseif ($v['is_browse'] == 7) {
                $yrzNum++;
            }
        }
        $this->yunset(array('current' => $current, 'rows' => $rows, 'JobList' => $JobList, 'StateList' => array(array('id' => 1, 'name' => '未查看', 'num' => $dclNum), array('id' => 3, 'name' => '已面试', 'num' => $dtzNum), array('id' => 4, 'name' => '不合适', 'num' => $bhsNum), array('id' => 7, 'name' => '已入职', 'num' => $yrzNum))));
        $resumestate = array(
            array('name'=>'已审核','val'=>1),
            array('name'=>'未审核','val'=>0),
            array('name'=>'未通过','val'=>3)
        );
        $this->yunset('resumestate',$resumestate);
		//查询该会员套餐数量
		$ratingM = $this->MODEL('rating');
		$ratinginfo = $ratingM->getInfo(array('id'=>$statis['rating']),array('field'=>'`freelook_num`'));
		//查询该企业会员今天免费下载次数
		$downresumeM = $this->MODEL('downresume');
		$downnum = $downresumeM->getDownNum(array('comid'=>$this->uid,'downtime'=>array('>=',strtotime('today')),'isfree'=>2));
		//查询简历是否投递该企业
		if ($ratinginfo['freelook_num']>$downnum){
			$freenum = $ratinginfo['freelook_num'] - $downnum;
			$this->yunset('freenum',$freenum);
		}
		if (isVip($statis['vip_etime'])) {
            //查询该会员套餐数量
            $ratingM = $this->MODEL('rating');
            $ratinginfo = $ratingM->getInfo(array('id' => $statis['rating']), array('field' => '`freelook_num`'));
            if ($ratinginfo['freelook_num'] > 0) {
                //查询该企业会员今天免费查看投递简历数
                $downresumeM = $this->MODEL('downresume');
                $downnum = $downresumeM->getFreeDownNum(array('comid' => $this->uid, 'downtime' => array('>=', strtotime('today'))));
                if ($ratinginfo['freelook_num'] > $downnum) {
                    $freenum = $ratinginfo['freelook_num'] - $downnum;
                    $this->yunset('freenum', $freenum);
                }
            }
        }
        //获取状态栏选项
        $CacheM =   $this->MODEL('cache');
        $cache  =   $CacheM->GetCache(array('com','user', 'uptime'));
        $this->yunset($cache);
        //邀请面试选择职位
        $this->yqmsInfo();
        $this->com_tpl('hr');
    }

    //标记
    function hrset_action()
    {

        if ($_POST['ajax'] == 1 && $_POST['ids']) {

            //批量阅读
            $JobM   =   $this->MODEL('job');
            $arr    =   $JobM->ReadSqJob($_POST['ids'], array('uid' => $this->uid, 'usertype' => $this->usertype));
            echo json_encode($arr);die;
        } else if ($_POST['delid'] || $_GET['delid']) {

            //删除
            $JobM   =   $this->MODEL('job');
            if (is_array($_POST['delid'])) {

                $id =   $_POST['delid'];
            } else {

                $id =   intval($_GET['delid']);
            }
            $arr    =   $JobM->delSqJob($id, array('utype' => 'com', 'uid' => $this->uid, 'usertype' => $this->usertype));
            $this->layer_msg($arr['msg'], $arr['errcode'], $arr['layertype'], $_SERVER['HTTP_REFERER']);
        } else if ($_POST['browse']) {

            //简历状态设置
            $JobM   =   $this->MODEL('job');
            $id     =   intval($_POST['id']);
            $data   =   array('uid' => $this->uid, 'usertype' => $this->usertype, 'username' => $this->username, 'browse' => intval($_POST['browse']), 'port' => '1');
            $arr    =   $JobM->BrowseSqJob($id, $data);
            echo $arr;die;
        }
    }
    //备注
    function remark_action()
    {
        if (empty($_POST['remark'])){
            $this -> ACT_layer_msg('请填写备注内容', 8);
        }
        $RemarkM    =   $this->MODEL('remark');
        $data       =   array(
            'status'    =>  $_POST['status'],
            'remark'    =>  $_POST['remark'],
            'eid'       =>  $_POST['eid'],
            'uid'       =>  $_POST['uid'],
            'comid'     =>  $this->uid,
            'ctime'     =>  time()
        );
        $return     =   $RemarkM -> Remark($data);
        $this -> ACT_layer_msg($return['msg'],$return['errcode'],$_SERVER['HTTP_REFERER']);
    }

    //简历详情
    function resumeInfo_action()
    {
        if ($_GET['eid']) {
            $id = $_GET['eid'];
            $this->yunset("eid", $id);
            //获取状态栏选项,举报选项
            $CacheM =   $this->MODEL('cache');
            $cache  =   $CacheM->GetCache(array('com','user'));
            $this->yunset($cache);
            // 企业登录时，获取企业的职位列表

                $JobM = $this->MODEL('job');
                $uid = $this->uid;

                $jobWhere = array('uid' => $uid, 'state' => 1, 'r_status' => 1, 'status' => 0);

                $jobList = $JobM->getMsyqJobList($jobWhere, array('link' => 'yes', 'field' => '`com_name`, `name`, `id`, `is_link`, `link_id`, `uid`'));

                $company_job = array();

                if (!empty($jobList['list'])) {
                    $company_job = $jobList['list'];
                }
                $this->yunset('company_job', $company_job);

                //邀请模板
                $yqmbM = $this->MODEL('yqmb');
                $ymlist = $yqmbM->getList(array('uid' => $uid, 'status' => 1));
                $ymnum = $yqmbM->getNum(array('uid' => $uid));
                $ymcan = $ymnum < $this->config['com_yqmb_num'] ? true : false;

                $this->yunset('ymlist', $ymlist);
                $this->yunset('ymcan', $ymcan);

            $resumeM = $this->MODEL('resume');
            $resume_expect = $resumeM->getInfoByEid(array('eid' => $id, 'uid' => $this->uid, 'usertype' => $this->usertype));

            if (empty($resume_expect)) {
                $this->yunset(array('errmsg' => '没有找到该人才！'));
                $this->com_tpl('hr_resume');exit();
            }

            if ($this->uid != $resume_expect['uid']) {
                // 检查简历隐私状态设置
                $canShow = true;
                if ($resume_expect['status'] == 2){
                    // 简历关闭
                    $canShow = false;
                }elseif ($resume_expect['status'] == 3){
                    // 简历状态是投递企业可见
                    $canShow = false;
                    if (isset($resume_expect['userid_job'])){
                        // 已向企业投递简历，简历可以展示
                        $canShow = true;
                    }
                }
                if (!$canShow){
                    $this->yunset(array('errmsg' => '简历已设置不对外开放！'));
                    $this->com_tpl('hr_resume');exit();
                }
            }
            $jobid  =   $JobM->getSqJobInfo(array('eid'=>$id,'com_id'=>$this->uid),array('field' => 'job_id,job_name,type'));
            $jobs   =   $JobM->getSqJobInfo(array('eid'=>$id,'com_id'=>$this->uid),array('field'=>'id'));

            $this->yunset('job', $jobid);
            $this->yunset('Infos', $jobs);
            $this->yunset('Info', $resume_expect);
            // 已邀请面试数量

            if (!empty($resume_expect) && !empty($this->uid)) {
                $usermsgnum = $JobM->getYqmsNum(array('fid' => $this->uid, 'uid' => $resume_expect['uid'], 'isdel' => 9));
                $this->yunset('usermsgnum', $usermsgnum);
            }
            //是否查看过
            $cData['uid'] = $this->uid;
            $cData['usertype'] = $this->usertype;
            $cData['eid'] = $resume_expect['id'];
            $cData['ruid'] = $resume_expect['uid'];
            $resumeCkeck = $resumeM->openResumeCheck($cData);
            $this->yunset('resumeCkeck', $resumeCkeck);
            /* 模糊字段 */
            $this->yunset('tj', $resume_expect['tj']);

            $this->yunset(array('resumestyle' => $this->config['sy_weburl'] . '/app/template/resume'));
            $statisM = $this->MODEL('statis');

            if (in_array($this->usertype, array(2))) {

                $category = intval($this->usertype) - 1;

                // 会员等级 增值包 套餐
                $ratingM = $this->MODEL('rating');
				if (!empty($this->comInfo['package'])) {
                    $ratingList = $ratingM->getList(array('display' => 1,'id'=>array('in', $this->comInfo['package']), 'orderby' => array('type,asc', 'sort,desc')));
                }else{
                    $ratingList = $ratingM->getList(array('display' => 1, 'orderby' => array('type,asc', 'sort,desc')));
                }
                $rating_1 = $rating_2 = $raV = array();
                if (!empty($ratingList)) {
                    foreach ($ratingList as $ratingV) {

                        $raV[$ratingV['id']] = $ratingV;

                        if ($ratingV['category'] == $category && $ratingV['service_price'] > 0) {
                            if ($ratingV['type'] == 1) {

                                $rating_1[] = $ratingV;
                            } elseif ($ratingV['type'] == 2) {

                                $rating_2[] = $ratingV;
                            }
                        }
                    }
                }
				if (!empty($this->config['com_package_open']) && empty($this->comInfo['package'])){

					$packageOpenArr =   explode(',', $this->config['com_package_open']);
					if (in_array($this->comInfo['rating'], $packageOpenArr) || ($this->comInfo['vipetime'] > 0 && $this->comInfo['vipetime'] < time() && in_array('999', $packageOpenArr))){

						$rating_1   =   array();
						$rating_2   =   array();
					}
				}
				if (empty($rating_1) && empty($rating_2) && !empty($this->config['com_package_open'])){
					$this->yunset('noPermission', 1);
				}
				if ($this->comInfo['r_status'] == 4){
					$this->yunset('isPaused', 1);
				}
				if ((int)$this->comInfo['crm_uid'] > 0){

                    $crmInfo    =   $this->obj->select_once('admin_user', array('uid' => $this->comInfo['crm_uid']), '`name`,`moblie`,`ewm`');
                    $consultant =   array('name' => $crmInfo['name'], 'mobile' => $crmInfo['moblie'], 'wechat' => !empty($crmInfo['ewm']) ? checkpic($crmInfo['ewm']): '');
                    $this->yunset(array('consultant' => $consultant, 'crm' => 1));
                }else{
    
                    $this->yunset(array('crm' => 2, 'webTel' => $this->config['sy_comwebtel']));
                }
                $this->yunset('rating_1', $rating_1);
                $this->yunset('rating_2', $rating_2);

                $statis = $statisM->getInfo($this->uid, array('usertype' => $this->usertype));

                if (!empty($statis)) {

                    $discount = isset($raV[$statis['rating']]) ? $raV[$statis['rating']] : array();
                    $this->yunset('discount', $discount);
                    $this->yunset('statis', $statis);
                }

                if (isVip($statis['vip_etime'])) {
                    //查询该会员套餐数量
                    $ratinginfo = $ratingM->getInfo(array('id' => $statis['rating']), array('field' => '`freelook_num`'));
                    if ($ratinginfo['freelook_num'] > 0) {
                        //查询该企业会员今天免费查看投递简历数
                        $downresumeM = $this->MODEL('downresume');
                        $downnum = $downresumeM->getFreeDownNum(array('comid' => $this->uid, 'downtime' => array('>=', strtotime('today'))));
                        //查询简历是否投递该企业
                        $jobM = $this->MODEL('job');
                        $userid_job = $jobM->getSqJobInfo(array('com_id' => $this->uid, 'eid' => $id, 'isdel' => 9));
                        if ($ratinginfo['freelook_num'] > $downnum && !empty($userid_job)) {
                            $freenum = $ratinginfo['freelook_num'] - $downnum;
                            $this->yunset('freenum', $freenum);
                        }
                    }
                }
                $add = $ratingM->getComSerDetailList(array('orderby' => array('type,asc', 'sort,desc')), array('pack' => '1'));
                $this->yunset('add', $add);

				
                if (!isVip($statis['vip_etime'])) {

                    $this->yunset('vipIsDown', 1); //  会员过期
                }
            }
            $cacheM = $this->MODEL('cache');

            $options = array('user');

            $cache = $cacheM->GetCache($options);

            $this->yunset($cache);
            $this->com_tpl('hr_resume');
        } else {
            $this->yunset(array('errmsg' => '参数错误！'));
            $this->com_tpl('hr_resume');exit();
        }
    }
    //简历是否申请过公司职位
    function everApplied_action(){
        if (!$_GET['eid']){
            echo json_encode(array('code' => 400, 'msg' => '非法访问'));
            exit();
        }
        // 企业登录时，获取企业的职位列表
        if ($this->usertype == 2) {
            $JobM = $this->MODEL('job');
            $sqJob = $JobM->getSqJobInfo(array('eid' => intval($_GET['eid']), 'com_id' => $this->uid));
            if ($sqJob) {
                echo json_encode(array('code' => 200, 'msg' => '用户申请过该企业职位'));
                exit();
            } else {
                echo json_encode(array('code' => 400, 'msg' => '用户未申请过该企业职位', 'data' => $sqJob));
                exit();
            }
        }


        }


    // 查询备注
    function remarkhr_action()
    {

        $where   = array('eid'=>$_POST['eid'],'uid'=>$_POST['uid'],'comid'=>$this->uid);

        $remarkM = $this->MODEL('remark');

        $remarks = $remarkM->getRemarkInfo($where, array('field' => 'remark,status'));

        echo json_encode($remarks);

    }

    //简历弹窗举报简历
    function report_action()
    {
        $data = array(
            'reason'    =>  implode(',', $_POST['reason']),
            'c_uid'     =>  (int)$_POST['r_uid'],
            'inputtime' =>  time(),
            'p_uid'     =>  $this->uid,
            'did'       =>  $this->userid,
            'usertype'  =>  $this->usertype,
            'eid'       =>  (int)$_POST['eid'],
            'r_name'    =>  $_POST['r_name'],
            'username'  =>  $this->username
        );
        $reportM = $this->MODEL('report');
        $return = $reportM->ReportResume($data);
        $this->ACT_layer_msg($return['msg'], $return['errcode'],$_SERVER['HTTP_REFERER']);
    }

    //查询下一条未查看的投递简历id
    function nexts_action()
    {
        $state  =   1;

        $jobM   =   $this->MODEL('job');

        $jobs   =   $jobM->getSqJobInfo(array('com_id'=>$this->uid,'is_browse'=>1,'orderby'=>array('datetime,DESC')),array('field'=>'eid'));

        if(!empty($jobs)){
            $this->layer_msg('',9,0,'index.php?c=hr&act=resumeInfo&eid='.$jobs['eid'].'&state='.$state);
        }else{
            $this->layer_msg('',8);
        }
    }
}

?>