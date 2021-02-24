const webpack = require('webpack')

process.env.NODE_ENV = 'development'

const config = require('./webpack.config.js')



webpack(config, (err, stats) => {
    if (err) throw err
    process.stdout.write(stats.toString({
        colors: true,
        modules: false,
        children: false, // If you are using ts-loader, setting this to true will make TypeScript errors show up during build.
        chunks: false,
        chunkModules: false
      }) + '\n\n')
})