# Flask Basic Routes Test
# Tests: @app.route decorators, HTTP methods, simple paths

from flask import Flask, jsonify, request

app = Flask(__name__)

# Simple GET routes
@app.route('/users')
def get_users():
    """Get all users"""
    return jsonify([
        {'id': 1, 'name': 'John Doe'},
        {'id': 2, 'name': 'Jane Smith'}
    ])

@app.route('/users', methods=['POST'])
def create_user():
    """Create a new user"""
    user_data = request.get_json()
    user = {'id': 123, **user_data}
    return jsonify(user), 201

@app.route('/users/<int:user_id>')
def get_user(user_id):
    """Get user by ID"""
    return jsonify({'id': user_id, 'name': f'User {user_id}'})

@app.route('/users/<int:user_id>', methods=['PUT'])
def update_user(user_id):
    """Update user by ID"""
    user_data = request.get_json()
    return jsonify({'id': user_id, **user_data})

@app.route('/users/<int:user_id>', methods=['DELETE'])
def delete_user(user_id):
    """Delete user by ID"""
    return '', 204

# API namespace routes
@app.route('/api/posts')
def get_posts():
    """Get all posts"""
    return jsonify([])

@app.route('/api/posts', methods=['POST'])
def create_post():
    """Create a new post"""
    post_data = request.get_json()
    post = {'id': 1, **post_data}
    return jsonify(post), 201

@app.route('/api/posts/<int:post_id>')
def get_post(post_id):
    """Get post by ID"""
    return jsonify({'id': post_id, 'title': f'Post {post_id}'})

# Health check endpoint
@app.route('/health')
def health_check():
    """Health check endpoint"""
    return jsonify({'status': 'ok', 'service': 'flask-app'})

# Admin routes
@app.route('/admin/stats')
def admin_stats():
    """Get admin statistics"""
    return jsonify({'users': 100, 'posts': 50, 'active_sessions': 25})

# Multiple HTTP methods on single route
@app.route('/api/status', methods=['GET', 'POST'])
def api_status():
    """Handle both GET and POST for status"""
    if request.method == 'GET':
        return jsonify({'status': 'running'})
    else:
        return jsonify({'status': 'updated'}), 201

if __name__ == '__main__':
    app.run(debug=True, port=5000)
