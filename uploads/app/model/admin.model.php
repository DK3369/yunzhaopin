<?php

class admin_model extends model{

    /**
     * 获取管理员列表
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getList($whereData, $data = array())
    {

        $field  =   $data['field'] ? $data['field'] : '*';
        $List   =   $this->select_all('admin_user', $whereData, $field);

        if (!empty($List)){
            
            $uids   =   $gid    =   $dids   =   array();
            
            foreach ($List as $v){
                
                if (!intval($v['m_id'], $gid)) {
                    $gid[] = $v['m_id'];
                }
                if (!in_array($v['uid'], $uids)) {
                    $uids[] = $v['uid'];
                }
            
                if($v['did']>0){
                    $dids[] =   $v['did'];
                }
            }

            $group  =  $this -> select_all('admin_user_group',array('id'=>array('in',pylode(',', $gid))));
            
            $domain	=  $this -> select_all('domain',array('id'=>array('in',pylode(",",$dids))),"`id`,`title`");
            
            foreach ($List as $k => $v){
                
                $List[$k]['kh_n']       =   $v['crm_kh'] == 1 ? true : false;
                $List[$k]['status_n']   =   $v['status'] == 1 ? true : false;
                foreach ($group as $val){
                    
                    if ($v['m_id'] == $val['id']){
                        
                        $List[$k]['group_name']  =  $val['group_name'];
                    }
                }
                if(!empty($dids)){
                    foreach ($domain as $val){
                        
                        if($v['did']==$val['id']){
                            
                            $List[$k]['domain_name']    =   $val['title'];
                        }
                    }
                }
				
            }
        }
        return $List;
    }
    /**
     * 获取管理员权限
     * @param array $whereData
     * @param array $data
     */
    public function getPower($whereData=array()){
        
        $return  =  null;
        
        if (!empty($whereData)){
            
            $adminGroup  =  $this -> select_once('admin_user_group',array('id'=>$whereData['id']));
            
            $return['group_name']  =  $adminGroup['group_name'];
            $return['power']       =  unserialize($adminGroup['group_power']);
        }
        return $return;
    }


    /**
     * 添加管理员
     * @param array $addData
     * @param array $data
     * @return array
     */
    public function addAdminUser($addData=array(),$data=array()){
        //检测用户名重复性
        if (!empty($addData['username'])){
            
            $check  =  $this -> checkUsername($addData['username']);
            
            if ($check['msg']){
                
                return $check;
            }
        }
        if ($addData['password']){
            
            $addData['password']    =  $this -> makePass($addData['password']);
        }

        if (!isset($addData['did'])){
            $addData['did']         =   0;
        }

        $return['id']           =  $this -> insert_into('admin_user',$addData);
        
        if ($return['id']){
            
            $return['msg']      = yun_auto_t('管理员(id:').$return['id'].')添加成功';
            $return['errcode']  =  '9';
            
        }else{
            $return['msg']      =  yun_at('common_06357');
            $return['errcode']  =  '8';
        }
        
        return	$return;
    }

    /**
     * 获取管理员信息
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getAdminUser($whereData = array(), $data = array())
    {

        $field      =   $data['field'] ? $data['field'] : '*';

        $adminUser  =   $this->select_once('admin_user', $whereData, $field);

        return $adminUser;
    }

    /**
     * 修改管理员
     * @param array $upData
     * @param array $whereData
     * @param array $data
     * @return mixed
     */
    public function upInfo($upData=array(),$whereData=array(),$data=array()){
        
        $return['id']  =  $this -> update_once('admin_user',$upData,$whereData);
        return	$return;
    }

