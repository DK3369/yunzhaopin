<?php
/**
 * Fix comments broken by multi-line API translation:
 *   //The parameter
 *           $cache_id must be passed...
 * Merges continuation lines into a single // comment when they are not valid PHP.
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$skip = '/vendor|PHPExcel|install\/|data\/lang|tools\/|dbbackup|PHPWord|tcpdf|ueditor|tecentcode|aliyunemail/i';

function isPhpStatementStart($line)
{
    $line = ltrim($line);
    if ($line === '' || $line[0] === '}' || $line[0] === '{') return true;
    if (preg_match('~^(if|foreach|for|while|switch|return|function|class|public|private|protected|static|else|elseif|try|catch|throw|new|unset|echo|print|include|require)\b~', $line)) return true;
    if (preg_match('~^\$[a-zA-Z_][\w]*\s*(=|\-\>|::|\+\=|\-\=)~', $line)) return true;
    if (preg_match('~^[a-zA-Z_\\][\w]*\s*(\(|=)~', $line)) return true;
    return false;
}

$fixed = 0;
foreach (array('app', 'admin', 'member', 'api', 'wap') as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) continue;

        $lines = file($f->getPathname());
        if (!$lines) continue;
        $changed = false;
        $i = 0;
        while ($i < count($lines) - 1) {
            if (!preg_match('~^(\s*)//(.+)$~', rtrim($lines[$i], "\r\n"), $cm)) {
                $i++;
                continue;
            }
            $indent = $cm[1];
            $next = rtrim($lines[$i + 1], "\r\n");
            if (!preg_match('~^(\s+)(.+)$~', $next, $nm)) {
                $i++;
                continue;
            }
            $cont = $nm[2];
            if (isPhpStatementStart($cont)) {
                $i++;
                continue;
            }
            // Merge continuation into comment line
            $merged = rtrim($lines[$i], "\r\n") . ' ' . trim($cont) . "\n";
            $lines[$i] = $merged;
            array_splice($lines, $i + 1, 1);
            $changed = true;
        }
        if (!$changed) continue;
        $origContent = file_get_contents($f->getPathname());
        $out = implode('', $lines);
        if (!$dryRun) file_put_contents($f->getPathname(), $out);
        exec('php -l ' . escapeshellarg($f->getPathname()) . ' 2>&1', $lintOut, $codeAfter);
        if (!$dryRun && $codeAfter !== 0) {
            file_put_contents($f->getPathname(), $origContent);
            continue;
        }
        echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        $fixed++;
    }
}

echo "\nFixed $fixed files\n";
