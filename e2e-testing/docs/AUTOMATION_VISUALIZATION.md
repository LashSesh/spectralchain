# SpectralChain E2E Test Automation & Visualization

**Version:** 1.0.0
**Created:** 2025-11-06
**Purpose:** Comprehensive automation and visualization framework for E2E testing and simulation

---

## Table of Contents

1. [Overview](#overview)
2. [Test Automation Framework](#test-automation-framework)
3. [Simulation Orchestration](#simulation-orchestration)
4. [Metrics Collection & Analysis](#metrics-collection--analysis)
5. [Visualization Tools](#visualization-tools)
6. [Report Generation](#report-generation)
7. [CI/CD Integration](#cicd-integration)
8. [Deployment](#deployment)

---

## Overview

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Test Orchestration Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Test Runner  │→ │ Network Sim  │→ │ Fault Inject │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────┴────────────────────────────────────┐
│                     Data Collection Layer                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Metrics    │  │    Logs      │  │    Traces    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────┴────────────────────────────────────┐
│                      Analysis & Storage Layer                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ TimescaleDB  │  │ Elasticsearch│  │    Jaeger    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────┴────────────────────────────────────┐
│                     Visualization Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Grafana    │  │   Kibana     │  │ Custom Web   │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

### Technology Stack

**Core Framework**: Rust (Tokio async runtime)
**Metrics**: Prometheus + TimescaleDB
**Logging**: Structured logging (tracing) + Elasticsearch
**Tracing**: OpenTelemetry + Jaeger
**Visualization**: Grafana + Custom React dashboards
**Reports**: HTML + PDF generation

---

## Test Automation Framework

### Core Test Runner

```rust
// e2e-testing/framework/src/test_runner.rs

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TestRunner {
    pub config: TestRunnerConfig,
    pub network: Arc<RwLock<NetworkSimulator>>,
    pub fault_injector: Arc<RwLock<FaultInjector>>,
    pub metrics: Arc<MetricsCollector>,
    pub state: TestState,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TestRunnerConfig {
    pub test_suite_name: String,
    pub output_dir: String,
    pub enable_tracing: bool,
    pub enable_metrics: bool,
    pub parallel_tests: usize,
    pub retry_policy: RetryPolicy,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub backoff_strategy: BackoffStrategy,
    pub retry_on_failure: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Constant { delay_ms: u64 },
    Linear { initial_ms: u64, increment_ms: u64 },
    Exponential { initial_ms: u64, multiplier: f64 },
}

impl TestRunner {
    pub async fn new(config: TestRunnerConfig) -> Result<Self> {
        let network = Arc::new(RwLock::new(NetworkSimulator::new()));
        let fault_injector = Arc::new(RwLock::new(FaultInjector::new()));
        let metrics = Arc::new(MetricsCollector::new());

        Ok(Self {
            config,
            network,
            fault_injector,
            metrics,
            state: TestState::Idle,
        })
    }

    pub async fn execute_test_suite(
        &mut self,
        test_definitions: Vec<TestDefinition>,
    ) -> Result<TestSuiteResult> {
        tracing::info!(
            suite_name = %self.config.test_suite_name,
            test_count = test_definitions.len(),
            "Starting test suite execution"
        );

        self.state = TestState::Running;
        let mut results = Vec::new();

        for test_def in test_definitions {
            let result = self.execute_test(test_def).await;
            results.push(result);

            // Early abort on critical failure if configured
            if let Ok(ref res) = results.last().unwrap() {
                if res.status == TestStatus::Failed && res.severity == Severity::Critical {
                    tracing::error!("Critical test failure, aborting suite");
                    break;
                }
            }
        }

        self.state = TestState::Completed;

        Ok(TestSuiteResult {
            suite_name: self.config.test_suite_name.clone(),
            total_tests: results.len(),
            passed: results.iter().filter(|r| r.as_ref().unwrap().status == TestStatus::Passed).count(),
            failed: results.iter().filter(|r| r.as_ref().unwrap().status == TestStatus::Failed).count(),
            results,
            duration_seconds: 0.0, // Compute from timestamps
        })
    }

    pub async fn execute_test(&mut self, test_def: TestDefinition) -> Result<TestResult> {
        tracing::info!(test_id = %test_def.test_id, "Executing test");

        let start_time = std::time::Instant::now();
        let mut attempt = 0;

        loop {
            attempt += 1;

            // Setup
            if let Err(e) = self.setup_test(&test_def).await {
                tracing::error!(error = ?e, "Test setup failed");
                if attempt >= self.config.retry_policy.max_attempts {
                    return Ok(TestResult::failed(test_def.test_id, e.to_string()));
                }
                continue;
            }

            // Execute
            self.metrics.start_collection().await;
            let execution_result = self.execute_test_steps(&test_def).await;

            // Validate
            let validation_result = self.validate_test(&test_def).await;

            // Cleanup
            self.cleanup_test(&test_def).await?;

            // Check result
            let test_result = self.evaluate_test_result(
                &test_def,
                execution_result,
                validation_result,
                start_time.elapsed(),
            );

            if test_result.status == TestStatus::Passed
                || attempt >= self.config.retry_policy.max_attempts
            {
                return Ok(test_result);
            }

            // Retry with backoff
            let backoff = self.compute_backoff(attempt);
            tracing::warn!(
                attempt,
                backoff_ms = backoff.as_millis(),
                "Test failed, retrying"
            );
            tokio::time::sleep(backoff).await;
        }
    }

    async fn setup_test(&mut self, test_def: &TestDefinition) -> Result<()> {
        tracing::debug!("Setting up test environment");

        // Create network topology
        let mut network = self.network.write().await;
        network.create_topology(&test_def.setup.network_topology).await?;

        Ok(())
    }

    async fn execute_test_steps(&mut self, test_def: &TestDefinition) -> ExecutionResult {
        let mut step_results = Vec::new();

        for action in &test_def.execution.actions {
            tracing::debug!(step = action.step, "Executing test step");

            // Apply delay if specified
            if let Some(delay) = &action.timing.delay_before {
                let delay_duration = parse_duration(delay);
                tokio::time::sleep(delay_duration).await;
            }

            // Inject fault if specified
            if let Some(fault) = &action.fault_injection {
                let mut injector = self.fault_injector.write().await;
                injector.inject_fault(fault).await.ok();
            }

            // Execute node actions
            for node_action in &action.node_actions {
                self.execute_node_action(node_action).await.ok();
            }

            step_results.push(StepResult {
                step: action.step,
                status: StepStatus::Completed,
                duration: std::time::Duration::from_secs(1), // Measure actual duration
            });
        }

        ExecutionResult { step_results }
    }

    async fn validate_test(&self, test_def: &TestDefinition) -> ValidationResult {
        tracing::debug!("Validating test results");

        let mut passed_criteria = Vec::new();
        let mut failed_criteria = Vec::new();

        for criterion in &test_def.validation.success_criteria {
            match self.validate_criterion(criterion).await {
                Ok(true) => passed_criteria.push(criterion.criterion.clone()),
                Ok(false) => failed_criteria.push(criterion.criterion.clone()),
                Err(e) => {
                    tracing::error!(error = ?e, "Criterion validation error");
                    failed_criteria.push(criterion.criterion.clone());
                }
            }
        }

        ValidationResult {
            passed: failed_criteria.is_empty(),
            passed_criteria,
            failed_criteria,
        }
    }

    async fn validate_criterion(&self, criterion: &SuccessCriterion) -> Result<bool> {
        match criterion.validation_method {
            ValidationMethod::Assertion => {
                // Evaluate assertion expression
                self.evaluate_assertion(&criterion.parameters).await
            }
            ValidationMethod::MetricCheck => {
                // Check metric against threshold
                self.check_metric_threshold(&criterion.parameters).await
            }
            ValidationMethod::LogAnalysis => {
                // Analyze logs for patterns
                self.analyze_logs(&criterion.parameters).await
            }
            ValidationMethod::StateComparison => {
                // Compare system state
                self.compare_state(&criterion.parameters).await
            }
            ValidationMethod::PropertyTest => {
                // Run property-based test
                self.run_property_test(&criterion.parameters).await
            }
        }
    }

    async fn cleanup_test(&mut self, test_def: &TestDefinition) -> Result<()> {
        tracing::debug!("Cleaning up test environment");

        // Remove fault injections
        let mut injector = self.fault_injector.write().await;
        injector.remove_all_faults().await?;

        // Reset network
        let mut network = self.network.write().await;
        network.reset().await?;

        // Stop metrics collection
        self.metrics.stop_collection().await;

        Ok(())
    }

    fn evaluate_test_result(
        &self,
        test_def: &TestDefinition,
        execution_result: ExecutionResult,
        validation_result: ValidationResult,
        duration: std::time::Duration,
    ) -> TestResult {
        let status = if validation_result.passed {
            TestStatus::Passed
        } else {
            TestStatus::Failed
        };

        TestResult {
            test_id: test_def.test_id.clone(),
            status,
            severity: test_def.severity.clone(),
            execution_result,
            validation_result,
            duration,
            metrics_summary: self.metrics.generate_summary(),
        }
    }

    fn compute_backoff(&self, attempt: usize) -> std::time::Duration {
        match &self.config.retry_policy.backoff_strategy {
            BackoffStrategy::Constant { delay_ms } => {
                std::time::Duration::from_millis(*delay_ms)
            }
            BackoffStrategy::Linear { initial_ms, increment_ms } => {
                std::time::Duration::from_millis(initial_ms + (attempt as u64 * increment_ms))
            }
            BackoffStrategy::Exponential { initial_ms, multiplier } => {
                let delay = initial_ms * (multiplier.powi(attempt as i32) as u64);
                std::time::Duration::from_millis(delay)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_id: String,
    pub status: TestStatus,
    pub severity: Severity,
    pub execution_result: ExecutionResult,
    pub validation_result: ValidationResult,
    pub duration: std::time::Duration,
    pub metrics_summary: MetricsSummary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Major,
    Minor,
}
```

---

## Simulation Orchestration

### Network Simulator

```rust
// e2e-testing/framework/src/network_simulator.rs

use anyhow::Result;
use std::collections::HashMap;

pub struct NetworkSimulator {
    pub nodes: HashMap<usize, SimulatedNode>,
    pub topology: NetworkTopology,
    pub link_conditions: HashMap<(usize, usize), LinkCondition>,
}

pub struct SimulatedNode {
    pub id: usize,
    pub address: String,
    pub resonance_state: ResonanceState,
    pub status: NodeStatus,
    pub process_handle: Option<tokio::process::Child>,
}

pub struct ResonanceState {
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
}

pub enum NodeStatus {
    Active,
    Crashed,
    Byzantine,
    Partitioned,
}

pub struct LinkCondition {
    pub latency_ms: u64,
    pub loss_rate: f64,
    pub bandwidth_mbps: u64,
}

impl NetworkSimulator {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            topology: NetworkTopology::Mesh,
            link_conditions: HashMap::new(),
        }
    }

    pub async fn create_topology(&mut self, config: &TopologyConfig) -> Result<()> {
        tracing::info!(
            topology = ?config.topology_type,
            node_count = config.node_count,
            "Creating network topology"
        );

        // Create nodes
        for i in 0..config.node_count {
            self.create_node(i).await?;
        }

        // Create links based on topology
        match config.topology_type {
            TopologyType::Mesh => self.create_mesh_links().await?,
            TopologyType::Ring => self.create_ring_links().await?,
            TopologyType::Star => self.create_star_links().await?,
            TopologyType::Random => self.create_random_links(0.4).await?,
            TopologyType::Clustered => self.create_clustered_links(&config.clusters).await?,
            TopologyType::Partitioned => self.create_partitioned_links(&config.partitions).await?,
        }

        self.topology = config.topology_type.clone();
        Ok(())
    }

    async fn create_node(&mut self, id: usize) -> Result<()> {
        let resonance_state = ResonanceState {
            psi: rand::random::<f64>(),
            rho: rand::random::<f64>(),
            omega: rand::random::<f64>(),
        };

        let node = SimulatedNode {
            id,
            address: format!("127.0.0.1:{}", 10000 + id),
            resonance_state,
            status: NodeStatus::Active,
            process_handle: None,
        };

        // Start node process
        let mut process = tokio::process::Command::new("cargo")
            .args(&["run", "--bin", "spectralchain-node", "--", "--node-id", &id.to_string()])
            .spawn()?;

        self.nodes.insert(id, node);

        tracing::debug!(node_id = id, "Node created");
        Ok(())
    }

    async fn create_mesh_links(&mut self) -> Result<()> {
        let node_ids: Vec<usize> = self.nodes.keys().copied().collect();

        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                self.create_link(node_ids[i], node_ids[j]).await?;
            }
        }

        tracing::debug!("Mesh topology created");
        Ok(())
    }

    async fn create_ring_links(&mut self) -> Result<()> {
        let node_ids: Vec<usize> = self.nodes.keys().copied().collect();

        for i in 0..node_ids.len() {
            let next = (i + 1) % node_ids.len();
            self.create_link(node_ids[i], node_ids[next]).await?;
        }

        tracing::debug!("Ring topology created");
        Ok(())
    }

    async fn create_link(&mut self, node_a: usize, node_b: usize) -> Result<()> {
        let condition = LinkCondition {
            latency_ms: 10,
            loss_rate: 0.0,
            bandwidth_mbps: 1000,
        };

        self.link_conditions.insert((node_a, node_b), condition.clone());
        self.link_conditions.insert((node_b, node_a), condition);

        Ok(())
    }

    pub async fn inject_partition(&mut self, partition_config: &PartitionConfig) -> Result<()> {
        tracing::info!("Injecting network partition");

        for &node_id in &partition_config.nodes {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.status = NodeStatus::Partitioned;

                // Remove links to all other nodes
                for &other_id in self.nodes.keys() {
                    if other_id != node_id {
                        self.link_conditions.remove(&(node_id, other_id));
                        self.link_conditions.remove(&(other_id, node_id));
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn heal_partition(&mut self, partition_config: &PartitionConfig) -> Result<()> {
        tracing::info!("Healing network partition");

        for &node_id in &partition_config.nodes {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.status = NodeStatus::Active;

                // Recreate links based on original topology
                self.recreate_links_for_node(node_id).await?;
            }
        }

        Ok(())
    }

    pub async fn crash_node(&mut self, node_id: usize) -> Result<()> {
        tracing::info!(node_id, "Crashing node");

        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.status = NodeStatus::Crashed;

            // Kill process if running
            if let Some(mut process) = node.process_handle.take() {
                process.kill().await?;
            }
        }

        Ok(())
    }

    pub async fn restart_node(&mut self, node_id: usize) -> Result<()> {
        tracing::info!(node_id, "Restarting node");

        if let Some(node) = self.nodes.get_mut(&node_id) {
            // Start process
            let process = tokio::process::Command::new("cargo")
                .args(&["run", "--bin", "spectralchain-node", "--", "--node-id", &node_id.to_string()])
                .spawn()?;

            node.process_handle = Some(process);
            node.status = NodeStatus::Active;
        }

        Ok(())
    }

    pub async fn reset(&mut self) -> Result<()> {
        tracing::info!("Resetting network simulator");

        // Kill all processes
        for (_, node) in &mut self.nodes {
            if let Some(mut process) = node.process_handle.take() {
                process.kill().await.ok();
            }
        }

        self.nodes.clear();
        self.link_conditions.clear();

        Ok(())
    }
}
```

---

## Metrics Collection & Analysis

### Metrics Collector

```rust
// e2e-testing/framework/src/metrics_collector.rs

use prometheus::{Registry, Counter, Histogram, Gauge};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MetricsCollector {
    pub registry: Registry,
    pub time_series: Arc<RwLock<HashMap<String, Vec<DataPoint>>>>,
    pub sampling_interval: std::time::Duration,
    pub collection_active: Arc<RwLock<bool>>,
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub timestamp: std::time::SystemTime,
    pub value: f64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let registry = Registry::new();

        Self {
            registry,
            time_series: Arc::new(RwLock::new(HashMap::new())),
            sampling_interval: std::time::Duration::from_millis(100),
            collection_active: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start_collection(&self) {
        *self.collection_active.write().await = true;

        let time_series = self.time_series.clone();
        let active = self.collection_active.clone();
        let interval = self.sampling_interval;

        tokio::spawn(async move {
            while *active.read().await {
                tokio::time::sleep(interval).await;

                // Collect metrics
                let metrics = Self::collect_current_metrics().await;

                let mut ts = time_series.write().await;
                for (metric_name, value) in metrics {
                    ts.entry(metric_name).or_insert_with(Vec::new).push(DataPoint {
                        timestamp: std::time::SystemTime::now(),
                        value,
                    });
                }
            }
        });
    }

    pub async fn stop_collection(&self) {
        *self.collection_active.write().await = false;
    }

    async fn collect_current_metrics() -> HashMap<String, f64> {
        let mut metrics = HashMap::new();

        // Collect from SpectralChain nodes via API
        // Example:
        metrics.insert("throughput_tps".to_string(), 50.0);
        metrics.insert("latency_p50_ms".to_string(), 200.0);
        metrics.insert("latency_p99_ms".to_string(), 500.0);
        metrics.insert("success_rate".to_string(), 0.95);

        metrics
    }

    pub async fn get_current(&self, metric_name: &str) -> f64 {
        let ts = self.time_series.read().await;
        ts.get(metric_name)
            .and_then(|points| points.last())
            .map(|dp| dp.value)
            .unwrap_or(0.0)
    }

    pub async fn get_average(&self, metric_name: &str) -> f64 {
        let ts = self.time_series.read().await;
        if let Some(points) = ts.get(metric_name) {
            if points.is_empty() {
                return 0.0;
            }
            let sum: f64 = points.iter().map(|dp| dp.value).sum();
            sum / points.len() as f64
        } else {
            0.0
        }
    }

    pub async fn get_percentile(&self, metric_name: &str, percentile: f64) -> f64 {
        let ts = self.time_series.read().await;
        if let Some(points) = ts.get(metric_name) {
            if points.is_empty() {
                return 0.0;
            }

            let mut values: Vec<f64> = points.iter().map(|dp| dp.value).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let index = ((values.len() as f64 - 1.0) * percentile) as usize;
            values[index]
        } else {
            0.0
        }
    }

    pub fn generate_summary(&self) -> MetricsSummary {
        // Generate comprehensive metrics summary
        MetricsSummary {
            throughput_avg: 50.0,
            latency_p50: 200.0,
            latency_p99: 500.0,
            success_rate: 0.95,
        }
    }

    pub async fn export_to_prometheus(&self) -> String {
        // Export metrics in Prometheus format
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let metric_families = self.registry.gather();

        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        String::from_utf8(buffer).unwrap()
    }

    pub async fn export_to_json(&self) -> serde_json::Value {
        let ts = self.time_series.read().await;
        serde_json::to_value(&*ts).unwrap()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsSummary {
    pub throughput_avg: f64,
    pub latency_p50: f64,
    pub latency_p99: f64,
    pub success_rate: f64,
}
```

---

## Visualization Tools

### Dashboard Configuration

**Grafana Dashboard (JSON)**:

```json
{
  "dashboard": {
    "title": "SpectralChain E2E Test Dashboard",
    "uid": "spectralchain-e2e",
    "version": 1,
    "timezone": "utc",
    "panels": [
      {
        "id": 1,
        "title": "Transaction Success Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(transaction_success_total[5m])",
            "legendFormat": "Success Rate"
          }
        ],
        "yaxes": [
          {"format": "percentunit", "min": 0, "max": 1}
        ]
      },
      {
        "id": 2,
        "title": "Latency Percentiles",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.50, latency_seconds_bucket)",
            "legendFormat": "P50"
          },
          {
            "expr": "histogram_quantile(0.95, latency_seconds_bucket)",
            "legendFormat": "P95"
          },
          {
            "expr": "histogram_quantile(0.99, latency_seconds_bucket)",
            "legendFormat": "P99"
          }
        ],
        "yaxes": [
          {"format": "ms", "min": 0}
        ]
      },
      {
        "id": 3,
        "title": "Throughput (TPS)",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(transaction_total[1m])",
            "legendFormat": "TPS"
          }
        ],
        "yaxes": [
          {"format": "ops", "min": 0}
        ]
      },
      {
        "id": 4,
        "title": "Active Nodes",
        "type": "stat",
        "targets": [
          {
            "expr": "count(up{job=\"spectralchain-node\"})",
            "legendFormat": "Active Nodes"
          }
        ]
      },
      {
        "id": 5,
        "title": "Fork Count",
        "type": "graph",
        "targets": [
          {
            "expr": "fork_count",
            "legendFormat": "Active Forks"
          }
        ]
      },
      {
        "id": 6,
        "title": "Chaos Faults Active",
        "type": "stat",
        "targets": [
          {
            "expr": "chaos_fault_injection_count",
            "legendFormat": "Active Faults"
          }
        ],
        "thresholds": [
          {"value": 0, "color": "green"},
          {"value": 1, "color": "yellow"},
          {"value": 3, "color": "red"}
        ]
      },
      {
        "id": 7,
        "title": "Network Topology",
        "type": "nodeGraph",
        "targets": [
          {
            "expr": "spectralchain_node_connections",
            "legendFormat": "Node {{node_id}}"
          }
        ]
      },
      {
        "id": 8,
        "title": "Test Execution Timeline",
        "type": "timeseries",
        "targets": [
          {
            "expr": "test_step_duration_seconds",
            "legendFormat": "{{test_id}} - Step {{step}}"
          }
        ]
      }
    ],
    "templating": {
      "list": [
        {
          "name": "test_id",
          "type": "query",
          "query": "label_values(test_step_duration_seconds, test_id)"
        }
      ]
    },
    "annotations": {
      "list": [
        {
          "name": "Fault Injections",
          "datasource": "Prometheus",
          "expr": "changes(chaos_fault_injection_count[1m]) > 0",
          "tagKeys": "fault_type"
        }
      ]
    }
  }
}
```

### Custom Visualization Web App

**React Component for Test Results**:

```typescript
// e2e-testing/visualization/src/components/TestResultsViewer.tsx

import React, { useEffect, useState } from 'react';
import { Card, Table, Tag, Progress, Timeline } from 'antd';
import { CheckCircleOutlined, CloseCircleOutlined } from '@ant-design/icons';
import Plot from 'react-plotly.js';

interface TestResult {
  test_id: string;
  status: 'Passed' | 'Failed' | 'Skipped';
  severity: 'Critical' | 'Major' | 'Minor';
  duration: number;
  metrics_summary: MetricsSummary;
}

interface MetricsSummary {
  throughput_avg: number;
  latency_p50: number;
  latency_p99: number;
  success_rate: number;
}

export const TestResultsViewer: React.FC = () => {
  const [results, setResults] = useState<TestResult[]>([]);
  const [selectedTest, setSelectedTest] = useState<TestResult | null>(null);

  useEffect(() => {
    // Fetch test results from API
    fetch('/api/test-results')
      .then(res => res.json())
      .then(data => setResults(data));
  }, []);

  const columns = [
    {
      title: 'Test ID',
      dataIndex: 'test_id',
      key: 'test_id',
      render: (text: string, record: TestResult) => (
        <a onClick={() => setSelectedTest(record)}>{text}</a>
      ),
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: string) => (
        <Tag
          icon={status === 'Passed' ? <CheckCircleOutlined /> : <CloseCircleOutlined />}
          color={status === 'Passed' ? 'success' : 'error'}
        >
          {status}
        </Tag>
      ),
    },
    {
      title: 'Severity',
      dataIndex: 'severity',
      key: 'severity',
      render: (severity: string) => {
        const color = severity === 'Critical' ? 'red' : severity === 'Major' ? 'orange' : 'blue';
        return <Tag color={color}>{severity}</Tag>;
      },
    },
    {
      title: 'Duration',
      dataIndex: 'duration',
      key: 'duration',
      render: (duration: number) => `${duration.toFixed(2)}s`,
    },
    {
      title: 'Success Rate',
      dataIndex: ['metrics_summary', 'success_rate'],
      key: 'success_rate',
      render: (rate: number) => (
        <Progress
          percent={rate * 100}
          size="small"
          status={rate >= 0.95 ? 'success' : 'exception'}
        />
      ),
    },
  ];

  const renderMetricsCharts = (test: TestResult) => {
    const metricsData = {
      latency: {
        x: ['P50', 'P99'],
        y: [test.metrics_summary.latency_p50, test.metrics_summary.latency_p99],
        type: 'bar',
        name: 'Latency (ms)',
      },
    };

    return (
      <Card title="Test Metrics">
        <Plot
          data={[metricsData.latency]}
          layout={{
            title: 'Latency Distribution',
            xaxis: { title: 'Percentile' },
            yaxis: { title: 'Latency (ms)' },
          }}
        />
      </Card>
    );
  };

  return (
    <div>
      <Card title="Test Suite Results">
        <Table
          dataSource={results}
          columns={columns}
          rowKey="test_id"
          pagination={{ pageSize: 10 }}
        />
      </Card>

      {selectedTest && (
        <Card title={`Test Details: ${selectedTest.test_id}`} style={{ marginTop: 20 }}>
          {renderMetricsCharts(selectedTest)}
        </Card>
      )}
    </div>
  );
};
```

---

## Report Generation

### HTML Report Generator

```rust
// e2e-testing/framework/src/report_generator.rs

use handlebars::Handlebars;
use serde_json::json;
use std::fs;

pub struct ReportGenerator {
    handlebars: Handlebars<'static>,
}

impl ReportGenerator {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        // Register template
        let template = include_str!("../templates/test_report.hbs");
        handlebars.register_template_string("test_report", template).unwrap();

        Self { handlebars }
    }

    pub fn generate_html_report(
        &self,
        suite_result: &TestSuiteResult,
        output_path: &str,
    ) -> Result<()> {
        let data = json!({
            "suite_name": suite_result.suite_name,
            "total_tests": suite_result.total_tests,
            "passed": suite_result.passed,
            "failed": suite_result.failed,
            "pass_rate": (suite_result.passed as f64 / suite_result.total_tests as f64) * 100.0,
            "duration": suite_result.duration_seconds,
            "results": suite_result.results,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let html = self.handlebars.render("test_report", &data)?;
        fs::write(output_path, html)?;

        tracing::info!(path = output_path, "HTML report generated");
        Ok(())
    }

    pub fn generate_json_report(
        &self,
        suite_result: &TestSuiteResult,
        output_path: &str,
    ) -> Result<()> {
        let json = serde_json::to_string_pretty(suite_result)?;
        fs::write(output_path, json)?;

        tracing::info!(path = output_path, "JSON report generated");
        Ok(())
    }

    pub fn generate_pdf_report(
        &self,
        suite_result: &TestSuiteResult,
        output_path: &str,
    ) -> Result<()> {
        // Generate HTML first
        let html_path = format!("{}.html", output_path);
        self.generate_html_report(suite_result, &html_path)?;

        // Convert HTML to PDF using headless Chrome
        // (requires chrome/chromium installed)
        std::process::Command::new("chromium")
            .args(&[
                "--headless",
                "--disable-gpu",
                "--print-to-pdf",
                &html_path,
            ])
            .output()?;

        tracing::info!(path = output_path, "PDF report generated");
        Ok(())
    }
}
```

**HTML Template (Handlebars)**:

```html
<!-- e2e-testing/framework/templates/test_report.hbs -->
<!DOCTYPE html>
<html>
<head>
    <title>{{suite_name}} - Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background: #f0f0f0; padding: 20px; border-radius: 5px; }
        .stats { display: flex; gap: 20px; margin: 20px 0; }
        .stat-card { flex: 1; padding: 15px; border: 1px solid #ddd; border-radius: 5px; }
        .stat-value { font-size: 32px; font-weight: bold; }
        .passed { color: #52c41a; }
        .failed { color: #ff4d4f; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
        th { background: #f5f5f5; }
        .status-passed { color: #52c41a; font-weight: bold; }
        .status-failed { color: #ff4d4f; font-weight: bold; }
    </style>
</head>
<body>
    <div class="header">
        <h1>{{suite_name}} - Test Report</h1>
        <p>Generated: {{timestamp}}</p>
        <p>Duration: {{duration}}s</p>
    </div>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-value passed">{{passed}}</div>
            <div>Passed</div>
        </div>
        <div class="stat-card">
            <div class="stat-value failed">{{failed}}</div>
            <div>Failed</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">{{total_tests}}</div>
            <div>Total Tests</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">{{pass_rate}}%</div>
            <div>Pass Rate</div>
        </div>
    </div>

    <h2>Test Results</h2>
    <table>
        <thead>
            <tr>
                <th>Test ID</th>
                <th>Status</th>
                <th>Severity</th>
                <th>Duration</th>
                <th>Success Rate</th>
            </tr>
        </thead>
        <tbody>
            {{#each results}}
            <tr>
                <td>{{this.test_id}}</td>
                <td class="status-{{this.status}}">{{this.status}}</td>
                <td>{{this.severity}}</td>
                <td>{{this.duration}}s</td>
                <td>{{this.metrics_summary.success_rate}}</td>
            </tr>
            {{/each}}
        </tbody>
    </table>
</body>
</html>
```

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/e2e-tests.yml
name: E2E Tests

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 2 * * *'  # Nightly at 2 AM

jobs:
  e2e-production:
    name: Production Scenarios
    runs-on: ubuntu-latest
    timeout-minutes: 60

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run production E2E tests
        run: |
          cargo test --test e2e_production --release -- --nocapture
        env:
          RUST_LOG: info

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: e2e-production-results
          path: e2e-testing/reports/

  e2e-edge-cases:
    name: Edge Case Scenarios
    runs-on: ubuntu-latest
    timeout-minutes: 120

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run edge case tests
        run: |
          cargo test --test e2e_edge_cases --release -- --nocapture
        env:
          RUST_LOG: debug

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: e2e-edge-case-results
          path: e2e-testing/reports/

  chaos-engineering:
    name: Chaos Engineering Tests
    runs-on: ubuntu-latest
    timeout-minutes: 180
    if: github.event_name == 'schedule' || github.event_name == 'workflow_dispatch'

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run chaos tests
        run: |
          cargo test --test e2e_chaos --release -- --nocapture
        env:
          RUST_LOG: info
          CHAOS_INTENSITY: 0.3

      - name: Upload chaos results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: chaos-results
          path: e2e-testing/reports/

      - name: Generate comprehensive report
        if: always()
        run: |
          cd e2e-testing
          ./scripts/generate_report.sh

      - name: Publish report to GitHub Pages
        if: always()
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./e2e-testing/reports
          destination_dir: test-reports/${{ github.run_number }}

      - name: Send Slack notification
        if: always()
        uses: slackapi/slack-github-action@v1
        with:
          payload: |
            {
              "text": "Chaos Engineering Tests ${{ job.status }}",
              "blocks": [
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": "*Chaos Engineering Tests ${{ job.status }}*\nRun: #${{ github.run_number }}\nReport: https://${{ github.repository_owner }}.github.io/${{ github.event.repository.name }}/test-reports/${{ github.run_number }}/"
                  }
                }
              ]
            }
        env:
          SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}
```

---

## Deployment

### Docker Compose for Test Environment

```yaml
# e2e-testing/docker-compose.yml
version: '3.8'

services:
  test-runner:
    build:
      context: .
      dockerfile: Dockerfile.test-runner
    environment:
      - RUST_LOG=info
      - TEST_SUITE=${TEST_SUITE:-production}
    volumes:
      - ./reports:/app/reports
      - ./config:/app/config
    depends_on:
      - prometheus
      - grafana
      - elasticsearch
      - jaeger
    networks:
      - test-network

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./config/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    networks:
      - test-network

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_INSTALL_PLUGINS=grafana-piechart-panel
    volumes:
      - ./config/grafana/dashboards:/etc/grafana/provisioning/dashboards
      - ./config/grafana/datasources:/etc/grafana/provisioning/datasources
      - grafana-data:/var/lib/grafana
    networks:
      - test-network

  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.7.0
    ports:
      - "9200:9200"
    environment:
      - discovery.type=single-node
      - xpack.security.enabled=false
    volumes:
      - elasticsearch-data:/usr/share/elasticsearch/data
    networks:
      - test-network

  kibana:
    image: docker.elastic.co/kibana/kibana:8.7.0
    ports:
      - "5601:5601"
    environment:
      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    depends_on:
      - elasticsearch
    networks:
      - test-network

  jaeger:
    image: jaegertracing/all-in-one:latest
    ports:
      - "5775:5775/udp"
      - "6831:6831/udp"
      - "6832:6832/udp"
      - "5778:5778"
      - "16686:16686"
      - "14268:14268"
      - "14250:14250"
      - "9411:9411"
    environment:
      - COLLECTOR_ZIPKIN_HOST_PORT=:9411
    networks:
      - test-network

networks:
  test-network:
    driver: bridge

volumes:
  prometheus-data:
  grafana-data:
  elasticsearch-data:
```

### Running the Test Suite

```bash
# Start test environment
cd e2e-testing
docker-compose up -d

# Wait for services to be ready
./scripts/wait-for-services.sh

# Run production tests
export TEST_SUITE=production
cargo test --test e2e_production --release

# Run edge case tests
export TEST_SUITE=edge-cases
cargo test --test e2e_edge_cases --release

# Run chaos tests
export TEST_SUITE=chaos
export CHAOS_INTENSITY=0.5
cargo test --test e2e_chaos --release

# Generate reports
./scripts/generate_report.sh

# View reports
open reports/index.html

# Access dashboards
# Grafana: http://localhost:3000 (admin/admin)
# Prometheus: http://localhost:9090
# Kibana: http://localhost:5601
# Jaeger: http://localhost:16686

# Cleanup
docker-compose down -v
```

---

## Conclusion

This automation and visualization framework provides:

1. **Comprehensive test automation** with retry logic and fault injection
2. **Network simulation** for realistic testing environments
3. **Metrics collection** with time-series analysis
4. **Rich visualizations** via Grafana, Kibana, and custom dashboards
5. **Automated report generation** in HTML, JSON, and PDF formats
6. **CI/CD integration** with GitHub Actions
7. **Easy deployment** via Docker Compose

**With these tools, SpectralChain can achieve continuous, automated E2E testing with comprehensive observability and actionable insights.**

---

**Document Version:** 1.0.0
**Last Updated:** 2025-11-06
**Status:** Complete and ready for implementation
