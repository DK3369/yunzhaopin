<?php

class YunQrcode {

	//不保存成图片文件
	static function generatePng2($inputContent,$size){
		require_once LIB_PATH."phpqrcode.php";
		return QRcode::png($inputContent, false, QR_ECLEVEL_L, $size);
	}
}


?>