<?php

class list_controller extends article_controller{
    /**
     * 职场资讯-热门话题-列表
     */
	function index_action(){
		$articleM	=	$this->MODEL('article');
        $class		=	$articleM->getGroup(array('id'=>(int)$_GET['nid']),array('field'=>"`name`"));
        $this->yunset("classname",$class['name']);
		
		$data['news_class']	=	$class['name'];
		$this->data			=	$data;
		$this->seo("newslist");
		
		$this->yun_tpl(array('list'));
	}
}
?>