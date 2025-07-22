const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());
app.use(cors());
app.use(helmet());

// API Gateway routes and proxies
app.get('/health', (req, res) => {
    res.json({ 
        status: 'healthy',
        gateway: 'api-gateway',
        timestamp: new Date().toISOString(),
        services: ['auth', 'user', 'payment', 'notification']
    });
});

app.get('/api/v1/status', (req, res) => {
    res.json({ 
        gateway: 'running',
        version: '1.0.0',
        uptime: process.uptime()
    });
});

// Service discovery endpoint
app.get('/api/v1/services', (req, res) => {
    res.json({
        services: [
            { name: 'auth-service', url: 'http://localhost:3001', status: 'healthy' },
            { name: 'user-service', url: 'http://localhost:3002', status: 'healthy' },
            { name: 'payment-service', url: 'http://localhost:3003', status: 'healthy' },
            { name: 'notification-service', url: 'http://localhost:3004', status: 'healthy' }
        ]
    });
});

// Rate limiting endpoint
app.get('/api/v1/rate-limits/:userId', (req, res) => {
    res.json({ 
        userId: req.params.userId,
        remaining: 1000,
        resetTime: new Date(Date.now() + 3600000).toISOString()
    });
});

// Load balancer health check
app.get('/api/v1/load-balancer/health', (req, res) => {
    res.json({ status: 'healthy', load: 'low' });
});

// Analytics collection endpoint
app.post('/api/v1/analytics/events', (req, res) => {
    res.json({ message: 'Event recorded' });
});

// Circuit breaker status
app.get('/api/v1/circuit-breaker/status', (req, res) => {
    res.json({ 
        services: {
            'auth-service': 'closed',
            'user-service': 'closed',
            'payment-service': 'closed',
            'notification-service': 'closed'
        }
    });
});

app.listen(PORT, () => {
    console.log(`API Gateway running on port ${PORT}`);
});
