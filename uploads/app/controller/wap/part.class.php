<?php



class part_controller extends common
{

    function index_action()
    {

        $this->toLoginPage();

        $this->get_moblie();

        if ($this->config['sy_part_web'] == '2') {

            $this->ACT_msg_wap('index.php', yun_at('wap_00365'), 1, 3);
        }

        $CacheM     =   $this->MODEL('cache');
        $CacheArr   =   $CacheM->GetCache(array('part', 'city'));
        $this->yunset($CacheArr);

        $searchurl  =   array();
        $searchUrlObj = array();

        foreach ($_GET as $k => $v) {
            if ($k != '') {
                $searchurl[]    =   $k.'='.$v;
                $searchUrlObj[$k]    = $v;
            }
        }
        $searchurl  =   @implode('&', $searchurl);
        $this->yunset('searchurl', $searchurl);
        $this->yunset('searchUrlObj',yun_json_encode($searchUrlObj));

        $this->yunset('backurl', Url('wap'));
        $this->seo('part_index');
        $this->yunset('topplaceholder', yun_at('admin_system_00198'));
        $this->yunset('headertitle', yun_at('wap_user_00220'));
        $this->yuntpl(array('wap/part'));
    }

    function show_action()
    {

        if ($this->config['sy_part_web'] == '2') {
            $this->ACT_msg_wap('index.php', yun_at('wap_00365'), 1, 3);
        }

        $this->get_moblie();

        if (!empty($_GET['id'])) {

            $id     =   (int)$_GET['id'];
            $partM  =   $this->MODEL('part');
            $info   =   $partM->getInfo(array('id' => $id), array('cache' => 1, 'com' => 1, 'uid' => $this->uid, 'usertype' => $this->usertype));
            $job    =   $info['info'];

            $this->yunset($info['cache']);

            if ($job['id']) {

                $job['com_name']    =   $info['com']['name'];
                $partM->upInfo(array('hits' => array('+', 1)), array('id' => $id)); // bump hits

                if ($this->usertype == 1) {

                    $apply          =   $partM->getPartSqInfo(array('uid' => $this->uid, 'jobid' => $id));
                    $this->yunset('apply', $apply);

                    $collect        =   $partM->getPartCollectInfo(array('uid' => $this->uid, 'jobid' => $id));
                    $this->yunset('collect', $collect);
                }

                $this->yunset('job', $job);
            } else {

                $this->ACT_msg_wap('index.php', yun_at('wap_00366'), 1, 3);
            }
        }

        $data['part_name']  =   $job['name'];
        $this->data         =   $data;

        $this->seo('part_show');
        $this->yunset('headertitle', yun_at('wap_user_00220'));
        $this->yuntpl(array('wap/part_show'));
    }

    /**
 */
    function collect_action()
    {

        $partM  =   $this->MODEL('part');

        $data   =   array(
            'uid'       =>  $this->uid,
            'usertype'  =>  $this->usertype,
            'jobid'     =>  (int)$_POST['jobid'],
            'comid'     =>  (int)$_POST['comid']
        );
        $return =   $partM->addPartCollect($data);
        echo yun_json_encode($return);
        die();
    }

    /**
 */
    function apply_action()
    {

        $partM  =   $this->MODEL('part');

        $data   =   array(
            'uid'       =>  $this->uid,
            'usertype'  =>  $this->usertype,
            'jobid'     =>  (int)$_POST['jobid'],
            'port'      =>  '2'
        );
        $return =   $partM->addPartSq($data);
        echo yun_json_encode($return);
        die();
    }

    
    function telQrcode_action()
    {

        $WxM    =   $this->MODEL('weixin');
        $qrcode =   $WxM->pubWxQrcode('parttel', $_GET['id']);
        if (isset($qrcode)) {

            $imgStr = CurlGet($qrcode);

            header("Content-Type:image/png");

            echo $imgStr;
        }
    }
}

?>