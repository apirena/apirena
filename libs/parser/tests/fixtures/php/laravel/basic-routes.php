<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\UserController;
use App\Http\Controllers\PostController;
use App\Http\Controllers\HealthController;

/*
|--------------------------------------------------------------------------
| Laravel Basic Routes Test
| Tests: Route facade methods, controller actions, resource routes
|--------------------------------------------------------------------------
*/

// Basic CRUD routes with controller actions
Route::get('/users', [UserController::class, 'index']);
Route::post('/users', [UserController::class, 'store']);
Route::get('/users/{id}', [UserController::class, 'show']);
Route::put('/users/{id}', [UserController::class, 'update']);
Route::delete('/users/{id}', [UserController::class, 'destroy']);

// API namespace routes
Route::get('/api/posts', [PostController::class, 'index']);
Route::post('/api/posts', [PostController::class, 'store']);

// Health check route
Route::get('/health', [HealthController::class, 'check']);

// Alternative syntax with string controller references
Route::get('/legacy/users', 'UserController@index');
Route::post('/legacy/users', 'UserController@store');

// Resource routes (generates multiple endpoints)
Route::resource('articles', 'ArticleController');

// API resource routes (excludes create/edit forms)
Route::apiResource('products', 'ProductController');

// Routes with middleware
Route::middleware(['auth'])->group(function () {
    Route::get('/profile', [UserController::class, 'profile']);
    Route::put('/profile', [UserController::class, 'updateProfile']);
});

// Routes with route model binding
Route::get('/users/{user}/posts', [PostController::class, 'userPosts']);
Route::get('/users/{user}/posts/{post}', [PostController::class, 'show']);

// Closure routes
Route::get('/ping', function () {
    return response()->json(['status' => 'pong']);
});

Route::post('/webhook', function () {
    return response()->json(['received' => true]);
});

// Routes with constraints
Route::get('/users/{id}', [UserController::class, 'show'])->where('id', '[0-9]+');
Route::get('/posts/{slug}', [PostController::class, 'showBySlug'])->where('slug', '[a-z0-9-]+');

// Named routes
Route::get('/dashboard', [DashboardController::class, 'index'])->name('dashboard');
Route::get('/settings', [SettingsController::class, 'index'])->name('settings');

// Routes with optional parameters
Route::get('/search/{term?}', [SearchController::class, 'search']);
Route::get('/archive/{year}/{month?}', [ArchiveController::class, 'show']);
