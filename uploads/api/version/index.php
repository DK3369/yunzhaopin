<?php

include (dirname(dirname(dirname(__FILE__))) . '/global.php');
$model    =  $_GET['m'];
$action   =  $_GET['c'];
if ($model == '')
    $model = 'index';
if ($action == '')
    $action = 'index';
require (APP_PATH . 'app/public/common.php');
require ('version_base.controller.php');
require ('model/' . $model . '.class.php');
$conclass = $model . '_controller';
$actfunc = $action . '_action';
$views = new $conclass($phpyun, $db, $db_config['def']);
if (! method_exists($views, $actfunc)) {
    $views->DoException();
}
$views->$actfunc();
?>
