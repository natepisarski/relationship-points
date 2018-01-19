import { combineReducers } from 'redux'

const InitialState = {
    appName: "Relationship Points"
}

export type stateType = {
    application: {
        appName: string
    },
    emissary: {

    }
}


export const ApplicationReducer = (state: stateType = InitialState, action) => {
        return state
}

export const EmissaryReducer = (state = {}, action) => {
    if(action.type === 'emissary') {
        return Object.assign({}, state, {[action.key]: action.value})
    }
    return state
}

const RootReducer = combineReducers(
    {
        application: ApplicationReducer,
        emissary: EmissaryReducer
    }
)

export default RootReducer