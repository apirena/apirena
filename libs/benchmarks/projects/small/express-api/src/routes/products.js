const express = require('express');
const router = express.Router();

// Product routes
router.get('/', async (req, res) => {
    try {
        const { page = 1, limit = 20, category, minPrice, maxPrice, search } = req.query;
        
        let query = {};
        if (category) query.category = category;
        if (minPrice || maxPrice) {
            query.price = {};
            if (minPrice) query.price.$gte = parseFloat(minPrice);
            if (maxPrice) query.price.$lte = parseFloat(maxPrice);
        }
        if (search) {
            query.$or = [
                { name: { $regex: search, $options: 'i' } },
                { description: { $regex: search, $options: 'i' } }
            ];
        }
        
        const products = await Product.find(query)
            .limit(limit * 1)
            .skip((page - 1) * limit)
            .populate('category')
            .sort({ createdAt: -1 });
            
        const total = await Product.countDocuments(query);
        
        res.json({
            products,
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
        const { name, description, price, category, sku, stockQuantity } = req.body;
        
        const product = new Product({
            name,
            description,
            price,
            category,
            sku,
            stockQuantity: stockQuantity || 0
        });
        
        await product.save();
        res.status(201).json(product);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

router.get('/:id', async (req, res) => {
    try {
        const product = await Product.findById(req.params.id)
            .populate('category')
            .populate('reviews');
        
        if (!product) {
            return res.status(404).json({ error: 'Product not found' });
        }
        
        res.json(product);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.put('/:id', async (req, res) => {
    try {
        const updates = req.body;
        const product = await Product.findByIdAndUpdate(
            req.params.id,
            updates,
            { new: true, runValidators: true }
        );
        
        if (!product) {
            return res.status(404).json({ error: 'Product not found' });
        }
        
        res.json(product);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

router.delete('/:id', async (req, res) => {
    try {
        const product = await Product.findByIdAndDelete(req.params.id);
        
        if (!product) {
            return res.status(404).json({ error: 'Product not found' });
        }
        
        res.status(204).send();
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Product categories
router.get('/categories/list', async (req, res) => {
    try {
        const categories = await Product.distinct('category');
        res.json(categories);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Product search
router.get('/search/query', async (req, res) => {
    try {
        const { q } = req.query;
        if (!q) {
            return res.status(400).json({ error: 'Search query required' });
        }
        
        const products = await Product.find({
            $or: [
                { name: { $regex: q, $options: 'i' } },
                { description: { $regex: q, $options: 'i' } },
                { sku: { $regex: q, $options: 'i' } }
            ]
        }).limit(50);
        
        res.json(products);
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Stock management
router.put('/:id/stock', async (req, res) => {
    try {
        const { quantity } = req.body;
        
        const product = await Product.findByIdAndUpdate(
            req.params.id,
            { stockQuantity: quantity },
            { new: true }
        );
        
        if (!product) {
            return res.status(404).json({ error: 'Product not found' });
        }
        
        res.json({ stockQuantity: product.stockQuantity });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

// Bulk operations
router.post('/bulk/create', async (req, res) => {
    try {
        const { products } = req.body;
        const createdProducts = await Product.insertMany(products);
        res.status(201).json(createdProducts);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

router.patch('/bulk/update', async (req, res) => {
    try {
        const { updates } = req.body;
        const results = [];
        
        for (const update of updates) {
            const product = await Product.findByIdAndUpdate(
                update.id,
                update.data,
                { new: true }
            );
            if (product) results.push(product);
        }
        
        res.json(results);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

module.exports = router;
