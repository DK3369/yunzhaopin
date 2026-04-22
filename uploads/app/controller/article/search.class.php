<?php

class search_controller extends article_controller{
	function index_action(){ 
	
		$articleM	=	$this->MODEL('article');
        $class		=	$articleM->getGroup(array('id'=>(int)$_GET['kid']),array('field'=>'`name`'));
        $this->yunset("classname",$class['name']);
		
		//新闻搜索结束
		$data['news_class']	=	$class['name'];
		$this->data			=	$data;
		$this->seo("newslist");
		$this->yun_tpl(array('search'));
	}
}
?>