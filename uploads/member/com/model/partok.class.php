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
class partok_controller extends company
{

    function index_action()
    {
        $this -> public_action();
        
        $partM          =   $this -> MODEL('part');
         
        include PLUS_PATH . 'part.cache.php';
        
        $where          =   array();
        
        $urlarr         =   array();
        
        $uid            =   $this -> uid;
        
        $where['uid']   =   $uid;

        $jobM       =   $this->MODEL('job');
        $this->yunset('jobNum', $jobM->getJobNum(array_merge($where, array('status' => 0))));
        $this->yunset('partNum', $partM->getpartJobNum(array_merge($where, array('status' => 0))));
        
        $urlarr['c']    =   'partok';

        $urlarr['page'] =   '{{page}}';

        $w              =   intval($_GET['w']);

        if ($w == 4) {

            $where['status']    =   '1';

            $urlarr['w']        =   $w;
        } elseif ($_GET["w"] == 1) {

            $where['status']    =   '0';
            $where['state']     =   '1';
            $urlarr['w']        =   $_GET['w'];
        } elseif ($w >= 0) {

            $where['state']     =   $w;

            $urlarr['w']        =   $w;
        } else {

            $where['edate']     =   array('<', time());

            $urlarr['w']        =   0;
        }

        $pageurl                =   Url('member', $urlarr);

        $pageM                  =   $this -> MODEL('page');

        $pages                  =   $pageM -> pageList('partjob', $where, $pageurl, $_GET['page']);

        if ($pages['total'] > 0) {
            
            $where['orderby']   =   'lastupdate,desc';
            $where['limit']     =   $pages['limit'];
            
            $rows               =   $partM -> getList($where); 
        }
 
        $this->yunset('rows', $rows);
        $this -> yunset('i_know_part', !empty($_COOKIE['i_know_part_' . $this->uid]) ? 1 : 0);

        $this->company_satic();
        if ($w == 1) {

            $this->com_tpl('part'); // 招聘中的职位
        } else {

            $this->com_tpl('partok');
        }
    }

    function opera_action()
    {
        $this->part();
    }
}
?>