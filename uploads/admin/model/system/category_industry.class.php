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
class category_industry_controller extends adminCommon
{
    /**
     * 行业类别
     */
    function index_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['orderby'] = array('sort,desc', 'id,asc');
        $list = $categoryM->getIndustryClassList($whereData);

        $this->render_json(0, '', $list);
    }

    //添加
    function add_action()
    {
        $_POST = $this->post_trim($_POST);
        $data['name'] = @explode('-', $_POST['name']);
        $categoryM = $this->MODEL('category');
        $return = $categoryM->addIndustryClass($data);

        $this->admin_json($return['error'], $return['msg']);
    }

    //删除
    function del_action()
    {
        $whereData = array();
        $data = array();
        $categoryM = $this->MODEL('category');
        if ($_POST['delid']) {//单个删除
            $whereData['id'] = $_POST['delid'];
            $data['type'] = 'one';
        }
        if ($_POST['del']) {//批量删除
            $whereData['id'] = array('in', pylode(',', $_POST['del']));
            $data['type'] = 'all';
        }
        if ($whereData) {
            $return = $categoryM->delIndustryClass($whereData, $data);
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    function ajax_action()
    {
        if ($_POST) {
            $categoryM = $this->MODEL('category');
            $whereData['id'] = $_POST['id'];
            $addData['sort'] = $_POST['sort'];
            $addData['name'] = $_POST['name'];

            $return = $categoryM->upIndustryClass($addData, $whereData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }
}