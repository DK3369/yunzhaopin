<?php



/**
 * 计划任务：预约职位刷新
 * 仅作参考
 */
global $db_config, $db;

include(dirname(dirname(dirname(__FILE__))).'/model/job.model.php');
$jobM   =   new job_model($db, $db_config['def']);
$jobM -> upReserveJob();

?>