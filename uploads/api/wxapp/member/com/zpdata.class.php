<?php



class zpdata_controller extends com_controller
{

    private function zpdataTitle($text)
    {
        return yun_auto_t($text);
    }

    private function zpdataUnit($text)
    {
        return $text === '' || $text === '-' ? $text : yun_auto_t($text);
    }

    
    function getTcData_action()
    {
        $this->checkOpen();
        $statisM = $this->MODEL('statis');
        $statis = $statisM->vipOver($this->member['uid'], 2);

        // $statis['addjobnum'];
        // $statis['spviewNum'];

        $statis['integral'] == '' && $statis['integral'] = 0;

        if ($statis['rating']) {
            $ratingM = $this->MODEL('rating');
            $rating = $ratingM->getInfo(array('id' => $statis['rating']));
        }

        $statis['zhjf'] = number_format($statis['integral']);

        if ($statis['rating_type'] == 1) {
            $jobNum = $this->obj->select_num('company_job', array('uid' => $this->member['uid'], 'status' => 0));
            $partNum = $this->obj->select_num('partjob', array('uid' => $this->member['uid'], 'status' => 0));
            $zzNum = $jobNum + $partNum;
            $JobNum = $statis['job_num'] - $zzNum;
            $statis['job_num'] = $JobNum > 0 ? $JobNum : 0;
        }

        // 
        $list[] = array('id' => 'ksj', 'title' => $this->zpdataTitle('member_com_00134'), 'tc_num' => $rating['job_num'], 'num' => $statis['job_num'], 'unit' => $this->zpdataUnit('个'));

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
                $refreshUnit = yun_at('wap_com_00049');
            }
            $rating['breakjob_num'] == 0 && $refreshTcNum = '-';

            if ($statis['invite_resume'] == 0) {
                $inviteNum = '-';
            } else {
                $inviteNum = $statis['invite_resume'];
                $inviteUnit = yun_at('wap_com_00049');
            }
            $rating['interview'] == 0 && $inviteTcNum = '-';

            if ($statis['down_resume'] == 0) {
                $downNum = '-';
            } else {
                $downNum = $statis['down_resume'];
                $downUnit = yun_at('admin_system_00408');
            }
            $rating['resume'] == 0 && $downTcNum = '-';

            if ($statis['zph_num'] == 0) {
                $zphNum = '-';
            } else {
                $zphNum = $statis['zph_num'];
                $zphUnit = yun_at('wap_com_00049');
            }
            $rating['zph_num'] == 0 && $zphTcNum = '-';

            if ($statis['top_num'] == 0) {
                $topNum = '-';
            } else {
                $topNum = $statis['top_num'];
                $topUnit = yun_at('wap_01197');
            }
            $rating['top_num'] == 0 && $topTcNum = '-';

            if ($statis['urgent_num'] == 0) {
                $urgentNum = '-';
            } else {
                $urgentNum = $statis['urgent_num'];
                $urgentUnit = yun_at('wap_01197');
            }
            $rating['urgent_num'] == 0 && $urgentTcNum = '-';

