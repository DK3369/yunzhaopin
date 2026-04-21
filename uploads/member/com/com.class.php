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

class company extends common
{

    public $comInfo =   array();
    public $leftNav =   array();

    function __construct($tpl, $db, $def = '', $model = 'index', $m = '')
    {
        $this->common($tpl, $db, $def, $model, $m);

        // 入口判断企业资料是否完善 排除部分act
        $Company        =   $this->MODEL('company');

        $uid            =   $this->uid;

        $this->comInfo  =   $Company->getInfo($uid, array('info' => '1', 'edit' => '1', 'logo' => '1', 'utype' => 'user'));

        if ($_GET['c'] == 'jobadd'){

            $addressM   =   $this->MODEL('address');
            $addressList=   $addressM->getAddressList(array('uid' => $this->uid,'orderby'=>'id'));
            $this->yunset('addressList', $addressList);

            $defLink    =   array(

                'link_man'      =>  $this->comInfo['linkman'],
                'link_moblie'   =>  !empty($this->comInfo['linktel']) ? $this->comInfo['linktel'] : '',
                'link_phone'   =>  !empty($this->comInfo['linkphone']) ? $this->comInfo['linkphone'] : '',
                'email'         =>  $this->comInfo['linkmail'],
                'city'          =>  $this->comInfo['job_city_one'].$this->comInfo['job_city_two'].$this->comInfo['job_city_three'],

                'provinceid'    =>  $this->comInfo['provinceid'],
                'cityid'        =>  $this->comInfo['cityid'],
                'three_cityid'  =>  $this->comInfo['three_cityid'],
                'address'       =>  $this->comInfo['address'],
                'x'             =>  $this->comInfo['x'],
                'y'             =>  $this->comInfo['y']
            );
            $this->yunset('defLink', $defLink);
        }

        if (!in_array($_GET['c'], array('binding', 'map', 'info','log', 'uppic')) && !in_array($_GET['act'], array('logout', 'jobCheck'))) {

            // 强制完善基本资料
            if ($this->config['com_enforce_info'] == 1) {

                if (!$this->comInfo['info']['name'] || !$this->comInfo['info']['provinceid'] || (!$this->comInfo['info']['linktel'] && !$this->comInfo['info']['linkphone'])) {

                    $remindInfo['url']          =   'index.php?c=info';
                    $remindInfo['msg']          =   '完善企业信息有助于帮您快速招聘人才！';

                    if (!$this->comInfo['info']['provinceid']) {

                        $remindInfo['title']    =   '所在地区尚未完善！';
                    } elseif (!$this->comInfo['info']['linktel'] && !$this->comInfo['info']['linkphone']) {

                        $remindInfo['title']    =   '联系方式尚未完善！';
                    } else {

                        $remindInfo['title']    =   '企业信息尚未完善！';
                    }

                    $this->yunset('isremind', 1);
                    $this->yunset('remindInfo', $remindInfo);
                    $this->yunset('row', $this->comInfo['info']);
                    $this->yunset($this->MODEL('cache')->GetCache(array('com', 'city', 'job', 'hy')));

                    $this->com_tpl('info');
                }
            } elseif (!$this->comInfo['info']['uid']) {

                //容错机制，前期强制完善资料，后期开放，防止部分数据无uid 又可以直接操作会员中心
                $userInfoM  =   $this->MODEL("userinfo");
                $userInfoM->activUser($uid, 2);
            }
        }

        if (!$_GET['c'] ||in_array($_GET['c'], array('sysnews', 'msg'))) {

            $this->yunset('leftCur', '1');
        } else if (in_array($_GET['c'], array('job', 'jobadd', 'partok', 'part', 'partadd', 'partapply', 'tongji'))) {

            $this->yunset('leftCur', '2');
        } else if (in_array($_GET['c'], array('hr', 'down', 'look_job', 'attention_me', 'look_resume', 'talent_pool'))) {

            $this->yunset('leftCur', '3');
        } else if (in_array($_GET['c'], array('invite', 'yqmb'))) {

            $this->yunset('leftCur', '4');
        } else if (in_array($_GET['c'], array('resume'))) {

            $this->yunset('leftCur', '5');
        } else if (in_array($_GET['c'], array('zhaopinhui'))) {

            $this->yunset('leftCur', '7');
        }  else if (in_array($_GET['c'], array('right', 'ad', 'ad_order', 'pay', 'payment', 'paylogtc', 'paylog', 'integral', 'integral_reduce', 'reward_list'))) {

            $this->yunset('leftCur', '9');
        } else if (in_array($_GET['c'], array('info', 'show', 'uppic', 'product', 'news', 'map', 'comtpl', 'banner'))) {

            $this->yunset('leftCur', '10');
        } else if (in_array($_GET['c'], array('binding', 'address', 'vs', 'logout', 'sysnews', 'msg', 'setname', 'report'))) {

            $this->yunset('leftCur', '11');
        } else if (in_array($_GET['c'], array('record', 'finder', 'special', 'report', 'customize'))) {

            $this->yunset('leftCur', '12');
        }

        // 新投递尚未查看的简历
        $JobM           =   $this->MODEL('job');
        $sqJobWhere     =   array('com_id' => $this->uid, 'is_browse' => 1, 'isdel' => 9);
        $newResumeNum   =   $JobM->getSqJobNum($sqJobWhere);
        $this->yunset('newResumeNum', $newResumeNum);

        if ($this->comInfo['is_nav'] == 2){
            $this->leftNav  =   $Company->getLeftNav($this->uid);

            $hideNav    =   array();
            foreach ($this->leftNav as $lk => $lv){
                if ($lv['show'] == 2){
                    $hideNav[]  =   $lv;
                }
            }

            $this->yunset(array('leftNav' => $this->leftNav, 'hideNav' => $hideNav));
        }
    }

