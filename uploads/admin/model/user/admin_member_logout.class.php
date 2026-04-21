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
class admin_member_logout_controller extends adminCommon{	 
	function index_action()
	{
	    if ($_POST['status']){
	        $where['status']  =  intval($_POST['status']);
	    }
		if(trim($_POST['keyword'])){
			if($_POST['type']==1){
				$where['username']		=	array('like',trim($_POST['keyword']));
			}elseif($_POST['type']==2){
				$where['tel']		=	array('like',trim($_POST['keyword']));
			}elseif($_POST['type']==3){
				$where['uid']			=	array('=',trim($_POST['keyword']));
			}
		}
		$where['orderby']  =  array('id,desc');
        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM	=	$this  -> MODEL('page');
        $pages	=	$pageM -> adminPageList('member_logout',$where,$page,array('limit' => $pageSize));
        if (!$pages['total']) {
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0]);
        }
        $where['limit']	=  	$pages['limit'];

        $logoutM  =  $this->MODEL('logout');
        $List     =  $logoutM -> getList($where);

        if (!$List) {
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }
        $uIds = array();
        foreach ($List as $value) {
            $uIds[] = $value['uid'];
        }
        $userType = [
            1 => '个人',
            2 => '企业',
        ];
        // 查询会员对应角色
        $userInfoModel = $this->MODEL('userinfo');
        $whereData = ['uid'=> ['in',implode(',', $uIds)]];
        $userList = $userInfoModel->getList($whereData, ['field' => 'usertype,uid']);
        $userListNew = array();
        foreach ($userList as $value) {
            $userListNew[$value['uid']] = $value['usertype'];
        }
        foreach ($List as $key => $value) {
            $List[$key]['usertype_name'] = isset($userType[$userListNew[$value['uid']]])? $userType[$userListNew[$value['uid']]] : '未知';
            $List[$key]['ctime_ymd'] = $value['ctime']? date('Y-m-d H:i:s',$value['ctime']) : '-';
        }
        $data = array(
            'data'=>$List,'total'=>(int)$pages['total'],
            'pageSizes'=>$pages['page_sizes']
        );
        $this->render_json(0,'',$data);
	}
	/**
	 * 注销账号申请审核
	 */
	function status_action()
	{
	    $id  =  intval($_POST['id']);
	    
	    $logoutM  =  $this->MODEL('logout');
	    $return   =  $logoutM->status(array('id'=>$id));
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}
	/**
	 * 注销账号申请删除
	 */
	function del_action(){
	    
	    if($_POST['id']){
	        $delid  =  intval($_POST['id']);
	    }
	    if($_POST['del']){
	        $delid  =  $_POST['del'];
	    }
        if (!$delid){
            $this->render_json(1,'参数错误');
        }
	    $logoutM  =  $this->MODEL('logout');
	    $return	  =	 $logoutM->del($delid);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}

    /**
     *
     */
    public function  memNum_action()
    {
        $memberLogoutM = $this->MODEL('logout');
        $arr =  $memberLogoutM->getListNumV1();
        $this->render_json('0','',$arr);
    }
}
?>