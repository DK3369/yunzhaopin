<?php
/**
 * i18n 全局翻译函数（PHP）
 *
 * 结构化 key：yun_t('home.search_placeholder')
 * 编号 key：  yun_at('wap_00703') 或 yun_t('common_09190')
 * 中文兜底：  yun_auto_t('成为企业会员，高效挑选人才！')
 */

function yun_i18n()
{
    global $i18n;
    return (is_object($i18n) && $i18n instanceof Yun_I18n) ? $i18n : null;
}

function yun_lang()
{
    $i18n = yun_i18n();
    return $i18n ? $i18n->getLang() : 'zh_cn';
}

function yun_is_auto_key($key)
{
    $i18n = yun_i18n();
    if ($i18n && method_exists($i18n, 'isAutoKey')) {
        return $i18n->isAutoKey($key);
    }
    return is_string($key) && preg_match('/^(common|wap|admin|company|user|ask|member|model)_[0-9]{5}$/', $key);
}

function yun_t($key, $params = array(), $default = '')
{
    $i18n = yun_i18n();
    if ($i18n) {
        return $i18n->t($key, $params, $default);
    }
    return $default !== '' ? $default : $key;
}

function yun_at($key, $params = array(), $default = '')
{
    return yun_t($key, $params, $default);
}

function lc($key, $params = array(), $default = '')
{
    $lookupKey = strpos($key, '.') === false ? 'lc.' . $key : $key;
    return yun_t($lookupKey, $params, $default);
}

function yun_auto_t($text)
{
    $i18n = yun_i18n();
    if ($i18n && method_exists($i18n, 'autoT')) {
        return $i18n->autoT($text);
    }
    return $text;
}

function yun_auto_array($value)
{
    $i18n = yun_i18n();
    if ($i18n && method_exists($i18n, 'autoArray')) {
        return $i18n->autoArray($value);
    }
    return $value;
}

function yun_json_encode($value, $options = 0)
{
    if (function_exists('yun_auto_array')) {
        $value = yun_auto_array($value);
    }
    if (defined('JSON_UNESCAPED_UNICODE')) {
        $options = $options | JSON_UNESCAPED_UNICODE;
    }
    return json_encode($value, $options);
}

function yun_i18n_js_keys(array $keys)
{
    $pack = array();
    foreach ($keys as $key) {
        if ($key === '') {
            continue;
        }
        $pack[$key] = yun_t($key);
    }
    return $pack;
}

function yun_i18n_langpack($keys = null)
{
    $i18n = yun_i18n();
    if (!$i18n) {
        return array(
            'lang' => 'zh_cn',
            'structured' => array(),
            'auto' => array(),
            'lc' => array(),
        );
    }

    $auto = isset($i18n->autoMessages) && is_array($i18n->autoMessages) ? $i18n->autoMessages : array();
    if (is_array($keys) && !empty($keys)) {
        $filtered = array();
        foreach ($keys as $key) {
            if (isset($auto[$key])) {
                $filtered[$key] = $auto[$key];
            } elseif (yun_is_auto_key($key)) {
                $filtered[$key] = yun_t($key);
            }
        }
        $auto = $filtered;
    }

    $structured = isset($i18n->messages) && is_array($i18n->messages) ? $i18n->messages : array();
    unset($structured['_meta']);

    return array(
        'lang' => $i18n->getLang(),
        'structured' => $structured,
        'auto' => $auto,
        'lc' => isset($structured['lc']) && is_array($structured['lc']) ? $structured['lc'] : array(),
    );
}

function yun_i18n_langpack_json($keys = null)
{
    $options = defined('JSON_UNESCAPED_UNICODE') ? JSON_UNESCAPED_UNICODE : 0;
    return json_encode(yun_i18n_langpack($keys), $options);
}

function yun_i18n_build_path($file, $lang = '')
{
    $i18n = yun_i18n();
    if ($lang === '' && $i18n) {
        $lang = $i18n->getLang();
    }
    $lang = preg_replace('/[^a-z_]/', '', strtolower($lang));
    if ($lang === '' || $lang === 'zh_cn' || !defined('DATA_PATH')) {
        return '';
    }
    $file = ltrim($file, '/\\');
    $path = DATA_PATH . 'i18n_build/current/' . $lang . '/' . $file;
    return is_file($path) ? $path : '';
}

function yun_i18n_plus_path($file)
{
    $path = yun_i18n_build_path($file);
    if ($path !== '') {
        return $path;
    }
    return defined('PLUS_PATH') ? PLUS_PATH . ltrim($file, '/\\') : $file;
}

function yun_i18n_plus_style($file = 'job.cache.js')
{
    global $config;
    $base = isset($config['sy_weburl']) ? rtrim($config['sy_weburl'], '/') : '';
    $path = yun_i18n_build_path($file);
    if ($path !== '') {
        return $base . '/data/i18n_build/current/' . yun_lang();
    }
    return $base . '/data/plus';
}

?>
