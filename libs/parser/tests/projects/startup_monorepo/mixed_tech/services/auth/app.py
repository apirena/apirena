from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/login', methods=['POST'])
def login():
    data = request.json
    return jsonify({
        'token': 'auth-token',
        'user': data.get('email', 'user@example.com')
    })

@app.route('/validate', methods=['POST'])
def validate_token():
    token = request.headers.get('Authorization')
    return jsonify({'valid': True, 'user': 'user@example.com'})

if __name__ == '__main__':
    app.run(debug=True)
