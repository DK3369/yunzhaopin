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
class category_reason_controller extends adminCommon
{
    /**
     * 举报原因
     */
    function index_action()
    {
        $categoryM = $this->MODEL('category');
        $list = $categoryM->getReasonClassList();
        $this->render_json(0, '', $list);
    }

    function save_action()
    {
        $whereData = array();
        if ($_POST['id']) {
            $whereData['id'] = $_POST['id'];
            $addData['name'] = trim($_POST['name']);
        } else {
            $addData['name'] = trim($_POST['name']);
        }
        $categoryM = $this->MODEL('category');
        $return = $categoryM->addReasonClass($addData, $whereData);
        $this->admin_json($return['error'], $return['msg']);

    }

    function del_action()
    {
        $whereData = array();
        if ($_POST['del']) {
            $delid = pylode(',', $_POST['del']);
            $whereData['id'] = array('in', $delid);
            $data['type'] = 'all';
        }
        if (isset($_POST['delid'])) {
            $whereData['id'] = $_POST['delid'];
            $data['type'] = 'one';
        }
        if ($whereData) {
            $categoryM = $this->MODEL('category');
            $return = $categoryM->delReasonClass($whereData, $data);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    function ajax_action()
    {
        if ($_POST['id']) {
            $categoryM = $this->MODEL('category');
            $result = $categoryM->addReasonClass(array('name' => trim($_POST['name'])), array('id' => $_POST['id']));
            $this->admin_json($result['error'], $result['msg']);
        }
    }
}