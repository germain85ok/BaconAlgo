#!/bin/bash
# Supabase Integration Verification Script

echo "🥓 BaconAlgo Supabase Integration Verification"
echo "=============================================="
echo ""

# Check if files exist
echo "📁 Checking files..."
files=(
    "station/src/lib/supabase/client.ts"
    "supabase/migrations/001_initial_schema.sql"
    "supabase/README.md"
)

all_exist=true
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file"
    else
        echo "❌ $file (missing)"
        all_exist=false
    fi
done
echo ""

# Check package.json for dependency
echo "📦 Checking dependencies..."
if grep -q "@supabase/supabase-js" station/package.json; then
    echo "✅ @supabase/supabase-js dependency found"
else
    echo "❌ @supabase/supabase-js dependency missing"
    all_exist=false
fi
echo ""

# Count database objects in migration
echo "🗄️  Database schema objects..."
tables=$(grep -c "^CREATE TABLE" supabase/migrations/001_initial_schema.sql)
functions=$(grep -c "^CREATE.*FUNCTION" supabase/migrations/001_initial_schema.sql)
triggers=$(grep -c "^CREATE TRIGGER" supabase/migrations/001_initial_schema.sql)
indexes=$(grep -c "^CREATE INDEX" supabase/migrations/001_initial_schema.sql)
policies=$(grep -c "^CREATE POLICY" supabase/migrations/001_initial_schema.sql)
enums=$(grep -c "^CREATE TYPE" supabase/migrations/001_initial_schema.sql)

echo "  Tables: $tables"
echo "  Functions: $functions"
echo "  Triggers: $triggers"
echo "  Indexes: $indexes"
echo "  Policies: $policies"
echo "  Enums: $enums"
echo ""

# Check TypeScript exports
echo "📝 Checking TypeScript exports..."
exports=(
    "submitDonation"
    "getDonations"
    "getSignals"
    "subscribeToSignals"
    "getSponsors"
    "getSongRequests"
)

for export in "${exports[@]}"; do
    if grep -q "export.*function $export" station/src/lib/supabase/client.ts; then
        echo "✅ $export"
    else
        echo "❌ $export (missing)"
        all_exist=false
    fi
done
echo ""

# Summary
echo "=============================================="
if [ "$all_exist" = true ]; then
    echo "✅ All checks passed!"
    echo ""
    echo "Next steps:"
    echo "1. Add SUPABASE_URL and SUPABASE_ANON_KEY to your .env file"
    echo "2. Run the migration: supabase db push"
    echo "3. Import and use the client in your app"
    exit 0
else
    echo "❌ Some checks failed!"
    exit 1
fi
