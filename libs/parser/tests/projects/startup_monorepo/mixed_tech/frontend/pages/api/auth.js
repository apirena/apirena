export default function handler(req, res) {
  if (req.method === 'POST') {
    res.json({ token: 'auth-token', user: req.body.email });
  } else {
    res.status(405).json({ error: 'Method not allowed' });
  }
}
