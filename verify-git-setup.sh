#!/bin/bash

# Git Setup Verification Script
# This script verifies that your local git repository is correctly configured to push to the remote

echo "================================"
echo "Git Configuration Verification"
echo "================================"
echo ""

# Check if we're in a git repository
if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
    echo "❌ ERROR: Not in a git repository"
    exit 1
fi

echo "✅ In git repository"
echo ""

# Check current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current Branch: $CURRENT_BRANCH"
echo ""

# Check remote configuration
echo "🌐 Remote Configuration:"
git remote -v
echo ""

# Check if branch has upstream tracking
UPSTREAM=$(git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null)
if [ -n "$UPSTREAM" ]; then
    echo "✅ Branch tracking configured: $UPSTREAM"
else
    echo "⚠️  WARNING: Branch is not tracking a remote branch"
    echo "   To set up tracking, run: git push -u origin $CURRENT_BRANCH"
fi
echo ""

# Check connection to remote
echo "🔌 Testing connection to remote..."
if git ls-remote origin HEAD > /dev/null 2>&1; then
    echo "✅ Successfully connected to remote repository"
    REMOTE_HEAD=$(git ls-remote origin HEAD | awk '{print $1}')
    echo "   Remote HEAD: $REMOTE_HEAD"
else
    echo "❌ ERROR: Cannot connect to remote repository"
    exit 1
fi
echo ""

# Check for uncommitted changes
if git diff-index --quiet HEAD -- 2>/dev/null; then
    echo "✅ No uncommitted changes"
else
    echo "📝 You have uncommitted changes:"
    git status --short
fi
echo ""

# Check if local is ahead/behind remote
if [ -n "$UPSTREAM" ]; then
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u} 2>/dev/null)
    BASE=$(git merge-base @ @{u} 2>/dev/null)
    
    if [ "$LOCAL" = "$REMOTE" ]; then
        echo "✅ Local branch is up to date with remote"
    elif [ "$LOCAL" = "$BASE" ]; then
        echo "⬇️  Local branch is behind remote (need to pull)"
        echo "   Run: git pull"
    elif [ "$REMOTE" = "$BASE" ]; then
        echo "⬆️  Local branch is ahead of remote (ready to push)"
        AHEAD=$(git rev-list --count @{u}..HEAD)
        echo "   Commits ahead: $AHEAD"
        echo "   Run: git push"
    else
        echo "🔀 Local and remote have diverged (need to merge)"
        echo "   Run: git pull --rebase"
    fi
fi
echo ""

# Summary
echo "================================"
echo "Summary"
echo "================================"

# Only show success summary if all checks passed
if [ -n "$UPSTREAM" ]; then
    echo "✅ Repository is properly configured for pushing"
    echo "✅ Remote repository is accessible"
    echo ""
    echo "To push your changes, simply run:"
    echo "  git push"
else
    echo "⚠️  Repository needs configuration"
    echo "   Run: git push -u origin $CURRENT_BRANCH"
fi
echo ""
echo "For more detailed instructions, see GIT_PUSH_GUIDE.md"
