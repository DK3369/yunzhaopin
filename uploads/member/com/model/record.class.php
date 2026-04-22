<?php



class record_controller extends company
{

    function index_action()
    {
        $this->public_action();

        $where['comid'] =   $this->uid;
        $urlarr['c']    =   $_GET['c'];
        $urlarr['page'] =   '{{page}}';
        $pageurl        =   Url('member', $urlarr);

        $pageM  =   $this->MODEL('page');
        $pages  =   $pageM->pageList('user_entrust_record', $where, $pageurl, $_GET['page']);

        if ($pages['total'] > 0) {

            $where['orderby']   =   'id';
            $where['limit']     =   $pages['limit'];

            $userEntrustM       =   $this->MODEL('userEntrust');

            $List               =   $userEntrustM->getRecordList($where);
        }

        //邀请面试选择职位
        $this->yqmsInfo();

        $this->yunset("rows", $List['list']);
        $this->com_tpl('record');
    }

    function del_action()
    {
        if ($_POST['delid'] || $_GET['del']) {
            if ($_GET['del']) {

                $id =   intval($_GET['del']);
            } elseif ($_POST['delid']) {

                $id =   $_POST['delid'];
            }

            $userEntrustM   =   $this->MODEL('userEntrust');

            $return         =   $userEntrustM->delRecord($id, array('uid' => $this->uid, 'usertype' => $this->usertype));

            $this->layer_msg($return['msg'], $return['errcode'], $return['layertype'], $_SERVER['HTTP_REFERER']);
        }
    }
}

?>