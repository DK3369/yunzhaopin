<?php

class company_job_refresh_log_controller extends adminCommon
{
    //职位日志
    function sxLog_action()
    {

    }

    function index_action()
    {
        $pageM = $this->MODEL('page');
        $logM = $this->MODEL('log');
        $where = array();
        $type = 1;
        if ($_POST['type']) {
            $type = (int)$_POST['type'];
            $where['type'] = $_POST['type'];
        }
        $keyStr = trim($_POST['keyword']);
        $typeStr = (int)$_POST['ktype'];
        if (!empty($keyStr)) {
            $jobWhere = array();
            if ($typeStr == 1) {
                $jobWhere['com_name'] = array('like', $keyStr);
            } elseif ($typeStr == 2) {
                if ($type == '3') {
                    $jobWhere['job_name'] = array('like', $keyStr);
                } else {
                    $jobWhere['name'] = array('like', $keyStr);
                }
            }

            $jobS = $logM->getJobBySxLog($jobWhere, array('field' => 'id', 'type' => $type));
            $jobIds = array();
            foreach ($jobS as $jk => $jv) {
                if (!in_array($jv['id'], $jobIds)) {
                    $jobIds[] = $jv['id'];
                }
            }
            $where['jobid'] = array('in', pylode(',', $jobIds));
        }
        if (isset($_POST['uid']) && !empty($uid)) {
            $where['uid'] = $_POST['uid'];
        }
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('job_refresh_log', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $list = array();
        if ($pages['total'] > 0) {
            //limit order 只有在列表查询时才需要
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = 'id,desc';
            }

            $where['limit'] =  $pages['limit'];
            $list = $logM->getSxJobLogList($where, array('utype' => 'admin'));
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    function delSxLog_action()
    {
        $LogM = $this->MODEL('log');
        if ($_POST['del']) {
            //array
            $id = $_POST['del'];
        } else {
            //int
            $id = $_POST['id'];
        }
        $arr = $LogM->delSxJobLog($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg'] . '(ID:' . pylode(',', $id) . ')');
        } else {
            $this->render_json($arr['errcode'], $arr['msg']);
        }
    }
}