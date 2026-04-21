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

class report_resume_controller extends adminCommon
{

    function index_action()
    {
        $reportM    =   $this->MODEL('report');

        $where      =   array();

        $ftypeStr   =   intval($_POST['ftype']);
        $keywordStr =   trim($_POST['keyword']);
        $where['type']      =   0;
        $where['usertype']  = 2;
        $reports    =   $this->obj->select_all('report', array('usertype' => 2, 'type' => 0), '`eid`,`p_uid`');
        $eids   =   $uids   =   $comIds =   array();
        foreach ($reports as $rk => $rv) {
            $eids[$rv['eid']]       =   $rv['eid'];
            $comIds[$rv['p_uid']]   =   $rv['p_uid'];
        }
        if ($ftypeStr == 1) {        //  简历名称
            $resumeA            =   $this->obj->select_all('resume_expect', array('id' => array('in', pylode(',', $eids)), 'name' => array('like', $keywordStr)), 'id');
            if (!empty($resumeA)) {
                $reIds          =   array();
                foreach ($resumeA as $rak => $rav) {
                    $reIds[]    =   $rav['id'];
                }
            }
            $where['eid']       =   array('in', pylode(',', $reIds));
        } elseif ($ftypeStr == 2) {   //  个人姓名

            $where['r_name']    =   array('like', $keywordStr);
        } elseif ($ftypeStr == 3) {   //  企业名称

            $coms               =   $this->obj->select_all('company', array('uid' => array('in', pylode(',', $comIds)), 'name' => array('like', $keywordStr)), 'uid');
            if (!empty($coms)) {

                $puids          =   array();
                foreach ($coms as $ck => $cv) {
                    $puids[]    =   $cv['uid'];
                }
            }
            $where['p_uid']     =   array('in', pylode(',', $puids));
        }
        if(isset($_POST['status']) && $_POST['status']!==''){
            $where['status'] = $_POST['status'];
        }
        if ($_POST['order']) {
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        } else {
            $where['orderby'] = 'id';
        }

        $pageM = $this->MODEL('page');

        $pages = $pageM->adminPageList('report', $where, $_POST['page'], array('limit' => $_POST['limit'], 'maxPage' => true));
        extract($pages);
        $limit = $pages['limit'][1];

        $list = [];
        if ($pages['total'] > 0) {
            $where['limit'] = $pages['limit'];
            $list = $reportM->getReportList($where, array('utype' => 'admin', 'type' => 0));
            $list = $list['list'];
        }
        $this->render_json(0, 'ok', compact('list', 'total', 'page_sizes', 'limit', 'page'));
    }

    function index_base_data_action()
    {
        $integral_pricename = $this->config['integral_pricename'];
        $this->render_json(0, 'ok', compact('integral_pricename'));
    }

