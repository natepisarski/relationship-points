//@flow
import React from 'react'

type userType = {
    firstName: string,
    lastName: string,
    emailAddress: string | "NO EMAIL ADDRESS"
}

type stateTypes = {
    displayedUsers: Array<userType>
}

type propTypes = {
    users: Array<userType>
}

export class UsersContainer extends React.Component<propTypes, stateTypes> {
    constructor(props: propTypes) {
        super(props);

        this.state = {
            displayedUsers: []
        }
    }

    render(){
        return (
            <div>
                <h1>{this.state.displayedUsers.length} / {this.props.users.length} shown</h1>
            <b>You have successfully rendered the Users container!</b>
            </div>
        )
    }
}