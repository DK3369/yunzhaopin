<?php
require __DIR__ . '/migrate_admin_vue.php';
$path = 'app/template/admin/user/users/component/renzheng_logo.vue';
$vmap = buildValueMap();
$newKeys = []; $keyNum = 1;
$fullPath = ROOT . $path;
$content = file_get_contents($fullPath);
$lines = explode("\n", $content);
$changed = 0;
for ($i = 0; $i < count($lines); $i++) {
    $sec = sectionAt($lines, $i);
    $nl = migrateLine($lines[$i], $sec, $vmap, $path, $newKeys, $keyNum, 9999);
    if ($nl !== $lines[$i]) {
        $changed++;
        if ($changed <= 8) echo "L$i sec=$sec\n  OLD: " . trim($lines[$i]) . "\n  NEW: " . trim($nl) . "\n\n";
    }
}
echo "Total changed lines: $changed\n";
echo "migrateFile changed: " . (migrateFile($path, $vmap, $newKeys, $keyNum, 9999, true) ? 'yes' : 'no') . "\n";
