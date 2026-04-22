<?php


if($_GET['id']){
	
	$id = intval($_GET['id']);
	include_once("../outside/".$id.".php");
	$list = $content;
	echo "document.write('$list');";
}
?>