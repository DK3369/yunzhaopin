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
class finance_company_pay_controller extends adminCommon{
	function index_action(){

		$OrderM							=	$this -> MODEL('companyorder');
		$where							=   array();
	    $keywordStr						=   trim($_POST['keyword']);
		if($_POST['comid']){
			$where['com_id']			=	$_POST['comid'];
			$where['usertype']			=	'2';
		}
		if(!empty($keywordStr)){
			if ($_POST['type']=='1') {
				$where['order_id']		=	array('like', $keywordStr);
			}elseif ($_POST['type']=='3') {
				$where['pay_remark']	=	array('like', $keywordStr);
			}elseif($_POST['type']=='2'){
				$UserinfoM				=	$this -> MODEL('userinfo');
				$member					=	$UserinfoM -> getList(array('username'=>array('like',$keywordStr)),array('field'=>'uid'));
				if (is_array($member)){
					foreach ($member as $val){
						$muids[]	=	$val['uid'];
					}
					$where['com_id']	=	array('in', pylode(",",$muids));
				}
			}
		}

		if($_POST['pay_state']!=""){
			$where['pay_state']			=	$_POST['pay_state'];
        }
		if($_POST['end']){
			if($_POST['end'] == 1){
				$where['pay_time']	=	array('>=',strtotime(date("Y-m-d 00:00:00")));
			}else{
				$where['pay_time']	=	array('>=',strtotime('-'.intval($_POST['end']).' day'));
			}
		}
        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM	=	$this  -> MODEL('page');
        $pages	=	$pageM -> adminPageList('company_pay',$where,$page,array('limit' => $pageSize));
		if(!$pages['total']) {
            $this->render_json(0,'暂无数据',['data'=>array(),'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }
        //limit order 只有在列表查询时才需要
        if($_POST['order']){
            $where['orderby']		=	$_POST['t'].','.$_POST['order'];

        }else{
            $where['orderby']		=	array('id,desc');
        }
        $where['limit']				=	$pages['limit'];
        $rows    					=   $OrderM -> getPayList($where,array('utype'=>'admin'));
        $integral_pricename = $this->config['integral_pricename'];
        foreach ($rows as &$v){
            if ($v['type'] == 1){
                $v['price_str'] = $v['order_price'] .$integral_pricename;
            }else{
                $v['price_str'] = $v['order_price'] ."元";
            }
            $v['pay_time'] = $v['pay_time']?date('Y-m-d H:i:s',$v['pay_time']):'';
        }
        $this->render_json(0,'',['data'=>$rows,'total'=>(int)$pages['total'],'pageSizes'=>$pages['page_sizes']]);

    }

	function del_action(){
		$OrderM		=	$this -> MODEL('companyorder');
		$delID		=	is_array($_POST['del']) ? $_POST['del'] : $_POST['id'];
        if (!$delID){
            $this->render_json(1,'参数错误,请重试');
        }
		$return		=	$OrderM -> delPay($delID);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}
}
?>