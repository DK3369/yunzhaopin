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
class zph_controller extends com_controller{
	
	function zphlist_action(){
		$ZphM				=	$this -> MODEL('zph');
		$where['uid']		=	$this -> member['uid'];
		$page						=		$_POST['page'];
		if ($_POST['limit']){
			$limit					=		$_POST['limit'];
			if($page){
				$pagenav			=		($page-1)*$limit;
				$where['limit']		=		array($pagenav,$limit);
			}else{
				$where['limit']		=		$limit;
			}         
		}
		$where['orderby']	=	array('ctime,desc');
		$rows				=	$ZphM -> getZphCompanyList($where);
		
		$data['total']      =   $ZphM -> getZphComNum(array('uid'=>$this->member['uid']));
		
		if(is_array($rows) && !empty($rows)){
			$data['list']	=	$rows;
			$this->render_json(0,'ok',$data);
		}else{
			$error	=	2;
			$this->render_json($error);
		}
	}
	
	function delzph_action(){
		$ZphM	=	$this -> MODEL('zph');
		$id   	= 	intval($_POST['ids']);
		$return	=	$ZphM -> delZphCom($id,array('uid'=>$this->member['uid']));
		if($return['errcode']==9){
			$error	=	1;
		}else{
			$error	=	2;
		}
		$this->render_json($error,$return['msg']);
	}
   
}