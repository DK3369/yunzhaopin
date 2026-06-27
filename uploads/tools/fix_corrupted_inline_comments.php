<?php
/**
 * Fix PHP files where // comment and code were merged onto one line (translation bug).
 * Pattern: //...Chinese or English...    $var = ...
 * Restores newline before the code portion.
 *
 * Usage:
 *   php tools/fix_corrupted_inline_comments.php              # fix in place
 *   php tools/fix_corrupted_inline_comments.php --dry-run    # list only
 *   php tools/fix_corrupted_inline_comments.php --scan-only  # JSON report, no writes
 */
define('ROOT', dirname(__DIR__) . '/');
$argv = $argv ?? array();
$dryRun = in_array('--dry-run', $argv, true);
$scanOnly = in_array('--scan-only', $argv, true);

$skip = '/vendor|PHPExcel|install\/|data\/lang|tools\/|dbbackup|PHPWord|tcpdf|ueditor|include\/libs/i';
$fixed = 0;
$scanResults = array();

function corruptedCommentMatches($raw)
{
    if (!preg_match('~^//~', $raw)) {
        return false;
    }
    if (!preg_match('~^//(.+?)(\s+)((?:\$[a-zA-Z_][\w]*\s*=|if\s*\(|foreach\s*\(|while\s*\(|for\s*\(|switch\s*\(|function\s+\w|return\s|else\b).*)$~s', $raw, $m)) {
        return false;
    }
    $commentPart = $m[1];
    $codePart = $m[3];
    if (!preg_match('~^(?:\$[a-zA-Z_][\w]*\s*=|if\s*\(|foreach\s*\(|while\s*\(|for\s*\(|switch\s*\(|function\s+\w|return\b|else\b)~', trim($codePart))) {
        return false;
    }
    if (preg_match('/[;{}=]/', $commentPart)) {
        return false;
    }
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $commentPart)) {
        return false;
    }
    if (!preg_match('/[;{}]/', $codePart)) {
        return false;
    }
    if (strpos($commentPart, trim($codePart)) !== false) {
        return false;
    }
    if (mb_strlen($commentPart) > 80) {
        return false;
    }
    return array(
        'preview' => mb_substr(trim($raw), 0, 120),
        'code' => mb_substr(trim($codePart), 0, 80),
    );
}

function applyCommentFix(&$tokens)
{
    $changed = false;
    foreach ($tokens as &$token) {
        if (!is_array($token)) {
            continue;
        }
        if (!in_array($token[0], array(T_COMMENT, T_DOC_COMMENT), true)) {
            continue;
        }
        $raw = $token[1];
        $match = corruptedCommentMatches($raw);
        if (!$match) {
            continue;
        }
        if (!preg_match('~^//(.+?)(\s+)((?:\$[a-zA-Z_][\w]*\s*=|if\s*\(|foreach\s*\(|while\s*\(|for\s*\(|switch\s*\(|function\s+\w|return\s|else\b).*)$~s', $raw, $m)) {
            continue;
        }
        $commentPart = $m[1];
        $spaces = $m[2];
        $codePart = $m[3];
        $indent = preg_match('/^(\s+)/', $spaces, $im) ? $im[1] : "\t\t";
        if ($indent === ' ' || $indent === '  ') {
            $indent = '            ';
        }
        $token[1] = '//' . rtrim($commentPart) . "\n" . $indent . ltrim($codePart);
        $changed = true;
    }
    unset($token);
    return $changed;
}

foreach (array('app', 'admin', 'member', 'api', 'wap') as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) {
        continue;
    }
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) {
            continue;
        }

        $content = file_get_contents($f->getPathname());
        $tokens = token_get_all($content);
        $hits = array();

        foreach ($tokens as $token) {
            if (!is_array($token)) {
                continue;
            }
            if (!in_array($token[0], array(T_COMMENT, T_DOC_COMMENT), true)) {
                continue;
            }
            $match = corruptedCommentMatches($token[1]);
            if ($match) {
                $hits[] = $match;
            }
        }

        if (!$hits) {
            continue;
        }

        if ($scanOnly) {
            $scanResults[] = array(
                'file' => $rel,
                'count' => count($hits),
                'hits' => $hits,
            );
            continue;
        }

        $tokens = token_get_all($content);
        if (!applyCommentFix($tokens)) {
            continue;
        }

        $fixed++;
        $out = '';
        foreach ($tokens as $token) {
            $out .= is_array($token) ? $token[1] : $token;
        }
        echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        if (!$dryRun) {
            file_put_contents($f->getPathname(), $out);
        }
    }
}

if ($scanOnly) {
    $report = array(
        'scanned_at' => date('c'),
        'file_count' => count($scanResults),
        'hit_count' => array_sum(array_map(function ($row) {
            return $row['count'];
        }, $scanResults)),
        'files' => $scanResults,
    );
    $reportPath = ROOT . 'tools/corrupted_comment_scan.json';
    file_put_contents($reportPath, json_encode($report, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
    echo "Scan complete: {$report['file_count']} files, {$report['hit_count']} hits\n";
    echo "Report: tools/corrupted_comment_scan.json\n";
    exit(0);
}

echo "\nFixed $fixed files\n";
