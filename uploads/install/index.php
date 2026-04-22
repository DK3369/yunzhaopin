<?php

header("Content-Type: text/html; charset=utf-8");
ob_start();
error_reporting(0);
set_time_limit(0);
$i_model = 1;
define('APP_PATH',dirname(dirname(__FILE__)).'/');  
define('S_ROOT', dirname(__FILE__).DIRECTORY_SEPARATOR);
define('VERSION', '7.0 beta');
define('YEAR', '2023');
if (substr(PHP_VERSION, 0, 1) == '7') {
	$installDir = 'php7';
}else{
	$installDir = 'php5';
}
define('INS_DIR',$installDir.'/'); 
require_once 'install_lang.php';
require_once $installDir.'/install_function.php';
require_once $installDir.'/install_mysql.php';
require_once $installDir.'/install_var.php';
include $installDir.'/install.php';

?>