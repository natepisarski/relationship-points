//@flow
import React, { Component } from 'react';
import logo from './logo.svg';
import { Switch, Route } from 'react-router-dom';

import {HomeContainer} from "./Containers/HomeContainer";
import {UsersContainer} from "./Containers/UsersContainer";

import './App.css';

class App extends Component {
    render() {
        return (
            <main>
                <Switch>
                    <Route exact path={'/'} component={HomeContainer} />
                    <Route path={'/users'} component={UsersContainer} />
                </Switch>
            </main>
        )
    }
}

export default App;
