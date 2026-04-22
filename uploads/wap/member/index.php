<?php

include (dirname(dirname(dirname(__FILE__)))."/global.php");

$pageType   =   'wap';
 
$model      =   $_GET['m'];
$action     =   $_GET['c'];

if ($action == '')
    $action =   'index';

$usertype   =   $_COOKIE['usertype'];

if ($usertype == 1) {
    $model  =   'index';
} elseif ($usertype == 2) {
    $model  =   'com';
} else {
    if(!$_COOKIE['uid']){
    	$model  =   'index';
    }else{
    	header('Location: '.Url('wap', array('c' => 'register', 'a' => 'ident')));
    	die();
    }
}

require (APP_PATH . 'app/public/common.php');
require ('wap.controller.php');
require ("model/" . $model . '.class.php');

$conclass   =   $model.'_controller';
$actfunc    =   $action.'_action';

$views      =   new $conclass($phpyun, $db, $db_config['def'], 'wap_member');

if (!method_exists($views, $actfunc)) {
    $views->DoException();
}

$views->$actfunc();
?>