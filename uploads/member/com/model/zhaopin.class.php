<?php
/* *
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/
class zhaopin_controller extends company{
    /**
     * 招聘数据中心
     */
	function index_action(){
		$this->public_action();
		$this->com_tpl("zhaopin");
	}

    /**
     * 获取今日数据
     */
    function getTodayData_action()
    {
        $this->checkOpen();
        $todaystart = strtotime('today');
        $todayend = strtotime(date('Y-m-d 23:59:59', $todaystart));
        $yesterdaystart = $todaystart - 86400;
        $yesterdayend = $todayend - 86400;
        if ($_POST['type'] == 'yesterday') {// 昨日数据
            $todaystart = $yesterdaystart;
            $todayend = $yesterdayend;
            $yesterdaystart = $todaystart - 86400;
            $yesterdayend = $todayend - 86400;
        }
        $todayWhere = array(
            array('>=', $todaystart),
            array('<=', $todayend)
        );
        $yesterdayWhere = array(
            array('>=', $yesterdaystart),
            array('<=', $yesterdayend)
        );
        // 我看过/比昨日
        $lookresumeM = $this->MODEL('lookresume');
        $lookResumeWhere = array('com_id' => $this->uid, 'usertype' => $this->usertype, 'com_status' => 0);
        $look_resume_num = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $todayWhere)));
        $look_resume_num_y = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $yesterdayWhere)));
        $look_resume_jzr = $look_resume_num - $look_resume_num_y;
        $list['wkg'] = array('num' => $look_resume_num, 'jzr' => $look_resume_jzr);

        // 看过我/比昨日
        $jobM = $this->MODEL("job");
        $lookJobWhere = array('com_id' => $this->uid, 'com_status' => 0);
        $look_job_num = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $todayWhere)));
        $look_job_num_y = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $yesterdayWhere)));
        $look_job_jzr = $look_job_num - $look_job_num_y;
        $list['kgw'] = array('num' => $look_job_num, 'jzr' => $look_job_jzr);


        // 下载简历/比昨日
        $downM = $this->MODEL('downresume');
        $downWhere = array('comid' => $this->uid, 'usertype' => $this->usertype);
        $down_num = $downM->getDownNum(array_merge($downWhere, array('downtime' => $todayWhere)));
        $down_num_y = $downM->getDownNum(array_merge($downWhere, array('downtime' => $yesterdayWhere)));
        $fdown_num = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $todayWhere)));
        $fdown_num_y = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $yesterdayWhere)));
        $down_num_sum = intval($down_num) + intval($fdown_num);
        $down_num_sum_y = intval($down_num_y) + intval($fdown_num_y);
        $down_jzr = $down_num_sum - $down_num_sum_y;
        $list['xzjl'] = array('num' => $down_num_sum, 'jzr' => $down_jzr);

        // 投递简历/比昨日
        $tdWhere = array('com_id' => $this->uid, 'type' => array('<>', 3));
        $td_num = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $todayWhere)));
        $td_num_y = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $yesterdayWhere)));
        $td_jzr = $td_num - $td_num_y;
        $list['tdjl'] = array('num' => $td_num, 'jzr' => $td_jzr);


        // 邀请面试/比昨日
        $inviteWhere = array('fid' => $this->uid);
        $invite_num = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $todayWhere)));
        $invite_num_y = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $yesterdayWhere)));
        $invite_jzr = $invite_num - $invite_num_y;
        $list['yqms'] = array('num' => $invite_num, 'jzr' => $invite_jzr);
        foreach ($list as $key => &$val) {
            $val['num'] = intval($val['num']);
        }
        echo json_encode($list);
        exit();
    }

    // 获取统计时间
    function day(){
        if ((int) $_POST['days'] > 0) {
            $days =	intval($_POST['days']);
            $sdate = date('Y-m-d', (time() - ($days - 1) * 86400));
            $edate = date('Y-m-d', time());
        } elseif ($_POST['sdate']) {
            $sdate = $_POST['sdate'] . ' 00:00:00';
            $days =	ceil(abs(time() - strtotime($sdate)) / 86400);
            if ($_POST['edate']) {
                $edate = $_POST['edate'] . ' 23:59:59';
                $days =	ceil(abs(strtotime($edate) - strtotime($sdate)) / 86400);
            }
        } else {// 不传days和起止时间默认查当天
            $days =	1;
            $sdate = $edate = date('Y-m-d');
        }
        return array('sdate' => $sdate, 'days' => $days, 'edate' => $edate);
    }

    // 日数据 近期趋势
    function recentTrend_action(){
        $this->checkOpen();
        $type = $_POST['type'] ? $_POST['type'] : 1;
        $TimeDate =	$this->day();
        $sdate = $TimeDate['sdate'];
        $edate = $TimeDate['edate'];
        // 日期不带时分秒
        if (strlen($edate) == 10) {
            $edate = $edate . ' 23:59:59';
        }
        if (strlen($sdate) == 10) {
            $sdate = $sdate . ' 00:00:00';
        }
        $days =	$TimeDate['days'];
        $defaultData = array();

        $TongjiM = $this->MODEL('tongji');
        if ($type == 1) {// 我看过
            $field = 'datetime';
            $where = array(
                'com_id' => $this->uid,
                'usertype' => $this->usertype,
                'com_status' => 0
            );
            $tbl = 'look_resume';
            $name = '我看过';
        } else if ($type == 2) {// 看过我
            $field = 'datetime';
            $where = array(
                'com_id' => $this->uid,
                'com_status' => 0
            );
            $tbl = 'look_job';
            $name = '看过我';
        } else if ($type == 5) {// 下载简历
            $field = 'downtime';
            $where = array(
                'comid' => $this->uid,
                'usertype' => $this->usertype,
            );
            $tbl = 'down_resume';
            $name = '下载简历';
        } else if ($type == 6) {// 投递简历
            $field = 'datetime';
            $where = array(
                'com_id' => $this->uid,
                'type' => array('<>', 3),
            );
            $tbl = 'userid_job';
            $name = '投递简历';
        } else {// 邀请面试
            $field = 'datetime';
            $where = array(
                'fid' => $this->uid,
            );
            $tbl = 'userid_msg';
            $name = '邀请面试';
        }

        if ($days > 1) {
            $groupBy = "FROM_UNIXTIME(`".$field."`,'%Y-%m-%d')";
            for($i = 0; $i < $days; $i++){
                $date = date('Y-m-d', strtotime($sdate) + 86400 * $i);
                $defaultData[$date] = 0;
            }
        } else {
            $groupBy =  "FROM_UNIXTIME(`".$field."`,'%H:00')";
            for($i = 0; $i < 24; $i++){
                $hour = $i < 10 ? '0' . $i : $i;
                $defaultData[$hour . ':00'] = 0;
            }
        }

        $where[$field] = array(
            array('>=', strtotime($sdate)),
            array('<=', strtotime($edate))
        );
        $where['groupby'] = 'td';
        $where['orderby'] = array('td,desc');

        $statistics = $TongjiM->tjtotal($tbl, $where, array('field' => $groupBy . ' as td,count(*) as cnt'));
        $list = array();
        foreach ($statistics as $v) {
            $list[$v['td']] = $v['cnt'];
        }

        if ($type == 5) {// 下载简历 需要累加免费简历
            $freedownStats = $TongjiM->tjtotal('freedown_resume', $where, array('field' => $groupBy . ' as td,count(*) as cnt'));
            foreach ($freedownStats as $fv) {
                $list[$fv['td']] = !empty($list[$fv['td']]) ? (intval($list[$fv['td']]) + intval($fv['cnt'])) : $fv['cnt'];
            }
        }

        $list = array_merge($defaultData, $list);
        ksort($list);

        $newList = array();
        if ($days > 1) {
            foreach ($list as $lk => $lv) {
                $newList[substr($lk,5)] = $lv; // 此处剔除年，因为没有年第二年12月会排到后边
            }
        }

        echo json_encode(array('name' => $name, 'data' => $newList ? $newList : $list));
        exit();
    }

    /**
     * 查询剩余权益使用量
     */
    function getTcData_action()
    {
        $this->checkOpen();
        $statisM = $this->MODEL('statis');
        $statis = $statisM->vipOver($this->uid, 2);
        // $statis['addjobnum'];
        // $statis['spviewNum'];
        $statis['integral'] == '' && $statis['integral'] = 0;
        if ($statis['rating']) {
            $ratingM = $this->MODEL('rating');
            $rating = $ratingM->getInfo(array('id' => $statis['rating']));
        }
        $statis['zhjf'] = number_format($statis['integral']);
        if ($statis['rating_type'] == 1) {
            $jobNum = $this->obj->select_num('company_job', array('uid' => $this->uid, 'status' => 0));
            $partNum = $this->obj->select_num('partjob', array('uid' => $this->uid, 'status' => 0));
            $zzNum = $jobNum + $partNum;
            $JobNum = $statis['job_num'] - $zzNum;
            $statis['job_num'] = $JobNum > 0 ? $JobNum : 0;
        }
        // 可上架职位数
        $list['ksj'] = array('title' => '可上架职位数', 'tc_num' => $rating['job_num'], 'num' => $statis['job_num'], 'unit' => '个');
        $recUnit = $urgentUnit = $topUnit = $zphUnit = $downUnit = $inviteUnit = $refreshUnit = '';
        $refreshTcNum = $rating['breakjob_num'];
        $inviteTcNum = $rating['interview'];
        $downTcNum = $rating['resume'];
        $zphTcNum = $rating['zph_num'];
        $topTcNum = $rating['top_num'];
        $urgentTcNum = $rating['urgent_num'];
        $recTcNum = $rating['rec_num'];
        if ($statis['rating_type'] == 2) {
            if ($statis['breakjob_num'] == 0) {
                $refreshNum = '-';
            } else {
                $refreshNum = $statis['breakjob_num'];
                $refreshUnit = '次/天';
            }
            $rating['breakjob_num'] == 0 && $refreshTcNum = '-';
            if ($statis['invite_resume'] == 0) {
                $inviteNum = '-';
            } else {
                $inviteNum = $statis['invite_resume'];
                $inviteUnit = '次/天';
            }
            $rating['interview'] == 0 && $inviteTcNum = '-';
            if ($statis['down_resume'] == 0) {
                $downNum = '-';
            } else {
                $downNum = $statis['down_resume'];
                $downUnit = '份/天';
            }
            $rating['resume'] == 0 && $downTcNum = '-';
            if ($statis['zph_num'] == 0) {
                $zphNum = '-';
            } else {
                $zphNum = $statis['zph_num'];
                $zphUnit = '次/天';
            }
            $rating['zph_num'] == 0 && $zphTcNum = '-';
            if ($statis['top_num'] == 0) {
                $topNum = '-';
            } else {
                $topNum = $statis['top_num'];
                $topUnit = '天';
            }
            $rating['top_num'] == 0 && $topTcNum = '-';
            if ($statis['urgent_num'] == 0) {
                $urgentNum = '-';
            } else {
                $urgentNum = $statis['urgent_num'];
                $urgentUnit = '天';
            }
            $rating['urgent_num'] == 0 && $urgentTcNum = '-';
            if ($statis['rec_num'] == 0) {
                $recNum = '-';
            } else {
                $recNum = $statis['rec_num'];
                $recUnit = '天';
            }
            $rating['rec_num'] == 0 && $recTcNum = '-';
        } else {
            $refreshNum = $statis['breakjob_num'];
            $refreshUnit = '次';
            $inviteNum = $statis['invite_resume'];
            $inviteUnit = '次';
            $downNum = $statis['down_resume'];
            $downUnit = '份';
            $zphNum = $statis['zph_num'];
            $zphUnit = '次';
            $topNum = $statis['top_num'];
            $topUnit = '天';
            $urgentNum = $statis['urgent_num'];
            $urgentUnit = '天';
            $recNum = $statis['rec_num'];
            $recUnit = '天';
        }
        // 可刷新职位数
        $list['ksx'] = array('title' => '可刷新职位数', 'tc_num' => $refreshTcNum, 'num' => $refreshNum, 'unit' => $refreshUnit);
        // 可邀请面试数
        $list['kms'] = array('title' => '可邀请面试数', 'tc_num' => $inviteTcNum, 'num' => $inviteNum, 'unit' => $inviteUnit);
        // 可下载简历数
        $list['kxz'] = array('title' => '可下载简历数', 'tc_num' => $downTcNum, 'num' => $downNum, 'unit' => $downUnit);
        
        // 置顶天数
        $list['zd'] = array('title' => '置顶天数', 'tc_num' => $topTcNum, 'num' => $topNum, 'unit' => $topUnit);
        // 紧急天数
        $list['jj'] = array('title' => '紧急天数', 'tc_num' => $urgentTcNum, 'num' => $urgentNum, 'unit' => $urgentUnit);
        // 推荐天数
        $list['tj'] = array('title' => '推荐天数', 'tc_num' => $recTcNum, 'num' => $recNum, 'unit' => $recUnit);
        // 招聘会报名次数
        $list['zph'] = array('title' => '招聘会报名次数', 'tc_num' => $zphTcNum, 'num' => $zphNum, 'unit' => $zphUnit);
       
        foreach ($list as $key => &$val) {
            // 计算进度条宽度
            if ($val['tc_num'] === '-') {
                $val['width'] = '100'; // 不限量
            } else {
                if ($val['tc_num'] == 0) {
                    $val['width'] = '0'; // 套餐数量不足
                } else {
                    if ($val['num'] > 0) {
                        $val['width'] = bcmul(bcdiv($val['num'], $val['tc_num'], 2), 100); // 100-(可用数量/套餐数量*100) 计算剩余宽度
                    } else {
                        $val['width'] = '0'; // 套餐已被使用完
                    }
                }
            }
        }
        echo json_encode($list);
        exit();
    }

    /**
     * 获取数据统计
     */
    function getDataStats_action()
    {
        list($start, $end) = $this->formatTimes();
        $where = array(
            array('>=', $start),
            array('<=', $end)
        );

        // 查看
        // 我看过
        $lookresumeM = $this->MODEL('lookresume');
        $lookResumeWhere = array('com_id' => $this->uid, 'usertype' => $this->usertype, 'com_status' => 0);
        $look_resume_num = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $where)));
        $data['wkg'] = array('num' => $look_resume_num);
        // 看过我
        $jobM = $this->MODEL("job");
        $lookJobWhere = array('com_id' => $this->uid, 'com_status' => 0);
        $look_job_num = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $where)));
        $data['kgw'] = array('num' => $look_job_num);
        // 我登录
        $logWhere = array('uid' => $this->uid, 'usertype' => 2);
        $login_num = $this->MODEL("log")->getLoginlogNum(array_merge($logWhere, array('ctime' => $where)));
        $data['wdl'] = array('num' => $login_num);
        // 简历
        // 下载简历
        $downM = $this->MODEL('downresume');
        $downWhere = array('comid' => $this->uid, 'usertype' => $this->usertype);
        $down_num = $downM->getDownNum(array_merge($downWhere, array('downtime' => $where)));
        $fdown_num = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $where)));
        $down_num_sum = intval($down_num) + intval($fdown_num);
        $data['xzjl'] = array('num' => $down_num_sum);
        // 投递简历
        $tdWhere = array('com_id' => $this->uid, 'type' => array('<>', 3));
        $td_num = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $where)));
        $data['tdjl'] = array('num' => $td_num);
        // 面试
        // 邀请面试
        $inviteWhere = array('fid' => $this->uid);
        $invite_num = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $where)));
        $data['yqms'] = array('num' => $invite_num);

        echo json_encode($data);
        exit();
    }

    /**
     * 获取数据概览 - 右侧图表数据
     */
    function getDataStatsChart_action()
    {
        list($start, $end) = $this->formatTimes();
        $where = array(
            array('>=', $start),
            array('<=', $end)
        );

        if ($_POST['type'] == 3) {
            $fu = '%c';
            $n = 12;
            if (date('Y', $start) == date('Y')) {
                $n = date('n');
            }
            for ($i = 1; $i <= $n; $i++) {
                $dates[] = "{$i}月";
            }
        } else {
            $fu = '%m-%d';
            $day = 24*60*60;
            for ($i = $start; $i <= $end;) {
                $dates[] = date('m-d', $i);
                $i += $day;
            }
        }
        $data['dates'] = $dates;

        // 查看
        // 我看过
        $lookResumeWhere = array('com_id' => $this->uid, 'usertype' => $this->usertype, 'com_status' => 0, 'groupby' => 'dates');
        $look_resume_list = $this->obj->select_all('look_resume', array_merge($lookResumeWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['wkg'] = $this->formatChartData($dates, $look_resume_list);
        // 看过我
        $lookJobWhere = array('com_id' => $this->uid, 'com_status' => 0, 'groupby' => 'dates');
        $look_job_list = $this->obj->select_all('look_job', array_merge($lookJobWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['kgw'] =  $this->formatChartData($dates, $look_job_list);
        // 我登录
        $logWhere = array('uid' => $this->uid, 'usertype' => 2, 'groupby' => 'dates');
        $login_list = $this->obj->select_all('login_log', array_merge($logWhere, array('ctime' => $where)), "FROM_UNIXTIME(ctime,'{$fu}') as dates,count(*) as num");
        $data['wdl'] = $this->formatChartData($dates, $login_list);
        // 简历
        // 下载简历
        $downWhere = array('comid' => $this->uid, 'usertype' => $this->usertype, 'groupby' => 'dates');
        $down_list = $this->obj->select_all('down_resume', array_merge($downWhere, array('downtime' => $where)), "FROM_UNIXTIME(downtime,'{$fu}') as dates,count(*) as num");
        $data['xzjl'] = $this->formatChartData($dates, $down_list);
        // 免费下载简历
        $fdown_list = $this->obj->select_all('freedown_resume', array_merge($downWhere, array('downtime' => $where)), "FROM_UNIXTIME(downtime,'{$fu}') as dates,count(*) as num");
        $fdownData = $this->formatChartData($dates, $fdown_list);
        foreach ($fdownData as $fkey => $fval) {
            $data['xzjl'][$fkey] += $fval; // 下载简历 + 免费下载简历数
        }
        // 投递简历
        $tdWhere = array('com_id' => $this->uid, 'type' => array('<>', 3), 'groupby' => 'dates');
        $td_list = $this->obj->select_all('userid_job', array_merge($tdWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['tdjl'] = $this->formatChartData($dates, $td_list);
        // 面试
        // 邀请面试
        $inviteWhere = array('fid' => $this->uid, 'groupby' => 'dates');
        $invite_list = $this->obj->select_all('userid_msg', array_merge($inviteWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['yqms'] = $this->formatChartData($dates, $invite_list);

        echo json_encode($data);
        exit();
    }

    /**
     * 获取数据概览 - 人才分析
     */
    function getDataStatsRcfx_action()
    {
        list($start, $end) = $this->formatTimes();
        $where = array(
            array('>=', $start),
            array('<=', $end)
        );

        // 查看
        // 我看过
        $lookResumeWhere = array('com_id' => $this->uid, 'usertype' => $this->usertype, 'com_status' => 0, 'groupby' => 'uid');
        $look_resume_list = $this->obj->select_all('look_resume', array_merge($lookResumeWhere, array('datetime' => $where)), "uid");
        $data['wkg'] = $this->getRcfxData($look_resume_list ? array_column($look_resume_list, 'uid') : array());
        // 看过我
        $lookJobWhere = array('com_id' => $this->uid, 'com_status' => 0, 'groupby' => 'uid');
        $look_job_list = $this->obj->select_all('look_job', array_merge($lookJobWhere, array('datetime' => $where)), "uid");
        $data['kgw'] =  $this->getRcfxData($look_job_list ? array_column($look_job_list, 'uid') : array());
        // 简历
        // 下载简历
        $downWhere = array('comid' => $this->uid, 'usertype' => $this->usertype, 'groupby' => 'uid');
        $down_list = $this->obj->select_all('down_resume', array_merge($downWhere, array('downtime' => $where)), "uid");
        $downUidArr = $down_list ? array_column($down_list, 'uid') : array();
        // 免费下载简历
        $fdown_list = $this->obj->select_all('freedown_resume', array_merge($downWhere, array('downtime' => $where)), "uid");
        $data['xzjl'] = $this->getRcfxData(array_merge($downUidArr, $fdown_list ? array_column($fdown_list, 'uid') : array()));
        // 投递简历
        $tdWhere = array('com_id' => $this->uid, 'type' => array('<>', 3), 'groupby' => 'uid');
        $td_list = $this->obj->select_all('userid_job', array_merge($tdWhere, array('datetime' => $where)), "uid");
        $data['tdjl'] = $this->getRcfxData($td_list ? array_column($td_list, 'uid') : array());
        // 面试
        // 邀请面试
        $inviteWhere = array('fid' => $this->uid, 'groupby' => 'uid');
        $invite_list = $this->obj->select_all('userid_msg', array_merge($inviteWhere, array('datetime' => $where)), "uid");
        $data['yqms'] = $this->getRcfxData($invite_list ? array_column($invite_list, 'uid') : array());

        echo json_encode($data);
        exit();
    }

    // 人才分析数据，公用处理
    private function getRcfxData($uidArr)
    {
        if (empty($uidArr)) {
            return array('exp' => array(), 'edu' => array(), 'salary' => array());
        }

        $resumeM = $this->MODEL('resume');
        $cacheM = $this->MODEL('cache');
        $cache = $cacheM->GetCache('user');
        $userclass_name = $cache['userclass_name'];

        // 工作经验
        $user_exp = $cache['userdata']['user_word'];
        $expwhere = array(
            'uid' => array('in', pylode(',', $uidArr)),
            'defaults' => 1,
            'groupby' => 'exp',
            'exp' => array('in', pylode(',', $user_exp))
        );

        $explist = $this->obj->select_all('resume_expect', $expwhere, '`exp`,count(*) as num');

        $exp = array();
        foreach ($explist as $expkey => $expvalue) {
            $exp[$expkey]['name'] = $userclass_name[$expvalue['exp']];
            $exp[$expkey]['value'] = $expvalue['num'];
        }

        // 学历
        $user_edu = $cache['userdata']['user_edu'];
        $eduwhere = array(
            'uid' => array('in', pylode(',', $uidArr)),
            'groupby' => 'edu',
            'edu' => array('in', pylode(',', $user_edu))
        );

        $edulist = $this->obj->select_all('resume', $eduwhere, 'edu,count(*) as num');

        $edu = array();
        foreach ($edulist as $edukey => $eduvalue) {
            $edu[$edukey]['name'] = $userclass_name[$eduvalue['edu']];
            $edu[$edukey]['value'] = $eduvalue['num'];
        }

        // 期望薪资
        $salaryList = array(
            array('name' => '2k-4k', 'val' => '2000,4000'),
            array('name' => '4k-6k', 'val' => '4000,6000'),
            array('name' => '6k-8k', 'val' => '6000,8000'),
            array('name' => '8k-10k', 'val' => '8000,10000'),
            array('name' => '10k以上', 'val' => '10000,20000'),
            array('name' => '20k以上', 'val' => '20000,0'),
        );
        $salary = array();
        foreach ($salaryList as $salaryk => $salaryv) {
            $salarr = @explode(',', $salaryv['val']);

            $where = array('uid' => array('in', pylode(',', $uidArr)), 'defaults' => 1);
            if ($salarr[1] == 0) {
                $where['minsalary'] = array('>=', $salarr[0]);
            } else {
                $where['minsalary'] = array(array('>=', $salarr[0]), array('<', $salarr[1]));
            }

            $num = $resumeM->getExpectNum($where);
            $num > 0 && $salary[] = array('name' => $salaryv['name'], 'value' => $num);
        }

        return compact('exp', 'edu', 'salary');
    }

    // 数据明细
    function getDataStatsDetails_action()
    {
        list($start, $end) = $this->formatTimes();
        $where = array(
            array('>=', $start),
            array('<=', $end)
        );

        if ($_POST['type'] == 3) {
            $fu = '%c';
            $n = 12;
            if (date('Y', $start) == date('Y')) {
                $n = date('n');
            }
            for ($i = 1; $i <= $n; $i++) {
                $dates[] = "{$i}月";
            }
        } else {
            $fu = '%m%d';
            $day = 24*60*60;
            for ($i = $start; $i <= $end;) {
                $dates[] = date('m月d日', $i);
                $i += $day;
            }
        }
        $data['dates'] = $dates;

        // 查看
        // 我看过
        $lookResumeWhere = array('com_id' => $this->uid, 'usertype' => $this->usertype, 'com_status' => 0, 'groupby' => 'dates');
        $look_resume_list = $this->obj->select_all('look_resume', array_merge($lookResumeWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['wkg'] = $this->formatChartData($dates, $look_resume_list);
        // 看过我
        $lookJobWhere = array('com_id' => $this->uid, 'com_status' => 0, 'groupby' => 'dates');
        $look_job_list = $this->obj->select_all('look_job', array_merge($lookJobWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['kgw'] =  $this->formatChartData($dates, $look_job_list);
        // 我登录
        $logWhere = array('uid' => $this->uid, 'usertype' => 2, 'groupby' => 'dates');
        $login_list = $this->obj->select_all('login_log', array_merge($logWhere, array('ctime' => $where)), "FROM_UNIXTIME(ctime,'{$fu}') as dates,count(*) as num");
        $data['wdl'] = $this->formatChartData($dates, $login_list);
        // 简历
        // 下载简历
        $downWhere = array('comid' => $this->uid, 'usertype' => $this->usertype, 'groupby' => 'dates');
        $down_list = $this->obj->select_all('down_resume', array_merge($downWhere, array('downtime' => $where)), "FROM_UNIXTIME(downtime,'{$fu}') as dates,count(*) as num");
        $data['xzjl'] = $this->formatChartData($dates, $down_list);
        // 免费下载简历
        $fdown_list = $this->obj->select_all('freedown_resume', array_merge($downWhere, array('downtime' => $where)), "FROM_UNIXTIME(downtime,'{$fu}') as dates,count(*) as num");
        $fdownData = $this->formatChartData($dates, $fdown_list);
        foreach ($fdownData as $fkey => $fval) {
            $data['xzjl'][$fkey] += $fval; // 下载简历 + 免费下载简历数
        }
        // 投递简历
        $tdWhere = array('com_id' => $this->uid, 'type' => array('<>', 3), 'groupby' => 'dates');
        $td_list = $this->obj->select_all('userid_job', array_merge($tdWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['tdjl'] = $this->formatChartData($dates, $td_list);
        // 面试
        // 邀请面试
        $inviteWhere = array('fid' => $this->uid, 'groupby' => 'dates');
        $invite_list = $this->obj->select_all('userid_msg', array_merge($inviteWhere, array('datetime' => $where)), "FROM_UNIXTIME(datetime,'{$fu}') as dates,count(*) as num");
        $data['yqms'] = $this->formatChartData($dates, $invite_list);

        echo json_encode($data);
        exit();
    }

    private function formatTimes()
    {
        $this->checkOpen();
        if ($_POST['type'] == 2) { // 月
            $start = strtotime($_POST['times']);
            if ($start >= strtotime(date('Y-m'))) { // 开始时间大于当前时间，最大时间取今天
                $end = strtotime(date('Y-m-d') . ' 23:59:59');
            } else {
                $end = strtotime('+1 month ' . $_POST['times']) - 1;
            }
        } elseif ($_POST['type'] == 3) { // 年
            $start = strtotime($_POST['times'] . '-01-01');
            $end = strtotime($_POST['times'] . '-12-31 23:59:59');
        } else { // 日
            $start = strtotime($_POST['times'][0]);
            $end = strtotime($_POST['times'][1] . ' 23:59:59');
        }

        return array($start, $end);
    }

    private function formatChartData($dates, $list)
    {
        $arr = $list ? array_column($list, 'num', 'dates') : array();

        $data = array();
        foreach ($dates as $dval) {
            // 日期格式处理
            $key = str_replace(array('月', '日'), '', $dval);
            if (isset($arr[$key])) {
                $data[] = intval($arr[$key]);
            } else {
                $data[] = 0;
            }
        }

        return $data;
    }

    // 判断后台企业设置-招聘数据开关是否开启
    private function checkOpen()
    {
        if (isset($this->config['com_zpdata']) && $this->config['com_zpdata'] != 1) {
            exit('招聘数据未开放');
        }
    }
}
?>