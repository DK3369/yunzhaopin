<?php

/************
* 计划任务：每日自动更新部分职位
* 仅作参考
*/
global $db_config,$db;

$stime = time()-86400*31;
$query = $db->query("SELECT r1.id FROM $db_config[def]company_job AS r1 JOIN (SELECT ROUND(RAND() * (SELECT MAX(id) FROM $db_config[def]company_job)) AS id) AS r2 WHERE r1.id >= r2.id AND `sdate`>'".$stime."'  AND `state`=1 AND `r_status`=1 AND `status`<>1 ORDER BY r1.id ASC LIMIT 30");


while($rs = $db->fetch_array($query))
{
	$LastTime = strtotime('-'.rand(1,59).' minutes', time());
	$db->query("update $db_config[def]company_job set `lastupdate`='".$LastTime."' where `id`='".$rs['id']."'");
	$db->query("update $db_config[def]company set `lastupdate`='".$LastTime."' , `jobtime`='".$LastTime."' where `uid`='".$rs['uid']."'");
    $db->query("INSERT INTO $db_config[def]job_refresh_log (`id`, `uid`, `usertype`, `jobid`, `type`, `r_time`, `port`, `ip`, `remark`) VALUES (NULL, '".$rs['uid']."', '2', '".$rs['id']."', '1', '".$LastTime."', '1', '".fun_ip_get()."', '计划任务：自动刷新');");
}


?>