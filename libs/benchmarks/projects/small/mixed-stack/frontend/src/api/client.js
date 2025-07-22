const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:5000/api';

class ApiClient {
    constructor() {
        this.baseURL = API_BASE_URL;
    }

    async request(endpoint, options = {}) {
        const url = `${this.baseURL}${endpoint}`;
        const config = {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers,
            },
            ...options,
        };

        if (config.body && typeof config.body === 'object') {
            config.body = JSON.stringify(config.body);
        }

        const response = await fetch(url, config);
        
        if (!response.ok) {
            const error = await response.json().catch(() => ({}));
            throw new Error(error.message || `HTTP ${response.status}`);
        }

        return response.json();
    }

    // Auth endpoints
    async login(credentials) {
        return this.request('/auth/login', {
            method: 'POST',
            body: credentials,
        });
    }

    async register(userData) {
        return this.request('/auth/register', {
            method: 'POST',
            body: userData,
        });
    }

    async logout() {
        return this.request('/auth/logout', {
            method: 'POST',
        });
    }

    async getCurrentUser() {
        return this.request('/auth/me');
    }

    // User endpoints
    async getUsers(params = {}) {
        const query = new URLSearchParams(params).toString();
        return this.request(`/users${query ? `?${query}` : ''}`);
    }

    async getUser(id) {
        return this.request(`/users/${id}`);
    }

    async createUser(userData) {
        return this.request('/users', {
            method: 'POST',
            body: userData,
        });
    }

    async updateUser(id, userData) {
        return this.request(`/users/${id}`, {
            method: 'PUT',
            body: userData,
        });
    }

    async deleteUser(id) {
        return this.request(`/users/${id}`, {
            method: 'DELETE',
        });
    }

    // Product endpoints
    async getProducts(params = {}) {
        const query = new URLSearchParams(params).toString();
        return this.request(`/products${query ? `?${query}` : ''}`);
    }

    async getProduct(id) {
        return this.request(`/products/${id}`);
    }

    async createProduct(productData) {
        return this.request('/products', {
            method: 'POST',
            body: productData,
        });
    }

    async updateProduct(id, productData) {
        return this.request(`/products/${id}`, {
            method: 'PUT',
            body: productData,
        });
    }

    async deleteProduct(id) {
        return this.request(`/products/${id}`, {
            method: 'DELETE',
        });
    }

    async searchProducts(query) {
        return this.request(`/products/search?q=${encodeURIComponent(query)}`);
    }

    // Order endpoints
    async getOrders(params = {}) {
        const query = new URLSearchParams(params).toString();
        return this.request(`/orders${query ? `?${query}` : ''}`);
    }

    async getOrder(id) {
        return this.request(`/orders/${id}`);
    }

    async createOrder(orderData) {
        return this.request('/orders', {
            method: 'POST',
            body: orderData,
        });
    }

    async updateOrderStatus(id, status) {
        return this.request(`/orders/${id}/status`, {
            method: 'PATCH',
            body: { status },
        });
    }

    async cancelOrder(id) {
        return this.request(`/orders/${id}`, {
            method: 'DELETE',
        });
    }

    async getUserOrders(userId, params = {}) {
        const query = new URLSearchParams(params).toString();
        return this.request(`/orders/user/${userId}${query ? `?${query}` : ''}`);
    }

    async getOrderStats() {
        return this.request('/orders/stats');
    }

    // Analytics endpoints
    async getDashboardStats() {
        return this.request('/analytics/dashboard');
    }

    async getUserAnalytics(timeRange = '30d') {
        return this.request(`/analytics/users?range=${timeRange}`);
    }

    async getProductAnalytics(timeRange = '30d') {
        return this.request(`/analytics/products?range=${timeRange}`);
    }

    async getOrderAnalytics(timeRange = '30d') {
        return this.request(`/analytics/orders?range=${timeRange}`);
    }

    // File upload endpoints
    async uploadFile(file, type = 'general') {
        const formData = new FormData();
        formData.append('file', file);
        formData.append('type', type);

        return this.request('/upload', {
            method: 'POST',
            body: formData,
            headers: {}, // Let browser set Content-Type for FormData
        });
    }

    async deleteFile(fileId) {
        return this.request(`/files/${fileId}`, {
            method: 'DELETE',
        });
    }
}

export default new ApiClient();
