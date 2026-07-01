<?php
define('ROOT', dirname(__DIR__) . '/');
$file = ROOT . 'app/template/admin/user/users/component/renzheng_logo.vue';
$content = file_get_contents($file);
$lines = explode("\n", $content);

function sectionAt($lines, $idx) {
    $sec = 'template';
    for ($i = 0; $i <= $idx; $i++) {
        if (preg_match('/<template[\s>]/', $lines[$i])) $sec = 'template';
        if (preg_match('/<script[\s>]/', $lines[$i])) $sec = 'script';
        if (preg_match('/<style[\s>]/', $lines[$i])) $sec = 'style';
        if (preg_match('/<\/template>/', $lines[$i])) $sec = 'other';
        if (preg_match('/<\/script>/', $lines[$i])) $sec = 'other';
        if (preg_match('/<\/style>/', $lines[$i])) $sec = 'other';
    }
    return $sec;
}

function isCommentLine($line) {
    $t = ltrim($line);
    return $t === '' || strpos($t, '//') === 0 || strpos($t, '*') === 0 || strpos($t, '/*') === 0
        || strpos($t, '<!--') === 0 || preg_match('/^\s*<!--/', $line);
}

function inHtmlComment($lines, $idx) {
    for ($i = $idx; $i >= 0; $i--) {
        if (strpos($lines[$i], '<!--') !== false) {
            if (strpos($lines[$i], '-->') === false || strpos($lines[$i], '<!--') > strrpos($lines[$i], '-->')) return true;
        }
        if (strpos($lines[$i], '-->') !== false) return false;
    }
    return false;
}

$changed = 0;
for ($i = 0; $i < count($lines); $i++) {
    $sec = sectionAt($lines, $i);
    $comment = isCommentLine($lines[$i]);
    $htmlComment = inHtmlComment(explode("\n", $lines[$i]), 0);
    if (preg_match('/[\x{4e00}-\x{9fff}]/u', $lines[$i]) && !$comment && !$htmlComment) {
        if ($sec !== 'template' && $sec !== 'script') {
            echo "L$i sec=$sec (skip): " . trim($lines[$i]) . "\n";
        }
    }
}
echo "Total lines: " . count($lines) . "\n";

// Count lines that would be processed in template with Chinese
$n = 0;
for ($i = 0; $i < count($lines); $i++) {
    $sec = sectionAt($lines, $i);
    if ($sec !== 'template' && $sec !== 'script') continue;
    if (isCommentLine($lines[$i])) continue;
    if (inHtmlComment(explode("\n", $lines[$i]), 0)) continue;
    if (preg_match('/[\x{4e00}-\x{9fff}]/u', $lines[$i]) && !preg_match('/\{yun:\}t/u', $lines[$i])) {
        $n++;
        if ($n <= 5) echo "Process L$i sec=$sec: " . trim(substr($lines[$i], 0, 80)) . "\n";
    }
}
echo "Lines to process: $n\n";
