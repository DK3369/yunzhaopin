<?php
/**
 * i18n 全局翻译函数（PHP）
 *
 * 推荐入口：yun_t('home.search_placeholder')、lc('save')、yun_auto_t('存量中文')、lcCoin(...)
 * 兼容入口：yun_at('wap_00703') 是 yun_t() 的历史别名，保留旧代码兼容；新增代码优先 yun_t()。
 * 内部工具：yun_auto_array() 主要供 yun_json_encode() 等数组输出流程使用。
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
    return is_string($key) && preg_match('/^([a-z][a-z0-9_]*)_([0-9]{5})$/', $key, $m) && count(explode('_', $m[1])) <= 3;
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
    // Historical alias kept for existing numbered keys; new code should prefer yun_t().
    return yun_t($key, $params, $default);
}

function lc($key, $params = array(), $default = '')
{
    $lookupKey = strpos($key, '.') === false ? 'lc.' . $key : $key;
    $text = yun_t($lookupKey, $params, '');
    if ($text !== '' && $text !== $lookupKey) {
        return $text;
    }
    if (strpos($key, '.') === false) {
        $autoText = yun_t($key, $params, '');
        if ($autoText !== '' && $autoText !== $key) {
            return $autoText;
        }
    }
    return $default !== '' ? $default : $key;
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
    // Internal helper for translating arrays before JSON/API output.
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
