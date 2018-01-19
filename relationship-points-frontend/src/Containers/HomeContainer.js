//@flow
import React from 'react'
import { connect } from 'react-redux'

export type homeContainerProps = {
    appName: string,
    children: any
}

class HomeContainer extends React.Component<homeContainerProps> {
    constructor(props) {
        super(props)
    }

    render() {
        return (
            <div>
                <h1>You have successfully rendered the Home container! The application name is {this.props.appName}</h1>
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

const Home = connect(
    mapStateToProps
)(HomeContainer)

export default Home