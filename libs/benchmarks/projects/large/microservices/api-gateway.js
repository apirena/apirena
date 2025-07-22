const express = require('express');
const router = express.Router();

// Comprehensive API routes for performance testing

// Users CRUD
router.get('/api/users', async (req, res) => {
    const users = await User.findAll();
    res.json(users);
});

router.post('/api/users', async (req, res) => {
    const user = await User.create(req.body);
    res.status(201).json(user);
});

router.get('/api/users/:id', async (req, res) => {
    const user = await User.findByPk(req.params.id);
    res.json(user);
});

router.put('/api/users/:id', async (req, res) => {
    await User.update(req.body, { where: { id: req.params.id } });
    res.json({ updated: true });
});

router.delete('/api/users/:id', async (req, res) => {
    await User.destroy({ where: { id: req.params.id } });
    res.json({ deleted: true });
});

// Posts CRUD
router.get('/api/posts', async (req, res) => {
    const posts = await Post.findAll();
    res.json(posts);
});

router.post('/api/posts', async (req, res) => {
    const post = await Post.create(req.body);
    res.status(201).json(post);
});

router.get('/api/posts/:id', async (req, res) => {
    const post = await Post.findByPk(req.params.id);
    res.json(post);
});

router.put('/api/posts/:id', async (req, res) => {
    await Post.update(req.body, { where: { id: req.params.id } });
    res.json({ updated: true });
});

router.delete('/api/posts/:id', async (req, res) => {
    await Post.destroy({ where: { id: req.params.id } });
    res.json({ deleted: true });
});

// Comments
router.get('/api/posts/:postId/comments', async (req, res) => {
    const comments = await Comment.findAll({ where: { postId: req.params.postId } });
    res.json(comments);
});

router.post('/api/posts/:postId/comments', async (req, res) => {
    const comment = await Comment.create({ ...req.body, postId: req.params.postId });
    res.status(201).json(comment);
});

// Search endpoints
router.get('/api/search/users', async (req, res) => {
    const { q } = req.query;
    const users = await User.findAll({ where: { name: { [Op.like]: `%${q}%` } } });
    res.json(users);
});

router.get('/api/search/posts', async (req, res) => {
    const { q } = req.query;
    const posts = await Post.findAll({ where: { title: { [Op.like]: `%${q}%` } } });
    res.json(posts);
});

module.exports = router;
