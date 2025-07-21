const express = require('express');
const app = express();

app.get('/api/health', (req, res) => {
    res.json({ status: 'ok' });
});

app.listen(3000, () => {
    console.log('API server running on port 3000');
});

module.exports = app;
