<?php

/**
 * Class dataBoard_controller
 *
 * @author shiy
 * @version 7.0
 */

class dataBoard_controller extends adminCommon
{

    function index_action()
    {
        $TongJi = $this->MODEL('tongji');


        $userStats = $TongJi->getTj('member', $_POST, 'reg_date', array('usertype' => '1'));

        $list['adduser']['name'] = '新增个人';
        $list['adduser']['list'] = $userStats['list'];
        $allNum['adduser'] = $userStats['allnum'];

        $expectStats = $TongJi->getTj('resume_expect', $_POST, 'ctime');
        $list['addexpect']['name'] = '新增简历';
        $list['addexpect']['list'] = $expectStats['list'];
        $allNum['addexpect'] = $expectStats['allnum'];

        $resumeDeliveryStats = $TongJi->getTj('userid_job', $_POST, 'datetime');
        $list['resumeDelivery']['name'] = '简历投递';
        $list['resumeDelivery']['list'] = $resumeDeliveryStats['list'];
        $allNum['resumeDelivery'] = $resumeDeliveryStats['allnum'];

        $resumeRefreshStats = $TongJi->getTj('resume_refresh_log', $_POST, 'r_time');
        $list['resumeRefresh']['name'] = '简历刷新';
        $list['resumeRefresh']['list'] = $resumeRefreshStats['list'];
        $allNum['resumeRefresh'] = $resumeRefreshStats['allnum'];

        $comStats = $TongJi->getTj('member', $_POST, 'reg_date', array('usertype' => '2'));
        $list['addcom']['name'] = '新增企业';
        $list['addcom']['list'] = $comStats['list'];
        $allNum['addcom'] = $comStats['allnum'];

        $jobStats = $TongJi->getTj('company_job', $_POST, 'sdate');
        $list['addjob']['name'] = '新增职位';
        $list['addjob']['list'] = $jobStats['list'];
        $allNum['addjob'] = $jobStats['allnum'];

        $downResumeStats = $TongJi->getTj('down_resume', $_POST, 'downtime');
        $list['downResume']['name'] = '简历下载';
        $list['downResume']['list'] = $downResumeStats['list'];
        $allNum['downResume'] = $downResumeStats['allnum'];

        $jobRefreshStats = $TongJi->getTj('job_refresh_log', $_POST, 'r_time');
        $list['jobRefresh']['name'] = '职位刷新';
        $list['jobRefresh']['list'] = $jobRefreshStats['list'];
        $allNum['jobRefresh'] = $jobRefreshStats['allnum'];

        $Stats = $TongJi->getTj('userid_msg', $_POST, 'datetime');
        $list['inviteInterview']['name'] = '邀请面试';
        $list['inviteInterview']['list'] = $Stats['list'];
        $allNum['inviteInterview'] = $Stats['allnum'];

        
        $this->render_json(0, '', array('allNum' => $allNum, 'list' => $list));
    }

