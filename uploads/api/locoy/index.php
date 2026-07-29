<?php

include dirname(dirname(dirname(__FILE__))) . '/global.php';

$locoyRequestId = bin2hex(random_bytes(8));
header('X-Request-ID: ' . $locoyRequestId);
header('X-Content-Type-Options: nosniff');

function locoy_security_log($event)
{
    global $locoyRequestId;
    $ip = isset($_SERVER['REMOTE_ADDR']) ? $_SERVER['REMOTE_ADDR'] : '';
    $model = isset($_GET['m']) ? (string) $_GET['m'] : '';
    $action = isset($_GET['c']) ? (string) $_GET['c'] : '';
    error_log(sprintf(
        '[locoy:%s] event=%s ip=%s route=%s/%s',
        $locoyRequestId,
        $event,
        $ip,
        $model,
        $action
    ));
}

function locoy_fail($code, $event, $status = 200)
{
    locoy_security_log($event);
    http_response_code($status);
    header('Content-Type: text/plain; charset=utf-8');
    echo (string) $code;
    exit;
}

function locoy_rate_limit()
{
    $ip = isset($_SERVER['REMOTE_ADDR']) ? $_SERVER['REMOTE_ADDR'] : 'unknown';
    $model = isset($_GET['m']) ? (string) $_GET['m'] : '';
    $directory = sys_get_temp_dir() . '/phpyun-locoy-rate';
    if (!is_dir($directory) && !mkdir($directory, 0700, true) && !is_dir($directory)) {
        locoy_security_log('rate_store_unavailable');
        return;
    }

    $file = $directory . '/' . hash('sha256', $ip . '|' . $model);
    $handle = fopen($file, 'c+');
    if ($handle === false) {
        locoy_security_log('rate_store_unavailable');
        return;
    }
    if (!flock($handle, LOCK_EX)) {
        fclose($handle);
        locoy_security_log('rate_lock_unavailable');
        return;
    }

    $raw = stream_get_contents($handle);
    $state = json_decode($raw, true);
    $now = time();
    if (!is_array($state) || empty($state['start']) || $state['start'] <= $now - 60) {
        $state = array('start' => $now, 'count' => 0);
    }
    $state['count']++;
    ftruncate($handle, 0);
    rewind($handle);
    fwrite($handle, json_encode($state));
    fflush($handle);
    flock($handle, LOCK_UN);
    fclose($handle);

    if ($state['count'] > 120) {
        locoy_fail(5, 'rate_limited', 429);
    }
}

function locoy_config()
{
    static $config = null;
    if (is_array($config)) {
        return $config;
    }

    include APP_PATH . 'data/api/locoy/locoy_config.php';
    $config = isset($locoyinfo) && is_array($locoyinfo) ? $locoyinfo : array();

    if (empty($config['locoy_online'])) {
        locoy_fail(4, 'disabled');
    }

    $contentLength = isset($_SERVER['CONTENT_LENGTH']) ? (int) $_SERVER['CONTENT_LENGTH'] : 0;
    if ($contentLength > 4 * 1024 * 1024) {
        locoy_fail(2, 'request_too_large', 413);
    }

    locoy_rate_limit();

    $allowedIps = isset($config['locoy_allow_ip']) ? $config['locoy_allow_ip'] : '';
    if ($allowedIps !== '') {
        $allowed = array_filter(array_map('trim', explode(',', (string) $allowedIps)));
        $remoteIp = isset($_SERVER['REMOTE_ADDR']) ? $_SERVER['REMOTE_ADDR'] : '';
        if (!in_array($remoteIp, $allowed, true)) {
            locoy_fail(5, 'ip_denied', 403);
        }
    }

    $expected = isset($config['locoy_key']) ? (string) $config['locoy_key'] : '';
    $provided = isset($_GET['key']) && !is_array($_GET['key']) ? trim((string) $_GET['key']) : '';
    if ($expected === '' || $provided === '' || strlen($provided) > 256 || !hash_equals($expected, $provided)) {
        locoy_fail(5, 'invalid_key');
    }

    return $config;
}

function locoy_normalize_post($input)
{
    if (!is_array($input)) {
        locoy_fail(2, 'invalid_payload', 400);
    }

    $integerFields = array_flip(array(
        'nid', 'did', 'hits', 'sort', 'minsalary', 'maxsalary', 'jobstatus',
        'info_height', 'info_weight', 'skill_ing', 'skill_longtime'
    ));
    $normalized = array();
    foreach ($input as $field => $value) {
        if (!is_string($field) || !preg_match('/^[A-Za-z0-9_]+$/D', $field) || is_array($value)) {
            locoy_fail(2, 'invalid_field', 400);
        }
        $value = (string) $value;
        if (preg_match('/(content|description|partcontent|intro)$/i', $field)) {
            $maximum = 1024 * 1024;
        } elseif (preg_match('/(url|photo|logo|website|homepage)$/i', $field)) {
            $maximum = 2048;
        } else {
            $maximum = 512;
        }
        if (strlen($value) > $maximum) {
            locoy_fail(2, 'field_too_long', 413);
        }
        $normalized[$field] = isset($integerFields[$field]) ? (int) $value : $value;
    }
    return $normalized;
}

