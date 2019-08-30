import React, { lazy, Suspense } from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import * as serviceWorker from './serviceWorker';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import { Spin } from 'antd';

const App = lazy(() => import('./mod/app/App'));

// 路由页
// 布局也写到该页
ReactDOM.render(
  <Suspense fallback={<Spin />}>
    <Router>
      <Switch>
        <Route exact path="/" component={Home} />
        <Route path="/list" component={App} />
        <Route component={NoMatch} />
      </Switch>
    </Router>
  </Suspense>,
  document.getElementById('root')
);

function Home() {
  return <h2>Home</h2>;
}

function NoMatch() {
  return <h2>NoMatch!</h2>;
}

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
