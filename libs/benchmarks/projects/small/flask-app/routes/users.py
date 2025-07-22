from flask import Blueprint, request, jsonify
from flask_jwt_extended import jwt_required, get_jwt_identity
from models.user import User
from app import db

users_bp = Blueprint('users', __name__)

@users_bp.route('/', methods=['GET'])
@jwt_required()
def get_users():
    """Get all users with pagination"""
    page = request.args.get('page', 1, type=int)
    per_page = request.args.get('per_page', 10, type=int)
    
    users = User.query.paginate(
        page=page, per_page=per_page, error_out=False
    )
    
    return jsonify({
        'users': [user.to_dict() for user in users.items],
        'total': users.total,
        'pages': users.pages,
        'current_page': page
    })

@users_bp.route('/', methods=['POST'])
def create_user():
    """Create a new user"""
    data = request.get_json()
    
    if not data or 'email' not in data or 'password' not in data:
        return jsonify({'error': 'Email and password required'}), 400
    
    user = User(
        email=data['email'],
        username=data.get('username'),
        first_name=data.get('first_name'),
        last_name=data.get('last_name')
    )
    user.set_password(data['password'])
    
    db.session.add(user)
    db.session.commit()
    
    return jsonify(user.to_dict()), 201

@users_bp.route('/<int:user_id>', methods=['GET'])
@jwt_required()
def get_user(user_id):
    """Get a specific user by ID"""
    user = User.query.get_or_404(user_id)
    return jsonify(user.to_dict())

@users_bp.route('/<int:user_id>', methods=['PUT'])
@jwt_required()
def update_user(user_id):
    """Update a user"""
    current_user_id = get_jwt_identity()
    user = User.query.get_or_404(user_id)
    
    # Users can only update their own profile or admins can update any
    if current_user_id != user_id and not User.query.get(current_user_id).is_admin:
        return jsonify({'error': 'Unauthorized'}), 403
    
    data = request.get_json()
    if 'email' in data:
        user.email = data['email']
    if 'username' in data:
        user.username = data['username']
    if 'first_name' in data:
        user.first_name = data['first_name']
    if 'last_name' in data:
        user.last_name = data['last_name']
    
    db.session.commit()
    return jsonify(user.to_dict())

@users_bp.route('/<int:user_id>', methods=['DELETE'])
@jwt_required()
def delete_user(user_id):
    """Delete a user"""
    current_user_id = get_jwt_identity()
    user = User.query.get_or_404(user_id)
    
    # Users can only delete their own profile or admins can delete any
    if current_user_id != user_id and not User.query.get(current_user_id).is_admin:
        return jsonify({'error': 'Unauthorized'}), 403
    
    db.session.delete(user)
    db.session.commit()
    return '', 204

@users_bp.route('/<int:user_id>/profile', methods=['GET'])
@jwt_required()
def get_user_profile(user_id):
    """Get detailed user profile"""
    user = User.query.get_or_404(user_id)
    return jsonify({
        **user.to_dict(),
        'orders_count': user.orders.count(),
        'created_at': user.created_at.isoformat(),
        'updated_at': user.updated_at.isoformat()
    })

@users_bp.route('/<int:user_id>/orders', methods=['GET'])
@jwt_required()
def get_user_orders(user_id):
    """Get all orders for a specific user"""
    user = User.query.get_or_404(user_id)
    orders = user.orders.all()
    
    return jsonify([order.to_dict() for order in orders])
