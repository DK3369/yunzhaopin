<?php

include_once(dirname(dirname(dirname(__FILE__)))."/global.php");
include_once(PLUS_PATH."pimg_cache.php");
if($_GET['ad_id']&&$_GET['classid']){
	$ad_id = "ad_".$_GET['ad_id'];
	$ad_class_id = intval($_GET['classid']);
	if($ad_label[$ad_class_id][$ad_id]['did']=="-1" || $ad_label[$ad_class_id][$ad_id]['did']=="0" || stripos($ad_label[$ad_class_id][$ad_id]['did'],$_SESSION['did'])!==false){
		$ad_info = $ad_label[$ad_class_id][$ad_id]['html'];
		$ad_info=str_replace("\n","",$ad_info);
		$ad_info=str_replace("\r","",$ad_info);
		$ad_info=str_replace("'","\'",$ad_info);
		echo "document.write('$ad_info');";
	}
}
?>