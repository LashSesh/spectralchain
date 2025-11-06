#!/bin/bash
# Daily Health Check for SpectralChain
# Runs comprehensive health checks and generates report

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DATE=$(date +%Y%m%d)
REPORT_FILE="$PROJECT_ROOT/.ai/reports/daily-health-$DATE.json"

echo "🏥 SpectralChain Daily Health Check"
echo "===================================="
echo "Date: $(date)"
echo ""

# Initialize report
cat > "$REPORT_FILE" << EOF
{
  "date": "$(date -Iseconds)",
  "version": "1.0.0",
  "checks": []
}
EOF

# Function to add check result
add_check() {
    local name="$1"
    local status="$2"
    local details="$3"

    python3 -c "
import json
import sys

with open('$REPORT_FILE', 'r') as f:
    data = json.load(f)

data['checks'].append({
    'name': '$name',
    'status': '$status',
    'details': '$details',
    'timestamp': '$(date -Iseconds)'
})

with open('$REPORT_FILE', 'w') as f:
    json.dump(data, f, indent=2)
"
}

# Check 1: Build Status
echo "📦 Checking build status..."
if cargo build --workspace --quiet 2>&1 | tee /tmp/build.log; then
    add_check "build" "pass" "Build successful"
    echo "✅ Build: PASS"
else
    add_check "build" "fail" "$(cat /tmp/build.log | tail -10)"
    echo "❌ Build: FAIL"
fi

# Check 2: Test Status
echo ""
echo "🧪 Running tests..."
if cargo test --workspace --quiet 2>&1 | tee /tmp/test.log; then
    TEST_COUNT=$(grep -o "test result: ok" /tmp/test.log | wc -l)
    add_check "tests" "pass" "All tests passing ($TEST_COUNT test suites)"
    echo "✅ Tests: PASS ($TEST_COUNT suites)"
else
    FAILED=$(grep -o "failures:" /tmp/test.log | wc -l)
    add_check "tests" "fail" "Test failures detected: $FAILED"
    echo "❌ Tests: FAIL ($FAILED failures)"
fi

# Check 3: Clippy Lints
echo ""
echo "🔍 Running clippy..."
if cargo clippy --workspace --quiet -- -D warnings 2>&1 | tee /tmp/clippy.log; then
    add_check "clippy" "pass" "No warnings"
    echo "✅ Clippy: PASS"
else
    WARNINGS=$(grep "warning:" /tmp/clippy.log | wc -l)
    add_check "clippy" "fail" "$WARNINGS warnings"
    echo "❌ Clippy: $WARNINGS warnings"
fi

# Check 4: Formatting
echo ""
echo "📝 Checking formatting..."
if cargo fmt --check 2>&1; then
    add_check "formatting" "pass" "Code properly formatted"
    echo "✅ Formatting: PASS"
else
    add_check "formatting" "fail" "Code needs formatting"
    echo "❌ Formatting: FAIL"
fi

# Check 5: Security Audit
echo ""
echo "🔒 Running security audit..."
if cargo audit 2>&1 | tee /tmp/audit.log; then
    add_check "security_audit" "pass" "No vulnerabilities"
    echo "✅ Security: PASS"
else
    VULNS=$(grep "Crate:" /tmp/audit.log | wc -l)
    add_check "security_audit" "fail" "$VULNS vulnerabilities"
    echo "❌ Security: $VULNS vulnerabilities"
fi

# Check 6: Documentation
echo ""
echo "📚 Checking documentation..."
if cargo doc --workspace --no-deps --quiet 2>&1 | tee /tmp/doc.log; then
    MISSING_DOCS=$(grep "warning: missing documentation" /tmp/doc.log | wc -l || echo "0")
    if [ "$MISSING_DOCS" -eq 0 ]; then
        add_check "documentation" "pass" "All public items documented"
        echo "✅ Documentation: PASS"
    else
        add_check "documentation" "warn" "$MISSING_DOCS items missing docs"
        echo "⚠️  Documentation: $MISSING_DOCS missing"
    fi
else
    add_check "documentation" "fail" "Doc generation failed"
    echo "❌ Documentation: FAIL"
fi

# Calculate overall health score
python3 -c "
import json

with open('$REPORT_FILE', 'r') as f:
    data = json.load(f)

total = len(data['checks'])
passed = sum(1 for c in data['checks'] if c['status'] == 'pass')
warned = sum(1 for c in data['checks'] if c['status'] == 'warn')
failed = sum(1 for c in data['checks'] if c['status'] == 'fail')

score = (passed + warned * 0.5) / total * 100 if total > 0 else 0

data['summary'] = {
    'total_checks': total,
    'passed': passed,
    'warned': warned,
    'failed': failed,
    'health_score': round(score, 1),
    'status': 'healthy' if score >= 80 else 'degraded' if score >= 60 else 'unhealthy'
}

with open('$REPORT_FILE', 'w') as f:
    json.dump(data, f, indent=2)

print(f\"\\n📊 Health Score: {score:.1f}/100\")
print(f\"✅ Passed: {passed}\")
print(f\"⚠️  Warned: {warned}\")
print(f\"❌ Failed: {failed}\")
print(f\"\\nReport saved to: $REPORT_FILE\")
"

# Cleanup temp files
rm -f /tmp/build.log /tmp/test.log /tmp/clippy.log /tmp/audit.log /tmp/doc.log

echo ""
echo "✅ Daily health check complete!"
