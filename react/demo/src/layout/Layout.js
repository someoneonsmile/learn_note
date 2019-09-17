import React, { lazy, Suspense, useState } from 'react';

// 路由
import { BrowserRouter as Router, Route, Switch, Link } from 'react-router-dom';

// 第三方组件库
import { Spin, Layout, Menu, Icon } from 'antd';

// 样式
import './layout.css';

// 自定义组件
const Home = lazy(() => import('../mod/home/Home'));
const App = lazy(() => import('../mod/app/App'));
const NoMatch = lazy(() => import('../mod/nomatch/Nomatch'));

const { Header, Sider, Content } = Layout;

const urlList = [
  {
    path: '/',
    name: 'Home',
    icon: 'home',
    exact: true,
    component: Home
  },
  {
    path: '/list',
    name: 'list',
    icon: 'caret-right',
    component: App
  },
  {
    path: '',
    component: NoMatch
  }
];

function LayoutSwitch({ location }) {
  const [collapsed, setCollapsed] = useState(false);
  // 切换侧边栏
  const toggle = () => {
    setCollapsed(!collapsed);
  };

  return (
    <Router>
      <Layout>
        <Sider
          trigger={null}
          collapsible
          collapsed={collapsed}
          breakpoint="lg"
          collapsedWidth="0"
        >
          <div className="logo" />
          <Menu
            theme="dark"
            mode="inline"
            defaultSelectedKeys={[location.pathname]}
          >
            {urlList.map(url => {
              return (
                <Menu.Item key={url.path}>
                  <Link to={url.path}>
                    <Icon type={url.icon} />
                    <span>{url.name}</span>
                  </Link>
                </Menu.Item>
              );
            })}
          </Menu>
        </Sider>
        <Layout>
          <Header>
            <Icon
              className="trigger"
              type={collapsed ? 'menu-unfold' : 'menu-fold'}
              onClick={toggle}
              style={{
                color: '#ffffff'
              }}
            />
          </Header>
          <Content className={'content'}>
            <Suspense fallback={<Spin />}>
              <Switch>
                {urlList.map(url => {
                  return <Route {...url} />;
                })}
              </Switch>
            </Suspense>
          </Content>
        </Layout>
      </Layout>
    </Router>
  );
}

function HomeLayout() {
  return (
    <Router>
      <Route component={LayoutSwitch} />
    </Router>
  );
}

export default HomeLayout;
