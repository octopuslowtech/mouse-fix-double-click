#!/bin/bash

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
COMMIT_MSG="code gen - $TIMESTAMP"

echo "🔄 Committing changes in root..."
git add .
git commit -a -m "$COMMIT_MSG"
echo "✅ Root committed"

if [ -d "pnix-svelte" ]; then
    echo "🔄 Committing changes in pnix-svelte..."
    cd pnix-svelte
    git add .
    git commit -a -m "$COMMIT_MSG"
    cd ..
    echo "✅ pnix-svelte committed"
fi

if [ -d "pnix-dotnet" ]; then
    echo "🔄 Committing changes in pnix-dotnet..."
    cd pnix-dotnet
    git add .
    git commit -a -m "$COMMIT_MSG"
    cd ..
    echo "✅ pnix-dotnet committed"
fi

echo "✨ All commits completed!"