    function public_action()
    {

        $statis     = $this->company_satic();
        $adminM     =   $this->MODEL('admin');

        $now_url    =   @explode('/', $_SERVER['REQUEST_URI']);
        $now_url    =   $now_url[count($now_url) - 1];

        $this->yunset('now_url', $now_url);

        $company    =   $this->comInfo;
        $this->yunset('company', $company);

        $guweninfo              =   $adminM->getAdminUser(array('uid' => $company['crm_uid']));
        $guweninfo['photo_n']   =   checkpic($guweninfo['photo'], $this->config['sy_guwen']);
        $guweninfo['ewm_n']     =   checkpic($guweninfo['ewm'], $this->config['sy_wx_qcode']);
        $this->yunset('guweninfo', $guweninfo);
        if ((int)$this->comInfo['crm_uid'] > 0){

            $consultant =   array('name' => $guweninfo['name'], 'mobile' => $guweninfo['moblie'], 'wechat' => !empty($guweninfo['ewm']) ? checkpic($guweninfo['ewm']): '');
            $this->yunset(array('consultant' => $consultant, 'crm' => 1));
        }else{

            $this->yunset(array('crm' => 2, 'webTel' => $this->config['sy_comwebtel']));
        }

        if (!empty($guweninfo)) {

            $ReportM            =   $this->MODEL('report');
            $reportwhere        =   array('p_uid' => $this->uid, 'eid' => $guweninfo['uid'], 'type' => 2, 'orderby' => array('inputtime, desc'));
            $report             =   $ReportM->getReportOne($reportwhere);
            $this->yunset('report', $report);
        }

        $qq                     =   explode(',', $this->config['sy_qq']);
        $this->yunset('kfqq', $qq[0]);

        // 会员等级、增值包、套餐
        $ratingM    =   $this->MODEL('rating');
        if (!empty($this->comInfo['package'])) {

            $ratingList =   $ratingM->getList(array('id' => array('in', $this->comInfo['package']), 'orderby' => array('type,asc', 'sort,desc')));
        } else {
            $ratingList =   $ratingM->getList(array('display' => 1, 'orderby' => array('type,asc', 'sort,desc')));
        }
        $rating_1   =   $rating_2   =   $raV    =   array();

        if (!empty($ratingList)) {
            foreach ($ratingList as $ratingV) {

                $raV[$ratingV['id']]    =   $ratingV;

                if ($ratingV['category'] == 1) {
                    if ($ratingV['type'] == 1) {

                        $rating_1[]     =   $ratingV;
                    } elseif ($ratingV['type'] == 2) {

                        $rating_2[]     =   $ratingV;
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
        $this->yunset('rating_1', $rating_1);
        $this->yunset('rating_2', $rating_2);

        if (!empty($statis)) {

            $discount   =   isset($raV[$statis['rating']]) ? $raV[$statis['rating']] : array();
            $this->yunset('discount', $discount);
        }
        $add            =   $ratingM->getComSerDetailList(array('orderby' => array('type,asc', 'sort,desc')), array('pack' => '1'));
        $this->yunset('add', $add);

        return $statis;
    }

    /**
     * @desc 会员统计信息调用
     */
    function company_satic()
    {
        // 会员套餐过期检测，并处理
        $uid        =   $this->uid;
        $statisM    =   $this->MODEL('statis');
        $statis     =   $statisM->vipOver($uid, 2);

        $this->yunset('addjobnum', $statis['addjobnum']);
        $this->yunset('spviewNum', $statis['spviewNum']);

        if (!isVip($statis['vip_etime'])) {

            $statis['vipIsDown']    =   1;
            $this->yunset('vipIsDown', 1); //  会员过期
        }

        $this->yunset('statis', $statis);
        $this->yunset('todayStart', strtotime('today'));

        return $statis;
    }

    /**
     * @desc 职位操作：举报、删除、修改状态
     */
    function job()
    {

        $ReportM    =   $this->MODEL('report');
        $logM       =   $this->Model('log');
        $jobM       =   $this->Model('job');
        $CompanyM   =   $this->Model('company');
        $statisM    =   $this->Model('statis');

        // 举报
        if ($_GET['r_uid']) {
            if ($_GET['r_reason'] == "") {

                $this->ACT_layer_msg('举报内容不能为空！', 8, 'index.php?c=down');
            }

            $data['p_uid']      =   (int)$_GET['r_uid'];
            $data['inputtime']  =   time();
            $data['c_uid']      =   $this->uid;
            $data['did']        =   $this->userid;
            $data['eid']        =   (int)$_GET['eid'];
            $data['r_name']     =   $_GET['r_name'];
            $data['usertype']   =   (int)$this->usertype;
            $data['username']   =   $this->username;
            $data['reason']     =   $_GET['r_reason'];

            $result =   $ReportM->ReportResume($data);

            $this->ACT_layer_msg($result['msg'], $result['errcode'], "index.php?c=down");
        }

        // 职位发布状态修改---js
        if ($_POST['status'] && ($_POST['id'] || is_array($_POST['id']))) {

            $id     =   $_POST['id'];

            if ($_POST['status'] == 2) {

                $_POST['status'] = 0;
            }

            $data   =   array(
                'status'        =>  $_POST['status'],
            );
            if($_POST['status']==0){
                $data['upstatus_time'] =  time();
            }

            if($_POST['status']==0){
                $statis =   $this->company_satic();
                // 上架职位
                $jobnum = count($id);
                $rating_type = $statis['rating_type'];
                $return = $statisM->getCom(array('uid' =>$this->uid, 'usertype' =>$this->usertype,'upstatus'=>1,'jobnum'=>$jobnum));

                if (!empty($return)) {
                    if($return['errcode']==9){
                        $rating_type = 2;
                    }else{
                        $return['errcode'] = 2;
                        echo json_encode($return);die();
                    }
                }

                $joblist = $jobM->getList(array('id' => array('in', pylode(',', $id))),array('field'=>'`id`,`upstatus_count`,`upstatus_time`'));

                if(!empty($joblist['list'])){

                    $today      = strtotime('today');

                    foreach ($joblist['list'] as $jk => $jv) {

                        $upstatus_count = $jv['upstatus_count'];

                        if($rating_type==2){
                            if($jv['upstatus_time']> $today){
                                $upstatus_count += 1;
                            }else{
                                $upstatus_count = 1;
                            }
                        }else if($rating_type==1){
                            $upstatus_count += 1;
                        }

                        $data['upstatus_count'] = $upstatus_count;

                        $nid    =   $jobM->upInfo($data, array('id' => $jv['id'], 'uid' => $this->uid));

                        $statis =   $this->company_satic();
                        if ($statis['rating_type'] == 1) {

                            $statisM    =   $this->MODEL('statis');
                            $payDetail  =   '上架职位，消耗上架套餐数量：' . $jobnum;
                            $statisM->addStatisDetail(array('uid' => $this->uid, 'type' => 1, 'num' => $jobnum, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                        }
                    }
                }

            }else{
                $nid    =   $jobM->upInfo($data, array('id' => array('in', pylode(',', $id), 'uid' => $this->uid)));
            }
            

            if ($nid) {

                $logContent =   '职位更新：调整职位招聘状态';
                $logDetail  =   $_POST['status'] == 0 ? '修改职位(ID:'.pylode(',', $id).')发布状态：下架->上架' : '修改职位(ID:'.pylode(',', $id).')发布状态：上架->下架';
                $logM->addMemberLog($this->uid, $this->usertype, $logContent, 1, 2, $logDetail);

                echo json_encode(array('errcode'=>1));
                die();
            } else {

                echo json_encode(array('errcode'=>8,'msg'=>'设置失败！'));
                die();
            }
        }

        // 职位删除
        if ($_GET['del'] || is_array($_POST['checkboxid'])) {

            if (is_array($_POST['checkboxid'])) {

                $layer_type =   1; // 提示方式
            } else if ($_GET['del']) {

                $layer_type =   0; // 提示方式
            }

            $delid              =   is_array($_POST['checkboxid']) ? $_POST['checkboxid'] : $_GET['del'];

            $numWhere['uid']    =   $this->uid;

            $numWhere['jobid']  =   array('in', pylode(',', $delid));

            // 判断是否有赏金职位未处理 处理完才允许删除职位
            $rewardJobNum       =   $PackM->getCompanyJobRewardNum($numWhere);
            $shareJobNum        =   $PackM->getCompanyJobShareNum($numWhere);

            if ($rewardJobNum > 0 || $shareJobNum > 0) {

                $this->layer_msg('您还有赏金职位未处理！', 8, $layer_type, $_SERVER['HTTP_REFERER']);
            } else {

                $return         =   $jobM->delJob($delid, array('uid' => $this->uid));

                if ($return['id']) {

                    $jobWhereData   =   array(
                        'uid'       =>  $this->uid,
                        'orderby'   => 'lastupdate,desc'
                    );

                    $newest =   $jobM->getInfo($jobWhereData, array('field' => 'lastupdate'));
                    $uid    =   $this->uid;
                    $companydata    =   array(

                        'jobtime'   =>  $newest['lastupdate']
                    );

                    $CompanyM->upInfo($uid, '', $companydata);

                    $this->layer_msg($return['msg'], $return['errcode'], $return['layertype'], $_SERVER['HTTP_REFERER']);
                } else {

                    $this->layer_msg($return['msg'], $return['errcode'], $return['layertype'], $_SERVER['HTTP_REFERER']);
                }
            }
        }
    }

    /**
     * 兼职操作
     */
    function part()
    {

        $partM  =   $this->MODEL("part");

        $logM   =   $this->Model('log');

        // 兼职职位延期
        if ($_POST['gotimeid']) {

            $_POST['day']   =   intval($_POST['day']);

            if ($_POST['day'] < 1) {

                $this->ACT_layer_msg("请正确填写延期天数！", 8);
            } else {

                $postTime   =   (int)$_POST['day'] * 86400;
                $uid        =   $this->uid;
                $ids        =   @explode(",", $_POST['gotimeid']);
                $row        =   $partM->getList(array('id' => array('in', pylode(',', $ids))), array('field' => 'state,edate,id'));

                foreach ($row as $val) {

                    if ($val['edate']) {
                        $edate      =   $val['edate'] + $postTime;
                    } else {
                        $edate      =   '';
                    }

                    $where['id']    =   $val['id'];
                    $where['uid']   =   $uid;

                    if ($row['state'] == 2) {
                        $data       =   array(

                            'state' => 1,
                            'edate' => $edate
                        );
                    } else {
                        $data       =   array(

                            'edate' => $edate
                        );
                    }

                    $id =   $partM->upInfo($data, $where);
                    if ($id) {

                        $logContent =   '兼职更新：延期招聘';
                        $logDetail  =   '兼职延期招聘天数 + '. $_POST['day'];
                        $logM->addMemberLog($this->uid, $this->usertype, $logContent, 9, 2, $logDetail);

                        $this->ACT_layer_msg("兼职延期成功！", 9, $_SERVER['HTTP_REFERER']);
                    } else {

                        $this->ACT_layer_msg("兼职延期失败！", 8, $_SERVER['HTTP_REFERER']);
                    }
                }
            }
        }

        // 兼职职位推荐
        if ($_POST['recid']) {

            $IntegralM  =   $this->MODEL('integral');
            $StatisM    =   $this->MODEL('statis');

            $id         =   intval($_POST['recid']);
            $uid        =   $this->uid;

            $_POST['day'] = intval($_POST['day']);

            if ($_POST['day'] < 1) {
                $this->ACT_layer_msg("请正确填写推荐天数！", 8, $_SERVER['HTTP_REFERER']);
            }

            $reow       =   $StatisM->getInfo($uid, array('usertype' => '2', 'field' => 'integral'));

            $part       =   $partM->geInfo('', array('where' => array('uid' => $uid, 'id' => $id), 'field' => 'name,rec_time'));
            $part       =   $part['info'];

            if ($part['rec_time'] < time()) {

                $time   =   time() + $_POST['day'] * 86400;
            } else {

                $time   =   $part['rec_time'] + $_POST['day'] * 86400;
            }

            $integral   =   $this->config['com_recjob'] * $_POST['day'];

            if ($reow['integral'] < $integral && $this->config['com_recjob_type'] == "2") {

                $this->ACT_layer_msg("您的" . $this->config['integral_pricename'] . "不足，请充值！", 8, "index.php?c=pay");
            } else {
                // 积分处理
                $IntegralM->company_invtal($this->uid, $this->usertype, $integral, false, "推荐兼职职位", true, 2, 'integral', 12);
            }

            $jobdata    =   array(

                'rec'       =>  1,
                'rec_time'  =>  $time
            );

            $where['id']    =   $id;
            $where['uid']   =   $uid;
            $partM->upInfo($jobdata, $where);

            $logContent =   '兼职推广：推荐兼职职位《'.$part['name'].'》';
            $logDetail  =   '兼职推荐，推荐天数 + '.$_POST['day'];
            $logM->addMemberLog($this->uid, $this->usertype, $logContent, 9, 2, $logDetail);

            $this->ACT_layer_msg("推荐兼职成功！", 9, $_SERVER['HTTP_REFERER']);
        }

        // 兼职上下架
        if ($_POST['status'] && ($_POST['id'] || is_array($_POST['id']))) {

            $id     =   $_POST['id'];

            if ($_POST['status'] == 2) {

                $_POST['status'] = 0;
            }

            $data   =   array(
                'status'        =>  $_POST['status'],
            );
            if($_POST['status']==0){
                $data['upstatus_time'] =  time();
            }

            if($_POST['status']==0){
                // 上架职位
                $jobnum = count($id);
                $rating_type = 1;
                $statisM    =   $this->Model('statis');
                $return = $statisM->getCom(array('uid' =>$this->uid, 'usertype' =>$this->usertype,'upstatus'=>1,'jobnum'=>$jobnum));

                if (!empty($return)) {
                    if($return['errcode']==9){
                        $rating_type = 2;
                    }else{
                        $return['errcode'] = 2;
                        echo json_encode($return);die();
                    }
                }

                $joblist = $partM->getList(array('id' => array('in', pylode(',', $id))),array('field'=>'`id`,`upstatus_count`,`upstatus_time`'));

                if(!empty($joblist)){

                    $today      = strtotime('today');

                    foreach ($joblist as $jk => $jv) {

                        $upstatus_count = $jv['upstatus_count'];

                        if($rating_type==2){
                            if($jv['upstatus_time']> $today){
                                $upstatus_count += 1;
                            }else{
                                $upstatus_count = 1;
                            }
                        }else if($rating_type==1){
                            $upstatus_count += 1;
                        }

                        $data['upstatus_count'] = $upstatus_count;

                        $nid    =   $partM->upInfo($data, array('id' => $jv['id'], 'uid' => $this->uid));

                        $statis =   $this->company_satic();
                        if ($statis['rating_type'] == 1) {

                            $statisM    =   $this->MODEL('statis');
                            $payDetail  =   '上架兼职，消耗上架套餐数量：' . $jobnum;
                            $statisM->addStatisDetail(array('uid' => $this->uid, 'type' => 1, 'num' => $jobnum, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));
                        }
                    }
                }

            }else{
                $nid    =   $partM->upInfo($data, array('id' => array('in', pylode(',', $id), 'uid' => $this->uid)));
            }

            if ($nid) {

                $logContent =   '兼职更新：调整兼职发布状态';
                $logDetail  =   $_POST['status'] == 0 ? '修改兼职发布状态：下架->上架' : '修改兼职发布状态：上架->下架';
                $logM->addMemberLog($this->uid, $this->usertype, $logContent, 9, 2, $logDetail);

                echo json_encode(array('errcode'=>1));
                die();
            } else {

                echo json_encode(array('errcode'=>8,'msg'=>'设置失败！'));
                die();
            }
        }
    }

    /**
     * @return mixed
     */
    function yqmsInfo()
    {

        $JobM   =   $this->MODEL('job');

        $where  =   array(

            'uid'       => $this->uid,
            'state'     =>  1,
            'r_status'  =>  1,
            'status'    =>  0
        );

        $company_job    =   $JobM->getMsyqJobList($where, array('field' => '`name`,`id`,`is_link`,`link_id`', 'link' => 'yes'));
        $this->yunset('company_job', $company_job['list']);

        $yqmbM  =   $this->MODEL('yqmb');
        $ymlist =   $yqmbM->getList(array('uid' => $this->uid, 'status' => 1));
        $this->yunset('ymlist', $ymlist);

        $ymnum  =   $yqmbM->getNum(array('uid' => $this->uid));
        $ymcan  =   $ymnum < $this->config['com_yqmb_num'] ? true : false;
        $this->yunset('ymcan', $ymcan);
        return $company_job['list'];
    }

    function com_tpl($tpl)
    {
        $this->yuntpl(array('member/com/' . $tpl));
    }

    function logout_action()
    {
        $this->logout();
    }

}

?>