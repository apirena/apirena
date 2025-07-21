const express = require('express');
const app = express();

app.get('/', (req, res) => {
    res.json({ message: 'Hello World' });
});

app.post('/users', (req, res) => {
    res.json({ id: 1, name: 'User' });
});

app.get('/users/:id', (req, res) => {
    res.json({ id: req.params.id, name: 'User' });
});

module.exports = app;
