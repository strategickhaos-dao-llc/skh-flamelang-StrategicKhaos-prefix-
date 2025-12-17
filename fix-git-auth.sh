#!/bin/bash

# Git Authentication Fix Script for StrategicKhaos FlameLang Repository
# This script automates the resolution of Git authentication and permission issues

# Don't use 'set -e' as some commands are expected to fail (e.g., rebase --abort when no rebase is active)

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

# Configuration
TARGET_BRANCH="main"
FORCE_PUSH=false
ABORT_REBASE=false
SKIP_CREDENTIAL_CLEAR=false

# Helper functions
print_step() {
    echo -e "\n${CYAN}==> $1${NC}"
}

print_success() {
    echo -e "    ${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "    ${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "    ${RED}✗ $1${NC}"
}

print_info() {
    echo -e "    ${GRAY}$1${NC}"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --force-push)
            FORCE_PUSH=true
            shift
            ;;
        --abort-rebase)
            ABORT_REBASE=true
            shift
            ;;
        --skip-credential-clear)
            SKIP_CREDENTIAL_CLEAR=true
            shift
            ;;
        --branch)
            TARGET_BRANCH="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --force-push              Enable force push (WARNING: overwrites remote)"
            echo "  --abort-rebase            Abort any active rebase before proceeding"
            echo "  --skip-credential-clear   Skip clearing cached credentials"
            echo "  --branch BRANCH           Target branch (default: main)"
            echo "  --help                    Display this help message"
            echo ""
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Header
cat << "EOF"

╔════════════════════════════════════════════════════════════════╗
║  StrategicKhaos FlameLang - Git Authentication Fix Script     ║
║  Repository: skh-flamelang-StrategicKhaos-prefix-             ║
╚════════════════════════════════════════════════════════════════╝

EOF

# Step 1: Abort rebase if needed
if [ "$ABORT_REBASE" = true ] || [ -d ".git/rebase-merge" ] || [ -d ".git/rebase-apply" ]; then
    print_step "Step 1: Aborting broken rebase"
    if git rebase --abort 2>/dev/null; then
        print_success "Rebase aborted successfully"
    else
        print_warning "No rebase in progress or already aborted"
    fi
else
    print_step "Step 1: Checking rebase state"
    print_success "No active rebase detected"
fi

# Step 2: Check credential helper
print_step "Step 2: Checking credential helper configuration"
CRED_HELPER=$(git config --global credential.helper || echo "")
if [ -n "$CRED_HELPER" ]; then
    print_info "Current credential helper: $CRED_HELPER"
else
    print_success "No global credential helper configured"
fi

# Step 3: Clear cached credentials
if [ "$SKIP_CREDENTIAL_CLEAR" = false ]; then
    print_step "Step 3: Clearing cached GitHub credentials"
    print_warning "This will remove stored GitHub credentials"
    
    read -p "    Continue? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Use printf for better portability across systems
        if printf "protocol=https\nhost=github.com\n\n" | git credential reject 2>/dev/null; then
            print_success "Cached credentials cleared"
        else
            print_warning "Could not clear credentials automatically"
        fi
    else
        print_warning "Skipped credential clearing"
    fi
else
    print_step "Step 3: Skipping credential clearing (--skip-credential-clear flag)"
fi

# Step 4: Force SSH usage
print_step "Step 4: Configuring Git to use SSH instead of HTTPS"
if git config --global url."git@github.com:".insteadOf "https://github.com/"; then
    print_success "SSH override configured"
else
    print_error "Failed to configure SSH override"
fi

# Step 5: Verify SSH authentication
print_step "Step 5: Verifying SSH authentication to GitHub"
SSH_TEST=$(ssh -T git@github.com 2>&1 || true)
if echo "$SSH_TEST" | grep -q "successfully authenticated"; then
    print_success "SSH authentication successful!"
    print_info "$SSH_TEST"
else
    print_error "SSH authentication failed"
    print_info "$SSH_TEST"
    print_warning "You may need to add your SSH key to GitHub or as a deploy key"
fi

# Step 6: Check Git user configuration
print_step "Step 6: Checking Git user configuration"
CURRENT_NAME=$(git config user.name || echo "")
CURRENT_EMAIL=$(git config user.email || echo "")
print_info "Current user.name:  $CURRENT_NAME"
print_info "Current user.email: $CURRENT_EMAIL"

