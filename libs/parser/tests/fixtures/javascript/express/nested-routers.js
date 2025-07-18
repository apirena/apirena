// Express.js Nested Routers Test
// Tests: Router.use(), modular routing, middleware application

const express = require('express');
const app = express();

// Create separate routers
const apiRouter = express.Router();
const adminRouter = express.Router();
const userRouter = express.Router();

// API Router routes
apiRouter.get('/health', (req, res) => {
  res.json({ status: 'ok' });
});

apiRouter.get('/version', (req, res) => {
  res.json({ version: '1.0.0' });
});

// User Router routes
userRouter.get('/', (req, res) => {
  res.json([]);
});

userRouter.post('/', (req, res) => {
  res.status(201).json({ id: 1 });
});

userRouter.get('/:id', (req, res) => {
  res.json({ id: req.params.id });
});

userRouter.put('/:id', (req, res) => {
  res.json({ id: req.params.id, updated: true });
});

userRouter.delete('/:id', (req, res) => {
  res.status(204).send();
});

// Admin Router routes
adminRouter.get('/dashboard', (req, res) => {
  res.json({ dashboard: 'data' });
});

adminRouter.get('/users', (req, res) => {
  res.json({ users: [] });
});

adminRouter.post('/backup', (req, res) => {
  res.json({ backup: 'started' });
});

// Mount routers
app.use('/api', apiRouter);
app.use('/api/users', userRouter);
app.use('/admin', adminRouter);

// Nested router mounting
const v1Router = express.Router();
const v2Router = express.Router();

v1Router.get('/posts', (req, res) => {
  res.json({ version: 'v1', posts: [] });
});

v2Router.get('/posts', (req, res) => {
  res.json({ version: 'v2', posts: [] });
});

app.use('/api/v1', v1Router);
app.use('/api/v2', v2Router);

module.exports = app;
