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

class zpdata_controller extends com_controller
{

    /**
     * 查询剩余权益使用量
     */
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

        // 可上架职位数
        $list[] = array('title' => '可上架职位数', 'tc_num' => $rating['job_num'], 'num' => $statis['job_num'], 'unit' => '个');

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
        $list[] = array('title' => '可刷新职位数', 'tc_num' => $refreshTcNum, 'num' => $refreshNum, 'unit' => $refreshUnit);

        // 可邀请面试数
        $list[] = array('title' => '可邀请面试数', 'tc_num' => $inviteTcNum, 'num' => $inviteNum, 'unit' => $inviteUnit);

        // 可下载简历数
        $list[] = array('title' => '可下载简历数', 'tc_num' => $downTcNum, 'num' => $downNum, 'unit' => $downUnit);

        // 置顶天数
        $list[] = array('title' => '置顶天数', 'tc_num' => $topTcNum, 'num' => $topNum, 'unit' => $topUnit);

        // 紧急天数
        $list[] = array('title' => '紧急天数', 'tc_num' => $urgentTcNum, 'num' => $urgentNum, 'unit' => $urgentUnit);

        // 推荐天数
        $list[] = array('title' => '推荐天数', 'tc_num' => $recTcNum, 'num' => $recNum, 'unit' => $recUnit);

        // 招聘会报名次数
        $list[] = array('title' => '招聘会报名次数', 'tc_num' => $zphTcNum, 'num' => $zphNum, 'unit' => $zphUnit);
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

