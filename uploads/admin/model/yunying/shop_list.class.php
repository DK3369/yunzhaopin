<?php

class shop_list_controller extends adminCommon{
	
	function index_action(){
		$redeemM	=	$this->MODEL('redeem');
		if($_POST['status']){
            if($_POST['status']	==	'-1'){
            	$where['status']	=	'0';
            }else{
				$where['status']	=	$_POST['status'];
            }
		}
		
		if($_POST['change']){
			if($_POST['change']	==	'1'){
				$where['ctime']		=	array('>=',strtotime(date("Y-m-d 00:00:00")));
			}else{
				$where['ctime']		=	array('>=',strtotime('-'.$_POST['change'].'day'));
			}
		}
		
		if(trim($_POST['keyword'])){
			if($_POST['type']=='1'){
				$where['name']		=	array('like',trim($_POST['keyword']));
			}elseif($_POST['type']=='2'){
				$where['username']	=	array('like',trim($_POST['keyword']));
			}
		}
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        } else {
            $where['orderby'] = 'status,asc';
        }
        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('change', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = [];
        if ($pages['total'] > 0) {
            $where['limit'] = $pages['limit'];
            $list = $redeemM->getChangeList($where);
            $list = $list['list'];
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
	}

    function index_base_data_action()
    {
        $search_list[]	=	array("param"=>"change","name"=>'wap_user_00002',"value"=>array("1"=>'common_01940',"3"=>'admin_user_00179',"7"=>'admin_user_00178',"15"=>'admin_user_00180',"30"=>'admin_user_00175'));
        $search_list[]	=	array("param"=>"status","name"=>'wap_com_00406',"value"=>array("-1"=>'wap_user_00166',"1"=>'wap_user_00165',"2"=>'wap_user_00167'));
        $this->render_json(0, 'ok', compact('search_list'));
    }

	function status_action(){ 
	
		$redeemM	=	$this->MODEL('redeem');
		
		if(intval($_POST['id'])){
			
			$change	=	$redeemM->getChangeInfo(array('id'=>intval($_POST['id'])),array('field'=>'`gid`,`num`,`status`,`uid`,`integral`,`name`,`usertype`'));
			$reward	=	$redeemM->getInfo(array('id'=>$change['gid']),array('field'=>'`id`,`stock`,`name`'));
			
			if($_POST['status']>0&&$change['status']=='0'){
				if($_POST['status']=='1'){
					if(trim($_POST['express'])&&trim($_POST['expnum'])){
						$value['express']	=	trim($_POST['express']);
						$value['expnum']	=	trim($_POST['expnum']);
					}
				}else{
					$stock			=	$reward['stock']+$change['num'];
					if($stock<0){
						$stock='0';
					}
					$orderM		=	$this->MODEL("companyorder");
					$ordernum	=	$orderM->getCompanyPayNum(array('com_id'=>$change['uid'],'pay_remark'=>'admin_01428'));
					if(!$ordernum){
						$IntegralM		=	$this->MODEL("integral");
						$IntegralM->company_invtal($change['uid'],$change['usertype'],$change['integral'],true,'admin_01428',true,2,'integral',24);//积分操作记录
					}
					
					$upReData['num']	=	array('-',$change['num']);
					$upReData['stock']	=	$stock;
					$upReWhere['id']	=	$change['gid'];
					$redeemM->upInfo($upReData,$upReWhere);
					
					$value['express']	=	'';
					$value['expnum']	=	'';
				}
			}
			
			/* 消息前缀 */		
			$tagName  				=	'admin_yunying_00012';
			
			/* 处理审核信息 */
			if ($_POST['status'] == 2){
				
				$statusInfo  =  '您兑换的商品:<a href="rewardtpl,'.$reward['id'].'">'.$reward['name'].'</a>,审核未通过';
				
				if($_POST['statusbody']){
					
					$statusInfo  .=  ' , 原因：'.$_POST['statusbody'];
					
				}
				
				$msg[$change['uid']]  =  $statusInfo;
				
			}elseif($_POST['status'] == 1){
				
				$msg[$change['uid']]  =  '您兑换的商品:<a href="rewardtpl,'.$reward['id'].'">'.$reward['name'].'</a>,已审核通过';
				
			}
			
			if(!empty($msg)){
				$uids[]		=	$change['uid'];
				
				//发送系统通知
				
				$sysmsgM	=	$this->MODEL('sysmsg');
				
				$sysmsgM -> addInfo(array('uid'=>$uids,'usertype' => $change['usertype'],'content'=>$msg));

			}
			
			$value['status']		=	$_POST['status'];
			$value['linktel']		=	$_POST['linktel'];
			$value['linkman']		=	$_POST['linkman'];
			$value['statusbody']	=	$_POST['statusbody'];
			$where['id']			=	intval($_POST['id']);
			$id						=	$redeemM->upChangeInfo($where,$value);	

            $this->render_json($id? 0 : 1, $id? yun_at('admin_01429'): yun_at('api_wxapp_00016'));
		}else{
            $this->render_json(1, yun_at('member_com_00320'));
		}
	}

	function del_action(){
		$IntegralM	=	$this->MODEL('integral');
		$redeemM	=	$this->MODEL('redeem');
		if($_POST['del']){
			if(is_array($_POST['del'])){
				$where['id']	=	$_POST['del'];

				$delid			=	pylode(',',$_POST['del']);
                $rowsWhere	=	array('id'=>array('in',$delid));
			}else{
				$where['id']	=	(int)$_POST['del'];

				$delid			=	(int)$_POST['del'];
                $rowsWhere['id']	=	$where['id'];
			}


			$rowsData['field']	=	'`uid`,`gid`,`num`,`integral`,`usertype`,`status`';
			$rowss				=	$redeemM->getChangeList($rowsWhere,$rowsData['field']);
			$rowss				=	$rowss['list'];
			if($rowss&&is_array($rowss)){
				foreach($rowss as $val){
					if($val['status']==0){
						$IntegralM->company_invtal($val['uid'],$val['usertype'],$val['integral'],true,'wap_user_00003',true,2,'integral',24);
						$upReData['stock']	=	array('+',$val['num']);
						$upReData['num']	=	array('-',$val['num']);
						$upRewhere['id']	=	$val['gid'];
						$redeemM->upInfo($upReData,$upRewhere);
					}
				}
			}
			$del	=	$redeemM->delChange($rowsWhere);
			$this->render_json($del?0:1, $del?yun_at('admin_01430'):yun_at('wap_user_00146'));
		}else{
            $this->render_json(1, yun_at('common_01162'));
		}
	}
}
?>