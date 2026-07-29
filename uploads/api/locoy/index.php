<?php

include(dirname(dirname(dirname(__FILE__)))."/global.php");

$model = isset($_GET['m']) ? $_GET['m'] : '';
$action = isset($_GET['c']) ? $_GET['c'] : '';

if (!preg_match('/^[0-9a-zA-Z_]+$/', $model) || !preg_match('/^[0-9a-zA-Z_]+$/', $action)) {
    http_response_code(400);
    echo 'Invalid locoy request';
    exit;
}

$modelFile = __DIR__ . '/model/' . $model . '.class.php';
if (!is_file($modelFile)) {
    http_response_code(404);
    echo 'Locoy model not found';
    exit;
}

require(APP_PATH.'app/public/common.php');
require($modelFile);

$conclass = $model . '_controller';
$actfunc = $action . '_action';

if (!class_exists($conclass)) {
    http_response_code(404);
    echo 'Locoy controller not found';
    exit;
}

$views = new $conclass($phpyun, $db, $db_config['def'], 'index');
if (!method_exists($views, $actfunc)) {
    http_response_code(404);
    echo 'Locoy action not found';
    exit;
}

$views->$actfunc();

?>
