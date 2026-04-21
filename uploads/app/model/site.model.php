<?php

/**
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class site_model extends model
{
    /**
     * 5.0使用此方法，修改分站的id
     * @param array $tableData 修改的表名数组
     * @param array $whereData 修改条件数据
     * @param array $upData 修改的数据
     * @return bool
     */
    public function updDid($tableData = array(), $whereData = array(), $upData = array())
    {
		if(empty($tableData)){
			return false;
		}
		foreach($tableData as $tableV){		
			$this->update_once($tableV, $upData, $whereData);
		}
		return true;
	}

    /**
     * @desc 获取缓存数据
     *
     * @param $options
     * @return array|string|void
     */
    private function getClass($options)
    {
        if (!empty($options)) {
            include_once('cache.model.php');
            $cacheM = new cache_model($this->db, $this->def);
            $cache = $cacheM->GetCache($options);
            return $cache;
        }
    }

    /**
	 * 获取分站列表
     * @param $whereData    查询条件
     * @param array $data 自定义处理数组
     * @return array|bool|false|string|void
     */
    function getList($whereData, $data = array())
    {

        $field = $data['field'] ? $data['field'] : '*';
        $List = $this->select_all('domain', $whereData, $field);

        if (!empty($List)){

            $options= array('hy', 'city');
            $cache  = $this->getClass($options);

            foreach ($List as $k => $v){

                $List[$k]['name']   =   (int)$v['mode'] == 2 ? $v['indexdir'] : $v['domain'];
                $List[$k]['hy_n']   =   '--';
                $List[$k]['city']   =   '--';

                $List[$k]['typeStatus'] =   $v['type'] == 1 ? true : false;

                if ($v['fz_type'] == 1){
                    if (!empty($v['three_cityid'])){

                        $List[$k]['city']   =   $cache['city_name'][$v['cityid']].' - '.$cache['city_name'][$v['three_cityid']];
                    }else if (!empty($v['cityid'])){

                        $List[$k]['city']   =   $cache['city_name'][$v['province']].' - '.$cache['city_name'][$v['cityid']];
                    }else if (!empty($v['province'])){

                        $List[$k]['city']   =   $cache['city_name'][$v['province']];
                    }
                } elseif ($v['fz_type'] == 2){

                    $List[$k]['hy_n']   =   $cache['industry_name'][$v['hy']];
                }
            }
        }
        return $List;
    }

    /**
	* 获取分站详情
     * @param $whereData    查询条件
     * @param array $data 自定义处理数组
     * @return array|bool|false|string|void
     */
    function getInfo($whereData, $data = array())
    {

        if ($whereData) {

            $data['field'] = empty($data['field']) ? '*' : $data['field'];

            $domainInfo = $this->select_once('domain', $whereData, $data['field']);

            if ($domainInfo) {

                $domainInfo['weblogo'] = checkpic($domainInfo['weblogo']);
            }
        }
        return $domainInfo;
    }

    /**
     * 创建分站
     * @param array $setData 自定义处理数组
     * @param array $whereData 查询条件
     * @return bool
     */
    function addInfo($setData = array(), $whereData = array())
    {

        if (!empty($setData)) {

            $data   =   array(
                'title'     =>  $setData['title'],
                'mode'      =>  $setData['mode'],
                'domain'    =>  $setData['domain'],
                'indexdir'  =>  $setData['indexdir'],
                'fz_type'   =>  $setData['fz_type'],
                'province'  =>  $setData['province'],
                'cityid'    =>  $setData['cityid'],
                'three_cityid'  =>  $setData['three_cityid'],
                'hy'        =>  $setData['hy'],
                'type'      =>  $setData['type'],
                'tpl'       =>  $setData['tpl'],
                'style'     =>  $setData['style'],
                'webtitle'  =>  $setData['webtitle'],
                'webkeyword'=>  $setData['webkeyword'],
                'webmeta'   =>  $setData['webmeta']
            );
            if (isset($setData['weblogo'])) {

                $data['weblogo']    =   $setData['weblogo'];
            }
            if (!empty($whereData)) {

                $nid    =   $this->update_once('domain', $data, $whereData);
            } else {

                $nid    =   $this->insert_into('domain', $data);
            }
            return $nid;
        }
    }

    /**
     * 更新数据
     * @param array $data 处理数据
     * @param $whereData    查询条件
     * @return bool
     */
    function upInfo($data = array(), $whereData)
    {

        if (!empty($whereData)) {

            $nid = $this->update_once('domain', $data, $whereData);
        }
        return $nid;
    }

    /**
     * 删除分站
     * @param $delId    条件id
     * @return mixed
     */
    function delDomain($delId)
    {

        if ($delId) {
            if (is_array($delId)) {

                $delId = pylode(',', $delId);
                $return['layertype'] = 1;
            } else {

                $return['layertype'] = 0;
            }

            $return['id'] = $this->delete_all("domain", array('id' => array('in', $delId)), "");
            $return['msg'] = '分站(ID:' . $delId . ')';
            $return['error'] = $return['id'] ? 0 : 1;
            $return['msg'] = $return['id'] ? $return['msg'] . '删除成功！' : $return['msg'] . '删除失败！';
        } else {

            $return['msg'] = '请选择要删除的分站信息！';
            $return['error'] = 1;
        }
        return $return;
    }
	
    /**
     * 会员列表分配分站
     * @param array $data
     * @return array
     */
    function memberSiteDid($data = array())
    {
		if(!empty($data)){
			if(empty($data['uid'])){
				
				return array('msg'=>'参数不全请重试！','status'=>8);
			}
			
			$uids	=	@explode(',',$data['uid']);
			$uid	=	pylode(',',$uids);
			if(empty($uid)){
				
				return array('msg'=>'请正确选择需分配用户！','status'=>8);
			}
			$didData	=	array('did' => intval($data['did']));
			$minfo		=	$this->select_all('member',array('uid'=>array('in',$uid)),'`uid`,`usertype`');
			
			if(is_array($minfo)&&$minfo){
				foreach($minfo as $v){
					if($v['usertype']==1){
						$rids[]		=	$v['uid'];
					}
					if($v['usertype']==2){
						$comids[]	=	$v['uid'];
					}
					
				}
			}
			if(is_array($rids)&&$rids) {
				
				$Table	=	array(
					'member', 'company_cert', 'company_order',
					'look_job','member_statis',
					'resume','resume_expect','user_entrust','userid_job'
				);
			}
			if(is_array($comids)&&$comids) {
				
				$Table	=	array(
					'member', 'company_cert', 'company_order',
					'company','company_statis','company_job','company_news',
					'company_product','partjob','hotjob'
				);
				$this -> updDid(array('userid_msg'), array('fid' => array('in', $uid)), $didData);
			}
			
			if(is_array($rids)||is_array($comids)) {
				
				$this -> updDid(array('report'), array('p_uid' => array('in', $uid)), $didData);
			}
			if(is_array($comids)) {
				
				$this -> updDid(array('down_resume'), array('comid' => array('in', $uid)), $didData);
				$this -> updDid(array('look_resume', 'userid_job'), array('com_id'=>array('in', $uid)), $didData);
			}
			
			$this -> updDid(array('company_pay'),array('com_id'=>array('in', $uid)),$didData);
			$this -> updDid($Table,array('uid'=>array('in', $uid)),$didData);
			$return['msg']		=	'会员(ID:'.$data['uid'].')分配站点成功！';
			$return['errcode']	=	9;
		}
		return $return;
	}

    /**
     * @param array $data
     * @return array|int[]
     */
    function getCityDomain($data = array())
    {

		$x = $data['x'];
		$y = $data['y'];
		$did = intval($data['did']);

		$return = array();
        $citydomain = array();
        $error = 0;

        //两个分站开关只要有一个没开 就把设置的缓存清掉
        if($this->config['sy_web_site']!=1 || $this->config['sy_gotocity']!=1){
        	return array('error'=>2);
        }

		if(!empty($x) && !empty($y)){
            
            $geoarr = array(
                'bmap_webak'=>$this->config['bmap_webak'],
                'lat'=>round($y,6),
                'lng'=>round($x,6)
            );
            $geores = bd_reverse_geocoding($geoarr);

            if($geores['error']==1){
                $province = $geores['data']['addressComponent']['province'];
                $city     = $geores['data']['addressComponent']['city'];
                $district = $geores['data']['addressComponent']['district'];

                $address = array($district,$city,$province);
            }
            
            if(!empty($address)){

                include(PLUS_PATH."domain_cache.php");
                
                include(PLUS_PATH . "city.cache.php");
                
                $address[0] = str_replace(array('市', '县', '区'), '',$address[0]);
                $address[1] = str_replace(array('市', '县', '区'), '',$address[1]);
                $address[2] = str_replace('省', '',$address[2]);

                $match_arr = array();
                $sort_arr = array();
                
                foreach($site_domain as $k=>$v){

                	if($v['fz_type']=='1'){
                		$city_str = '';
                		if($v['province']>0){
                            $city_sort = 1;
                            $city_str .= $city_name[$v['province']];
						}
						if($v['cityid']>0){
                            $city_sort = 2;
                            $city_str .= $city_name[$v['cityid']];
						}
                		if($v['three_cityid']>0){
                			$city_sort = 3;
                			$city_str .= $city_name[$v['three_cityid']];
                		}

                		$matched = false;

                		if(!$matched && strpos($v['cityname'],$address[0]) !== false){//匹配的接口返回的城市是县级
                			//该分站地址是一级城市，或者如果该分站地址不是一级城市，就再验证一下上一级，避免 南京市鼓楼区和徐州市鼓楼区这种情况
                			if($city_sort==1 || ($city_sort>1 && strpos($city_str,$address[1]) !== false)){
                				$matched = true;
                			}
                		}
                		if(!$matched && strpos($v['cityname'],$address[1]) !== false){//匹配的接口返回的城市是市级
                			//该分站地址是一级城市，或者如果该分站地址不是一级城市，就再验证一下上一级
                			if($city_sort==1 || ($city_sort>1 && strpos($city_str,$address[2]) !== false)){
                				$matched = true;
                			}
                		}
                		if(!$matched && strpos($v['cityname'],$address[2]) !== false){//匹配的接口返回的城市是省级
                			
                			$matched = true;
                		}
                		if($matched){
                			$v['city_str'] = $city_str;
	                		$v['city_sort'] = $city_sort;
	                		$sort_arr[] = $city_sort;
	                		$match_arr[] = $v;
                		}
                	}
                }
                if(!empty($match_arr)){

                	array_multisort($sort_arr,SORT_DESC,$match_arr);

                	$citydomain['did']		=	$match_arr[0]['id'];
                    $citydomain['cityname']	=	$match_arr[0]['cityname'];
                    $citydomain['domain_n']	=	$match_arr[0]['webname'];
					if ($match_arr[0]['city_sort']==3){
                        $citydomain['curcityid'] = $match_arr[0]['three_cityid'];
                    }elseif ($match_arr[0]['city_sort']==2){
                        $citydomain['curcityid'] = $match_arr[0]['cityid'];
                    }elseif ($match_arr[0]['city_sort']==1){
                        $citydomain['curcityid'] = $match_arr[0]['province'];
                    }

                    if($match_arr[0]['mode']=='2'){
                        $citydomain['url']=$this->config['sy_weburl'].'/'.$match_arr[0]['indexdir'].'/';
                    }else{
                        $protocol   =   getprotocol($this->config['sy_weburl']);
                        $citydomain['url']=$protocol.$match_arr[0]['host'];
                    }
                    
                    $error = 1;
                }
            }
        }
        if(!empty($citydomain) && $did==$citydomain['did']){
        	$error = 0;//如果定位到的分站和当前分站一样  就不用再提示跳转了
        	$citydomain = array();
        }
        $return['error'] = $error;
        $return['citydomain'] = $citydomain;

        return $return;
	}
}

?>