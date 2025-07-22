from flask import Blueprint, request, jsonify
from flask_jwt_extended import jwt_required
from models.product import Product
from app import db

products_bp = Blueprint('products', __name__)

@products_bp.route('/', methods=['GET'])
def get_products():
    """Get all products with filtering and pagination"""
    page = request.args.get('page', 1, type=int)
    per_page = request.args.get('per_page', 20, type=int)
    category = request.args.get('category')
    min_price = request.args.get('min_price', type=float)
    max_price = request.args.get('max_price', type=float)
    
    query = Product.query
    
    if category:
        query = query.filter(Product.category == category)
    if min_price:
        query = query.filter(Product.price >= min_price)
    if max_price:
        query = query.filter(Product.price <= max_price)
    
    products = query.paginate(
        page=page, per_page=per_page, error_out=False
    )
    
    return jsonify({
        'products': [product.to_dict() for product in products.items],
        'total': products.total,
        'pages': products.pages,
        'current_page': page
    })

@products_bp.route('/', methods=['POST'])
@jwt_required()
def create_product():
    """Create a new product"""
    data = request.get_json()
    
    required_fields = ['name', 'price', 'category']
    for field in required_fields:
        if field not in data:
            return jsonify({'error': f'{field} is required'}), 400
    
    product = Product(
        name=data['name'],
        description=data.get('description', ''),
        price=data['price'],
        category=data['category'],
        stock_quantity=data.get('stock_quantity', 0),
        sku=data.get('sku')
    )
    
    db.session.add(product)
    db.session.commit()
    
    return jsonify(product.to_dict()), 201

@products_bp.route('/<int:product_id>', methods=['GET'])
def get_product(product_id):
    """Get a specific product by ID"""
    product = Product.query.get_or_404(product_id)
    return jsonify(product.to_dict())

@products_bp.route('/<int:product_id>', methods=['PUT'])
@jwt_required()
def update_product(product_id):
    """Update a product"""
    product = Product.query.get_or_404(product_id)
    data = request.get_json()
    
    updatable_fields = ['name', 'description', 'price', 'category', 'stock_quantity', 'sku']
    for field in updatable_fields:
        if field in data:
            setattr(product, field, data[field])
    
    db.session.commit()
    return jsonify(product.to_dict())

@products_bp.route('/<int:product_id>', methods=['DELETE'])
@jwt_required()
def delete_product(product_id):
    """Delete a product"""
    product = Product.query.get_or_404(product_id)
    db.session.delete(product)
    db.session.commit()
    return '', 204

@products_bp.route('/categories', methods=['GET'])
def get_categories():
    """Get all product categories"""
    categories = db.session.query(Product.category.distinct()).all()
    return jsonify([cat[0] for cat in categories])

@products_bp.route('/search', methods=['GET'])
def search_products():
    """Search products by name or description"""
    query = request.args.get('q', '')
    if not query:
        return jsonify({'error': 'Search query required'}), 400
    
    products = Product.query.filter(
        db.or_(
            Product.name.contains(query),
            Product.description.contains(query)
        )
    ).all()
    
    return jsonify([product.to_dict() for product in products])

@products_bp.route('/<int:product_id>/stock', methods=['PUT'])
@jwt_required()
def update_stock(product_id):
    """Update product stock quantity"""
    product = Product.query.get_or_404(product_id)
    data = request.get_json()
    
    if 'quantity' not in data:
        return jsonify({'error': 'quantity is required'}), 400
    
    product.stock_quantity = data['quantity']
    db.session.commit()
    
    return jsonify({'stock_quantity': product.stock_quantity})