# Step 7: Set organization identity
print_step "Step 7: Setting organization identity"
CORRECT_NAME="strategickhaos-dao-llc"
CORRECT_EMAIL="security@strategickhaos.ai"

if [ "$CURRENT_NAME" != "$CORRECT_NAME" ] || [ "$CURRENT_EMAIL" != "$CORRECT_EMAIL" ]; then
    print_warning "Current configuration doesn't match organization identity"
    read -p "    Set to organization identity? (y/n): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git config user.name "$CORRECT_NAME"
        git config user.email "$CORRECT_EMAIL"
        print_success "User identity updated to organization account"
    fi
else
    print_success "User identity already matches organization account"
fi

# Step 8: Verify and update remote URL
print_step "Step 8: Verifying remote URL configuration"
REMOTE_URL=$(git remote get-url origin)
print_info "Current remote URL: $REMOTE_URL"

if [[ $REMOTE_URL == https://* ]]; then
    print_warning "Remote is using HTTPS, updating to SSH"
    if git remote set-url origin "git@github.com:strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-.git"; then
        print_success "Remote URL updated to SSH"
    else
        print_error "Failed to update remote URL"
    fi
else
    print_success "Remote is already using SSH"
fi

# Step 9: Checkout target branch and push
print_step "Step 9: Preparing to push changes"

# Check current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
print_info "Current branch: $CURRENT_BRANCH"

if [ "$CURRENT_BRANCH" != "$TARGET_BRANCH" ]; then
    print_warning "Not on $TARGET_BRANCH branch"
    read -p "    Checkout $TARGET_BRANCH? (y/n): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if git checkout "$TARGET_BRANCH"; then
            print_success "Switched to $TARGET_BRANCH branch"
        else
            print_error "Failed to checkout $TARGET_BRANCH branch"
            exit 1
        fi
    else
        print_warning "Staying on current branch"
    fi
fi

# Step 10: Push
print_step "Step 10: Pushing changes to remote"

if [ "$FORCE_PUSH" = true ]; then
    print_warning "FORCE PUSH requested - this will overwrite remote history!"
    read -p "    Are you absolutely sure? Type 'force' to continue: " CONFIRM_FORCE
    
    if [ "$CONFIRM_FORCE" = "force" ]; then
        if git push origin "$TARGET_BRANCH" --force; then
            print_success "Force push completed"
        else
            print_error "Force push failed"
            echo -e "\n${YELLOW}If permission was denied, you may need to add a deploy key with write access.${NC}"
            echo -e "${YELLOW}See GIT_AUTHENTICATION_FIX.md for instructions.${NC}"
        fi
    else
        print_warning "Force push cancelled"
    fi
else
    read -p "    Push to origin/$TARGET_BRANCH? (y/n): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if git push origin "$TARGET_BRANCH"; then
            print_success "Push completed successfully"
        else
            print_error "Push failed"
            echo -e "\n${YELLOW}If permission was denied, you may need to:${NC}"
            echo -e "${YELLOW}  1. Add your SSH key as a deploy key with write access${NC}"
            echo -e "${YELLOW}  2. Use --force-push flag if you need to overwrite remote${NC}"
            echo -e "\n${YELLOW}See GIT_AUTHENTICATION_FIX.md for detailed instructions.${NC}"
        fi
    else
        print_warning "Push cancelled by user"
    fi
fi

# Summary
cat << EOF

╔════════════════════════════════════════════════════════════════╗
║  Script Execution Complete                                     ║
╚════════════════════════════════════════════════════════════════╝

Next Steps:
  • If push was denied, add deploy key via GitHub web UI
  • Repository: https://github.com/strategickhaos-dao-llc/skh-flamelang-StrategicKhaos-prefix-/settings/keys
  • See GIT_AUTHENTICATION_FIX.md for complete instructions

Usage Examples:
  ./fix-git-auth.sh                           # Interactive mode
  ./fix-git-auth.sh --abort-rebase            # Abort rebase first
  ./fix-git-auth.sh --force-push              # Enable force push option
  ./fix-git-auth.sh --skip-credential-clear   # Skip credential clearing
  ./fix-git-auth.sh --branch develop          # Target different branch

EOF
