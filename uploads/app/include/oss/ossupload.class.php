<?php
/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2022 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

require_once 'autoload.php';
require_once 'Config.php';

use OSS\OssClient;
use OSS\Core\OssException;


class ossUpload
{

    const endpoint = Config::OSS_ENDPOINT;          //  img.douxu.com 图片服务器别名路径
    const accessKeyId = Config::OSS_ACCESS_ID;      //  阿里OSS ACCESS
    const accessKeySecret = Config::OSS_ACCESS_KEY; //  阿里OSS ACCESSkey
    const bucket = Config::OSS_TEST_BUCKET;         //  BUCKET 名称 即目录 或者 盘符
    const userdomain = Config::OSS_USERDOMAIN;      //  图片配置域名（远程OSS 地址 image.douxu.com）

    private $ossClient;
    private $maxsize;
    private $pic_type;
    private $is_picself;
    private $is_picthumb;

    function __construct($paras = array())
    {

        $this->maxsize      =   $paras['maxsize'];      // 上传大小
        $this->pic_type     =   $paras['pic_type'];     // 图片类型
        $this->is_picself   =   $paras['is_picself'];   // 安全验证
        $this->is_picthumb  =   $paras['is_picthumb'];  // 强制压缩

        //实例化 ossClient true为开启CNAME。CNAME是指将自定义域名绑定到存储空间上。
        $this->ossClient    =   new OssClient(self::accessKeyId, self::accessKeySecret, self::endpoint, false);
    }

    /**
     * 上传本地图片至 OSS
     * @param $file 本地文件路径
     * @param string $dir OSS 目录名称
     * @param array $thumb crop 将图最长边限制在 100 像素，短边按比例处理 | watermark 图片水印
     * @return mixed
     */
    public function uploadImg($file, $dir = 'img', $thumb = array())
    {

        //根据本地文件路径获取后缀名
        $imgType    =   end(@explode('.', $file['name']));

        //判断后缀名是否合法
        if (!$file['tmp_name']) {

            $returnMsg['msg'] = '未找到相关图片';
        } elseif (!in_array(strtolower($imgType), $this->pic_type)) {

            $returnMsg['msg'] = '上传文件类型不符';
        } elseif ($file['size'] > $this->maxsize * 1024) {//判断大小是否超限

            $returnMsg['msg'] = '上传图片太大';
        } else {

            if ($dir) {

                $dir .= '/' . date('Ymd') . '/';
            }

            //重置文件名称 时间戳+随机数
            $newImgName =   time().'C_'.rand(1000, 9999).'.'.$imgType;

            $object     =   $dir.$newImgName;//OSS 新文件名称 可带目录 如 user com 。。

            $filePath   =   $file['tmp_name'];//本地上传文件路径

            try {

                $this->ossClient->uploadFile(self::bucket, $object, $filePath);

                //是否需要缩略 按传入长度取最长边长度，短边等比例压缩。
                if (!empty($thumb['crop'])) {
                    // 图片缩放
                    $newImgName =   time().'C_'.rand(1000, 9999).'.'.$imgType;

                    $Newobject  =   $dir.$newImgName;//OSS 新文件名称

                    // 缩放图片格式。等比例缩放，最长边未$fbl
                    $style      =   "image/resize,l_".$thumb['crop'];
                }

                //  图片水印
                if (!empty($thumb['watermark'])){

                    $watermark  =   $this->base64url_encode($thumb['watermark']['waterimg']);
                    $g          =   'se';

                    $waterPos   =   (int)$thumb['watermark']['waterpos'];

                    if ($waterPos == 1){

                        $g      =   'nw';
                    }elseif ($waterPos == 3){

                        $g      =   'ne';
                    }elseif ($waterPos == 5){

                        $g      =   'center';
                    }elseif ($waterPos == 7){

                        $g      =   'sw';
                    }elseif ($waterPos == 9){

                        $g      =   'se';
                    }

                    if (!empty($style)){

                        $style  .=  "/watermark,image_".$watermark.',t_90,g_'.$g;
                    }else{

                        $newImgName =   time().'C_'.rand(1000, 9999).'.'.$imgType;

                        $Newobject  =   $dir.$newImgName;//OSS 新文件名称

                        $style  =   "image/watermark,image_".$watermark.',t_90,g_'.$g;
                    }
                }

                if ($style) {

                    // 将处理后的图片转存到当前Bucket。
                    $process = $style . '|sys/saveas' . ',o_' . $this->base64url_encode($Newobject) . ',b_' . $this->base64url_encode(self::bucket);

                    $this->ossClient->processObject(self::bucket, $object, $process);

                    $this->delObject($object);

                    $object = $Newobject;
                }
                if (strpos($dir, 'logo') !== false) {

                    $returnMsg['picurl'] = $object;
                } else {

                    $returnMsg['picurl'] = './' . $object;
                }

                return $returnMsg;
            } catch (OssException $e) {

                //错误信息输出
                $returnMsg['msg']   =   $e->getMessage();
            }
        }

        return $returnMsg;
    }


