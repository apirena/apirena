from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route('/')
def home():
    return jsonify({'message': 'Hello World'})

@app.route('/users', methods=['GET'])
def get_users():
    return jsonify({'users': []})

@app.route('/users', methods=['POST'])
def create_user():
    data = request.json
    return jsonify({'id': 1, **data})

@app.route('/users/<int:user_id>')
def get_user(user_id):
    return jsonify({'id': user_id, 'name': 'User'})

if __name__ == '__main__':
    app.run(debug=True)
