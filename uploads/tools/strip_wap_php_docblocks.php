<?php
define('ROOT', dirname(__DIR__) . '/');
$dirs = array('api/wxapp', 'app/controller/wap');
$skip = array('wap.enum.php');

function cleanDocLine($line)
{
    if (preg_match('/^\s*\*[\s@]/', $line) && preg_match('/[\x{4e00}-\x{9fff}]/u', $line)) {
        if (preg_match('/^\s*\*\s*@/', $line)) {
            return preg_replace('/(@desc|@param)\s+[\x{4e00}-\x{9fff}\w\s，。、：；！？（）\-]+/u', '$1', $line);
        }
        return preg_replace('/^(\s*\*).*/u', '$1/', $line);
    }
    if (preg_match('/^\s*\/\*\*?[\x{4e00}-\x{9fff}]/u', trim($line))) {
        return "    /**\n";
    }
    return $line;
}

$n = 0;
foreach ($dirs as $dir) {
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT . $dir));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        foreach ($skip as $s) {
            if (strpos($rel, $s) !== false) {
                continue 2;
            }
        }
        $lines = file($f->getPathname());
        $out = array();
        $changed = false;
        foreach ($lines as $line) {
            $new = cleanDocLine($line);
            if ($new !== $line) {
                $changed = true;
            }
            $out[] = $new;
        }
        if ($changed) {
            file_put_contents($f->getPathname(), implode('', $out));
            echo "$rel\n";
            $n++;
        }
    }
}
echo "Docblocks cleaned: $n files\n";
