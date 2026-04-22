<?php

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