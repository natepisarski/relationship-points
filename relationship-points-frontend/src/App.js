//@flow
import React, { Component } from 'react';
import logo from './logo.svg';
import { createStore } from 'redux'
import {Provider} from 'react-redux'
import { Switch, Route } from 'react-router-dom';

import {HomeContainer} from "./Containers/HomeContainer";
import {UsersContainer} from "./Containers/UsersContainer";

import './App.css';

export type STORE_TYPE = {
    applicationName: string,
    userList: Array
}

const store: STORE_TYPE = createStore(

) // FOR TOMORROW: You have a root reducer, the store, and the beginning of an API working.
// You need to (before you can test react-redux-router): Hook HomeComponent up to the initial state
// You need to (before you test lifecycle): Create a reducer / action creator tied to a HomeController button
// You need to (before you test API calls): Create a reducer / action JUST for the easiest return type of data

class App extends Component {
    render() {
        return (
            <Provider store={store}>
            <main>
                <Switch>
                    <Route exact path={'/'} component={HomeContainer} />
                    <Route path={'/users'} component={UsersContainer} />
                </Switch>
            </main>
            </Provider>
        )
    }
}

export default App;
