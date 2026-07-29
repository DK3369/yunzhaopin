<?php
/**
 * Build a deterministic inventory of externally routable *_action methods.
 *
 * Usage:
 *   php tools/php85_interface_inventory.php
 */
define('ROOT', dirname(__DIR__) . DIRECTORY_SEPARATOR);

$scanDirs = array('app/controller', 'member', 'admin', 'api', 'wap/member');
$excluded = array(
    '/vendor/',
    '/templates_c/',
    '/uc_client-old/',
    '/install/',
    '/node_modules/'
);

function php85InventoryFiles($dirs, $excluded)
{
    $files = array();
    foreach ($dirs as $dir) {
        $path = ROOT . $dir;
        if (!is_dir($path)) {
            continue;
        }
        $iterator = new RecursiveIteratorIterator(
            new RecursiveDirectoryIterator($path, FilesystemIterator::SKIP_DOTS)
        );
        foreach ($iterator as $file) {
            if (!$file->isFile() || strtolower($file->getExtension()) !== 'php') {
                continue;
            }
            $relative = str_replace('\\', '/', substr($file->getPathname(), strlen(ROOT)));
            $skip = false;
            foreach ($excluded as $fragment) {
                if (strpos('/' . $relative, $fragment) !== false) {
                    $skip = true;
                    break;
                }
            }
            if (!$skip) {
                $files[$relative] = $file->getPathname();
            }
        }
    }
    ksort($files);
    return $files;
}

function php85InventoryClass($tokens)
{
    $count = count($tokens);
    for ($i = 0; $i < $count; $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_CLASS) {
            continue;
        }
        for ($j = $i + 1; $j < $count; $j++) {
            if (is_array($tokens[$j]) && $tokens[$j][0] === T_STRING) {
                return $tokens[$j][1];
            }
            if ($tokens[$j] === '{') {
                break;
            }
        }
    }
    return '';
}

function php85InventoryRoute($path, $class, $action)
{
    if (preg_match('#^api/wxapp/member/(user|com)/([^/]+)\.class\.php$#', $path, $match)) {
        return '/api/wxapp/index.php?h=' . $match[1] . '&m=' . $match[2] . '&c=' . $action;
    }
    if (preg_match('#^api/wxapp/model/([^/]+)\.class\.php$#', $path, $match)) {
        return '/api/wxapp/index.php?m=' . $match[1] . '&c=' . $action;
    }
    if (preg_match('#^app/controller/wap/([^/]+)\.class\.php$#', $path, $match)) {
        return '/wap/index.php?c=' . $match[1] . '&a=' . $action;
    }
    if (preg_match('#^app/controller/([^/]+)/([^/]+)\.class\.php$#', $path, $match)) {
        $route = '/index.php?m=' . $match[1];
        if ($match[2] !== 'index') {
            $route .= '&c=' . $match[2];
        }
        return $route . '&a=' . $action;
    }
    if (preg_match('#^member/(user|com)/model/([^/]+)\.class\.php$#', $path, $match)) {
        return '/member/index.php?c=' . $match[2] . '&a=' . $action;
    }
    if (strpos($path, 'admin/') === 0) {
        return '/admin/index.php?m=' . basename($path, '.class.php') . '&c=' . $action;
    }
    if (strpos($path, 'api/') === 0) {
        return '/' . $path . '#action=' . $action;
    }
    return '/' . $path . '#action=' . $action;
}

