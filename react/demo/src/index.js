import React from 'react';
import ReactDOM from 'react-dom';
import * as serviceWorker from './serviceWorker';

import HomeLayout from './layout/Layout';

// 样式
import './index.css';
import './global.css';

// mock
import './mock/mock-global';

// 路由页
// 布局也写到该页
ReactDOM.render(<HomeLayout />, document.getElementById('root'));

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.register();
