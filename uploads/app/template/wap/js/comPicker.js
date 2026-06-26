// Get experience options
function getExp() {
    return comFormat('job_exp');
}

// Get education options
function getEdu() {
    return comFormat('job_edu');
}

// Get report-to-work time options
function getReport() {
    return comFormat('job_report');
}

// Get company nature options
function getPr() {
    return comFormat('job_pr');
}

// Get company size options
function getMun() {
    return comFormat('job_mun');
}

function comPickerI18n(key) {
    var i18n = typeof COM_PICKER_I18N !== 'undefined' ? COM_PICKER_I18N : {};
    return i18n[key] || '';
}

// Get gender options (display labels are i18n; ids are API values)
function getSex() {
    return {
        'id': [3, 1, 2],
        'name': [comPickerI18n('noLimit'), comPickerI18n('male'), comPickerI18n('female')]
    };
}
function getSexReq() {
    return {
        'id': [3, 2],
        'name': [comPickerI18n('noLimit'), comPickerI18n('female')]
    };
}

// Get marital status options
function getMarriage() {
    return comFormat('job_marriage');
}

// Get resume remark options
function getremark() {
    return comFormat('job_remark');
}

// Get language requirements
function getLang() {
    var data = [];
    if (typeof comd['job_lang'] !== 'undefined') {
        var arr = comd['job_lang'];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.push({id: val, name: comn[val], checked: false})
        }
    }
    return data;
}

// Format picker data from cache
function comFormat(key) {
    var data = {
        name: [],
        id: []
    };
    if (typeof comd[key] !== 'undefined') {
        var arr = comd[key];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(comn[val]);
            data.id.push(val);
        }
    }

    return data
}

// Get industry options
function getHy(defaultOptionName) {
    if (defaultOptionName) {
        var data = {
            name: [defaultOptionName],
            id: [0]
        };
    } else {
        var data = {
            name: [],
            id: []
        };
    }

    if (typeof hi !== 'undefined') {
        var arr = hi;
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(hyname[val]);
            data.id.push(val);
        }
    }

    return data
}

// Get experience requirement options
function getExpReq() {
    let exp = userFormat('user_word');
    exp.name.unshift(comPickerI18n('selectExperience'));
    exp.id.unshift('0');
    return exp;
}

// Get education requirement options
function getEduReq() {
    let edu = userFormat('user_edu');
    edu.name.unshift(comPickerI18n('selectEducation'));
    edu.id.unshift('0');
    return edu;
}

// Format user cache data for pickers
function userFormat(key) {
    var data = {
        name: [],
        id: []
    };
    if (typeof useri[key] !== 'undefined') {
        var arr = useri[key];
        for (var i = 0; i < arr.length; i++) {
            var val = arr[i];
            data.name.push(usern[val]);
            data.id.push(val);
        }
    }

    return data
}

// Get founding year options
function getFoundedYear(){
    var date = new Date(),
        year = date.getFullYear(),
        yearArr = [],
        yearSuffix = comPickerI18n('yearSuffix');
    for (var i = year; i >= 1900; i--){
        yearArr.push({date: i, text: i + yearSuffix});
    }
    return yearArr;
}
