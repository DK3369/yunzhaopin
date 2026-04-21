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
class job_controller extends company
{

    function index_action()
    {

        $this -> public_action();
        $this -> company_satic();

        $urlarr     =   array('c' => 'job', 'page' => '{{page}}');

        $uid        =   $this->uid;
		
        $where['uid']	=   $uid;

        $jobM       =   $this->MODEL('job');
        $this->yunset('jobNum', $jobM->getJobNum(array_merge($where, array('status' => 0))));

        $partM          =   $this -> MODEL('part');
        $this->yunset('partNum', $partM->getpartJobNum(array_merge($where, array('status' => 0, 'state' => 1))));
        
        // 关键字刷选
        if ($_GET['keyword']) {
            
            $where['name']      =   array('like', trim($_GET['keyword']));
            $urlarr['keyword']  =   $_GET['keyword'];
        }
        // 职位状态 0:待审核职位 1:招聘中的职位 3:未通过职位 4:已下架职位 5:所有职位
        if($_GET['type']==2){
            $urlarr['type']        =   $_GET['type'];
        }
		$_GET['w']=isset($_GET['w']) ? $_GET['w'] : 1;
        if ($_GET['w'] == 4) {
            
            $where['status']    =   '1';
            $urlarr['w']        =   $_GET['w'];
        } elseif ($_GET["w"] == 1) {

            $where['status']    =   '0';
            $where['state']     =   '1';
            $urlarr['w']        =   $_GET['w'];
        } elseif ($_GET["w"] == 5) {

            $urlarr['w']        =   $_GET['w'];
        } else {
            $_GET['w']          =   isset($_GET['w']) ? $_GET['w'] : 1;
            $where['state']     =   $_GET['w'];
            $urlarr['w']        =   $_GET['w'];
        }
        
        $pageurl    =   Url('member', $urlarr);

        $pageM      =   $this->MODEL('page');
        $pages      =   $pageM->pageList('company_job', $where, $pageurl, $_GET['page'], $this->config['sy_listnum'], '', true);
        if ($pages['total'] > 0) {
            if ($_GET['order']) {
                
                $where['orderby']   =   $_GET['t'].','.$_GET['order'];
                $urlarr['order']    =   $_GET['order'];
                $urlarr['t']        =   $_GET['t'];
            } else {
                
                $where['orderby']   =   array('lastupdate,desc','id,desc');
            }
            
            $where['limit']         =   $pages['limit'];
            $List                   =   $jobM -> getList($where, array('sqjobnum' => 'yes', 'reserve' => 1));
			
            $this->yunset('rows', $List['list']);
        }

        // 统计各状态职位数量
        // 职位状态 0:待审核职位 1:招聘中的职位 3:未通过职位 4:已下架职位 5:所有职位
		
		$swhere['com_id']	=	$uid;
        $audit  =   $jobM->getList($swhere, array('field' => "`status`,`state`"));
    
        $w0 = $w1 = $w2 = $w3 = $w4 = $w5 = 0;

        if (is_array($audit['list'])) {
            foreach ($audit['list'] as $value) {

                if ($value['state'] == '0') {
                    $w0 += 1;
                }
                if ($value['status'] == '0' && $value['state'] == '1') {
                    $w1 += 1;
                }

                if ($value['state'] == '3') {
                    $w3 += 1;
                }
                if ($value['status'] == '1') {
                    $w4 += 1;
                }
                $w5 += 1;
            }
        }
        
        $this->yunset(array(
            'w0' => $w0,
            'w1' => $w1,
            'w3' => $w3,
            'w4' => $w4,
            'w5' => $w5
        ));
        $this -> yunset('audit', $w0);

        $this -> yunset('i_know_job', !empty($_COOKIE['i_know_job_' . $this->uid]) ? 1 : 0);

        if (intval($_GET['w']) == 1) {

            $WhbM       =   $this->MODEL('whb');
            $syComHb    =   $WhbM->getWhbList(array('type' => 1, 'isopen' => 1));
            $this->yunset('hbNum', count($syComHb));
            $this->yunset('comHb', $syComHb);
            $this->yunset('hb_uid', $this->uid);

            $type       =   $_GET['type'];
            $this->yunset('type', $type);
            $this->com_tpl('joblist');
        } else {

            $this->com_tpl('job');
        }
    }

    function opera_action()
    {
        $this->job();
    }

