<?php

session_start();

include(dirname(dirname(__FILE__))."/global.php");
global $config;

$m = isset($_GET['m']) ? $_GET['m'] : 'index';
$c = isset($_GET['c']) ? $_GET['c'] : 'index';
$a = isset($_GET['a']) ? $_GET['a'] : 'index';

if (!preg_match("/^[0-9a-zA-Z\_]*$/",$m)){
    $m = 'index';
}
$Module		=	explode("\\",str_replace("/","\\",getcwd()));

if(end($Module)){$ModuleName=end($Module);}else{$ModuleName='admin';}

require(APP_PATH.'app/public/common.php');
require(APP_PATH.$ModuleName.'/adminCommon.class.php');
if ($m == 'index'){
    require('model/index.class.php');
    $conclass	=	$m.'_controller';
    $actfunc	=	$c.'_action';
}else{
    require('model/'.$m.'/'.$c.'.class.php');
    $conclass	=	$c.'_controller';
    $actfunc	=	$a.'_action';
}
$adminDir	=	$ModuleName;
$views		=	new $conclass($phpyun,$db,$db_config["def"],"admin");
if(!method_exists($views,$actfunc)){
    $views->DoException();
}
$views->$actfunc();



?>