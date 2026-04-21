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

class info_errorlog_controller extends adminCommon
{
    function index_action()
    {
        $where = array();
        $errlogM    =   $this->MODEL('errlog');
        if (trim($_POST['keyword'])) {
            if ($_POST['type'] == "1" || $_POST['type'] == '') {
                $where['uid']       =   $_POST['keyword'];
            } else {
                $where['content']   =   array('like', trim($_POST['keyword']));
            }
        }

        if (isset($_POST['ctime'])) {
            if ($_POST['ctime'] == '1') {
                $where['ctime']     =   array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {
                $where['ctime']     =   array('>=', strtotime('-' . $_POST['ctime'] . 'day'));
            }
        }

        if ($_POST['logtype']) {
            $where['type']          =   $_POST['logtype'];
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('error_log', $where, $page, array('limit' => $pageSize));
        $total  =   intval($pages['total']);
        if ($total > 0) {
            if($_POST['order']){
                $where['orderby']		=	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby']		=	array('id,desc');
            }
            $where['limit'] =   $pages['limit'];
        }

        $List =	$errlogM -> getList($where, array('utype'=>'admin'));
        $data = array(
            'list'=> $List ? $List : array(),
            'total'=> intval($total),
            'pageSize'=>intval($pageSize),
            'pageSizes'=>$pages['page_sizes']
        );
        $this->render_json(0,'ok',$data);
    }

    function del_action()
    {
        $errlogM    =   $this->MODEL('errlog');
        if ($_POST["id"] == 'all') {
            $where['id']    =   array('>', 0);
            $errlogM->delErrlog($where);
            $this->admin_json(0, "已清空错误日志！");
        } else {

            if ($_POST["del"]) {
                $where['id']    =   array('in', pylode(',', $_POST["del"]));
            } elseif (isset($_POST["id"])) {
                $where['id']    =   $_POST["id"];
            }
            $return =   $errlogM->delErrlog($where);
            $this->admin_json($return['errcode'] == 9 ? 0 : 1,$return['msg']);
        }
    }
}