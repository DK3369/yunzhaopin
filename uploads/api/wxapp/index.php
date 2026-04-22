<?php


include (dirname(dirname(dirname(__FILE__))) . '/global.php');
// 处理跨域（CORS）
$allowOrigin = '*';
if (!empty($config['sy_wapdomain'])){
    $protocol   = isset($config['sy_wapssl']) && $config['sy_wapssl']=='1' ? 'https://' : 'http://';
    $allowOrigin = $protocol.$config['sy_wapdomain'];
}
header('Access-Control-Allow-Origin: ' . $allowOrigin);
header('Access-Control-Allow-Methods: POST, GET, OPTIONS');
header('Access-Control-Allow-Credentials: true');
header('Access-Control-Allow-Headers: Content-Type, Accept, xcxcode, codeplat, mcsdk');
header('Access-Control-Max-Age: 86400');
// 处理 OPTIONS 预检请求
if ($_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
    http_response_code(200);
    exit();
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