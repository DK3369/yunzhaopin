<?php
define('ROOT', dirname(__DIR__) . '/');
$remove = [
    'admin_vue_00041', 'admin_vue_00059', 'admin_vue_00060', 'admin_vue_00062',
    'admin_vue_00070', 'admin_vue_00072', 'admin_vue_00074',
    'admin_vue_00104', 'admin_vue_00105', 'admin_vue_00106', 'admin_vue_00107',
    'admin_vue_00108', 'admin_vue_00109', 'admin_vue_00110', 'admin_vue_00111',
    'admin_vue_00112', 'admin_vue_00113', 'admin_vue_00114', 'admin_vue_00115',
    'admin_vue_00116', 'admin_vue_00117',
];
$add = [
    'admin_vue_00118' => ['批量设置属性', 'Batch set attributes'],
    'admin_vue_00119' => ['修改新闻', 'Edit news'],
    'admin_vue_00120' => ['添加新闻', 'Add news'],
    'admin_vue_00121' => ['修改场地', 'Edit venue'],
    'admin_vue_00122' => ['修改图片', 'Edit image'],
];
foreach (['zh_cn', 'en_us'] as $lang) {
    $path = ROOT . "data/lang/auto/{$lang}.php";
    $data = include $path;
    foreach ($remove as $k) unset($data[$k]);
    foreach ($add as $k => $pair) {
        $data[$k] = $lang === 'zh_cn' ? $pair[0] : $pair[1];
    }
    // trim bad whitespace keys
    if (isset($data['admin_vue_00040'])) $data['admin_vue_00040'] = trim($data['admin_vue_00040']);
    if (isset($data['admin_vue_00042'])) $data['admin_vue_00042'] = trim($data['admin_vue_00042']);
    if (isset($data['admin_vue_00043'])) $data['admin_vue_00043'] = trim($data['admin_vue_00043']);
    if (isset($data['admin_vue_00044'])) $data['admin_vue_00044'] = trim($data['admin_vue_00044']);
    $out = "<?php\nreturn array(\n";
    foreach ($data as $k => $v) {
        $out .= "  '" . addslashes($k) . "' => '" . addslashes($v) . "',\n";
    }
    $out .= ");\n";
    file_put_contents($path, $out);
    echo "$lang: " . count($data) . " keys\n";
}