    function class_action()
    {

        $TongJi = $this->MODEL('tongji');

        if ($_POST['type'] == 1) {

            $Stats = $TongJi->getTj('member', $_POST, 'reg_date');
            $List['allReg']['name'] = '所有会员';
            $List['allReg']['list'] = $Stats['list'];
            $AllNum['allReg'] = $Stats['allnum'];

            $comStats = $TongJi->getTj('member', $_POST, 'reg_date', array('usertype' => '2'));
            $List['comReg']['name'] = '企业会员';
            $List['comReg']['list'] = $comStats['list'];
            $AllNum['comReg'] = $comStats['allnum'];

            $userStats = $TongJi->getTj('member', $_POST, 'reg_date', array('usertype' => '1'));
            $List['userReg']['name'] = '个人会员';
            $List['userReg']['list'] = $userStats['list'];
            $AllNum['userReg'] = $userStats['allnum'];

            $CountTj = $TongJi->DataTj('reg', $Stats['timedate']['DateWhere'], 'member', 'uid');

            $Stats = $TongJi->getTj('login_log', $_POST, 'ctime', array('usertype' => array('in', '1,2')));
            $List['allLogin']['name'] = '所有会员';
            $List['allLogin']['list'] = $Stats['list'];
            $AllNum['allLogin'] = $Stats['allnum'];

            $comStats = $TongJi->getTj('login_log', $_POST, 'ctime', array('usertype' => '2'));
            $List['comLogin']['name'] = '企业会员';
            $List['comLogin']['list'] = $comStats['list'];
            $AllNum['comLogin'] = $comStats['allnum'];

            $userStats = $TongJi->getTj('login_log', $_POST, 'ctime', array('usertype' => '1'));
            $List['userLogin']['name'] = '个人会员';
            $List['userLogin']['list'] = $userStats['list'];
            $AllNum['userLogin'] = $userStats['allnum'];


        } elseif ($_POST['type'] == 2) {

            $Stats = $TongJi->getTj('look_job', $_POST, 'datetime');

            $List['job']['name'] = '职位浏览';
            $List['job']['list'] = $Stats['list'];
            $AllNum['job'] = $Stats['allnum'];
            $TopList['jobList'] = $TongJi->TopTen("look_job", $Stats['timedate']['DateWhere'], "jobid", "job");
            $TopList['jobComList'] = $TongJi->TopTen("look_job", $Stats['timedate']['DateWhere'], "com_id", "company");

            $Stats = $TongJi->getTj('look_resume', $_POST, 'datetime');
            $List['resume']['name'] = '简历浏览';
            $List['resume']['list'] = $Stats['list'];
            $AllNum['resume'] = $Stats['allnum'];

            $TopList['resumeList'] = $TongJi->TopTen("look_resume", $Stats['timedate']['DateWhere'], "resume_id", "expect");
            $TopList['resumeComList'] = $TongJi->TopTen("look_resume", $Stats['timedate']['DateWhere'], "com_id", "company");

        } elseif ($_POST['type'] == 3) {

            $Stats = $TongJi->getTj('userid_msg', $_POST, 'datetime');

            $List['invite']['name'] = '邀请面试';
            $List['invite']['list'] = $Stats['list'];
            $AllNum['invite'] = $Stats['allnum'];

            $TopList['inviteCom'] = $TongJi->TopTen("userid_msg", $Stats['timedate']['DateWhere'], "fid", "company");
            $TopList['inviteResume'] = $TongJi->TopTen("userid_msg", $Stats['timedate']['DateWhere'], "uid", "resume");
            $inviteCountTj = $TongJi->DataTj('job', $Stats['timedate']['DateWhere'], 'userid_msg', 'jobid');

            $Stats = $TongJi->getTj('down_resume', $_POST, 'downtime');
            $freeStats = $TongJi->getTj('freedown_resume', $_POST, 'downtime');
            $List['down']['name'] = '简历下载';
            $List['down']['list'] =  $this->handlerDownResume($Stats['list'],$freeStats['list']) ;
            $AllNum['down'] = $Stats['allnum']+$freeStats['allnum'];

            $TopList['downCom'] = $TongJi->TopTen("down_resume", $Stats['timedate']['DateWhere'], "comid", "company");
            $TopList['downResume'] = $TongJi->TopTen("down_resume", $Stats['timedate']['DateWhere'], "uid", "resume");


            $freedownCom = $TongJi->TopTen("freedown_resume", $Stats['timedate']['DateWhere'], "comid", "company");
            $freedownResume = $TongJi->TopTen("freedown_resume", $Stats['timedate']['DateWhere'], "uid", "resume");
            $TopList['downCom'] = $this->handlerDownResumeList($TopList['downCom'],$freedownCom);
            $TopList['downResume'] = $this->handlerDownResumeList($TopList['downResume'],$freedownResume);

            $downCountTj = $TongJi->DataTj('resume_expect', $Stats['timedate']['DateWhere'], 'down_resume', 'eid');

            $inviteCountTj['job1'] = $this->pieMutiDeal(array('arr'=>$inviteCountTj['job1'],'isother'=>true));
            $downCountTj['job1'] = $this->pieMutiDeal(array('arr'=>$downCountTj['job1'],'isother'=>true));
            $CountTj = [$inviteCountTj, $downCountTj];

        } elseif ($_POST['type'] == 4) {

            $where = array('order_state' => '2');

            $Stats = $TongJi->getTj('company_order', $_POST, 'order_time', $where, "SUM(`order_price`) as count");

            $List['order']['name'] = '充值金额';
            $List['order']['list'] = $Stats['list'];
            $AllNum['order'] = round($Stats['allnum'], 2);

            $TopList['orderCom'] = $TongJi->TopTen('company_order', array_merge($Stats['timedate']['DateWhere'], $where), "uid", "order", '10', "SUM(`order_price`) as count");

            $CountTj = $TongJi->DataTj('order', array_merge($Stats['timedate']['DateWhere'], $where), 'company_order', 'id');

            $Stats = $TongJi->getTj('adclick', $_POST, 'addtime');
            $List['ad']['name'] = '点击量';
            $List['ad']['list'] = $Stats['list'];
            $AllNum['ad'] = $Stats['allnum'];

            $TopList['adClick'] = $TongJi->TopTen("adclick", $Stats['timedate']['DateWhere'], "aid", "ad");

        } elseif ($_POST['type'] == 5) {

            $Stats = $TongJi->getTj('company_job', $_POST, 'sdate');
            $List['addJob']['list'] = $Stats['list'];
            $List['addJob']['name'] = '发布职位';
            $AllNum['addJob'] = $Stats['allnum'];

            $updateStats = $TongJi->getTj('job_refresh_log', $_POST, 'r_time');
            $List['upJob']['list'] = $updateStats['list'];
            $List['upJob']['name'] = '刷新职位';
            $AllNum['upJob'] = $updateStats['allnum'];

            $TopList['addJobCom'] = $TongJi->TopTen("company_job", $Stats['timedate']['DateWhere'], "uid", "company");

            $CountTjJob = $TongJi->DataTj('job', $Stats['timedate']['DateWhere'], 'company_job', 'id');

            $Stats = $TongJi->getTj('resume_expect', $_POST, 'ctime');
            $List['addResume']['list'] = $Stats['list'];
            $List['addResume']['name'] = '简历新增';
            $AllNum['addResume'] = $Stats['allnum'];
            $updateStats = $TongJi->getTj('resume_refresh_log', $_POST, 'r_time');
            $List['upResume']['list'] = $updateStats['list'];
            $List['upResume']['name'] = '简历刷新';
            $AllNum['upResume'] = $updateStats['allnum'];

            $CountTjResume = $TongJi->DataTj('resume_expect', $Stats['timedate']['DateWhere'], 'resume_expect', 'id');
            $CountTjJob['job1'] = $this->pieMutiDeal(array('arr'=>$CountTjJob['job1'],'isother'=>true));
            $CountTjResume['job1'] = $this->pieMutiDeal(array('arr'=>$CountTjResume['job1'],'isother'=>true));
            $CountTj = [$CountTjJob, $CountTjResume];
        } elseif ($_POST['type'] == 6) {

            $Stats = $TongJi->getTj('member', $_POST, 'reg_date', array('usertype' => '2'));
            $List['comOne']['name'] = '企业统计';
            $List['comOne']['list'] = $Stats['list'];
            $AllNum['comOne'] = $Stats['allnum'];

            $comStats = $TongJi->getTj('member', $_POST, 'reg_date', array('usertype' => '2', 'status' => '0'));
            $List['comTwo']['name'] = '待审核企业';
            $List['comTwo']['list'] = $comStats['list'];
            $AllNum['comTwo'] = $comStats['allnum'];

            $CountTjCom = $TongJi->DataTj('company', $Stats['timedate']['DateWhere'], 'member', 'uid');
           

            $CountTjCom['hy'] = $this->pieMutiDeal(array('arr'=>$CountTjCom['hy'],'isother'=>true));
            $Stats = $TongJi->getTj('userid_job', $_POST, 'datetime');
            $List['apply']['name'] = '简历投递';
            $List['apply']['list'] = $Stats['list'];
            $AllNum['apply'] = $Stats['allnum'];

            $TopList['applyCom'] = $TongJi->TopTen("userid_job", $Stats['timedate']['DateWhere'], "com_id", "company");
            $TopList['applyResume'] = $TongJi->TopTen("userid_job", $Stats['timedate']['DateWhere'], "eid", "expect");
            $CountTjResume = $TongJi->DataTj('resume_expect', $Stats['timedate']['DateWhere'], 'userid_job', 'eid');
            $CountTjResume['job1'] = $this->pieMutiDeal(array('arr'=>$CountTjResume['job1'],'isother'=>true));
            $CountTj = [$CountTjCom, $CountTjResume];
        }

        $this->render_json(0, '', array('AllNum' => $AllNum, 'List' => $List, 'topList' => $TopList, 'CountTj' => $CountTj));
    }
    //饼状图数据处理
    function pieMutiDeal($data=array()){
        $result = array();

        $arr    =   !empty($data['arr'])?$data['arr']:array();
        $sortkey=   !empty($data['sortkey'])?$data['sortkey']:'count';
        $sort   =   !empty($data['sort'])?$data['sort']:'desc';
        $num    =   !empty($data['num'])?$data['num']:10;//饼图分隔数
        $isother=   $data['isother']?true:false;//超过num的部分是否需要展示为其它

        if(!empty($arr)){

            $sortData = array();
            foreach ($arr as $key => $value) {
                $sortData[] = $value[$sortkey];
            }
            $sort_type = $sort=='desc'?SORT_DESC:SORT_ASC;
            array_multisort($sortData,$sort_type, $arr);
            
            if($num>0 && count($arr)>$num){

                $result = array_slice($arr,0,$num);

                if($isother){

                    $hasother=false; 
                    $otherKey = 0;
                    $otherCount = 0;

                    foreach ($result as $rk => $rv) {
                        if($rv['isother']){
                            $otherCount = $rv[$sortkey];
                            $hasother = true;
                            $otherKey = $rk;
                        }
                    }

                    $otherdata = array_slice($arr,$num);

                    $otherNumArr=array_column($otherdata,$sortkey);

                    $otherCount += array_sum($otherNumArr);

                    if($hasother){
                        $result[$otherKey][$sortkey] = $otherCount;
                    }else{
                        $result[] = array('name'=>'其他',$sortkey=>$otherCount);
                    }
                    
                }

            }
        }
        
        return $result;

    }



