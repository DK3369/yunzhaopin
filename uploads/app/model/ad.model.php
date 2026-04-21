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
class ad_model extends model{
    /**
     * 查询广告类别数量
     * @param $whereData
     * @return int
     */
    function getAdClassNum($whereData = array())
    {
        return intval($this->select_num('ad_class', $whereData));
    }

    /**
     * 查询广告类别
     * @param array $whereData
     * @param array $data
     * @return array
     */
    function getAdClassList($whereData = array(), $data = array())
    {
        $ListNew = array();
        $data['field'] = empty($data['field']) ? '*' : $data['field'];
        $List = $this->select_all('ad_class', $whereData, $data['field']);

        if (!empty($List)) {
            foreach ($List as $k => $v) {
                if ($data['href']) {

                    $List[$k]['hrefn']  =   checkpic($v['href']);
                }
                if ($v['place'] == 1) {

                    $List[$k]['place_n'] = 'PC';
                } else if ($v['place'] == 2) {

                    $List[$k]['place_n'] = 'WAP';
                }else{

                    $List[$k]['place_n'] = '未定义';
                }
            }
            $ListNew['list']    =   $List;
        }
        
        return $ListNew;
    }

    // 查询单个广告类
    function getAdClassInfo($whereData = array(), $data = array())
    {
        if ($whereData) {
            $data['field'] = empty($data['field']) ? '*' : $data['field'];
            
            $acInfo = $this->select_once('ad_class', $whereData, $data['field']);
        }
        
        return $acInfo;
    }

    /**
     * 更新广告类
     * @param $whereData
     * @param $data
     * @return false|null
     */
    function upAdClass($whereData = array(), $data = array())
    {
        if (! empty($whereData)) {
            
            $nid = $this->update_once('ad_class', $data, $whereData);
        }
        return $nid;
    }

    // 添加广告类
    function addAdClass($setData = array())
    {
        if (! empty($setData)) {
            
            $nid = $this->insert_into('ad_class', $setData);
        }
        
        return $nid;
    }

    // 删除广告
    function delAdClass($whereData, $data)
    {
        if ($data['type'] == 'one') { // 单个删除
            
            $limit = 'limit 1';
        }
        
        if ($data['type'] == 'all') { // 多个删除
            
            $limit = '';
        }
        
        $result = $this->delete_all('ad_class', $whereData, $limit);
        
        return $result;
    }

    /**
     * 查询广告数量
     * @param array $whereData
     * @return int
     */
    function getAdNum($whereData = array())
    {
        return intval($this->select_num('ad', $whereData));
    }

    // 查询广告
    function getList($whereData = array(), $data = array())
    {
        $ListNew = array();
        $data['field']      =   empty($data['field']) ? '*' : $data['field'];
        
        if($this->config['did']){
            $whereData['did']   =   $this->config['did'];
        }
        $List = $this->select_all('ad', $whereData, $data['field']);
        
        if (! empty($List)) {
            
            $ListNew['list'] = $List;
        }
        
        return $ListNew;
    }

    // 添加广告
    function addAd($setData = array())
    {
        if (! empty($setData)) {
            
            $nid = $this->insert_into('ad', $setData);
        }
        
        return $nid;
    }

    // 查询广告
    function getInfo($whereData = array(), $data = array())
    {
        if ($whereData) {
            $data['field'] = empty($data['field']) ? '*' : $data['field'];
            
            $info = $this->select_once('ad', $whereData, $data['field']);
            if ($info['pic_url']) {
                $info['pic_url_n'] = checkpic($info['pic_url']);
            }
        }
        
        return $info;
    }

    // 更新广告
    function upInfo($whereData = array(), $data = array())
    {
        if (! empty($whereData)) {
            
            $nid = $this->update_once('ad', $data, $whereData);
        }
        return $nid;
    }

