# FastAPI Basic Routes Test  
# Tests: @app decorators, async routes, type hints

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Optional

app = FastAPI(title="FastAPI Test App")

# Pydantic models
class User(BaseModel):
    id: Optional[int] = None
    name: str
    email: str

class Post(BaseModel):
    id: Optional[int] = None
    title: str
    content: str
    user_id: int

# Simple GET routes
@app.get("/users")
async def get_users() -> List[User]:
    """Get all users"""
    return [
        User(id=1, name="John Doe", email="john@example.com"),
        User(id=2, name="Jane Smith", email="jane@example.com")
    ]

@app.post("/users")
async def create_user(user: User) -> User:
    """Create a new user"""
    user.id = 123
    return user

@app.get("/users/{user_id}")
async def get_user(user_id: int) -> User:
    """Get user by ID"""
    return User(id=user_id, name=f"User {user_id}", email=f"user{user_id}@example.com")

@app.put("/users/{user_id}")
async def update_user(user_id: int, user: User) -> User:
    """Update user by ID"""
    user.id = user_id
    return user

@app.delete("/users/{user_id}")
async def delete_user(user_id: int):
    """Delete user by ID"""
    return {"message": f"User {user_id} deleted"}

# Posts endpoints
@app.get("/posts")
async def get_posts() -> List[Post]:
    """Get all posts"""
    return []

@app.post("/posts")
async def create_post(post: Post) -> Post:
    """Create a new post"""
    post.id = 1
    return post

@app.get("/posts/{post_id}")
async def get_post(post_id: int) -> Post:
    """Get post by ID"""
    return Post(id=post_id, title=f"Post {post_id}", content="Content", user_id=1)

# Health check
@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "ok", "service": "fastapi-app"}

# API versioning
@app.get("/api/v1/status")
async def api_v1_status():
    """API v1 status"""
    return {"version": "1.0", "status": "active"}

@app.get("/api/v2/status")
async def api_v2_status():
    """API v2 status"""
    return {"version": "2.0", "status": "active"}

# Query parameters
@app.get("/search")
async def search(q: str, limit: int = 10):
    """Search with query parameters"""
    return {"query": q, "limit": limit, "results": []}

# Path and query parameters combined
@app.get("/users/{user_id}/posts")
async def get_user_posts(user_id: int, published: Optional[bool] = None):
    """Get posts for a specific user"""
    return {"user_id": user_id, "published": published, "posts": []}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
