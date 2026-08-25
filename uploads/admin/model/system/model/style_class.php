<?php



class style
{

    function __construct($obj)
    {
        $this->obj = $obj;
    }

    /**
     * @desc 获取前台模板风格目录
     */
    function model_list_action()
    {

        $path   =   TPL_PATH;
        $handle =   @opendir($path);
        $list   =   array();

        if ($handle) {
            while (($file = @readdir($handle)) !== false) {

                if ($file == '.' || $file == '..' || $file == '.svn' || $file == 'admin' || $file == 'ask' || $file == 'chat' || $file == 'company' || $file == 'lietou' || $file == 'member' || $file == 'promoter' || $file == 'resume' || $file == 'school' || $file == 'shop' || $file == 'siteadmin' || $file == 'train' || $file == 'im' || $file == 'wap' || $file == 'wapadmin') continue;

                if (is_dir($path . $file)) {

                    $list[] = $file;
                }
            }
            closedir($handle);
        }
        $lists = array();
        if (isset($list) && is_array($list)) {

            foreach ($list as $key => $value) {

                $filepath   =   $path.$value.'/info.txt';

                if (!file_exists($filepath)) {
                    @file_put_contents($filepath, '');
                }

                $text       =   is_file($filepath) ? (string)@file_get_contents($filepath) : '';
                if ($text == '') {

                    $text   =   $value . '||PHPYUN||' . $value . '||../app/template/' . $value . '/images/preview.jpg';
                    @file_put_contents($filepath, $text);
                }

                $content    =   @explode('||', $text);
                $lists[$key]['name']    =   isset($content[0]) ? $content[0] : $value;
                $lists[$key]['author']  =   isset($content[1]) ? $content[1] : '';
                $lists[$key]['dir']     =   !empty($content[2]) ? $content[2] : $value;
                $lists[$key]['img']     =   checkpic(isset($content[3]) ? $content[3] : '');
            }
        }
        return $lists;
    }

    /**
     * @desc 获取风格信息修改
     * @param $dir
     * @return array
     */
    function model_modify_action($dir)
    {
        $path       =   TPL_PATH.$dir.'/info.txt';
        $text       =   is_file($path) ? (string)@file_get_contents($path) : '';
        $content    =   @explode('||', $text);

        return array('name' => $content[0], 'author' => $content[1], 'dir' => $content[2], 'img' => $content[3],);

    }

    /**
     * @param $arr
     */
    function model_save_action($arr)
    {

        $path   =   TPL_PATH.$arr['dir'].'/info.txt';
        $text   =   $arr['name'] . '||' . $arr['author'] . '||' . $arr['dir'] . '||../app/template/' . $arr['dir'] . '/images/preview.jpg';
        @file_put_contents($path, $text);

    }

}