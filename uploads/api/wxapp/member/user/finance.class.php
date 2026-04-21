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
class finance_controller extends user_controller{
    
	function getserver_action()
	{
	    $eid			=	$_POST['id'];
	    $resumeM  		= 	$this -> MODEL('resume');

	    $info			=	$resumeM -> getExpect(array('uid'=>$this->member['uid'],'id'=>$eid));
        if($info['topdate']>1){
			$info['topdate']	=	date('Y-m-d',$info['topdate']);
		}else{
			$info['topdate']	=	'未设置';
		}
		$data['info']	=	$info;
	    $data['webtel']	=	$this->config['sy_freewebtel'];
	    $data['price']	=	$this->config['integral_resume_top'];
	    
	    $data['fktype']  =  $this->fktype();
		$this->render_json('1','',$data);
	}
	//设置简历置顶简历
	function setServer_action()
	{
	    $result   =  array();
	    
	    if ($_POST['fktype'] == 'fkal'){
	        $paytype  =  'alipay';
	    } 
        if($_POST['type']){
            $data['type']		   =  'wap';
        }
	    $data['uid']		=	$this->member['uid'];
	    $data['usertype']	=	$this->member['usertype'];
	    $data['username']	=	$this->member['username'];
	    $data['did']		=	$this->member['did'];
	    $data['paytype']	=	$paytype;
	    $data['resumeid']	=	$_POST['id'];
	    $data['days']		=	$_POST['rdays'];
	    $userpayM			=	$this->MODEL('userpay');
	    if($_POST['server'] == 'zdresume') {
            $return = $userpayM->buyZdresume($data);
        } 
	    if($return['order']['order_id'] && $return['order']['id']){
	        
	        $result['id']  =  $return['order']['id'];
            if($_POST['type']) {
                $result['url'] = $return['url'];
            }
            if($return['pic0']){
                $result['goto'] = 1;
            }else{
                $result['goto'] = 0;
            }
	        $errmsg  =  'ok';
	    }else{
	        $errmsg	 =	$return['error'];
	    }
	    $this->render_json(0,'ok',$result);
	}
	function getStatis_action()
	{
		$StatisM	=	$this		->	MODEL('statis');
		$orderM		=	$this		->	MODEL('companyorder');
        $integralM		                =	$this->MODEL('integral');
		$statis		=	$StatisM	->	getInfo($this->member['uid'],array('usertype'=>'1'));//查询会员信息
        $orders		=	$orderM->getPayList(array('com_id'=>$this->member['uid'], 'usertype' =>1, 'type'=>'1'),array('field'=>'`order_price`'));
        $allprice   =   0;
        foreach($orders as $key=>$val){
            $allprice	+=	$val['order_price'];
        }
        if($allprice<0){
            $statis['allprice']		=	number_format(str_replace('-','', $allprice));
        }else{
            $statis['allprice']		=	'0';
        }
		$data['integral_proportion']	=	$this->config['integral_proportion'];
		$data['integral_pricename']		=	$this->config['integral_pricename'];
        $data['integral_avatar']        =   $this->config['integral_avatar'];
        $data['integral_mobliecert']    =   $this->config['integral_mobliecert'];
        $data['integral_signin']        =   $this->config['integral_signin'];
        $data['integral_invite_reg']    =   $this->config['integral_invite_reg'];
        $data['integral_userinfo']      =   $this->config['integral_userinfo'];
		$statusList		                =	$integralM	->	integralMission(array('type'=>'member','uid'=>$this->member['uid'],'usertype'=>1));;

        $data['task']                   =   $statusList;

        // app用分享数据
        if (isset($_POST['provider']) && $_POST['provider'] == 'app'){

            $data['shareData']  =   array(
                'url'       =>  Url('wap').'index.php?c=register&uid='.$this->member['uid'],
                'title'     =>  '邀请注册',
                'summary'   =>  '我在'.$this->config['sy_webname'].'上找工作；真的很不错，忍不住推荐给你',
                'imageUrl'  =>  checkpic($this->config['sy_wx_sharelogo'])
            );
        }
        
		$data['statis']		=	$statis;
		$this->render_json(1,'',$data);
	}
    //消费明细
	function consume_action()
	{
		
		$orderM				=	$this->MODEL('companyorder');
		$where['com_id']	=	$this->member['uid'];
		$where['usertype']	=	$this->member['usertype'];
		
		$total = $orderM->getCompanyPayNum($where);
		$where['orderby']	=	'pay_time,desc';
	    $page				=	$_POST['page'];
		$limit				=	$_POST['limit'] ? $_POST['limit'] : 10;
		if($page){//分页
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	$limit;
		}
	    $List				=	$orderM->getPayList($where,array('utype'=>'paylog'));
		$config['unit']		=	$this->config['integral_priceunit'];
		$config['pname']	=	$this->config['integral_pricename'];

		$List				=	count($List) ? $List : array();
		foreach($List as $k=>$v){
			$List[$k]['dingdan_id']		=	$v['order_id'];
			$List[$k]['dingdan_price']	=	$v['order_price'];
			$List[$k]['fk_state_n']		=	strip_tags($v['pay_state_n']);
			$List[$k]['fk_time']		=	$v['pay_time'];
			$List[$k]['fk_remark']		=	$v['pay_remark'];
			$List[$k]['fk_state']       =   $v['pay_state'];
		}
		$data				=	array(
			'list'		=>	$List,
			'config'	=>	$config
		);

		$this->render_json(0,'',$data,$total);
	}
    //订单列表
	function fklog_action(){
		
		$orderM					=	$this->MODEL('companyorder');
		$where['uid']			=	$this->member['uid'];
		$where['usertype']		=	$this->member['usertype'];
		$where['order_price']	=	array('>', 0);
		$total = $orderM->getCompanyOrderNum($where);
		$where['orderby']		=	array('order_time,desc','order_state,asc');
	    $page					=	$_POST['page'];
		$limit					=	$_POST['limit'] ? $_POST['limit'] : 10;
		if($page){//分页
			$pagenav			=	($page-1)*$limit;
			$where['limit']		=	array($pagenav,$limit);
		}else{
			$where['limit']		=	$limit;
		}
	    $List					=	$orderM->getList($where,array('utype'=>'member','uid'=>$this->member['uid']));
	    $List					=	count($List) ? $List : array();
	    foreach($List as $lk=>$lv){
	    	$List[$lk]['dingdan_state_n']	=	strip_tags($lv['order_state_n']);
	    	$List[$lk]['dingdan_id']		=	$lv['order_id'];
	    	$List[$lk]['dingdan_type_n']	=   $lv['order_type_n'];
	    	$List[$lk]['dingdan_price']		=	$lv['order_price'];
	    	$List[$lk]['dingdan_time_n']	=	$lv['order_time_n'];
	    	$List[$lk]['dingdan_remark']	=	$lv['order_remark'];
	    	$List[$lk]['dingdan_state']		=	$lv['order_state'];
	    	$List[$lk]['dingdan_type']		=	$lv['order_type'];
	    	if ($lv['type']=='1'){
		    	$List[$lk]['type_n']		=	'购买会员';
		    }elseif($lv['type']=='2'){
		    	$List[$lk]['type_n']		=	$this->config['integral_pricename'].'充值';
		    }elseif($lv['type']=='3'){
		    	$List[$lk]['type_n']		=	'银行转账';
		    }elseif($lv['type']=='4'){
		    	$List[$lk]['type_n']		=	'金额充值';
		    }elseif($lv['type']=='14'){
		    	$List[$lk]['type_n']		=	'简历置顶';
		    }
	    }
		$data['list']		=	$List ;
		$data['iosfk']		=	$this->config['sy_iospay'] ;
		$this->render_json(0,'',$data,$total);
	
	}
	//删除订单
	function delfklog_action(){
		$orderM		=	$this->MODEL('companyorder');
		$id			=	(int)$_POST['id'];
		$return		=	$orderM->cancelOrder(array('id'=>$id, 'uid'=>$this->member['uid']));
		
		$this->render_json($return['errcode'],$return['msg']);
	}
	//订单付款页面
	function fk_action()
    {
	    
    	$error	=	0;
    	$msg	=	'ok';
		
    	$fktype  =  $this->fktype();
		
    	if(!empty($fktype)){
		    
		    $id  =  (int)$_POST['id'];
		    
			if($id){//订单
			    
			    $orderM		=	$this->MODEL('companyorder');
				$order		=	$orderM->getInfo(array('id'=>$id));

				if(empty($order)){
					$error	=	2;
					$msg	=	'订单不存在'; 
				}elseif($order['order_state']!='1'){
				    $error	=	3;
				    $msg	=	'请检查订单状态,本订单无需付款';
				}else{
					$ordertype	=	array(
						'2'	=>	$this->config['integral_pricename'].'充值',
						'3'	=>	'银行转帐',
						'4'	=>	'金额充值',
						'14'=>	'简历置顶'
					);
					$order['type_n']	=	$ordertype[$order['type']];
					$error	=	1;
					$data['dingdan']	=	$order;
				}
			}
 			
			$data['fktype']  =  $fktype;
		}else{
			$error	=  2;
			$msg	=  '暂未开通支付'; 
			$data   =  array();
		}

	    $this->render_json($error,$msg,$data);
	}
	//生成订单
	function dingdan_action()
	{
	    $data['price_int']	   =  intval($_POST['price_int']);
	    $data['integralid']	   =  intval($_POST['integralid']);
	    $data['uid']		   =  $this->member['uid'];
	    $data['did']		   =  $this->member['did'];
	    $data['usertype']	   =  $this->member['usertype'];
	    if($_POST['type']){
            $data['type']		   =  'wap';
        }

	    if ($_POST['fktype'] == 'fkal'){
	        $data['paytype']  =  'alipay';
	    }
	    
	    $orderM   =  $this->MODEL('companyorder');
	    $return   =  $orderM->addComOrder($data);
	    
	    $result   =  array();
	    
	    if($return['errcode'] == 9 && !empty($return['url'])){
	        $msg  =  'ok';
	        $result['id']  =  $return['id'];
	        $result['url'] = $return['url'];
	    }else{
	        $msg  =  $return['msg'];
	    }
	    $this->render_json(0,$msg,$result);
  	}
  	/**
  	 * 充值页面
  	 */
	function fkclass_action()
	{
		
		$cacheM  =  $this->MODEL('cache');
	    $cList   =  $cacheM->GetCache('integralclass');
	    
	    $class_index		       =  $cList['integralclass_index']    ? $cList['integralclass_index']    : array();
	    $return['class_name']      =  $cList['integralclass_name']     ? $cList['integralclass_name']     : array();
	    $return['class_discount']  =  $cList['integralclass_discount'] ? $cList['integralclass_discount'] : array();
	    $class_price  =  array();
	    
	    if(!empty($class_index)){
	    	$fkey = 0;
	        foreach ($class_index as $k => $v){
	        	$classindex[$k]['val'] = (int)$v;
	            $discount  =  100;
	            if($return['class_discount'][$v]>0){
	                $discount      =  $return['class_discount'][$v];
	            }
	            $class_price[$v]   =  round($return['class_name'][$v]/$this->config['integral_proportion']*$discount/100,2);

	            $num 			   =  (int)$return['class_name'][$v];
	            if($num >= $this->config['integral_min_recharge']){
	            	$classindex[$k]['canchoose'] = 1;
	            	
	            	if($fkey == 0){
	            		$fkey = $k+1;

	            	}
	            }else{
	            	$classindex[$k]['canchoose'] = 0;
	            }
	        }
	        
	        if($fkey !=0){
	        	$return['first']       =  $class_index[$fkey-1];
	        	$return['firstprice']  =  $class_price[$class_index[$fkey-1]];
	        	$return['firstjf']     =  $return['class_name'][$class_index[$fkey-1]];
	        }
	       
	    }
	    $return['class_index']	=  !empty($classindex) ? $classindex : array();
	    $return['class_price']  =  $class_price;

	    $orderM		=	$this	->	MODEL('companyorder');
	    $nopayorder	=	$orderM	->	getCompanyOrderNum(array('uid'=>$this->member['uid'],'usertype' => $this->member['usertype'],'order_state'=>'1'));

	    $config		=	array(
	        'name'   		=>	$this->config['integral_pricename'],
	        'unit'   		=>	$this->config['integral_priceunit'],
	        'proportion'  	=>	$this->config['integral_proportion'],
	        'min_recharge'	=>	$this->config['integral_min_recharge']
	    );
	    $return['nopayorder']	=  $nopayorder;
	    $return['fktype']  		=  $this->fktype();
	    $return['config']  		=  $config;
	    $this->render_json(0, 'ok', $return);
	}
	 
