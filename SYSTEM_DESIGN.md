# System Design Questions for Tech Company Interviews

This document covers common system design questions asked at major tech companies, with focus on monitoring, observability, and distributed systems.

## Monitoring & Observability Systems

### 1. Design a Metrics Collection System

**Problem**: Design a distributed system to collect, aggregate, and query metrics from thousands of servers.

**Key Components**:
- Metric agents on each server
- Time-series database (e.g., InfluxDB, Prometheus)
- Aggregation layer for roll-ups
- Query API for dashboards
- Alerting engine

**Scaling Considerations**:
- Handle 1M+ metrics per second
- Data retention policies (high resolution for recent data, downsampled for historical)
- Geographic distribution
- Fault tolerance and redundancy

**Follow-up Questions**:
- How do you handle metric cardinality explosion?
- What's your data retention strategy?
- How do you implement efficient querying across time ranges?

### 2. Design a Log Aggregation System

**Problem**: Build a system to collect, process, and search logs from distributed applications.

**Architecture**:
- Log collection agents (Fluentd, Filebeat)
- Message queue for buffering (Kafka)
- Stream processing for parsing/enrichment
- Search index (Elasticsearch)
- Web interface for querying

**Challenges**:
- Handle log spikes during incidents
- Real-time vs batch processing trade-offs
- Cost optimization for storage
- Schema evolution for different log formats

### 3. Design an Alerting System

**Problem**: Create a system that monitors metrics and sends alerts when thresholds are breached.

**Components**:
- Rule engine for alert conditions
- Notification channels (email, SMS, Slack, PagerDuty)
- Alert correlation and deduplication
- Escalation policies
- Silence/acknowledgment functionality

**Advanced Features**:
- Anomaly detection using ML
- Alert fatigue prevention
- Root cause analysis suggestions
- Incident response automation

## Distributed Systems Design

### 4. Design a Rate Limiting Service

**Problem**: Build a distributed rate limiter that can handle millions of requests per second.

**Algorithms**:
- Token bucket
- Sliding window log
- Fixed window counter
- Sliding window counter

**Implementation Considerations**:
- Local vs distributed state
- Redis-based coordination
- Consistent hashing for partitioning
- Rate limit policies (per user, per API key, global)

**Trade-offs**:
- Accuracy vs performance
- Memory usage vs precision
- Network latency impact

### 5. Design a Configuration Management System

**Problem**: Create a system to manage configuration for thousands of microservices.

**Features**:
- Version control for configurations
- Environment-specific overrides
- Real-time configuration updates
- Rollback capabilities
- Access control and audit logging

**Architecture**:
- Configuration store (etcd, Consul)
- Change propagation mechanism
- Client libraries for config polling/streaming
- UI for configuration management

### 6. Design a Service Discovery System

**Problem**: Build a system for services to find and communicate with each other in a dynamic environment.

**Components**:
- Service registry
- Health checking
- Load balancing integration
- DNS-based discovery
- Circuit breaker integration

**Challenges**:
- Handling service failures
- Split-brain scenarios
- Performance at scale
- Cross-datacenter service discovery

## Data Storage & Processing

### 7. Design a Time-Series Database

**Problem**: Build a database optimized for time-series data (metrics, events, IoT data).

**Characteristics**:
- High write throughput
- Time-based partitioning
- Compression for storage efficiency
- Fast range queries
- Retention policies

**Architecture Decisions**:
- LSM trees vs B-trees
- Column-oriented storage
- Sharding strategies
- Replication and consistency

### 8. Design a Real-Time Analytics System

**Problem**: Process streaming data to provide real-time insights and dashboards.

**Stream Processing**:
- Apache Kafka for data ingestion
- Stream processors (Kafka Streams, Apache Flink)
- Real-time aggregations
- Event time vs processing time handling

**Storage Layer**:
- OLAP databases (ClickHouse, Apache Druid)
- Pre-computed aggregations
- Data freshness vs query performance trade-offs

## Infrastructure & DevOps

### 9. Design a Container Orchestration System

**Problem**: Build a system to manage containerized applications across a cluster.

**Features**:
- Container scheduling and placement
- Service networking and load balancing
- Resource allocation and limits
- Health monitoring and self-healing
- Rolling deployments

**Challenges**:
- Resource optimization
- Multi-tenancy and isolation
- Storage orchestration
- Security and compliance

### 10. Design a CI/CD Pipeline

**Problem**: Create an automated system for building, testing, and deploying code.

**Stages**:
- Source code management integration
- Build automation
- Test execution (unit, integration, e2e)
- Artifact management
- Deployment automation
- Monitoring and rollback

**Considerations**:
- Pipeline parallelization
- Secret management
- Environment promotion
- Compliance and audit trails

## Common Interview Themes

### Scalability Patterns
- Horizontal vs vertical scaling
- Database sharding strategies
- Caching layers (Redis, CDNs)
- Load balancing algorithms
- Microservices vs monolith trade-offs

### Reliability Patterns
- Circuit breaker pattern
- Bulkhead isolation
- Retry mechanisms with exponential backoff
- Graceful degradation
- Disaster recovery strategies

### Consistency Models
- Strong consistency vs eventual consistency
- CAP theorem implications
- Consensus algorithms (Raft, PBFT)
- Conflict resolution strategies

### Security Considerations
- Authentication and authorization
- Network security and encryption
- Secret management
- Audit logging and compliance
- DDoS protection strategies

## Tips for System Design Interviews

1. **Start with requirements**: Clarify functional and non-functional requirements
2. **Estimate scale**: Calculate QPS, storage needs, bandwidth requirements
3. **High-level design first**: Draw the main components before diving into details
4. **Address bottlenecks**: Identify and solve potential scaling issues
5. **Consider trade-offs**: Discuss pros/cons of different approaches
6. **Think about operations**: How to deploy, monitor, and maintain the system
7. **Security and compliance**: Don't forget about authentication, encryption, and privacy

## Real-World Examples

### Monitoring Companies
- **Datadog**: Multi-tenant SaaS monitoring platform
- **Prometheus**: Open-source metrics and alerting
- **Grafana**: Visualization and dashboarding
- **New Relic**: Application performance monitoring

### Key Technologies
- **Time-series DBs**: InfluxDB, TimescaleDB, Prometheus TSDB
- **Message Queues**: Apache Kafka, RabbitMQ, AWS SQS
- **Stream Processing**: Apache Flink, Kafka Streams, Apache Storm
- **Search**: Elasticsearch, Apache Solr
- **Coordination**: Apache Zookeeper, etcd, Consul

This document provides a foundation for system design discussions. Practice drawing architectures, calculating capacity requirements, and discussing trade-offs for these common scenarios.