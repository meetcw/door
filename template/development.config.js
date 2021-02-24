const path = require('path')

module.exports = {
    mode : 'development',
    getPageFilename : function (name){
        return name + '.html'
    },
    outputPath: path.resolve(__dirname, 'dist')
}