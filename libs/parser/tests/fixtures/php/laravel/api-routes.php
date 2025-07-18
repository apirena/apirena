<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\Api\V1\UserController;
use App\Http\Controllers\Api\V1\PostController;
use App\Http\Controllers\Api\V1\AuthController;

/*
|--------------------------------------------------------------------------
| Laravel API Routes Test
| Tests: API versioning, resource controllers, middleware, rate limiting
|--------------------------------------------------------------------------
*/

// API v1 routes with rate limiting
Route::prefix('api/v1')->middleware(['api', 'throttle:60,1'])->group(function () {
    
    // Authentication endpoints
    Route::post('/login', [AuthController::class, 'login']);
    Route::post('/register', [AuthController::class, 'register']);
    Route::post('/logout', [AuthController::class, 'logout'])->middleware('auth:api');
    Route::post('/refresh', [AuthController::class, 'refresh'])->middleware('auth:api');
    
    // User endpoints with authentication
    Route::middleware('auth:api')->group(function () {
        Route::get('/users', [UserController::class, 'index']);
        Route::post('/users', [UserController::class, 'store']);
        Route::get('/users/{user}', [UserController::class, 'show']);
        Route::put('/users/{user}', [UserController::class, 'update']);
        Route::delete('/users/{user}', [UserController::class, 'destroy']);
        
        // User-specific routes
        Route::get('/users/{user}/posts', [PostController::class, 'userPosts']);
        Route::get('/users/{user}/followers', [UserController::class, 'followers']);
        Route::post('/users/{user}/follow', [UserController::class, 'follow']);
        Route::delete('/users/{user}/unfollow', [UserController::class, 'unfollow']);
    });
    
    // Post endpoints
    Route::get('/posts', [PostController::class, 'index']); // Public
    Route::get('/posts/{post}', [PostController::class, 'show']); // Public
    
    Route::middleware('auth:api')->group(function () {
        Route::post('/posts', [PostController::class, 'store']);
        Route::put('/posts/{post}', [PostController::class, 'update']);
        Route::delete('/posts/{post}', [PostController::class, 'destroy']);
        
        // Post interactions
        Route::post('/posts/{post}/like', [PostController::class, 'like']);
        Route::delete('/posts/{post}/unlike', [PostController::class, 'unlike']);
        Route::post('/posts/{post}/comments', [PostController::class, 'addComment']);
    });
});

// API v2 routes with different structure
Route::prefix('api/v2')->middleware(['api', 'throttle:100,1'])->group(function () {
    
    // Simplified authentication
    Route::post('/auth/login', 'Api\\V2\\AuthController@login');
    Route::post('/auth/logout', 'Api\\V2\\AuthController@logout');
    
    // Resource routes with different naming
    Route::apiResource('people', 'Api\\V2\\PersonController');
    Route::apiResource('articles', 'Api\\V2\\ArticleController');
    
    // Bulk operations
    Route::post('/people/bulk', 'Api\\V2\\PersonController@bulkCreate');
    Route::put('/people/bulk', 'Api\\V2\\PersonController@bulkUpdate');
    Route::delete('/people/bulk', 'Api\\V2\\PersonController@bulkDelete');
});

// Public API endpoints (no authentication required)
Route::prefix('api')->group(function () {
    Route::get('/health', function () {
        return response()->json(['status' => 'ok', 'timestamp' => now()]);
    });
    
    Route::get('/status', function () {
        return response()->json([
            'api_version' => '2.0',
            'server_time' => now(),
            'environment' => app()->environment()
        ]);
    });
    
    // Documentation endpoints
    Route::get('/docs', 'DocumentationController@index');
    Route::get('/docs/{section}', 'DocumentationController@section');
});

// Webhook endpoints with special handling
Route::prefix('webhooks')->middleware('webhook.verify')->group(function () {
    Route::post('/payment/stripe', 'WebhookController@stripePayment');
    Route::post('/payment/paypal', 'WebhookController@paypalPayment');
    Route::post('/notification/slack', 'WebhookController@slackNotification');
    Route::post('/ci/github', 'WebhookController@githubCI');
});
