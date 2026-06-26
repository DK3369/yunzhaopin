/**
 * yun-i18n.js — 前端统一翻译（对应 PHP yun_t / yun_at / yun_auto_t）
 *
 * yunT('home.search_placeholder')   结构化 key
 * yunAt('wap_00703')                编号 key（yunT 别名）
 * yunAutoT('成为企业会员…')          中文兜底（auto 语言包）
 * yunLc('save')                     后台 lc 命名空间
 */
(function (global) {
    'use strict';

    var AUTO_KEY_RE = /^[a-z][a-z0-9_]*_[0-9]{5}$/;

    function getStore() {
        global.yunI18n = global.yunI18n || {
            lang: 'zh_cn',
            structured: {},
            auto: {},
            lc: {},
            autoKeys: [],
            langPackUrl: ''
        };
        return global.yunI18n;
    }

    function hasZh(text) {
        return typeof text === 'string' && /[\u4e00-\u9fff]/.test(text);
    }

    function isAutoKey(key) {
        var m = typeof key === 'string' && key.match(AUTO_KEY_RE);
        if (!m) return false;
        return m[1].split('_').length <= 3;
    }

    function buildAutoKeys(store) {
        store.autoKeys = Object.keys(store.auto || {}).sort(function (a, b) {
            return b.length - a.length;
        });
    }

    function getStructured(key, store) {
        if (!key || key.indexOf('.') === -1) {
            return null;
        }
        var parts = key.split('.');
        var value = store.structured;
        for (var i = 0; i < parts.length; i++) {
            if (!value || typeof value !== 'object' || !(parts[i] in value)) {
                return null;
            }
            value = value[parts[i]];
        }
        return typeof value === 'string' ? value : null;
    }

    function replaceParams(text, params) {
        var output = String(text);
        if (!params) {
            return output;
        }
        if (Object.prototype.toString.call(params) === '[object Array]') {
            for (var i = 0; i < params.length; i++) {
                output = output.split('{' + i + '}').join(params[i]);
            }
            return output;
        }
        if (typeof params === 'object') {
            for (var key in params) {
                if (Object.prototype.hasOwnProperty(key)) {
                    output = output.split('{' + key + '}').join(params[key]);
                }
            }
        }
        return output;
    }

    function lookupKey(key, store, fallback) {
        if (!key) {
            return fallback || '';
        }

        var structured = getStructured(key, store);
        if (structured !== null) {
            return structured;
        }

        if (store.auto && store.auto[key]) {
            return store.auto[key];
        }

        if (store.lc) {
            var lcKey = key.indexOf('.') === -1 ? 'lc.' + key : key;
            if (store.lc[lcKey]) {
                return store.lc[lcKey];
            }
            if (store.lc[key]) {
                return store.lc[key];
            }
        }

        return fallback !== undefined && fallback !== '' ? fallback : key;
    }

    function yunT(key, params, fallback) {
        var store = getStore();
        var text = lookupKey(key, store, fallback);
        return replaceParams(text, params);
    }

    function yunAutoT(text) {
        if (text === '' || text === null || text === undefined) {
            return text;
        }

        var store = getStore();
        var source = String(text);
        var leading = source.match(/^\s*/)[0];
        var trailing = source.match(/\s*$/)[0];
        var body = source.replace(/^\s+|\s+$/g, '');

        if (!body) {
            return leading + trailing;
        }

        if (store.lang === 'zh_cn') {
            return text;
        }

        if (store.auto && store.auto[body]) {
            return leading + store.auto[body] + trailing;
        }

        if (!hasZh(body)) {
            return text;
        }

        var keys = store.autoKeys || [];
        for (var i = 0; i < keys.length; i++) {
            var autoKey = keys[i];
            if (autoKey.length > 1 && store.auto[autoKey] && body.indexOf(autoKey) !== -1) {
                body = body.split(autoKey).join(store.auto[autoKey]);
            }
        }

        return leading + body + trailing;
    }

    function yunLc(key, params, fallback) {
        return yunT(key.indexOf('.') === -1 ? 'lc.' + key : key, params, fallback);
    }

    function mergeLangPack(data) {
        var store = getStore();
        if (!data || typeof data !== 'object') {
            return store;
        }
        if (data.lang) {
            store.lang = data.lang;
        }
        if (data.structured && typeof data.structured === 'object') {
            store.structured = data.structured;
        }
        if (data.auto && typeof data.auto === 'object') {
            store.auto = data.auto;
            buildAutoKeys(store);
        }
        if (data.lc && typeof data.lc === 'object') {
            store.lc = data.lc;
        }
        if (data.messages && typeof data.messages === 'object') {
            store.auto = data.messages;
            buildAutoKeys(store);
        }
        return store;
    }

    function yunI18nInit(config) {
        return mergeLangPack(config || {});
    }

    function yunI18nLoad(url, sync) {
        var store = getStore();
        var requestUrl = url || store.langPackUrl;
        if (!requestUrl || store.lang === 'zh_cn') {
            return store;
        }

        var xhr = new XMLHttpRequest();
        xhr.open('GET', requestUrl, sync !== true);
        xhr.onreadystatechange = function () {
            if (xhr.readyState === 4 && xhr.status >= 200 && xhr.status < 300) {
                try {
                    mergeLangPack(JSON.parse(xhr.responseText));
                } catch (e) {}
            }
        };
        try {
            xhr.send(null);
        } catch (e) {}
        return store;
    }

    global.yunT = yunT;
    global.yunAt = yunT;
    global.yunAutoT = yunAutoT;
    global.yunLc = yunLc;
    global.yunI18nInit = yunI18nInit;
    global.yunI18nLoad = yunI18nLoad;
    global.yunI18nMerge = mergeLangPack;

    var boot = getStore();
    buildAutoKeys(boot);
    if (boot.lang !== 'zh_cn' && boot.langPackUrl) {
        yunI18nLoad(boot.langPackUrl, true);
    }
})(typeof window !== 'undefined' ? window : this);