function locoy_post_defaults($model, $post)
{
    $schemas = array(
        'news' => array(
            'title', 'content', 'nid', 'did', 'author', 'description', 'source',
            'ctime', 'hits', 'sort', 'newsphoto', 's_thumb', 'keyword'
        ),
        'job' => array(
            'job_name', 'com_name', 'description', 'job_hy', 'hy', 'job_cate',
            'job_city', 'city', 'sdate', 'minsalary', 'maxsalary', 'exp', 'report',
            'age', 'type', 'sex', 'edu', 'marriage', 'number', 'lastupdate',
            'address', 'linkphone', 'email', 'zip', 'linkman', 'linkjob', 'linkqq',
            'moblie', 'website', 'mapx', 'mapy', 'logo', 'com_sdate', 'money', 'content'
        ),
        'partjob' => array(
            'part_name', 'com_name', 'partcontent', 'job_city', 'city', 'sdate',
            'edate', 'worktime', 'sex', 'number', 'type', 'salary_type', 'salary',
            'lastupdate', 'address', 'linkphone', 'email', 'zip', 'linkman',
            'linkjob', 'linkqq', 'linktel', 'moblie', 'website', 'x', 'y',
            'com_sdate', 'money', 'content', 'hy', 'pr', 'mun'
        ),
        'user' => array(
            'info_name', 'info_classid', 'info_hy', 'info_city', 'info_report',
            'info_type', 'info_hits', 'jobstatus', 'info_edu', 'info_exp',
            'info_sex', 'minsalary', 'maxsalary', 'info_address', 'info_height',
            'info_weight', 'info_birthday', 'info_telphone', 'info_homepage',
            'info_description', 'info_living', 'info_domicile', 'info_email',
            'info_qq', 'info_photo', 'info_marriage', 'nationality',
            'skill_name', 'skill_skill', 'skill_ing', 'skill_longtime',
            'pro_name', 'pro_sdate', 'pro_edate', 'pro_sys', 'pro_content', 'pro_title',
            'cert_name', 'cert_title', 'cert_sdate', 'cert_content',
            'other_content', 'other_title', 'other_name'
        )
    );

    $fields = isset($schemas[$model]) ? $schemas[$model] : array();
    if ($model === 'job') {
        for ($index = 1; $index <= 8; $index++) {
            $fields[] = 'welfare' . $index;
        }
        for ($index = 1; $index <= 3; $index++) {
            $fields[] = 'lang' . $index;
        }
    }
    if ($model === 'user') {
        foreach (array('work' => 4, 'edu' => 2, 'train' => 1) as $prefix => $maximum) {
            for ($index = 0; $index <= $maximum; $index++) {
                $suffix = $index === 0 ? '' : (string) $index;
                if ($prefix === 'work') {
                    foreach (array('name', 'sdate', 'edate', 'department', 'content', 'title') as $field) {
                        $fields[] = 'work_' . $field . $suffix;
                    }
                } elseif ($prefix === 'edu') {
                    foreach (array('name', 'title', 'sdate', 'edate', 'specialty', 'content') as $field) {
                        $fields[] = 'edu_' . $field . $suffix;
                    }
                } else {
                    foreach (array('name', 'title', 'sdate', 'edate', 'content') as $field) {
                        $fields[] = 'train_' . $field . $suffix;
                    }
                }
            }
        }
    }

    $integerFields = array_flip(array(
        'nid', 'did', 'hits', 'sort', 'minsalary', 'maxsalary', 'jobstatus',
        'info_height', 'info_weight', 'skill_ing', 'skill_longtime'
    ));
    foreach (array_unique($fields) as $field) {
        if (!array_key_exists($field, $post)) {
            $post[$field] = isset($integerFields[$field]) ? 0 : '';
        }
    }
    return $post;
}


$model = isset($_GET['m']) ? (string) $_GET['m'] : '';
$action = isset($_GET['c']) ? (string) $_GET['c'] : '';
$routes = array(
    'job' => 'add',
    'user' => 'add',
    'partjob' => 'add',
    'news' => 'addnews'
);

if (!isset($routes[$model])
    || !hash_equals($routes[$model], $action)
    || !preg_match('/^[0-9A-Za-z_]+$/D', $model)
    || !preg_match('/^[0-9A-Za-z_]+$/D', $action)
) {
    locoy_fail('Invalid locoy request', 'invalid_route', 400);
}

$locoyinfo = locoy_config();
$_POST = locoy_post_defaults($model, locoy_normalize_post($_POST));

$modelFile = __DIR__ . '/model/' . $model . '.class.php';
require APP_PATH . 'app/public/common.php';
require $modelFile;

$conclass = $model . '_controller';
$actfunc = $action . '_action';
if (!class_exists($conclass, false) || !method_exists($conclass, $actfunc)) {
    locoy_fail('Locoy action not found', 'missing_action', 400);
}

try {
    $views = new $conclass($phpyun, $db, $db_config['def'], 'index');
    $views->$actfunc();
} catch (Throwable $exception) {
    error_log(sprintf(
        '[locoy:%s] %s in %s:%d',
        $locoyRequestId,
        $exception->getMessage(),
        $exception->getFile(),
        $exception->getLine()
    ));
    http_response_code(500);
    header('Content-Type: text/plain; charset=utf-8');
    echo '0';
}
