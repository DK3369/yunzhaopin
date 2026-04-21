<?php
/* *
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
*/
class reward_list_controller extends user{
    //积分兑换记录列表
	function index_action(){
		$this->public_action();
		
		$redeemM		=	$this->MODEL('redeem');
		
		$statisM		=	$this->MODEL('statis');
		
		$where['uid']	    =	$this->uid;
		$where['usertype']	=	$this->usertype;
		//分页链接
		$urlarr['page']	=	'{{page}}';
		$urlarr['c']	=	'reward_list';
		$pageurl		=	Url('member',$urlarr);
		
		//提取分页
		$pageM			=	$this  -> MODEL('page');
		$pages			=	$pageM -> pageList('change',$where,$pageurl,$_GET['page']);
		
		//分页数大于0的情况下 执行列表查询
		if($pages['total'] > 0){
			
			$where['orderby']		=	'id,desc';
		    $where['limit']			=	$pages['limit'];
			
		    $List					=	$redeemM->getChangeList($where);
			
			$this->yunset("rows",$List['list']);
		}
		
		$statis				=	$statisM->getInfo($this->uid,array('usertype'=>1));
		$statis['integral']	=	number_format($statis['integral']);
		
		$num				=	$redeemM->getChangeNum(array('uid'=>$this->uid));
		
		$this	->	yunset("num",$num);
		$this	->	yunset("statis",$statis);
		$this	->	user_tpl('reward_list');
	}	
	//删除积分兑换记录
	function del_action(){
		$redeemM	=	$this->MODEL('redeem');
		
		$return		=	$redeemM->delChange('',array('uid'=>$this->uid, 'id'=>(int)$_GET['id'], 'usertype'=>$this->usertype,'member'=>1));
		
		$this->layer_msg($return['msg'],$return['cod'],0,$_SERVER['HTTP_REFERER']);
		
	}
	
	
}
?>