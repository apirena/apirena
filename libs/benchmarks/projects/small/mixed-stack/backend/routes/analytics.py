from flask import Blueprint, request, jsonify
from flask_jwt_extended import jwt_required
import logging
from datetime import datetime, timedelta
from models.user import User
from models.product import Product
from models.order import Order
from extensions import db

analytics_bp = Blueprint('analytics', __name__)

@analytics_bp.route('/dashboard', methods=['GET'])
@jwt_required()
def get_dashboard_stats():
    """Get overall dashboard statistics"""
    try:
        # User stats
        total_users = User.query.count()
        new_users_today = User.query.filter(
            User.created_at >= datetime.utcnow().date()
        ).count()
        
        # Product stats  
        total_products = Product.query.count()
        low_stock_products = Product.query.filter(
            Product.stock_quantity < 10
        ).count()
        
        # Order stats
        total_orders = Order.query.count()
        pending_orders = Order.query.filter_by(status='pending').count()
        
        # Revenue stats
        total_revenue = db.session.query(
            db.func.sum(Order.total_amount)
        ).filter(
            Order.status.in_(['confirmed', 'shipped', 'delivered'])
        ).scalar() or 0
        
        # Recent activity
        recent_orders = Order.query.order_by(
            Order.created_at.desc()
        ).limit(5).all()
        
        return jsonify({
            'users': {
                'total': total_users,
                'new_today': new_users_today
            },
            'products': {
                'total': total_products,
                'low_stock': low_stock_products
            },
            'orders': {
                'total': total_orders,
                'pending': pending_orders
            },
            'revenue': {
                'total': float(total_revenue)
            },
            'recent_activity': [order.to_dict() for order in recent_orders]
        })
    except Exception as e:
        logging.error(f"Dashboard stats error: {e}")
        return jsonify({'error': 'Failed to fetch dashboard stats'}), 500

@analytics_bp.route('/users', methods=['GET'])
@jwt_required()
def get_user_analytics():
    """Get user analytics for specified time range"""
    try:
        range_param = request.args.get('range', '30d')
        
        # Parse time range
        if range_param == '7d':
            start_date = datetime.utcnow() - timedelta(days=7)
        elif range_param == '30d':
            start_date = datetime.utcnow() - timedelta(days=30)
        elif range_param == '90d':
            start_date = datetime.utcnow() - timedelta(days=90)
        else:
            start_date = datetime.utcnow() - timedelta(days=30)
        
        # User registration trends
        user_registrations = db.session.query(
            db.func.date(User.created_at).label('date'),
            db.func.count(User.id).label('count')
        ).filter(
            User.created_at >= start_date
        ).group_by(
            db.func.date(User.created_at)
        ).all()
        
        # Active users (users who placed orders)
        active_users = db.session.query(User).join(Order).filter(
            Order.created_at >= start_date
        ).distinct().count()
        
        # User demographics
        user_countries = db.session.query(
            User.country,
            db.func.count(User.id).label('count')
        ).filter(
            User.country.isnot(None)
        ).group_by(User.country).all()
        
        return jsonify({
            'registration_trends': [
                {
                    'date': reg.date.isoformat(),
                    'count': reg.count
                }
                for reg in user_registrations
            ],
            'active_users': active_users,
            'demographics': {
                'countries': [
                    {
                        'country': country.country,
                        'count': country.count
                    }
                    for country in user_countries
                ]
            }
        })
    except Exception as e:
        logging.error(f"User analytics error: {e}")
        return jsonify({'error': 'Failed to fetch user analytics'}), 500

