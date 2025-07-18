// Gin Basic Routes Test
// Tests: Basic HTTP methods with Gin framework

package main

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

type User struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

type Post struct {
	ID     int    `json:"id"`
	Title  string `json:"title"`
	UserID int    `json:"user_id"`
}

func main() {
	r := gin.Default()

	// Simple CRUD routes
	r.GET("/users", getUsers)
	r.POST("/users", createUser)
	r.GET("/users/:id", getUser)
	r.PUT("/users/:id", updateUser)
	r.DELETE("/users/:id", deleteUser)

	// Posts routes
	r.GET("/api/posts", getPosts)
	r.POST("/api/posts", createPost)
	r.GET("/api/posts/:id", getPost)

	// Health check
	r.GET("/health", healthCheck)

	// Admin routes
	r.GET("/admin/stats", adminStats)

	r.Run(":8080")
}

func getUsers(c *gin.Context) {
	users := []User{
		{ID: 1, Name: "John Doe"},
		{ID: 2, Name: "Jane Smith"},
	}
	c.JSON(http.StatusOK, users)
}

func createUser(c *gin.Context) {
	var user User
	if err := c.ShouldBindJSON(&user); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	user.ID = 123
	c.JSON(http.StatusCreated, user)
}

func getUser(c *gin.Context) {
	id, _ := strconv.Atoi(c.Param("id"))
	user := User{ID: id, Name: "User " + c.Param("id")}
	c.JSON(http.StatusOK, user)
}

func updateUser(c *gin.Context) {
	id, _ := strconv.Atoi(c.Param("id"))
	var user User
	if err := c.ShouldBindJSON(&user); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	user.ID = id
	c.JSON(http.StatusOK, user)
}

func deleteUser(c *gin.Context) {
	c.Status(http.StatusNoContent)
}

func getPosts(c *gin.Context) {
	c.JSON(http.StatusOK, []Post{})
}

func createPost(c *gin.Context) {
	var post Post
	if err := c.ShouldBindJSON(&post); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	post.ID = 1
	c.JSON(http.StatusCreated, post)
}

func getPost(c *gin.Context) {
	id, _ := strconv.Atoi(c.Param("id"))
	post := Post{ID: id, Title: "Post " + c.Param("id"), UserID: 1}
	c.JSON(http.StatusOK, post)
}

func healthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":  "ok",
		"service": "gin-app",
	})
}

func adminStats(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"users": 100,
		"posts": 50,
	})
}
