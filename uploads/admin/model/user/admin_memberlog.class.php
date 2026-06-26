<?php


class admin_memberlog_controller extends adminCommon
{
    function index_action()
    {

        $logM       =   $this->MODEL('log');
        $memberM    =   $this->MODEL('userinfo');

        if ($_POST['utype']) {
            $where['usertype']  =   trim($_POST['utype']);
        } else {

            $where['usertype']  =   1;
        }
        if (isset($_POST['uid'])) {

            $where['uid']       =   intval($_POST['uid']);
        }

        $keywordStr =   trim($_POST['keyword']);

        if (!empty($keywordStr)) {
            if ($_POST['type'] == 1) {

                $member         =   $memberM->getList(array('username' => array('like', $keywordStr)), array('field' => '`uid`,`username`'));
                foreach ($member as $v) {
                    $uid[]      =   $v['uid'];
                }
                $where['uid']   =   array('in', pylode(",", $uid));

            } elseif ($_POST['type'] == 3) {

                $where['uid']   =   $keywordStr;
            }

        }

        $contentStr =   trim($_POST['content']);

        if (!empty($contentStr)) {

            $logDetailList      =   $this->obj->select_all('member_log_detail', array('detail' => array('like', $contentStr)), 'log_id');

            if (!empty($logDetailList)){

                $logIds         =   array();
                foreach ($logDetailList as $lk => $lv){

                    $logIds[]   =   $lv['log_id'];
                }

                $where['PHPYUNBTWSTART_A']  =   '';
                $where['id']        =   array('in', pylode(',', $logIds), '');
                $where['content']   =   array('like', $contentStr, 'OR');
                $where['PHPYUNBTWEND_A']    =   '';
            }else{

                $where['content']   =   array('like', $contentStr);
            }

        }

        if (!empty($_POST['end'])) {

            if ($_POST['end'] == '1') {

                $where['ctime'] =   array('>=', strtotime(date("Y-m-d 00:00:00")));
            } else {

                $where['ctime'] =   array('>=', strtotime('-' . (int)$_POST['end'] . 'day'));
            }
        }

        if (!empty($_POST['operas'])){

            $operaStr   =   intval($_POST['operas']);
            $operaSql 	= 	array(
                '1'		=>	array('name' => array('wap_user_00154')),
                '2'		=>	array('name' => array('common_01951','wap_com_00428', 'common_02021')),
                '3'		=>	array('name' => array('wap_00070')),
                '4'	 	=>	array('name' => array('common_02040')),
                '5'	 	=>	array('name' => array('wap_00379', 'common_01949', 'member_user_00242')),
                '6'		=>	array('name' => array('wap_00574', 'common_01991', 'common_01982', 'common_01971')),
                '7'		=>	array('name' => array('wap_00456')),
                '8'		=>	array('name' => array('member_user_00226')),
                '9'		=>	array('name' => array('wap_user_00220')),
                '11'    =>	array('name' => array('admin_user_00140', 'common_02035')),
                '12'    =>	array('name' => array('member_com_00093', 'common_02028', 'member_user_00234', 'member_user_00236', 'common_02034', 'admin_user_00171', 'member_user_00235')),
                '14'    =>	array('name' => array('member_com_00293', 'common_01937')),
                '15'    =>	array('name' => array('wap_00317', 'common_01954')),
                '16'	=>	array('name' => array('wap_js_00081', 'member_user_00161', 'LOGO', 'common_02012', 'default_00092', 'admin_tool_00428', 'common_01886', 'member_com_00077')),
                '17' 	=>	array('name' => array('admin_yunying_00117', 'wap_user_00008'), 'realId' => 17),
                '18'	=>	array('name' => array('common_01967', 'common_01965', 'common_02015', 'wap_user_00363')),
                '19' 	=>	array('name' => array('wap_user_00223')),
                '20'	=>	array('name' => array('admin_user_00018', 'admin_user_00020')),
                '21'	=>	array('name' => array('common_02031')),
                '22'	=>	array('name' => array('admin_tool_00428')),
                '23'	=>	array('name' => array('wap_com_00350')),
                '25' 	=>	array('name' => array('wap_com_00357', 'admin_user_company_00379')),
                '26' 	=>	array('name' => array('wap_user_00221', 'member_user_00044')),
                '29' 	=>	array('name' => array('common_02046')),
                '30' 	=>	array('name' => array('admin_user_00019')),
                '88' 	=>	array('name' => array('common_02029'))
            );
            if (array_key_exists($operaStr, $operaSql)) {

                if (count($operaSql[$operaStr]['name']) == 1){
                    $where['content']           =   array('like', $operaSql[$operaStr]['name'][0]);
                }else{

                    $where['PHPYUNBTWSTART']    =   '';
                    foreach ($operaSql[$operaStr]['name'] as $oV) {
                        $where['content'][]     =   array('like', $oV, 'OR');
                    }
                    $where['PHPYUNBTWEND']      =   '';
                }
            } elseif (!empty($operasStr)) {
                $where['opera']             =   $operaStr;
            }
        }
        if (isset($_POST['parrs']) && $_POST['parrs']) {
            $where['type']      =   intval($_POST['parrs']);
        }

        if (!empty($_POST['time'])) {
            $time = $_POST['time'];
            $time_begin = $time[0]? date('Y-m-d',strtotime(str_replace(array('T','Z'),' ',$time[0]))): date('Y-m-d', strtotime('-30 days'));
            $time_end  = $time[1] ? date('Y-m-d',strtotime(str_replace(array('T','Z'),' ',$time[1]))):date('Y-m-d');
            $where['ctime'][]   =   array('>=', strtotime($time_begin . "00:00:00"));
            $where['ctime'][]   =   array('<=', strtotime($time_end . "23:59:59"));
        }
        $pageM			=	$this  -> MODEL('page');
        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pages	=	$pageM -> adminPageList('member_log',$where,$page,array('limit' => $pageSize));
        if(!$pages['total']){
            $this->render_json(0,yun_at('wap_js_00113'),['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }

        if ($_POST['order']) {

            $where['orderby']   =   $_POST['t'].','.$_POST['order'];

        } else {

            $where['orderby']   =   array('id,desc');
        }
        $where['limit']         =   $pages['limit'];

        $List   =   $logM->getMemlogList($where, array('utype' => 'admin'));
        foreach ($List as &$value){
            $value['ctime_ymd'] = $value['ctime']?date('Y-m-d H:i:s',$value['ctime']):'';
        }
        $this->render_json(0,'',['data'=>$List,'total'=>(int)$pages['total'],'pageSizes'=>$pages['page_sizes']]);
    }

    function delLog_action()
    {

        $logM   =   $this->MODEL('log');

        if ($_POST['del'] == 'allcom') {

            $where['usertype']  =   2;
            $logM->delMemlog($where);
            $this->layer_msg('admin_01296', 9, 0, $_SERVER['HTTP_REFERER']);
        } elseif ($_POST['del'] == 'alluser') {

            $where['usertype']  =   1;
            $logM->delMemlog($where);
            $this->layer_msg('admin_01297', 9, 0, $_SERVER['HTTP_REFERER']);
        } elseif ($_POST['del'] == 'alltrain') {

            $where['usertype']  =   4;
            $logM->delMemlog($where);
            $this->layer_msg('admin_01298', 9, 0, $_SERVER['HTTP_REFERER']);
        } elseif ($_POST['del']) {

            $del    =   $_POST['del'];
            if (is_array($del)) {

                $where['id']    =   array('in', pylode(',', $del));
            } else {

                $where['id']    =   $del;
            }
            $return =   $logM->delMemlog($where);
            if ($return['errcode']==9){
                $this->render_json(0,$return['msg']);
            }else{
                $this->render_json(1,$return['msg']);
            }
        }
    }
}

?>