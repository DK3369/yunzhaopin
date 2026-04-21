<?php

/**
 * Class dataCall_controller
 *
 * @author shiy
 * @version 7.0
 */

class dataCall_controller extends adminCommon
{

    function index_action()
    {

        $dataM = $this->MODEL('data');

        $where = array();

        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('outside', $where, $page, array('limit' => $pageSize));

        $list = array();
        $total = intval($pages['total']);
        if ($total > 0) {
            if (isset($_POST['order'])) {

                $where['orderby'] = $_POST['t'] . ' ,' . $_POST['order'];
            } else {

                $where['orderby'] = 'id';
            }

            $where['limit'] = $pages['limit'];
            $list = $dataM->getList($where);
        }


        $result = array(

            'list' => $list,
            'total' => intval($total),
            'pageSize' => $pageSize,
            'pageSizes' => $pages['page_sizes'],
        );
        $this->render_json(0, '', $result);
    }

    function index_base_data_action()
    {
        include CONFIG_PATH . "/db.data.php";
        $result = array(
            'dataCall' => $arr_data['datacall']
        );
        $this->render_json(0, '', $result);
    }

    //  获取预览信息展示
    function getPreviewData_action()
    {

        include LIB_PATH."datacall.class.php";
        $call   =   new datacall(PLUS_PATH . "data/", $this->obj);
        $row    =   $call->get_data($_POST['id']);//生成缓存
        $row    =   str_replace(array("<!--循环开始-->", "<!--循环结束-->", "\n", "\r"), "", $row);
        $result =   array('list' => $row,);
        $this->render_json(0, '', $result);
    }

    //  添加/修改保存数据信息
    function saveCall_action()
    {
        if ($_POST) {

            $dataM  =   $this->MODEL('data');
            /*$row    =   $dataM->getInfo(array('name' => $_POST['name'], 'id' => array('<>', $_POST['id'])));
            if (!empty($row)) {

                $this->render_json(1, '数据调用名称重复，请重新输入', $row);
            }*/

            include LIB_PATH."/datacall.class.php";
            $call   =   new datacall("../data/plus/data/", $this->obj);

            if ($_POST['id']) {

                $return =   $dataM->upInfo($_POST, array('id' => $_POST['id']));
            } else {

                $return =   $dataM->addInfo($_POST);
            }

            if ($return['errcode'] == 9){

                $this->admin_json(0, '数据调用（Id：' . $return['id'] . '）保存成功！');
            }else{

                $this->render_json(1, $return['msg']);
            }
        }else{

            $this->render_json(1, '参数异常，请稍后重试！');
        }
    }

    //  更新数据信息
    function upCall_action()
    {

        include LIB_PATH . "datacall.class.php";
        $call = new datacall("../data/plus/data/", $this->obj);
        $call->editcache($_POST['id']);
        $this->admin_json(0, '数据调用（Id：' . $_POST['id'] . '）更新成功！');
    }

    //  删除数据信息
    function delCall_action()
    {

        $dataM = $this->MODEL('data');
        $delId = $_POST['id'];
        if (!empty($delId)) {

            $return = $dataM->delData($delId);

            if ($return['error'] == 0) {

                $this->admin_json(0, $return['msg']);
            } else {

                $this->admin_json(1, $return['msg']);
            }
        } else {

            $this->render_json(1, '参数错误，请重试！');
        }
    }
}

?>