<?php

class camphoto_controller extends user{
	//摄像头图像保存
	function index_action(){
	
		header("Location: ../../member/index.php?c=uppic");
	}	
}
?>