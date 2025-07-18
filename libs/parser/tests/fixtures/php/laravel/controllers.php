<?php

use Illuminate\Support\Facades\Route;

/*
|--------------------------------------------------------------------------
| Laravel Controller Routes Test
| Tests: Controller method routing, middleware groups, nested routes
|--------------------------------------------------------------------------
*/

// Admin routes with middleware group
Route::middleware(['auth', 'admin'])->prefix('admin')->group(function () {
    Route::get('/dashboard', 'AdminController@dashboard');
    Route::get('/users', 'AdminController@users');
    Route::post('/users/{id}/ban', 'AdminController@banUser');
    Route::delete('/users/{id}', 'AdminController@deleteUser');
    
    // Nested admin routes
    Route::prefix('reports')->group(function () {
        Route::get('/sales', 'AdminController@salesReport');
        Route::get('/users', 'AdminController@usersReport');
        Route::post('/export', 'AdminController@exportReports');
    });
});

// Public API routes
Route::prefix('api')->group(function () {
    Route::get('/status', 'ApiController@status');
    Route::get('/version', 'ApiController@version');
    
    // Versioned API routes
    Route::prefix('v1')->group(function () {
        Route::get('/users', 'Api\\V1\\UserController@index');
        Route::post('/users', 'Api\\V1\\UserController@store');
        Route::get('/users/{id}', 'Api\\V1\\UserController@show');
        Route::put('/users/{id}', 'Api\\V1\\UserController@update');
        Route::delete('/users/{id}', 'Api\\V1\\UserController@destroy');
    });
    
    Route::prefix('v2')->group(function () {
        Route::get('/users', 'Api\\V2\\UserController@index');
        Route::post('/users', 'Api\\V2\\UserController@store');
    });
});

// Authentication routes
Route::post('/login', 'AuthController@login');
Route::post('/logout', 'AuthController@logout');
Route::post('/register', 'AuthController@register');
Route::post('/password/reset', 'AuthController@resetPassword');

// File upload routes
Route::post('/upload/avatar', 'UploadController@avatar');
Route::post('/upload/documents', 'UploadController@documents');
Route::delete('/uploads/{id}', 'UploadController@delete');

// Social media integration
Route::get('/auth/github', 'SocialController@redirectToGithub');
Route::get('/auth/github/callback', 'SocialController@handleGithubCallback');
Route::get('/auth/google', 'SocialController@redirectToGoogle');
Route::get('/auth/google/callback', 'SocialController@handleGoogleCallback');

// Webhook routes
Route::post('/webhooks/stripe', 'WebhookController@stripe');
Route::post('/webhooks/github', 'WebhookController@github');
Route::post('/webhooks/slack', 'WebhookController@slack');
