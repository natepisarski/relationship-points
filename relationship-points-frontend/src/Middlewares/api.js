//@flow
import axios from 'axios'
import type {Store, Middleware} from 'redux'

export const API_ACTION = 'API_CALL'
export const API_RESPONSE_ACTION = 'API_RESPONSE'

export type API_ACTION_TYPE = {
    type: typeof API_ACTION,
    method: "GET" | "POST" | "OPTIONS",
    url: string,
    payload: any
}

export type API_RESPONSE_ACTION_TYPE = {
    type: typeof API_RESPONSE_ACTION,
    data: string
}

export type API_CALL_TYPE = {
    url: string,
    method: "GET" | "POST" | "OPTIONS",
    payload: Object
}

export const dispatchApiAction = (dispatch, url, method, payload) => {
    return dispatch({
        type: API_ACTION,
        method,
        url,
        payload
    })
}

export const CallApi = (data: API_CALL_TYPE) => {
    return axios({
        method: data.method.toLowerCase(),
        url: data.url,
        data: data.payload,
        headers: {
            'Content-Type': 'application/json'
        }
    })
}

export const ApiMiddleware = (store: Store) => (next: Middleware) => (action: API_ACTION_TYPE) => {
    console.log('testing middleware')
    if(action.type === API_ACTION) {
        CallApi(
            {
                url: action.url,
                method: action.method,
                payload: action.payload
            }
        ).then(response => {
          if(response.status === 200) {
              store.dispatch({
                  type: API_RESPONSE_ACTION,
                  data: "OH BOY WE JUST WENT END-TO-END BOI"
              })
          }
        })
    }
    return next(action)
}