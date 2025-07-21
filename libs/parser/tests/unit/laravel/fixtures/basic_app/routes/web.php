<?php

use Illuminate\Support\Facades\Route;

Route::get('/', function () {
    return view('welcome');
});

Route::get('/users', function () {
    return response()->json(['users' => []]);
});

Route::post('/users', function () {
    return response()->json(['id' => 1, 'name' => 'New User']);
});
