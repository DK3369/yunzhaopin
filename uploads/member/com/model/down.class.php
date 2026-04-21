<?php

/*
 *
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class down_controller extends company
{
    //下载简历列表
    function index_action()
    {

        $where = array(
            'comid' => $this->uid,
            'usertype' => $this->usertype,
            'isdel' => 9,
        );

        if (isset($_GET['keyword']) && trim($_GET['keyword'])) {

            $resumeM = $this->MODEL('resume');
            $nwhere['PHPYUNBTWSTARTB'] = '';
            $nwhere['name'] = array('like',trim($_GET['keyword']));
            $nwhere['uname']	    =	array('like', trim($_GET['keyword']), 'OR');
            $nwhere['PHPYUNBTWENDB']  = '';
            $resume = $resumeM->getSimpleList($nwhere, array('field' => 'id'));

            if ($resume) {
                $uids = array();
                foreach ($resume as $v) {
                    if ($v['id'] && !in_array($v['id'], $uids)) {
                        $uids[] = $v['id'];
                    }
                }
                $where['eid'] = array('in', pylode(',', $uids));
            }
            $urlarr['keyword'] = trim($_GET['keyword']);
        }
        
            $this->public_action();

            $urlarr['c'] = $_GET['c'];
            $urlarr['page'] = '{{page}}';
            $pageurl = Url('member', $urlarr);

            $pageM = $this->MODEL('page');
            $pages = $pageM->pageList('down_resume', $where, $pageurl, $_GET['page']);

            if ($pages['total'] > 0) {

                $where['orderby'] = 'id';
                $where['limit'] = $pages['limit'];

                $downM = $this->MODEL('downresume');
                $List = $downM->getList($where, array('uid'=>$this->uid, 'utype' => 'pc'));
                $this->yunset('rows', $List['list']);
            }


            $CacheM = $this->MODEL('cache');
            $cache = $CacheM->GetCache(array('com'));
            $this->yunset($cache);

            //邀请面试选择职位
            $this->yqmsInfo();
            $this->company_satic();

            $this->yunset('total', $pages['total']);
            $this->com_tpl('down');
        }

        //删除已下载简历
        function del_action()
        {

            if ($_GET['id']) {
                $id = intval($_GET['id']);
            } elseif ($_POST['delid']) {
                $id = $_POST['delid'];
            }
            $downM = $this->MODEL('downresume');
            $return = $downM->delInfo($id, array('uid' => $this->uid, 'usertype' => $this->usertype));
            $this->layer_msg($return['msg'], $return['errcode'], $return['layertype'], $_SERVER['HTTP_REFERER']);
        }

        //举报简历
        function report_action()
        {

            $data = array(

                'reason' => $_POST['reason'],
                'c_uid' => (int)$_POST['uid'],
                'inputtime' => time(),
                'p_uid' => $this->uid,
                'did' => $this->userid,
                'usertype' => $this->usertype,
                'eid' => (int)$_POST['eid'],
                'r_name' => $_POST['r_name'],
                'username' => $this->username
            );
            $reportM = $this->MODEL('report');
            $return = $reportM->ReportResume($data);
            $this->ACT_layer_msg($return['msg'], $return['errcode'], $_SERVER['HTTP_REFERER']);
        }

        //导出下载简历
        function xls_action()
        {

            if ($_POST['delid']) {

                $downM = $this->MODEL('downresume');
                $list = $downM->Xls($_POST['delid'], array('uid' => $this->uid, 'usertype' => $this->usertype));
                if ($list) {

                    $this->yunset('list', $list);
                    header("Content-Type: application/vnd.ms-excel");
                    header("Content-Disposition: attachment; filename=已下载的简历.xls");
                    $this->com_tpl('resume_xls');
                }
            }
        }

}
?>