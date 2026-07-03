/**
 * Province/city/district chained select form.
 */
function cityPublicT(key, params, fallback) {
    var text;
    if (typeof yunT === 'function') {
        text = yunT(key, params, fallback);
    } else if (typeof yunAt === 'function') {
        text = yunAt(key, params, fallback);
    } else {
        text = fallback !== undefined ? fallback : key;
    }
    if (params && typeof text === 'string') {
        for (var name in params) {
            if (Object.prototype.hasOwnProperty.call(params, name)) {
                text = text.split('{' + name + '}').join(params[name]);
            }
        }
    }
    return text;
}

function cityPublicDefaultOption() {
    return "<option value=''>" + cityPublicT('city_js_00001', null, 'Please select') + "</option>";
}

layui.use(['form'], function () {
    var $ = layui.$,
        form = layui.form;
    if (ct.length > 0 && ct != 'new Array()') {
        form.on('select(citys)', function (data) {
            var html = cityPublicDefaultOption();
            if (data.value) {
                $.each(ct[data.value], function (k, v) {
                    html += "<option value='" + v + "'>" + cn[v] + "</option>";
                });
            }
            if (data.elem.name == 'provinceid') {
                $("#cityid").html(html);
                $("#three_cityid").html(cityPublicDefaultOption());
            } else if (data.elem.name == 'cityid'){
                $("#cityshowth").show();
                $("#three_cityid").html(html);
            }
            form.render('select');
        });
    } else {
        if ($("#cityid")) {
            $("#cityid").parent().remove();
        }
        if ($("#three_cityid")) {
            $("#three_cityid").parent().remove();
        }
        form.render('select');
    }
});