import {
  BrowserRouter as Router,
  Switch,
  Route
} from "react-router-dom";
import React from 'react';
import Main_view from './Main_view';
import Yygq from './yygq';
class APP extends React.Component{
  render(){
    return(
      <Route>
        <Switch>
        <Route exact path="/">
            <Main_view />
        </Route>
        <Route path="/yygq">
            <Yygq />
        </Route>
        </Switch>
      </Route>
    );
  }
}

export default APP;