    function saveresultall_action()
    {
        $reportM    =   $this->MODEL('report');
        $statisM    =   $this->MODEL('statis');
        $integralM  =   $this->MODEL('integral');
        $orderM     =   $this->MODEL('companyorder');
        $downM      =   $this->MODEL('downresume');
        if (!empty($_POST['rid'])) {
            $rids   =  $_POST['rid'];
            if (isset($_POST['datafh']) && intval($_POST['datafh']) == 1) {
                $reportArr  =   $reportM->getReportList(array('id' => array('in', pylode(",", $rids))));
                $reportList =   $reportArr['list'];
                $comid  =   $dreport    =   array();
                foreach ($reportList as $key => $val) {
                    $dResume    =   $downM->getDownNum(array('eid' => $val['eid'], 'uid' => $val['c_uid'], 'comid' => $val['p_uid'], 'usertype' => 2));
                    if ($dResume > 0 && $val['datafh'] != 1) {
                        $val['fhtype']  =   1;
                        $comid[]        =   $val['p_uid'];
                        $dreport[]      =   $val;
                    }
                }

                if (isset($dreport) && !empty($dreport)) {
                    $order  =   $orderM->getList(array('type' => 19, 'order_remark' => array('like', '下载简历'), 'uid' => array('in', pylode(',', $comid))));
                    $compay =   $integralM->getList(array('type' => 1, 'pay_type' => '12', 'pay_remark' => array('like', '简历下载'), 'com_id' => array('in', pylode(',', $comid))));
                    foreach ($dreport as $key => $val) {
                        foreach ($order as $k => $v) {
                            if ($val['p_uid'] == $v['uid'] && $val['eid'] == $v['sid']) {
                                $dreport[$key]['fhtype']    =   2;//金额操作
                                $dreport[$key]['fhprice']   =   $v['order_price'];
                            }
                        }
                        foreach ($compay as $k => $v) {
                            if ($val['p_uid'] == $v['com_id'] && $val['eid'] == $v['eid']) {
                                if ($dreport[$key]['fhtype'] == 2) {
                                    $dreport[$key]['fhtype']        =   4;//金额+积分操作
                                    $dreport[$key]['fhprice_two']   =   abs($v['order_price']);
                                } else {
                                    $dreport[$key]['fhtype']        =   3;//积分操作
                                    $dreport[$key]['fhprice']       =   abs($v['order_price']);
                                }
                            }
                        }
                    }

                    foreach ($dreport as $key => $val) {
                        if ($val['fhtype'] == 2) {
                            $integralM->company_invtal($val['p_uid'], 2, $val['fhprice'], true, '举报简历返还金额', true, 2, 'packpay', 99);
                        } elseif ($val['fhtype'] == 3) {
                            $integralM->company_invtal($val['p_uid'], 2, $val['fhprice'], true, '举报简历返还积分', true, 2, 'integral', 99);
                        } elseif ($val['fhtype'] == 4) {
                            $integralM->company_invtal($val['p_uid'], 2, $val['fhprice'], true, '举报简历返还金额', true, 2, 'packpay', 99);
                            $integralM->company_invtal($val['p_uid'], 2, $val['fhprice_two'], true, '举报简历返还积分', true, 2, 'integral', 99);
                        } else {
                            $statisM->upInfo(array('down_resume' => array('+', 1)), array('usertype' => 2, 'uid' => $val['p_uid']));
                        }
                    }
                }
            }
            $data['datafh'] =   $_POST['datafh'];
            $data['result'] =   trim($_POST['result']);
            $data['admin']  =   $_SESSION['auid'];
            $data['rtime']  =   time();
            $data['status'] =   1;
            $where['id']    =   array('in', pylode(",", $rids));

            $return         =   $reportM->upReport($where, $data);
            if ($return){
                $logM   =   $this->MODEL('log');
                $logM->addAdminLog("简历投诉(ID:" . pylode(',', $rids) . ")处理");
                $this->render_json(0, '操作成功');
            }else{
                $this->render_json(1, '操作失败，请重试');
            }
        }
    }
    function saveresult_action()
    {
        $reportM=   $this->MODEL('report');
        $id     =   intval($_POST['pid']);
        $result =   trim($_POST['result']);
        $eid    =   intval($_POST['eid']);
        $uid    =   intval($_POST['uid']);

        $downM      =   $this->MODEL('downresume');
        $statisM    =   $this->MODEL('statis');
        $integralM  =   $this->MODEL('integral');
        $orderM     =   $this->MODEL('companyorder');
        $rtComIds = array();// 初始化需要返还下载数量的企业用户id
        if (isset($_POST['tongbu']) && $_POST['tongbu'] == 1){     //  同步处理

            $reportList     =   $reportM -> getReportList(array('eid' => $eid, 'type' => 0, 'usertype' => 2, 'status' => 0));
            $reportResume   =   $reportList['list'];
            $rids   =   $comIds =   array();
            foreach ($reportResume as $key => $val){
                $rids[]     =   $val['id'];// 记录待处理的举报记录id
                $comIds[]   =   $val['p_uid'];
            }
            if (isset($_POST['datafh']) && $_POST['datafh'] == 1){  //  返还企业特权（积分/简历数）
                $comIdA =   $dReport    =   array();
                $dResume=   $downM -> getSimpleList(array('eid' => $eid, 'comid' => array('in', pylode(",", $comIds))));
                foreach ($reportResume as $key => $val){
                    foreach ($dResume as $k => $v) {
                        if ($v['comid'] == $val['p_uid'] && $v['eid'] == $val['eid'] && $val['datafh']!= 1) {
                            $dReport[]  =   $val;
                            $comIdA[]   =   $val['p_uid'];
                        }
                    }
                }
                $notUid =   array();
                $order  =   $orderM->getList(array('type' => 19, 'sid' => $eid, 'order_remark' => array('like', '下载简历'), 'uid' => array('in', pylode(',', $comIdA))));
                $compay =   $integralM->getList(array('type' => 1, 'eid' => $eid, 'pay_type' => '12', 'pay_remark' => array('like', '简历下载'), 'com_id' => array('in', pylode(',', $comIdA))));
                foreach ($order as $k => $v) {
                    $notUid[]   =   $v['uid'];
                    $integralM->company_invtal($v['uid'], 2, $v['order_price'], true, '举报简历返还金额', true, 2, 'packpay', 99);
                }
                foreach ($compay as $k => $v) {
                    $noUid[]    =   $v['com_id'];
                    $integralM->company_invtal($v['com_id'], 2, abs($v['order_price']), true, '举报简历返还积分', true, 2, 'integral', 99);
                }
                foreach ($dReport as $drk => $drv) {
                    if (!in_array($drv['p_uid'], $notUid) && in_array($drv['p_uid'], $comIdA)) {
                        !in_array($drv['p_uid'], $rtComIds) && array_push($rtComIds, $drv['p_uid']);// 记录返还下载数量的企业用户id
                        // 没有充值购买记录，有下载记录的，返还下载数量
                        $statisM->upInfo(array('down_resume' => array('+', 1)), array('usertype' => 2, 'uid' => $drv['p_uid']));
                    }
                }
            }

            $where['id']    =   array('in', pylode(",", $rids));
        }else{
            $rids = array($id);// 待处理的举报记录id
            $report =   $reportM->getReportOne(array('id' => $id), array('field' => '`p_uid`, `datafh`'));

            if (intval($_POST['datafh']) == 1 && $report['datafh']!= 1) {

                $comId      =   $report['p_uid'];

                $dResume    =   $downM->getDownResumeInfo(array('eid' => $eid, 'uid' => $uid, 'comid' => $comId), array('field' => '`eid`'));
                if (!empty($dResume)) {

                    $order  =   $orderM->getInfo(array('type' => 19, 'sid' => $eid, 'order_remark' => array('like', '下载简历'), 'uid' => $comId), array('field' => '`order_price`'));
                    $compay =   $integralM->getInfo(array('type' => 1, 'eid' => $eid, 'pay_type' => '12', 'pay_remark' => array('like', '下载简历'), 'com_id' => $comId), array('field' => '`order_price`'));
                }

                if (isset($order) && !empty($order)) {
                    $integralM->company_invtal($comId, 2, $order['order_price'], true, '举报简历返还金额', true, 2, 'packpay', 99);
                }
                if (isset($compay) && !empty($compay)) {
                    $integralM->company_invtal($comId, 2, abs($compay['order_price']), true, '举报简历返还积分', true, 2, 'integral', 99);
                }
                if (empty($order) && empty($compay) && !empty($dResume)) {
                    !in_array($comId, $rtComIds) && array_push($rtComIds, $comId);// 记录返还下载数量的企业用户id
                    // 没有充值购买记录，有下载记录的，返还下载数量
                    $statisM->upInfo(array('down_resume' => array('+', 1)), array('usertype' => 2, 'uid' => $comId));
                }
            }
            $where['id']    =   intval($_POST['pid']);
        }

        $upData     =   array(
            'status'    =>  1,
            'result'    =>  $result,
            'rtime'     =>  time(),
            'admin'     =>  $_SESSION['auid'],
            'datafh'    =>  $_POST['datafh']
        );
        $return     =   $reportM->upReport($where, $upData);
        if ($return){
            $logM   =   $this->MODEL('log');
            $m = $rtComIds ? "，已为ID为(" . implode(',', $rtComIds) . ")的企业用户返还简历下载数量" : "";
            $logM->addAdminLog("简历投诉(ID:" . implode(',', $rids) . ")处理" . $m);
            $this->render_json(0, '操作成功');
        }else{
            $this->render_json(1, '操作失败，请重试');
        }
    }
    /**
     * @desc 删除举报简历,返还套餐特权
     */
    function delresume_action()
    {

        $reportM    =   $this->MODEL('report');
        $resumeM    =   $this->MODEL('resume');
        $statisM    =   $this->MODEL('statis');

        $integralM  =   $this->MODEL('integral');
        $orderM     =   $this->MODEL('companyorder');
        $downM      =   $this->MODEL('downresume');

        $eid        =   intval($_POST['eid']);
        $id         =   intval($_POST['id']);
        $uid        =   intval($_POST['uid']);

        $report     =   $reportM->getReportOne(array('id' => $id), array('field' => '`p_uid`, `datafh`'));
        $comid      =   intval($report['p_uid']);

        $dresume    =   $downM->getDownResumeInfo(array('eid' => $eid, 'uid' => $uid, 'comid' => $comid), array('field' => '`eid`'));

        if (!empty($dresume)) {

            $order  =   $orderM->getInfo(array('type' => 19, 'sid' => $eid, 'order_remark' => array('like', '下载简历'), 'uid' => $comid), array('field' => '`order_price`'));
            $compay =   $integralM->getInfo(array('type' => 1, 'eid' => $eid, 'pay_type' => '12', 'pay_remark' => array('like', '下载简历'), 'com_id' => $comid), array('field' => '`order_price`'));
        }
        $result     =   $resumeM->delResume($eid, array('utype' => 'admin'));

        if ($result) {

            if (!empty($order) && is_array($order) && $report['datafh'] != 1) {

                $integralM->company_invtal($comid, 2, $order['order_price'], true, '举报简历返还金额', true, 2, 'packpay', 99);
            }
            if (!empty($compay) && is_array($compay) && $report['datafh'] != 1) {

                $integralM->company_invtal($comid, 2, abs($compay['order_price']), true, '举报简历返还积分', true, 2, 'integral', 99);
            }
            if (empty($order) && empty($compay) && !empty($dresume) && $report['datafh'] != 1) {
                // 没有充值购买记录，有下载记录的，返还下载数量
                $statisM->upInfo(array('down_resume' => array('+', 1)), array('usertype' => 2, 'uid' => $comid));
            }

            $statisM->upInfo(array('resume_num' => array('-', 1)), array('usertype' => 1, 'uid' => $uid));

            if ($result){

                $return =   array('msg' => '投诉简历（ID:'.$eid.'）删除成功', 'errcode' => 0);
            }else{
                $return =   array('msg' => '投诉简历（ID:'.$eid.'）删除失败', 'errcode' => 1);
            }
            $this->render_json($return['errcode'], $return['msg']);
        }
    }
    /**
     * @desc 批量删除举报简历
     */
    function delresumeall_action()
    {
        $reportM    =   $this->MODEL('report');
        $statisM    =   $this->MODEL('statis');
        $integralM  =   $this->MODEL('integral');
        $orderM     =   $this->MODEL('companyorder');
        $downM      =   $this->MODEL('downresume');
        $resumeM    =   $this->MODEL('resume');

        if (!empty($_POST['rid'])) {
            $rids       =   $_POST['rid'];
            $reportArr  =   $reportM->getReportList(array('id' => array('in', pylode(",", $rids))));
            $reportList =   $reportArr['list'];
            $comid      =   $dreport    =   $resumeIds  =   array();
            foreach ($reportList as $key => $val) {
                $resumeIds[]    =   $val['eid'];
                $dresume=   $downM->getDownNum(array('eid' => $val['eid'], 'uid' => $val['c_uid'], 'comid' => $val['p_uid'], 'usertype' => 2));
                if ($dresume > 0 && $val['datafh']!=1) {
                    $val['fhtype']  =   1;
                    $comid[]        =   $val['p_uid'];
                    $dreport[]      =   $val;
                }
            }
            $result     =   $resumeM->delResume($resumeIds, array('utype' => 'admin'));
            if ($result && isset($comid) && !empty($comid)) {
                $order  =   $orderM->getList(array('type' => 19, 'order_remark' => array('like', '下载简历'), 'uid' => array('in', pylode(',', $comid))));
                $compay =   $integralM->getList(array('type' => 1, 'pay_type' => '12', 'pay_remark' => array('like', '简历下载'), 'com_id' => array('in', pylode(',', $comid))));
                foreach ($dreport as $key => $val) {
                    foreach ($order as $k => $v) {
                        if ($val['p_uid'] == $v['uid'] && $val['eid'] == $v['sid']) {
                            $dreport[$key]['fhtype']    =   2;//金额操作
                            $dreport[$key]['fhprice']   =   $v['order_price'];
                        }
                    }
                    foreach ($compay as $k => $v) {
                        if ($val['p_uid'] == $v['com_id'] && $val['eid'] == $v['eid']) {
                            if ($dreport[$key]['fhtype'] == 2) {
                                $dreport[$key]['fhtype']        =   4;//金额+积分操作
                                $dreport[$key]['fhprice_two']   =   abs($v['order_price']);
                            } else {
                                $dreport[$key]['fhtype']        =   3;//积分操作
                                $dreport[$key]['fhprice']       =   abs($v['order_price']);
                            }
                        }
                    }
                }

                foreach ($dreport as $key => $val) {
                    if ($val['fhtype'] == 2) {
                        $integralM->company_invtal($val['p_uid'], 2, $val['fhprice'], true, '举报简历返还金额', true, 2, 'packpay', 99);
                    } elseif ($val['fhtype'] == 3) {
                        $integralM->company_invtal($val['p_uid'], 2, $val['fhprice'], true, '举报简历返还积分', true, 2, 'integral', 99);
                    } elseif ($val['fhtype'] == 4) {
                        $integralM->company_invtal($val['p_uid'], 2, $val['fhprice'], true, '举报简历返还金额', true, 2, 'packpay', 99);
                        $integralM->company_invtal($val['p_uid'], 2, $val['fhprice_two'], true, '举报简历返还积分', true, 2, 'integral', 99);
                    } else {
                        $statisM->upInfo(array('down_resume' => array('+', 1)), array('usertype' => 2, 'uid' => $val['p_uid']));
                    }
                }
            }
            if ($result){
                $return =   array('msg' => '投诉简历（ID:'.pylode(',', $resumeIds).'）删除成功', 'errcode' => 9, 'layertype' => 1);
            }else{
                $return =   array('msg' => '投诉简历（ID:'.pylode(',', $resumeIds).'）删除失败', 'errcode' => 8, 'layertype' => 1);
            }
            $this->render_json($return['errcode']==9?0:1, $return['msg']);
        }
    }
    function del_action()
    {
        $reportM    =   $this->MODEL('report');
        if ($_POST['type'] == 'pldel') {
            $one    =   $reportM->getReportOne(array('id' => $_POST['del']), array('field' => '`eid`'));
            $report =   $reportM->getReportList(array('eid' => $one['eid']));
            $rids   =   array();
            foreach ($report['list'] as $key => $val) {
                $rids[]     =   $val['id'];
            }
            $where['id']    =   $rids;
        } else {
            $where['id']    =   $_POST['del'];
        }
        $return =   $reportM->delReport($where, array('title' => '举报'));

        $this->render_json($return['errcode']==9?0:1, $return['msg']);
    }

}

?>