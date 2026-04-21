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
class partjob_controller extends adminCommon
{
    function set_search()
    {
        $cacheM = $this->MODEL('cache');
        $cache = $cacheM->GetCache(array('part'));
        $partdata = $cache['partdata'];
        $partclass_name = $cache['partclass_name'];
        $billing_cycle = array();
        foreach ($partdata['part_billing_cycle'] as $k => $v) {
            $billing_cycle[$v] = $partclass_name[$v];
        }
        $states = array("1" => "已审核", "4" => "未审核", "3" => "未通过", "2" => "已过期", "5" => "已锁定");
        $updates = array("1" => "今天", "3" => "最近三天", "7" => "最近七天", "15" => "最近半月", "30" => "最近一个月");
        $edates = array("1" => "已到期", "3" => "最近三天", "7" => "最近七天", "15" => "最近半月", "30" => "最近一个月");
        $search_list = array();
        $search_list['state'] = array("name" => '审核状态', "value" => $states);
        $search_list['status'] = array('name' => '招聘状态', 'value' => array('1' => '已下架', '2' => '招聘中'));
        $search_list['lastupdate'] = array("name" => '更新时间', "value" => $updates);
        $search_list['edate'] = array("name" => '结束日期', "value" => $edates);
        $search_list['billing_cycle'] = array("name" => '结算周期', "value" => $billing_cycle);
        return array('search_list' => $search_list);
    }

