<?php

/**
 * Class dataRecycle_controller
 *
 * @author shiy
 * @version 7.0
 */

class dataRecycle_controller extends adminCommon
{

    function index_action()
    {

        $recycleM = $this->MODEL('recycle');

        $where = array();

        if (isset($_POST['username']) && !empty($_POST['username'])) {

            $where['username'] = array('like', $_POST['username']);
        }

        if (isset($_POST['keyword']) && !empty($_POST['keyword'])) {

            $where['body'] = array('like', $_POST['keyword']);
        }

        if (isset($_POST['table']) && !empty($_POST['table'])) {

            $where['tablename'] = array('like', $_POST['table']);
        }

        if (isset($_POST['time']) && !empty($_POST['time'])) {

            $start  =   date('Y-m-d', $_POST['time'][0] / 1000);
            $end    =   date('Y-m-d', $_POST['time'][1] / 1000);

            $where['PHPYUNBTWSTART_A'] = '';
            $where['ctime'][] = array('>=', strtotime($start . " 00:00:00"));
            $where['ctime'][] = array('<=', strtotime($end . " 23:59:59"));
            $where['PHPYUNBTWEND_A'] = '';
        }

        if (isset($_POST['ident']) && !empty($_POST['ident'])) {

            $where['ident'] = $_POST['ident'];
        }

        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);

        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('recycle', $where, $page, array('limit' => $pageSize));

        $list = array();
        $total = intval($pages['total']);
        if ($total > 0) {
            if (isset($_POST['order'])) {

                $where['orderby'] = $_POST['t'] . ' ,' . $_POST['order'];
            } else {

                $where['orderby'] = 'id';
            }

            $where['limit'] = $pages['limit'];
            $list = $recycleM->getList($where);
        }

        $result = array(

            'list' => $list,
            'total' => intval($total),
            'pageSize' => $pageSize,
            'pageSizes' => $pages['page_sizes']
        );
        $this->render_json(0, '', $result);
    }

    function recover_action()
    {

        if ($_POST['id']) {

            $recycleM = $this->MODEL('recycle');
            $return = $recycleM->recoverTb(array('id' => $_POST['id']));
            $error = $return['errcode'] == 9 ? 0 : 1;
            $this->admin_json($error, $return['msg']);
        } else {

            $this->render_json(1, '参数错误，请重试！');
        }
    }

    function recoverAll_action()
    {

        if ($_POST['ident']) {

            $recycleM = $this->MODEL('recycle');
            $return = $recycleM->recoverByIdent(array('ident' => $_POST['ident']));
            $error = $return['errcode'] == 9 ? 0 : 1;
            $this->admin_json($error, $return['msg']);
        } else {

            $this->render_json(1, '参数错误，请重试！');
        }
    }

    function delRecycle_action()
    {

        $recycleM = $this->MODEL('recycle');

        $delId = $_POST['id'];
        if (!empty($delId)) {

            if (is_array($delId)) {

                $where['id'] = array('in', pylode(',', $delId));
            } else {

                $where['id'] = $delId;
            }

            $return = $recycleM->delRecycle($where);

            if ($return['error'] == 0) {

                $this->admin_json(0, $return['msg']);
            } else {

                $this->admin_json(1, $return['msg']);
            }
        } else {

            $this->render_json(1, '参数错误，请重试！');
        }

        $this->db->Query("TRUNCATE `" . $this->def . "recycle`");
        $this->layer_msg("已清空回收站！", 9, 0, $_SERVER['HTTP_REFERER']);
    }

    function tuncateRecycle_action()
    {

        if (isset($_POST['recycle']) && $_POST['recycle'] == 'tuncate') {

            $this->db->Query("TRUNCATE `" . $this->def . "recycle`");
            $this->admin_json(0, '回收站已清空！');
        } else {

            $this->admin_json(1, '清空回收站操作错误！');
        }
    }

}

?>