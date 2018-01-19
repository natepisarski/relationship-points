import { combineReducers } from 'redux'

const InitialState = {
    appName: "Relationship Points"
}

export const ApplicationReducer = (state = InitialState, action) => {
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