    function index_action()
    {
        $partM = $this->MODEL('part');

        if (isset($_POST['state']) && !empty($_POST['state'])) {
            $state = intval($_POST['state']);
            if ($state == 1) {
                $where['state'] = '1';
                $where['PHPYUNBTWSTART_A'] = '';
                $where['edate'][] = array('>', time(), 'OR');
                $where['edate'][] = array('=', '', 'OR');
                $where['PHPYUNBTWEND_A'] = '';
            } elseif ($state == 2) {
                $where['PHPYUNBTWSTART_A'] = '';
                $where['edate'][] = array('<', time(), 'AND');
                $where['edate'][] = array('>', '0', 'AND');
                $where['PHPYUNBTWEND_A'] = '';
            } elseif ($_POST['state'] == "3") {
                $where['state'] = $state;
            } elseif ($_POST['state'] == "4") {
                $where['state'] = '0';
            } elseif ($_POST['state'] == '5') {
                $where['r_status'] = 2;
            }
        }
        if (isset($_POST['status']) && !empty($_POST['status'])) {
            $status = intval($_POST['status']);
            $where['status'] = $status == 2 ? 0 : $status;
        }
        if (isset($_POST['lastupdate']) && !empty($_POST['lastupdate'])) {
            if (intval($_POST['lastupdate']) == 1) {
                $where['lastupdate'] = array('>', strtotime(date('Y-m-d 00:00:00')));
            } else {
                $where['lastupdate'] = array('>', strtotime('-' . intval($_POST['lastupdate']) . ' day'));
            }
        }
        if (isset($_POST['edate']) && !empty($_POST['edate'])) {
            if (intval($_POST['edate']) == 1) {
                $where['edate'][] = array('<', time(), 'AND');
                $where['edate'][] = array('>', '0', 'AND');
            } else {
                $where['edate'][] = array('<', strtotime('+' . intval($_POST['edate']) . ' day'), 'AND');
                $where['edate'][] = array('>', time(), 'AND');
            }
        }
        if (isset($_POST['billing_cycle']) && !empty($_POST['billing_cycle'])) {
            $where['billing_cycle'] = intval($_POST['billing_cycle']);
        }
        if (isset($_POST['keyword']) && !empty($_POST['keyword'])) {
            $typeSkr = intval($_POST['type']);
            $keywordSkr = trim($_POST['keyword']);
            if ($typeSkr == 1) {
                $where['com_name'] = array('like', $keywordSkr);
            } else if ($typeSkr == 2) {
                $where['name'] = array('like', $keywordSkr);
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('partjob', $where, $page, array('limit' => $pageSize));
        if ($pages['total'] > 0) {
            if ($_POST['order']) {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby'] = array('state,asc', 'id,desc');
            }
            $where['limit'] = $pages['limit'];
            $partList = $partM->getList($where, array('utype' => 'admin'));
        }
        $rt['list'] = $partList ? $partList : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }

    /**
     *users:王旭
     *Data:2023/10/17
     *Time:11:37
     *  缓存信息
     */
    public function  getCacheData_action(){
        $cacheM = $this->MODEL('cache');
        $cache = $cacheM->GetCache(array('city', 'part'));
        $citys = array();
        foreach ($cache['city_index'] as $fir_v) {
            $citys[$fir_v] = array('value' => $fir_v, 'label' => $cache['city_name'][$fir_v]);
            if ($cache['city_type'][$fir_v]) {// 二级城市
                $citys[$fir_v]['children'] = array();
                foreach ($cache['city_type'][$fir_v] as $sec_v) {
                    $citys[$fir_v]['children'][$sec_v] = array('value' => $sec_v, 'label' => $cache['city_name'][$sec_v]);
                    if ($cache['city_type'][$sec_v]) {// 三级城市
                        $citys[$fir_v]['children'][$sec_v]['children'] = array();
                        foreach ($cache['city_type'][$sec_v] as $thi_v) {
                            $citys[$fir_v]['children'][$sec_v]['children'][$thi_v] = array('value' => $thi_v, 'label' => $cache['city_name'][$thi_v]);
                        }
                    }
                }
            }
        }
        $citys = array_values($citys);
        foreach ($citys as $k => $v) {
            if ($v['children']) {
                $citys[$k]['children'] = array_values($v['children']);
                foreach ($citys[$k]['children'] as $kk => $vv) {
                    if ($vv['children']) {
                        $citys[$k]['children'][$kk]['children'] = array_values($vv['children']);
                    }
                }
            }
        }
        $cache['citys'] = $citys;
        $data['cache'] = $cache;
        $data['mapkey'] = $this->config['map_key'];
        $data['mapurl'] = $this->config['mapurl'];
        $setsearch = $this->set_search();
        $data['search_list'] = $setsearch['search_list'];
        $this->render_json(0,'',$data);
    }
    /**
     * @desc 职位详情及修改
     */
    function show_action()
    {
        $partM = $this->MODEL('part');
        $companyM = $this->MODEL('company');
        if ($_POST['id'] && !$_POST['update']) {
            $List = $partM->getInfo(array('id' => intval($_POST['id'])), array('edit' => 1));
            $show = $List['info'];
            $show['workcishu'] = count($show['worktime_n']);
            $rt['show'] = $show;
            $uid = $show['uid'];
            $company = $companyM->getInfo($uid, array('field' => '`uid`,r_status'));
            $rt['company'] = $company;
            $rt['today'] = date("Y-m-d");
            $this->render_json(0, '', $rt);
        }
        if ($_POST['update']) {
            if ($_POST['timetype']) {
                $_POST['edate'] = "";
            } else {
                $_POST['edate'] = strtotime($_POST['edate']);
            }
            $data = $_POST;
            if ($_POST['r_status'] == 1) {
                $data['state'] = '1';
            } else {
                $data['state'] = '0';
            }
            $arr = $partM->upPartInfo($data);
            if ($arr['errcode'] == 9) {
                $this->admin_json(0, $arr['msg']);
            } else {
                $this->render_json(1, $arr['msg']);
            }
        }
    }

    /**
     * @desc 删除兼职
     */
    function del_action()
    {
        $partM = $this->MODEL('part');
        $id = $_POST['del'];
        $arr = $partM->delPart($id, array('utype' => 'admin'));
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    /**
     * @desc 兼职推荐
     */
    function recommend_action()
    {
        $id = trim($_POST['pid']);
        $data = array('rec' => intval($_POST['s']), 'days' => intval($_POST['days']));
        $partM = $this->MODEL('part');
        $arr = $partM->addRecPart($id, $data);
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    /**
     * @desc 兼职延期
     */
    function ctime_action()
    {
        $partM = $this->MODEL('part');
        $id = trim($_POST['jobid']);
        $arr = $partM->addPartTime($id, array('days' => intval($_POST['days'])));
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    /**
     * @desc 兼职职位审核
     */
    function status_action()
    {
        $partM = $this->MODEL('part');
        $statusData = array(
            'state' => intval($_POST['status']),
            'statusbody' => trim($_POST['statusbody']),
            'single' => isset($_POST['single']) ? $_POST['single'] : '',
            'lock_status' => trim($_POST['lock_status'])
        );
        $return = $partM->statusPartJob($_POST['pid'], $statusData);
        if (isset($_POST['single']) && !empty($_POST['single'])){
            if ($return['errcode'] == 9){
                if($_POST['atype'] == 1){
                    $this->admin_json(0, $return['msg']);
                }else{
                    $row = $partM->getInfo(array('state' => 0, 'orderby' => array('lastupdate,DESC')), array('field'=>'id'));
                    $this->admin_json(0, $return['msg'], $row ? $row : array());
                }
            }else{
                $this->render_json(1, $return['msg']);
            }
        }else{
            if ($return['errcode'] == 9) {
                $this->admin_json(0, $return['msg']);
            } else {
                $this->render_json(1, $return['msg']);
            }
        }
    }

    function tbStatus_action()
    {
        if ($_POST) {
            $id = intval($_POST['pid']);
            $uid = intval($_POST['uid']);
            $status = intval($_POST['status']);
            $statusbody = trim($_POST['statusbody']);
            $partM = $this->MODEL('part');
            $post = array('uid' => $uid, 'state' => $status, 'statusbody' => $statusbody);
            $return = $partM->status($id, $post);
            if (isset($_POST['single']) && !empty($_POST['single'])){
                if ($return['errcode'] == 9){
                    if($_POST['atype'] == 1){
                        $this->admin_json(0, $return['msg']);
                    }else{
                        $row   =  $partM->getInfo(array('state'=>0,'orderby'=>array('lastupdate,DESC')), array('field'=>'id'));
                        $this->admin_json(0, $return['msg'], $row ? $row : array());
                    }
                }else{
                    $this->render_json(1, $return['msg']);
                }
            }else{
                if ($return['errcode'] == 9) {
                    $this->admin_json(0, $return['msg']);
                } else {
                    $this->render_json(1, $return['msg']);
                }
            }
        }
    }

    /**
     * @desc 刷新兼职
     */
    function refresh_action()
    {
        $partM = $this->MODEL('part');
        $data['ids'] = $_POST['ids'];
        $partM->refreshPartJob($data);
        $this->admin_json(0, "兼职职位(ID" . $_POST['ids'] . ")刷新成功");
    }

    /**
     * @desc 获取兼职职位数据数目
     */
    function partNum_action()
    {
        $MsgNum = $this->MODEL('msgNum');
        $rt = json_decode($MsgNum->partNum(), 1);
        $this->render_json(0, '', $rt);
    }

    // 招聘/下架操作
    function checkstate_action()
    {
        if ($_POST['id'] && $_POST['state']) {
            if ($_POST['state'] == 2) {
                $_POST['state'] = 0;
            }
            $partM = $this->MODEL('part');
            $id = intval($_POST['id']);
            $postData['status'] = intval($_POST['state']);
            $partM->upInfo($postData, array('id' => $id));
        }
        $this->render_json(0, '');
    }

    function partAudit_action()
    {
        $partM = $this->MODEL('part');
        $partId = intval($_POST['id']);
        $part = $partM->getInfo(array('id' => $partId), array('edit' => 1));
        $Info = $part['info'];
        $memberInfo = $this->obj->select_once('member', array('uid' => $Info['uid']), '`status`,`lock_info`');
        $Info['c_status'] = $memberInfo['status'];
        $Info['lock_info'] = trim($memberInfo['lock_info']);
        $rt['info'] = $Info;
        $snum = $partM->getpartJobNum(array('state' => 0, 'id' => array('<>', $partId)));
        $rt['snum'] = $snum;
        $this->render_json(0, '', $rt);
    }
}