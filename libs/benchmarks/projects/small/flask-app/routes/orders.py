from flask import Blueprint, request, jsonify
from flask_jwt_extended import jwt_required, get_jwt_identity
from models.order import Order
from models.user import User
from models.product import Product
from app import db

orders_bp = Blueprint('orders', __name__)

@orders_bp.route('/', methods=['GET'])
@jwt_required()
def get_orders():
    """Get all orders (admin) or user's orders"""
    current_user_id = get_jwt_identity()
    current_user = User.query.get(current_user_id)
    
    if current_user.is_admin:
        # Admin can see all orders
        orders = Order.query.all()
    else:
        # Regular users see only their orders
        orders = Order.query.filter_by(user_id=current_user_id).all()
    
    return jsonify([order.to_dict() for order in orders])

@orders_bp.route('/', methods=['POST'])
@jwt_required()
def create_order():
    """Create a new order"""
    current_user_id = get_jwt_identity()
    data = request.get_json()
    
    if 'items' not in data or not data['items']:
        return jsonify({'error': 'Order items required'}), 400
    
    # Validate products and calculate total
    total_amount = 0
    order_items = []
    
    for item in data['items']:
        product = Product.query.get(item.get('product_id'))
        if not product:
            return jsonify({'error': f'Product {item.get("product_id")} not found'}), 400
        
        quantity = item.get('quantity', 1)
        if product.stock_quantity < quantity:
            return jsonify({'error': f'Insufficient stock for {product.name}'}), 400
        
        item_total = product.price * quantity
        total_amount += item_total
        
        order_items.append({
            'product': product,
            'quantity': quantity,
            'price': product.price,
            'total': item_total
        })
    
    # Create order
    order = Order(
        user_id=current_user_id,
        total_amount=total_amount,
        status='pending',
        shipping_address=data.get('shipping_address')
    )
    
    db.session.add(order)
    db.session.flush()  # Get order ID
    
    # Update product stock
    for item in order_items:
        item['product'].stock_quantity -= item['quantity']
    
    db.session.commit()
    
    return jsonify(order.to_dict()), 201

@orders_bp.route('/<int:order_id>', methods=['GET'])
@jwt_required()
def get_order(order_id):
    """Get a specific order"""
    current_user_id = get_jwt_identity()
    current_user = User.query.get(current_user_id)
    
    order = Order.query.get_or_404(order_id)
    
    # Users can only see their own orders, admins can see all
    if not current_user.is_admin and order.user_id != current_user_id:
        return jsonify({'error': 'Unauthorized'}), 403
    
    return jsonify(order.to_dict())

@orders_bp.route('/<int:order_id>/status', methods=['PUT'])
@jwt_required()
def update_order_status(order_id):
    """Update order status (admin only)"""
    current_user_id = get_jwt_identity()
    current_user = User.query.get(current_user_id)
    
    if not current_user.is_admin:
        return jsonify({'error': 'Admin access required'}), 403
    
    order = Order.query.get_or_404(order_id)
    data = request.get_json()
    
    if 'status' not in data:
        return jsonify({'error': 'Status is required'}), 400
    
    valid_statuses = ['pending', 'confirmed', 'shipped', 'delivered', 'cancelled']
    if data['status'] not in valid_statuses:
        return jsonify({'error': 'Invalid status'}), 400
    
    order.status = data['status']
    db.session.commit()
    
    return jsonify({'status': order.status})

@orders_bp.route('/<int:order_id>', methods=['DELETE'])
@jwt_required()
def cancel_order(order_id):
    """Cancel an order"""
    current_user_id = get_jwt_identity()
    current_user = User.query.get(current_user_id)
    
    order = Order.query.get_or_404(order_id)
    
    # Users can only cancel their own orders, admins can cancel any
    if not current_user.is_admin and order.user_id != current_user_id:
        return jsonify({'error': 'Unauthorized'}), 403
    
    if order.status in ['shipped', 'delivered']:
        return jsonify({'error': 'Cannot cancel shipped or delivered orders'}), 400
    
    order.status = 'cancelled'
    
    # Restore product stock if order was confirmed
    if order.status == 'confirmed':
        for item in order.items:
            item.product.stock_quantity += item.quantity
    
    db.session.commit()
    
    return jsonify({'message': 'Order cancelled successfully'})

@orders_bp.route('/stats', methods=['GET'])
@jwt_required()
def get_order_stats():
    """Get order statistics (admin only)"""
    current_user_id = get_jwt_identity()
    current_user = User.query.get(current_user_id)
    
    if not current_user.is_admin:
        return jsonify({'error': 'Admin access required'}), 403
    
    stats = {
        'total_orders': Order.query.count(),
        'pending_orders': Order.query.filter_by(status='pending').count(),
        'confirmed_orders': Order.query.filter_by(status='confirmed').count(),
        'shipped_orders': Order.query.filter_by(status='shipped').count(),
        'delivered_orders': Order.query.filter_by(status='delivered').count(),
        'cancelled_orders': Order.query.filter_by(status='cancelled').count(),
        'total_revenue': db.session.query(db.func.sum(Order.total_amount)).filter(
            Order.status.in_(['confirmed', 'shipped', 'delivered'])
        ).scalar() or 0
    }
    
    return jsonify(stats)
