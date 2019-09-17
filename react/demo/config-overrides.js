const {
  override,
  fixBabelImports,
  useBabelRc,
  useEslintRc
} = require('customize-cra');
const rewireUglifyjs = require('react-app-rewire-uglifyjs');

module.exports = override(
  fixBabelImports('import', {
    libraryName: 'antd',
    libraryDirectory: 'es',
    style: 'css'
  }),
  useBabelRc(),
  useEslintRc(),
  rewireUglifyjs
);
