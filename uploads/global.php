<?php

error_reporting(E_ALL);
ini_set('display_errors', 0);
ini_set('log_errors', 1);
ini_set('error_log', '/tmp/phpyun_debug.log');

define('APP_PATH',dirname(__FILE__).'/');
define('CONFIG_PATH',APP_PATH.'config/');
define('DATA_PATH',APP_PATH.'data/');
define('LIB_PATH',APP_PATH.'app/include/');
define('TPL_PATH',APP_PATH.'app/template/');
define('MODEL_PATH',APP_PATH.'model/');
define('PLUS_PATH',DATA_PATH.'plus/');
define('ALL_PS','conn');

ini_set('session.gc_maxlifetime',9000);
ini_set('session.gc_probability',10);
ini_set('session.gc_divisor',100);

include(CONFIG_PATH.'db.config.php');
include_once(PLUS_PATH.'config.php');
include(PLUS_PATH.'seo.cache.php');

if(!empty($config)){
	//入口处理
	include_once(CONFIG_PATH.'initDeal.php');

	$initObj = new initDeal();
	$hatReturn = $initObj->getAllParam();

	if($hatReturn['param']){
		$_GET = array_merge($_GET,$hatReturn['param']);
	}
	//入口处理end
}
include(CONFIG_PATH.'db.safety.php');

$starttime=time();
define('DEF_DATA', $db_config['def']);
unset ($_ENV, $HTTP_ENV_VARS, $_REQUEST, $HTTP_POST_VARS, $HTTP_GET_VARS);
$_COOKIE = (is_array($_COOKIE)) ? $_COOKIE : $HTTP_COOKIE_VARS;

header('P3P: CP="NOI ADM DEV PSAi COM NAV OUR OTRo STP IND DEM"');
header('Content-Type: text/html; charset=UTF-8');
header("Cache-control: private");
header('X-Frame-Options: SAMEORIGIN');
date_default_timezone_set($db_config['timezone']);
if (substr(PHP_VERSION, 0, 1) == '7') {
    
    include_once(LIB_PATH."mysqli.class.php");
    
}else{
    
    include_once(LIB_PATH."mysql.class.php");
    
}

include_once(LIB_PATH.'libs/Smarty.class.php');
$phpyun = new Smarty;
$phpyun->debugging      = false;
$phpyun->caching        = false;
$phpyun->force_cache    = false;
$phpyun->template_dir   = TPL_PATH;
$phpyun->compile_dir    = DATA_PATH.'/templates_c/';
$phpyun->cache_dir      = DATA_PATH.'/cache/';
$phpyun->left_delimiter = '{yun:}';
$phpyun->right_delimiter= '{/yun}';
$phpyun->get_install();

if(is_file(LIB_PATH.'webscan360/360safe/360webscan.php')){
    require_once(LIB_PATH.'webscan360/360safe/360webscan.php');
}

$db = new mysql($db_config['dbhost'], $db_config['dbuser'], $db_config['dbpass'], $db_config['dbname'], ALL_PS, $db_config['charset'],$db_config['def']);

include_once(LIB_PATH.'public.function.php');
include_once(LIB_PATH.'public.domain.php');
include(LIB_PATH.'public.url.php');
?>