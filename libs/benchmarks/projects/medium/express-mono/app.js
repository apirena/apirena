const express = require('express');
const cors = require('cors');
const helmet = require('helmet');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(express.json());
app.use(cors());
app.use(helmet());

// Import routes
const authRoutes = require('./routes/auth');
const userRoutes = require('./routes/users');
const productRoutes = require('./routes/products');
const orderRoutes = require('./routes/orders');
const analyticsRoutes = require('./routes/analytics');
const webhookRoutes = require('./routes/webhooks');

// API routes
app.use('/api/auth', authRoutes);
app.use('/api/users', userRoutes);
app.use('/api/products', productRoutes);
app.use('/api/orders', orderRoutes);
app.use('/api/analytics', analyticsRoutes);
app.use('/api/webhooks', webhookRoutes);

// Health check
app.get('/health', (req, res) => {
    res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

// Root endpoint
app.get('/', (req, res) => {
    res.json({ 
        message: 'Express Monorepo API',
        version: '1.0.0',
        services: ['auth', 'users', 'products', 'orders', 'analytics', 'webhooks']
    });
});

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
