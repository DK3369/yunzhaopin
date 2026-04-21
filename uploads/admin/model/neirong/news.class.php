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
class news_controller extends adminCommon
{
    /**
     * 内容 - 新闻 - 新闻管理
     */
    public function index_action(){
        //实例化新闻类
        $articleM =	$this->MODEL('article');
        //提取新闻分类
        $listNew = $articleM->getClass(array('isson' => 1));
        $newsGroup = $listNew['list'];
        $catearr = $_POST['cate'];
        //搜索条件
        if (!empty($catearr)){
            if(count($catearr)==1){
                $cateStr = $catearr[0];
                    if(isset($two_class[$cateStr])){
                    $ids = array_keys($two_class[$cateStr]);
                }
                $ids[] = $cateStr;
                $where['nid'] = array('in', pylode(',',$ids));
            }else if(count($catearr)==2){
                $where['nid'] = $catearr[1];
            }
            
        }
        
        if ($_POST['cates'] != '') {
            $where['nid'] =	$_POST['cates'];
        }
        if($_POST['adtime']) {
            if($_POST['adtime']=='1') {
                $where['datetime'] = array('>',strtotime(date('Y-m-d 00:00:00')));
            }else{
                $where['datetime'] = array('>',strtotime('-'.intval($_POST['adtime']).' day'));
            }
        }
        if($_POST['publish']) {
            if($_POST['publish']=='1') {
                $where['datetime'] = array('>=',strtotime(date('Y-m-d 00:00:00')));
            }else{
                $where['datetime'] = array('>=',strtotime('-'.(int)$_POST['publish'].'day'));
            }
        }
        $keywordStr = trim($_POST['keyword']);
        if ($keywordStr) {
            if ($_POST['type'] == 1) {
                $where['title']	= array('like', $keywordStr);
            }elseif ($_POST['type'] == 2){
                $where['author'] = array('like', $keywordStr);
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('news_base', $where, $page, array('limit' => $pageSize));
        //分页数大于0的情况下 执行列表查询
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby'] =	'id';
            }
            $where['limit'] = $pages['limit'];
            $List =	$articleM->getList($where,array('tlen'=>'20','group'=>$newsGroup,'content' => 1));
        }
        $rt['list'] = $List['list'] ? $List['list'] : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $rt);
    }
    
    public function  getCache_action(){
        //实例化新闻类
        $articleM =	$this->MODEL('article');
        //提取新闻分类
        $listNew = $articleM->getClass(array('isson' => 1));
        $two_class = $listNew['two_class'];
        $rt['one_class'] = $listNew['one_class'];
        $rt['two_class'] = $two_class;
        $class_cascader = $class_arr = array();
        foreach ($listNew['one_class'] as $v) {
            $children = array();
            $class_arr[] = $v;
            if (isset($two_class[$v['id']])) {
                foreach ($two_class[$v['id']] as $vv) {
                    $class_arr[] = array('id' => $vv['id'], 'name' => ' 　┗' . $vv['name']);
                    $children[] = array('value'=>$vv['id'],'label'=>$vv['name']);
                }
            }
            $one = array('value'=>$v['id'],'label'=>$v['name'],'children'=>$children);
            $class_cascader[] = $one;
        }

        $property			=	$articleM -> getProperty();
        $propertyList = array();
        if(!empty($property)){
            foreach($property as $value)
            {
                $propertyList[$value['value']]	=	$value['name'];
            }
        }
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain',$Options = array('needreturn' => true, 'needassign' => true, 'needall' => true));
        $rt['Dname'] = (object)$domain['Dname'];
        $rt['property']	=	$propertyList?$propertyList:array();
        $rt['class_arr'] = $class_arr;
        $rt['class_cascader'] = $class_cascader;
        $rt['today'] = date('Y-m-d');
        $this->render_json(0,'ok', $rt);

    }

    /**
     * 内容 - 新闻 - 新闻管理
     * 分配分站
     */
    public function checksitedid_action(){
        if(empty($_POST['uid'])){
            $this->render_json(1, '参数不全请重试！');
        }
        $uids =	@explode(',', $_POST['uid']);
        $uid = pylode(',', $uids);
        if(empty($uid)){
            $this->render_json(1, '请正确选择需分配新闻！');
        }
        $siteDomain = $this->MODEL('site');
        $didData = array('did' => $_POST['did']);
        $siteDomain->updDid(array('news_base'), array('id' => array('in', $uid)), $didData);
        $siteDomain->updDid(array('news_content'),  array('nbid' => array('in', $uid)), $didData);
        $this->admin_json(0, '新闻(ID:'.$_POST['uid'].')分配站点成功！');
    }

    /**
     * 内容 - 新闻 - 新闻管理
     * 保存新闻属性 取消新闻属性
     */
    public function savepro_action(){
        $typeStr = trim($_POST['type']);
        $_POST = $this->post_trim($_POST);
        if(empty($_POST['proid'])){
            $this->render_json(1, '参数错误！');
        }
        //实例化新闻类
        $articleM =	$this->MODEL('article');
        $baseWhereData = array('id' => array('in', $_POST['proid']));
        $list =	$articleM->getList($baseWhereData, array('field' => '`id`, `describe`'));
        if(empty($list['list'])){
            $this->render_json(1, '数据错误！');
        }
        //保存新闻属性
        if($typeStr == 'add'){
            $describe =	pylode(',', $_POST['describe']);
            if(empty($describe)){
                $this->render_json(1, '请选择属性！');
            }
            $articleM->upBase($baseWhereData, array('describe' => $describe));
            $this->admin_json(0, '新闻(ID:'.$_POST['proid'].')设置属性成功！');
        }
        //删除新闻属性
        if($typeStr == 'del'){
            foreach($list['list'] as $key => $value){
                if(!empty($value['describe'])){
                    $describe =	@explode(',', $value['describe']);
                    foreach($describe as $key => $v){
                        if(in_array($v, $_POST['describe'])){
                            unset($describe[$key]);
                        }
                    }
                    $articleM->upBase(array('id' => array('=', $value['id'])), array('describe' => pylode(',', $describe)));
                }
            }
            $this->admin_json(0, '新闻(ID:'.$_POST['proid'].')删除属性成功！');
        }
    }

    /**
     * 内容 - 新闻 - 新闻管理
     * 保存新闻
     */
    public function addnews_action()
    {
        $articleM = $this->MODEL('article');
        if (isset($_POST['add'])){
            $newContent =   array();
            if (isset($_POST['id']) && !empty($_POST['id'])){
                $newContent =   $articleM->getInfo(array('id' => $_POST['id']), array('iscon' => 1));
            }
            $this->render_json(0, '', array('content' => $newContent['content']));
        }else{
            $postData = $this->post_trim($_POST);
            if (!$postData['title']) {
                $this->render_json(1, '新闻标题不能为空');
            }
            if (!$postData['nid']) {
                $this->render_json(1, '请选择新闻类别');
            }
            if (!$postData['content']) {
                $this->render_json(1, '新闻内容不能为空');
            }
            // 新上传图片文件处理
            foreach ($_FILES['pic'] as $nk => $nv) {
                foreach ($nv as $nkk => $nvv) {
                    $pic_file[$nkk][$nk] = $nvv;
                }
            }
            if ($pic_file[0]) {
                $postData['file'] = $pic_file[0];
            }
            if($postData['starttime'] == ''){
                $postData['starttime'] = date('Y-m-d');
            }
            $return = $articleM->addNews($postData);
            if($return['errcode'] == 9 && !empty($return['data'])){
                $this->articleshow($return['data']);
                $this->admin_json(0, $return['msg']);
            }else{
                $this->render_json(1, $return['msg']);
            }
        }
    }

    //通过smarty缓存直接生成静态文件
    public function articleshow($id){
        // 系统-页面设置-新闻显示形式-静态，才需要生成静态文件
        if($this->config['sy_news_rewrite'] == 2){
            $articleM =	$this->MODEL('article');
            $news =	$articleM->getInfo(array('id' => $id),array('iscon' => '1'));
            if(empty($news)){
                return false;
            }
            $fieldArr =	array('feild' => '`datetime`, `id`, `title`');
            //获取前一篇文章
            $news_last = $articleM->getInfo(array('id' => array('<', $id), 'orderby' => 'id,des'), $fieldArr);
            if(!empty($news_last)){
                if($this->config['sy_news_rewrite'] == 2){
                    $news_last['url'] =	$this->config['sy_weburl'].'/news/'.date('Ymd', $news_last['datetime']).'/'.$news_last['id'].'.html';
                }else{
                    $news_last['url'] = Url('article', array('c' => 'show', 'id' => $news_last['id']), 1);
                }
            }
            //获取后一篇文章
            $news_next = $articleM->getInfo(array('id' => array('>', $id), 'orderby' => 'id,asc'), $fieldArr);
            if(!empty($news_next)){
                if($this->config['sy_news_rewrite'] == 2){
                    $news_next['url'] =	$this->config['sy_weburl'].'/news/'.date('Ymd', $news_next['datetime']).'/'.$news_next['id'].'.html';
                }else{
                    $news_next['url'] =	Url('article',array('c'=>'show', 'id' => $news_next['id']), 1);
                }
            }
            //相关文章,按照关键字获取
            if($news["keyword"]!=""){
                $keyarr = @explode(",",$news["keyword"]);
                if(is_array($keyarr) && !empty($keyarr)){
                    $where['PHPYUNBTWSTART_A'] = '' ;
                    foreach($keyarr as $key=>$value){
                        $where['keyword'][] = array('like',$value,'OR') ;
                    }
                    $where['PHPYUNBTWEND_A'] = '' ;
                    $where['id'] = array('<>',$id);
                    $where['orderby'] =	'id,desc';
                    $where['limit'] = 6;
                    $aboutlist = $articleM->getList($where);//相关文章
                    $about = $aboutlist['list'];
                    if(is_array($about)){
                        foreach($about as $k=>$v){
                            if($this->config['sy_news_rewrite'] == "2"){
                                $about[$k]["url"] =	$this->config['sy_weburl']."/news/".date("Ymd", $v["datetime"]) . "/" . $v['id'] . ".html";
                            }else{
                                $about[$k]["url"] =	Url('article',array('c' => 'show',"id" => $v['id']),"1");
                            }
                        }
                    }
                }
            }
            //新闻类别
            $class = $articleM->getGroup(array('id' => $news['nid']));
            $info =	$news;
            $data['news_title'] = $news['title'];//新闻名称
            $data['news_keyword'] =	$news['keyword'];//描述
            $data['news_class'] = $class['name'];//新闻类别
            $data['news_desc'] = $this->GET_content_desc($news['description']);//描述
            $this->data = $data;
            $info['news_class'] = $class['name'];
            $info['last'] =	$news_last;
            $info['next'] =	$news_next;
            $info['like'] =	$about;
	        $this -> yunset('Info', $info);
	        $this -> yunset('ishtml', 1);
            $this->seo('news_article');
            global $phpyun;
            //必须传参数$cache_id,否则多个文件的内容会重复
            $contect = $phpyun->fetch(TPL_PATH.'default/article/show.htm', $id);
            if(!file_exists(APP_PATH.'news/'.date("Ymd",$news["datetime"]))){
                mkdir(APP_PATH.'news/'.date('Ymd', $news['datetime']));
            }
            $fp = fopen(APP_PATH.'news/'.date("Ymd",$news["datetime"]).'/'.$id.'.html', 'w');
            fwrite($fp, $contect);
            fclose($fp);
        }
    }
    /**
     * 内容 - 新闻 - 新闻管理
     * 删除新闻
     */
    public function delnews_action(){
        $del = $_POST['del'];
        if(is_array($del)){
            $linkid	= pylode(',', $del);
        }else{
            $linkid = $del;
        }
        if(empty($linkid)){
            $this->render_json(1, '请选择您要删除的信息！');
        }
        $articleM =	$this->MODEL('article');
        $articleM->delNews(array('id' => array('in', $linkid)));
        $this->admin_json(0, '新闻(ID:'.$linkid.')删除成功！');
    }

    /**
     * 内容 - 新闻 - 新闻管理
     * 转移分类 -> 保存数据
     * 2019-06-11 hjy
     */
    public function changeClass_action(){
        $_POST = $this->post_trim($_POST);
        if(empty($_POST['id'])){
            $this->render_json(1, '参数不全请重试！');
        }
        $ids = @explode(',', $_POST['id']);
        $id = pylode(',', $ids);
        $nid = intval($_POST['nid']);
        if(!empty($id)){
            $articleM =	$this->MODEL('article');
            $articleM->upBase(array('id' => array('in', $id)), array('nid' => $nid));
            $this->admin_json(0, '新闻转移类别成功！');
        }else{
            $this->render_json(1, '请正确选择需转移的新闻！');
        }
    }

    /**
     * 内容 - 新闻 - 新闻类别
     * 新闻类别
     */
    public function group_action()
    {
        $articleM = $this->MODEL('article');
        $listNew = $articleM->getClass(array('isson' => 1, 'orderby' => 'sort'));
        $news_group = $listNew['list'];
        //获取新闻分类的文章数量
        $newsCountData = array();
        $newsCountData['nid'] = array('in', pylode(',', array_keys($news_group)));
        $newsCountData['groupby'] = 'nid';
        $newsFieldData = array('field' => '`nid`, COUNT(*) AS nums');
        $numsRes = $articleM->getList($newsCountData, $newsFieldData);
        $groupNewsnum = array();
        if (!empty($numsRes['list'])) {
            foreach ($numsRes['list'] as $nv) {
                if (isset($news_group[$nv['nid']])) {
                    $groupNewsnum[$nv['nid']] = $nv['nums'];
                    if ($news_group[$nv['nid']]['keyid'] != 0) {
                        $groupNewsnum[$news_group[$nv['nid']]['keyid']] += $nv['nums'];
                    }
                }
            }
        }
        foreach ($news_group as $key => $value) {
            if (isset($listNew['two_class'][$value['id']])) {
                $news_group[$key]['roots'] = count($listNew['two_class'][$value['id']]);
            } else {
                $news_group[$key]['roots'] = 0;
            }
            if (isset($groupNewsnum[$value['id']])) {
                $news_group[$key]['count'] = $groupNewsnum[$value['id']];
                $news_group[$key]['url'] = Url($_POST['m'], array('cate' => $value['id']), 'admin');
            }
        }
        /***类别end******/
        //导航
        $naviM = $this->MODEL('navigation');
        $type = $naviM->getNavTypeList();
        foreach ($news_group as $k => $v) {
            $news_group[$k]['rec_news'] = $v['rec_news'] == 1 ? true : false;
            $news_group[$k]['rec'] = $v['rec'] == 1 ? true : false;
            if ($v['keyid']) {
                $news_group[$v['keyid']]['children'][] = $v;
                unset($news_group[$k]);
            }
        }
        $this->render_json(0, '', array(
            'list' => array_values($news_group),
            'type' => $type
        ));
    }

    /**
     * 内容 - 新闻 - 新闻类别
     * 添加新闻类别
     */
    public function addgroup_action(){
        $_POST = $this->post_trim($_POST);
        $position =	explode('-', $_POST['name']);
        foreach ($position as $val){
            $name[] = $val;
        }
        if(empty($name)){
            $this->render_json(1, '类别名称不能为空');
        }
        $articleM =	$this->MODEL('article');
        $newsclass = $articleM->getClass(array('name' => array('in', implode(",", $name))));
        if (!empty($newsclass)) {
            $this->render_json(1, '已有此类别，请重新输入！');
        }
        $fid = intval($_POST['fid']);
        $rec = intval($_POST['rec']);
        foreach ($name as $key=>$val){
            $groupAdd =	array();
            $groupAdd['name'] =	$val;
            $groupAdd['keyid'] = $fid;
            if($fid == 0){//一级分类
                $groupAdd['rec'] = $rec;
            }
            $add = $articleM->addGroup($groupAdd);
        }
        $this->get_cache();
        if ($add) {
            $this->admin_json(0, '新闻类别添加成功');
        } else {
            $this->render_json(1, '新闻类别添加失败');
        }
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * type rec_news:新闻首页，rec:首页是否推荐
     * 设置新闻首页
     */
    public function recommend_action(){
        $articleM =	$this->MODEL('article');
        $nid = $articleM->updGroup(array('id' => $_POST['id'], 'keyid'=>'0'), array('' . $_POST['type'] . '' => intval($_POST['rec'])));
        $this->get_cache();
        if ($_POST['type'] == 'rec_news') {
            $type = '新闻首页';
        } else {
            $type = '首页是否推荐';
        }
        $this->render_json($nid ? 0 : 1, $nid ? $type . '修改成功' : $type . '修改失败');
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * 删除类别、批量删除类别
     */
    public function delgroup_action(){
        if(isset($_POST['del'])){
            $articleM =	$this->MODEL('article');
            $result = $articleM->delGroup($_POST['del']);
            if($result['errcode'] == 9){
                $this->get_cache();
                $this->admin_json(0, $result['msg']);
            }else{
                $this->render_json(1, $result['msg']);
            }
        }
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * ajax修改
     */
    public function ajax_action(){
        $articleM =	$this->MODEL('article');
        $idStr = intval($_POST['id']);
        if(empty($idStr)){
            $this->render_json(1, '参数错误');
        }
        $row = $articleM->getGroup(array('id' => array('=', $idStr)));
        if(empty($row)){
            $this->render_json(1, '数据错误');
        }
        $_POST = $this->post_trim($_POST);
        if(isset($_POST['sort'])&& $_POST['sort'] >= 0){//修改排序
            $articleM->updGroup(array('id' => array('=', $idStr)), array('sort' => $_POST['sort']));
            $msg = '新闻类别(ID:'.$idStr.')修改排序成功';
        }
        if($_POST['name']){//修改类别名称
            if($row['is_menu'] == 1){
                $naviM = $this->MODEL('navigation');
                $naviM->upNav(array('name' => $_POST['name']), array('news' => array('=', $idStr)));
                $this->menu_cache_action();
            }
            $articleM->updGroup(array('id' => array('=', $idStr)), array('name' => $_POST['name']));
            $msg = '新闻类别(ID:'.$idStr.')修改名称成功';
        }
        $this->get_cache();
        $this->admin_json(0, $msg);
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * 更新缓存
     */
    public function make_cache_action(){
        $result = $this->get_cache();
        $this->render_json($result ? 0 : 1, $result ? "更新成功！" : "更新失败！");
    }
    public function get_cache(){
        include_once(LIB_PATH.'cache.class.php');
        $cacheclass = new cache(PLUS_PATH, $this -> obj);
        return $makecache = $cacheclass->group_cache('group.cache.php');
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * 设为导航 -> 获取导航信息
     */
    public function ajax_menu_action(){//获取导航
        $idStr = intval($_POST['id']);
        $arr = array();
        if(empty($idStr)){
            echo urldecode(json_encode($arr));die;
        }
        $articleM =	$this->Model('article');
        $row = $articleM->getGroup(array('id' => array('=', $idStr)));
        if($row['is_menu'] == 1){
            $naviM = $this->MODEL('navigation');
            $info =	$naviM->getNav(array('news' => array('=', $idStr)));
            $type =	$naviM->getNavType(array('id' => array('=', $info['nid'])));
            $arr['id'] = $info['id'];
            $arr['nid']	= $info['nid'];
            $arr['name'] = $info['name'];
            $arr['typename'] = $type['typename'];
            $arr['color'] =	$info['color'];
            $arr['url']	= $info['url'];
            $arr['furl'] = $info['furl'];
            $arr['type'] = $info['type'];
            $arr['sort'] = $info['sort'];
            $arr['eject'] =	$info['eject'];
            $arr['model'] =	$info['model'];
            $arr['bold'] = $info['bold'];
            $arr['display'] = $info['display'];
        }else{
            $arr['name'] = $row['name'];
            $arr['url']	= 'news/'.$row['id'].'/';
            $arr['furl'] = 'article/c_list-nid_'.$row['id'].'.html';
        }
        echo urldecode(json_encode($arr));die;
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * 设为导航 -> 保存数据
     */
    public function set_menu_action(){//设置导航
        if(empty($_POST['submit'])){
            $this->render_json(1, "参数错误！");
        }
        $_POST = $this->post_trim($_POST);
        $idStr = intval($_POST['id']);
        $navData = array();
        $navData['name'] = array('=', $_POST['name']);
        $navData['nid'] = array('=', $_POST['nid']);
        if(!empty($idStr)){
            $navData['id'] = array('<>', $idStr);
        }
        $naviM = $this->MODEL('navigation');
        $row = $naviM->getNav($navData);
        if(empty($row)){
            $addData = $_POST;
            $addData['url'] = str_replace("amp;", "", $_POST['url']);
            if(!empty($idStr)){
                $nbid =	$naviM->upNav($addData, array('id' => array('=', $idStr)));
                $msg = '新闻类别导航更新';
            }else{
                $addData['news'] = $_POST['did'];
                $nbid =	$naviM->addNav($addData);
                $articleM =	$this->Model('article');
                $articleM->updGroup(array('id' => array('=', $_POST['did'])), array('is_menu' => 1));
                $msg = '新闻类别导航添加';
            }
            $this->menu_cache_action();
            if(!empty($nbid)){
                $this->admin_json(0, $msg."成功！");
            }else{
                $this->render_json(1, $msg."失败！");
            }
        }else{
            $this->render_json(1, '已经存在此导航！');
        }
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * 取消导航
     */
    public function delmenu_action(){
        $idStr = intval($_POST['id']);
        if(empty($idStr)){
            $this->render_json(1, '非法操作');
        }
        $articleM =	$this->Model('article');
        $articleM->updGroup(array('id' => array('=', $idStr)), array('is_menu' => 0));
        $naviM = $this->MODEL('navigation');
        $naviM->delNav(array('news' => array('=', $idStr)));
        $this->menu_cache_action();
        $this->admin_json(0, '新闻类别导航('.$idStr.')取消成功');
    }
    /**
     * 内容 - 新闻 - 新闻类别
     * 导航缓存
     */
    public function menu_cache_action(){
        include_once(LIB_PATH.'cache.class.php');
        $cacheclass	= new cache(PLUS_PATH,$this->obj);
        $makecache = $cacheclass->menu_cache('menu.cache.php');
    }

    /**
     * 内容 - 新闻 - 新闻类别
     * 转移分类 -> 保存数据
     */
    public function changeSon_action(){
        $_POST = $this->post_trim($_POST);
        if(empty($_POST['id'])){
            $this->render_json(1, '参数不全请重试！');
        }
        $idss =	@explode(',', $_POST['id']);
        $nid = intval($_POST['nid']);
        if(in_array($nid, $idss)){
            $this->render_json(1, '一级类别不能转移到本类别之下！');
        }
        if(empty($idss)){
            $this->render_json(1, '请正确选择需转移的类别！');
        }
        $articleM =	$this->MODEL('article');
        $articleM->updGroup(array('id' => array('in', pylode(',', $idss))), array('keyid' => $nid));
        $this->get_cache();
        $this->admin_json(0, '转移类别成功！');
    }
    /**
     * 内容 - 新闻 - 新闻属性
     * 属性列表
     */
    public function type_action(){
        $articleM =	$this->MODEL('article');
        $keyword = trim($_POST['keyword']);
        if($keyword){
            if ($_POST['type'] == '1') {
                $where['name'] = array('like', $keyword);
            } elseif ($_POST['type'] == '2') {
                $where['value'] = array('like', $keyword);
            }
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('property', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']) {
                $where['orderby'] =	$_POST['t'] . ',' . $_POST['order'];
            }else{
                $where['orderby'] =	'id';
            }
            $where['limit'] = $pages['limit'];
            $List =	$articleM->getProperty($where);
        }
        $data['list'] = $List ? $List : array();
        $data['total'] = intval($pages['total']);
        $data['perPage'] = $pageSize;
        $data['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $data);
    }
    /**
     * 内容 - 新闻 - 新闻属性
     * 保存属性
     */
    public function property_action(){
        $articleM =	$this->MODEL('article');
        if($_POST['id']){
            $whereData['id'] = intval($_POST['id']);
        } else {
            $whereData = array();
        }
        $addArr = $articleM->addProperty(array('name' => $_POST['name'],'value' => $_POST['value']), $whereData);
        if ($addArr['errcode'] == 9) {
            $this->admin_json(0, $addArr['msg']);
        } else {
            $this->render_json(1, $addArr['msg']);
        }
    }
    /**
     * 内容 - 新闻 - 新闻属性
     * 删除属性
     */
    public function delpro_action(){
        $articleM =	$this->Model('article');
        $delID = $_POST['del'];
        $addArr = $articleM->delProperty($delID);
        if ($addArr['errcode'] == 9) {
            $this->admin_json(0, $addArr['msg']);
        } else {
            $this->render_json(1, $addArr['msg']);
        }
    }
}