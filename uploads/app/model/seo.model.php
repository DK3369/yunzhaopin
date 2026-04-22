<?php

class seo_model extends model{
	
	/**
	 * 获取seo列表
	 * $whereData 	查询条件
	 * $data		自定义
	 */
	public function getSeoList($whereData=array(),$data=array()){
		$field	=   $data['field'] ? $data['field'] : '*';
		$list  	=  	$this -> select_all('seo',$whereData,$field);

		if (!empty($list)) {
		    foreach ($list as $key => $val) {
		        !empty($val['time']) && $list[$key]['time_n'] = date('Y-m-d', $val['time']);
            }
        }

		return	$list;
	}
	/**
	 * 获取seo详细信息
	 * $whereData 	查询条件
	 * $data		自定义查询字段 field:查询字段，默认为*
	 */
	public function getSeoInfo($whereData=array(),$data=array('field'=>'*')){
		$info  =  $this -> select_once('seo',$whereData,$data['field']);
		return	$info;
	}
	/**
	 * 添加seo
	 * $data		自定义
	 */
	public function addSeo($addData=array(),$data=array()){
		$return  =  $this -> insert_into('seo',$addData);
		return	$return;
	}
	/**
	 * 更新seo
	 * $whereData 	查询条件
	 * $data		自定义
	 */
	public function upSeo($whereData=array(),$addData=array()){
		$return  =  $this -> update_once('seo',$addData,$whereData);
		return	$return;
	}
	/**
	 * 删除seo
	 * 删除条件
	 */
	public function delSeo($whereData=array(),$data=array()){
		$result	=  $this -> delete_all('seo',$whereData,'');		
		return	$result;
	}
}
?>