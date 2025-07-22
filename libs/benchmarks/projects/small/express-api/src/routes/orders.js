const express = require('express');
const router = express.Router();

// Order routes
router.get('/', async (req, res) => {
    try {
        const { page = 1, limit = 10, status, userId } = req.query;
        
        let query = {};
        if (status) query.status = status;
        if (userId) query.userId = userId;
        
        const orders = await Order.find(query)
            .limit(limit * 1)
            .skip((page - 1) * limit)
            .populate('userId', 'email username')
            .populate('items.productId', 'name price')
            .sort({ createdAt: -1 });
            
        const total = await Order.countDocuments(query);
        
        res.json({
            orders,
            totalPages: Math.ceil(total / limit),
            currentPage: page,
            total
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/', async (req, res) => {
    try {
        const { userId, items, shippingAddress, paymentMethod } = req.body;
        
        // Calculate total
        let totalAmount = 0;
        for (const item of items) {
            const product = await Product.findById(item.productId);
            if (!product) {
                return res.status(400).json({ error: `Product ${item.productId} not found` });
            }
            
            if (product.stockQuantity < item.quantity) {
                return res.status(400).json({ 
                    error: `Insufficient stock for product ${product.name}` 
                });
            }
            
            totalAmount += product.price * item.quantity;
        }
        
        const order = new Order({
            userId,
            items,
            totalAmount,
            shippingAddress,
            paymentMethod,
            status: 'pending'
        });
        
        await order.save();
        
        // Update product stock
        for (const item of items) {
            await Product.findByIdAndUpdate(
                item.productId,
                { $inc: { stockQuantity: -item.quantity } }
            );
        }
        
        await order.populate('items.productId', 'name price');
        res.status(201).json(order);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

router.get('/:id', async (req, res) => {
    try {
        const order = await Order.findById(req.params.id)
            .populate('userId', 'email username')
            .populate('items.productId', 'name price sku');
        
        if (!order) {
            return res.status(404).json({ error: 'Order not found' });
        }
        
        res.json(order);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.patch('/:id/status', async (req, res) => {
    try {
        const { status } = req.body;
        const validStatuses = ['pending', 'confirmed', 'shipped', 'delivered', 'cancelled'];
        
        if (!validStatuses.includes(status)) {
            return res.status(400).json({ error: 'Invalid status' });
        }
        
        const order = await Order.findByIdAndUpdate(
            req.params.id,
            { status, updatedAt: new Date() },
            { new: true }
        );
        
        if (!order) {
            return res.status(404).json({ error: 'Order not found' });
        }
        
        res.json(order);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

router.delete('/:id', async (req, res) => {
    try {
        const order = await Order.findById(req.params.id);
        
        if (!order) {
            return res.status(404).json({ error: 'Order not found' });
        }
        
        if (order.status === 'shipped' || order.status === 'delivered') {
            return res.status(400).json({ 
                error: 'Cannot cancel shipped or delivered orders' 
            });
        }
        
        // Restore product stock if order was confirmed
        if (order.status === 'confirmed') {
            for (const item of order.items) {
                await Product.findByIdAndUpdate(
                    item.productId,
                    { $inc: { stockQuantity: item.quantity } }
                );
            }
        }
        
        await Order.findByIdAndDelete(req.params.id);
        res.status(204).send();
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// User's orders
router.get('/user/:userId', async (req, res) => {
    try {
        const { userId } = req.params;
        const { page = 1, limit = 10 } = req.query;
        
        const orders = await Order.find({ userId })
            .limit(limit * 1)
            .skip((page - 1) * limit)
            .populate('items.productId', 'name price')
            .sort({ createdAt: -1 });
            
        const total = await Order.countDocuments({ userId });
        
        res.json({
            orders,
            totalPages: Math.ceil(total / limit),
            currentPage: page,
            total
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Order statistics
router.get('/stats/summary', async (req, res) => {
    try {
        const stats = await Order.aggregate([
            {
                $group: {
                    _id: '$status',
                    count: { $sum: 1 },
                    totalAmount: { $sum: '$totalAmount' }
                }
            }
        ]);
        
        const totalOrders = await Order.countDocuments();
        const totalRevenue = await Order.aggregate([
            {
                $match: {
                    status: { $in: ['confirmed', 'shipped', 'delivered'] }
                }
            },
            {
                $group: {
                    _id: null,
                    total: { $sum: '$totalAmount' }
                }
            }
        ]);
        
        res.json({
            totalOrders,
            totalRevenue: totalRevenue[0]?.total || 0,
            statusBreakdown: stats
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Track order
router.get('/:id/tracking', async (req, res) => {
    try {
        const order = await Order.findById(req.params.id)
            .select('status createdAt updatedAt trackingNumber');
        
        if (!order) {
            return res.status(404).json({ error: 'Order not found' });
        }
        
        const trackingInfo = {
            orderId: order._id,
            status: order.status,
            createdAt: order.createdAt,
            updatedAt: order.updatedAt,
            trackingNumber: order.trackingNumber,
            estimatedDelivery: order.estimatedDelivery
        };
        
        res.json(trackingInfo);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

module.exports = router;
