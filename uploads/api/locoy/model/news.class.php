<?php

class news_controller extends common{
	function addnews_action(){//新闻添加
		include(APP_PATH."data/api/locoy/locoy_config.php");
		if($locoyinfo['locoy_online']!=1){
			echo 4;die;
		}
		if($locoyinfo['locoy_key']!=trim($_GET['key'])){
			echo 5;die;
		}
        if(!$_POST['title'] || !$_POST['content'] || !$_POST['nid']){
			echo 2;die;
		}
		$nid = intval($_POST['nid']);
		$row = $this->obj->select_once(
			"news_base",
			array(
				'title' => trim($_POST['title']),
				'nid'   => $nid
			)
		);
		if(is_array($row)){
			echo 3;die;
		}
		$content=$_POST['content'];

		$baseData = array(
			'title'  => trim($_POST['title']),
			'nid'    => $nid,
			'did'    => !empty($_POST['did']) ? intval($_POST['did']) : 0,
			'author' => isset($_POST['author']) ? $_POST['author'] : ''
		);
		$description=mb_substr(strip_tags(html_entity_decode($content,ENT_NOQUOTES,"GB2312")),0,180,"utf-8");
		$description=$_POST['description']?$_POST['description']:$description;
		$description=str_replace(array(' ',"\n","\r","\r\n"," "),array(''),$description);
		$baseData['description'] = $description;
		$baseData['source'] = isset($_POST['source']) ? $_POST['source'] : '';
		if($_POST['ctime']){
			$baseData['datetime'] = strtotime($_POST['ctime']);
			$baseData['starttime'] = strtotime($_POST['ctime']);
		}else{
			$baseData['datetime'] = time();
			$baseData['starttime'] = strtotime('today');
		}
		if($_POST['hits']){
			$baseData['hits'] = intval($_POST['hits']);
		}else{
			$row=explode('-',$locoyinfo['locoy_rand']);
			if(is_array($row)){
				$rand=rand(trim($row[0]),trim($row[1]));
			}else{
				$rand=!trim($row)?0:$row;
			}
			$baseData['hits'] = intval($rand);
		}
		if($_POST['sort']){
			$baseData['sort'] = intval($_POST['sort']);
		}else{
			$row=explode('-',$locoyinfo['locoy_sort']);
			if(is_array($row)){
				$rand=rand(trim($row[0]),trim($row[1]));
			}else{
				$rand=!trim($row)?0:$row;
			}
			$baseData['sort'] = intval($rand);
		}
		if($_POST['newsphoto']){
			$baseData['newsphoto'] = trim($_POST['newsphoto']);
		}
		if($_POST['s_thumb']){
			$baseData['s_thumb'] = trim($_POST['s_thumb']);
		}
       if(!$_POST['keyword'] && $locoyinfo['locoy_keyword']==1){
			require(LIB_PATH."lib_splitword_class.php");
			$sp = new SplitWord();
			$keywordarr=$sp->getkeyword(strip_tags(html_entity_decode($content)));
			$baseData['keyword'] = strip_tags(@implode(",",$keywordarr));
		}elseif($_POST['keyword']){
			$baseData['keyword'] = str_replace("，",",",$_POST['keyword']);
		}
		$new_base = $this->obj->insert_into("news_base", $baseData);
		if ($new_base) {
			$this->obj->insert_into(
				"news_content",
				array(
					'nbid'    => $new_base,
					'content' => html_entity_decode($content, ENT_NOQUOTES, "GB2312")
				)
			);
		}
		if($new_base){
			echo 1;die;
		}else{
			echo 0;die;	
		}
	}
}
?>
