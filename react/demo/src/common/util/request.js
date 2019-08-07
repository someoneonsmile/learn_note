import axios from "axios"

/**
 * 发送 get 请求
 * @param {*} url 
 * @param {*} data 
 */
export function get(url, data){
    return axios.get(url, data).then(res => res.data);
}

/**
 * 发送 post 请求
 * @param {*} url 
 * @param {*} data 
 */
export function post(url, data){
    return axios.post(url, data).then(res => res.data);
}