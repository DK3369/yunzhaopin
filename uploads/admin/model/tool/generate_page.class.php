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

class generate_page_controller extends adminCommon
{
    //工具-生成

//region 工具-生成-页面生成：生成首页
    function baseData_action()
    {
        $config = array(
            'make_index_url' => $this->config['make_index_url'],
            'make_new_url' => $this->config['make_new_url'],
        );

        $news_group_rows = $this->MODEL('article')->getClass();

        $descM = $this->MODEL('description');
        $description_rows = $descM->getDesList(array(), array('field' => "`id`,`name`"));

        $this->render_json(0, '', array(
            'config' => $config,
            'news_group_list' => array_values($news_group_rows['list']),
            'description_list' => array_values($description_rows),
        ));
    }

    function index_action()
    {
        if ($_POST["madeall"]) {
            $configM = $this->MODEL('config');
            if ($this->config['sy_web_site'] == 1) {
                $index = '../index.html';
                if (file_exists($index)) {
                    @unlink($index);
                }
                $this->render_json(8, "分站已开启，不支持生成首页静态！");
            } else {
                $fw = $this->webindex($_POST['make_index_url']);
                $configM->setConfig(array('make_index_url' => $_POST['make_index_url']));
                $this->web_config();
                $fw ? $this->admin_json(0, '生成首页成功！') : $this->render_json(1, '生成失败！');
            }
        }
    }

    private function webindex($path)
    {
        global $phpyun;
        if ($this->config['sy_jobdir'] != "") {
            $jobclassurl = $this->config['sy_weburl'] . "/index.php?m=job&c=search&";
        } else {
            $jobclassurl = $this->config['sy_weburl'] . "/index.php?m=job&c=search&";
        }
        global $ModuleName;
        $ModuleName = 'index';
        $this->yunset("jobclassurl", $jobclassurl);
        $this->yunset("ishtml", '1');
        $this->yunset("tplindex", '1');
        $this->yunset("admincache", '1');
        $CacheM = $this->MODEL('cache');
        $CacheList = $CacheM->GetCache(array('job', 'city', 'com', 'user', 'hy'));
        $this->yunset($CacheList);
        $this->seo("index");
        //必须传参数$cache_id,否则多个文件的内容会重复
        $contect = $phpyun->fetch(TPL_PATH . $this->config['style'] . '/index/index.htm', 'abc');

        $fp = fopen($path, "w");
        $fw = fwrite($fp, $contect);
        fclose($fp);
        return $fw;
    }
//endregion

//region 工具-生成-页面生成：生成新闻首页
    function news_action()
    {
        if ($_POST["madeall"]) {
            $configM = $this->MODEL('config');
            set_time_limit(200);
            if ($this->config['sy_web_site'] == 1) {
                $index = '../news.html';
                if (file_exists($index)) {
                    @unlink($index);
                }
                $this->render_json(8, "分站已开启，不支持生成文件！");
            } else {
                $fw = $this->articleindex($_POST["make_new_url"]);
                $configM->setConfig(array('make_new_url' => $_POST['make_new_url']));
                $this->web_config();
                $fw ? $this->admin_json(0, "生成新闻首页(ID:$fw)成功！") : $this->render_json(1, "生成失败！");
            }
        }
    }

