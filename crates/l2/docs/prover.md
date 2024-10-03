# Ethereum Rust L2 Prover

## ToC

- [Prover](#prover)
- [Workflow](#workflow)

## Prover

The SP1 Prover currently runs a demo fibonacci program. It uses Groth16, which requires at least 128GB of RAM. To mock proof generation and avoid RAM requirements, you can use the `SP1_PROVER=mock` env var.

## Workflow

The ProofDataClient is in charge of request for new jobs to the ProofDataProvider when the prover is free. When a new job arrives, the Prover will generate the proof and then the ProofDataClient will submit it to the ProofDataProvider

```mermaid
sequenceDiagram
    participant Prover
    participant ProofDataClient
    participant ProofDataProvider
    ProofDataClient->>+ProofDataProvider: ProofData::Request
    ProofDataProvider-->>-ProofDataClient: ProofData::Response(id)
    ProofDataClient->>+Prover: Prove(id)
    Prover-->>-ProofDataClient: Proof
    ProofDataClient->>+ProofDataProvider: ProofData::Submit(id)
    ProofDataProvider-->>-ProofDataClient: ProofData::SubmitAck(id)
```