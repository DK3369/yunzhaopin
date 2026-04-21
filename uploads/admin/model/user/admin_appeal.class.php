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
class admin_appeal_controller extends adminCommon{
    function index_action(){
		
		$memberM						=		$this->MODEL('userinfo');
		$where['appeal']				=		array('<>','');
        $promiss = array('email'=>0,'moblie'=>0);
        if(in_array('141',$this->adminPower['power'])){
            $promiss['email'] = 1;
            $promiss['moblie'] = 1;
        }

        if(trim($_POST['keyword'])){
			
			$where['username']			=		array('like',trim($_POST['keyword']));
        }
        if(isset($_POST['appealstate']) && $_POST['appealstate']!==''){
            
            $where['appealstate']       =       $_POST['appealstate']=='1'  ?   1   :   2;
        }
        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM	=	$this  -> MODEL('page');
        $pages	=	$pageM -> adminPageList('member',$where,$page,array('limit' => $pageSize));

	    if(!$pages['total'] ){
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'promiss'=>$promiss,'pageSizes'=>$pages['page_sizes']]);
        }
        if($_POST['order']){
            $where['orderby']		=		$_POST['t'].','.$_POST['order'];
        }else{
            $where['orderby']		=		array('appealstate,asc','appealtime,desc');
        }
				
        $where['limit']				=		$pages['limit'];
        $List    					=   	$memberM -> getList($where,array('utype'=>'admin','field'=>'uid,username,appeal,appealtime,appealstate,moblie,email'));
        foreach ($List as &$value){
            $value['appealtime_ymd'] = $value['appealtime']? date('Y-m-d H:i:s',$value['appealtime']):'';
        }
        $data = array('data'=>$List,'total'=>(int)$pages['total'],'promiss'=>$promiss, 'pageSizes'=>$pages['page_sizes']);
        $this->render_json(0,'ok',$data);

    }
    function info_action(){
        if (!$_POST['id']){
            $this->render_json(1,'参数错误');
        }
		$memberM						=		$this->MODEL('userinfo');
		$info 							= 		$memberM->getInfo(array('uid'=>$_POST['id']));
        if ($info){
            $info['login_date_ymd'] = $info['login_date']?date('Y-m-d',$info['login_date']):'';
            $info['reg_date_ymd'] = $info['reg_date']?date('Y-m-d',$info['reg_date']):'';
        }
		$user 							= 		$memberM->getUserInfo(array('uid'=>$info['uid']),array('usertype'=>$info['usertype']));
		$this->render_json('0','',['user'=>$user,'info'=>$info]);
	}


    function success_action(){
        if (!$_POST['id']) {
            $this->render_json(1,'参数错误');
        }
        $memberM					=		$this->MODEL('userinfo');

        $result						=		$memberM->upInfo(array('uid'=>intval($_POST['id'])),array('appealstate'=>'2'));
        if ($result){
            $this->admin_json(0,'申诉确认成功！');
        }else{
            $this->render_json(1,'确认失败！');
        }
    }
    //删除
    function del_action(){
        $memberM						=		$this->MODEL('userinfo');
		
        if(is_array($_POST['del'])){
            $delid						=		$_POST['del'];
        }else{
			$delid[]					=		(int)$_POST['id'];
        }
		
		if(!$delid){
            $this->render_json('1','请选择要删除的内容！');
        }
		
		$result 						=		$memberM->upInfo(array('uid'=>array('in',pylode(',',$delid))),array('appeal'=>'','appealtime'=>'','appealstate'=>'1'));
        if ($result){
            $this->admin_json(0,'申诉(ID:'.pylode(',',$delid).')删除成功！');
        }else{
            $this->render_json(1,'删除失败！');
        }
    }
}