function php85InventoryClassification($path, $class, $action)
{
    $subject = strtolower($path . ' ' . $class . ' ' . $action);
    $writePattern = '/(^|_)(add|save|insert|edit|update|delete|del|remove|upload|apply|submit|login|logout|register|reg|send|bind|unbind|set|cancel|pay|buy|order|callback|notify|return|check|report|invite|down|collect|atn|click)(_|$)/i';

    if (strpos($subject, 'locoy') !== false) {
        return array('collection', 'signed', 'write', 'invalid_key');
    }
    if (preg_match('/(alipay|wxpay|tenpay|pay|finance|order|recharge|\bfk\b)/i', $subject)) {
        return array('payment', 'signed_or_authenticated', 'write', 'invalid_signature');
    }
    if (preg_match('#api/(uc|uc_php7|pw_api)/#i', $path)) {
        return array('federation', 'signed', 'callback', 'invalid_signature');
    }
    if (preg_match('/(upload|uppic|ajaxfile|image)/i', $subject)) {
        return array('upload', 'authenticated', 'write', 'unauthorized');
    }

    $authenticated = strpos($path, 'member/') === 0
        || strpos($path, 'api/wxapp/member/') === 0
        || strpos($path, 'admin/') === 0;
    $write = preg_match($writePattern, str_replace('-', '_', $action)) === 1;

    if ($authenticated) {
        return array(
            $write ? 'authenticated_write' : 'authenticated_read',
            strpos($path, 'admin/') === 0 ? 'admin' : 'authenticated',
            $write ? 'write' : 'read',
            'unauthorized'
        );
    }
    if ($write) {
        return array('public_write', 'public_or_token', 'write', 'missing_params');
    }
    if (strpos($path, 'api/') === 0) {
        return array('external_read', 'public_or_signed', 'read', 'missing_params');
    }
    return array('public_read', 'public', 'read', 'normal_response');
}

$actions = array();
foreach (php85InventoryFiles($scanDirs, $excluded) as $relative => $absolute) {
    $source = file_get_contents($absolute);
    if ($source === false) {
        fwrite(STDERR, "Unable to read: " . $relative . PHP_EOL);
        exit(1);
    }
    $tokens = token_get_all($source);
    $class = php85InventoryClass($tokens);
    $count = count($tokens);
    for ($i = 0; $i < $count; $i++) {
        if (!is_array($tokens[$i]) || $tokens[$i][0] !== T_FUNCTION) {
            continue;
        }
        $name = '';
        $line = $tokens[$i][2];
        for ($j = $i + 1; $j < $count; $j++) {
            if (is_array($tokens[$j]) && $tokens[$j][0] === T_STRING) {
                $name = $tokens[$j][1];
                break;
            }
            if ($tokens[$j] === '(') {
                break;
            }
        }
        if ($name === '' || substr($name, -7) !== '_action') {
            continue;
        }
        $action = substr($name, 0, -7);
        list($category, $access, $operation, $verification) =
            php85InventoryClassification($relative, $class, $action);
        $actions[] = array(
            'id' => $relative . ':' . $class . '::' . $name,
            'path' => $relative,
            'line' => $line,
            'class' => $class,
            'action' => $action,
            'route_hint' => php85InventoryRoute($relative, $class, $action),
            'category' => $category,
            'access' => $access,
            'operation' => $operation,
            'verification' => $verification
        );
    }
}

usort($actions, function ($left, $right) {
    return strcmp($left['id'], $right['id']);
});

$categories = array();
$verifications = array();
foreach ($actions as $action) {
    $categories[$action['category']] = isset($categories[$action['category']])
        ? $categories[$action['category']] + 1 : 1;
    $verifications[$action['verification']] = isset($verifications[$action['verification']])
        ? $verifications[$action['verification']] + 1 : 1;
}
ksort($categories);
ksort($verifications);

$report = array(
    'schema_version' => 1,
    'php_target' => '8.5.7',
    'action_count' => count($actions),
    'classified_count' => count($actions),
    'unclassified_count' => 0,
    'categories' => $categories,
    'verification_cases' => $verifications,
    'actions' => $actions
);

$output = ROOT . 'tools/php85_interface_inventory.json';
$json = json_encode($report, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE);
if ($json === false || file_put_contents($output, $json . PHP_EOL) === false) {
    fwrite(STDERR, "Unable to write inventory report" . PHP_EOL);
    exit(1);
}

echo 'Actions: ' . count($actions) . PHP_EOL;
echo 'Classified: ' . count($actions) . PHP_EOL;
echo 'Unclassified: 0' . PHP_EOL;
foreach ($categories as $category => $total) {
    echo '  ' . $category . ': ' . $total . PHP_EOL;
}
echo 'Report: ' . $output . PHP_EOL;

if (count($actions) === 0) {
    exit(1);
}
