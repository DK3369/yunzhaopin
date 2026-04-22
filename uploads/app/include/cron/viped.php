<?php

/************
* 计划任务：会员到期自动下架职位
* 仅作参考
*/
global $db_config,$db,$config;

include(dirname(dirname(dirname(__FILE__)))."/model/statis.model.php");

$statisM = new statis_model($db, $db_config['def']);

$statisM->setViped();

?>