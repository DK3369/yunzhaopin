<?php

class topic_controller extends ask_controller{  
	function index_action(){
		$M	=	$this -> MODEL('ask');
		
		$where['add_time']	=	array('>',strtotime("-30 day"));
		
		$where['groupby']	=	'uid';
		
		$where['orderby']	=	'num';
		
		$where['limit']		=	'6';
		
		$recom=$M -> getAnswersList($where,array("field"=>"uid,count(id) as num,sum(support) as support,nickname,pic"));
		
		if($_GET['pid']){
			
			$CacheM		=	$this -> MODEL('cache');
			
			$CacheList	=	$CacheM -> GetCache(array('ask'));
			
			$data['ask_class_name']	=	$CacheList['ask_name'][$_GET['pid']];
			
			$data['ask_desc']		=	strip_tags($CacheList['ask_intro'][$_GET['pid']]);
		
		}else{
			$data['ask_class_name']	=	'';
			
			$data['ask_desc']		=	'';
		}
		$this->data=$data;
		
		$this->yunset('recom',$recom);
		
		$this->yunset("navtype","topic");
		
		$this->seo("ask_topic");
		
		$this->ask_tpl('topic');
	}
}
?>