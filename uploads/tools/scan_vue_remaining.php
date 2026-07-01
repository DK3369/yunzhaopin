<?php
define('ROOT', dirname(__DIR__) . '/');
function stripComments($c) {
    $c = preg_replace('/<!--[\s\S]*?-->/u', '', $c);
    $c = preg_replace('#/\*[\s\S]*?\*/#u', '', $c);
    $c = preg_replace('#(?<!:)//[^\n]*#u', '', $c);
    return $c;
}
$f = $argv[1] ?? 'app/template/admin/user/users/component/usersall.vue';
$raw = file_get_contents(ROOT . $f);
$code = stripComments($raw);
$code = preg_replace('/\{yun:\}t key=[^}]+\{\/yun\}/u', '', $code);
$code = preg_replace('/lc\([^)]*\)/u', '', $code);
preg_match_all('/["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $code, $m);
foreach (array_unique($m[1]) as $s) {
    if (mb_strlen($s) > 100) $s = mb_substr($s, 0, 100) . '...';
    echo $s . "\n---\n";
}
