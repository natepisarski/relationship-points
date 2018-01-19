//@flow
import axios from 'axios'

export type API_CALL_TYPE = {
    url: string,
    method: "GET" | "POST" | "OPTIONS",
    payload: Object,
    returnKey: string
}

export const CallApi = (data: API_CALL_TYPE) => {
    axios({
        method: data.method.toLowerCase(),
        url: data.url,
        data: data.payload
    }).then(response => {

    })
}