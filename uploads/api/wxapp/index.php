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
$model    = isset($_GET['m']) ? $_GET['m'] : '';
$action   = isset($_GET['c']) ? $_GET['c'] : '';
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
    $modelFile = 'member/user/' . $model . '.class.php';
    require ('member/user.class.php');
} elseif ($member == 'com') {
    $modelFile = 'member/com/' . $model . '.class.php';
    require ('member/com.class.php');
} else {
    $modelFile = 'model/' . $model . '.class.php';
}

if (!is_file($modelFile)) {
    header('content-type:application/json; charset=utf-8');
    echo json_encode(array('error' => 404, 'msg' => 'api not found', 'data' => array()));
    exit;
}
require ($modelFile);

$conclass = $model . '_controller';
$actfunc = $action . '_action';
$views = new $conclass($phpyun, $db, $db_config['def']);
if (! method_exists($views, $actfunc)) {
    $views->DoException();
}

$views->$actfunc();
?>