import { Layout, Menu } from 'antd';
import "../node_modules/antd/dist/antd.css";
import React from 'react';
import {Link} from 'react-router-dom';
const { Header, Footer, Content } = Layout;
class Main_view extends React.Component{
    render(){
      return(
        <Layout>
      <Layout>
        <Header className="header">
          <div className="logo" />
          <Menu theme="dark" mode="horizontal" defaultSelectedKeys={['2']}>
          <Menu.Item key="1"><Link to="/">🐖哥blog</Link></Menu.Item>
          <Menu.Item key="2"><Link to="/yygq">yygq打分系统</Link></Menu.Item>  
          </Menu>
        </Header>
        <Content className="site-layout" style={{ padding: '0 50px', marginTop: 64 }}>
        <div className="site-layout-background" style={{ padding: 24, minHeight: 600 }}>
          这里是🐖哥的blog
        </div>
        </Content>
        <Footer style={{ textAlign: 'center' }}>👴</Footer>
      </Layout>
    </Layout>
    
      );
    }
}
export default Main_view;