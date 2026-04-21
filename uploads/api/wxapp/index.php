<?php
/*
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

include (dirname(dirname(dirname(__FILE__))) . '/global.php');
// 判断处理h5跨域
if (!empty($config['sy_wapdomain'])){
    
    $protocol = isset($config['sy_wapssl']) && $config['sy_wapssl']=='1' ? 'https://' : 'http://';
    $wapurl   = $protocol.$config['sy_wapdomain'];
    
    header("Access-Control-Allow-Origin: ".$wapurl); // 允许跨域的域名
    header('Access-Control-Allow-Methods:POST,GET'); // 允许请求的类型
    header('Access-Control-Allow-Credentials: true'); // 设置是否允许发送 cookies
    header('Access-Control-Allow-Headers: xcxcode, codeplat, mcsdk'); // 设置允许自定义请求头的字段
}

$pageType = 'wxapp';
$model    = $_GET['m'];
$action   = $_GET['c'];
$member   = '';

if (isset($_GET['h'])){
    $member   = $_GET['h'];
}
if ($model == '')
    $model = 'index';
if ($action == '')
    $action = 'index';

require (APP_PATH . 'app/public/common.php');
require ('wxapp.controller.php');

if ($member == 'user') {
    require ('member/user.class.php');
    require ('member/user/' . $model . '.class.php');
} elseif ($member == 'com') {
    require ('member/com.class.php');
    require ('member/com/' . $model . '.class.php');
} else {
    require ('model/' . $model . '.class.php');
}

$conclass = $model . '_controller';
$actfunc = $action . '_action';
$views = new $conclass($phpyun, $db, $db_config['def']);
if (! method_exists($views, $actfunc)) {
    $views->DoException();
}

$views->$actfunc();
?>