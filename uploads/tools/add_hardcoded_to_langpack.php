<?php
/**
 * Add missing lang keys from tools/hardcoded_php.json missing_pack section.
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

$dryRun = in_array('--dry-run', $argv ?? array(), true);
$hardFile = ROOT . 'tools/hardcoded_php.json';
if (!is_file($hardFile)) {
    fwrite(STDERR, "Run scan_hardcoded_php.php first.\n");
    exit(1);
}

$issues = json_decode(file_get_contents($hardFile), true);
$entries = $issues['missing_pack'] ?? array();

$zhFile = DATA_PATH . 'lang/auto/zh_cn.php';
$enFile = DATA_PATH . 'lang/auto/en_us.php';
$zh = include $zhFile;
$en = include $enFile;
$zhByValue = array_flip($zh);

$modMax = array();
foreach (array_keys($zh) as $key) {
    if (preg_match('/^([a-z_]+)_(\d+)$/', $key, $m)) {
        $modMax[$m[1]] = max($modMax[$m[1]] ?? 0, (int) $m[2]);
    }
}

function nextKey($module, &$modMax)
{
    $modMax[$module] = ($modMax[$module] ?? 0) + 1;
    return $module . '_' . str_pad($modMax[$module], 5, '0', STR_PAD_LEFT);
}

function guessModule($rel)
{
    if (strpos($rel, 'app/model/') === 0) {
        return 'model';
    }
    if (strpos($rel, 'member/com/') === 0) {
        return 'member_com';
    }
    if (strpos($rel, 'member/user/') === 0) {
        return 'member_user';
    }
    if (strpos($rel, 'admin/') === 0) {
        return 'admin';
    }
    return 'common';
}

function translateEn($zhText, $zhPack, $enPack, $zhByValue)
{
    $patterns = array(
        '/^请填写(.+)$/u' => 'Please enter $1',
        '/^请输入(.+)$/u' => 'Please enter $1',
        '/^请选择(.+)$/u' => 'Select $1',
        '/^(.+)成功！$/u' => '$1 successful!',
        '/^(.+)成功$/u' => '$1 successful',
        '/^(.+)失败！$/u' => '$1 failed!',
        '/^(.+)失败$/u' => '$1 failed',
        '/^(.+)设置失败！\(ID:$/u' => '$1 update failed (ID:',
        '/^更新(.+)\(ID：$/u' => 'Updated $1 (ID:',
        '/^删除(.+)\(ID：$/u' => 'Deleted $1 (ID:',
    );
    foreach ($patterns as $pat => $tpl) {
        if (preg_match($pat, $zhText, $m)) {
            $inner = isset($zhByValue[$m[1]]) ? $enPack[$zhByValue[$m[1]]] : $m[1];
            if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $inner)) {
                return preg_replace('/\$(\d+)/', $inner, $tpl);
            }
        }
    }
    static $dict = array(
        '非法操作！' => 'Invalid operation!',
        '操作失败！' => 'Operation failed!',
        '取消失败！' => 'Cancel failed!',
        '取消收藏失败' => 'Failed to remove favorite',
        '职位推广取消成功' => 'Job promotion cancelled successfully',
        '职位推广取消失败' => 'Failed to cancel job promotion',
        '职位信息查询失败' => 'Failed to query job information',
        '预约刷新关闭成功' => 'Scheduled refresh disabled successfully',
        '投递成功' => 'Application submitted successfully',
        '设置成功！' => 'Settings saved successfully!',
        '请填写联系人姓名' => 'Please enter contact name',
        '请填写反馈内容' => 'Please enter feedback content',
        '删除城市失败' => 'Failed to delete city',
        '您的套餐暂无权限' => 'Your plan does not include this feature',
        '套餐信息更新成功！' => 'Plan updated successfully!',
        '套餐信息更新失败！' => 'Failed to update plan!',
        '请填写汇款银行' => 'Please enter remittance bank',
        '请填写汇入账号' => 'Please enter receiving account',
    );
    if (isset($dict[$zhText])) {
        return $dict[$zhText];
    }
    if (isset($zhByValue[$zhText])) {
        return $enPack[$zhByValue[$zhText]];
    }
    return null;
}

function writeLangFile($path, $data)
{
    ksort($data);
    $out = "<?php\n\n// Auto lang keys: module_NNNNN\nreturn array (\n";
    foreach ($data as $k => $v) {
        $out .= '  ' . var_export($k, true) . ' => ' . var_export($v, true) . ",\n";
    }
    $out .= ");\n";
    file_put_contents($path, $out);
}

$added = 0;
foreach ($entries as $text => $rel) {
    if (isset($zhByValue[$text])) {
        continue;
    }
    $module = guessModule($rel);
    $enText = translateEn($text, $zh, $en, $zhByValue);
    if ($enText === null || preg_match('/[\x{4e00}-\x{9fff}]/u', $enText)) {
        $enText = '[TODO] ' . $text;
    }
    $key = nextKey($module, $modMax);
    $zh[$key] = $text;
    $en[$key] = $enText;
    $zhByValue[$text] = $key;
    $added++;
    echo "ADD $key: $text => $enText\n";
}

echo "Added: $added keys\n";
if (!$dryRun && $added > 0) {
    writeLangFile($zhFile, $zh);
    writeLangFile($enFile, $en);
    echo "Written zh_cn.php / en_us.php (" . count($zh) . " keys)\n";
}
