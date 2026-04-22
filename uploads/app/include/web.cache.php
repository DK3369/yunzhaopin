<?php
/**   
 * @param string $cache_dir 缓文件夹
 * @param int $cache_create_time 文件缓存时间
 * @example $cache=new Esj_Cache('./_cache',100) 
 * @example $cache->Read_Cache() 读取缓存并输出
 * @example $cache->creatre_cache() 创建缓存文件(放在文件未尾) 
 * @example $cache->CacheList() 返回所有缓存文件列表
 * @example $cache->Del_Cache() 删除所有缓存文件
 */
ob_start();

class Phpyun_Cache
{

    private $cache_dir = "./cache";
 // cacher文件夹
    private $web_dir = '';
 // 站点目录
    private $cache_time = 3600;
 // cache文件的建立时间
    public function __construct($cache_dir, $web_dir, $cache_time = 3600)
    {
        ob_start();
        $this->web_dir = $web_dir;
        $this->cache_dir = $cache_dir;
        $this->cache_time = $cache_time;
    }

    public function Read_Cache()
    { // 读取缓存
        if (count($_POST) > 0 || $_GET[m] == "ajax" || $_GET[m] == "includejs") {
            return false;
        }
        try {
            if (self::Create_Dir($this->cache_dir)) {
                self::Get_Cache(); // 输出缓存文件信息
            } else {
                echo "缓存文件夹创建失败!";
                return false;
            }
        } catch (Exception $e) {
            echo $e;
            return false;
        }
    }

    private function Exist_Dir($foler)
    { // 测试缓存文件夹是否存在
        if (@file_exists($this->web_dir . "/" . $foler)) {
            return true;
        } else {
            return false;
        }
    }

    private function Create_Dir($foler)
    { // 建立一个新的文件夹
        if (! self::Exist_Dir($foler)) {
            try {
                @mkdir($this->web_dir . "/" . $foler, 0777);
                @chmod($this->web_dir . "/" . $foler, 0777);
                return true;
            } catch (Exception $e) {
                self::Get_Cache(); // 输出缓存
                return false;
            }
            return false;
        } else {
            return true;
        }
    }

    private function Get_Cache()
    { // 读取缓存文件
        $file_name = self::get_CacheName();
        if (@file_exists($file_name) && ((filemtime($file_name) + $this->cache_time) > time())) {
            $content = @file_get_contents($file_name);
            if (isset($content)) {
                if (! empty($content)) {
                    echo $content;
                    ob_end_flush();
                    exit();
                }
            } else {
                echo "缓存文件读取失败";
                exit();
            }
        } elseif (@file_exists($file_name)) {
            
            $this->Del_Cache();
        }
    }

    private function get_CacheName()
    { // 返回文件的名字
        $name = $_SERVER['HTTP_HOST'] . $_SERVER["REQUEST_URI"];
        if ($_COOKIE['cityid']) {
            $name .= $_COOKIE['cityid'] . $_COOKIE['host'];
        }
        $filename = $file_name = $this->web_dir . '/' . $this->cache_dir . '/' . md5($name) . ".html";
        return $filename;
    }

    public function CacheCreate()
    { // 建立缓存文件
        $filename = self::get_CacheName();
        if ($filename != "") {
            try {
                $op = ob_get_contents();
                if (! empty($op)) {
                    @file_put_contents($filename, $op);
                }
                return true;
            } catch (Exception $e) {
                echo "缓存文件写入失败:" . $e;
                exit();
            }
            return true;
        }
    }

    public function CacheList()
    { // 取得缓存中的所有文件
        $path = $this->cache_dir;
        if ($handle = opendir($path)) {
            while (false !== ($file = readdir($handle))) {
                if ($file != "." && $file != "..") {
                    $path1 = $path . "/" . $file;
                    if (@file_exists($path1)) {
                        $result[] = $file;
                    }
                }
            }
            @closedir($handle);
        }
        return $result;
    }

    public function Del_Cache()
    { // 删除缓存中的所有文件
        $path = $this->web_dir . $this->cache_dir;
        if ($handle = @opendir($path)) {
            while (false !== ($file = @readdir($handle))) {
                if ($file != "." && $file != "..") {
                    $path1 = $path . "/" . $file;
                    if (@file_exists($path1)) {
                        @unlink($path1);
                    }
                }
            }
            @closedir($handle);
        }
        return true;
    }
}
?>