    private function articleindex($path)
    {
        $this->seo("news");
        global $phpyun;//必须传参数$cache_id,否则多个文件的内容会重复
        $this->yunset("ishtml", '1');
        $contect = $phpyun->fetch(APP_PATH . "/app/template/" . $this->config['style'] . "/article/index.htm");
        $path = $this->format_url($path);
        $DirList = explode('/', $path);
        $Dir = '';
        foreach ($DirList as $k => $v) {
            $Dir .= $v . '/';
            if (!is_dir(APP_PATH . $Dir) && !strstr($Dir, '.html')) {
                mkdir(APP_PATH . $Dir, 0777);
            }
        }
        $fp = fopen(APP_PATH . $path, "w");
        $fw = fwrite($fp, $contect);
        fclose($fp);
        return $fw;
    }
//endregion

//region 工具-生成-页面生成：生成新闻详情首页
    function archive_action()
    {
        if ($_POST['action'] == "makearchive") {
            set_time_limit(200);
            $pagesize = $_POST['limit'];
            $page = $this->mk_archive($pagesize);
            if ($page) {
                if ($page != 1) {
                    $npage = $page;
                    $page = $page - 1;
                    $spage = $page * $pagesize;
                    $topage = $spage + $pagesize;
                } else {
                    $npage = $page;
                    $spage = $page;
                    $topage = $pagesize;
                }
                $name = $spage . "-" . $topage;
                $this->render_json(0, '正在生成' . $name . '新闻', array('type' => 'archive', 'value' => $npage));
            } else {
                $this->render_json(0, '全部生成完成', array('type' => 'ok', 'value' => 0));
            }
        }
    }

    private function mk_archive($pagesize)
    {
        $articleM = $this->MODEL('article');
        if ($_POST['value'] == 0) {
            $stime = strtotime($_POST['stime']);
            $etime = strtotime($_POST['etime']);
            if ($stime && preg_match("/^\d*$/", $stime)) {

                $where['datetime'][] = array('>=', $stime);
            }
            if ($etime && preg_match("/^\d*$/", $etime)) {

                $where['datetime'][] = array('<=', $etime);
            }
            if ($_POST['group'] > 0) {

                $where['nid'] = $_POST['group'];
            }
            if ($_POST['startid'] > 0) {

                $where['id'][] = array('>=', $_POST['startid']);
            }
            if ($_POST['endid'] > 0) {

                $where['id'][] = array('<=', $_POST['endid']);
            }

            $nlist = $articleM->getList($where, array('field' => "`id`,`datetime`"));

            $news_list = $nlist['list'];
            $allnum = count($news_list);
            $allpage = ceil(($allnum) / $pagesize);
            $i = 1;
            $ncache = array();
            foreach ($news_list as $v) {
                if (count($ncache[$i]) <= $pagesize) {
                    $ncache[$i][$v['id']] = $v['datetime'];
                } else {
                    $i++;
                    $ncache[$i][$v['id']] = $v['datetime'];
                }
            }
            if ($ncache && is_array($ncache)) {
                made_web("../data/plus/news.cache.php", ArrayToString($ncache), "newscache");
                $page = 1;
            }
        } else {
            $page = $_POST['value'];
            include_once(PLUS_PATH . "news.cache.php");
            if (is_array($newscache)) {
                foreach ($newscache as $k => $va) {
                    if ($k == $page) {
                        $index = 0;
                        foreach ($va as $key => $value) {
                            $NewsIDList[] = $key;
                        }
                    } elseif ($k > $page) {
                        $val[$k] = $va;
                    }
                }
            }
            $where['id'] = array('in', implode(',', $NewsIDList));

            $where['orderby'] = array('id,desc');
            $nlist = $articleM->getList($where, array('content' => '1'));

            $news_list = $nlist['list'];

            foreach ($news_list as $k1 => $v1) {
                $this->articleshow($v1['id'], $v1['datetime'], $v1, $news_list[$k1 + 1], $news_list[$k1 - 1]);
            }
            $page = $page + 1;
            if (!is_array($val)) {
                $page = 0;
                unlink("../data/plus/news.cache.php");
            }
        }
        return $page;
    }

