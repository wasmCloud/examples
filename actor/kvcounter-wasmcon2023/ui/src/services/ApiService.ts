class ApiService {
    url;

    constructor(endpoint = window.location.href.replace(/\/$/, '')) {
        this.url = endpoint;
    }

    async increment(bucketName?: string) {
        return await this.#getRequest(`/counter/${bucketName ?? ''}`);
    }

    async #checkApiError(response: Response) {
        if (response.status >= 400) {
            if (response.status === 404) {
                throw new Error('not found');
            }
        
            const err = await response.json().catch((err) => {
                throw err;
            });
        
            throw new Error(err);
        }

        const data = await response.json();
        return data;
    }

    async #getRequest(path: string) {
        const response = await fetch(`${this.url}${path}`, {
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            }
        })
        return this.#checkApiError(response);
    }
}

const api = new ApiService(process.env.REACT_APP_API_URL ?? '/api');

export default api;