            if ($statis['rec_num'] == 0) {
                $recNum = '-';
            } else {
                $recNum = $statis['rec_num'];
                $recUnit = yun_at('wap_01197');
            }
            $rating['rec_num'] == 0 && $recTcNum = '-';
        } else {
            $refreshNum = $statis['breakjob_num'];
            $refreshUnit = yun_at('wap_01543');

            $inviteNum = $statis['invite_resume'];
            $inviteUnit = yun_at('wap_01543');

            $downNum = $statis['down_resume'];
            $downUnit = yun_at('wap_00929');

            $zphNum = $statis['zph_num'];
            $zphUnit = yun_at('wap_01543');

            $topNum = $statis['top_num'];
            $topUnit = yun_at('wap_01197');

            $urgentNum = $statis['urgent_num'];
            $urgentUnit = yun_at('wap_01197');

            $recNum = $statis['rec_num'];
            $recUnit = yun_at('wap_01197');
        }

        // 
        $list[] = array('id' => 'ksx', 'title' => $this->zpdataTitle('member_com_00136'), 'tc_num' => $refreshTcNum, 'num' => $refreshNum, 'unit' => $this->zpdataUnit($refreshUnit));

        // 
        $list[] = array('id' => 'kms', 'title' => $this->zpdataTitle('member_com_00137'), 'tc_num' => $inviteTcNum, 'num' => $inviteNum, 'unit' => $this->zpdataUnit($inviteUnit));

        // 
        $list[] = array('id' => 'kxz', 'title' => $this->zpdataTitle('member_com_00135'), 'tc_num' => $downTcNum, 'num' => $downNum, 'unit' => $this->zpdataUnit($downUnit));

        // 
        $list[] = array('id' => 'zd', 'title' => $this->zpdataTitle('wap_user_00209'), 'tc_num' => $topTcNum, 'num' => $topNum, 'unit' => $this->zpdataUnit($topUnit));

        // 
        $list[] = array('id' => 'jj', 'title' => $this->zpdataTitle('wap_com_00043'), 'tc_num' => $urgentTcNum, 'num' => $urgentNum, 'unit' => $this->zpdataUnit($urgentUnit));

        // 
        $list[] = array('id' => 'tj', 'title' => $this->zpdataTitle('wap_com_00041'), 'tc_num' => $recTcNum, 'num' => $recNum, 'unit' => $this->zpdataUnit($recUnit));

        // 
        $list[] = array('id' => 'zph', 'title' => $this->zpdataTitle('member_com_00323'), 'tc_num' => $zphTcNum, 'num' => $zphNum, 'unit' => $this->zpdataUnit($zphUnit));
        foreach ($list as $key => &$val) {
            // 
            if ($val['tc_num'] === '-') {
                $val['width'] = '100'; // unlimited
            } else {
                if ($val['tc_num'] == 0) {
                    $val['width'] = '0'; // quota insufficient
                } else {
                    if ($val['num'] > 0) {
                        $val['width'] = bcmul(bcdiv($val['num'], $val['tc_num'], 2), 100);
                    } else {
                        $val['width'] = '0'; // quota exhausted
                    }
                }
            }
        }

        $this->render_json(0, '', compact('list'));
    }

    
    function getTodayData_action()
    {
        $this->checkOpen();
        $today = strtotime('today');
        $yesterday = strtotime('-1 day', $today);
        $todayWhere = array('>=', $today);
        $yesterdayWhere = array(
            array('>=', $yesterday),
            array('<', $today)
        );

        // 
        $lookresumeM = $this->MODEL('lookresume');
        $lookResumeWhere = array('com_id' => $this->member['uid'], 'usertype' => $this->member['usertype'], 'com_status' => 0);
        $look_resume_num = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $todayWhere)));
        $look_resume_num_y = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $yesterdayWhere)));
        $look_resume_jzr = $this->jzrPercentage($look_resume_num, $look_resume_num_y);
        $list[] = array('id' => 'wkg', 'title' => $this->zpdataTitle('member_com_00371'), 'num' => $look_resume_num, 'unit' => $this->zpdataUnit('人'), 'jzr' => $look_resume_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn&page=1&type=3', 'page' => 'pson/pages/commember/resumecolumn/index?type=3');

        // 
        $jobM = $this->MODEL("job");
        $lookJobWhere = array('com_id' => $this->member['uid'], 'com_status' => 0);
        $look_job_num = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $todayWhere)));
        $look_job_num_y = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $yesterdayWhere)));
        $look_job_jzr = $this->jzrPercentage($look_job_num, $look_job_num_y);
        $list[] = array('id' => 'kgw', 'title' => $this->zpdataTitle('member_com_00372'), 'num' => $look_job_num, 'unit' => $this->zpdataUnit('人'), 'jzr' => $look_job_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=look_job', 'page' => 'pson/pages/commember/lookjob/index');

        // 
        $downM = $this->MODEL('downresume');
        $downWhere = array('comid' => $this->member['uid'], 'usertype' => $this->member['usertype']);
        $down_num = $downM->getDownNum(array_merge($downWhere, array('downtime' => $todayWhere)));
        $down_num_y = $downM->getDownNum(array_merge($downWhere, array('downtime' => $yesterdayWhere)));
        $fdown_num = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $todayWhere)));
        $fdown_num_y = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $yesterdayWhere)));
        $down_num_sum = intval($down_num) + intval($fdown_num);
        $down_num_sum_y = intval($down_num_y) + intval($fdown_num_y);
        $down_jzr = $down_num_sum - $down_num_sum_y;
        $list[] = array('id' => 'xzjl', 'title' => $this->zpdataTitle('wap_00451'), 'num' => $down_num_sum, 'unit' => $this->zpdataUnit('人'), 'jzr' => $down_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn', 'page' => 'pson/pages/commember/resumecolumn/index');

        // 
        $tdWhere = array('com_id' => $this->member['uid'], 'type' => array('<>', 3));
        $td_num = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $todayWhere)));
        $td_num_y = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $yesterdayWhere)));
        $td_jzr = $this->jzrPercentage($td_num, $td_num_y);
        $list[] = array('id' => 'tdjl', 'title' => $this->zpdataTitle('wap_com_00235'), 'num' => $td_num, 'unit' => $this->zpdataUnit('人'), 'jzr' => $td_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=hr', 'page' => 'pson/pages/commember/hr/index');

        // 
        $inviteWhere = array('fid' => $this->member['uid']);
        $invite_num = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $todayWhere)));
        $invite_num_y = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $yesterdayWhere)));
        $invite_jzr = $this->jzrPercentage($invite_num, $invite_num_y);
        $list[] = array('id' => 'yqms', 'title' => $this->zpdataTitle('resume_00029'), 'num' => $invite_num, 'unit' => $this->zpdataUnit('人'), 'jzr' => $invite_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=invite', 'page' => 'pson/pages/commember/invite/index');

        foreach ($list as $key => &$val) {
            $val['num'] = intval($val['num']);
        }

        $this->render_json(0, '', compact('list'));
    }

    /**
     * @param mixed $tData today count
     * @param mixed $yData yesterday count
     */
    function jzrPercentage($tData, $yData)
    {
        // $ratio = 0;
        // if ($tData && !$yData) {
        //     $ratio = 100;
        // } elseif (!$tData && $yData) {
        //     $ratio = -100;
        // } elseif ($tData && $yData) {
        // (-)/*100
        //     $ratio = bcmul(bcdiv(bcsub($tData, $yData), $yData, 2), 100);
        // }
        //
        // return $ratio;

        return $tData - $yData; // delta
    }

    
    function getWeekData_action()
    {
        $this->checkOpen();
        $today = strtotime('today');

        $times = !empty($_POST['times']) ? intval($_POST['times']) : 1;
        if ($times == 4) {
            $start = strtotime('-1 month', $today); // last month
        } else {
            $days  = $times * 7 - 1; // include today
            $start = strtotime('-' . $days . ' day', $today); // seven days incl. today
        }
        $dates = date('Y.m.d', $start) . '-' . date('Y.m.d', $today); // display date range

        $where = array(
            array('>=', $start),
            array('<', strtotime('+1 day', $today))
        );

        // 
        // 
        $lookresumeM = $this->MODEL('lookresume');
        $lookResumeWhere = array('com_id' => $this->member['uid'], 'usertype' => $this->member['usertype'], 'com_status' => 0);
        $look_resume_num = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $where)));
        $lookData[] = array('id' => 'wkg', 'title' => $this->zpdataTitle('member_com_00371'), 'num' => $look_resume_num, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn&page=1&type=3', 'page' => 'pson/pages/commember/resumecolumn/index?type=3');
        // 
        $jobM = $this->MODEL("job");
        $lookJobWhere = array('com_id' => $this->member['uid'], 'com_status' => 0);
        $look_job_num = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $where)));
        $lookData[] = array('id' => 'kgw', 'title' => $this->zpdataTitle('member_com_00372'), 'num' => $look_job_num, 'wap_url' => Url('wap') . 'member/index.php?c=look_job', 'page' => 'pson/pages/commember/lookjob/index');
        // 
        $logWhere = array('uid' => $this->member['uid'], 'usertype' => 2);
        $login_num = $this->MODEL("log")->getLoginlogNum(array_merge($logWhere, array('ctime' => $where)));
        $lookData[] = array('id' => 'wdl', 'title' => $this->zpdataTitle('member_com_00370'), 'num' => $login_num, 'wap_url' => '', 'page' => '');

        // 
        // 
        $downM = $this->MODEL('downresume');
        $downWhere = array('comid' => $this->member['uid'], 'usertype' => $this->member['usertype']);
        $down_num = $downM->getDownNum(array_merge($downWhere, array('downtime' => $where)));
        $fdown_num = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $where)));
        $down_num_sum = intval($down_num) + intval($fdown_num);
        $resumeData[] = array('id' => 'xzjl', 'title' => $this->zpdataTitle('wap_00451'), 'num' => $down_num_sum, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn', 'page' => 'pson/pages/commember/resumecolumn/index');
        // 
        $tdWhere = array('com_id' => $this->member['uid'], 'type' => array('<>', 3));
        $td_num = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $where)));
        $resumeData[] = array('id' => 'tdjl', 'title' => $this->zpdataTitle('wap_com_00235'), 'num' => $td_num, 'wap_url' => Url('wap') . 'member/index.php?c=hr', 'page' => 'pson/pages/commember/hr/index');
        // 
        array_unshift($resumeData, array('id' => 'total', 'title' => $this->zpdataTitle('wap_com_00426'), 'num' => intval($td_num)));

        // 
        // 
        $inviteWhere = array('fid' => $this->member['uid']);
        $invite_num = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $where)));
        $msData[] = array('id' => 'yqms', 'title' => $this->zpdataTitle('resume_00029'), 'num' => $invite_num, 'wap_url' => Url('wap') . 'member/index.php?c=invite', 'page' => 'pson/pages/commember/invite/index');
        // 
        // $msData[] = array('title'=>yun_at('wap_01056'), 'num' => $accept_num, 'wap_url' => Url('wap') . 'member/index.php?c=invite', 'page' => 'pson/pages/commember/invite/index');

        $this->render_json(0, '', compact('lookData', 'resumeData', 'msData', 'dates'));
    }

    // -
    private function checkOpen()
    {
        if (isset($this->config['com_zpdata']) && $this->config['com_zpdata'] != 1) {
            $this->render_json(403, yun_at('wap_01284'));
        }
    }
}
