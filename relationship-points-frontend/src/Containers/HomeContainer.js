//@flow
import React from 'react'
import { connect } from 'react-redux'
import {dispatchApiAction} from "../Middlewares/api";

export type homeContainerProps = {
    appName: string,
    children: any,
    callApi: Function
}

class HomeContainer extends React.Component<homeContainerProps> {
    constructor(props) {
        super(props)
    }

    render() {
        return (
            <div>
                <h1 onClick={this.props.callApi}>You have successfully rendered the Home container! The application name is {this.props.appName}</h1>
                {this.props.children}
            </div>
        )
    }
}

const mapStateToProps = state => {
    return {
        appName: state.application.appName
    }
}

const mapDispatchToProps = (dispatch, ownProps) => {
    return {
        callApi: () => { return dispatch(dispatchApiAction(dispatch, 'http://localhost:8000/endpoint','POST', '{"id": 5,"name": "TEST"}'))}
    }
}

const Home = connect(
    mapStateToProps,
    mapDispatchToProps
)(HomeContainer)

export default Home