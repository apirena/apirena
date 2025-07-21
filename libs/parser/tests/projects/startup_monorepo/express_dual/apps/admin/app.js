const express = require('express');
const app = express();

app.post('/admin/users', (req, res) => {
    res.json({ id: 1, name: 'Admin Created User' });
});

app.get('/admin/dashboard', (req, res) => {
    res.json({ stats: { users: 0, orders: 0 } });
});

module.exports = app;
