<?php

class evaluate_controller extends common{ 
	function evaluate_tpl($tpl){
		$this->yuntpl(array('default/evaluate/'.$tpl));
	}
	
	function imgWebUrl($imgPath){
		return $this->config['sy_weburl']."/".$imgPath;
	}	
	
	function create_uuid($prefix = "yun"){    //可以指定前缀
		$str 	= 	md5(uniqid(mt_rand(), true));   
		$uuid  	= 	substr($str,0,12);   
		return $prefix.$uuid; 
	}	
}
?>