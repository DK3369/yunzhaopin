<?php
require __DIR__ . '/migrate_admin_vue.php';
$line = '                            <el-option label="个人姓名" value="1"></el-option>';
$sec = 'template';
$vmap = buildValueMap();
$newKeys = []; $keyNum = 1;
$path = 'app/template/admin/user/users/component/renzheng_logo.vue';

echo "isCommentLine: " . (isCommentLine($line)?'Y':'N') . "\n";
echo "inHtmlComment: " . (inHtmlComment(explode("\n", $line), 0)?'Y':'N') . "\n";
echo "skipBackendCompare: " . (skipBackendCompare($line)?'Y':'N') . "\n";
echo "concat+: " . (preg_match('/[\'"][\x{4e00}-\x{9fff}][^\'"]*[\'"]\s*\+/u', $line)?'Y':'N') . "\n";

$nl = migrateLine($line, $sec, $vmap, $path, $newKeys, $keyNum, 9999);
echo "OUT: $nl\n";
echo "Changed: " . ($nl !== $line ? 'yes' : 'no') . "\n";

// test line 39
$line2 = '<el-table-column prop="uid" label="用户ID" width="120" sortable="custom"></el-table-column>';
echo "\nLine2 skipBackend: " . (skipBackendCompare($line2)?'Y':'N') . "\n";
$nl2 = migrateLine($line2, 'template', $vmap, $path, $newKeys, $keyNum, 9999);
echo "OUT2: $nl2\n";
