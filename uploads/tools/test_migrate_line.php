<?php
define('ROOT', dirname(__DIR__) . '/');
$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$vmap = [];
foreach ($zh as $k => $v) {
    if (is_string($v) && $v !== '') {
        if (!isset($vmap[$v])) $vmap[$v] = [];
        $vmap[$v][] = $k;
    }
}

function yunKey($key) { return "{yun:}t key='" . $key . "'{/yun}"; }

$line = '                            <el-option label="个人姓名" value="1"></el-option>';
$newLine = preg_replace_callback(
    '/\b(label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text|inactive-text|active-text)\s*=\s*"([^"]*[\x{4e00}-\x{9fff}][^"]*)"/u',
    function ($m) use ($vmap) {
        $text = $m[2];
        $keys = $vmap[$text];
        $key = $keys[0];
        foreach ($keys as $k) { if (strpos($k, 'admin_user_') === 0) { $key = $k; break; } }
        return $m[1] . '="' . yunKey($key) . '"';
    }, $line
);
echo "IN:  $line\nOUT: $newLine\n";

$file = ROOT . 'app/template/admin/user/users/component/renzheng_logo.vue';
$content = file_get_contents($file);
$lines = explode("\n", $content);
echo "Line 9: " . $lines[8] . "\n";
$sec = 'template';
foreach ($lines as $i => $l) {
    if ($i < 8 || $i > 10) continue;
    $nl = preg_replace_callback(
        '/\b(label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text|inactive-text|active-text)\s*=\s*"([^"]*[\x{4e00}-\x{9fff}][^"]*)"/u',
        function ($m) use ($vmap) {
            $text = $m[2];
            if (preg_match('/\{yun:\}t/u', $text)) return $m[0];
            $keys = $vmap[$text] ?? ['NEW'];
            $key = $keys[0];
            return $m[1] . '="' . yunKey($key) . '"';
        }, $l
    );
    echo "L$i changed: " . ($nl !== $l ? 'YES' : 'NO') . " => $nl\n";
}
