<?php
define('ROOT', dirname(__DIR__) . '/');
$dirs = array('api/wxapp', 'app/controller/wap');

function repairContent($c)
{
    // Empty docblock followed by orphan * lines then */
    $c = preg_replace(
        '/\/\*\*\s*\*\/\s*\n\s*\*[^\n]*\n(\s*\*[^\n]*\n)*\s*\*\/\s*\n/s',
        "\n",
        $c
    );
    // Double closing */
    $c = preg_replace('/\/\*\*\s*\*\/\s*\n\s*\*\/\s*\n/s', "\n", $c);
    // /** */ alone before function - ok, leave
    // @desc     */ broken
    $c = preg_replace('/\/\*\*\s*\n\s*\*\s*@desc\s+\*\/\s*\n/s', "/**\n */\n", $c);
    $c = preg_replace('/\/\*\*\s*\n\s*\*\s*@desc\s+\*\/\s*/s', "/**\n */\n", $c);
    // Orphan lines after empty block: /**\n */\n     * foo\n     */\n
    $c = preg_replace(
        '/(\/\*\*\s*\n\s*\*\/\s*\n)(\s*\*[^\/][^\n]*\n)+\s*\*\/\s*\n/',
        "$1",
        $c
    );
    return $c;
}

$bad = array();
foreach ($dirs as $dir) {
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT . $dir));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $path = $f->getPathname();
        $c = file_get_contents($path);
        $new = repairContent($c);
        if ($new !== $c) {
            file_put_contents($path, $new);
        }
        exec('php -l ' . escapeshellarg($path) . ' 2>&1', $out, $code);
        if ($code !== 0) {
            $bad[$path] = implode("\n", $out);
        }
    }
}

if ($bad) {
    echo "Still broken:\n";
    foreach ($bad as $p => $msg) {
        echo str_replace(ROOT, '', $p) . ": $msg\n";
    }
} else {
    echo "All PHP files OK\n";
}
