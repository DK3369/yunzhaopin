<?php

class zph_controller extends common{  
	
	function Zphpublic_action(){
		if($this->config['sy_zph_web']=="2"){//后台已关闭
			header("location:".Url('error'));
		}
	}
}
?>