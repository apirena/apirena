const express = require('express');
const app = express();

app.post('/api/users', (req, res) => {
    res.json({ id: 1, name: 'Created User' });
});

app.get('/api/health', (req, res) => {
    res.json({ status: 'healthy' });
});

module.exports = app;
