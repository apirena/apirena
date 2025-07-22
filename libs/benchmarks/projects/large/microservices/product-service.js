const express = require('express');
const router = express.Router();

// Product catalog service
router.get('/products', async (req, res) => {
    const products = await ProductService.search(req.query);
    res.json(products);
});

router.post('/products', async (req, res) => {
    const product = await ProductService.create(req.body);
    res.status(201).json(product);
});

router.get('/products/:id', async (req, res) => {
    const product = await ProductService.getById(req.params.id);
    res.json(product);
});

router.put('/products/:id', async (req, res) => {
    const product = await ProductService.update(req.params.id, req.body);
    res.json(product);
});

router.delete('/products/:id', async (req, res) => {
    await ProductService.delete(req.params.id);
    res.status(204).send();
});

// Product categories
router.get('/categories', async (req, res) => {
    const categories = await CategoryService.getAll();
    res.json(categories);
});

router.get('/categories/:id/products', async (req, res) => {
    const products = await ProductService.getByCategory(req.params.id);
    res.json(products);
});

// Product reviews
router.get('/products/:id/reviews', async (req, res) => {
    const reviews = await ReviewService.getByProduct(req.params.id);
    res.json(reviews);
});

router.post('/products/:id/reviews', async (req, res) => {
    const review = await ReviewService.create(req.params.id, req.body);
    res.status(201).json(review);
});

// Inventory management
router.get('/products/:id/inventory', async (req, res) => {
    const inventory = await InventoryService.getStock(req.params.id);
    res.json(inventory);
});

router.put('/products/:id/inventory', async (req, res) => {
    await InventoryService.updateStock(req.params.id, req.body.quantity);
    res.json({ updated: true });
});

module.exports = router;