    //通过smarty缓存直接生成静态文件
    private function articleshow($id, $datetime, $news, $news_next, $news_last)
    {
        $M = $this->MODEL('article');

        if (!empty($news_last)) {
            if ($this->config['sy_news_rewrite'] == "2") {
                $news_last["url"] = $this->config['sy_weburl'] . "/news/" . date("Ymd", $news_last["datetime"]) . "/" . $news_last['id'] . ".html";
            } else {
                $news_last["url"] = Url('article', array('c' => 'show', "id" => $news_last['id']), "1");
            }
        }
        if (!empty($news_next)) {
            if ($this->config['sy_news_rewrite'] == "2") {
                $news_next["url"] = $this->config['sy_weburl'] . "/news/" . date("Ymd", $news_next["datetime"]) . "/" . $news_next['id'] . ".html";
            } else {
                $news_next["url"] = Url('article', array('c' => 'show', "id" => $news_next['id']), "1");
            }
        }
        $class = $M->getGroup(array("id" => $news['nid']));
        //相关文章,按照关键字获取
        if ($news["keyword"] != "") {

            $keyarr = @explode(",", $news["keyword"]);
            if (is_array($keyarr) && !empty($keyarr)) {
                $where['PHPYUNBTWSTART_A'] = '';
                foreach ($keyarr as $key => $value) {
                    $where['keyword'][] = array('like', $value, 'OR');
                }
                $where['PHPYUNBTWEND_A'] = '';
                $where['id'] = array('<>', $id);
                $where['orderby'] = 'id,desc';
                $where['limit'] = 6;

                $aboutlist = $M->getList($where);//相关文章
                $about = $aboutlist['list'];

                if (is_array($about)) {
                    foreach ($about as $k => $v) {
                        if ($this->config['sy_news_rewrite'] == "2") {

                            $about[$k]["url"] = $this->config['sy_weburl'] . "/news/" . date("Ymd", $v["datetime"]) . "/" . $v['id'] . ".html";

                        } else {
                            $about[$k]["url"] = Url('article', array('c' => 'show', "id" => $v['id']), "1");
                        }
                    }
                }
            }
        }
        $info = $news;
        $data['news_title'] = $news['title'];//新闻名称
        $data['news_keyword'] = $news['keyword'];//描述
        $data['news_class'] = $class['name'];//新闻类别
        $data['news_desc'] = $this->GET_content_desc($news['description']);//描述
        $this->data = $data;
        $info["news_class"] = $class['name'];
        $info["last"] = $news_last;
        $info["next"] = $news_next;
        $info["like"] = $about;
        $info['content'] = htmlspecialchars_decode($info['content']);
        $this->yunset("Info", $info);
        $this->yunset("ishtml", '1');
        $this->seo("news_article");
        global $phpyun;
        //必须传参数$cache_id,否则多个文件的内容会重复
        if (!is_dir(APP_PATH . 'news/' . date('Ymd', $news['datetime']) . '/')) @mkdir(APP_PATH . 'news/' . date('Ymd', $news['datetime']) . '/');
        @chmod(APP_PATH . 'news/' . date('Ymd', $news['datetime']) . '/', 0777);
        $contect = $phpyun->fetch(TPL_PATH . $this->config['style'] . '/article/show.htm', $id);
        $fp = fopen(APP_PATH . 'news/' . date('Ymd', $news['datetime']) . '/' . $id . '.html', "w");
        fwrite($fp, $contect);
        fclose($fp);
    }
//endregion

//region 工具-生成-页面生成：生成单页面
    function once_action()
    {
        if ($_POST['make']) {
            $descM = $this->MODEL('description');
            set_time_limit(200);
            $where = array();
            $where['is_type'] = '1';
            if ($_POST['desc'] > 0) {
                $where['id'] = $_POST['desc'];
            }
            $rows = $descM->getDesList($where);
            if (@is_array($rows)) {
                foreach ($rows as $row) {
                    $this->descriptionshow($row['id'], $row['url']);
                }
            }
            $this->render_json(0, '生成单页面成功！');
        }
    }

