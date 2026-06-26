<?php
$files = glob(dirname(__DIR__) . '/app/include/libs/sysplugins/smarty_internal_compile_*.php');
foreach ($files as $f) {
    $c = file_get_contents($f);
    $n = preg_replace("/yun_at\\('([a-z][a-z0-9_]*\\d+)'\\)/", "yun_at(\\'$1\\')", $c);
    if ($n !== $c) {
        file_put_contents($f, $n);
        echo basename($f) . "\n";
    }
}