    // 删除广告
    function delAd($whereData, $data)
    {
        if ($data['type'] == 'one') { // 单个删除
            
            $limit = 'limit 1';
        }
        
        if ($data['type'] == 'all') { // 多个删除
            
            $limit = '';
        }
		 
        $result = $this->delete_all('ad', $whereData, $limit);
        
        return $result;
    }
    function getAdClassArr()
    {
        $adClass    =   $this->select_all('ad_class', array('orderby' => 'id, desc'), '`id`,`class_name`,`place`');
        $classOne   =   array(array('id' => 1, 'name' => 'PC'), array('id' => 2, 'name' => 'WAP'), array('id' => 3, 'name' => '未定义'));
        $classTwo   =   array();
        $classNameArr   =   array();
        $maxidlen = 1;
        foreach ($adClass as $kk => $vv) {
            strlen($vv['id']) > $maxidlen && $maxidlen = strlen($vv['id']);
        }
        foreach ($adClass as $ak => $av) {
            $middle = str_pad($av['id'], $maxidlen, $pad_string = "0", $pad_type = 0) . '&nbsp;&nbsp;&nbsp;';
            if ($av['place'] == 1) {
                
                $classTwo[1][]  =   array('id' => $av['id'], 'name' => $av['class_name'], 'id_name' => $middle . $av['class_name']);
            } else if ($av['place'] == 2) {
                
                $classTwo[2][]  =   array('id' => $av['id'], 'name' => $av['class_name'], 'id_name' => $middle . $av['class_name']);
            } else {
                
                $classTwo[3][]  =   array('id' => $av['id'], 'name' => $av['class_name'], 'id_name' => $middle . $av['class_name']);
            }
            
            $classNameArr[$av['id']]    =   $av['class_name'];
        }
        $classArr    =   array('classOne' => $classOne, 'classTwo' => $classTwo);
        
        return array('classArr' => $classArr, 'classNameArr' => $classNameArr);
    }
    /**
     * @desc 广告列表信息
     * @param array $whereData
     * @param array $data
     * @return array
     */
    function getAdList($whereData = array(), $data = array())
    {

        $ListNew=   array();
        $List   =   $this->getList($whereData, $data['field']);
        if (!empty($List['list'])) {
            $adClassArr = $this->getAdClassArr();
            $classNameArr = $adClassArr['classNameArr'];
            $ListNew['classArr']    =   $adClassArr['classArr'];

            // 提取分站
            require_once('cache.model.php');
            $cacheM     =   new cache_model($this->db, $this->def);
            $domain     =   $cacheM->GetCache('domain');
            $linkrows   =   $List['list'];

            if (is_array($linkrows)) {
                foreach ($linkrows as $key => $value) {

                    $start  =   @strtotime($value['time_start']);
                    $end    =   @strtotime($value['time_end']." 23:59:59");
                    $time   =   time();

                    $linkrows[$key]['class_name']   =   $classNameArr[$value['class_id']];
                    if ($value['did'] > 0) {

                        $linkrows[$key]['d_title']  =   $domain['Dname'][$value['did']];
                    } elseif ($value['did'] == -1) {

                        $linkrows[$key]['d_title']  =   '全站';
                    } else {

                        $linkrows[$key]['d_title']  =   '主站';
                    }
                    if ($value['is_check'] == "1") {
                        $linkrows[$key]['check']    =   "<font color='green'>已审核</font>";
                    } else {
                        $linkrows[$key]['check']    =   "<font color='red'>未审核</font>";
                    }
                    $value['pic_url']   =   checkpic($value['pic_url']);

                    switch ($value['ad_type']) {
                        case "word":
                            $linkrows[$key]['ad_typename'] = "文字广告";
                            break;
                        case "pic":
                            $linkrows[$key]['ad_typename'] = '<a href="javascript:void(0)" class="preview admin_n_img" url="' . $value['pic_url'] . '"></a>';
                            break;
                        case "flash":
                            $linkrows[$key]['ad_typename'] = "FLASH广告";
                            break;
                        case "lianmeng":
                            $linkrows[$key]['ad_typename'] = "联盟广告";
                            break;
                    }

                    if ($value['time_start'] != "" && $start != "" && ($value['time_end'] == "" || $end != "")) {
                        if ($value['time_end'] == "" || $end > $time) {
                            if ($value['is_open'] == '1' && $start < $time) {

                                $linkrows[$key]['type'] =   "<font color='green'>使用中..</font>";
                            } else if ($start < $time && $value['is_open'] == '0') {

                                $linkrows[$key]['type'] =   "<font color='red'>已停用</font>";
                            } elseif ($start > $time && ($end > $time || $value['time'] == "")) {

                                $linkrows[$key]['type'] =   "<font color='#ff6600'>广告暂未开始</font>";
                            }
                        } else {

                            $linkrows[$key]['type']     =   "<font color='red'>过期广告</font>";
                            $linkrows[$key]['is_end']   =   '1';
                        }
                    } else {
                        $linkrows[$key]['type'] =   "<font color='red'>无效广告</font>";
                    }

                    //图片
                    if ($linkrows[$key]['pic_url']) {
                        $pic_url_arr = explode(',', $linkrows[$key]['pic_url']);
                        foreach ($pic_url_arr as $url) {
                            $linkrows[$key]['pic_url_list'][] = checkpic($url);
                        }
                    }
                }
            }

            $ListNew['list']    =   $linkrows;
        }
        return $ListNew;
    }

