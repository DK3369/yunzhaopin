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
class info_systeminfo_controller extends adminCommon{

	function index_action()
	{
        $where = array();
		$sysmsgM=$this->MODEL('sysmsg');
		if(trim($_POST['keyword']))
		{
			if($_POST['type']=="1" || $_POST['type'] == '')
			{
				$where['username']	=	array('like',trim($_POST['keyword']));
			}else if($_POST['type']=="2"){
				$where['content']	=	array('like',trim($_POST['keyword']));
			}else if($_POST['type']=="3"){
                $where['fa_uid']	=	array('=',trim($_POST['keyword']));
            }
		}
		if($_POST['end']){
			if($_POST['end']=='1'){
				$where['ctime']		=	array('>=',strtotime(date("Y-m-d 00:00:00")));
			}else{
				$where['ctime']		=	array('>=',strtotime('-'.$_POST['end'].'day'));
			}
		}
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('sysmsg', $where, $page, array('limit' => $pageSize));

        $total  =   intval($pages['total']);

        if($_POST['order'])
        {
            $where['orderby']	=	$_POST['t'].','.$_POST['order'];
        }else{
            $where['orderby']	=	'id,desc';
        }
        $where['limit'] =   $pages['limit'];
        $List		=	$sysmsgM -> getList($where);
		$data['list'] = $List;
		$data['total'] = intval($total);
		$data['pageSize'] =intval($pageSize);
		$data['pageSizes'] =$pages['page_sizes'];

        $this->render_json(0,'ok',$data);
	}
	//发送系统消息
	function sendSys_action(){

	    if ($_POST['utype'] == ''){
            $this->render_json(1, '请选择用户身份');
        }
	    if($_POST['content']=="")
	    {
            $this->render_json(1, '请填写发送内容');
	    }

	    $userinfoM = $this -> MODEL('userinfo');
	    if($_POST['utype']=="5")
	    {
	        $userarr=@explode(",",$_POST['userarr']);

	        $where['PHPYUNBTWSTART']	=	'OR';

	        foreach($userarr as $v)
	        {
	            $where['username'][]    =   array('=', $v, 'OR');
	        }
	        $where['PHPYUNBTWEND']		=	'';
	    }else{
	        $where = array('usertype'=>$_POST['utype']);
	        $count = $userinfoM->getMemberNum($where);
	        // 每次发送1000条;
	        $size = 1000;
	        //循环次数
	        $num = ceil($count/$size);
	        $page = $_POST['page'];

	        $where['limit'] = array(($page - 1) * $size, $size);
	    }

	    $member	=	$userinfoM -> getList($where,array('field'=>"`uid`,`username`"));
	    if(!empty($member))
	    {
	        $data['content']			=	$_POST['content'];
	        foreach($member as $v)
	        {
	            $uid[]					=	$v['uid'];
	        }
	        $data['uid']				=	$uid;
	        $this -> MODEL('sysmsg') -> addInfo($data);
	        if (isset($num)){
	            // 按用户类型分批发送的
	            if ($num > $page){
	                $return = array(
	                    'msg'=>'共'.$count.'条，已发送'.$page * $size.'条',
	                    'error'=> 3,
	                    'page'=>$page + 1
	                );
                    $this->admin_json($return['error'],$return['msg'],$return['page']);
	            }
	        }
	        $return = array(
	            'msg'=>'发送完成',
	            'error'=> 0
	        );
	    }else{
	        $return = array(
	            'msg'=>'用户不存在',
	            'error'=> 1
	        );
	    }
        $this->admin_json($return['error'],$return['msg']);
	}
	function del_action()
	{
	    $del=$_POST['del'];
	    if($del){
	    	$return	=	$this -> MODEL('sysmsg') -> delInfo($del,'');
            $this->admin_json($return['errcode'] == 9 ? 0 : 1,$return['msg']);
	    }else{
            $this->render_json(1, '参数错误,请重试');
	    }
	}

}