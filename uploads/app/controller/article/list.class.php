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