    public function fenxiabiao_action(){
        $_POST['fxStatus'] = 1;
        if (!$_POST['startTime'] && !$_POST['endTime']){
            $this->render_json('1','请选择对比的时间');
        }
        $fxType = $_POST['type']?$_POST['type']:1; // 1表示一年2表示月
        $time = $_POST['startTime'];
        $TongJi = $this->MODEL('tongji');

        if($_POST['startTime']){
            $arr=$this->setFenxiabiaoData($fxType,$time);
            $firstData = $TongJi->fenxiabiaoClient($arr['sdate'],$arr['edate']);//
        }
        if ($_POST['endTime']){
            $time = $_POST['endTime'];
            $arr2= $this->setFenxiabiaoData($fxType,$time);
            $secondData = $TongJi->fenxiabiaoClient($arr2['sdate'],$arr2['edate']);
        }
        $timeStart = $arr['sdate'];
        $timeEnd = $arr2['sdate'];
        $statistics  = array(
            'member'=>'gerezce',
            'login_log'=>'login_log',
            'resume_expect'=>'jilizce',
            'company'=>'comzce',
            'company_login_log'=>'company_login_log',
            'company_job'=>'fabuzhw',
            'userid_job'=>'jilitod',
            'chat_log'=>'liaotan',
            'userid_msg'=>'yaoqms',
            'down_resume'=>'jilixza'
        );
        $firstResult = array();
        $secondResult = array();
        $steps = $arr['month']>1? $arr['month']+1:$arr['month'];
        for ($i = 0; $i < $steps; $i++) {
            if ($i == ($steps-1)){
                $onday = 'sum';
                $onday2 = 'sum';
                $firstResult[$onday]['years'] = '总计';
                $secondResult[$onday2]['years'] = '总计';
            }else{
                $onday = date("Y-m", strtotime('+'.$i. ' month ',$timeStart));
                $onday2 = date("Y-m", strtotime('+'.$i. ' month ',$timeEnd));
                $firstResult[$onday]['years'] = $onday;
                $secondResult[$onday2]['years'] = $onday2;
            }

            foreach ($statistics as $k=> $v){
                $first = isset($firstData[$k])?$firstData[$k]:[];
                $second = isset($secondData[$k])?$secondData[$k]:[];
                // 获取对应数据
                $firstNum =  isset($first[$onday])?$first[$onday]:0;
                $secondNum = isset($second[$onday2])?$second[$onday2]:0;

                if ($k == 'down_resume'){
                    $freeNum1 = isset($firstData['freedown_resume'][$onday]) ? $firstData['freedown_resume'][$onday]:0;
                    $freeNum2 = isset($secondData['freedown_resume'][$onday2]) ? $secondData['freedown_resume'][$onday2]:0;
                    $firstNum = $firstNum+$freeNum1;
                    $secondNum = $secondNum+$freeNum2;
                }
                $firstResult[$onday][$v] =$firstNum;
                $secondResult[$onday2][$v] =$secondNum;
                $secondResult[$onday2][$v.'_percent'] = bcmul(bcdiv(bcsub($secondNum, $firstNum), $firstNum, 2), 100);
            }
        }
        sort($secondResult);
        sort($firstResult);
        $this->render_json(0,'',array('firstResult'=>$firstResult,'secondResult'=>$secondResult));
    }


