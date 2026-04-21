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
class announcement_model extends model{


	/**
	* @desc   获取公告列表
	* @param  $whereData:查询条件
	* @param  $data:自定义处理数组
	*/
	public function getList($whereData,$data=array()) {

		$ListAnnouncement    =   array();

		$select    			 =   $data['field'] ? $data['field'] : '*';

		$List      			 =   $this   ->  select_all('admin_announcement',$whereData,$select);

		if (!empty($List)) {
			if($data['utype']=='wxapp'){
				foreach($List as $key=>$va){
					$List[$key]['title']	=	mb_substr($va['title'], 0,26,'utf-8');
					$List[$key]['datetime_n']	=	date('Y-m-d',$va['startime']);
				}
			} else {
			    if (!empty($data['domain'])) {
			        include_once 'cache.model.php';
			        $cacheM = new cache_model($this->db, $this->def);
                    $domain = $cacheM->GetCache('domain', $Options = array('needreturn' => true, 'needassign' => true, 'needall' => true));
                    $domainName = $domain['Dname'];
                }

			    foreach ($List as $key => $val) {
                    !empty($val['datetime']) && $List[$key]['datetime_n'] = date('Y-m-d', $val['datetime']);
                    !empty($val['startime']) && $List[$key]['startime_n'] = date('Y-m-d', $val['startime']);
                    !empty($val['endtime']) && $List[$key]['endtime_n'] = date('Y-m-d', $val['endtime']);

                    if (!empty($data['domain']) && isset($val['did'])) {
                        $List[$key]['dname'] = $domainName[$val['did']];
                    }
                }
            }

			$ListAnnouncement ['list']	=	$List;

		}

	   return $ListAnnouncement;
    }
    /**
     *获取公告数量
     * */
    function getNum(){
        return $this->select_num('admin_announcement');
    }
	/**
	* @desc   获取公告详情
	*/
	function getInfo($where=array(),$data	=	array()){

		$select    		 	=   $data['field'] ? $data['field'] : '*';
		$Info	=	 $this -> select_once('admin_announcement', $where, $select);
		if(!empty($Info)){
			$Info['content']=str_replace(array("&nbsp;","&"),array(" ","&amp;"),$Info['content']);

			preg_match_all('/<img(.*?)src=("|\'|\s)?(.*?)(?="|\'|\s)/',$Info['content'],$res);

			if(!empty($res[3])){
				foreach($res[3] as $v){
					if(strpos($v,'http:')===false && strpos($v,'https:')===false){

						$Info['content'] = str_replace($v,$this->config['sy_ossurl'].$v,$Info['content']);
					}
				}
			}

            !empty($Info['datetime']) && $Info['datetime_n'] = date('Y-m-d', $Info['datetime']);
            !empty($Info['startime']) && $Info['startime_n'] = date('Y-m-d', $Info['startime']);
            !empty($Info['endtime']) && $Info['endtime_n'] = date('Y-m-d', $Info['endtime']);
		}
		return $Info;

	}
	/**
    * @desc    添加数据
    */
	public function addInfo($data = array())
	{
		if(!empty($data['content'])){

		    $content	=	str_replace(array('"','\''),array('',''),$data['content']);

			preg_match_all('/<img[^>]+src=(.*?)\s[^>]+>/im',$content,$match);

		    if(!empty($match[1])){

		        $mbstr	=  substr(strrchr($match[1][0], "\\"), 1);

				$str    =  str_replace($mbstr,'',$match[1][0]);

				$str    =  str_replace("\\",'',$str);

			}

			$contentRep		=	array('&amp;',"background-color:#ffffff","background-color:#fff","white-space:nowrap;");

			$contentStr		=	str_replace($contentRep, array('&','', '', ''), $data['content']);

		}
        $time    = time();
		$AddData = array(

		    'did'			=> 	$data['did'] == '' ? 0 : $data['did'],

			'title'			=>	$data['title'],

		    'startime'      =>  !empty($data['startime']) ? strtotime($data['startime']) : $time, //开始时间

			'endtime'       =>  strtotime($data['endtime']),

			'keyword'		=>	$data['keyword'],

			'description'	=>	$data['description'],

		    'content'		=>	$contentStr	,

		    'datetime'		=>	$time
		);

		$nid  =  $this->insert_into('admin_announcement', $AddData);

		return $nid;
	}
	/**
    * @desc    更新数据
    */
	public function upInfo($whereData, $data = array()){

		if(!empty($whereData)) {

			if(!empty($data['content'])){

				$content	=	str_replace(array('"','\''),array('',''),$data['content']);

				preg_match_all('/<img[^>]+src=(.*?)\s[^>]+>/im',$content,$match);

				if(!empty($match[1])){

					$mbstr	=  substr(strrchr($match[1][0], "\\"), 1);

					$str    =  str_replace($mbstr,'',$match[1][0]);

					$str    =  str_replace("\\",'',$str);

				}

				$contentRep		=	array('&amp;',"background-color:#ffffff","background-color:#fff","white-space:nowrap;");

				$contentStr		=	str_replace($contentRep, array('&','', '', ''), $data['content']);


			}

			$PostData			=	array(

			    'did'			=> 	$data['did'] == '' ? 0 : $data['did'],

				'title'			=>	$data['title'],

				'startime'      =>  strtotime($data['startime']), //开始时间

				'endtime'       =>  strtotime($data['endtime']),

				'keyword'		=>	$data['keyword'],

				'description'	=>	$data['description'],

			    'content'		=>	$contentStr
			);

			$nid  =  $this -> update_once('admin_announcement', $PostData, array('id'=>$whereData['id']));

			return $nid;
		}
    }

	/**
    * @desc    删除数据
    */
	public function delAnnouncement($delId){

		if(empty($delId)){

			return	array(

			  'errcode' => 8,

			  'msg' 	=> '请选择要删除的数据！',
            );

		}else{

			if(is_array($delId)){

				$delId	=	pylode(',',$delId);

				$return['layertype']	=	1;

			}else{

				$return['layertype']	=	0;
			}


			$nid	=	$this -> delete_all('admin_announcement',array('id' => array('in',$delId)),'');

			if($nid){

				$return['msg']		=	'公告(ID:' . $delId . ')';

				$return['errcode']	=	$nid ? '9' :'8';

				$return['msg']		=	$nid ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';

			}

		}

		return	$return;

	}

	/*
	 * 增加浏览次数
	 */
	public function upViewNum($id){
        return $this->update_once('admin_announcement', array('view_num' =>  array('+', 1)), array('id' => $id));
    }
}
?>