    //通过smarty缓存直接生成静态文件
    private function descriptionshow($id, $path)
    {
        $M = $this->MODEL('description');
        $row = $M->getDes(array("id" => $id));
        $top = "";
        $footer = "";
        if ($row['top_tpl'] == 1) {
            $top = APP_PATH . "/app/template/" . $this->config['style'] . "/header.htm";
        } else if ($row['top_tpl'] == 3) {
            $top = APP_PATH . "/app/template/" . $row['top_tpl_dir'] . '.htm';
        }
        if ($row['footer_tpl'] == 1) {
            $footer = APP_PATH . "/app/template/" . $this->config['style'] . "/footer.htm";
        } else if ($row['footer_tpl'] == 3) {
            $footer = APP_PATH . "/app/template/" . $row['footer_tpl_dir'] . '.htm';
        }
        $seo['title'] = $row['title'];
        $seo['keywords'] = $row['keyword'];
        $seo['description'] = $row['descs'];
        $this->yunset("seo", $seo);
        $this->yunset("name", $row['name']);
        $this->yunset("content", $row['content']);
        $this->header_desc($row['title'], $row['keyword'], $row['descs']);
        $make = APP_PATH . "/app/template/" . $this->config['style'] . "/make.htm";
        $make_top = APP_PATH . "/app/template/" . $this->config['style'] . "/make_top.htm";

        global $phpyun;
        //必须传参数$cache_id,否则多个文件的内容会重复
        if ($make_top) {
            $contect = $phpyun->fetch($make_top, $id);
        }
        if ($top) {
            $contect .= $phpyun->fetch($top, $id);
        }
        if ($make) {
            $contect .= $phpyun->fetch($make, $id);
        }
        if ($footer) {
            $contect .= $phpyun->fetch($footer, $id);
        }
        $DirList = explode('/', $path);
        $Dir = '';
        foreach ($DirList as $k => $v) {
            $Dir .= $v . '/';
            if (!is_dir(APP_PATH . $Dir) && !strstr($Dir, '.html')) {
                mkdir(APP_PATH . $Dir, 0777);
            }
        }
        $fp = fopen(APP_PATH . $path, "w");
        fwrite($fp, $contect);
        fclose($fp);
    }
//endregion

//region 工具-生成-生成缓存

//endregion

//region 工具-生成-页面生成：生成新闻类别
    function newsclass_action()
    {
        if ($_POST['action'] == "makeclass") {
            set_time_limit(200);
            $val = $this->mk_newsclass();
            if (is_array($val)) {
                $name = '';
                foreach ($val as $va) {
                    if ($name == "") {
                        $name = $va;
                    }
                }
                $this->render_json(0, "正在生成新闻类别--" . $name, array('type' => "class", 'value' => $val));
            } else {
                $this->render_json(0, "全部生成完成", array('type' => "ok", 'value' => 0));
            }
        }
    }

    private function mk_newsclass()
    {
        if ($_POST['value'] == 0) {
            if ($_POST['group'] != "all" && $_POST['group']) {
                $where['id'] = $_POST['group'];
            }
            $rows = $this->MODEL('article')->getClass($where);
            if (is_array($rows['list'])) {
                foreach ($rows['list'] as $v) {
                    $val[$v['id']] = $v['name'];
                }
            }
        } else {
            $rows = $_POST['value'];
            $nid = '';
            if (is_array($rows)) {
                foreach ($rows as $k => $va) {
                    if ($nid == "") {
                        $nid = $k;
                    } else {
                        $val[$k] = $va;
                    }
                }
            }
            $this->makenewsclass($nid);
        }
        return $val;
    }

    private function makenewsclass($nid)
    {
        include_once(PLUS_PATH . "group.cache.php");
        if ($group_type[$nid]) {
            $nids = $group_type[$nid];

            $nids[] = $nid;

            $where['nid'] = array('in', pylode(',', $nids));

        } else {

            $where['nid'] = $nid;

        }
        $newsnum = $this->MODEL('article')->getNum($where);
        $allpage = ceil($newsnum / 20) > 100 ? 100 : ceil($newsnum / 20);
        $this->articleclass($nid, "../news/" . $nid . "/" . "index.html", 'index');
        for ($i = 1; $i <= $allpage; $i++) {
            if ($allpage >= $i) {
                $fw = $this->articleclass($nid, "../news/" . $nid . "/" . "$i.html", $i);
            }
        }
    }

