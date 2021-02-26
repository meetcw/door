const path = require('path')
module.exports = {
    mode : 'production',
    getPageFilename : function (name){
        return './layout/_' + name + '.hbs'
    },
    outputPath: path.resolve(__dirname, '../core/resource/template')
}
