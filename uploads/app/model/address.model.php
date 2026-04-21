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

class address_model extends model
{

    /**
     * @desc   获取缓存数据
     * @param $options
     * @return array|string|void
     */
    private function getClass($options)
    {

        if (!empty($options)) {

            include_once('cache.model.php');
            $cacheM = new cache_model($this->db, $this->def);
            $cache  = $cacheM->GetCache($options);
            return $cache;
        }
    }

    /**
     * @param array $data
     * @return array
     */
    public function getCityStr($data = array())
    {

        $return = array();
        if (!empty($data)) {

            $cache = $this->getClass(array('city'));

            $cityStr = $cache['city_name'][$data['provinceid']] . $cache['city_name'][$data['cityid']] . $cache['city_name'][$data['three_cityid']];

            $return['cityStr'] = $cityStr;
        }

        return $return;
    }


    /**
     * @desc    工作地址
     * @param array $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getAddressList($where = array(), $data = array())
    {

        $field  =   isset($data['field']) ? $data['field'] : '*';

        $List   =   $this->select_all('company_job_link', $where, $field);

        if (!empty($List)){

            $cache  =   $this->getClass(array('city'));
            foreach ($List as $k => $v) {

                if ($v['three_cityid']){

                    $List[$k]['city']   =   $cache['city_name'][$v['provinceid']].$cache['city_name'][$v['cityid']].$cache['city_name'][$v['three_cityid']];
                }else{

                    $List[$k]['city']   =   $cache['city_name'][$v['provinceid']].$cache['city_name'][$v['cityid']];
                }

                $phone_n                =   !empty($v['link_moblie']) ? $v['link_moblie'] : $v['link_phone'];

                $List[$k]['linkmsg']    =   $v['link_man'] . ' - ' . $phone_n . ' - ' . $List[$k]['city'] . ' - ' . $v['link_address'];

                if ($v['mappic']){

                    $List[$k]['staticimg'] =   checkpic($v['mappic']);
                }
            }
        }

        return  $List;
    }

    /**
     * @param array $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getAddressInfo($where = array(), $data = array())
    {

        $field  =   isset($data['field']) ? $data['field'] : '*';

        $Info   =   $this->select_once('company_job_link', $where, $field);

        $cache  =  $this->getClass(array('city'));

        if ($Info['provinceid']){

            $Info['city_one']   =   $cache['city_name'][$Info['provinceid']];
        }
        if ($Info['cityid']){

            $Info['city_two']   =   $cache['city_name'][$Info['cityid']];
        }
        if ($Info['three_cityid']){

            $Info['city_three'] =   $cache['city_name'][$Info['three_cityid']];
        }

        if ($Info['mappic']){

            $Info['staticimg'] =   checkpic($Info['mappic']);
        }

        return  $Info;
    }

    /**
     * @param array $where
     * @return array|bool|false|string|void
     */
    public function getAddressNum($where = array())
    {

        $addressNum =   $this->select_num('company_job_link', $where);

        return $addressNum;
    }

    /**
     * @param array $data
     * @return bool
     */
    public function saveAddress($data = array(), $eData = array())
    {
        if (!empty($data)) {

            $mdata['lng']       =   $data['x'];
            $mdata['lat']       =   $data['y'];
            $mdata['copyright'] =   1;
            $mdata['width']     =   320;
            $mdata['height']    =   140;
            $mdata['zoom']      =   14;
            $mdata['refer']     =   $this->config['sy_weburl'];

            $return =   curlMappic($mdata);
            if ($return['error'] == 0) {
                $data['mappic'] =   $return['url'];
            }
            if (!isset($eData['utype']) && isset($data['uid'])){

                $logContent =   '基本信息：新增工作地址';
                $logDetail  =   '详细信息：';
                $logDetail  .=  '联系人：'.$data['link_man'];
                if (isset($data['link_moblie']) && !empty($data['link_moblie'])) {
                    $logDetail  .=  '，手机号码：'.$data['link_moblie'];
                }
                if (isset($data['link_phone']) && !empty($data['link_phone'])) {
                    $logDetail  .=  '，固定电话：'.$data['link_phone'];
                }
                if (isset($data['email']) && !empty($data['email'])) {
                    $logDetail  .=  '，电子邮箱：'.$data['email'];
                }
                if (isset($data['link_address']) && !empty($data['link_address'])) {
                    $logDetail  .=  '，工作地址：'.$data['link_address'];
                }
                $this->addMemberLog($data['uid'], 2, $logContent, 7, 1, $logDetail);
            }
            $result = $this->insert_into('company_job_link', $data);
        }

        return $result;
    }

