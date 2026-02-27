# Tahoe Programming Language — v0.1

## Purpose
Tahoe is a compiled programming language used for:
- Application development
- IDE-driven UI workflows
- Secure server and infrastructure management

## Relationship
- Sierra → system & OS language
- Tahoe → application & server language

## Execution Model
- Compiled language
- Reverse-order execution (bottom → top)
- Deterministic output

## Security Model
- Role-Based Access Control (RBAC)
- Signed execution plans
- No arbitrary command execution
- No kernel or boot access

## Scope Limits
Tahoe cannot:
- Access raw memory
- Control bootloaders
- Execute shell commands directly