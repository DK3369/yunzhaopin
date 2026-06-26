<?php
/**
 * Fix PHP files where // comment and code were merged onto one line (translation bug).
 * Pattern: //...Chinese or English...    $var = ...
 * Restores newline before the code portion.
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);

$skip = '/vendor|PHPExcel|install\/|data\/lang|tools\/|dbbackup|PHPWord|tcpdf|ueditor/i';
$fixed = 0;
$files = 0;

foreach (array('app', 'admin', 'member', 'api', 'wap') as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) continue;

        $content = file_get_contents($f->getPathname());
        $tokens = token_get_all($content);
        $changed = false;

        foreach ($tokens as &$token) {
            if (!is_array($token)) continue;
            if (!in_array($token[0], array(T_COMMENT, T_DOC_COMMENT), true)) continue;
            $raw = $token[1];
            if (!preg_match('~^//~', $raw)) continue;
            // Code accidentally merged into // comment (translation bug)
            if (!preg_match('~^//(.+?)(\s+)((?:\$[a-zA-Z_][\w]*\s*(?:=|\-\>|::)|if\s*\(|foreach\s*\(|while\s*\(|for\s*\(|switch\s*\(|function\s+\w|return\s|else\b).*)$~s', $raw, $m)) continue;

            $commentPart = $m[1];
            $spaces = $m[2];
            $codePart = $m[3];
            if (preg_match('/[;{}=]/', $commentPart)) continue;
            if (mb_strlen($commentPart) > 80) continue;

            $indent = preg_match('/^(\s+)/', $spaces, $im) ? $im[1] : "\t\t";
            if ($indent === ' ' || $indent === '  ') $indent = '            ';
            $token[1] = '//' . rtrim($commentPart) . "\n" . $indent . ltrim($codePart);
            $changed = true;
        }
        unset($token);

        if (!$changed) continue;
        $files++;
        $out = '';
        foreach ($tokens as $token) $out .= is_array($token) ? $token[1] : $token;
        echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        if (!$dryRun) file_put_contents($f->getPathname(), $out);
        $fixed++;
    }
}

echo "\nFixed $fixed files\n";
