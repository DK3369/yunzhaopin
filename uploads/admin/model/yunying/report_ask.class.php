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

class report_ask_controller extends adminCommon
{

    function index_action()
    {
        $reportM    =   $this->MODEL('report');

        $where      =   array();

        $ftypeStr   =   intval($_POST['ftype']);
        $keywordStr =   trim($_POST['keyword']);
        $where['type']      =   1;
        if (!empty($keywordStr)) {
            if ($ftypeStr == 1) {
                $where['r_name']    =   array('like', $keywordStr);
            } else {
                $where['username']  =   array('like', $keywordStr);
            }
        }
        if(isset($_POST['status']) && $_POST['status']!==''){
            $where['status'] = $_POST['status'];
        }
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        } else {
            $where['orderby'] = 'id';
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('report', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = [];
        if ($pages['total'] > 0) {
            $where['limit'] = $pages['limit'];
            $list = $reportM->getReportList($where, array('utype' => 'admin', 'type' => 1));
            $list = $list['list'];
        }

        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    function saveresult_action()
    {
        $reportM=   $this->MODEL('report');
        $id     =   intval($_POST['pid']);
        $result =   trim($_POST['result']);

        $upData =   array(
            'result'    =>  $result,
            'status'    =>  1,
            'rtime'     =>  time(),
            'admin'     =>  $_SESSION['auid']
        );

        $return =   $reportM->upReport(array('id' => $id), $upData);
        if ($return){
            $logM   =   $this->MODEL('log');
            $logM->addAdminLog("问答投诉(ID:" . $id . ")处理");
            $this->render_json(0, '操作成功');
        }else{
            $this->render_json(1, '操作失败，请重试');
        }
    }

    function del_action()
    {
        $reportM    =   $this->MODEL('report');
        $del = $_POST['del'];
        $where['id'] = $del;
        $return =   $reportM->delReport($where, array('title' => '举报'));

        $this->render_json($return['errcode']==9?0:1, $return['msg']);
    }
    function delquestion_action()
    {
        if ($_POST['del']) {
            $askM = $this->MODEL('ask');
            $askM->delquestion($_POST['del'],array('utype'=>'admin'));
            $this->render_json(0, '问答(ID:' . $_POST['del'] . ')删除成功！');
        }
    }
    function getclass_action(){
        $AskM	=	$this -> MODEL('ask');
        if($_POST['pid']){
            $class	=	$AskM -> getQclassList(array('pid'=>$_POST['pid'],'orderby'=>'sort,desc'),array('field'=>'id,name,pid'));
            $this->render_json(0,'ok',compact('class'));
        }
    }
    function edit_action(){

        $AskM	=	$this -> MODEL('ask');
        if($_POST['id']){
            $id		= 	intval($_POST['id']);
            $info		=		$AskM -> getInfo($id);
        }
        $all_class	=	$AskM -> getQclassList(array('orderby'=>'sort,desc'),array('field'=>'id,name,pid'));
        $class = array();
        foreach($all_class as $k=>$v){
            if($v['id'] == $info['cid']){
                $pid	=	$v['pid'];
                $all_class[$k]['is_select']		=	'1';
            }
            if($v['pid'] == '0'){
                $class[]	=	$v;
            }
        }
        $this->render_json(0,'ok',compact('info','class','pid'));
    }
    function save_action(){
        $AskM	=	$this -> MODEL('ask');
        if($_POST['id']){
            $_POST['content']	=	str_replace("&amp;","&",$_POST['content']);

            $nbid	=	$AskM -> upAskInfo(array('id'=>intval($_POST['id'])),$_POST);

            $this->render_json(isset($nbid)? 0: 1, isset($nbid)?'更新成功':'更新失败');
        }
    }
}

?>