@analytics_bp.route('/products', methods=['GET'])
@jwt_required()
def get_product_analytics():
    """Get product analytics"""
    try:
        range_param = request.args.get('range', '30d')
        
        if range_param == '7d':
            start_date = datetime.utcnow() - timedelta(days=7)
        elif range_param == '30d':
            start_date = datetime.utcnow() - timedelta(days=30)
        elif range_param == '90d':
            start_date = datetime.utcnow() - timedelta(days=90)
        else:
            start_date = datetime.utcnow() - timedelta(days=30)
        
        # Top selling products
        top_products = db.session.query(
            Product.id,
            Product.name,
            db.func.sum(OrderItem.quantity).label('total_sold'),
            db.func.sum(OrderItem.quantity * OrderItem.price).label('revenue')
        ).join(
            OrderItem, Product.id == OrderItem.product_id
        ).join(
            Order, OrderItem.order_id == Order.id
        ).filter(
            Order.created_at >= start_date,
            Order.status.in_(['confirmed', 'shipped', 'delivered'])
        ).group_by(
            Product.id, Product.name
        ).order_by(
            db.func.sum(OrderItem.quantity).desc()
        ).limit(10).all()
        
        # Category performance
        category_performance = db.session.query(
            Product.category,
            db.func.count(Product.id).label('product_count'),
            db.func.sum(OrderItem.quantity).label('total_sold'),
            db.func.sum(OrderItem.quantity * OrderItem.price).label('revenue')
        ).join(
            OrderItem, Product.id == OrderItem.product_id
        ).join(
            Order, OrderItem.order_id == Order.id
        ).filter(
            Order.created_at >= start_date,
            Order.status.in_(['confirmed', 'shipped', 'delivered'])
        ).group_by(
            Product.category
        ).all()
        
        # Stock levels
        stock_analysis = db.session.query(
            Product.category,
            db.func.avg(Product.stock_quantity).label('avg_stock'),
            db.func.count(
                db.case([(Product.stock_quantity < 10, 1)])
            ).label('low_stock_count')
        ).group_by(Product.category).all()
        
        return jsonify({
            'top_products': [
                {
                    'id': product.id,
                    'name': product.name,
                    'total_sold': product.total_sold or 0,
                    'revenue': float(product.revenue or 0)
                }
                for product in top_products
            ],
            'category_performance': [
                {
                    'category': cat.category,
                    'product_count': cat.product_count,
                    'total_sold': cat.total_sold or 0,
                    'revenue': float(cat.revenue or 0)
                }
                for cat in category_performance
            ],
            'stock_analysis': [
                {
                    'category': stock.category,
                    'avg_stock': float(stock.avg_stock or 0),
                    'low_stock_count': stock.low_stock_count
                }
                for stock in stock_analysis
            ]
        })
    except Exception as e:
        logging.error(f"Product analytics error: {e}")
        return jsonify({'error': 'Failed to fetch product analytics'}), 500

@analytics_bp.route('/orders', methods=['GET'])
@jwt_required()
def get_order_analytics():
    """Get order analytics"""
    try:
        range_param = request.args.get('range', '30d')
        
        if range_param == '7d':
            start_date = datetime.utcnow() - timedelta(days=7)
        elif range_param == '30d':
            start_date = datetime.utcnow() - timedelta(days=30)
        elif range_param == '90d':
            start_date = datetime.utcnow() - timedelta(days=90)
        else:
            start_date = datetime.utcnow() - timedelta(days=30)
        
        # Order trends
        order_trends = db.session.query(
            db.func.date(Order.created_at).label('date'),
            db.func.count(Order.id).label('order_count'),
            db.func.sum(Order.total_amount).label('revenue')
        ).filter(
            Order.created_at >= start_date
        ).group_by(
            db.func.date(Order.created_at)
        ).order_by('date').all()
        
        # Order status distribution
        status_distribution = db.session.query(
            Order.status,
            db.func.count(Order.id).label('count')
        ).filter(
            Order.created_at >= start_date
        ).group_by(Order.status).all()
        
        # Average order value
        avg_order_value = db.session.query(
            db.func.avg(Order.total_amount)
        ).filter(
            Order.created_at >= start_date,
            Order.status.in_(['confirmed', 'shipped', 'delivered'])
        ).scalar() or 0
        
        # Customer retention
        repeat_customers = db.session.query(
            Order.user_id,
            db.func.count(Order.id).label('order_count')
        ).filter(
            Order.created_at >= start_date
        ).group_by(Order.user_id).having(
            db.func.count(Order.id) > 1
        ).count()
        
        return jsonify({
            'order_trends': [
                {
                    'date': trend.date.isoformat(),
                    'order_count': trend.order_count,
                    'revenue': float(trend.revenue or 0)
                }
                for trend in order_trends
            ],
            'status_distribution': [
                {
                    'status': status.status,
                    'count': status.count
                }
                for status in status_distribution
            ],
            'avg_order_value': float(avg_order_value),
            'repeat_customers': repeat_customers
        })
    except Exception as e:
        logging.error(f"Order analytics error: {e}")
        return jsonify({'error': 'Failed to fetch order analytics'}), 500
