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
class index_controller extends common{
    /*
     *zip解压
     */
    function deal_zip($file, $todir){
        if (trim($file) == '' || trim($todir) == '') {
            return false;
        }
        // 创建保存目录
        if (!file_exists($todir) && !mkdir($todir, 0777, true)) {
            return false;
        }
        $zip = new ZipArchive();
        // 中文文件名要使用ANSI编码的文件格式
        if ($zip->open($file) === true) {
            // 提取全部文件
            $zip->extractTo($todir);
            $zip->close();
            return true;
        } else {
            return false;
        }
    }

    /**
     * 自动升级
     */
    function index_action(){
        session_start();
        if (!$this->config['sy_update_secret']) {
            echo '请先配置自动升级秘钥！';
            die;
        }
        if (!$_GET['secret'] || $_GET['secret'] != $this->config['sy_update_secret']) {
            echo '参数异常！';
            die;
        }
        $down_path = DATA_PATH . 'update_dir/';// 下载升级包保存地址
        $unzip_path = DATA_PATH . 'update_data/';// 升级包解压地址
        $step = isset($_GET['step']) && $_GET['step'] ? intval($_GET['step']) : 1;
        if (!is_writable(DATA_PATH)) {
            echo '/data目录没有写入权限，请先修改对应目录权限后再试！';
            die;
        }
        include (APP_PATH . '/version.php');
        if ($step == 1) {
            // 定制版不支持自动升级
            if (strpos($version, '定制') !== false) {
                echo '您的系统版本为定制版，请联系客服进行升级！';
                die;
            }
            // 根据当前系统版本请求综合平台获取是否有新版更新包
            $r = CurlPost('https://u.phpyun.com/back/package/getUpdatePack', array('version' => $version, 'appSecret' => $this->config['sy_update_secret']));
            $r = json_decode($r, 1);
            if (intval($r['code']) == 404) {
                echo $r['message'] ? $r['message'] : '自动升级安全码无效';
                die;
            }
            if (!(intval($r['code']) == 200 && $r['data']['path'])) {
                echo '您的系统已经是最新版，暂无可用更新包！';
                die;
            } else {
                $_SESSION['down_url'] = $r['data']['path'];
                echo '<div>发现新版本【'.$r["data"]["version"].'】,确定要从【'.$r["data"]["pre_version"].'】升级到新版吗？<a href="'.$this->config[sy_weburl].'?m=upgrade&step=2">确定</a></div>';
            }
        }elseif ($step == 2){
            // 下载更新包
            $res = $this->down_file($_SESSION['down_url'], $down_path);
            $_SESSION['zip_path'] = $res['save_path'];
            echo '下载完成，<a href="'.$this->config[sy_weburl].'?m=upgrade&step=3">解压</a>';
        } elseif ($step == 3) {
            // 解压下载的新版更新包
            $unzip = $this->deal_zip($_SESSION['zip_path'], $unzip_path);
            if (!$unzip) {
                echo '解压失败！';
                die;
            }
            include (PLUS_PATH . '/admindir.php');
            if ($admindir !== 'admin') {// 用户自定义后台文件夹名称
                is_dir($unzip_path . 'admin') && rename($unzip_path . 'admin', $unzip_path . $admindir);
            }
            echo '解压成功，<a href="'.$this->config[sy_weburl].'?m=upgrade&step=4">升级准备</a>';
        } elseif ($step == 4) {
            $versionArr = explode(' ', $version);
            // 更新包内更新文件合并到现有的系统目录
            $mergeR = $this->copy_merge($unzip_path, APP_PATH, $versionArr[0]);
            if ($mergeR) {
                $this->deldir($unzip_path);
            }
            echo '准备完毕，<a href="'.$this->config[sy_weburl].'/update/index.php">开始升级</a>';
        }
    }

