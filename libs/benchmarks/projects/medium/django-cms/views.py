from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods
import json

# User views
@require_http_methods(["GET"])
def user_list(request):
    """Get list of all users"""
    users = User.objects.all()
    return JsonResponse({'users': [user.to_dict() for user in users]})

@csrf_exempt
@require_http_methods(["POST"])
def user_create(request):
    """Create a new user"""
    data = json.loads(request.body)
    user = User.objects.create(**data)
    return JsonResponse(user.to_dict(), status=201)

@require_http_methods(["GET"])
def user_detail(request, user_id):
    """Get specific user details"""
    try:
        user = User.objects.get(id=user_id)
        return JsonResponse(user.to_dict())
    except User.DoesNotExist:
        return JsonResponse({'error': 'User not found'}, status=404)

@csrf_exempt
@require_http_methods(["PUT"])
def user_update(request, user_id):
    """Update user information"""
    try:
        user = User.objects.get(id=user_id)
        data = json.loads(request.body)
        for key, value in data.items():
            setattr(user, key, value)
        user.save()
        return JsonResponse(user.to_dict())
    except User.DoesNotExist:
        return JsonResponse({'error': 'User not found'}, status=404)

@csrf_exempt
@require_http_methods(["DELETE"])
def user_delete(request, user_id):
    """Delete a user"""
    try:
        user = User.objects.get(id=user_id)
        user.delete()
        return JsonResponse({'deleted': True})
    except User.DoesNotExist:
        return JsonResponse({'error': 'User not found'}, status=404)

# Post views
@require_http_methods(["GET"])
def post_list(request):
    """Get list of all posts"""
    posts = Post.objects.all()
    return JsonResponse({'posts': [post.to_dict() for post in posts]})

@csrf_exempt
@require_http_methods(["POST"])
def post_create(request):
    """Create a new post"""
    data = json.loads(request.body)
    post = Post.objects.create(**data)
    return JsonResponse(post.to_dict(), status=201)

@require_http_methods(["GET"])
def post_detail(request, post_id):
    """Get specific post details"""
    try:
        post = Post.objects.get(id=post_id)
        return JsonResponse(post.to_dict())
    except Post.DoesNotExist:
        return JsonResponse({'error': 'Post not found'}, status=404)
