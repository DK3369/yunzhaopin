<?php


function passwordCheck($pass, $salt = '', $oldpass = ''){

	if ($pass) {
    
		// 有oldpass传入，则证明是需要密码对比的
		if ($oldpass) {
			
			return md5(md5($pass).$salt) == $oldpass ? true : false;
		} else { // 生成密码
			
			return md5(md5($pass).$salt);
		} 
		
	} else {

		return false;
	}

}
?>