    /**
     * 上传本地文件至 OSS
     * @param $file 本地文件路径
     * @param string $dir OSS 目录名称
     * @return mixed
     */
    public function uploadDoc($file, $dir = 'doc')
    {

        //允许上传文件后缀
        $upTypes = array('doc', 'docx', 'rar', 'zip', 'txt', 'xls', 'xlsx', 'pdf');

        //根据本地文件路径获取后缀名
        $docType = end(@explode('.', $file['name']));

        //判断后缀名是否合法
        if (!$file['tmp_name']) {

            $returnMsg['error'] = '未找到相关文件';
        } elseif (!in_array(strtolower($docType), $upTypes)) {

            $returnMsg['error'] = '仅允许上传 doc,docx,xls,rar,zip,txt格式文件';
        } elseif ($file['size'] > 10 * 1024 * 1024) {//判断大小是否超限 限定10M

            $returnMsg['error'] = '上传文件大小不能超过10M';
        } else {

            if ($dir) {
                $dir    .=  '/'.date('Ymd').'/';
            }
            //重置文件名称 时间戳+随机数
            $newDocName =   time().'C_'.rand(1000, 9999).'.'.$docType;

            $object     =   $dir.$newDocName;//OSS 新文件名称 可带目录

            $filePath   =   $file['tmp_name'];//本地上传文件路径
            $options    =   array();//暂不需要，为空即可

            try {

                $this->ossClient->uploadFile(self::bucket, $object, $filePath, $options);
                $returnMsg['docurl']    =   '/' . $object;
            } catch (OssException $e) {

                //错误信息输出
                $returnMsg['msg']       =   $e->getMessage();
            }
            
        }
        return $returnMsg;
    }

    /**
     * 删除文件
     * @param $fileUrl  OSS 文件存储地址 如 img/xxxx.jpg|array 可传入数组形式 多文件删除
     * @return bool
     */
    public function delObject($fileUrl)
    {

        //批量删除多文件
        if (is_array($fileUrl)) {

            $object =   array();

            foreach ($fileUrl as $value) {

                //判断是否存储的缩略图格式 即带@ 符号
                if (strstr($value, '@')) {

                    $fileUrls = explode('@', $value);
                    $object[] = $fileUrls[0];
                } else {

                    $object[] = $value;
                }
            }
            if ($object && is_array($object)) {

                foreach ($object as $k => $v) {

                    $v = str_replace('//', '', $v);

                    if (substr($v, 0, 1) == '/') {

                        $v = substr($v, 1);
                    }

                    $object[$k] = $v;
                }
            }
            try {

                $this->ossClient->deleteObjects(self::bucket, $object);
                return true;
            } catch (OssException $e) {

                //return $returnMsg['error'] = $e->getMessage();//失败具体原因 仅供调试需要
                return false;
            }
        } else {

            //判断是否存储的缩略图格式 即带@ 符号
            if (strstr($fileUrl, '@')) {

                $fileUrls   =   explode('@', $fileUrl);
                $object     =   $fileUrls[0];
            } else {

                $object     =   $fileUrl;
            }

            $object     =   str_replace('//', '', $object);

            if (substr($object, 0, 1) == '/') {

                $object =   substr($object, 1);
            }

            try {

                $this->ossClient->deleteObject(self::bucket, $object);
                return true;
            } catch (OssException $e) {

                //return $returnMsg['error'] = $e->getMessage();//失败具体原因 仅供调试需要
                return false;
            }
        }
    }

    /**
     * OSS 同项目内文件复制
     * @param $from_object  源文件
     * @param $to_object    复制后新文件
     * @return bool
     */
    function copyObject($from_object, $to_object)
    {

        $to_object  =   str_replace('//', '', $to_object);
        if (substr($to_object, 0, 1) == '/') {

            $to_object  =   substr($to_object, 1);
        }
        $options    =   array();
        try {

            $this->ossClient->copyObject(self::bucket, $from_object, self::bucket, $to_object, $options);
            //return $returnMsg['error'] = 'ok';
            return true;
        } catch (OssException $e) {

            //return $returnMsg['error'] = $e->getMessage();
            return false;
        }
    }

