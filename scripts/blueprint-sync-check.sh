#!/bin/bash
# Blueprint Compliance Check for SpectralChain
# Verifies implementation matches original Blueprint specifications

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DATE=$(date +%Y%m%d)
REPORT_FILE="$PROJECT_ROOT/BLUEPRINT_COMPLIANCE_$DATE.md"

echo "🔍 Blueprint Compliance Check"
echo "============================="
echo ""

# Initialize report
cat > "$REPORT_FILE" << EOF
# Blueprint Compliance Report

**Generated**: $(date)
**Blueprint**: Quantenresonante Spektralfeld-Blockchain by Sebastian Klemm

---

## Executive Summary

This report verifies that the SpectralChain implementation matches the original Blueprint specifications.

EOF

# 1. Verify Operator Implementations
echo "🧮 Checking Operator Implementations..."
echo "## 1. Operator Implementations" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

check_operator() {
    local name="$1"
    local file="$2"
    local required_functions="$3"

    echo "### $name" >> "$REPORT_FILE"

    if [ -f "$PROJECT_ROOT/$file" ]; then
        echo "✅ **File**: $file exists" >> "$REPORT_FILE"

        # Check for required functions
        for func in $required_functions; do
            if grep -q "pub fn $func" "$PROJECT_ROOT/$file"; then
                echo "  - ✅ Function \`$func\` implemented" >> "$REPORT_FILE"
            else
                echo "  - ❌ Function \`$func\` missing" >> "$REPORT_FILE"
            fi
        done
    else
        echo "❌ **File**: $file not found" >> "$REPORT_FILE"
    fi
    echo "" >> "$REPORT_FILE"
}

# Check Masking Operator
check_operator "Masking Operator (M)" \
    "mef-quantum-ops/src/masking.rs" \
    "mask unmask new"

# Check Resonance Operator
check_operator "Resonance Operator (R_ε)" \
    "mef-quantum-ops/src/resonance.rs" \
    "is_resonant resonance_strength new"

# Check Steganography Operator
check_operator "Steganography Operator (T)" \
    "mef-quantum-ops/src/steganography.rs" \
    "embed extract"

# Check ZK Proof Operator
check_operator "Zero-Knowledge Proof Operator (ZK)" \
    "mef-quantum-ops/src/zk_proofs.rs" \
    "generate_proof verify_proof"

# 2. Verify Protocol Flow
echo ""
echo "📡 Checking Ghost Protocol Flow..."
echo "## 2. Ghost Protocol Flow" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if [ -f "$PROJECT_ROOT/mef-ghost-network/src/protocol.rs" ]; then
    echo "✅ **Protocol File**: mef-ghost-network/src/protocol.rs exists" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"

    PROTOCOL_FILE="$PROJECT_ROOT/mef-ghost-network/src/protocol.rs"

    # Check 6-step protocol
    STEPS=("create_transaction" "apply_masking" "apply_steganography" "broadcast" "receive_and_check" "commit_to_ledger")

    echo "**Required Steps**:" >> "$REPORT_FILE"
    for step in "${STEPS[@]}"; do
        if grep -q "$step" "$PROTOCOL_FILE"; then
            echo "  - ✅ $step" >> "$REPORT_FILE"
        else
            echo "  - ⚠️ $step (may have different name)" >> "$REPORT_FILE"
        fi
    done
else
    echo "❌ **Protocol File**: mef-ghost-network/src/protocol.rs not found" >> "$REPORT_FILE"
fi

echo "" >> "$REPORT_FILE"

# 3. Verify Mathematical Properties via Tests
echo ""
echo "🧪 Checking Mathematical Properties..."
echo "## 3. Mathematical Properties (via Tests)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

check_property_test() {
    local property="$1"
    local test_name="$2"
    local package="$3"

    if cargo test --package "$package" "$test_name" --quiet 2>&1 > /dev/null; then
        echo "  - ✅ $property: \`$test_name\` passes" >> "$REPORT_FILE"
    else
        echo "  - ❌ $property: \`$test_name\` fails" >> "$REPORT_FILE"
    fi
}

echo "### Masking Operator Properties" >> "$REPORT_FILE"
check_property_test "Self-Inverse" "test_masking_is_reversible" "mef-quantum-ops"
check_property_test "Deterministic" "test_masking_is_deterministic" "mef-quantum-ops"

echo "" >> "$REPORT_FILE"
echo "### Resonance Operator Properties" >> "$REPORT_FILE"
check_property_test "Symmetric" "test_resonance_is_symmetric" "mef-quantum-ops"
check_property_test "Bounded" "test_resonance_bounds" "mef-quantum-ops"

echo "" >> "$REPORT_FILE"