    private function articleclass($id, $path, $page)
    {
        global $phpyun;
        $_GET['nid'] = $id;
        $_GET['page'] = $page;
        $_GET['cache'] = '1';
        $M = $this->MODEL('article');
        $class = $M->getGroup(array('id' => (int)$_GET['nid']), array('field' => "`name`"));
        $this->yunset("classname", $class['name']);
        $data['news_class'] = $class['name'];
        $this->data = $data;
        $this->seo("newslist");
        $this->yunset("nid", $id);
        $this->yunset("ishtml", '1');

        //必须传参数$cache_id,否则多个文件的内容会重复
        $contect = $phpyun->fetch(APP_PATH . "/app/template/" . $this->config['style'] . "/article/list.htm", 'articleclass-nid' . $id . '-page' . $page);
        $path = $this->format_url($path);
        $DirList = explode('/', $path);
        $Dir = '';
        foreach ($DirList as $k => $v) {
            $Dir .= $v . '/';
            if (!is_dir(APP_PATH . $Dir) && !strstr($Dir, '.html')) {
                mkdir(APP_PATH . $Dir, 0777);
            }
        }
        $fp = fopen(APP_PATH . $path, "w");
        $fw = fwrite($fp, $contect);
        fclose($fp);
        return $fw;
    }
//endregion

//region 工具-生成-页面生成：一键更新
    function all_action()
    {
        if ($_POST['action'] == "makeall") {
            $configM = $this->MODEL('config');
            set_time_limit(200);
            if ($this->config['sy_web_site'] == 1) {
                if ($_POST['type'] == "cache") {
                    include_once(LIB_PATH . "cache.class.php");
                    $cacheclass = new cache(PLUS_PATH, $this->obj);
                    include_once(CONFIG_PATH . "db.data.php");
                    $value = $_POST['value'] + 1;

                    if ($value == 1) {
                        $makecache = $cacheclass->city_cache("city.cache.php");
                    }
                    if ($value == 2) {
                        $makecache = $cacheclass->industry_cache("industry.cache.php");
                    }
                    if ($value == 3) {
                        $makecache = $cacheclass->job_cache("job.cache.php");
                    }
                    if ($value == 4) {
                        $makecache = $cacheclass->user_cache("user.cache.php");
                    }
                    if ($value == 5) {
                        $makecache = $cacheclass->com_cache("com.cache.php");
                        $makecache = $cacheclass->comrating_cache("comrating.cache.php");
                    }
                    if ($value == 7) {
                        $cache = $this->del_dir("../data/templates_c", 1);
                        $cache = $this->del_dir("../data/cache", 1);
                    }
                    if ($value == 8) {
                        $makecache = $cacheclass->seo_cache("seo.cache.php");
                    }
                    if ($value == 9) {
                        $makecache = $cacheclass->menu_cache("menu.cache.php");
                    }
                    if ($value == 10) {
                        $makecache = $cacheclass->part_cache("part.cache.php");
                    }
                    if ($value == 11) {
                        $makecache = $cacheclass->link_cache("link.cache.php");
                    }
                    if ($value == 12) {
                        $makecache = $cacheclass->group_cache("group.cache.php");
                    }
                    if ($value == 13) {
                        $makecache = $cacheclass->redeem_cache("redeem.cache.php");
                    }
                    if ($value == 14) {
                        $adM = $this->MODEL('ad');
                        $makecache = $adM->model_ad_arr();
                    }
                    if ($value == 15) {
                        $makecache = $cacheclass->reason_cache("reason.cache.php");
                    }
                    if ($value == 16) {
                        $makecache = $cacheclass->integralclass_cache("integralclass.cache.php");
                    }

                    if ($value == 18) {
                        $makecache = $cacheclass->moblienav_cache("moblienav.cache.php");
                    }
                    if ($value == 19) {
                        $makecache = $cacheclass->navmap_cache("navmap.cache.php");
                    }
                    if ($value == 20) {
                        $makecache = $cacheclass->ask_cache("ask.cache.php");
                    }
                    if ($value == 23) {
                        $makecache = $cacheclass->introduce_cache("introduce.cache.php");
                    }
                    if ($value == 24) {
                        $makecache = $cacheclass->keyword_cache("keyword.cache.php");
                    }
                    if ($value == 25) {
                        $makecache = $cacheclass->desc_cache("desc.cache.php");
                    }
                    if ($value == 26) {
                        $makecache = $cacheclass->database_cache("dbstruct.cache.php");
                    }
                    if ($value == 27) {
                        $makecache = $cacheclass->emailconfig_cache("emailconfig.cache.php");
                    }
                    if ($value <= 28) {
                        $v = $value + 1;
                        $this->render_json(0, "正在生成" . $arr_data['cache'][$v], array('type' => "cache", 'value' => $value));
                    }
                    $index = '../index.html';
                    $news = '../news.html';
                    if (file_exists($index) || file_exists($news)) {
                        @unlink($index);
                        @unlink($news);
                    }
                    $this->render_json(0, "全部生成完成", array('type' => "ok", 'value' => 0));
                }
            } else {
                if ($_POST['type'] == "cache") {
                    include_once(LIB_PATH . "cache.class.php");
                    $cacheclass = new cache(PLUS_PATH, $this->obj);
                    include_once(CONFIG_PATH . "db.data.php");
                    $value = $_POST['value'] + 1;

                    if ($value == 1) {
                        $makecache = $cacheclass->city_cache("city.cache.php");
                    }
                    if ($value == 2) {
                        $makecache = $cacheclass->industry_cache("industry.cache.php");
                    }
                    if ($value == 3) {
                        $makecache = $cacheclass->job_cache("job.cache.php");
                    }
                    if ($value == 4) {
                        $makecache = $cacheclass->user_cache("user.cache.php");
                    }
                    if ($value == 5) {
                        $makecache = $cacheclass->com_cache("com.cache.php");
                    }
                    if ($value == 6) {
                        $makecache = $cacheclass->domain_cache("domain_cache.php");
                    }
                    if ($value == 7) {
                        $cache = $this->del_dir("../data/templates_c", 1);
                        $cache = $this->del_dir("../data/cache", 1);
                    }
                    if ($value == 8) {
                        $makecache = $cacheclass->seo_cache("seo.cache.php");
                    }
                    if ($value == 9) {
                        $makecache = $cacheclass->menu_cache("menu.cache.php");
                    }
                    if ($value == 10) {
                        $makecache = $cacheclass->part_cache("part.cache.php");
                    }
                    if ($value == 11) {
                        $makecache = $cacheclass->link_cache("link.cache.php");
                    }
                    if ($value == 12) {
                        $makecache = $cacheclass->group_cache("group.cache.php");
                    }
                    if ($value == 13) {
                        $makecache = $cacheclass->redeem_cache("redeem.cache.php");
                    }
                    if ($value == 14) {
                        $adM = $this->MODEL('ad');
                        $makecache = $adM->model_ad_arr();
                    }
                    if ($value == 15) {
                        $makecache = $cacheclass->reason_cache("reason.cache.php");
                    }
                    if ($value == 16) {
                        $makecache = $cacheclass->integralclass_cache("integralclass.cache.php");
                    }

                    if ($value == 18) {
                        $makecache = $cacheclass->moblienav_cache("moblienav.cache.php");
                    }
                    if ($value == 19) {
                        $makecache = $cacheclass->navmap_cache("navmap.cache.php");
                    }
                    if ($value == 20) {
                        $makecache = $cacheclass->ask_cache("ask.cache.php");
                    }
                    if ($value == 23) {
                        $makecache = $cacheclass->introduce_cache("introduce.cache.php");
                    }
                    if ($value == 24) {
                        $makecache = $cacheclass->keyword_cache("keyword.cache.php");
                    }
                    if ($value == 25) {
                        $makecache = $cacheclass->desc_cache("desc.cache.php");
                    }
                    if ($value == 26) {
                        $makecache = $cacheclass->database_cache("dbstruct.cache.php");
                    }
                    if ($value == 27) {
                        $makecache = $cacheclass->emailconfig_cache("emailconfig.cache.php");
                    }
                    if ($value <= 28) {
                        $v = $value + 1;
                        $this->render_json(0, "正在生成" . $arr_data['cache'][$v], array('type' => "cache", 'value' => $value));
                    }
                    $fw = $this->webindex($_POST['make_index_url']);

                    $configM->setConfig(array('make_index_url' => $_POST['make_index_url']));

                    $this->web_config();
                    $this->render_json(0, "正在生成首页", array('type' => "index", 'value' => "index"));
                }
                if ($_POST['type'] == "index") {
                    if ($_POST['value'] == "make_index_url") {

                        $fw = $this->webindex($_POST['make_index_url']);

                        $configM->setConfig(array('make_index_url' => $_POST['make_index_url']));

                        $this->web_config();

                        $this->render_json(0, "正在生成新闻首页", array('type' => "index", 'value' => "news"));
                    } else {

                        $this->articleindex($_POST["make_new_url"]);

                        $configM->setConfig(array('make_new_url' => $_POST['make_new_url']));

                        $this->web_config();

                        $this->render_json(0, "正在获取新闻类别数目", array('type' => "class", 'value' => 0));
                    }
                }
                if ($_POST['type'] == "class") {
                    $val = $this->mk_newsclass();
                    if (is_array($val)) {
                        $name = '';
                        foreach ($val as $va) {
                            if ($name == "") {
                                $name = $va;
                            }
                        }
                        $this->render_json(0, "正在生成新闻类别" . $name, array('type' => "class", 'value' => $val));
                    } else {
                        $this->render_json(0, "正在获取新闻详细页数目", array('type' => "archive", 'value' => 0));
                    }
                }
                if ($_POST['type'] == "archive") {
                    $pagesize = "20";
                    $page = $this->mk_archive($pagesize);
                    if ($page) {
                        if ($page != 1) {
                            $npage = $page;
                            $page = $page - 1;
                            $spage = $page * $pagesize;
                            $topage = $spage + $pagesize;
                        } else {
                            $npage = $page;
                            $spage = $page;
                            $topage = $pagesize;
                        }
                        $name = $spage . "-" . $topage;
                        $this->render_json(0, "正在生成" . $name . "新闻", array('type' => "archive", 'value' => $npage));
                    } else {
                        $this->render_json(0, "全部生成完成", array('type' => 'ok', 'value' => 0));
                    }
                }
            }
        }
    }

//endregion
    //多余的代码
    private function articlelist($id, $path)
    {

        $_GET['nid'] = $id;
        $M = $this->MODEL('article');
        $group = $M->GetNewsGroupList(array('keyid' => '0'), array('field' => "`id`,`name`"));
        if (is_array($group)) {
            foreach ($group as $k => $v) {
                if ($this->config[sy_news_rewrite] == "2") {
                    $group[$k]['url'] = $this->config['sy_weburl'] . "/news/" . $v['id'] . "/";
                } else {
                    $group[$k]['url'] = Url("article", array('c' => 'list', "id" => $v[id]), "1");
                }
            }
        }
        $this->yunset("group", $group);
        $this->seo("newslist");
        $this->yunset("ishtml", '1');
        global $phpyun;//必须传参数$cache_id,否则多个文件的内容会重复
        $contect = $phpyun->fetch(APP_PATH . "/app/template/" . $this->config['style'] . "/article/list.htm");
        $path = $this->format_url($path);
        $DirList = explode('/', $path);
        $Dir = '';
        foreach ($DirList as $k => $v) {
            $Dir .= $v . '/';
            if (!is_dir(APP_PATH . $Dir) && !strstr($Dir, '.html')) {
                mkdir(APP_PATH . $Dir, 0777);
            }
        }
        $fp = fopen(APP_PATH . $path, "w");
        $fw = fwrite($fp, $contect);
        fclose($fp);
        return $fw;
    }

    private function format_url($srcurl, $baseurl = '')
    {
        $SplitList = explode('/', $srcurl);
        foreach ($SplitList as $v) {
            switch ($v) {
                case '..':
                    $URL .= '';
                    break;
                case '.':
                    break;
                default:
                    $URL .= '/' . $v;
                    break;
            }
        }
        return $URL;
    }
}