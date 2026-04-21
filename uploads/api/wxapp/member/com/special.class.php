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
class special_controller extends com_controller{
	
	function speciallist_action()
	{
		$SpecialM			=	$this -> MODEL('special');
		$where['uid']		=	$this -> member['uid'];
		$page				=	$_POST['page'];
		if($_POST['limit']){
			$limit			=	$_POST['limit'];
			if($page){
				$pagenav	=	($page-1)*$limit;
				$where['limit']	=	array($pagenav,$limit);
			}else{
				$where['limit']	=	$limit;
			}
		}
		$where['orderby']	=	array('time,desc');
		$rows				=	$SpecialM -> getSpecialComList($where, array('utype'=>'user'));
		if(is_array($rows['list']) && !empty($rows['list'])){
			$data['list']	=	$rows['list'];
			$this->render_json(0,'ok',$data);
		}else{
			$error	=	2;
			$this->render_json($error);
		}
	}
	function delspecial_action(){
		$SpecialM	=	$this -> MODEL('special');
		$id   		= 	intval($_POST['ids']);
		$uid   		= 	$this -> member['uid'];
		$return	=	$SpecialM -> delSpecialCom(array('id'=>$id),array('uid'=>$uid));
		if($return){
			$error	=	1;
		}else{
			$error	=	2;
		}
		$this->render_json($error,$return['msg']);
	}
   
}