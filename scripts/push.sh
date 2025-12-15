#!/bin/bash

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
COMMIT_MSG="code gen - $TIMESTAMP"

echo "🔄 Committing and pushing changes in root..."
git add .
git commit -a -m "$COMMIT_MSG"
git push
echo "✅ Root committed and pushed"

if [ -d "pnix-svelte" ]; then
    echo "🔄 Committing and pushing changes in pnix-svelte..."
    cd pnix-svelte
    git add .
    git commit -a -m "$COMMIT_MSG"
    git push
    cd ..
    echo "✅ pnix-svelte committed and pushed"
fi

if [ -d "pnix-dotnet" ]; then
    echo "🔄 Committing and pushing changes in pnix-dotnet..."
    cd pnix-dotnet
    git add .
    git commit -a -m "$COMMIT_MSG"
    git push
    cd ..
    echo "✅ pnix-dotnet committed and pushed"
fi

echo "✨ All commits and pushes completed!"

