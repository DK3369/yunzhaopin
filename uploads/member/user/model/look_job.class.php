<?php

class look_job_controller extends user{
    //职位浏览记录列表
	function index_action(){
		$JobM				=   $this -> MODEL('job');
		
		$where['uid']		=  $this -> uid;
		$where['status']	=  0;
		 //分页链接
		$urlarr['c']		=	$_GET['c'];
        $urlarr['page']		=	'{{page}}';
        
        $pageurl			=	Url('member',$urlarr);

        $pageM				=	$this  -> MODEL('page');
        $pages				=	$pageM -> pageList('look_job',$where,$pageurl,$_GET['page']);
		
        if($pages['total'] > 0){
			
            if($_GET['order']){
                
                $where['orderby']		=	$_GET['t'].','.$_GET['order'];
                $urlarr['order']		=	$_GET['order'];
                $urlarr['t']			=	$_GET['t'];
            }else{
                
                $where['orderby']		=	array('id,desc');
            }
            $where['limit']				=	$pages['limit'];
            
            $rows    	=   $JobM -> getLookJobList($where);
        }
		$this -> yunset("js_def",2);
		$this -> yunset("look",$rows);
		$this -> public_action();
		$this -> user_tpl('look_job');
	}
	//删除职位浏览记录
	function del_action(){
		if($_GET['del']||$_GET['id']){
			$JobM   =   $this -> MODEL('job');
			if(is_array($_GET['del'])){
				$id =   $_GET['del'];
			}else{
				$id =   intval($_GET['id']);
			}
			$arr    =   $JobM -> delLookJob($id,array('uid'=>$this->uid,'usertype'=>$this->usertype));
			$this ->  layer_msg($arr['msg'], $arr['errcode'], $arr['layertype'],$_SERVER['HTTP_REFERER']);
		}
	}
}
?>