    function  setFenxiabiaoData($fxType,$time){
        if ($fxType == 1){
            $time = $time.'-01-01';
            $timeStart = strtotime($time);
            $sdate = $timeStart;
            $edate = strtotime("+1 year",$timeStart);
            $month =12;
        }else{
            $time = $time.'-01';
            $timeStart = strtotime($time);
            $sdate = $timeStart;
            $edate = strtotime("+1 month",$timeStart);
            $month = 1;
        }

        return array(
            'sdate'=>$sdate,
            'edate'=>$edate,
            'month'=>$month
        );

    }

    public function getAuth_action(){
        $navi_id = $_POST['navi_id'];
        if (!$navi_id){
            $this->render_json(1,'参数错误');
        }
        $status = false;
        if (in_array($navi_id,$this->adminPower['power'])){
            $status = true;
        }

        $result = array(
            'status'=>$status
        );
        $this->render_json('0','',$result);

    }
    public function handlerDownResume($arr,$freearr){

        foreach ($arr as &$vlue){
            foreach ($freearr as $freeval){
                if($vlue['tjtime'] == $freeval['tjtime']){
                    $vlue['count'] +=$freeval['count'];
                }
            }
        }
        return $arr;
    }


    public function handlerDownResumeList($list,$freelist){
        $listArr = array();
        foreach ($list as $value){
            $listArr[$value['uid']] =$value;
        }
        foreach ($freelist as $value1){
            if (isset($listArr[$value1['uid']])){
                $listArr[$value1['uid']]['count'] = $listArr[$value1['uid']]['count'] + $value1['count'];
            }else{
                $listArr[$value1['uid']] = $value1;
            }
        }
        $sort = array(
            'direction' => 'SORT_DESC', //排序顺序标志 SORT_DESC 降序；SORT_ASC 升序
            'field'     => 'count',       //排序字段
        );
        foreach($listArr AS $uniqid => $row){
            foreach($row AS $key=>$value){
                $arrSort[$key][$uniqid] = $value;
            }
        }
        if($sort['direction']){
            array_multisort($arrSort[$sort['field']], constant($sort['direction']), $listArr);
        }
        $count = count($listArr)>=10?10:count($listArr);
        $listArr = array_slice($listArr, 0, $count);

        return $listArr;
    }

}

?>