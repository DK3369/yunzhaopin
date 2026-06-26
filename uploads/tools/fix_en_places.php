<?php
/**
 * Fix EN translations that still contain Chinese (mostly place names).
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require_once ROOT . 'app/include/pin.php';

$zhFile = DATA_PATH . 'lang/auto/zh_cn.php';
$enFile = DATA_PATH . 'lang/auto/en_us.php';
$zh = include $zhFile;
$en = include $enFile;

function placeEn($zhText)
{
    if (preg_match('/^(.+)(区)$/u', $zhText, $m)) {
        $py = ucfirst(Pinyin($m[1], 1));
        return $py . ' District';
    }
    if (preg_match('/^(.+)(县)$/u', $zhText, $m)) {
        $py = ucfirst(Pinyin($m[1], 1));
        return $py . ' County';
    }
    if (preg_match('/^(.+)(市)$/u', $zhText, $m)) {
        $py = ucfirst(Pinyin($m[1], 1));
        return $py . ' City';
    }
    if (preg_match('/^(.+)(省)$/u', $zhText, $m)) {
        $py = ucfirst(Pinyin($m[1], 1));
        return $py . ' Province';
    }
    if (preg_match('/^(.+)(自治州)$/u', $zhText, $m)) {
        $py = ucfirst(Pinyin($m[1], 1));
        return $py . ' Autonomous Prefecture';
    }
    if (preg_match('/^(.+)(自治区)$/u', $zhText, $m)) {
        $py = ucfirst(Pinyin($m[1], 1));
        return $py . ' Autonomous Region';
    }
    if (preg_match('/^省直辖(行政单位|县级行政单位)$/u', $zhText, $m)) {
        return 'Provincially administered ' . ($m[1] === '行政单位' ? 'units' : 'county-level units');
    }
    return null;
}

function fixMixedEn($enVal, $zhText)
{
    // e.g. "广灵 County" -> use zh source
    $fromZh = placeEn($zhText);
    if ($fromZh !== null) {
        return $fromZh;
    }
    // Strip Chinese chars, keep English remainder
    if (preg_match('/^([\x{4e00}-\x{9fff}]+)\s*(.+)$/u', $enVal, $m)) {
        $fromPart = placeEn($m[1] . (preg_match('/County|District|City|Province/', $m[2]) ? '' : ''));
        if ($fromPart) return $fromPart;
        $py = ucfirst(Pinyin($m[1], 1));
        return trim($py . ' ' . $m[2]);
    }
    // Pure Chinese in EN
    if (preg_match('/^[\x{4e00}-\x{9fff}]+$/u', $enVal)) {
        $py = ucfirst(Pinyin($enVal, 1));
        return $py !== '' ? $py : $enVal;
    }
    return null;
}

$fixed = 0;
$skipped = 0;
foreach ($en as $key => $enVal) {
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $enVal)) {
        continue;
    }
    $zhText = isset($zh[$key]) ? $zh[$key] : '';
    $new = fixMixedEn($enVal, $zhText);
    if ($new !== null && $new !== $enVal && !preg_match('/[\x{4e00}-\x{9fff}]/u', $new)) {
        $en[$key] = $new;
        $fixed++;
    } else {
        $skipped++;
    }
}

function exportPhp($data, $file)
{
    $out = "<?php\n\nreturn " . var_export($data, true) . ";\n";
    file_put_contents($file, $out);
}

exportPhp($en, $enFile);

$remain = 0;
foreach ($en as $v) {
    if (preg_match('/[\x{4e00}-\x{9fff}]/u', $v)) $remain++;
}
echo "Fixed: $fixed, Skipped: $skipped, EN still with Chinese: $remain\n";
