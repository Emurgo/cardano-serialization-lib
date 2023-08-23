const webpack = require('webpack');

module.exports = function override(webpackConfig, env) {
            const wasmExtensionRegExp = /\.wasm$/;
            webpackConfig.resolve.extensions.push('.wasm');
            webpackConfig.experiments = {
                asyncWebAssembly: true,
                //lazyCompilation: true,
                syncWebAssembly: true,
                topLevelAwait: true,
            };
            webpackConfig.module.rules.forEach((rule) => {
                (rule.oneOf || []).forEach((oneOf) => {
                    if (oneOf.type === "asset/resource") {
                        oneOf.exclude.push(wasmExtensionRegExp);
                    }
                });
            });
            webpackConfig.plugins.push(new webpack.ProvidePlugin({
                Buffer: ['buffer', 'Buffer'],
            }));

            return webpackConfig;
}