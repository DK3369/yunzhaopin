<?php
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require_once ROOT . 'app/include/pin.php';

$zhFile = DATA_PATH . 'lang/auto/zh_cn.php';
$enFile = DATA_PATH . 'lang/auto/en_us.php';
$zh = include $zhFile;
$en = include $enFile;

function translateLocal($text)
{
    static $exact = array(
        '网络错误' => 'Network error', '请求中' => 'Requesting...', '正在加载' => 'Loading...',
        '向右旋转' => 'Rotate right', '向左旋转' => 'Rotate left', '忽略' => 'Ignore',
        '对齐方式' => 'Alignment', '首行缩进' => 'First line indent', '下划线' => 'Underline',
        '前插入行' => 'Insert row before', '前插入列' => 'Insert column before',
        '拆分成行' => 'Split into rows', '拆分成列' => 'Split into columns',
        '完全拆分单元格' => 'Split cell completely', '表格前插入行' => 'Insert row before table',
        '背景色' => 'Background color', '段前距' => 'Space before', '段后距' => 'Space after',
        '左浮动' => 'Float left', '右浮动' => 'Float right', '行间距' => 'Line spacing',
        '自动排版' => 'Auto format', '百度应用' => 'Baidu Apps', '明显强调' => 'Strong emphasis',
        '元素路径' => 'Element path', '平均分布各行' => 'Distribute rows evenly',
        '平均分布各列' => 'Distribute columns evenly', '按数值大小升序' => 'Sort ascending',
        '按数值大小降序' => 'Sort descending', '边框底纹' => 'Border and shading',
        '表格隔行变色' => 'Alternating row colors', '取消表格隔行变色' => 'Remove alternating colors',
        '红蓝相间' => 'Red-blue alternating', '三色渐变' => 'Three-color gradient',
        '标准颜色' => 'Standard colors', '主题颜色' => 'Theme colors', '符号转换' => 'Symbol conversion',
        '全角转半角' => 'Full-width to half-width', '半角转全角' => 'Half-width to full-width',
        '无背景色' => 'No background color', '有背景色' => 'With background color',
        '横向重复' => 'Repeat horizontally', '纵向重复' => 'Repeat vertically',
        '百度一下' => 'Baidu Search', '无浮动' => 'No float', '在线附件' => 'Online attachment',
        '视频尺寸' => 'Video size', '独占一行' => 'Block display', '点击选中' => 'Click to select',
        '背景颜色' => 'Background color', '图文混排' => 'Text and image layout',
        '科技论文' => 'Academic paper', '缩放背景' => 'Scale background',
        '上一个' => 'Previous', '下一个' => 'Next', '兔斯基' => 'Tuzki', '绿豆蛙' => 'Lvdouwa',
        '快捷键' => 'Shortcut', '给选中字加下划线' => 'Underline selected text',
        '日' => 'Sun',
    );
    if (isset($exact[$text])) {
        return $exact[$text];
    }
  return null;
}

function placeEnFull($zhText)
{
    if (preg_match('/^(.+)(特别行政区)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' SAR';
    }
    if (preg_match('/^(.+)(自治州)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' Autonomous Prefecture';
    }
    if (preg_match('/^(.+)(自治区)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' Autonomous Region';
    }
    if (preg_match('/^(.+)(区)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' District';
    }
    if (preg_match('/^(.+)(县)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' County';
    }
    if (preg_match('/^(.+)(市)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' City';
    }
    if (preg_match('/^(.+)(省)$/u', $zhText, $m)) {
        return ucfirst(Pinyin($m[1], 1)) . ' Province';
    }
    if (preg_match('/^省直辖(行政单位|县级行政单位)$/u', $zhText, $m)) {
        return 'Provincially administered ' . ($m[1] === '行政单位' ? 'units' : 'county-level units');
    }
    return null;
}

$fixed = 0;
foreach ($en as $key => $enVal) {
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $enVal)) {
        continue;
    }
    $zhText = $zh[$key] ?? '';
    $new = translateLocal($zhText);
    if ($new === null) {
        $new = placeEnFull($zhText);
    }
    if ($new === null && $enVal === $zhText) {
        $new = ucfirst(Pinyin($zhText, 1));
    }
    if ($new !== null && $new !== $enVal && !preg_match('/[\x{4e00}-\x{9fff}]/u', $new)) {
        $en[$key] = $new;
        $fixed++;
    }
}

$out = "<?php\n\nreturn " . var_export($en, true) . ";\n";
file_put_contents($enFile, $out);

$remain = 0;
foreach ($en as $v) {
    if (preg_match('/[\x{4e00}-\x{9fff}]/u', $v)) {
        $remain++;
    }
}
echo "Fixed: $fixed, EN still with Chinese: $remain\n";