        $this->render_json(0, '', compact('list'));
    }

    /**
     * 获取今日数据
     */
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

        // 我看过/比昨日
        $lookresumeM = $this->MODEL('lookresume');
        $lookResumeWhere = array('com_id' => $this->member['uid'], 'usertype' => $this->member['usertype'], 'com_status' => 0);
        $look_resume_num = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $todayWhere)));
        $look_resume_num_y = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $yesterdayWhere)));
        $look_resume_jzr = $this->jzrPercentage($look_resume_num, $look_resume_num_y);
        $list[] = array('title' => '我看过', 'num' => $look_resume_num, 'unit' => '人', 'jzr' => $look_resume_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn&page=1&type=3', 'page' => 'pson/pages/commember/resumecolumn/index?type=3');

        // 看过我/比昨日
        $jobM = $this->MODEL("job");
        $lookJobWhere = array('com_id' => $this->member['uid'], 'com_status' => 0);
        $look_job_num = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $todayWhere)));
        $look_job_num_y = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $yesterdayWhere)));
        $look_job_jzr = $this->jzrPercentage($look_job_num, $look_job_num_y);
        $list[] = array('title' => '看过我', 'num' => $look_job_num, 'unit' => '人', 'jzr' => $look_job_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=look_job', 'page' => 'pson/pages/commember/lookjob/index');

        // 下载简历/比昨日
        $downM = $this->MODEL('downresume');
        $downWhere = array('comid' => $this->member['uid'], 'usertype' => $this->member['usertype']);
        $down_num = $downM->getDownNum(array_merge($downWhere, array('downtime' => $todayWhere)));
        $down_num_y = $downM->getDownNum(array_merge($downWhere, array('downtime' => $yesterdayWhere)));
        $fdown_num = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $todayWhere)));
        $fdown_num_y = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $yesterdayWhere)));
        $down_num_sum = intval($down_num) + intval($fdown_num);
        $down_num_sum_y = intval($down_num_y) + intval($fdown_num_y);
        $down_jzr = $down_num_sum - $down_num_sum_y;
        $list[] = array('title' => '下载简历', 'num' => $down_num_sum, 'unit' => '人', 'jzr' => $down_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn', 'page' => 'pson/pages/commember/resumecolumn/index');

        // 投递简历/比昨日
        $tdWhere = array('com_id' => $this->member['uid'], 'type' => array('<>', 3));
        $td_num = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $todayWhere)));
        $td_num_y = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $yesterdayWhere)));
        $td_jzr = $this->jzrPercentage($td_num, $td_num_y);
        $list[] = array('title' => '投递简历', 'num' => $td_num, 'unit' => '人', 'jzr' => $td_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=hr', 'page' => 'pson/pages/commember/hr/index');

        // 邀请面试/比昨日
        $inviteWhere = array('fid' => $this->member['uid']);
        $invite_num = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $todayWhere)));
        $invite_num_y = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $yesterdayWhere)));
        $invite_jzr = $this->jzrPercentage($invite_num, $invite_num_y);
        $list[] = array('title' => '邀请面试', 'num' => $invite_num, 'unit' => '人', 'jzr' => $invite_jzr, 'wap_url' => Url('wap') . 'member/index.php?c=invite', 'page' => 'pson/pages/commember/invite/index');

        foreach ($list as $key => &$val) {
            $val['num'] = intval($val['num']);
        }

        $this->render_json(0, '', compact('list'));
    }

    /**
     * 较昨日百分比计算
     * @param $tData 今日数据
     * @param $yData 昨日数据
     */
    function jzrPercentage($tData, $yData)
    {
        // $ratio = 0;
        // if ($tData && !$yData) {
        //     $ratio = 100;
        // } elseif (!$tData && $yData) {
        //     $ratio = -100;
        // } elseif ($tData && $yData) {
        //     // (今日数据-昨日数据)/昨日数据*100
        //     $ratio = bcmul(bcdiv(bcsub($tData, $yData), $yData, 2), 100);
        // }
        //
        // return $ratio;

        return $tData - $yData; // 计算差集
    }

    /**
     * 获取周数据
     */
    function getWeekData_action()
    {
        $this->checkOpen();
        $today = strtotime('today');

        $times = !empty($_POST['times']) ? intval($_POST['times']) : 1;
        if ($times == 4) {
            $start = strtotime('-1 month', $today); // 近一月
        } else {
            $days  = $times * 7 - 1; // 减少1天，包含上当天
            $start = strtotime('-' . $days . ' day', $today); // 包含今天，共七天
        }
        $dates = date('Y.m.d', $start) . '-' . date('Y.m.d', $today); // 前台用来展示时间段

        $where = array(
            array('>=', $start),
            array('<', strtotime('+1 day', $today))
        );

        // 查看/沟通
        // 我看过
        $lookresumeM = $this->MODEL('lookresume');
        $lookResumeWhere = array('com_id' => $this->member['uid'], 'usertype' => $this->member['usertype'], 'com_status' => 0);
        $look_resume_num = $lookresumeM->getLookNum(array_merge($lookResumeWhere, array('datetime' => $where)));
        $lookData[] = array('title' => '我看过', 'num' => $look_resume_num, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn&page=1&type=3', 'page' => 'pson/pages/commember/resumecolumn/index?type=3');
        // 看过我
        $jobM = $this->MODEL("job");
        $lookJobWhere = array('com_id' => $this->member['uid'], 'com_status' => 0);
        $look_job_num = $jobM->getLookJobNum(array_merge($lookJobWhere, array('datetime' => $where)));
        $lookData[] = array('title' => '看过我', 'num' => $look_job_num, 'wap_url' => Url('wap') . 'member/index.php?c=look_job', 'page' => 'pson/pages/commember/lookjob/index');
        // 我登录
        $logWhere = array('uid' => $this->member['uid'], 'usertype' => 2);
        $login_num = $this->MODEL("log")->getLoginlogNum(array_merge($logWhere, array('ctime' => $where)));
        $lookData[] = array('title' => '我登录', 'num' => $login_num, 'wap_url' => '', 'page' => '');

        // 简历及联系方式
        // 下载简历
        $downM = $this->MODEL('downresume');
        $downWhere = array('comid' => $this->member['uid'], 'usertype' => $this->member['usertype']);
        $down_num = $downM->getDownNum(array_merge($downWhere, array('downtime' => $where)));
        $fdown_num = $downM->getFreeDownNum(array_merge($downWhere, array('downtime' => $where)));
        $down_num_sum = intval($down_num) + intval($fdown_num);
        $resumeData[] = array('title' => '下载简历', 'num' => $down_num_sum, 'wap_url' => Url('wap') . 'member/index.php?c=resumecolumn', 'page' => 'pson/pages/commember/resumecolumn/index');
        // 投递简历
        $tdWhere = array('com_id' => $this->member['uid'], 'type' => array('<>', 3));
        $td_num = $jobM->getSqJobNum(array_merge($tdWhere, array('datetime' => $where)));
        $resumeData[] = array('title' => '投递简历', 'num' => $td_num, 'wap_url' => Url('wap') . 'member/index.php?c=hr', 'page' => 'pson/pages/commember/hr/index');
        // 合计
        array_unshift($resumeData, array('title' => '合计', 'num' => intval($td_num)));

        // 面试
        // 邀请面试
        $inviteWhere = array('fid' => $this->member['uid']);
        $invite_num = $jobM->getYqmsNum(array_merge($inviteWhere, array('datetime' => $where)));
        $msData[] = array('title' => '邀请面试', 'num' => $invite_num, 'wap_url' => Url('wap') . 'member/index.php?c=invite', 'page' => 'pson/pages/commember/invite/index');
        // 接受面试
        // $acceptWhere = array('fid' => $this->member['uid'], 'is_browse' => 3);
        // $accept_num = $jobM->getYqmsNum(array_merge($acceptWhere, array('datetime' => $where)));
        // $msData[] = array('title' => '接受面试', 'num' => $accept_num, 'wap_url' => Url('wap') . 'member/index.php?c=invite', 'page' => 'pson/pages/commember/invite/index');

        $this->render_json(0, '', compact('lookData', 'resumeData', 'msData', 'dates'));
    }

    // 判断后台企业设置-招聘数据开关是否开启
    private function checkOpen()
    {
        if (isset($this->config['com_zpdata']) && $this->config['com_zpdata'] != 1) {
            $this->render_json(403, '招聘数据未开放');
        }
    }
}