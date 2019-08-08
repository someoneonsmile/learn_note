const {
  override,
  fixBabelImports,
  useBabelRc,
  useEslintRc
} = require('customize-cra');
module.exports = override(
  fixBabelImports('import', {
    libraryName: 'antd',
    libraryDirectory: 'es',
    style: 'css'
  }),
  useBabelRc(),
  useEslintRc()
);
