<?php

use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use App\Http\Controllers\Controller;

class ShopController extends Controller
{
    // Product endpoints
    public function getProducts(Request $request): JsonResponse
    {
        $products = Product::all();
        return response()->json(['products' => $products]);
    }

    public function createProduct(Request $request): JsonResponse
    {
        $product = Product::create($request->all());
        return response()->json($product, 201);
    }

    public function getProduct(Request $request, $id): JsonResponse
    {
        $product = Product::findOrFail($id);
        return response()->json($product);
    }

    public function updateProduct(Request $request, $id): JsonResponse
    {
        $product = Product::findOrFail($id);
        $product->update($request->all());
        return response()->json($product);
    }

    public function deleteProduct(Request $request, $id): JsonResponse
    {
        Product::destroy($id);
        return response()->json(['deleted' => true]);
    }

    // Cart endpoints
    public function getCart(Request $request): JsonResponse
    {
        $cartItems = CartItem::where('user_id', auth()->id())->get();
        return response()->json(['cart' => $cartItems]);
    }

    public function addToCart(Request $request): JsonResponse
    {
        $cartItem = CartItem::create([
            'user_id' => auth()->id(),
            'product_id' => $request->product_id,
            'quantity' => $request->quantity
        ]);
        return response()->json($cartItem, 201);
    }

    public function removeFromCart(Request $request, $id): JsonResponse
    {
        CartItem::where('id', $id)->where('user_id', auth()->id())->delete();
        return response()->json(['removed' => true]);
    }

    // Order endpoints
    public function getOrders(Request $request): JsonResponse
    {
        $orders = Order::where('user_id', auth()->id())->get();
        return response()->json(['orders' => $orders]);
    }

    public function createOrder(Request $request): JsonResponse
    {
        $order = Order::create([
            'user_id' => auth()->id(),
            'total' => $request->total,
            'status' => 'pending'
        ]);
        return response()->json($order, 201);
    }

    public function getOrder(Request $request, $id): JsonResponse
    {
        $order = Order::where('id', $id)->where('user_id', auth()->id())->firstOrFail();
        return response()->json($order);
    }

    public function updateOrderStatus(Request $request, $id): JsonResponse
    {
        $order = Order::findOrFail($id);
        $order->update(['status' => $request->status]);
        return response()->json($order);
    }

    // Category endpoints
    public function getCategories(Request $request): JsonResponse
    {
        $categories = Category::all();
        return response()->json(['categories' => $categories]);
    }

    public function getCategoryProducts(Request $request, $id): JsonResponse
    {
        $products = Product::where('category_id', $id)->get();
        return response()->json(['products' => $products]);
    }

    // Search endpoint
    public function searchProducts(Request $request): JsonResponse
    {
        $query = $request->query('q');
        $products = Product::where('name', 'LIKE', "%{$query}%")->get();
        return response()->json(['products' => $products]);
    }
}