    // 查询广告点击次数
    function getAdClickNum($Where)
    {
        return $this->select_num('adclick', $Where);
    }

    /**
     * 引用userinfo类，查询member列表信息
     */
    // 插入广告点击记录
    function addAdClick($data = array())
    {
		
        return $this->insert_into('adclick', $data);
    }

    function model_ad_arr()
    {
        $show = "<?php\r\n\$ad_label=array();\r\n";
        
        $ad_list = $this->select_all("ad", array('is_open' => 1,'orderby' => array('sort,desc','id,desc')));
        
        if (is_array($ad_list)) {
            
            $time = time();
            
            foreach ($ad_list as $key => $value) {
                
				foreach($value as $k=>$v){
					$value[$k] = setSecFilter($v);
				}

                $start = strtotime($value['time_start'] . " 00:00:00");
                
                $end   = strtotime($value['time_end'] . " 23:59:59");
                
                if (! empty($value['time_end'])) {
                    
                    if ($end > $time) {
                        
                        $end_type = 1;
                    } else {
                        
                        $end_type = 2;
                    }
                } else {
                    
                    $end_type = 1;
                }
                if ($start && $end_type == 1 && $value['is_check'] == "1") {
                    
                    if ($value['ad_type'] == "word") {
                        if ($value['target'] == 2) {
                            $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['html']=\"<a href='$value[word_url]' target='_blank'>$value[word_info]</a>\";\r\n";
                        }else{
                            $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['html']=\"<a href='$value[word_url]'>$value[word_info]</a>\";\r\n";
                        }
                    } elseif ($value['ad_type'] == "pic") {
                        
                        if (@! stripos("ttp://", $value['pic_url'])) {
                            
                            $pic_url = checkpic($value['pic_url']);
                        }
                        
                        $height = $width = $alt = "";
                        
                        if ($value['pic_height']) {
                            
                            $height = "height='$value[pic_height]'";
                        }
                        if ($value['pic_width']) {
                            
                            $width  = "width='$value[pic_width]'";
                        }
                        if ($value['pic_content']) {
                            
                            $alt    = "alt='".str_replace('"','\"',$value['pic_content'])."'";
                        }
                        if ($this->config['sy_seo_rewrite'] == '1') {
                            $pic_src = $this->config['sy_weburl'] . "/c_clickhits-id_" . $value['id'] . ".html";
                        } else {
                            
                            $pic_src = $this->config['sy_weburl'] . "/index.php?c=clickhits&id=" . $value['id'];
                        }
                        
                        if ($value['target'] == 2) {
                            
                            $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['html']=\"<a href='$pic_src' target='_blank' rel='nofollow'><img src='$pic_url'  " . $height . " " . $width . " " . $alt . "></a>\";\r\n";

							$show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['lay_html']=\"<a href='$pic_src' target='_blank' rel='nofollow'><img lay-src='$pic_url'  " . $height . " " . $width . " " . $alt . "></a>\";\r\n";


                        } else {
                            
                            $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['html']=\"<a href='$pic_src' rel='nofollow'><img src='$pic_url' " . $height . " " . $width . " " . $alt . " ></a>\";\r\n";

							$show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['lay_html']=\"<a href='$pic_src' rel='nofollow'><img lay-src='$pic_url' " . $height . " " . $width . " " . $alt . " ></a>\";\r\n";
                        }
                        
                        $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['pic']=\"$pic_url\";\r\n";
                        
                        $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['src']=\"$pic_src\";\r\n";
                    } elseif ($value['ad_type'] == "flash") {
                        
                        if (! stripos("ttp://", $value['flash_url'])) {
                            
                            $flash_url = checkpic($value['flash_url']);
                        }
                        
                        $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['html']=\"<object type='application/x-shockwave-flash' data='$flash_url' width='$value[flash_width]' height='$value[flash_height]'><param name='movie' value='$flash_url' /><param value='transparent' name='wmode'></object>\";\r\n";
                    } elseif ($value['ad_type'] == "lianmeng") {
                        
                        $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['html']=\"" . str_replace('"', '\\"', $value['lianmeng_url']) . "\";\r\n";
                    }
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['start']=\"" . strtotime(date('Y-m-d H:i:s', $start)) . "\";\r\n";
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['end']=\"" . strtotime(date('Y-m-d H:i:s', $end)) . "\";\r\n";
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['type']=\"" . $value['ad_type'] . "\";\r\n";
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['name']=\"" . $value['ad_name'] . "\";\r\n";
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['did']=\"" . $value['did'] . "\";\r\n";
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['id']=\"" . $value['id'] . "\";\r\n";
                    $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['class_id']=\"" . $value['class_id'] . "\";\r\n";
                    if($value['appurl']){
                        $show .= "\$ad_label['$value[class_id]']['ad_$value[id]']['appurl']=\"" . $value['appurl'] . "\";\r\n";
                    }
                }
            }
        }
        $show .= "?>";
        $path = PLUS_PATH . "pimg_cache.php";
        $fp = @fopen($path, "w");
        $fw = @fwrite($fp, $show);
        @fclose($fp);
        @chmod($path, 0777);
        return $fw;
    }

