<?php

include (dirname(dirname(__FILE__)) . '/global.php');
$pageType       =   'wap';
$Dir            =   str_replace("/", "\\", dirname(__FILE__));
$DirNameList    =   explode('\\', $Dir);
$ModuleName     =   end($DirNameList);
$DirName        =   end($DirNameList);

include (LIB_PATH . 'init.php');
?>