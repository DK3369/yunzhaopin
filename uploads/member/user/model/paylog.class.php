<?php

class paylog_controller extends user
{
    //账单明细
	function index_action(){
		include(CONFIG_PATH."db.data.php");
		$this->yunset("arr_data",$arr_data);
		$this->public_action(); 
		
		$orderM				=	$this->MODEL('companyorder');
		$where['com_id']	=	$this->uid;
		$where['usertype']	=	$this->usertype;
		
		//分页链接
		$urlarr['page']	=	'{{page}}';
		$urlarr['c']	=	"paylog";
		$pageurl		=	Url('member',$urlarr);
		
		//提取分页
		$pageM			=	$this  -> MODEL('page');
		$pages			=	$pageM -> pageList('company_pay',$where,$pageurl,$_GET['page']);
		
		//分页数大于0的情况下 执行列表查询
		if($pages['total'] > 0){
			
			  $where['orderby']	=	array('id,desc','pay_time,desc');
		    $where['limit']		=	$pages['limit'];
			
		    $List				=	$orderM->getPayList($where,array('utype'=>'paylog'));
			
			$this->yunset("rows",$List);
		}

		$this->yunset("ordertype","ok");  
		$this->user_tpl('paylog');
	} 
	
}
?>