    /**
     * 图片裁剪 缩放 水印 等
     * @param $img
     * @param $dir
     * @param $x        指定裁剪起点横坐标（默认左上角为原点）。
     * @param $y        指定裁剪起点纵坐标（默认左上角为原点）。
     * @param $width    指定裁剪宽度。
     * @param $height   指定裁剪高度。
     * @param int $scale
     * @return mixed
     */
    function cutPic($img, $dir, $x, $y, $width, $height, $scale = 1)
    {

        $object     =   $dir.'/'.date('Ymd').'/'.end(@explode('/', $img));

        //图片裁剪
        if ($dir) {
            $dir    .=  '/'.date('Ymd').'/';
        }

        $imgType    =   end(@explode('.', $object));

        //重置文件名称 时间戳+随机数
        $newImgName =   time().'C_'.rand(1000, 9999).'.'.$imgType;

        $file       =   $dir.$newImgName;//OSS 新文件名称 可带目录

        try {

            // 自定义裁剪
            $style  =   'image/crop,x_'.$x.',y_'.$y.',w_'.$width.',h_'.$height ;
            // 将处理后的图片转存到当前Bucket。
            $process=   $style.'|sys/saveas'.',o_'.$this->base64url_encode($file).',b_'.$this->base64url_encode(self::bucket);

            $this->ossClient->processObject(self::bucket, $object, $process);

            $returnMsg['picurl']    =   './' . $file;

            $this->delObject($object);

            return $returnMsg;
        } catch (OssException $e) {

            //错误信息输出
            $returnMsg['error'] = $e->getMessage();
            return $returnMsg;
        }
    }

    /**
     * base64图片上传
     * @param string $data base64
     * @param string $dir 上传位置
     * @return string
     */
    function imageBase($data, $dir = 'img')
    {

        preg_match('/^(data:\s*image\/(\w+);base64,)/', $data, $result);

        $uimage = str_replace($result[1], '', str_replace('#', '+', $data));
        // 获取图片格式
        if (in_array(strtolower($result[2]), $this->pic_type)) {
            $imgType = $result[2];
        } else {
            $imgType = 'jpg';
        }
        $new_file = time() .'C_'. rand(1000, 9999) . '.' . $imgType;

        try {
            $objdir = $dir . '/' . date('Ymd') . '/';
            $object = $objdir . $new_file;
            // 使用字符串上传，将base64写到图片里
            $this->ossClient->putObject(self::bucket, $object, base64_decode($uimage));

            if (strpos($dir, 'logo') !== false) {
                $returnMsg['picurl'] = $object;
            } else {
                $returnMsg['picurl'] = './' . $object;
            }

        } catch (OssException $e) {
            //错误信息输出
            $returnMsg['msg'] = $e->getMessage();
        }

        return $returnMsg;
    }

    /**
     * 上传本地图片至 OSS,用户编辑器内图片上传
     * $file 本地文件路径
     */
    public function uploadLocalImg($file)
    {
        if (strpos($file, 'data/upload') !== false) {

            $object = strstr($file, 'data/upload');

            try {

                $this->ossClient->uploadFile(self::bucket, $object, $file);

                $returnMsg['picurl'] = './' . $object;

            } catch (OssException $e) {
                //错误信息输出
                $returnMsg['msg'] = $e->getMessage();

            }
            return $returnMsg;
        }
    }

    public function uploadVoice($file, $dir = 'voice')
    {


        $fileType = end(@explode('.', $file['name']));
        if (!$file['tmp_name']) {

            $returnMsg['msg'] = '录音文件不存在，请重试';

        } else {
            if ($dir) {
                $dir .= '/' . date('Ymd') . '/';
            }
            $newVoiceName = time() .'C_'. rand(1000, 9999) . '.' . $fileType;

            $object = $dir . $newVoiceName;


            $filePath = $file['tmp_name'];
            $options = array();

            try {
                $this->ossClient->uploadFile(self::bucket, $object, $filePath, $options);
                $returnMsg['voiceurl'] = '/' . $object;

            } catch (OssException $e) {
                $returnMsg['msg'] = $e->getMessage();

            }
            return $returnMsg;
        }
    }

    private function base64url_encode($data)
    {
        return rtrim(strtr(base64_encode($data), '+/', '-_'), '=');
    }
}

?>