<?php


error_reporting(0);

include(dirname(dirname(dirname(__FILE__)))."/data/plus/config.php");

include(dirname(dirname(dirname(__FILE__)))."/app/include/verify.class.php");

$capth = new verify($config['code_width'],$config['code_height'],$config['code_strlength'],$config['code_filetype'],$config['code_type']);

$capth->entry(); 

?>