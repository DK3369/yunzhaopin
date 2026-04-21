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

class part_controller extends user_controller
{

    /**
     * wxapp个人会员兼职收藏列表
     */
    function partCollect_action()
    {

        $partM          =   $this->MODEL('part');
        $where['uid']   =   $this->member['uid'];

        if (isset($_POST['name']) && $_POST['name'] !== '') {

            $pWhere['name'] =   array('like', '%' . $_POST['name'] . '%');

            $pList          =   $partM->getList($pWhere, array('field' => 'id'));

            $partIds        =   array();

            foreach ($pList as $pk => $pv) {

                $partIds[]  =   $pv['id'];
            }

            $where['jobid'] =   array('in', pylode(',', $partIds));
        }

        $total  =   $partM->getPartcollectNum($where);

        $page   =   $_POST['page'];
        $limit  =   $_POST['limit'];
        $limit  =   !$limit ? 20 : $limit;

        $where['orderby']   =   array('id,desc');
        if ($page) {

            $pagenav        =   ($page - 1) * $limit;
            $where['limit'] =   array($pagenav, $limit);
        } else {

            $where['limit'] =   array('', $limit);
        }

        $rows       =   $partM->getPartCollectList($where);

        if (!empty($rows)) {

            $list   =   count($rows) ? $rows : array();
            $error  =   1;
        } else {

            $error  =   2;
        }
        $this->render_json($error, '', $list, $total);
    }

    /**
     * 删除兼职收藏列表
     */
    function delfavpart_action()
    {

        $partM  =   $this->MODEL('part');

        $uid    =   $this->member['uid'];
        $id     =   intval($_POST['ids']);

        $return =   $partM->delPartCollect($id, array('uid' => $uid, 'usertype' => $this->member['usertype']));

        $error  =   $return['errcode'] == 9 ? 1 : 2;

        $msg    =   preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $return['msg']));

        $this->render_json($error, $msg);
    }

    /**
     * 删除兼职报名
     */
    function delapply_action()
    {

        $partM  =   $this->MODEL('part');

        $uid    =   $this->member['uid'];
        $id     =   intval($_POST['ids']);

        $return =   $partM->delPartApply($id, array('uid' => $uid, 'usertype' => $this->member['usertype']));

        $error  =   $return['errcode'] == 9 ? 1 : 2;

        $msg    =    preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $return['msg']));

        $this->render_json($error, $msg);
    }

    /**
     * 个人兼职报名管理
     */
    function applylist_action()
    {

        $partM  =   $this->MODEL('part');

        $where['uid']   =   $this->member['uid'];

        if (isset($_POST['name']) && $_POST['name'] !== '') {

            $pWhere['name'] =   array('like', '%' . $_POST['name'] . '%');

            $pList          =   $partM->getList($pWhere, array('field' => 'id'));

            $partIds        =   array();

            foreach ($pList as $pk => $pv) {

                $partIds[]  =   $pv['id'];
            }

            $where['jobid'] =   array('in', pylode(',', $partIds));
        }

        $total  =   $partM->getPartSqNum($where);
        $page   =   $_POST['page'];

        $limit  =   $_POST['limit'];
        $limit  =   !$limit ? 20 : $limit;

        $where['orderby']   =   array('id,desc');

        if ($page) {

            $pagenav        =   ($page - 1) * $limit;
            $where['limit'] =   array($pagenav, $limit);
        } else {

            $where['limit'] =   array('', $limit);
        }

        $rows       =   $partM->getPartSqList($where);
        if (is_array($rows) && !empty($rows)) {

            $data   =   count($rows) ? $rows : array();
            $error  =   1;
        } else {

            $error  =   2;
        }
        $this->render_json($error, '', $data, $total);

    }

}