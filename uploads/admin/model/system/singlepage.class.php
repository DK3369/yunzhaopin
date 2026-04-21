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
class singlepage_controller extends adminCommon{

	function index_action(){

		$data = array();

		if($_POST['order']){
			if($_POST['order']=="desc"){
				$where['orderby']	=	$_POST['t'].",desc";
			}else{
				$where['orderby']	=	$_POST['t'].",asc";
			}
		}else{
			$where['orderby']	=	"id,desc";
		} 
		if ($_POST['is_type']!=""){
		    $where['is_type']	=	$_POST['is_type'];
		}
		if (trim($_POST['keyword'])){
		    $where['name']		=	array('like',trim($_POST['keyword']));
		}
		
		//提取分页
		$pageM			=	$this  -> MODEL('page');
		$pages			=	$pageM -> adminPageList('description',$where,$_POST['page'],array('limit'=>$_POST['limit']));

		if($pages['total'] > 0){
		    //limit order 只有在列表查询时才需要
		    if($_POST['order']){
				if($_POST['order']=="desc"){
					$where['orderby']	=	$_POST['t'].",desc";
				}else{
					$where['orderby']	=	$_POST['t'].",asc";
				}
			}else{
				$where['orderby']	=	"id,desc";
			}
		    $where['limit']				=	$pages['limit'];
		    $descM	=	$this 	-> MODEL('description');
		    $List	=	$descM	-> getDesList($where);
		    
		}

		$data['list']           =   !empty($List)?$List:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
	}
	//添加
	function add_action(){
		
		$data = array();

		$descM	=	$this 	-> MODEL('description');

		if($_POST['id']){
			$descrow			=	$descM	->	getDes(array('id'=>$_POST['id']));
		}
		
		$class	=	$descM	->	getDesClassList(array('orderby'=>'sort,desc'));

		$data['info'] = !empty($descrow)?$descrow:array();
		$data['class'] = $class;

		$this->render_json(0, '', $data);
	}
	//保存
	function save_action(){
	    $_POST=$this->post_trim($_POST);
		$addData['name']	=	$_POST['name'];
		if($_POST['url'] && $_POST['is_type']==1){
			//验证URL
			$url 		=	stripslashes($_POST['url']);
			$url 		=	str_replace("../","",$url);
			$url 		=	str_replace("./","",$url);
			$p_delfiles =	path_tidy($url);
			if($p_delfiles!=$url){
				$this->render_json(1, '无效的文件名！');
			}
			$urlArr	=	explode('/',$url);
			foreach($urlArr as $v){
				if(!preg_match("/^[".chr(0xa1)."-".chr(0xff)." |a-z|0-9|A-Z|\@\.\_\]\[\!]+$/",$v) && $v!='') {
					$this->render_json(1, '无效的文件名！');
				}
			}
			$urlarr	=	explode(".",$url);
			if(end($urlarr)!="html"){
				$this->render_json(1, '请正确填写静态网页名称！');
			}
			if(substr($url,0,1)=="/"){
				$url	=	substr($url,1);
			}
		}
		$addData['nid']				=	$_POST['nid'];
		$addData['url']				=	$_POST['is_type']==1 ? $url : $_POST['url'];
		$addData['title']			=	$_POST['title'];
		$addData['keyword']			=	$_POST['keyword'];
		$addData['descs']			=	$_POST['description'];
		$addData['top_tpl']			=	$_POST['top_tpl'];
		$addData['top_tpl_dir']		=	$_POST['top_tpl_dir'];
		$addData['footer_tpl']		=	$_POST['footer_tpl'];
		$addData['footer_tpl_dir']	=	$_POST['footer_tpl_dir'];
		$addData['ctime']			=	time();
		$addData['sort']			=	$_POST['sort'];
		$addData['is_nav']			=	$_POST['is_nav'];
		$addData['is_type']			=	$_POST['is_type'];
		$addData['content'] 		=	str_replace(array("&amp;","background-color:#ffffff","background-color:#fff","white-space:nowrap;","<img "),array("&",'','','','<img style="max-width:100%" '),$_POST["content"]);
		$descM	=	$this 	-> MODEL('description');
		
		if(!$_POST['id']){
			$descid	=	$descM	->	addDes($addData);
			$ids=$descid;
			$alert="添加";
		}else{
			$row	=	$descM	->	getDes(array('id'=>$_POST['id']));
			if($row['is_menu']=="1"){
				$url			    =	str_replace("amp;","",$url);
                $addNData['url']	=	$url;
                $addNData['furl']	=	$url;
                $addNData['name']	=	$_POST['name'];
				$navM   =   $this -> MODEL('navigation');
				$navM -> upNav($addNData,array('desc'=>$_POST['id']));
				$this -> menu_cache_action();
			}
			$descid =   $descM	->	upDes($addData,array('id'=>$_POST['id']));
			$ids    =   $_POST['id'];
			$alert  =   "更新";
		}

		if($descid){
			$this	->	cache_action();
			if($_POST['is_type']==1){
                $this	->	descriptionshow($ids,$url);
			}
			$this->admin_json(0,"单页面(ID:".$ids.")".$alert."成功！");
		}else{
			$this->render_json(1,$alert."失败！");
		}
	}
	//删除
	function del_action(){
		if(is_array($_POST['del'])){
			$linkid			=	$_POST['del'];
			$data['type']	=	'all';
		}else{
			$linkid			=	$_POST["del"];
			$data['type']	=	'one';
		}
		$descM	=	$this 	-> MODEL('description');
		$navM	=	$this 	-> MODEL('navigation');
		$return	=	$descM	->	delDes($linkid,$data);
		$navM	->	delNav(array('desc'=>array('in',pylode(',',$linkid))));

		$this	->	menu_cache_action();//更新导航缓存
		$this	->	cache_action();

		$this->admin_json($return['errcode'],$return['msg']);
	}
	//生成
	function make_action(){
		if($_POST['id']){
			$where['id']		=	$_POST['id'];
		}else{
			$where['is_type']	=	1;
		}
		$this	->	cache_action();
		$descM	=	$this 	-> MODEL('description');
		$rows	=	$descM	->	getDesList($where);
		if(is_array($rows)){
			foreach($rows as $row){
                $fw	=	$this	->	descriptionshow($row['id'],$row["url"]);
			}
		}

	    if($fw){
 			$this->render_json(0,"单页面生成成功！");
 		}else{
 			$this->render_json(-1,"生成失败！");
 		}
	}
	
	
	function menu_cache_action(){//导航缓存
		include_once(LIB_PATH."cache.class.php");
		$cacheclass	=	new cache(PLUS_PATH,$this->obj);
		$makecache	=	$cacheclass	->	menu_cache("menu.cache.php");
	}
    //通过smarty缓存直接生成静态文件
    function descriptionshow($id,$path){
		
		$M		=	$this	->	MODEL('description');
		$row	=	$M	->	getDes(array("id"=>$id));
		$top	="";
		$footer	="";
		if($row['top_tpl']==1){
            $top	=	APP_PATH."/app/template/".$this->config['style']."/header.htm";
		}else if($row['top_tpl']==3){
            $top	=	APP_PATH."/app/template/".$row['top_tpl_dir'].".htm";
		}
		if($row['footer_tpl']==1){
            $footer	=	APP_PATH."/app/template/".$this->config['style']."/footer.htm";
		}else if($row['footer_tpl']==3){
            $footer	=	APP_PATH."/app/template/".$row['footer_tpl_dir'].".htm";
		}
		$seo['title']		=	$row['title'];
		$seo['keywords']	=	$row['keyword'];
		$seo['description']	=	$row['descs'];
		$this	->	yunset("seo",$seo);
		$this	->	yunset("name",$row['name']);
		$this	->	yunset("content",$row['content']);

		$this	->	header_desc($row['name']."-".$this->config['sy_webname'],$row['keyword'],$row['descs']);

		$make		=	APP_PATH."/app/template/".$this->config['style']."/make.htm";
		$make_top	=	APP_PATH."/app/template/".$this->config['style']."/make_top.htm";

        global $phpyun;
        //必须传参数$cache_id,否则多个文件的内容会重复

        if($make_top){
            $contect	=	$phpyun	->	fetch($make_top,$id);
        }
		
        if($top){
            $contect .=	$phpyun	->	fetch($top,$id);
        }
        if($make){
            $contect .= $phpyun	->	fetch($make,$id);
        }
        if($footer){
        	$contect .= $phpyun	->	fetch($footer,$id);
        }
        $Dir      =  '';
        $DirList  =	explode('/',$path);
        foreach($DirList as $k=>$v){
            $Dir .=$v.'/';
            if(!strstr($Dir,'.html') && !is_dir(APP_PATH.$Dir)){
                mkdir(APP_PATH.$Dir,0777);
            }
        }
        $fp	=	fopen(APP_PATH.$path, "w");
        $fw	=	fwrite($fp, $contect);

        fclose($fp);
        return $fw;
	}
	function cache_action(){

		include_once(LIB_PATH."cache.class.php");
		$cacheclass	= new cache(PLUS_PATH,$this->obj);

		$makecache	=	$cacheclass	->	desc_cache("desc.cache.php");
	}
	function ajax_action()
    {
        $descM              =   $this   ->  MODEL('description');
        $whereData['id']    =   $_POST['id'];
        $addData['sort']    =   $_POST['sort'];
        
        $descM  ->  upDes($addData,$whereData);
        
        $this->render_json(0, '');
    }
}
?>