    //任务中心
    function getIntegralTask_action()
    {

        $integralM		                =	$this->MODEL('integral');
        $resumeM	=	$this->MODEL('resume');
        $statusList		                =	$integralM	->	integralMission(array('type'=>'member','uid'=>$this->member['uid'],'usertype'=>$this->member['usertype']));
        $data['task']                   =   $statusList;
        $expectnum	=	$resumeM->getExpectNum(array('uid'=>$this->member['uid']));
        $data['integral_pricename']		=	$this->config['integral_pricename'];

        $data['integral_signin']        =   $this->config['integral_signin'];
        $data['integral_invite_reg']    =   $this->config['integral_invite_reg'];
        $data['integral_avatar']        =   $this->config['integral_avatar'];
        $data['integral_mobliecert']    =   $this->config['integral_mobliecert'];
        $data['integral_userinfo']      =   $this->config['integral_userinfo'];
        $data['integral_login']         =   $this->config['integral_login'];
        $data['integral_emailcert']     =   $this->config['integral_emailcert'];
        $data['integral_banner']        =   $this->config['integral_banner'];
        $data['integral_identity']        =   $this->config['integral_identity'];
        $data['integral_add_resume']        =   $this->config['integral_add_resume'];
        $data['integral_question']        =   $this->config['integral_question'];
        $data['integral_answer']        =   $this->config['integral_answer'];
        $data['integral_answerpl']        =   $this->config['integral_answerpl'];
		$data['integral_bind_wx']      =   $this->config['integral_bind_wx'];
        $data['expectnum'] = $expectnum;
        // app用分享数据
        if (isset($_POST['provider']) && $_POST['provider'] == 'app'){

            $data['shareData']  =   array(
                'url'       =>  Url('wap').'index.php?c=register&uid='.$this->member['uid'],
                'title'     =>  '邀请注册',
                'summary'   =>  '我在'.$this->config['sy_webname'].'上找工作；真的很不错，忍不住推荐给你',
                'imageUrl'  =>  checkpic($this->config['sy_wx_sharelogo'])
            );
        }

        $this->render_json(1,'',$data);
    }
}