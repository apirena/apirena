const express = require('express');
const router = express.Router();

// User service microservice
router.get('/users', async (req, res) => {
    const users = await UserService.getAll();
    res.json(users);
});

router.post('/users', async (req, res) => {
    const user = await UserService.create(req.body);
    res.status(201).json(user);
});

router.get('/users/:id', async (req, res) => {
    const user = await UserService.getById(req.params.id);
    res.json(user);
});

router.put('/users/:id', async (req, res) => {
    const user = await UserService.update(req.params.id, req.body);
    res.json(user);
});

router.delete('/users/:id', async (req, res) => {
    await UserService.delete(req.params.id);
    res.status(204).send();
});

// User authentication
router.post('/users/login', async (req, res) => {
    const token = await UserService.authenticate(req.body.email, req.body.password);
    res.json({ token });
});

router.post('/users/logout', async (req, res) => {
    await UserService.logout(req.headers.authorization);
    res.status(200).send();
});

router.get('/users/profile', async (req, res) => {
    const profile = await UserService.getProfile(req.user.id);
    res.json(profile);
});

module.exports = router;