    /*
     * 合并目录且只覆盖不一致的文件
     * $param $source 要合并的文件夹
     * $param $target 要合并的目的地
     * $param $currVersion 升级前phpyun版本
     * $return int 处理的文件数
     */
    function copy_merge($source, $target, $currVersion){
        if (trim($source) == '' || trim($target) == '') {
            return false;
        }
        // 路径处理
        $source = preg_replace('#/\\\\#', DS, $source);
        $target = preg_replace('#\/#', DS, $target);
        $source = rtrim($source, DS) . DS;
        $target = rtrim($target, DS) . DS;
        // 记录处理的文件数
        $cnt = 0;
        // 目标文件夹不存在则创建
        if (!is_dir($target)) {
            mkdir($target, 0777, true);
            $cnt++;
        }
        $cp_path = DATA_PATH  . 'cp_data/' . date('Ymd') . '/' . $currVersion;// 升级备份地址
        // 搜索目录下的所有文件
        foreach (glob ($source . '*') as $filename) {
            if (is_dir($filename)) {
                // 备份文件按更新包目录层级创建文件夹
                $cpDir = preg_replace('#/\\\\#', DS, $filename);
                $cpDir = str_replace(array('/','\\'), DS, $cp_path . str_replace(DATA_PATH . 'update_data', '',$cpDir));
                $cpDir = rtrim($cpDir, DS) . DS;
                if (!is_dir($cpDir)) {
                    mkdir($cpDir, 0777, true);
                }
                // 如果是目录，递归合并子目录的文件
                $cnt += $this->copy_merge($filename, $target . basename($filename), $currVersion);
            } elseif (is_file($filename)) {
                // 如果是文件，判断当前文件与目标文件是否一样，不一样则拷贝覆盖。
                // 如果使用的是文件MD5进行一致性判断，可靠单性能低
                if (!file_exists($target . basename($filename)) || md5(file_get_contents($filename)) != md5(file_get_contents($target . basename($filename)))) {
                    // 已存在的文件有修改时，先备份防止升级失败恢复
                    if (file_exists($target . basename($filename))) {
                        copy($target . basename($filename), str_replace(array('/','\\'), DS, $cp_path . str_replace(DATA_PATH . 'update_data', '',$filename)));
                    }
                    copy($filename, $target . basename($filename));
                    $cnt++;
                }
            }
        }
        return $cnt;
    }

    /*
     * 遍历删除文件
     * $param $dir 要删除的目录
     * $return bool 成功与否
     */
    function deldir($dir){
        if (trim($dir) == '') {
            return false;
        }
        // 先删除目录下的文件
        $dh = opendir($dir);
        while ($file = readdir($dh)) {
            if ($file != '.' && $file != '..') {
                $fullpath = $dir . '/' . $file;
                if (!is_dir($fullpath)) {
                    unlink($fullpath);
                } else {
                    $this->deldir($fullpath);
                }
            }
        }
        closedir($dh);
        // 删除当前文件夹
        if (rmdir($dir)) {
            return true;
        } else {
            return false;
        }
    }

    /*
     * 下载程序压缩包文件
     * $param $url 要下载的url
     * $param $save_dir 要存放的目录
     * $return res 成功返回下载的信息 失败返回false
     */
    function down_file($url, $save_dir){
        if (trim($url) == '' || trim($save_dir) == '') {
            return false;
        }
        $save_dir = str_replace(array('/','\\'), DIRECTORY_SEPARATOR, $save_dir);
        $filename = basename($url);
        // 创建保存目录
        if (!file_exists($save_dir) && !mkdir($save_dir, 0777, true)) {
            return false;
        }
        // 开始下载
        $content = CurlGet($url);
        if (!$content) {
            return false;
        }
        $fp2 = @fopen($save_dir . $filename, 'a');
        fwrite($fp2, $content);
        fclose($fp2);
        unset($content, $url);
        return array(
            'file_name' => $filename,
            'save_path' => $save_dir . $filename
        );
    }
}
?>
