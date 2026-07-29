<?php
class DES_JAVA{
    function __construct($key, $iv=0){
        $this->DES_JAVA($key, $iv);
    }
    var $key;
    var $iv; //偏移量

    function DES_JAVA($key, $iv=0){
        $this->key = $key;
        if($iv == 0){
            $this->iv = $key;
        }else{
            $this->iv = $iv;
        }
    }
    //加密
    function encrypt($str){
        $size = 8;
        $str = $this->pkcs5Pad ( $str, $size );

        $data = openssl_encrypt($str, "DES-CBC", $this->key, OPENSSL_RAW_DATA | OPENSSL_ZERO_PADDING, $this->iv);
        return base64_encode($data);
    }
    //解密
    function decrypt($str){
        $str = base64_decode ($str);
        $str = openssl_decrypt($str, "DES-CBC", $this->key, OPENSSL_RAW_DATA | OPENSSL_ZERO_PADDING, $this->iv);
        $str = $this->pkcs5Unpad( $str );
        return $str;
    }
    function pkcs5Pad($text, $blocksize){
        $pad = $blocksize - (strlen ( $text ) % $blocksize);
        return $text . str_repeat ( chr ( $pad ), $pad );
    }
    function pkcs5Unpad($text){
        $pad = ord ( $text[strlen($text) - 1] );
        if ($pad > strlen ( $text ))
            return false;
        if (strspn ( $text, chr ( $pad ), strlen ( $text ) - $pad ) != $pad)
            return false;
        return substr ( $text, 0, - 1 * $pad );
    }
}