    /**
     * @param array $data
     * @param array $where
     * @return bool
     */
    public function upAddress($data = array(), $where = array())
    {
        if (!empty($data) && !empty($where)){

            if(intval($where['id'])){

                $addressInfo = $this->getAddressInfo(array('id'=>intval($where['id'])));

                if($addressInfo['x']!=$data['x'] || $addressInfo['y']!=$data['y'] || empty($addressInfo['mappic'])){

                    $mdata['lng']      =   $data['x'];
                    $mdata['lat']      =   $data['y'];
                    $mdata['copyright'] =   1;
                    $mdata['width']     =   320;
                    $mdata['height']    =   140;
                    $mdata['zoom']      =   14;
                    $mdata['refer']    =   $this->config['sy_weburl'];

                    $return = curlMappic($mdata);

                    if($return['error']==0){

                        $data['mappic'] = $return['url'];
                        
                    }
                }

                $logContent =   '基本信息：更新工作地址；';
                $logDetail  =   '详细信息：';

                if (isset($data['link_man']) && !empty($data['link_man']) && $data['link_man'] != $addressInfo['link_man']) {
                    $logDetail  .=  '联系人：'.$addressInfo['link_man'].' → '.$data['link_man'];
                }
                if (isset($data['link_moblie']) && !empty($data['link_moblie'])&& $data['link_moblie'] != $addressInfo['link_moblie']) {
                    $logDetail  .= '，手机号码：'.$addressInfo['link_moblie'].' → '.$data['link_moblie'];
                }
                if (isset($data['link_phone']) && !empty($data['link_phone'])&& $data['link_phone'] != $addressInfo['link_phone']) {
                    $logDetail  .= '，固定电话：'.$addressInfo['link_phone'].' → '.$data['link_phone'];
                }
                if (isset($data['email']) && !empty($data['email'])&& $data['email'] != $addressInfo['email']) {
                    $logDetail  .= '，电子邮箱：'.$addressInfo['email'].' → '.$data['email'];
                }
                if (isset($data['link_address']) && !empty($data['link_address'])&& $data['link_address'] != $addressInfo['link_address']) {
                    $logDetail  .= '，工作地址：'.$addressInfo['link_address'].' → '.$data['link_address'];
                }
                if ($logDetail != '详细信息：'){

                    $this->addMemberLog($data['uid'], 2, $logContent, 7, 2, $logDetail);
                }else{

                    $this->addMemberLog($data['uid'], 2, $logContent, 7, 2);
                }
            }

            $result =   $this->update_once('company_job_link', $data, $where);
            $linklist = $this->getAddressList($where);

            foreach($linklist as $k=>$v){
                $this->update_once('company_job',array('x'=>$v['x'],'y'=>$v['y'], 'provinceid' => $v['provinceid'], 'cityid' => $v['cityid'], 'three_cityid' => $v['three_cityid']),array('link_id'=>$v['id']));
            }
        }

        return  $result;
    }

    /**
     * @param array $where | uid,id
     * @return array
     */
    public function delAddress($where = array())
    {

        $return     =   array('errcode' => 8, 'msg' => '系统错误');

        if (!empty($where)){

            $result =   $this->delete_all('company_job_link', $where, '');

            if ($result){

                $this->update_once('company_job', array('is_link' => 1), array('link_id' => $where['id'], 'uid' => $where['uid'], 'is_link' => 2));
                $this->update_once('company_job', array('link_id' => ''), array('link_id' => $where['id'], 'uid' => $where['uid']));

                if (is_array($where['id'])){

                    $linkId =   $where['id'][1];
                }else{

                    $linkId =   $where['id'];
                }

                $logContent =   '职位更新：删除职位联系方式';
                $logDetail  =   '删除职位联系方式（ID：'.$linkId.'）';
                $this->addMemberLog($where['uid'], 2, $logContent, 1, 3, $logDetail);
            }

            $return =   array(

                'errcode'   =>  $result ? 9 : 8,
                'msg'       =>  $result ? '工作地址删除成功' : '工作地址删除失败'
            );
        }

        return  $return;
    }

    /**
     * @desc   引用log类，添加用户日志
     * @param $uid
     * @param $usertype
     * @param $content
     * @param string $opera
     * @param string $type
     * @param string $detail
     * @return void
     */
    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {
        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }

}
?>