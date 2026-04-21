<?php
/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class tongji_model extends model
{

    public function tjtotal($table, $whereData = array(), $data = array())
    {

        $data['field'] = empty($data['field']) ? '*' : $data['field'];
        return $this->select_all($table, $whereData, $data['field']);
    }

    public function tjtotalnum($table, $whereData = array(), $data = array())
    {
        return $this->select_num($table, $whereData);
    }

    public function tjtonce($table, $whereData = array(), $data = array())
    {

        $data['field'] = empty($data['field']) ? '*' : $data['field'];
        return $this->select_once($table, $whereData, $data['field']);
    }

    function getTj($Tablename, $Post, $Field, $Where = array(), $CountField = 'count(*) as count')
    {

        $TimeDate = $this->TimeDate($Field, $CountField, $Post);

        if ($Field == 'r_time') {
            if ($Tablename == 'resume_expect') {

                $Tablename = 'resume_refresh_log';
            } else if ($Tablename == 'company_job') {

                $Tablename = 'job_refresh_log';
                $Where['type'] = 1;
            }
        }
        $Stats = $this->select_all($Tablename, array_merge($Where, $TimeDate['FieldWhere']), $TimeDate['FieldFormat']);
        return $this->TjInfo($Stats, $TimeDate);
    }


    function TjInfo($Stats, $TimeDate)
    {

        if (is_array($Stats)) {
            $AllNum = 0;
            $Date = array();

            foreach ($Stats as $key => $value) {

                $AllNum += $value['count'];
                $Date[$value['tjtime']] = $value['count'];
            }
            foreach ($TimeDate['DateList'] as $key => $value) {

                if ($Date[$value['onday']] < 1) {
                    $Date[$value['onday']] = 0;
                }

                if (intval($_POST['days']) > 3) {

                    $value['date'] = intval($value['date']) . '月';
                }
                $List[$value['onday']] = array('tjtime' => $value['tjtime'], 'count' => $Date[$value['onday']], 'date' => $value['date']);
            }
        }

        $List = array_values($List);
        krsort($List);
        $List = array_values($List);
        return array('allnum' => $AllNum, 'list' => $List, 'timedate' => $TimeDate);
    }

    //时间线获取
    function TimeDate($field, $countfield = 'count(*) as count', $Post)
    {
        if (!$Post['days'] && !$Post['time']) {

            $Post['days'] = 1;
        }

        $day = intval($Post['days']);

        $DateList = $DateWhere = $FieldWhere = array();

        $field_u = "`$field`";
        if ($field == 'sendTime') {
            $field_u = "{$field}/1000";
        }

        if ($Post['days'] == '-1') { // 昨天 以24小时计算

            $days = 24;
            $sdate = strtotime('today') - 86400;
            $edate = strtotime(date('Y-m-d'));

            $FieldFormat = "FROM_UNIXTIME($field_u,'%k') as tjtime,$countfield";

            for ($i = 0; $i <= $days; $i++) {

                $DateList[$i]['tjtime'] = $i;
                $DateList[$i]['date'] = $i . ":00";
                $DateList[$i]['onday'] = $i;
            }
        } elseif ($day == 1) { // 今天 以24小时计算

            $days = 24;
            $sdate = strtotime(date('Y-m-d'));
            $edate = time();

            $FieldFormat = "FROM_UNIXTIME($field_u,'%k') as tjtime,$countfield";

            for ($i = 0; $i <= $days; $i++) {

                $DateList[$i]['tjtime'] = $i;
                $DateList[$i]['date'] = $i . ":00";
                $DateList[$i]['onday'] = $i;
            }

        } elseif ($day > 1 && $day < 4) {

            $FieldFormat = "FROM_UNIXTIME($field_u,'%m%d') as tjtime,$countfield";

            if ($day == 2) {

                $sdate = strtotime('-1 week');
                $days = 7;
            } else if ($day == 3) {

                $sdate = strtotime('-1 month');
                $days = 30;
            }

            $edate = time();

            for ($i = 0; $i <= $days; $i++) {

                $onday = date("md", strtotime(' -' . $i . 'day'));
                $tjtime = date('m-d', strtotime(' -' . $i . 'day'));
                $date = date('m-d', strtotime(' -' . $i . 'day'));

                $DateList[$i]['tjtime'] = $tjtime;
                $DateList[$i]['date'] = $date;
                $DateList[$i]['onday'] = $onday;
            }
        } elseif ($day > 3 && $day < 6) {

            if ($day == 4) {

                $days = 6;
            } elseif ($day == 5) {

                $days = 12;
            }

            $sdate = strtotime('-' . $days . ' month');
            $edate = time();

            $nowYear = date('y');

            $FieldFormat = "FROM_UNIXTIME($field_u,'%m') as tjtime,$countfield";

            for ($i = 0; $i <= $days; $i++) {

                $year = date("y", strtotime(' -' . $i . 'month'));
                $onday = date("m", strtotime(' -' . $i . 'month'));
                $tjtime = date('m', strtotime(' -' . $i . 'month'));
                $date = date('m', strtotime(' -' . $i . 'month'));

                $DateList[$i]['tjtime'] = $tjtime;
                $DateList[$i]['date'] = $date;
                $DateList[$i]['onday'] = $onday;
                if (intval($year) < intval($nowYear)) {
                    $DateList[$i]['year'] = $year;
                }
            }
        } elseif ($Post['time']) {

            $timeArr = $Post['time'];

            $timeStart = date('Y-m-d', $timeArr[0] / 1000);
            $timeEnd = date('Y-m-d', $timeArr[1] / 1000);
            $sdate = strtotime($timeStart . " 00:00:00");
            $edate = strtotime($timeEnd . " 23:59:59");

            $days = ceil(abs($edate - $sdate) / 86400);


            $FieldFormat = "FROM_UNIXTIME($field_u,'%m%d') as tjtime,$countfield";

            for ($i = 0; $i < $days; $i++) {

                $t = $edate - $i * 86400;
                $onday = date("md", $t);
                $tjtime = date('m-d', $t);
                $date = date('m-d', $t);

                $DateList[$t]['tjtime'] = $tjtime;
                $DateList[$t]['date'] = $date;
                $DateList[$t]['onday'] = $onday;
            }
        }

        $DateWhere[$field][] = array('>=', $sdate);
        $DateWhere[$field][] = array('<=', $edate);

        $FieldWhere['groupby'] = 'tjtime';
        $FieldWhere['orderby'] = 'tjtime,desc';

        return array(
            'days' => $days,
            'FieldWhere' => array_merge($DateWhere, $FieldWhere),
            'FieldFormat' => $FieldFormat,
            'DateList' => $DateList,
            'DateWhere' => $DateWhere
        );
    }


    //统计 前十排行
    function TopTen($Tablename, $Where, $Field, $Type, $Limit = '10', $CountField = "count(*) AS count")
    {

        $topwhere['groupby'] = $Field;
        $topwhere['orderby'] = array('count,desc');
        $topwhere['limit'] = $Limit;

        $List = $this->select_all($Tablename, array_merge($Where, $topwhere), $CountField . "," . $Field);

        foreach ($List as $key => $value) {

            $Fields[] = $value[$Field];
            $Count[$value[$Field]] = $value['count'];
        }

        if ($Type == 'job') {//统计职位信息

            $jobwhere['id'] = array('in', pylode(',', $Fields));

            $FieldList = $this->select_all("company_job", $jobwhere, "`id`,`uid`,`name`");

            foreach ($FieldList as $key => $value) {

                $value['count'] = $Count[$value['id']];
                $NewList[$value['id']] = $value;
            }

            foreach ($List as $key => $value) {

                $TopList[$key] = $NewList[$value[$Field]];
            }
        } elseif ($Type == 'company') {

            $comwhere['uid'] = array('in', pylode(',', $Fields));

            $FieldList = $this->select_all("company", $comwhere, "`uid`,`name`");


            foreach ($FieldList as $key => $value) {

                $value['count'] = $Count[$value['uid']];
                $NewList[$value['uid']] = $value;
            }

            foreach ($List as $key => $value) {
                if (!empty($NewList[$value[$Field]])) {
                    $TopList[$key] = $NewList[$value[$Field]];
                }

            }
        } elseif ($Type == 'expect') {

            include PLUS_PATH . 'city.cache.php';
            include PLUS_PATH . 'job.cache.php';
            include PLUS_PATH . 'user.cache.php';

            $ewhere['id'] = array('in', pylode(',', $Fields));

            $FieldList = $this->select_all("resume_expect", $ewhere, "`id`,`uid`,`uname`,`job_classid`,`city_classid`,`exp`,`edu`");

            foreach ($FieldList as $key => $value) {

                $value['count'] = $Count[$value['id']];
                $value['eduname'] = $userclass_name[$value['edu']];
                $value['expname'] = $userclass_name[$value['exp']];
                if ($value['job_classid']) {
                    $jobclassname = array();
                    $jobclassid = @explode(',', $value['job_classid']);
                    foreach ($jobclassid as $k => $v) {
                        if ($v) {
                            $jobclassname[] = $job_name[$v];
                        }
                    }
                    $value['jobclassname'] = implode(',', $jobclassname);
                }
                if ($value['city_classid']) {
                    $cityclassname = array();
                    $cityclassid = @explode(',', $value['city_classid']);
                    foreach ($cityclassid as $k => $v) {
                        if ($v) {
                            $cityclassname[] = $city_name[$v];
                        }
                    }
                    $value['cityclassname'] = implode(',', $cityclassname);
                }
                $NewList[$value['id']] = $value;
            }

            foreach ($List as $key => $value) {
                if (!empty($NewList[$value[$Field]])) {
                    $TopList[$key] = $NewList[$value[$Field]];
                }

            }
        } elseif ($Type == 'resume') {

            $rwhere['uid'] = array('in', pylode(',', $Fields));

            $FieldList = $this->select_all("resume", $rwhere, "`uid`,`name`,`sex`,`exp`,`edu`");

            $ExpectList = $this->select_all("resume_expect", $rwhere, "`id`,`uid`,`defaults`");

            foreach ($FieldList as $key => $value) {

                $value['count'] = $Count[$value['uid']];
                $NewList[$value['uid']] = $value;
            }
            foreach ($List as $key => $value) {
                if (!empty($NewList[$value[$Field]])) {
                    $TopList[$key] = $NewList[$value[$Field]];
                }
                foreach ($ExpectList as $val) {
                    if ($val['uid'] == $value['uid'] && $val['defaults'] == 1) {
                        $TopList[$key]['eid'] = $val['id'];
                    }
                }
            }
        } elseif ($Type == 'order') {

            $mwhere['uid'] = array('in', pylode(',', $Fields));

            $FieldList = $this->select_all("member", $mwhere, "`uid`,`username`,`usertype`");

            foreach ($FieldList as $key => $value) {

                $value['count'] = $Count[$value['uid']];
                $NewList[$value['uid']] = $value;
            }

            foreach ($List as $key => $value) {
                if (!empty($NewList[$value[$Field]])) {
                    $TopList[$key] = $NewList[$value[$Field]];
                }

            }
        }
        if ($Type == 'ad') {//统计广告点击量

            $adclickwhere['aid'] = array('in', pylode(',', $Fields));

            $FieldList = $this->select_all("adclick", $adclickwhere, "`aid`");

            $adwhere['id'] = array('in', pylode(',', $Fields));

            $adList = $this->select_all("ad", $adwhere);
            foreach ($adList as $value) {
                $adLists[$value['id']] = $value['ad_name'];
            }
            foreach ($FieldList as $key => $value) {

                $value['count'] = $Count[$value['aid']];
                $value['name'] = $adLists[$value['aid']];
                $NewList[$value['aid']] = $value;
            }

            foreach ($List as $key => $value) {

                $TopList[$key] = $NewList[$value[$Field]];
            }
        }
        return $TopList;
    }

    //完整数据分析，获取行业、薪资区间、区域等数据进行统一分析
    function DataTj($Type, $Where, $Tablename, $Field)
    {

        include PLUS_PATH . 'city.cache.php';
        include PLUS_PATH . 'industry.cache.php';
        include PLUS_PATH . 'job.cache.php';
        include PLUS_PATH . 'com.cache.php';
        include PLUS_PATH . 'user.cache.php';
        include CONFIG_PATH . 'db.data.php';

        include_once('cache.model.php');
        $cacheM =   new cache_model($this->db, $this->def);
        $cache  =   $cacheM->GetCache('com');
        $List   =   $this->select_all($Tablename, $Where, $Field);

        if (is_array($List)) {

            $Count = array();

            foreach ($List as $key => $value) {

                $Fields[] = $value[$Field];
            }
            if ($Type == 'resume_expect') {

                //查询职位信息
                $ewhere['id'] = array('in', pylode(',', $Fields));

                $citywhere['eid'] = array('in', pylode(',', $Fields));

                $ResumeList = $this->select_all("resume_expect", $ewhere, "`id`,`sex`,`edu`,`exp`,`job_classid`,`maxsalary`,`minsalary`,`source`");

                $ResumeCityList = $this->select_all("resume_cityclass", $citywhere, "`eid`,`provinceid`,`cityid`,`three_cityid`");
                $jobClassEnd = array();
                foreach ($job_index as $val) {

                    if (is_array($job_type[$val])) {
                        $jobClassAll[$val][] = $val;
                        foreach ($job_type[$val] as $v) {

                            $jobClassAll[$val] = array_merge($jobClassAll[$val], $job_type[$val]);

                            if (is_array($job_type[$v])) {

                                $jobClassAll[$val] = array_merge($jobClassAll[$val], $job_type[$v]);

                            }
                        }
                    }
                }
                foreach ($jobClassAll as $key => $value) {
                    if (is_array($value)) {
                        foreach ($value as $v) {
                            $jobClassEnd[$v] = $key;
                        }
                    }
                }
                foreach ($ResumeCityList as $val) {

                    $NewResumeCityList[$val['eid']] = $val;

                }

                foreach ($ResumeList as $key => $value) {
                    $Count['sex'][$value['sex']]['count']++;
                    $Count['sex'][$value['sex']]['name'] = $arr_data['sex'][$value['sex']];

                    $Count['source'][$value['source']]['count']++;
                    $Count['source'][$value['source']]['name'] = $arr_data['source'][$value['source']];

                    $Count['edu'][$value['edu']]['count']++;
                    $Count['edu'][$value['edu']]['name'] = $userclass_name[$value['edu']];

                    $Count['exp'][$value['exp']]['count']++;
                    $Count['exp'][$value['exp']]['name'] = $userclass_name[$value['exp']];
                    if ($value['job_classid']) {
                        $jobclassid = @explode(',', $value['job_classid']);
                        foreach ($jobclassid as $k => $v) {
                            $classid = $jobClassEnd[$v];
                            if ($classid) {
                                $Count['job1'][$classid]['count']++;
                                $Count['job1'][$classid]['name'] = $job_name[$classid];
                            }
                        }
                    }


                    $NewResumeCity = $NewResumeCityList[$value['id']];
                    if ($NewResumeCity) {

                        $Count['provinceid'][$NewResumeCity['provinceid']]['count']++;
                        $Count['provinceid'][$NewResumeCity['provinceid']]['name'] = $city_name[$NewResumeCity['provinceid']];

                        $Count['cityid'][$NewResumeCity['cityid']]['count']++;
                        $Count['cityid'][$NewResumeCity['cityid']]['name'] = $city_name[$NewResumeCity['cityid']];

                        $Count['three_cityid'][$NewResumeCity['three_cityid']]['count']++;
                        $Count['three_cityid'][$NewResumeCity['three_cityid']]['name'] = $city_name[$NewResumeCity['three_cityid']];
                    }

                    if ($value['maxsalary'] > 0) {
                        if ($value['maxsalary'] <= 2000) {
                            $Count['salary']['2000以下']['count']++;
                            $Count['salary']['2000以下']['name'] = '2000以下';
                        }
                        if ($value['maxsalary'] > 2000 && $value['maxsalary'] <= 4000) {
                            $Count['salary']['2000-4000']['count']++;
                            $Count['salary']['2000-4000']['name'] = '2000-4000';
                        }
                        if ($value['maxsalary'] > 4000 && $value['maxsalary'] <= 6000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['maxsalary'] > 6000 && $value['maxsalary'] <= 8000) {
                            $Count['salary']['6000-8000']['count']++;
                            $Count['salary']['6000-8000']['name'] = '6000-8000';
                        }
                        if ($value['maxsalary'] > 8000 && $value['maxsalary'] <= 10000) {
                            $Count['salary']['8000-10000']['count']++;
                            $Count['salary']['8000-10000']['name'] = '8000-10000';
                        }
                        if ($value['maxsalary'] > 10000) {
                            $Count['salary']['10000以上']['count']++;
                            $Count['salary']['10000以上']['name'] = '10000以上';
                        }
                    } else {
                        if ($value['minsalary'] <= 2000) {
                            $Count['salary']['2000以下']['count']++;
                            $Count['salary']['2000以下']['name'] = '2000以下';
                        }
                        if ($value['minsalary'] > 2000 && $value['minsalary'] <= 4000) {
                            $Count['salary']['2000-4000']['count']++;
                            $Count['salary']['2000-4000']['name'] = '2000-4000';
                        }
                        if ($value['minsalary'] > 4000 && $value['minsalary'] <= 6000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['minsalary'] > 6000 && $value['minsalary'] <= 8000) {
                            $Count['salary']['6000-8000']['count']++;
                            $Count['salary']['6000-8000']['name'] = '6000-8000';
                        }
                        if ($value['minsalary'] > 8000 && $value['minsalary'] <= 10000) {
                            $Count['salary']['8000-10000']['count']++;
                            $Count['salary']['8000-10000']['name'] = '8000-10000';
                        }
                        if ($value['minsalary'] > 10000) {
                            $Count['salary']['10000以上']['count']++;
                            $Count['salary']['10000以上']['name'] = '10000以上';
                        }

                    }
                }

            } elseif ($Type == 'job') {

                //查询职位信息
                $jobwhere['id'] = array('in', pylode(',', $Fields));

                $JobList = $this->select_all("company_job", $jobwhere, "`edu`,`exp`,`job1`,`job1_son`,`job_post`,`provinceid`,`cityid`,`three_cityid`,`minsalary`,`maxsalary`");

                foreach ($JobList as $key => $value) {

                    $Count['edu'][$value['edu']]['count']++;
                    $Count['edu'][$value['edu']]['name'] = $comclass_name[$value['edu']];

                    $Count['exp'][$value['exp']]['count']++;
                    $Count['exp'][$value['exp']]['name'] = $comclass_name[$value['exp']];

                    $Count['job1'][$value['job1']]['count']++;
                    $Count['job1'][$value['job1']]['name'] = $job_name[$value['job1']];

                    $Count['job1_son'][$value['job1_son']]['count']++;
                    $Count['job1_son'][$value['job1_son']]['name'] = $job_name[$value['job1_son']];

                    $Count['job_post'][$value['job_post']]['count']++;
                    $Count['job_post'][$value['job_post']]['name'] = $job_name[$value['job_post']];

                    $Count['provinceid'][$value['provinceid']]['count']++;
                    $Count['provinceid'][$value['provinceid']]['name'] = $city_name[$value['provinceid']];

                    $Count['cityid'][$value['cityid']]['count']++;
                    $Count['cityid'][$value['cityid']]['name'] = $city_name[$value['cityid']];

                    $Count['three_cityid'][$value['three_cityid']]['count']++;
                    $Count['three_cityid'][$value['three_cityid']]['name'] = $city_name[$value['three_cityid']];

                    if ($value['maxsalary'] > 0) {
                        if ($value['maxsalary'] <= 2000) {
                            $Count['salary']['2000以下']['count']++;
                            $Count['salary']['2000以下']['name'] = '2000以下';
                        }
                        if ($value['maxsalary'] > 2000 && $value['maxsalary'] <= 4000) {
                            $Count['salary']['2000-4000']['count']++;
                            $Count['salary']['2000-4000']['name'] = '2000-4000';
                        }
                        if ($value['maxsalary'] > 4000 && $value['maxsalary'] <= 6000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['maxsalary'] > 6000 && $value['maxsalary'] <= 8000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['maxsalary'] > 8000 && $value['maxsalary'] <= 10000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['maxsalary'] > 10000) {
                            $Count['salary']['10000以上']['count']++;
                            $Count['salary']['10000以上']['name'] = '10000以上';
                        }
                    } else {
                        if ($value['minsalary'] <= 2000) {
                            $Count['salary']['2000以下']['count']++;
                            $Count['salary']['2000以下']['name'] = '2000以下';
                        }
                        if ($value['minsalary'] > 2000 && $value['minsalary'] <= 4000) {
                            $Count['salary']['2000-4000']['count']++;
                            $Count['salary']['2000-4000']['name'] = '2000-4000';
                        }
                        if ($value['minsalary'] > 4000 && $value['minsalary'] <= 6000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['minsalary'] > 6000 && $value['minsalary'] <= 8000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['minsalary'] > 8000 && $value['minsalary'] <= 10000) {
                            $Count['salary']['4000-6000']['count']++;
                            $Count['salary']['4000-6000']['name'] = '4000-6000';
                        }
                        if ($value['minsalary'] > 10000) {
                            $Count['salary']['10000以上']['count']++;
                            $Count['salary']['10000以上']['name'] = '10000以上';
                        }

                    }

                }
            } elseif ($Type == 'reg') {


                //查询注册信息
                $mwhere['uid'] = array('in', pylode(',', $Fields));

                $mwhere['groupby'] = 'source';

                $RegList = $this->select_all("member", $mwhere, "count(*) as count,source");

                foreach ($RegList as $key => $value) {

                    $Count['source'][$value['source']]['count'] = $value['count'];
                    $Count['source'][$value['source']]['name'] = $arr_data['source'][$value['source']];
                }
            } elseif ($Type == 'order') {

                //  查询订单信息
                $owhere['id']   =   array('in', pylode(',', $Fields));
                $OrderList      =   $this->select_all("company_order", $owhere, "`id`,`order_type`,`type`");

                $TypeName       =   $arr_data['ordertype'];
                $OrderTypeName  =   $arr_data['pay'];
                foreach ($OrderList as $key => $value) {
                    if (!$value['order_type']) {
                        $value['order_type']    =   0;
                    }
                    $Count['ordertype'][$value['order_type']]['count']++;
                    $Count['ordertype'][$value['order_type']]['name']   =   $OrderTypeName[$value['order_type']] ? $OrderTypeName[$value['order_type']] : '其他';

                    if (!$value['type']) {
                        $value['type'] = 0;
                    }
                    $Count['type'][$value['type']]['count']++;
                    $Count['type'][$value['type']]['name']  =   $TypeName[$value['type']] ? $TypeName[$value['type']] : '其他';
                }
            } elseif ($Type == 'company') {
                $rating = $this->select_all('company_rating', array(), '`id`,`name`');
                foreach ($rating as $value) {
                    $ratingList[$value['id']] = $value['name'];
                }

                //查询企业资料
                $comwhere['uid'] = array('in', pylode(',', $Fields));

                $CompanyList = $this->select_all("company", $comwhere, "`uid`,`hy`,`provinceid`,`jobtime`");

                $CompanyRating = $this->select_all("company_statis", $comwhere, "`uid`,`rating`");
                foreach ($CompanyRating as $value) {

                    $Count['rating'][$value['rating']]['count']++;
                    $Count['rating'][$value['rating']]['name'] = $ratingList[$value['rating']];

                }

                foreach ($CompanyList as $key => $value) {
                    if (!$value['hy']) {
                        $Count['hy'][0]['count']++;
                        $Count['hy'][0]['name'] = '其他';
                        $Count['hy'][0]['isother'] = true;
                        $Count['is'][0]['count']++;//完善企业资料数量
                    } else {
                        $Count['is'][1]['count']++;//完善企业资料数量
                        $Count['hy'][$value['hy']]['count']++;
                        $Count['hy'][$value['hy']]['name'] = $industry_name[$value['hy']];
                    }


                    $Count['is'][0]['name'] = '信息不全';

                    $Count['is'][1]['name'] = '信息完善';

                }

            } elseif ($Type == 'ad') {


                //查询企业资料
                $comwhere['uid'] = array('in', pylode(',', $Fields));

                $CompanyList = $this->select_all("company", $comwhere, "`uid`,`hy`,`provinceid`,`jobtime`");


                $CompanyRating = $this->select_all("company_statis", $comwhere, "`uid`,`rating`");
                foreach ($CompanyRating as $value) {

                    $Count['rating'][$value['rating']]['count']++;
                    $Count['rating'][$value['rating']]['name'] = $ratingList[$value['rating']];

                }

                foreach ($CompanyList as $key => $value) {
                    if (!$value['hy']) {
                        $Count['hy'][0]['count']++;
                        $Count['hy'][0]['name'] = '其他';
                        $Count['hy'][0]['isother'] = true;
                        $Count['is'][0]['count']++;//完善企业资料数量
                    } else {
                        $Count['is'][1]['count']++;//完善企业资料数量
                        $Count['hy'][$value['hy']]['count']++;
                        $Count['hy'][$value['hy']]['name'] = $industry_name[$value['hy']];
                    }


                    $Count['is'][0]['name'] = '信息不全';

                    $Count['is'][1]['name'] = '信息完善';

                }

            }
        }
        return $Count;
    }
    //数据展示地区分布
    function cityDataShow()
    {
        include PLUS_PATH . 'city.cache.php';
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        if (isset($this->config['sy_datashow_city_lev']) && in_array(intval($this->config['sy_datashow_city_lev']), array(1, 2, 3))) {
            $citylev = intval($this->config['sy_datashow_city_lev']);
        } else {
            $citylev = 2;
        }
        $groupCol = 'cityid';
        if ($citylev == 1) {
            $groupCol = 'provinceid';
        } elseif ($citylev == 3) {
            $groupCol = 'three_cityid';
        }
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $resumes = $this->select_all('resume_expect', $where, '`id`');
        $resumeIds = array();
        foreach ($resumes as $v) {
            $resumeIds[] = $v['id'];
        }
        $data = $this->select_all('resume_cityclass', array('eid' => array('in', pylode(',', $resumeIds)), $groupCol => array('>', 0), 'groupby' => $groupCol, 'orderby' => 'num'), '`' . $groupCol . '`,count(*) as num');
        $total = 0;
        foreach ($data as $k => $v) {
            if ($v[$groupCol] && $city_name[$v[$groupCol]]) {
                $total += $v['num'];
            } else {
                unset($data[$k]);
            }
        }
        $data = array_values($data);
        $totalRate = 0;
        if ($total) {
            foreach ($data as $k => $v) {
                $data[$k]['city_name'] = $city_name[$v[$groupCol]];
                if ($k != count($data) - 1) {
                    $totalRate += sprintf('%.2f', 100 * $v['num'] / $total);
                    $data[$k]['rate'] = sprintf('%.2f', 100 * $v['num'] / $total);
                } else {
                    $data[$k]['rate'] = sprintf('%.2f', 100 - $totalRate);
                }
            }
        }
        return $data;
    }

    //数据展示企业地区分布
    function comcityDataShow()
    {
        include PLUS_PATH . 'city.cache.php';
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        if (isset($this->config['sy_datashow_city_lev']) && in_array(intval($this->config['sy_datashow_city_lev']), array(1, 2, 3))) {
            $citylev = intval($this->config['sy_datashow_city_lev']);
        } else {
            $citylev = 2;
        }
        $groupCol = 'cityid';
        if ($citylev == 1) {
            $groupCol = 'provinceid';
        } elseif ($citylev == 3) {
            $groupCol = 'three_cityid';
        }
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where[$groupCol] = array('>', 0);
        $where['groupby'] = $groupCol;
        $where['orderby'] = 'num';
//        $where['limit'] = 5;
        $data = $this->select_all('company', $where, '`' . $groupCol . '`,count(*) as num');
        $total = 0;
        foreach ($data as $k => $v) {
            if ($v[$groupCol] && $city_name[$v[$groupCol]]) {
                $total += $v['num'];
            } else {
                unset($data[$k]);
            }
        }
        $data = array_values($data);
        $totalRate = 0;
        if ($total) {
            foreach ($data as $k => $v) {
                $data[$k]['city_name'] = $city_name[$v[$groupCol]];
                if ($k != count($data) - 1) {
                    $totalRate += sprintf('%.2f', 100 * $v['num'] / $total);
                    $data[$k]['rate'] = sprintf('%.2f', 100 * $v['num'] / $total);
                } else {
                    $data[$k]['rate'] = sprintf('%.2f', 100 - $totalRate);
                }
            }
        }
        return $data;
    }

    //数据展示企业规模分布
    function comgmDataShow()
    {
        include PLUS_PATH . 'com.cache.php';
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['mun'] = array('>', 0);
        $where['groupby'] = 'mun';
        $where['orderby'] = 'num';
//        $where['limit'] = 4;
        $tmpdata = $this->select_all('company', $where, '`mun`,count(*) as num');
        $total = 0;
        $data = array();
        foreach ($tmpdata as $k => $v) {
            if ($comclass_name[$v['mun']] && count($data) < 4) {
                $total += $v['num'];
                $data[] = $v;
            } else if (count($data) == 4) {
                break;
            }
        }
        $totalRate = 0;
        if ($total) {
            foreach ($data as $k => $v) {
                $data[$k]['mun_n'] = $comclass_name[$v['mun']];
                if ($k != count($data) - 1) {
                    $totalRate += sprintf('%.1f', 100 * $v['num'] / $total);
                    $data[$k]['rate'] = sprintf('%.1f', 100 * $v['num'] / $total);
                } else {
                    $data[$k]['rate'] = sprintf('%.1f', 100 - $totalRate);
                }
            }
        }
        return $data;
    }

    //数据展示企业性质分布
    function comxzDataShow()
    {
        include PLUS_PATH . 'com.cache.php';
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['pr'] = array('>', 0);
        $where['groupby'] = 'pr';
        $where['orderby'] = 'num';
//        $where['limit'] = 7;
        $data = $this->select_all('company', $where, '`pr`,count(*) as num');
        $total = 0;
        $other = 0;// 企业性质中是否已设置其他
        foreach ($data as $k => $v) {
            $total += $v['num'];
            if ($comclass_name[$v['pr']] == '其他') {
                $otherKey = $k;
                $other = 1;
            }
        }
        $tmp = array();
        foreach ($data as $k => $v) {
            if ($k <= 6) {
                $tmp[$k] = $v;
                $tmp[$k]['pr_n'] = $comclass_name[$v['pr']];
            } else {
                if ($other && $otherKey <= 6) {// 有其他且其他在前7项中，将7项之后的统计项合并到其他中
                    $tmp[$otherKey]['num'] += $v['num'];
                } else {
                    // 有其他且其他不在前7项中，将7项之后的统计项合并到第七项中，第七项名字改为其他
                    // 或者没有其他项，直接将7项之后的所有项合并到第七项
                    $tmp[6]['num'] += $v['num'];
                    $tmp[6]['pr_n'] = '其他';
                }
            }
        }
        $sort = array_column($tmp, 'num');
        array_multisort($sort, SORT_DESC, $tmp);
        $data = $tmp;
        $totalRate = 0;
        if ($total) {
            foreach ($data as $k => $v) {
                // $data[$k]['pr_n'] = $comclass_name[$v['pr']];
                if ($k != count($data) - 1) {
                    $data[$k]['rate'] = sprintf('%.1f', 100 * $v['num'] / $total);
                    $totalRate += sprintf('%.1f', $data[$k]['rate']);
                } else {
                    $data[$k]['rate'] = sprintf('%.1f', 100 - $totalRate);
                }
            }
        }
        return $data;
    }

    //数据展示年龄分布
    function ageDataShow()
    {
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['birthday'] = array('<>', '');
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $resumes = $this->select_all('resume', $where, '`birthday`');
        $data = array(
            array('name' => '16-24', 'rate' => 0),
            array('name' => '24-30', 'rate' => 0),
            array('name' => '30-40', 'rate' => 0),
            array('name' => '40-65', 'rate' => 0)
        );
        $ageArr = array(0, 0, 0, 0);
        $total = 0;
        foreach ($resumes as $v) {
            $age = floor((time() - strtotime($v['birthday'])) / 86400 / 365);
            if ($age >= 16 && $age <= 24) {
                $ageArr[0]++;
            } else if ($age > 24 && $age <= 30) {
                $ageArr[1]++;
            } else if ($age > 30 && $age <= 40) {
                $ageArr[2]++;
            } else if ($age > 40 && $age <= 65) {
                $ageArr[3]++;
            }
            if ($age >= 16 && $age <= 65) {
                $total++;
            }
        }

        $totalRate = 0;
        foreach ($data as $k => $v) {
            if ($k < 3) {
                $data[$k]['rate'] = $total > 0 ? sprintf('%.1f', 100 * $ageArr[$k] / $total) : '0';
                $totalRate += $data[$k]['rate'];
            } else {
                $data[$k]['rate'] = sprintf('%.1f', 100 - $totalRate);
            }
        }
        return $data;
    }

    //数据展示经验分布
    function expDataShow()
    {
        include PLUS_PATH . 'user.cache.php';
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['exp'] = array('<>', 'notnull');
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'exp';
        $where['orderby'] = 'num';
//        $where['limit'] = 5;
        $data = $this->select_all('resume', $where, '`exp`,count(*) as num');
        $total = 0;
        foreach ($data as $k => $v) {
            if ($userclass_name[$v['exp']]) {
                $total += $v['num'];
            } else {
                unset($data[$k]);
            }
        }
        $data = array_values($data);
        $totalRate = 0;
        foreach ($data as $k => $v) {
            $data[$k]['exp_n'] = $userclass_name[$v['exp']];
            if ($k != count($data) - 1) {
                $data[$k]['rate'] = $total ? sprintf('%.2f', 100 * $v['num'] / $total) : 0;
                $totalRate += $data[$k]['rate'];
            } else {
                $data[$k]['rate'] = sprintf('%.2f', 100 - $totalRate);
            }
        }
        return $data;
    }

    //数据展示性别分布
    function sexDataShow()
    {
        include(CONFIG_PATH . 'db.data.php');
        $sex = array();
        foreach ($arr_data['sex'] as $k => $v) {
            ($v == '男' || $v == '女') && array_push($sex, $k);
        }
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['sex'] = array('in', pylode(',', $sex));
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'sex';
        $where['orderby'] = 'num';
        $where['limit'] = 2;
        $data = $this->select_all('resume', $where, '`sex`,count(*) as num');
        $total = 0;
        foreach ($data as $k => $v) {
            $total += $v['num'];
        }
        $totalRate = 0;
        foreach ($data as $k => $v) {
            $data[$k]['exp_n'] = $arr_data['sex'][$v['sex']];
            if ($k != count($data) - 1) {
                $data[$k]['rate'] = $total ? intval(100 * $v['num'] / $total) : 0;
                $totalRate += $data[$k]['rate'];
            } else {
                $data[$k]['rate'] = 100 - $totalRate;
            }
        }
        return $data;
    }

    //数据展示学历分布
    function eduDataShow()
    {
        include PLUS_PATH . 'user.cache.php';
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['edu'] = array('<>', 'notnull');
        $where['PHPYUNBTWSTART_A'] = '';
        $where['lastupdate'][] = array('>=', $start);
        $where['lastupdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'edu';
        $where['orderby'] = 'num';
//        $where['limit'] = 6;
        $data = $this->select_all('resume', $where, '`edu`,count(*) as num');
        $total = 0;
        $other = 0;// 学历中是否已设置其他
        foreach ($data as $k => $v) {
            $total += $v['num'];
            if ($userclass_name[$v['edu']] == '其他') {
                $otherKey = $k;
                $other = 1;
            }
        }
        $tmp = array();
        foreach ($data as $k => $v) {
            if ($k <= 6) {
                $tmp[$k] = $v;
                $tmp[$k]['edu_n'] = $userclass_name[$v['edu']];
            } else {
                if ($other && $otherKey <= 6) {// 有其他且其他在前7项中，将7项之后的统计项合并到其他中
                    $tmp[$otherKey]['num'] += $v['num'];
                } else {
                    // 有其他且其他不在前7项中，将7项之后的统计项合并到第七项中，第七项名字改为其他
                    // 或者没有其他项，直接将7项之后的所有项合并到第七项
                    $tmp[6]['num'] += $v['num'];
                    $tmp[6]['edu_n'] = '其他';
                }
            }
        }
        $sort = array_column($tmp, 'num');
        array_multisort($sort, SORT_DESC, $tmp);
        $data = $tmp;
        $totalRate = 0;
        foreach ($data as $k => $v) {
            // $data[$k]['edu_n'] = $userclass_name[$v['edu']];
            if ($k != count($data) - 1) {
                $data[$k]['rate'] = $total ? sprintf('%.2f', 100 * $v['num'] / $total) : 0;
                $totalRate += $data[$k]['rate'];
            } else {
                $data[$k]['rate'] = sprintf('%.2f', 100 - $totalRate);
            }
        }
        return $data;
    }

    //数据展示用户活跃趋势
    function userHyChart()
    {
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['usertype'] = 1;
        $where['PHPYUNBTWSTART_A'] = '';
        $where['ctime'][] = array('>=', $start);
        $where['ctime'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'month';
        $where['orderby'] = 'month,asc';
        $logs = $this->select_all('login_log', $where, 'FROM_UNIXTIME(ctime,\'%m\') as month,count(*) as num');
        return $this->getMonthTjData($logs, 1);
    }

    //数据展示用户注册趋势
    function userRegChart()
    {
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['usertype'] = 1;
        $where['PHPYUNBTWSTART_A'] = '';
        $where['reg_date'][] = array('>=', $start);
        $where['reg_date'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'month';
        $where['orderby'] = 'month,asc';
        $logs = $this->select_all('member', $where, 'FROM_UNIXTIME(reg_date,\'%m\') as month,count(*) as num');
        return $this->getMonthTjData($logs, 2);
    }

    //数据展示企业登陆趋势
    function comLogChart()
    {
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['usertype'] = 2;
        $where['PHPYUNBTWSTART_A'] = '';
        $where['ctime'][] = array('>=', $start);
        $where['ctime'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'month';
        $where['orderby'] = 'month,asc';
        $logs = $this->select_all('login_log', $where, 'FROM_UNIXTIME(ctime,\'%m\') as month,count(*) as num');
        return $this->getMonthTjData($logs, 3);
    }

    //数据展示发布职位趋势
    function comJobChart()
    {
        $start = strtotime(date('Y-01-01 00:00:00', strtotime('-1 year')));
        $end = strtotime(date('Y-12-31 23:59:59', strtotime('-1 year')));
        $where['PHPYUNBTWSTART_A'] = '';
        $where['sdate'][] = array('>=', $start);
        $where['sdate'][] = array('<=', $end);
        $where['PHPYUNBTWEND_A'] = '';
        $where['groupby'] = 'month';
        $where['orderby'] = 'month,asc';
        $logs = $this->select_all('company_job', $where, 'FROM_UNIXTIME(sdate,\'%m\') as month,count(*) as num');
        return $this->getMonthTjData($logs, 4);
    }

    //获取年度初始数据
    function getMonthTjData($monthsData, $basetype)
    {
        if ($basetype == 1) {
            $base = $this->config['sy_datashowhy_base'] ? $this->config['sy_datashowhy_base'] : 0;// 活跃趋势基数
        } else if ($basetype == 2) {
            $base = $this->config['sy_datashowreg_base'] ? $this->config['sy_datashowreg_base'] : 0;// 注册趋势基数
        } else if ($basetype == 3) {
            $base = $this->config['sy_datashowlogin_base'] ? $this->config['sy_datashowlogin_base'] : 0;// 登陆趋势基数
        } else if ($basetype == 4) {
            $base = $this->config['sy_datashowjob_base'] ? $this->config['sy_datashowjob_base'] : 0;// 发布岗位趋势基数
        } else {
            $base = 0;
        }
        $data = array(
            array('month' => '1月', 'num' => $base),
            array('month' => '2月', 'num' => $base),
            array('month' => '3月', 'num' => $base),
            array('month' => '4月', 'num' => $base),
            array('month' => '5月', 'num' => $base),
            array('month' => '6月', 'num' => $base),
            array('month' => '7月', 'num' => $base),
            array('month' => '8月', 'num' => $base),
            array('month' => '9月', 'num' => $base),
            array('month' => '10月', 'num' => $base),
            array('month' => '11月', 'num' => $base),
            array('month' => '12月', 'num' => $base),
        );
        foreach ($monthsData as $v) {
            $data[intval($v['month']) - 1]['num'] += $v['num'];
        }
        return $data;
    }

    /**
     * @return void
     */
    public function fenxiabiaoClient($sdate,$edate){
        // 处理data数据
        $DateList = array();

        $countfield = 'count(*) as count ';
        $statistics  = array(
            array('table'=>'member','field'=>'reg_date','name'=>'新增个人','where'=>array('usertype' => '1')),
            array('table'=>'login_log','field'=>'ctime','name'=>'个人登录','where'=>array('usertype' => '1')),
            array('table'=>'resume_expect','field'=>'ctime','name'=>'新增简历'),
            array('table'=>'member','field'=>'reg_date','name'=>'新增企业','where'=>array('usertype' => '2')),
            array('table'=>'login_log','field'=>'ctime','name'=>'企业登录','where'=>array('usertype' => '2')),
            array('table'=>'company_job','field'=>'sdate','name'=>'新增职位'),
            array('table'=>'userid_job','field'=>'datetime','name'=>'简历投递'),
            array('table'=>'chat_log','field'=>'sendTime','name'=>'聊天'),
            array('table'=>'userid_msg','field'=>'datetime','name'=>'邀请面试'),
            array('table'=>'down_resume','field'=>'downtime','name'=>'简历下载'),
            array('table'=>'freedown_resume','field'=>'downtime','name'=>'简历下载'),
        );
        $bigCount = array();
        foreach ($statistics as $statistic){

            $field_u = $statistic['field'];
            $Tablename = $statistic['table'];
            if ($Tablename == 'chat_log') {
                $field_u = "{$field_u}/1000";
            }
            $FieldFormat = "FROM_UNIXTIME($field_u,'%Y-%m') as tjtime,$countfield";
            $swhere = isset($statistic['where'])? $statistic['where']:'';
            $FieldWhere = array();
            $FieldWhere[$statistic['field']][] =array('>=', $sdate);
            $FieldWhere[$statistic['field']][] =array('<=', $edate);
            $FieldWhere['groupby'] = 'tjtime';
            $FieldWhere['orderby'] = 'tjtime,desc';
            if($statistic['where']){
                $FieldWhere = array_merge($FieldWhere,$swhere);
            }
            $tableArr = array();
            $Stats = $this->select_all($Tablename, $FieldWhere, $FieldFormat);
            if ($Stats){
                foreach ($Stats as $sselect){
                    $tableArr[$sselect['tjtime']] = $sselect['count'];
                }
                if ($Tablename == 'member' && $swhere['usertype'] == 2){
                    $Tablename = 'company';
                }
                if ($Tablename == 'login_log' && $swhere['usertype'] == 2){
                    $Tablename = 'company_login_log';
                }
                $bigCount[$Tablename] = $tableArr;
                $bigCount[$Tablename]["sum"] = array_sum($tableArr);
            }
        }
        return $bigCount;
    }
}

?>
