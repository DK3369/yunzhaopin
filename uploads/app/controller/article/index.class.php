<?php

class index_controller extends article_controller{
    /**
     * 职场资讯
     */
	function index_action(){
		$this->seo("news");
		$this->yun_tpl(array('index'));
	} 
}
?>