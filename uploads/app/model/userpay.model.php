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
class userpay_model extends model{


    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {
        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }

	//简历置顶
	function buyZdresume($data){
	    
	    $did   =   $data['did'] ? intval($data['did']) : $this->config['did'];

		if($data['resumeid'] && $data['days']){
			$resumeid 		= 		$data['resumeid'];
 			$days			=		intval($data['days']);
 			if($days>0 && $resumeid){
 				//判断简历ID真实性
 			    $resume		=		$this->select_once("resume_expect",array('uid'=>$data['uid'] , 'id'=>$resumeid));
				if(empty($resume)){
					$return['error'] 		= 	'请选择正确的简历置顶！';
				}else {
					//计算需付费金额
					$price 	= 		$days * $this->config['integral_resume_top']; // 购买置顶简历所需金额
					$price 	= 		sprintf("%.2f", $price);
 					if ($price ==0){
                        //生成相关订单
                        $dingdan=time().rand(10000,99999);
                        $orderData['type']			=		'14';//14简历置顶
                        $orderData['order_id']		=		$dingdan;
                        $orderData['order_price']	=		$price;
                        $orderData['order_time']	=		time();
                        $orderData['order_state']	=		"2";
                        $orderData['order_type']	=		$data['paytype'];
                        $orderData['order_remark']	=		'简历置顶';
                        $orderData['uid']			=		$data['uid'];
                        $orderData['did']			=		$data['did'];
                        $orderData['usertype']		=		$data['usertype'];
                        $orderData['order_info']	=		serialize(array('resumeid'=>$data['resumeid'],'days'=>$data['days'],'price'=>$price));
                        $id							=		$this->insert_into("company_order",$orderData);
                        if($id){
                            $this->addMemberLog($data['uid'], $data['usertype'], "财务订单：简历置顶生成订单，订单ID：".$orderData['order_id'], 88, 1);

                            $zdresume 	=	 $this -> select_once('resume_expect', array('id' => $data['resumeid']), '`id`,`topdate`');

                            if(!empty($zdresume)){

                                $topdate   =   $zdresume['topdate'] > time() ? array('+' , $data['days'] * 86400) : time() +$data['days']*86400;

                                $status	   =   $this -> update_once('resume_expect',array('topdate'=>$topdate,'top'=>'1'),array('uid'=>$data['uid'],'id'=>$data['resumeid']));
                                $return['error'] = '置顶成功！';
                                $orderData['id']		=		$id;
                                $return['order']		=		$orderData;
                                if($data['type'] == 'wap') {

                                }
                                $return['pic0'] = 0;
                            }
                        }
					} else {
						//生成相关订单
						$dingdan=time().rand(10000,99999);
						$orderData['type']			=		'14';//14简历置顶
						$orderData['order_id']		=		$dingdan;
						$orderData['order_price']	=		$price;
						$orderData['order_time']	=		time();
						$orderData['order_state']	=		"1";
						$orderData['order_type']	=		$data['paytype'];
						$orderData['order_remark']	=		'简历置顶';
						$orderData['uid']			=		$data['uid'];
						$orderData['did']			=		$data['did'];
						$orderData['usertype']		=		$data['usertype'];
						$orderData['order_info']	=		serialize(array('resumeid'=>$data['resumeid'],'days'=>$data['days'],'price'=>$price));
						$id							=		$this->insert_into("company_order",$orderData);
						 
 						if($id){
 							$orderData['id']		=		$id;
 							$return['order']		=		$orderData;
                            $this->addMemberLog($data['uid'], $data['usertype'],  "财务订单：委托简历生成订单，订单ID：" . $orderData['order_id'], 88, 1);
                            if($data['type'] == 'wap'){

                                if($data['paytype'] == 'alipay'){

                                    $url  =	 $this->config['sy_weburl'].'/api/wapalipay/alipayto.php?dingdan='.$orderData['order_id'].'&dingdanname='.$orderData['order_id'].'&alimoney='.$orderData['order_price'];

                                }
                                $return['url']  =  $url;

                            }
                            $return['pic0'] = 1;
 						}else{
							$return['error'] 		= 		'订单生成失败！';
						}
					}
				}
 			}else{
				$return['error'] = '请正确选择简历置顶以及置顶的天数！';
			}
		} else {
			$return['error'] = '参数填写错误，请重新设置！';
		}
		return $return;
	}
}
?>