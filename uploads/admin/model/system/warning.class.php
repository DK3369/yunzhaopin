<?php
/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class warning_controller extends adminCommon
{

    function index_action()
    {
        
        $ConfigM    =   $this->MODEL('config');

        if (isset($_POST['status']) && intval($_POST['status'])) {
            $where['status'] = intval($_POST['status']);
        }
        if ($_POST['date1'] && $_POST['date2']) {

            $where['ctime'][]   =   array('>=', $_POST['date1']);
            $where['ctime'][]   =   array('<=', $_POST['date2']+86399);
        }

        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('warning', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            $where['orderby']   =   'id';
            $where['limit']     =   $pages['limit'];
            $list               =   $ConfigM->getWarningList($where, array('utype' => 'admin'));
        }

        $data['list']           =   !empty($list)?$list:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['limit'][1];
        $this->render_json(0, '', $data);
    }
    function getWarningConfig_action(){

        $data = array(
            'warning_addjob'    =>  $this->config['warning_addjob'],
            'warning_addjob_type'   =>  $this->config['warning_addjob_type'],
            'warning_downresume'   =>  $this->config['warning_downresume'],
            'warning_downresume_type'   =>  $this->config['warning_downresume_type'],
            'warning_addresume'   =>  $this->config['warning_addresume'],
            'warning_addresume_type'   =>  $this->config['warning_addresume_type'],
            'warning_recharge'   =>  $this->config['warning_recharge'],
            'warning_recharge_type'   =>  $this->config['warning_recharge_type'],
            'sy_hour_msgnum'   =>  $this->config['sy_hour_msgnum'],
            'warning_closemsg_type'   =>  $this->config['warning_closemsg_type'],
            'warning_lookresume'   =>  $this->config['warning_lookresume'],
            'warning_lookresume_type'   =>  $this->config['warning_lookresume_type'],
            'warning_lookjob'   =>  $this->config['warning_lookjob'],
            'warning_lookjob_type'   =>  $this->config['warning_lookjob_type'],
            'warning_teljob'   =>  $this->config['warning_teljob'],
            'warning_teljob_type'   =>  $this->config['warning_teljob_type'],
            'warning_reg_ip'   =>  $this->config['warning_reg_ip'],
            'warning_reg_ip_type'   =>  $this->config['warning_reg_ip_type'],
            
            'warning_exchange_link'   =>  $this->config['warning_exchange_link'],
            'warning_exchange_link_type'   =>  $this->config['warning_exchange_link_type'],
            'warning_sendresume'   =>  $this->config['warning_sendresume'],
            'warning_sendresume_type'   =>  $this->config['warning_sendresume_type'],
            'warning_sendresume_tips'   =>  $this->config['warning_sendresume_tips'],
            'warning_sqjob'   =>  $this->config['warning_sqjob'],
            'warning_sqjob_type'   =>  $this->config['warning_sqjob_type'],
            'warning_sqjob_tips'   =>  $this->config['warning_sqjob_tips'],
        );
        $this->render_json(0, '', $data);
    }
    /**
     * 预警配置
     */
    function config_action()
    {
        $ConfigM = $this->MODEL('config');

        if ($_POST) {

            foreach ($_POST as $key => $v) {

                $config = $ConfigM->getNum(array('name' => $key));

                if ($config == false) {

                    $ConfigM->addInfo(array('name' => $key, 'config' => $v));
                } else {

                    $ConfigM->upInfo(array('name' => $key), array('config' => $v));
                }
            }

            $this->web_config();

        }

        $this->admin_json(0, '预警配置设置成功');
    }

    function del_action()
    {
        $ConfigM = $this->MODEL('config');
        if ($_POST['del']) {
            $return = $ConfigM->delWarning($_POST['del']);
            $this->admin_json($return['errcode'],$return['msg']);
        }
    }
}

?>