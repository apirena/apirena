// Express.js Parameter Routes Test
// Tests: Dynamic parameters, optional parameters, regex patterns

const express = require('express');
const app = express();

// Simple parameter routes
app.get('/users/:id', (req, res) => {
  res.json({ id: req.params.id });
});

app.get('/users/:id/posts', (req, res) => {
  res.json({ userId: req.params.id, posts: [] });
});

// Multiple parameters
app.get('/users/:userId/posts/:postId', (req, res) => {
  res.json({ 
    userId: req.params.userId, 
    postId: req.params.postId 
  });
});

// Optional parameters (using multiple routes)
app.get('/search', (req, res) => {
  res.json({ query: req.query.q });
});

app.get('/search/:category', (req, res) => {
  res.json({ 
    category: req.params.category,
    query: req.query.q 
  });
});

// Wildcard routes
app.get('/files/*', (req, res) => {
  res.json({ path: req.params[0] });
});

// Regex patterns
app.get(/.*fly$/, (req, res) => {
  res.json({ matched: 'ends with fly' });
});

// Parameter with constraints (numeric)
app.get('/products/:id(\\d+)', (req, res) => {
  res.json({ productId: parseInt(req.params.id) });
});

// Multiple optional parameters
app.get('/archive/:year/:month?/:day?', (req, res) => {
  res.json({
    year: req.params.year,
    month: req.params.month,
    day: req.params.day
  });
});

module.exports = app;
