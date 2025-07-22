from flask import Flask, request, jsonify
from flask_sqlalchemy import SQLAlchemy
from flask_cors import CORS
from flask_jwt_extended import JWTManager
import os

app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = os.getenv('DATABASE_URL', 'sqlite:///app.db')
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
app.config['JWT_SECRET_KEY'] = 'your-secret-key'

db = SQLAlchemy(app)
jwt = JWTManager(app)
CORS(app)

# Import routes after app creation to avoid circular imports
from routes.users import users_bp
from routes.products import products_bp
from routes.orders import orders_bp
from routes.auth import auth_bp

app.register_blueprint(users_bp, url_prefix='/api/users')
app.register_blueprint(products_bp, url_prefix='/api/products')
app.register_blueprint(orders_bp, url_prefix='/api/orders')
app.register_blueprint(auth_bp, url_prefix='/api/auth')

@app.route('/api/health', methods=['GET'])
def health_check():
    return jsonify({'status': 'healthy', 'service': 'flask-benchmark-api'})

@app.route('/api/status', methods=['GET'])
def status():
    return jsonify({
        'status': 'running',
        'version': '1.0.0',
        'endpoints': [
            '/api/health',
            '/api/status',
            '/api/users',
            '/api/products',
            '/api/orders',
            '/api/auth'
        ]
    })

if __name__ == '__main__':
    with app.app_context():
        db.create_all()
    app.run(debug=True, host='0.0.0.0', port=5000)