# 4. Verify Architecture Principles
echo ""
echo "🏗️ Checking Architecture Principles..."
echo "## 4. Architecture Principles" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Check ADD-ONLY principle
echo "### ADD-ONLY Integration" >> "$REPORT_FILE"
if git diff --exit-code resources_dev/infinityledger/ > /dev/null 2>&1; then
    echo "✅ **Infinity Ledger**: No modifications detected" >> "$REPORT_FILE"
else
    echo "❌ **Infinity Ledger**: MODIFICATIONS DETECTED - VIOLATION!" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "\`\`\`" >> "$REPORT_FILE"
    git diff --stat resources_dev/infinityledger/ >> "$REPORT_FILE" 2>&1 || true
    echo "\`\`\`" >> "$REPORT_FILE"
fi

echo "" >> "$REPORT_FILE"

# Check Determinism
echo "### Determinism" >> "$REPORT_FILE"
echo "Checking for non-deterministic patterns..." >> "$REPORT_FILE"

if rg "thread_rng\(\)" --type rust > /dev/null 2>&1; then
    COUNT=$(rg "thread_rng\(\)" --type rust | wc -l)
    echo "⚠️ Found $COUNT uses of \`thread_rng()\` - may violate determinism" >> "$REPORT_FILE"
else
    echo "✅ No \`thread_rng()\` usage detected" >> "$REPORT_FILE"
fi

if rg "SystemTime::now\(\).*unwrap\(\)" --type rust > /dev/null 2>&1; then
    COUNT=$(rg "SystemTime::now\(\).*unwrap\(\)" --type rust | wc -l)
    echo "⚠️ Found $COUNT unsafe timestamp operations" >> "$REPORT_FILE"
else
    echo "✅ No unsafe timestamp operations" >> "$REPORT_FILE"
fi

echo "" >> "$REPORT_FILE"

# 5. Overall Compliance Score
echo ""
echo "📊 Calculating Compliance Score..."
echo "## 5. Overall Compliance Score" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Run compliance checks and calculate score
python3 << 'PYTHON' >> "$REPORT_FILE"
import subprocess
import os

checks = {
    'Masking Operator File': os.path.exists('mef-quantum-ops/src/masking.rs'),
    'Resonance Operator File': os.path.exists('mef-quantum-ops/src/resonance.rs'),
    'Steganography Operator File': os.path.exists('mef-quantum-ops/src/steganography.rs'),
    'ZK Proof Operator File': os.path.exists('mef-quantum-ops/src/zk_proofs.rs'),
    'Ghost Protocol File': os.path.exists('mef-ghost-network/src/protocol.rs'),
    'Infinity Ledger Unchanged': subprocess.run(['git', 'diff', '--exit-code', 'resources_dev/infinityledger/'],
                                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0,
}

passed = sum(1 for v in checks.values() if v)
total = len(checks)
score = (passed / total * 100) if total > 0 else 0

print(f"**Compliance Score**: {score:.1f}%")
print("")
print(f"**Checks Passed**: {passed}/{total}")
print("")

if score >= 95:
    print("🎉 **Status**: EXCELLENT - Fully compliant with Blueprint")
elif score >= 80:
    print("✅ **Status**: GOOD - Minor deviations")
elif score >= 60:
    print("⚠️ **Status**: FAIR - Some compliance issues")
else:
    print("❌ **Status**: POOR - Major compliance issues")

print("")
print("### Detailed Results")
print("")
for check, result in checks.items():
    status = "✅" if result else "❌"
    print(f"- {status} {check}")
PYTHON

# 6. Recommendations
echo "" >> "$REPORT_FILE"
echo "## 6. Recommendations" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

if git diff --exit-code resources_dev/infinityledger/ > /dev/null 2>&1; then
    echo "✅ No critical issues detected." >> "$REPORT_FILE"
else
    echo "❌ **CRITICAL**: Infinity Ledger has been modified! Revert changes immediately." >> "$REPORT_FILE"
fi

echo "" >> "$REPORT_FILE"
echo "### Next Steps" >> "$REPORT_FILE"
echo "1. Review any failed checks" >> "$REPORT_FILE"
echo "2. Ensure all property tests pass" >> "$REPORT_FILE"
echo "3. Verify determinism in all modules" >> "$REPORT_FILE"
echo "4. Run full test suite: \`cargo test --workspace\`" >> "$REPORT_FILE"

echo "" >> "$REPORT_FILE"
echo "---" >> "$REPORT_FILE"
echo "**Report Generated**: $(date)" >> "$REPORT_FILE"
echo "**Next Check**: Run before every release" >> "$REPORT_FILE"

echo "✅ Blueprint compliance check complete!"
echo "📄 Report saved to: $REPORT_FILE"
echo ""
cat "$REPORT_FILE"
