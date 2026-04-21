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

class admin_user_member_controller extends adminCommon
{

	
	/**
	 * 会员  -  : 解绑日志
	 */
	function writtenOffLog_action(){
	    
  	    $where['opera']     =  12;

        $where['PHPYUNBTWSTART_A'] = '';
        $where['content'][] = array('like', '解除绑定');
        $where['content'][] = array('like', '解绑', 'OR');
        $where['content'][] = array('like', '解除', 'OR');
        $where['PHPYUNBTWEND_A'] = '';

        if ($_POST['utype']) {
            $where['usertype']  =   intval($_POST['utype']);;
        }

        if ($_POST['keyword']) {

            $type       =   intval($_POST['type']);
            $keyword    =   trim($_POST['keyword']);

            if ($type == 1) {

                $userInfoM  =   $this->MODEL('userinfo');
                $member     =   $userInfoM->getList(array('username' => array('like', $keyword)), array('field' => '`uid`'));
                if ($member) {
                    $muids = array();
                    foreach ($member as $val) {
                        $muids[] = $val['uid'];
                    }
                    $where['uid'] = array('in', pylode(',', $muids));
                }else{
                    $this->render_json(0,'暂无数据',['data'=>[],'total'=>0]);
                }
            } elseif ($type == 2) {

                $where['content'] =   array('like', $keyword);
            }
        }
	    
	    if($_POST['time']){
	        $times  =  @explode('~',$_POST['time']);
	        $where['PHPYUNBTWSTART']     =  '';
	        $where['ctime'][]       =  array('>=',strtotime($times[0]));
	        $where['ctime'][]       =  array('<=',strtotime($times[1].'23:59:59'));
	        $where['PHPYUNBTWEND']  =  '';
	    }
        //提取分页
        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM	=	$this  -> MODEL('page');
        $pages	=	$pageM -> adminPageList('member_log',$where,$page,array('limit' => $pageSize));
        if(!$pages['total']){
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }

        //limit order 只有在列表查询时才需要
        if($_POST['order']){
            $where['orderby']  =  $_POST['t'].','.$_POST['order'];
        }else{
            $where['orderby']  =  'id';
        }
        $where['limit']		   =  $pages['limit'];

        $logM   =   $this->MODEL('log');
        $List   =   $logM->getMemlogList($where, array('utype' => 'admin'));
        foreach ($List as &$v){
            $v['ctime_ymd'] = $v['ctime']?date('Y-m-d H:i:s',$v['ctime']):'';
        }
        $this->render_json(0,'',['data'=>$List,'total'=>(int)$pages['total'],'pageSizes'=>$pages['page_sizes']]);
	}
	
	/**
	 * 会员 - ： 解绑日志删除
	 */
	function delwflog_action(){
	    
	    if (!$_POST['del']){
            $this->render_json(1,'参数错误');
        }
	        
        if (is_array($_POST['del'])){
            $id     =  pylode(',', $_POST['del']);
        }else{
            $id = $_POST['del'];
        }
        $where  =  array('id'=> array('in',$id));

	    $logM    =  $this -> MODEL('log');
	    $return  =  $logM -> delMemlog($where);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
	}
}
?>