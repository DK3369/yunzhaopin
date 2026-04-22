<?php

class company_pay_controller extends adminCommon
{
    
    function index_action(){
        $OrderM = $this->MODEL('companyorder');
        $where = array();
        $keywordStr = trim($_GET['keyword']);
        if($_POST['comid']){
            $where['com_id'] = $_POST['comid'];
            $where['usertype'] = '2';
        }
        if(!empty($keywordStr)){
            if ($_POST['type']=='1') {
                $where['order_id'] = array('like', $keywordStr);
            }elseif ($_POST['type']=='3') {
                $where['pay_remark'] = array('like', $keywordStr);
            }elseif($_POST['type']=='2'){
                $UserinfoM = $this->MODEL('userinfo');
                $member = $UserinfoM->getList(array('username'=>array('like',$keywordStr)),array('field'=>'uid'));
                if (is_array($member)){
                    foreach ($member as $val){
                        $muids[] = $val['uid'];
                    }
                    $where['com_id'] = array('in', pylode(",",$muids));
                }
            }
        }
        if($_POST['pay_state']!=""){
            $where['pay_state'] = $_POST['pay_state'];
        }
        if($_POST['end']){
            if($_POST['end'] == 1){
                $where['order_time'] = array('>=',strtotime(date("Y-m-d 00:00:00")));
            }else{
                $where['order_time'] = array('>=',strtotime('-'.intval($_POST['end']).' day'));
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company_pay', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby'] = array('id,desc');
            }
            $where['limit']	= $pages['limit'];
            $rows = $OrderM->getPayList($where, array('utype' => 'admin'));
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $rt['integral_pricename'] = $this->config['integral_pricename'];
        $this->render_json(0, '', $rt);
    }

    function del_action(){
        $OrderM = $this->MODEL('companyorder');
        $delID = is_array($_POST['del']) ? $_POST['del'] : $_POST['id'];
        $return = $OrderM->delPay($delID);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }
}