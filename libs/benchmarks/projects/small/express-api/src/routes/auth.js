const express = require('express');
const router = express.Router();

// Authentication routes
router.post('/auth/login', async (req, res) => {
    try {
        const { email, password } = req.body;
        const user = await User.findOne({ where: { email } });
        
        if (!user || !await user.validatePassword(password)) {
            return res.status(401).json({ error: 'Invalid credentials' });
        }
        
        const token = generateToken(user.id);
        res.json({ success: true, token, user });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/v1/auth/login', async (req, res) => {
    try {
        const { email, password } = req.body;
        const user = await User.findOne({ where: { email } });
        
        if (!user || !await user.validatePassword(password)) {
            return res.status(401).json({ error: 'Invalid credentials' });
        }
        
        const token = generateToken(user.id);
        res.json({ success: true, token, user });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

router.post('/auth/register', async (req, res) => {
    try {
        const { email, password, name } = req.body;
        const existingUser = await User.findOne({ where: { email } });
        
        if (existingUser) {
            return res.status(400).json({ error: 'Email already registered' });
        }
        
        const user = await User.create({ email, password, name });
        const token = generateToken(user.id);
        
        res.status(201).json({ success: true, token, user });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

router.post('/auth/logout', authenticateToken, (req, res) => {
    // In a real app, you'd invalidate the token
    res.json({ success: true, message: 'Logged out successfully' });
});

router.get('/auth/me', authenticateToken, async (req, res) => {
    try {
        const user = await User.findByPk(req.user.id);
        res.json({ success: true, data: user });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

module.exports = router;
