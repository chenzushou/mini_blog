import { Layout, Menu,Button} from 'antd';
import "../node_modules/antd/dist/antd.css";
import React from 'react';
import {Link} from 'react-router-dom';
const { Header, Footer, Content } = Layout;
class Yygq extends React.Component{
  constructor(props) {
    super(props);
    this.state = {
      score:0
    };
    this.add=this.add.bind(this);
    this.sub=this.sub.bind(this);
  }
  add(){
    fetch('http://127.0.0.1:8080/zugescore/add',{
      method:'GET',
    })
    .then((response)=>{
      return response.json();
    })
    .then(
      (data)=>{
        console.log(data)
        this.setState({
          score:data.point,
        })
      }
    )
  }
  sub(){
    fetch('http://127.0.0.1:8080/zugescore/sub',{
      method:'GET',
    })
    .then((response)=>{
      return response.json();})
    .then(
      (data)=>{
        console.log(data)
        this.setState({
          score:data.point,
        })
      }
    )
  }

  componentDidMount() {
    fetch('http://127.0.0.1:8080/zugescore/get',{
      method:'GET',
    })
      .then((response)=>{
        return response.json();})
      .then(
        (data)=>{
          console.log(data)
          this.setState({
            score:data.point,
          })
        }
      )
  }
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
            <div>{this.state.score}</div>
            <Button onClick={this.add}>加10分！</Button>
            <Button onClick={this.sub}>减10分！</Button>
            </div>
        </Content>
        <Footer style={{ textAlign: 'center' }}>👴</Footer>
        </Layout>
        </Layout>
      );
    }
}
export default Yygq;