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