    /**
     * 修改管理员操作
     * @param array $upData
     * @param array $whereData
     * @param array $data
     * @return array
     */
    public function upAdminUser($upData=array(),$whereData=array(),$data=array()){
        
        $msg  =  '';
        //检测用户名重复性
        if (!empty($upData['username'])){
            
            $check  =  $this -> checkUsername($upData['username'],$whereData['uid']);
            
            if ($check['msg']){
                
                return $check;
            }
        }
        //修改密码
        if (isset($upData['password'])){
            if (!empty($upData['password'])){
                
                $return  =  array();
                
                if (isset($data['oldpass'])){
                    if(empty($data['oldpass'])){
                        
                        $return['msg']      =  yun_at('common_06358');
                        $return['errcode']  =  '8';
                        return $return;
                        
                    }elseif($data['oldpass'] == $upData['password']){
                        
                        $return['msg']      =  yun_at('common_00480');
                        $return['errcode']  =  '8';
                        return $return;
                        
                    }elseif($upData['password'] != $data['okpassword']){
                        
                        $return['msg']      =  yun_at('common_00890');
                        $return['errcode']  =  '8';
                        return $return;
                    }
                    $user    =  $this->select_once('admin_user',array('uid'=>$whereData['uid']));
                    $verify  =  $this->verifyPass($data['oldpass'], $user['password']);
                    
                    if ($verify == false){
                        
                        $return['msg']      =  yun_at('common_01235');
                        $return['errcode']  =  '8';
                        return $return;
                    }
                    if (!empty($return)){
                        
                        return $return;
                    }
                }
                
                $msg                 =  'wap_user_00371';
                
                $upData['password']  =  $this->makePass($upData['password']);
            }else{
                $return['msg']      =  yun_at('common_06359');
                $return['errcode']  =  '8';
                return $return;
            }
        }
        $return['id']  =  $this -> update_once('admin_user',$upData,$whereData);
        
        if ($return['id']){
            
            $return['msg']      =  yun_at('wap_user_00361').$msg.'(ID:'.$whereData['uid'].')修改成功';
            $return['errcode']  =  '9';
            
        }else{
            $return['msg']      =  yun_at('wap_user_00361').$msg.'(ID:'.$whereData['uid'].')修改失败';
            $return['errcode']  =  '8';
        }
        return	$return;
    }
    /**
     * 检测管理员用户名重复性
     * @param string $username
     */
    private function checkUsername($username, $uid = ''){
        
        $user    =  $this -> select_once('admin_user',array('username'=>$username));
        
        $return  =  array();
        
        if ($user && ($uid == '' || ($uid !='' && $uid != $user['uid']))){
            $return['msg']      =  yun_at('common_01138');
            $return['errcode']  =  '8';
        }
        
        return $return;
    }

