<?php

/************
* 计划任务：每日自动更新部分简历
* 仅作参考
*/
global $db_config,$db;

$stime = time()-86400*31;
$query = $db->query("SELECT r1.id FROM $db_config[def]resume_expect AS r1 JOIN (SELECT ROUND(RAND() * (SELECT MAX(id) FROM $db_config[def]resume_expect)) AS id) AS r2 WHERE r1.id >= r2.id  AND status='1' AND `r_status`='1' AND `defaults`='1' AND `ctime`>'".$stime."'  ORDER BY r1.id ASC LIMIT 30");

while($rs = $db->fetch_array($query))
{
	
	$LastTime = strtotime('-'.rand(1,59).' minutes', time());

	$db->update_all("resume_expect","`lastupdate`='".$LastTime."'","`id`='".$rs['id']."'");
}


?>