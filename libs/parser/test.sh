#!/bin/bash
# PinPath Parser Test Runner
# Demonstrates granular test execution using Nx

set -e

echo "🧪 PinPath Parser Test Suite"
echo "=============================="

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_section() {
    echo -e "\n${BLUE}$1${NC}"
    echo "----------------------------------------"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if nx is available
if ! command -v nx &> /dev/null; then
    print_error "Nx is not available. Please install it globally with: npm install -g nx"
    exit 1
fi

print_success "Nx detected - ready to run tests"

print_section "Available Test Commands"
echo "Fast development tests:"
echo "  nx run parser:test:fast"
echo ""
echo "Unit tests by framework:"
echo "  nx run parser:test:unit:nextjs"
echo "  nx run parser:test:unit:express"
echo "  nx run parser:test:unit:flask"
echo "  nx run parser:test:unit:fastapi"
echo "  nx run parser:test:unit:laravel"
echo ""
echo "Integration tests by category:"
echo "  nx run parser:test:projects:startup-monorepo"
echo "  nx run parser:test:projects:single-app"
echo "  nx run parser:test:projects:performance"
echo ""
echo "Language-specific tests:"
echo "  nx run parser:test:javascript"
echo "  nx run parser:test:python"
echo "  nx run parser:test:php"
echo ""
echo "All tests:"
echo "  nx run parser:test"
echo ""

# If no arguments provided, show help
if [ $# -eq 0 ]; then
    print_section "Usage"
    echo "  ./test.sh [command]"
    echo ""
    echo "Commands:"
    echo "  all       - Run all tests"
    echo "  fast      - Run fast unit tests only"
    echo "  unit      - Run all unit tests"
    echo "  projects  - Run all project integration tests"
    echo "  nextjs    - Run Next.js specific tests"
    echo "  express   - Run Express.js specific tests"
    echo "  flask     - Run Flask specific tests"
    echo "  ci        - Run CI-optimized tests"
    echo "  help      - Show this help"
    exit 0
fi

# Parse command
case $1 in
    "all")
        print_section "Running All Tests"
        nx run parser:test
        print_success "All tests completed"
        ;;
    "fast")
        print_section "Running Fast Unit Tests"
        nx run parser:test:fast
        print_success "Fast tests completed"
        ;;
    "unit")
        print_section "Running All Unit Tests"
        nx run parser:test:unit
        print_success "Unit tests completed"
        ;;
    "projects")
        print_section "Running Project Integration Tests"
        nx run parser:test:projects
        print_success "Project tests completed"
        ;;
    "nextjs")
        print_section "Running Next.js Tests"
        nx run parser:test:unit:nextjs
        print_success "Next.js tests completed"
        ;;
    "express")
        print_section "Running Express.js Tests"
        nx run parser:test:unit:express
        print_success "Express.js tests completed"
        ;;
    "flask")
        print_section "Running Flask Tests"
        nx run parser:test:unit:flask
        print_success "Flask tests completed"
        ;;
    "ci")
        print_section "Running CI Tests"
        nx run parser:test:ci
        print_success "CI tests completed"
        ;;
    "help")
        # Already handled above
        ;;
    *)
        print_error "Unknown command: $1"
        print_warning "Use './test.sh help' to see available commands"
        exit 1
        ;;
esac
