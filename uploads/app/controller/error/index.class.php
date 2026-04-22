<?php

class index_controller extends common{
	function index_action(){
		$this->seo("error");
		$this->yun_tpl(array('index'));
	}
}
?>