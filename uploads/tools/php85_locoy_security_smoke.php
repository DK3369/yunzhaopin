<?php
/**
 * Read-only locoy authentication, injection, and route smoke tests.
 */
$baseUrl = getenv('PHP85_SMOKE_BASE_URL') ?: 'http://127.0.0.1';
$host = getenv('PHP85_SMOKE_HOST') ?: 'dev.test';

function locoySmokeRequest($baseUrl, $host, $path, $fields = array())
{
    $handle = curl_init(rtrim($baseUrl, '/') . $path);
    curl_setopt_array($handle, array(
        CURLOPT_RETURNTRANSFER => true,
        CURLOPT_FOLLOWLOCATION => false,
        CURLOPT_HTTPHEADER => array('Host: ' . $host),
        CURLOPT_CONNECTTIMEOUT => 5,
        CURLOPT_TIMEOUT => 20,
        CURLOPT_POST => !empty($fields),
        CURLOPT_POSTFIELDS => $fields
    ));
    $body = curl_exec($handle);
    if ($body === false) {
        throw new RuntimeException(curl_error($handle));
    }
    return array(
        'status' => curl_getinfo($handle, CURLINFO_RESPONSE_CODE),
        'body' => $body
    );
}

$injection = "' OR 1=1 --";
$cases = array(
    array('news', '/api/locoy/index.php?m=news&c=addnews&key=invalid-test-key', array(
        'title' => $injection, 'nid' => '1', 'content' => 'test'
    )),
    array('job', '/api/locoy/index.php?m=job&c=add&key=invalid-test-key', array(
        'job_name' => $injection, 'com_name' => 'test'
    )),
    array('user', '/api/locoy/index.php?m=user&c=add&key=invalid-test-key', array(
        'info_name' => $injection
    )),
    array('partjob', '/api/locoy/index.php?m=partjob&c=add&key=invalid-test-key', array(
        'part_name' => $injection, 'com_name' => 'test'
    ))
);

$passed = 0;
foreach ($cases as $case) {
    $response = locoySmokeRequest($baseUrl, $host, $case[1], $case[2]);
    if ($response['status'] !== 200 || trim($response['body']) !== '5') {
        throw new RuntimeException($case[0] . ' invalid-key test failed');
    }
    echo '[OK] ' . $case[0] . ' invalid key and injection payload' . PHP_EOL;
    $passed++;
}

$route = locoySmokeRequest(
    $baseUrl,
    $host,
    '/api/locoy/index.php?m=..%2Fnews&c=addnews&key=invalid-test-key'
);
if ($route['status'] !== 400 || trim($route['body']) !== 'Invalid locoy request') {
    throw new RuntimeException('route traversal test failed');
}
echo '[OK] route traversal' . PHP_EOL;
$passed++;

echo 'Passed: ' . $passed . '/5' . PHP_EOL;
