<?php

include dirname(dirname(dirname(__FILE__))) . '/global.php';

$requestId = bin2hex(random_bytes(8));
header('X-Request-ID: ' . $requestId);

function wxapp_json_error($status, $error, $message, $requestId)
{
    http_response_code($status);
    header('Content-Type: application/json; charset=utf-8');
    echo json_encode(array(
        'error' => $error,
        'msg' => $message,
        'data' => array(),
        'request_id' => $requestId
    ));
    exit;
}

function wxapp_origin($value, $defaultScheme = 'http')
{
    $value = trim((string) $value);
    if ($value === '') {
        return '';
    }
    if (!preg_match('#^https?://#i', $value)) {
        $value = $defaultScheme . '://' . $value;
    }
    $parts = parse_url($value);
    if (empty($parts['host'])) {
        return '';
    }
    $scheme = strtolower(isset($parts['scheme']) ? $parts['scheme'] : $defaultScheme);
    $origin = $scheme . '://' . strtolower($parts['host']);
    if (!empty($parts['port'])) {
        $origin .= ':' . (int) $parts['port'];
    }
    return $origin;
}

$allowedOrigins = array();
$wapScheme = !empty($config['sy_wapssl']) ? 'https' : 'http';
foreach (array(
    wxapp_origin(isset($config['sy_wapdomain']) ? $config['sy_wapdomain'] : '', $wapScheme),
    wxapp_origin(isset($config['sy_weburl']) ? $config['sy_weburl'] : ''),
    wxapp_origin(isset($config['sy_indexdomain']) ? $config['sy_indexdomain'] : ''),
    'http://dev.test',
    'http://localhost',
    'http://127.0.0.1'
) as $allowedOrigin) {
    if ($allowedOrigin !== '') {
        $allowedOrigins[$allowedOrigin] = true;
    }
}

$origin = isset($_SERVER['HTTP_ORIGIN']) ? wxapp_origin($_SERVER['HTTP_ORIGIN']) : '';
if ($origin !== '') {
    header('Vary: Origin');
    if (isset($allowedOrigins[$origin])) {
        header('Access-Control-Allow-Origin: ' . $origin);
        header('Access-Control-Allow-Credentials: true');
    } elseif (isset($_SERVER['REQUEST_METHOD']) && $_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
        wxapp_json_error(403, 403, 'origin not allowed', $requestId);
    }
}
header('Access-Control-Allow-Methods: POST, GET, OPTIONS');
header('Access-Control-Allow-Headers: Content-Type, Accept, xcxcode, codeplat, mcsdk');
header('Access-Control-Max-Age: 86400');

if (isset($_SERVER['REQUEST_METHOD']) && $_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
    http_response_code(204);
    exit;
}

$pageType = 'wxapp';
$model = isset($_GET['m']) && $_GET['m'] !== '' ? (string) $_GET['m'] : 'index';
$action = isset($_GET['c']) && $_GET['c'] !== '' ? (string) $_GET['c'] : 'index';
$member = isset($_GET['h']) ? (string) $_GET['h'] : '';

if (!preg_match('/^[A-Za-z0-9_]+$/D', $model)
    || !preg_match('/^[A-Za-z0-9_]+$/D', $action)
    || !in_array($member, array('', 'user', 'com'), true)
) {
    wxapp_json_error(400, 400, 'invalid api route', $requestId);
}

require APP_PATH . 'app/public/common.php';
require __DIR__ . '/wxapp.controller.php';

if ($member === 'user') {
    $modelFile = __DIR__ . '/member/user/' . $model . '.class.php';
    require __DIR__ . '/member/user.class.php';
} elseif ($member === 'com') {
    $modelFile = __DIR__ . '/member/com/' . $model . '.class.php';
    require __DIR__ . '/member/com.class.php';
} else {
    $modelFile = __DIR__ . '/model/' . $model . '.class.php';
}

if (!is_file($modelFile)) {
    wxapp_json_error(400, 404, 'api not found', $requestId);
}
require $modelFile;

$conclass = $model . '_controller';
$actfunc = $action . '_action';
if (!class_exists($conclass, false)
    || !is_subclass_of($conclass, 'wxapp_controller')
    || !method_exists($conclass, $actfunc)
) {
    wxapp_json_error(400, 404, 'api action not found', $requestId);
}

try {
    $views = new $conclass($phpyun, $db, $db_config['def']);
    $views->$actfunc();
} catch (Throwable $exception) {
    error_log(sprintf(
        '[wxapp:%s] %s in %s:%d',
        $requestId,
        $exception->getMessage(),
        $exception->getFile(),
        $exception->getLine()
    ));
    wxapp_json_error(500, 500, 'internal server error', $requestId);
}
