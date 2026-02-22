# Synora -- Technology Stack

# Synora -- 技术选型说明

Generated on: 2026-02-21 23:15:09

------------------------------------------------------------------------

## Core Language / 核心语言

Rust

## CLI Framework / 命令行框架

clap

## Logging & Errors / 日志与错误

tracing + thiserror + anyhow

## Database / 数据库

SQLite (`rusqlite` current baseline, `sqlx` optional future migration)

## Windows Integration / Windows 集成

windows-rs + winreg

## Sources / 软件来源

winget + GitHub API (reqwest)

## Async Runtime / 异步运行时

tokio

## Security Model / 安全模型

Security Guard layer with whitelist + registry backup + quarantine

## Dev Environment / 开发环境

Code on Windows, AI tooling on WSL

------------------------------------------------------------------------

This document defines the official technical baseline for Synora.
本文件定义 Synora 的官方技术基线。
