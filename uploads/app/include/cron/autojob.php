<?php

global $db_config, $db;

$count  =   $db->select_num("company_job", "`autotime`>='" . strtotime(date('Y-m-d')) . "'");
$size   =   1000;

$num    =   ceil($count / $size);

include(dirname(dirname(dirname(__FILE__))).'/model/log.model.php');
$logM   =   new log_model($db, $db_config['def']);

for ($i = 0; $i < $num; $i++) {

    $offset     =   $i * $size;
    $autoList   =   $db->select_all("company_job", "`autotime`>='" . strtotime(date('Y-m-d')) . "' limit {$offset},{$size} ", "`id`,`uid`");

    $jobId      =   array();
    $uid        =   array();
    $SqlCase    =   'lastupdate = CASE id ';
    foreach ($autoList as $key => $value) {

        $jobId[]    =   $value['id'];
        if (!in_array($uid)) {
            $uid[]  =   $value['uid'];
        }

        $LastTime   =   strtotime('-' . rand(1, 59) . ' minutes', time());
        $SqlCase    .=  sprintf("WHEN %d THEN %d ", $value['id'], $LastTime);

        $logData[$key]['uid']       =   $value['uid'];
        $logData[$key]['usertype']  =   2;
        $logData[$key]['jobid']     =   $value['id'];
        $logData[$key]['type']      =   1;
        $logData[$key]['r_time']    =   $LastTime;
        $logData[$key]['port']      =   1;
        $logData[$key]['ip']        =   fun_ip_get();
        $logData[$key]['remark']    =   '计划任务：职位自动刷新';
    }
    $SqlCase .= 'END';

    $nid    =   $db->update_all("company_job", $SqlCase, "`id` IN (" . @implode(',', $jobId) . ")");
    $db->update_all("company", $SqlCase, "`uid` IN (" . @implode(',', $uid) . ")");
    $db->update_all("hot_job", $SqlCase, "`uid` IN (" . @implode(',', $uid) . ")");
    $logM->addJobSxLogS($logData);
}

?>