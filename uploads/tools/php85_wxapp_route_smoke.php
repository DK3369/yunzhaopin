<?php
/**
 * Read-only wxapp routing/CORS smoke tests for the local PHP 8.5 site.
 *
 * Usage:
 *   php tools/php85_wxapp_route_smoke.php
 */
$baseUrl = getenv('PHP85_SMOKE_BASE_URL');
if ($baseUrl === false || $baseUrl === '') {
    $baseUrl = 'http://127.0.0.1';
}
$host = getenv('PHP85_SMOKE_HOST');
if ($host === false || $host === '') {
    $host = 'dev.test';
}

function php85SmokeRequest($baseUrl, $host, $path, $method = 'GET', $headers = array())
{
    $responseHeaders = array();
    $handle = curl_init(rtrim($baseUrl, '/') . $path);
    $requestHeaders = array_merge(array('Host: ' . $host), $headers);
    curl_setopt_array($handle, array(
        CURLOPT_RETURNTRANSFER => true,
        CURLOPT_FOLLOWLOCATION => false,
        CURLOPT_CUSTOMREQUEST => $method,
        CURLOPT_HTTPHEADER => $requestHeaders,
        CURLOPT_CONNECTTIMEOUT => 5,
        CURLOPT_TIMEOUT => 20,
        CURLOPT_HEADERFUNCTION => function ($curl, $line) use (&$responseHeaders) {
            $length = strlen($line);
            $parts = explode(':', $line, 2);
            if (count($parts) === 2) {
                $name = strtolower(trim($parts[0]));
                $responseHeaders[$name] = trim($parts[1]);
            }
            return $length;
        }
    ));
    $body = curl_exec($handle);
    if ($body === false) {
        $error = curl_error($handle);
                throw new RuntimeException($error);
    }
    $status = curl_getinfo($handle, CURLINFO_RESPONSE_CODE);
        return array('status' => $status, 'headers' => $responseHeaders, 'body' => $body);
}

function php85SmokeJson($response)
{
    $json = json_decode($response['body'], true);
    if (!is_array($json)) {
        throw new RuntimeException('response is not JSON');
    }
    return $json;
}

$cases = array(
    array('public route', '/api/wxapp/index.php?m=public&c=com', 'GET', array(), 200, 0),
    array('route traversal', '/api/wxapp/index.php?m=..%2Fpublic&c=com', 'GET', array(), 400, 400),
    array('invalid member scope', '/api/wxapp/index.php?h=admin&m=index&c=index', 'GET', array(), 400, 400),
    array('missing model', '/api/wxapp/index.php?m=missing_model&c=index', 'GET', array(), 400, 404),
    array('missing action', '/api/wxapp/index.php?m=public&c=missing_action', 'GET', array(), 400, 404)
);

$passed = 0;
foreach ($cases as $case) {
    list($name, $path, $method, $headers, $expectedStatus, $expectedError) = $case;
    $response = php85SmokeRequest($baseUrl, $host, $path, $method, $headers);
    $json = php85SmokeJson($response);
    if ($response['status'] !== $expectedStatus || (int) $json['error'] !== $expectedError) {
        throw new RuntimeException($name . ' failed');
    }
    if (empty($response['headers']['x-request-id'])) {
        throw new RuntimeException($name . ' missing request id');
    }
    echo '[OK] ' . $name . PHP_EOL;
    $passed++;
}

$allowed = php85SmokeRequest(
    $baseUrl,
    $host,
    '/api/wxapp/index.php?m=public&c=com',
    'GET',
    array('Origin: http://dev.test')
);
if (($allowed['headers']['access-control-allow-origin'] ?? '') !== 'http://dev.test'
    || ($allowed['headers']['access-control-allow-credentials'] ?? '') !== 'true'
) {
    throw new RuntimeException('allowed CORS origin failed');
}
echo '[OK] allowed CORS origin' . PHP_EOL;
$passed++;

$denied = php85SmokeRequest(
    $baseUrl,
    $host,
    '/api/wxapp/index.php?m=public&c=com',
    'GET',
    array('Origin: https://evil.example')
);
if (isset($denied['headers']['access-control-allow-origin'])) {
    throw new RuntimeException('denied CORS origin was reflected');
}
echo '[OK] denied CORS origin' . PHP_EOL;
$passed++;

$preflight = php85SmokeRequest(
    $baseUrl,
    $host,
    '/api/wxapp/index.php?m=public&c=com',
    'OPTIONS',
    array('Origin: http://dev.test')
);
if ($preflight['status'] !== 204
    || ($preflight['headers']['access-control-allow-origin'] ?? '') !== 'http://dev.test'
) {
    throw new RuntimeException('allowed preflight failed');
}
echo '[OK] allowed preflight' . PHP_EOL;
$passed++;

$deniedPreflight = php85SmokeRequest(
    $baseUrl,
    $host,
    '/api/wxapp/index.php?m=public&c=com',
    'OPTIONS',
    array('Origin: https://evil.example')
);
$deniedJson = php85SmokeJson($deniedPreflight);
if ($deniedPreflight['status'] !== 403 || (int) $deniedJson['error'] !== 403) {
    throw new RuntimeException('denied preflight failed');
}
echo '[OK] denied preflight' . PHP_EOL;
$passed++;

echo 'Passed: ' . $passed . '/9' . PHP_EOL;
