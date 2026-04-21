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
class reward_controller extends com_controller{
    
	/**
     * 兑换记录列表
	*/
	function changeList_action(){
	    $redeemM = $this->MODEL('redeem');
        $statisM		=	$this->MODEL('statis');
        $where['uid']		=	$this->member['uid'];

        if($_POST['type']!='all'){
            $where['status'] = $_POST['type'];
        }
        $total = $redeemM->getChangeNum($where);
        $page				=	$_POST['page'];
        $limit				=	$_POST['limit'];
        $limit				=	!$limit?20:$limit;

        if($page){
            $pagenav		=	($page-1)*$limit;
            $where['limit']	=	array($pagenav,$limit);
        }else{
            $where['limit']	=	array('',$limit);
        }
        $where['orderby']	=	array('id,desc');
        $List	=	$redeemM -> getChangeList($where);
        $statis				=	$statisM->getInfo($this->member['uid'],array('usertype'=>2));

        $statis['integral']	=	number_format($statis['integral']);
        if($List['list'] && is_array($List['list'])){
            $data['error']=1;
        }else{
            $List['list'] = $List;
        }
        $List['statis'] = $statis;
        $this -> render_json($data['error'], 'ok', $List,$total);
    }
    function delChange_action(){
        $redeemM    			=       $this->MODEL('redeem');

        $uid 				=		$this->member['uid'];

        $id					=		intval($_POST['id']);

        $where['id'] = $id;
        $data['member'] = 'com';
        $data['usertype'] = 2;
        $data['uid'] = $uid;
        $data['type'] = 'one';
        $data['id'] = $id;

        $return  			=  		$redeemM -> delChange($id,$data);

        if($return['cod']==9){

            $error			=		1;

        }else{

            $error			=		2;
        }
        $this -> render_json($error,$return['msg']);
    }
     
}