<?php

class partapply_controller extends user{
    //兼职报名列表
	function index_action(){
		$this->public_action();
		$PartM			=   $this -> MODEL('part');
		
		$where['uid']	=  $this -> uid;
		 //分页链接
		$urlarr['c']	=	$_GET['c'];
        $urlarr['page']	=	'{{page}}';
        
        $pageurl		=	Url('member',$urlarr);

        $pageM			=	$this  -> MODEL('page');
        $pages			=	$pageM -> pageList('part_apply',$where,$pageurl,$_GET['page']);
		

			
            if($_GET['order']){
                
                $where['orderby']		=	$_GET['t'].','.$_GET['order'];
                $urlarr['order']		=	$_GET['order'];
                $urlarr['t']			=	$_GET['t'];
            }else{
                
                $where['orderby']		=	array('id,desc');
            }
            $where['limit']				=	$pages['limit'];
            
            $rows    	=   $PartM -> getPartSqList($where);
   
    $this->yunset("total",$pages['total']);
		$this->yunset("rows",$rows);
		
		$this->user_tpl('partapply');
	}
	//删除兼职报名
	function del_action(){
		if($_GET['id']){
			$partM	=   $this -> MODEL('part');
			
			$id 	=   intval($_GET['id']);
			
			$arr    =   $partM -> delPartApply($id,array('uid'=>$this->uid,'usertype'=>$this->usertype));
			$this ->  layer_msg($arr['msg'], $arr['errcode'], $arr['layertype'],$_SERVER['HTTP_REFERER']);
		}
	}
}
?>