import * as apiMethods from './ApiMethods';

class Api {
  url;
  constructor(endpoint) {
    this.url = endpoint || 'http://localhost:8080';
  }

  async checkApiError(response) {
    if (response.status > 399) {
      if (response.status === 404) {
        throw new Error('not found');
      } else {
        const err = await response.json().catch((err) => {
          throw err;
        });
        throw new Error(err);
      }
    } else {
      const data = await response.json();
      return data;
    }
  }

  async getRequest(url) {
    const headers = {
      'Accept': 'application/json',
      'Content-Type': 'application/json'
    };

    const response = await fetch(`${this.url}${url}`, {
      headers,
    })
    return this.checkApiError(response);
  }

  async modifyRequest(url, method, body) {
    const response = await fetch(`${this.url}${url}`, {
      method: method,
      body: body ? JSON.stringify(body) : null,
    })

    return this.checkApiError(response);
  }

}

Object.assign(Api.prototype, apiMethods);

const api = new Api();

export default api;