    /**
     * 删除管理员
     * @param array $whereData
     * @param array $data
     * @return mixed
     */
    public function delAdminUser($whereData=array(),$data=array())
	{
		
        $return['id']  =  $this -> delete_all('admin_user',$whereData, '');
        
        if ($return['id']){
            $this->update_once('company', array('crm_uid'=>0,'crm_status'=>0), array('crm_uid'=>$whereData['uid']));
            $this->update_once('company_order', array('crm_uid'=>0), array('crm_uid'=>$whereData['uid']));

            $return['msg']      =  yun_auto_t('管理员(ID:').$whereData['uid'].')删除成功';
            $return['errcode']  =  '9';
            
        }else{
            $return['msg']      =  yun_auto_t('管理员(ID:').$whereData['uid'].')删除失败';
            $return['errcode']  =  '8';
        }
        return	$return;
    }
    /**
     * 添加管理员类型
     * @param array $addData
     * @param array $data
     */
    public function addAdminGroup($addData=array(),$data=array()){
        
        $return['id']  =  $this -> insert_into('admin_user_group',$addData);
        
        if ($return['id']){
            
            $return['msg']      =  yun_at('common_06360');
            $return['errcode']  =  '9';
            
        }else{
            $return['msg']      =  yun_at('common_06361');
            $return['errcode']  =  '8';
        }
        
        return	$return;
    }
    /**
     * 获取管理员类型信息
     * @param array $whereData
     * @param array $data
     */
    public function getAdminGroup($whereData=array(),$data=array('field'=>'*')){
        
        //处理分站
        if (empty($whereData['did'])){
            
            unset($whereData['did']);
        }
        
        $return  =  $this -> select_once('admin_user_group',$whereData,$data['field']);
        
        return	$return;
    }
    /**
     * 获取管理员类型列表
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getAdminGroupList($whereData = array(), $data = array('field' => null, 'utype' => null))
    {

        $field  =   $data['field'] ? $data['field'] : '*';
        if (empty($whereData['did'])) {

            unset($whereData['did']);
        }
        
        $List  =  $this -> select_all('admin_user_group',$whereData,$field);
        
        if ($data['utype'] == 'admin'){
            if(empty($data['uwhere'])){
                $user  =  $this -> select_all('admin_user',array('did'=>0,'isdid'=>array('=',1,'OR'),'groupby'=>'m_id'),'`m_id`,count(`uid`) as num');
			}else{
				$user  =  $this -> select_all('admin_user',$data['uwhere'],'`m_id`,count(`uid`) as num');
			}
			
            $domainList =   $this->select_all('domain', '', '`id`,`title`');
            foreach($List as $k=>$v){
                
                $List[$k]['num']  =  0;
                $List[$k]['domain_name']=   '--';
                foreach($user as $val){
                    
                    if($v['id'] == $val['m_id']){
                        
                        $List[$k]['num']  =  $val['num'];
                    }
                }
                foreach ($domainList as $dk => $dv){
                    if ($v['did'] == $dv['id']){

                        $List[$k]['domain_name']    =   $dv['title'];
                    }
                }
            }
        }
        return	$List;
    }
    /**
     * 修改管理员类型
     * @param array $addData
     * @param array $whereData
     * @param array $data
     */
    public function upAdminGroup($upData=array(),$whereData=array(),$data=array()){
        
        $return['id']  =  $this -> update_once('admin_user_group',$upData,$whereData);
        
        if ($return['id']){
            
            $return['msg']      =  yun_auto_t('管理员类型(ID:').$whereData['id'].')修改成功';
            $return['errcode']  =  '9';
            
        }else{
            $return['msg']      =  yun_auto_t('管理员类型(ID:').$whereData['id'].')修改失败';
            $return['errcode']  =  '8';
        }
        
        return	$return;
    }
    /**
     * 删除管理员类型
     * @param array $whereData
     * @param array $data
     */
    public function delAdminGroup($whereData=array(),$data=array()){
        
        if (!empty($whereData['id'])){
            $num  =  $this->select_num('admin_user',array('m_id'=>$whereData['id']));
            
            if ($num>0){
                $return['msg']      =  yun_at('common_00307');
                $return['errcode']  =  '8';
            }else{
				 
                $return['id']  =  $this -> delete_all('admin_user_group',$whereData, '');
                
                if ($return['id']){
                    $return['msg']      =  yun_auto_t('管理员类型(ID:').$whereData['id'].')删除成功';
                    $return['errcode']  =  '9';
                    
                }else{
                    $return['msg']      =  yun_auto_t('管理员类型(ID:').$whereData['id'].')删除失败';
                    $return['errcode']  =  '8';
                }
            }
        }else{
            $return['msg']      =  yun_at('common_00747');
            $return['errcode']  =  '8';
        }
        return	$return;
    }
    /**
     * 后台生成密码
     */
    public function makePass($pw){
        
        $adminpw  =  md5(md5($pw));
        
        return $adminpw;
    }
    /**
     * 后台验证密码
     */
    public function verifyPass($postPass, $userPass){
        
        $pw  =  $this->makePass($postPass);
        
        if ($pw == $userPass){
            
            return true;
            
        }else {
            
            return false;
        }
    }

    /**
     * @desc  获取管理员类型数目
     */
    function getAdmintypeNum($WhereData = array())
    {
        $Sysmsgnum = $this->select_num('admin_user_group', $WhereData);
        return $Sysmsgnum;
    }
    /**
     * @desc  获取管理员数目
     */
    function getAdminNum($WhereData = array())
    {
        $num = $this->select_num('admin_user', $WhereData);
        return $num;
    }
}
?>