    // 刷新职位方法，套餐处理集合到comtc.model.php类里面
    function refresh_job_action()
    {
        if ($_POST) {

            $comtcM	            = 	$this->MODEL('comtc');
			
			$_POST['uid']		=	$this->uid;
			$_POST['usertype']	=	$this->usertype;
			$_POST['port']      =   1;
			
            $return	= 	$comtcM->refresh_job($_POST);

            if ($return['status'] == 1) {// 职位刷新成功
                
                echo json_encode(array(
                    'error' => 1,
                    'msg'   => $return['msg']
                ));
            } else if ($return['status'] == 2) { 
                
                echo json_encode(array(
                    'error'     =>  2,
                    'pro'       =>  $return['pro'],
                    'online'    =>  $return['online'],
                    'integral'  =>  $return['integral'],
                    'jifen'     =>  $return['jifen'],
                    'price'     =>  $return['price']
                ));
            } else {// 职位刷新失败
                
                echo json_encode(array(
                    'error' => 3,
                    'msg'   => $return['msg'],
                    'url'   => $return['url']
                ));
            }
        } else {
            echo json_encode(array(
                'error' => 3,
                'msg' => '参数错误，请重试！'
            ));
        }
    }

    /**
     * 点击知道了，缓存1天
     */
    function i_know_action()
    {
        $opt = $_POST['opt'];
        $time = time();
        $this->cookie->setCookie('i_know_' . $opt . '_' . $this->uid, 1,$time + strtotime(date('Y-m-d 23:59:59')) - $time);
    }

    /**
     * 职位推广套餐 查询
     */
    function jobPromote_action()
    {
        $uid    =   $this->uid;
        $type   =   intval($_POST['type']);
        //预留职位自动刷新
        if ($type != 5){
            
            $jobM   =   $this->MODEL('job');
            $return =   $jobM->jobPromote($uid, array('type' => $type));
        }else{

            if($this->config['job_auto'] == 0){

                $return = array('status' => 1);
            }else{

                $return = array('status'=>2);
            }
        }

        echo json_encode($return);
        die();
    }

    function setJobPromote_action()
    {
        if ($_POST) {

            $jobM   =   $this->MODEL('job');

            $_POST['uid']       =   $this->uid;
            $_POST['username']  =   $this->username;
            $_POST['usertype']  =   $this->usertype;

            $return = $jobM->setJobPromote($_POST['jobid'], $_POST);

            echo json_encode($return);
            die();
        }
    }

    function closeJobPromote_action()
    {
        if ($_POST) {

            $jobM   =   $this->MODEL('job');

            $_POST['uid']       =   $this->uid;
            $_POST['username']  =   $this->username;
            $_POST['usertype']  =   $this->usertype;

            $return = $jobM->closeJobPromote($_POST['jobid'], $_POST);

            echo json_encode($return);
            die();
        }
    }

    /**
     * 预约刷新
     */
    function reserveUp_action()
    {

        if ($_POST) {

            $jobM   =   $this->MODEL('job');

            $data   =   array(

                'job_id'    =>  $_POST['job_id'],
                'end_time'  =>  strtotime($_POST['end_time']),
                'interval'  =>  $_POST['interval'],
                'status'    =>  $_POST['status'],
                's_time'    =>  $_POST['s_time'],
                'e_time'    =>  $_POST['e_time']
            );
            $return =   $jobM->reserveUpJob($data, array('uid' => $this->uid));

            echo json_encode($return);
            die;
        } else {

            echo json_encode(array('error' => 0, 'msg' => '参数错误'));
            die;
        }
    }

    /**
     * @return void
     */
    function getRefresh_action()
    {

        if ($_POST) {

            $job_id = $_POST['job_id'];
            $jobM = $this->MODEL('job');

            $where['uid']   =   $this->uid;
            $where['job_id']   =   $job_id;
            $reserve = $jobM->reserveInfo($where);
            if (!$reserve) {
                $return =   array(
                    'error' =>  0,
                    'msg'   =>  '',
                    'data'  => array('refreshStatus' => 0)
                );
                echo json_encode($return);
                die();
            }
            $return =   array(
                'error' =>  0,
                'msg'   =>  '',
                'data'  => array('refreshStatus' => 1,'s_time'=>$reserve['s_time'],'e_time'=>$reserve['e_time'],'interval'=>$reserve['interval'])
            );
            echo json_encode($return);
            die();
        }
    }

}
?>