<?php
/**
 * Wrap model layer msg/error assignments with $this->at('lang_key').
 * Usage: php tools/wrap_model_msg.php [--dry-run]
 * Note: run convert_autotext_to_keys.php after adding new Chinese strings to lang pack.
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$dirs = array('app/model', 'member', 'api/wxapp', 'app/controller');
$fixed = 0;
$files = 0;

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        $c = file_get_contents($f->getPathname());
        if (!preg_match('/extends\s+(model|common|wxapp_controller|adminCommon|company|user|member)/', $c)) continue;
        $orig = $c;
        // ['msg'] = '中文';
        $c = preg_replace_callback(
            "/(\\[(?:'msg'|\"msg\"|'error'|\"error\"|'errmsg'|\"errmsg\"|'linkMsg'|\"linkMsg\")\\]\s*=\\s*)'([^'\\\\]*(?:\\\\.[^'\\\\]*)*[\x{4e00}-\x{9fff}][^'\\\\]*(?:\\\\.[^'\\\\]*)*)'(\\s*;)/u",
            function ($m) {
                if (strpos($m[0], 'autoText') !== false || strpos($m[0], 'yun_auto_t') !== false) {
                    return $m[0];
                }
                return $m[1] . '$this->autoText(\'' . str_replace("'", "\\'", $m[2]) . '\')' . $m[3];
            },
            $c
        );
        // 'msg' => '中文'
        $c = preg_replace_callback(
            "/((?:'msg'|\"msg\"|'error'|\"error\"|'linkMsg'|\"linkMsg\")\s*=>\s*)'([^'\\\\]*(?:\\\\.[^'\\\\]*)*[\x{4e00}-\x{9fff}][^'\\\\]*(?:\\\\.[^'\\\\]*)*)'/u",
            function ($m) {
                if (strpos($m[0], 'autoText') !== false) {
                    return $m[0];
                }
                return $m[1] . '$this->autoText(\'' . str_replace("'", "\\'", $m[2]) . '\')';
            },
            $c
        );
        if ($c !== $orig) {
            $files++;
            $fixed += substr_count($orig, 'autoText(') - substr_count($c, 'autoText(');
            $fixed = abs(substr_count($c, 'autoText(') - substr_count($orig, 'autoText('));
            if (!$dryRun) {
                file_put_contents($f->getPathname(), $c);
            }
            echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        }
    }
}
echo "Files: $files\n";
