const express = require('express');
const router = express.Router();

// Order management service
router.get('/orders', async (req, res) => {
    const orders = await OrderService.getAll(req.query);
    res.json(orders);
});

router.post('/orders', async (req, res) => {
    const order = await OrderService.create(req.body);
    res.status(201).json(order);
});

router.get('/orders/:id', async (req, res) => {
    const order = await OrderService.getById(req.params.id);
    res.json(order);
});

router.put('/orders/:id', async (req, res) => {
    const order = await OrderService.update(req.params.id, req.body);
    res.json(order);
});

router.delete('/orders/:id', async (req, res) => {
    await OrderService.cancel(req.params.id);
    res.status(204).send();
});

// Order status tracking
router.get('/orders/:id/status', async (req, res) => {
    const status = await OrderService.getStatus(req.params.id);
    res.json(status);
});

router.put('/orders/:id/status', async (req, res) => {
    await OrderService.updateStatus(req.params.id, req.body.status);
    res.json({ updated: true });
});

// Payment processing
router.post('/orders/:id/payment', async (req, res) => {
    const payment = await PaymentService.process(req.params.id, req.body);
    res.json(payment);
});

router.get('/orders/:id/payment', async (req, res) => {
    const payment = await PaymentService.getStatus(req.params.id);
    res.json(payment);
});

// Shipping
router.post('/orders/:id/ship', async (req, res) => {
    const shipping = await ShippingService.ship(req.params.id, req.body);
    res.json(shipping);
});

router.get('/orders/:id/tracking', async (req, res) => {
    const tracking = await ShippingService.getTracking(req.params.id);
    res.json(tracking);
});

module.exports = router;
