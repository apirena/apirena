# Flask Methods Test
# Tests: Different HTTP method specifications, method lists

from flask import Flask, jsonify, request

app = Flask(__name__)

# Single method specifications
@app.route('/users', methods=['GET'])
def list_users():
    return jsonify([])

@app.route('/users', methods=['POST'])
def create_user():
    return jsonify({'id': 1}), 201

@app.route('/users/<int:user_id>', methods=['PUT'])
def update_user(user_id):
    return jsonify({'id': user_id})

@app.route('/users/<int:user_id>', methods=['DELETE'])
def delete_user(user_id):
    return '', 204

@app.route('/users/<int:user_id>', methods=['PATCH'])
def patch_user(user_id):
    return jsonify({'id': user_id, 'patched': True})

# Multiple methods on same route
@app.route('/api/data', methods=['GET', 'POST'])
def handle_data():
    if request.method == 'GET':
        return jsonify({'data': []})
    return jsonify({'created': True}), 201

@app.route('/api/resource', methods=['GET', 'POST', 'PUT', 'DELETE'])
def handle_resource():
    method = request.method
    return jsonify({'method': method, 'handled': True})

# Less common HTTP methods
@app.route('/api/config', methods=['HEAD'])
def config_head():
    return '', 200

@app.route('/api/options', methods=['OPTIONS'])
def handle_options():
    return '', 200

# Default method (GET when no methods specified)
@app.route('/default-get')
def default_get_route():
    return jsonify({'method': 'GET'})

# Method with different parameter types
@app.route('/items/<string:item_type>', methods=['GET'])
def get_items_by_type(item_type):
    return jsonify({'type': item_type, 'items': []})

@app.route('/items/<string:item_type>/<int:item_id>', methods=['GET'])
def get_item(item_type, item_id):
    return jsonify({'type': item_type, 'id': item_id})

if __name__ == '__main__':
    app.run(debug=True)
