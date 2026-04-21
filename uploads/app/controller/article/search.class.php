<?php
/*
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
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