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
class category_introduce_class_controller extends adminCommon
{
    /**
     * 自我介绍
     */
    function index_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['orderby'] = array('sort,asc', 'id,asc');
        $list = $categoryM->getIntroduceClassList($whereData);
        $this->render_json(0, '', $list);
    }

    /**
     * 获取单个自我介绍分类
     */
    function classadd_action()
    {
        $categoryM = $this->MODEL('category');

        if ($_POST['id']) {
            $info = $categoryM->getIntroduceClass(array('id' => $_POST['id']));

            $this->render_json(0, '', $info);
        }
    }

    function save_action()
    {
        $_POST = $this->post_trim($_POST);
        if ($_POST) {
            $data = array(
                'name' => $_POST['name'],
                'sort' => $_POST['sort'],
                'content' => strip_tags($_POST['content'])
            );

            $categoryM = $this->MODEL('category');

            if ($_POST['id']) {

                $return = $categoryM->addIntroduceClass($data, array('id' => $_POST['id']));

            } else {

                $return = $categoryM->addIntroduceClass($data);
            }
            $this->admin_json($return['error'], $return['msg']);
        }
    }

    //删除
    function del_action()
    {
        $categoryM = $this->MODEL('category');
        if ((int)$_POST['delid']) {
            $ids = $_POST['delid'];
            $whereData['id'] = $_POST['delid'];
            $data['type'] = 'one';
        } else if ($_POST['del']) { //批量删除
            $ids = pylode(',', $_POST['del']);
            $whereData['id'] = array('in', $ids);
            $data['type'] = 'all';
        }

        if ($whereData) {
            $return = $categoryM->delIntroduceClass($whereData, $data);
            if($return['errcode'] == 9){
                $this->admin_json(0, $return['msg'] . '(ID:' . $ids . ')');
            } else {
                $this->render_json(8, $return['msg']);
            }
        }
    }

    function ajax_action()
    {
        $categoryM = $this->MODEL('category');
        $whereData['id'] = $_POST['id'];
        if ($_POST['name']) {
            $addData['name'] = $_POST['name'];
        } else {
            $addData['sort'] = $_POST['sort'];
        }
        if ($addData) {
            $return = $categoryM->addIntroduceClass($addData, $whereData);
            $this->admin_json($return['error'], $return['msg']);
        }
    }
}