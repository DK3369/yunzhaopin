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

class address_controller extends company
{

    /**
     * @desc 工作地址管理
     * @time 2022年3月5日
     */
    public function index_action()
    {

        $this->public_action();
        $this->company_satic();

        $addressM   =   $this->MODEL('address');

        $where      =   array('uid' => $this->uid);

        $urlarr     =   array('c' => 'address', 'page' => '{{page}}');

        $pageurl    =   Url('member', $urlarr);

        $pageM      =   $this->MODEL('page');

        $pages      =   $pageM->pageList('company_job_link', $where, $pageurl, $_GET['page']);

        if ($pages['total'] > 0) {

            $where['orderby']   =   'id';
            $where['limit']     =   $pages['limit'];
            $rows               =   $addressM->getAddressList($where);
        }

        $this->yunset('rows', $rows);

        $defLink    =   array(

            'link_man'      =>  $this->comInfo['linkman'],
            'link_moblie'   =>  $this->comInfo['linktel'],
            'link_phone'    =>  $this->comInfo['linkphone'],
            'email'         =>  $this->comInfo['linkmail'],
            'city'          =>  $this->comInfo['job_city_one'].$this->comInfo['job_city_two'].$this->comInfo['job_city_three'],

            'provinceid'    =>  $this->comInfo['provinceid'],
            'cityid'        =>  $this->comInfo['cityid'],
            'three_cityid'  =>  $this->comInfo['three_cityid'],
            'address'       =>  $this->comInfo['address'],
            'x'             =>  $this->comInfo['x'],
            'y'             =>  $this->comInfo['y']
        );

        $this->yunset('defLink', $defLink);

        $this->yunset($this->MODEL('cache')->GetCache(array('city')));
        $this->com_tpl('address');
    }

    public function saveAddress_action()
    {

        $addressM   =   $this->MODEL('address');

        $linkData   =   array(

            'link_man'      =>  $_POST['link_man'],
            'link_moblie'   =>  $_POST['link_moblie'],
            'link_phone'    =>  $_POST['link_phone'],
            'email'         =>  $_POST['email'],
            'link_address'  =>  $_POST['link_address'],
            'provinceid'    =>  $_POST['provinceid'],
            'cityid'        =>  $_POST['cityid'],
            'three_cityid'  =>  $_POST['three_cityid'],
            'x'             =>  $_POST['x'],
            'y'             =>  $_POST['y']
        );
		$link_id = (int)$_POST['id'];
        if ((int)$_POST['id'] > 0){

            $result         =   $addressM->upAddress($linkData, array('id' => $_POST['id'], 'uid' => $this->uid));
            $msg            =   $result ? '工作地址更新成功' : '工作地址更新失败';
        } else if ($_POST['id'] == -1) {
            $comM = $this->MODEL('company');
            $upLink = array(
                'provinceid' => $_POST['provinceid'],
                'cityid' => $_POST['cityid'],
                'three_cityid' => $_POST['three_cityid'],
                'x' => $_POST['x'],
                'y' => $_POST['y'],
                'address' => $_POST['link_address'],
                'linkmail' => $_POST['email'],
                'linkphone' => $_POST['link_phone'],
                'linktel' => $_POST['link_moblie'],
                'linkman' => $_POST['link_man'],
            );

            $cominfo =  $comM->getCompanyInfo(array('uid' => $this->uid));
            
            if($cominfo['x']!=$_POST['x'] || $cominfo['y']!=$_POST['y'] || empty($cominfo['mappic'])){

                $mdata['lng']      =   $_POST['x'];
                $mdata['lat']      =   $_POST['y'];
                $mdata['copyright'] =   1;
                $mdata['width']     =   320;
                $mdata['height']    =   140;
                $mdata['zoom']      =   14;
                $mdata['refer']    =   $this->config['sy_weburl'];

                $mapReturn = curlMappic($mdata);

                if($mapReturn['error']==0){

                    $upLink['mappic'] = $mapReturn['url'];
                    
                }
            }
            
            $result = $comM->upInfo('', array('uid' => $this->uid), $upLink);
            $msg = $result ? '工作地址更新成功' : '工作地址更新失败';
        } else {

            $linkData['uid']=   $this->uid;
            $result         =   $addressM->saveAddress($linkData, array('uid' => $this->uid));
            $msg            =   $result ? '工作地址添加成功' : '工作地址添加失败';
            $link_id        =   $result;
        }

        $errcode            =   $result ? 9 : 8;

        if ($result && (int)$_POST['is_link'] == 2){

            $addressList    =   $addressM->getAddressList(array('uid' => $this->uid));
            $this->comInfo  =   $this->MODEL('company')->getInfo($this->uid, array('info' => '1', 'edit' => '1', 'logo' => '1', 'utype' => 'user'));
            $defLink        =   array(

                'link_man'      =>  $this->comInfo['linkman'],
                'link_moblie'   =>  $this->comInfo['linktel'],
                'link_phone'    =>  $this->comInfo['linkphone'],
                'email'         =>  $this->comInfo['linkmail'],
                'city'          =>  $this->comInfo['job_city_one'].$this->comInfo['job_city_two'].$this->comInfo['job_city_three'],

                'provinceid'    =>  $this->comInfo['provinceid'],
                'cityid'        =>  $this->comInfo['cityid'],
                'three_cityid'  =>  $this->comInfo['three_cityid'],
                'address'       =>  $this->comInfo['address'],
                'x'             =>  $this->comInfo['x'],
                'y'             =>  $this->comInfo['y']
            );


            echo json_encode(array('errcode' => $errcode, 'msg' => $msg, 'defLink' => $defLink, 'addressList' => $addressList, 'link_id' => $link_id));
            exit();
        }else{

            echo json_encode(array('errcode' => $errcode, 'msg' => $msg));
            exit();
        }
    }

    public function delAddress_action()
    {

        $_POST      =   $this->post_trim($_POST);

        $addressM   =   $this->MODEL('address');

        $res        =   $addressM->delAddress(array('uid' => $this->uid, 'id' => $_POST['id']));

        echo json_encode($res);

        die();
    }

    public function delAllAddress_action()
    {

        $_POST      =   $this->post_trim($_POST);
        $addressM   =   $this->MODEL('address');

        $result     =   $addressM->delAddress(array('uid' => $this->uid, 'id' => array('in', pylode(',', $_POST['delid']))));
        if ($result) {

            $this->layer_msg('删除成功！', 9, 1, $_SERVER['HTTP_REFERER']);
        } else {

            $this->layer_msg('删除失败！', 8, 0, $_SERVER['HTTP_REFERER']);
        }
    }
}

?>