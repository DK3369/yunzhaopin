<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class finance_controller extends com_controller
{

	function consumelist_action()
	{

        $comorderM          =   $this->MODEL('companyorder');
        $page               =   $_POST['page'];
        $limit              =   $_POST['limit'];
        $limit              =   !$limit ? 10 : $limit;
        $where['com_id']    =   $this->member['uid'];
        $where['usertype']  =   2;
        $where['orderby']   =   'pay_time,desc';
        $total              =   $comorderM->getCompanyPayNum($where);
        if ($page) {

            $pagenav        =   ($page - 1) * $limit;
            $where['limit'] =   array($pagenav, $limit);
        } else {

            $where['limit'] =   array('', $limit);
        }

		$rows   =   $comorderM->getPayList($where);
		if(is_array($rows) && !empty($rows)){
		    foreach ($rows as $k=>$v){
		        $rows[$k]['fk_state'] = $v['pay_state'];
		    }
			$data['list']	=	count($rows)?$rows:array();
			$data['error']	=	1;
		}else{
			$data['error']	=	2;
		}
		$this->render_json($data['error'],'',$data['list'],$total);
	}
	
    function fk_action()
    {

    	//新订单页面
    	$error	=	0;
    	$msg	=	'';
    	$id		=	(int)$_POST['id'];
    	$fktype  =  $this->fktype();
    	
    	if(!empty($fktype)){
			if($id){//订单
			    $orderM		=	$this->MODEL('companyorder');
				$order		=	$orderM	->	getInfo(array('uid'=>$this->member['uid'],'id'=>$id));

				if(empty($order)){
					$error	=	2;
					$msg	=	'订单不存在'; 
				}elseif($order['order_state']!='1'){
					$error	=	3;
					$msg	=	'请检查订单状态,本订单无需付款';
				}else{
					$ordertype	=	array(
						'1'	=>	'购买会员',
						'2'	=>	$this->config['integral_pricename'].'充值',
						'3'	=>	'银行转帐',
						'5'	=>	'购买增值包',

						'8'	=>	'分享红包推广',

						'10'=>	'职位置顶',
						'11'=>	'职位紧急', 
	            		'12'=>	'职位推荐',
	            		'13'=>	'自动刷新', 
	            		'16'=>	'刷新职位',
	            		'17'=>	'刷新兼职', 

	            		'19'=>	'下载简历', 
	            		'20'=>	'上架职位',
	            		'21'=>	'上架兼职',

	            		'23'=>	'面试邀请', 
	            		'24'=> 	'兼职推荐',

	            		'28'=>	'购买招聘会报名次数'
					);
					$order['type_n']	=	$ordertype[$order['type']];
					$error	=	1;
					$data['dingdan']	=	$order;
				}
			}
 			
			$data['fktype']  =  $fktype;
			$data['webtel']	 =	$this->config['sy_comwebtel']?$this->config['sy_comwebtel']:$this->config['sy_freewebtel'];
		}else{
			$error	=	2;
			$msg	=	'暂未开通手机支付，请移步至电脑端充值！'; 
		}

	    $this->render_json($error,$msg,$data);
	}
	function dingdan_action()
	{
	    $data['price_int']	    =   intval($_POST['price_int']);
	    $data['integralid']	    =   intval($_POST['integralid']);
	    $data['uid']		    =   $this->member['uid'];
	    $data['did']		    =   $this->member['did'];
	    $data['usertype']	    =   $this->member['usertype'];
        if($_POST['type']){
            $data['type']	    =   'wap';
        }
	    if ($_POST['fktype'] == 'fkal'){
	        $data['paytype']    =   'alipay';
	    }elseif ($_POST['fktype'] == 'wxxcx'){
	        $data['paytype']    =   'wxxcx';
	    }
	    if ($this->comInfo['crm_uid']){
	        $data['crm_uid']    =   $this->comInfo['crm_uid'];
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
	
	function getSerives($id, $rtype, $uid = null)
	{
		$ratingM	=	$this	->	MODEL('rating');
		$data		=	array();
	    if ($rtype==1){
	        $rating			=	$ratingM->getInfo(array('id'=>$id ),array('field'=>"name,service_price,time_start,time_end,yh_price"));
	        $data['name']	=	$rating['name'];
	    	if ($rating['time_start']<time() && $rating['time_end']>time()){
	            $data['price']	=	$rating['yh_price'];
	        }else{
	            $data['price']	=	$rating['service_price'];
	        }
	    }else{
	        if ($uid){
				$statisM	=	$this->MODEL('statis');
				
	            $statis 	= 	$statisM->getInfo(array('uid'=>$uid),array('usertype'=>2,'field'=>'`rating`,`vip_etime`'));
	            if(!isVip($statis['vip_etime'])){
	                $data['errmsg']	=	'会员已过期，请先购买会员';
	                return $data;
	            }
	        }
            $details	=	$ratingM->getComSerDetailInfo(array('id'=>$id),array('field'=>'`type`,`service_price`'));
			$service	=	$ratingM->getComServiceInfo(array('id'=>$details['type']),array('field'=>'`name`'));
	        $ratinginfo	=	$ratingM->getInfo(array('id'=>$statis['rating']));
	        if($ratinginfo['service_discount']>0 && $ratinginfo['service_discount']<100){
	            $price	=	round($details['service_price']*($ratinginfo['service_discount']/100),2);
	        }else{
	            $price	=	$details['service_price'];
	        }
	        
	        $data['name']	=	$service['name'];
	        $data['price']	=	$price;
            $data['vip_etime'] = $statis['vip_etime'];
            $data['time']   =   strtotime('today');
	    }
	    return $data;
	}
	
    function setextension_action()
    {
      $jobM   =   $this->MODEL('job');
      $_POST['uid']	      =  $this->member['uid'];
      $_POST['usertype']  =  $this->member['usertype'];
      $_POST['username']  =  $this->member['username'];
      $_POST['did']	      =  $this->member['did'];
      
      $_POST['num']       =   intval($_POST['num']);
      $_POST['days']      =   intval($_POST['rdays']);
      if($_POST['num']>=$_POST['days']){
          
          if($_POST['serverid']==1){//置顶
              $_POST['type']    = 'top';
          }else if($_POST['serverid']==2){//推荐
              $_POST['type']    = 'rec';
          }else if($_POST['serverid']==3){//紧急招聘
              $_POST['type']    = 'urgent';
          }
          $return = $jobM->setJobPromote(intval($_POST['id']), $_POST);
          $this->render_json(0, $return['msg'], $return);
      }else{
          $this->render_json(8, '已超出套餐剩余数量');
      }
    }
	
	function fkclass_action()
	{

	    include(PLUS_PATH.'integralclass.cache.php');

        $cacheM =   $this->MODEL('cache');
        $cList  =   $cacheM->GetCache('integralclass');

        $class_index                =   $cList['integralclass_index'] ? $cList['integralclass_index'] : array();
        $return['class_name']       =   $cList['integralclass_name'] ? $cList['integralclass_name'] : array();
        $return['class_discount']   =   $cList['integralclass_discount'] ? $cList['integralclass_discount'] : array();

        $class_price    =   array();
        if (!empty($class_index)) {
            $fkey       =   0;
            foreach ($class_index as $k => $v) {
                $classindex[$k]['val']  =   (int)$v;
                $discount               =   100;
                if ($return['class_discount'][$v] > 0) {
                    $discount           =   $return['class_discount'][$v];
                }
                $class_price[$v]        =   round($return['class_name'][$v] / $this->config['integral_proportion'] * $discount / 100, 2);

                $num    =   (int)$return['class_name'][$v];
                if ($num >= $this->config['integral_min_recharge']) {

                    $classindex[$k]['canchoose']    =   1;

                    if ($fkey == 0) {
                        $fkey   =   $k + 1;
                    }
                } else {
                    $classindex[$k]['canchoose']    =   0;
                }
            }

            if ($fkey != 0) {

                $return['first']        =   $class_index[$fkey - 1];
                $return['firstprice']   =   $class_price[$class_index[$fkey - 1]];
                $return['firstjf']      =   $return['class_name'][$class_index[$fkey - 1]];
            }
        }

	    $return['class_index']	=   !empty($classindex) ? $classindex : array();
	    $return['class_price']  =   $class_price;

        $orderM		            =	$this->MODEL('companyorder');
        $nopayorder	            =	$orderM->getCompanyOrderNum(array('uid'=>$this->member['uid'],'usertype' => $this->member['usertype'],'order_state'=>'1'));

	    $return['nopayorder']	=   $nopayorder;
	    $return['fktype']  		=   $this->fktype();

        $config		            =   array(
            'name'   		=>	$this->config['integral_pricename'],
            'unit'   		=>	$this->config['integral_priceunit'],
            'proportion'  	=>	$this->config['integral_proportion'],
            'min_recharge'	=>	$this->config['integral_min_recharge']
        );
	    $return['config']  		=   $config;
	    $this->render_json(0, 'ok', $return);
	}
	/**
	 * 查询会员套餐名称
	 */
	function getRating_action(){
	    
	    
	    $statis  =  $this->company_statis($this->member['uid']);
	    
	    $return['rating']  =  $statis['rating_name'];
	    
	    $this->render_json(0, 'ok', $return);
	}
	
	function fklog_action(){
		$comorderM			=	$this	->	MODEL('companyorder');
        $where['uid']		=	$this	->	member['uid'];
        $where['usertype']	=	2;
        $total = $comorderM->getCompanyOrderNum($where);
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;
		


		$where['orderby']	=	array('order_time,desc','order_state,asc');
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
		$rows				=	$comorderM -> getList($where,array('utype'=>'com'));
		if(is_array($rows) && !empty($rows)){
			$data['list']	=	count($rows)?$rows:array();
			$data['error']	=	1;
		}else{
			$data['error']	=	2;
		}
		$data['iosfk']	=	$this->config['sy_iospay'] ;
		$this->render_json($data['error'], '', $data,$total);
	}
	
	function delfklog_action(){
		$comorderM	=	$this	->	MODEL('companyorder');
		$oid		=	$comorderM	->	getList(array('uid'=>$this->member['uid'],'id'=>(int)$_POST['ids'],'order_state'=>'1'));
		
		if(!empty($oid[0])){
			$comorderM	->	del($oid[0]['id'],array('uid'=>$this->member['uid']));
			$msg	=	'取消成功！';
			$error	=	1;
		}else{
			$msg	=	'订单不存在！';
			$error	=	2;
		}
		$this->render_json($error, $msg);
	}
	function buyvip_action(){
		$rdata['price']			=  $_POST['price'];
		$rdata['comvip']		=  $_POST['comvip'];
		$rdata['comservice']	=  $_POST['comservice'];
		$rdata['dkjf']			=  $_POST['dkjf'];
		$rdata['uid']			=  $this->member['uid'];
		$rdata['usertype']		=  $this->member['usertype'];
		$rdata['did']			=  $this->member['did'];
		$rdata['paytype']	    =  $_POST['paytype'];
		$rdata['type']		    =  'wap';
		if ($this->comInfo['crm_uid']){
		    $rdata['crm_uid']   =   $this->comInfo['crm_uid'];
        }
		$orderM	 =  $this	->	MODEL('companyorder');
		$return	 =  $orderM	->	addComOrder($rdata);
		
		$this->render_json(0, $return['msg']);
	}
	function dkzf_action(){
		if($_POST['comservice']){
			$data['tcid']	=	$_POST['comservice'];
		}elseif($_POST['comvip']){
			$data['id']		=	$_POST['comvip'];
		}
		$data['uid']		=	$this->member['uid'];
		$data['username']	=	$this->member['username'];
		$data['usertype']	=	$this->member['usertype'];
	
		$M			=	$this->MODEL('jfdk');
		$return		=	$M->dkBuy($data);
		$this->render_json(0, $return);
	}
	

	function searchcom_action(){
		$name	=	trim($_POST['name']);
		if($name){
			$companyM	=	$this -> MODEL('company');
			$where		=	array(
				'name'		=>	array('like',$name),
				'uid'		=>	array('<>',$this->member['uid']),
				'orderby'	=>	'lastupdate,desc',
				'limit'		=>	15
			);
			$company	=	$companyM -> getList($where,array('field'=>'`uid`,`name`','url'=>1));
			if($company&&is_array($company)){
				$this->render_json(0,'',$company['list']);
			}else{
				$this->render_json(1);
			}
		}
	}
	  
	
	function getStatis_action()
	{
		$StatisM	=	$this -> MODEL("statis");
		$orderM		=	$this -> MODEL('companyorder');
		$statis		=	$StatisM -> getInfo($this->member['uid'],array('usertype'=>'2'));//查询会员信息
		$where		=	array(
			'com_id'		=>	$this->member['uid'],
			'usertype'		=>	2,
			'pay_remark'	=>	array('like','转换'.$this->config['integral_pricename'])
		);
		$where['pay_time'][] = array('>',strtotime('today'));
		$where['pay_time'][] = array('<',strtotime('tomorrow'));
		if($statis['vip_etime']==0){
			$statis['vip_fwtime']		=	'永久';
		}else{
			$statis['vip_fwtime']		=	$statis['vip_stime_n'].'-'.$statis['vip_etime_n'];
		}
		$changeNum 	=	$orderM	 -> getCompanyPayNum($where);
		$data['integral_proportion']	=	$this->config['integral_proportion'];
		$data['integral_pricename']		=	$this->config['integral_pricename'];

		$data['iosfk']		            =	$this->config['sy_iospay'] ;
		$data['changeNum']	            =	$changeNum;
		
		$statis['packfk']               =   $statis['packpay'];
		$data['statis']		            =	$statis;

        $integralM		                =	$this->MODEL('integral');
        $statusList		                =	$integralM	->	integralMission(array('type'=>'com','uid'=>$this->member['uid'],'usertype'=>$this->member['usertype']));
		$data['task']                   =   $statusList;
        $data['integral_signin']        =   $this->config['integral_signin'];
        $data['integral_invite_reg']    =   $this->config['integral_invite_reg'];
        $data['integral_userinfo']      =   $this->config['integral_userinfo'];

        $data['integral_question']        =   $this->config['integral_question'];
        $data['integral_answer']        =   $this->config['integral_answer'];
        $data['integral_answerpl']        =   $this->config['integral_answerpl'];
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
	 
	
	function com_action(){
		$statisM	=	$this -> MODEL('statis');
		$suid     	=   $this->member['uid'];
		$statis   	=  	$statisM -> vipOver($suid, 2);
		if($statis['vip_etime']==0){

    		$statis['vip_fwtime']	=	'永久';

		}else{

    		$statis['vip_fwtime']	=	$statis['vip_stime_n'].'--'.$statis['vip_etime_n'];

		}
		$ratingM	=	$this -> MODEL('rating');
		$rating		=	$ratingM -> getInfo(array('id' => $statis['rating']));
		$data['statis']	=	$statis;
		$rating['resume']           =   !empty($rating['resume']) ? $rating['resume']: 0;
		$rating['breakjob_num']     =   !empty($rating['breakjob_num']) ? $rating['breakjob_num']: 0;
		$rating['zph_num']          =   !empty($rating['zph_num']) ? $rating['zph_num']: 0;
		$rating['top_num']          =   !empty($rating['top_num']) ? $rating['top_num']: 0;
		$rating['rec_num']          =   !empty($rating['rec_num']) ? $rating['rec_num']: 0;
		$rating['urgent_num']       =   !empty($rating['urgent_num']) ? $rating['urgent_num']: 0;



		$rating['job_num']          =   !empty($rating['job_num']) ? $rating['job_num']: 0;
		$rating['interview']        =   !empty($rating['interview']) ? $rating['interview']: 0;
		$data['rating']	            =	!empty($rating) ? $rating : array();
		$data['iosfk']  			=   $this->config['sy_iospay'];

        

        $jobNum     =   $this->obj->select_num('company_job', array('uid' => $suid, 'status' => 0));
        $partNum    =   $this->obj->select_num('partjob', array('uid' => $suid, 'status' => 0));
        $zzNum      =   $jobNum + $partNum;
        $JobNum     =   $statis['job_num'] - $zzNum;
        $data['JobNum']    =   $JobNum > 0 ? $JobNum : 0;
        
		 
		$this->render_json(0,'', $data);
	}
		 

    function getIntegralTask_action()
    {

        $integralM		                =	$this->MODEL('integral');
        $statusList		                =	$integralM	->	integralMission(array('type'=>'com','uid'=>$this->member['uid'],'usertype'=>$this->member['usertype']));
        $data['task']                   =   $statusList;

        $data['integral_pricename']		=	$this->config['integral_pricename'];

        $data['integral_signin']        =   $this->config['integral_signin'];
        $data['integral_invite_reg']    =   $this->config['integral_invite_reg'];
        $data['integral_avatar']        =   $this->config['integral_avatar'];
        $data['integral_mobliecert']    =   $this->config['integral_mobliecert'];
        $data['integral_map']           =   $this->config['integral_map'];
        $data['integral_comcert']       =   $this->config['integral_comcert'];
        $data['integral_userinfo']      =   $this->config['integral_userinfo'];
        $data['integral_login']         =   $this->config['integral_login'];
        $data['integral_emailcert']     =   $this->config['integral_emailcert'];
        $data['integral_banner']        =   $this->config['integral_banner'];

        $data['integral_question']      =   $this->config['integral_question'];
        $data['integral_answer']        =   $this->config['integral_answer'];
        $data['integral_answerpl']      =   $this->config['integral_answerpl'];
		$data['integral_bind_wx']      =   $this->config['integral_bind_wx'];

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