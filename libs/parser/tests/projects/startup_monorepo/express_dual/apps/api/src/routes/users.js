const router = require('express').Router();

router.get('/users', (req, res) => {
    res.json({ users: [] });
});

router.post('/users', (req, res) => {
    res.json({ id: 1, name: 'New User' });
});

module.exports = router;
