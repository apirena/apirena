// Express.js Basic Routes Test
// Tests: Basic HTTP methods, simple paths, route handlers

const express = require('express');
const app = express();

// Middleware
app.use(express.json());

// Simple CRUD operations for users
app.get('/users', (req, res) => {
  res.json([
    { id: 1, name: 'John Doe' },
    { id: 2, name: 'Jane Smith' }
  ]);
});

app.post('/users', (req, res) => {
  const user = { id: Date.now(), ...req.body };
  res.status(201).json(user);
});

app.put('/users/:id', (req, res) => {
  const id = parseInt(req.params.id);
  res.json({ id, ...req.body });
});

app.delete('/users/:id', (req, res) => {
  const id = parseInt(req.params.id);
  res.status(204).send();
});

// API namespace routes
app.get('/api/posts', (req, res) => {
  res.json([]);
});

app.post('/api/posts', (req, res) => {
  res.status(201).json({ 
    id: 1, 
    title: req.body.title,
    content: req.body.content 
  });
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

// Admin routes
app.get('/admin/stats', (req, res) => {
  res.json({ users: 100, posts: 50 });
});

module.exports = app;

if (require.main === module) {
  const PORT = process.env.PORT || 3000;
  app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
  });
}
