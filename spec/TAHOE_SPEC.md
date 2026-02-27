# Tahoe Language Specification

## Purpose
Tahoe is used for application development and secure server management.

## Execution Model
- Compiled language
- Reverse-order code execution (bottom → top)

## Security
- Role-based access control (RBAC)
- Signed execution plans
- No arbitrary command execution

## Scope
Tahoe cannot access kernel, bootloader, or raw memory.