    /**
     * 广告 添加 修改
     * @param array $post
     * @param string $pic 图片广告URL，FLASH广告URL
     * @return array
     */
    function model_saveadd($post, $pic = NULL)
    {
        if (empty($post['did'])) {
            
            $did = 0;
        }
        $value = array(
            
            'ad_name'    => $post['ad_name'],
            'target'     => $post['target'],
            'time_start' => $post['ad_time_start'],
            'time_end'   => $post['ad_time_end'],
            'ad_type'    => $post['ad_type'],
            'class_id'   => $post['class_id'],
            'is_check'   => $post['is_check'],
            'did'        => $post['did'],
            'is_open'    => $post['is_open'],
            'sort'       => $post['sort'],
            'remark'     => $post['remark'],
            'appurl'     => $post['appurl']
        );
        //广告类型
        if ($post['ad_type']) {
            //文字广告
            if ($post['ad_type'] == "word") {
				
                $value['word_info']	=	$post['word_info'];
                $value['word_url']	=	$post['word_url'];
            } elseif ($post['ad_type'] == "pic") {
                //图片广告
                if ($pic != "") {
                    if (strpos($pic,$this->config['sy_weburl']) !== false){
                        $pic  =  str_replace($this->config['sy_weburl'].'/data','./data',$pic);
                    }
                    if (strpos($pic,$this->config['sy_ossurl']) !== false){
                        $pic  =  str_replace($this->config['sy_ossurl'].'/data','./data',$pic);
                    }
                    $value['pic_url']	=	$pic;
                }
                $pic_src = str_replace("amp;", "", $post['pic_src']);
                //图片链接
                $value['pic_src']		=	$pic_src;
                $value['pic_content']	=	$post['pic_content'];
                $value['pic_width']		=	$post['pic_width'];
                $value['pic_height']	=	$post['pic_height'];
            } elseif ($post['ad_type'] == "flash") {
                //FLASH广告
				if ($pic != "") {
                    $value['flash_url']	=	$pic;
                }
                $value['flash_width']	=	$post['flash_width'];
                $value['flash_height']	=	$post['flash_height'];
                
            } elseif ($post['ad_type'] == "lianmeng") {
                //联盟广告
                $value['lianmeng_url']	=	$post['lianmeng_url'];
            }

			if($post['id']){
				$nid 	= 	$this->update_once("ad", $value, array('id' => $post['id']));
				$msg	= 	'广告更新(ID:' . $post['id'] . ')';
			}else{
				$nid 	= 	$this->insert_into("ad", $value);
				$msg	=	'广告添加(ID:' . $nid . ')';
			}
            $this->model_ad_arr();
            
            if ($nid) {
                $return['msg']		= 	$msg."成功！";
                $return['error']	=	0;
            } else {
                
                $return['msg']		=	$msg."失败！";
                $return['error']	=	1;
            }
        } else {
            
            $return['msg'] 		= 	"您还未选择广告类型！";
            $return['error']	=	2;
        }
        return $return;
    }

	/**
	 * 未发现使用：   -----  【 sy  2020年6月12日 11:42:30】
	 * 暂时不做删除
	 */
    /**
	function model_del_ad($id)
    {
        
        if ($id) {

            $result =   $this->delete_all('ad', array('id' => $id));

            $return['msg']      =   $result ? '删除成功！' : '删除失败！';
            $return['url']      =   'index.php?m=advertise';
            $return['errcode']  =   $result ? 9 : 8;

            $this->model_ad_arr();
        }

        return $return;
    }
	*/
}
?>