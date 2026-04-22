<?php

class payment_controller extends user{
	function index_action(){//付款
		$comorderM	=	$this->MODEL('companyorder');
		$ConfigM    =	$this->MODEL('config');
		$rows		=	$ConfigM->getBankList(array('id'=>array('>', 0)));
		$this->yunset("rows",$rows);
		
		if($this->usertype!='1' || $this->uid==''){
			
			$this->ACT_msg("index.php","非法操作！");
			
		}else{
			$order	=	$comorderM->getInfo(array('uid'=>$this->uid,'id'=>(int)$_GET['id']));
			
			if(is_array($order)){
				$orderbank				=	@explode("@%",$order['order_bank']);
				$order['bank_name']		=	$orderbank[0];
				$order['bank_number']	=	$orderbank[1];			
			}
			
			if(empty($order)){
				
				header("Location:index.php?c=paylist");
				
				exit();
				
			}else{
				
				$this->yunset("order",$order);
				$this->public_action();
				$this->yunset("comstyle","../app/template/member/com");
				$this->user_tpl('payment');
			}
		}
	}
	/**
	 * 微信扫码支付
	 */
	function wxurl_action(){
		$comorderM			=	$this->MODEL('companyorder');
		
		$data['orderId']	=	(int)$_POST['orderId'];
		
		$return				=	$comorderM->payComOrderByWXPC($data);
		
		echo $return['msg'];die;
	}
	/**
	 * 银行转账
	 */
	function paybank_action(){
		$comorderM	=	$this->MODEL('companyorder');
		
		$data['id']				=	$_POST['oid'];
		$data['uid']			=	$this->uid;
		$data['usertype']		=	$this->usertype;
		$data['did']			=	$this->userdid;
		$data['file']			=	$_FILES['file'];
		$data['bank_name']		=	$_POST['bank_name'];
		$data['bank_number']	=	$_POST['bank_number'];
		$data['bank_price']		=	$_POST['bank_price'];
		$data['bank_time']		=	$_POST['bank_time'];
		$data['order_remark']	=	$_POST['order_remark'];
		
		$return		=	$comorderM->payComOrderByBank($data);
		
		$this->ACT_layer_msg($return['msg'],$return['errcode'],$return['url']);
	
	}
	
}
?>