itle: "Ferriki — GitHub Copilot 인계 문서"
author: "Lessernalsa (레서날사)"
owners: ["@lessernalsa"]
version: "0.1"
status: "draft"
last_updated: "2025-08-18"
repo: "github.com//ferriki"
license: "Apache-2.0"
timezone: "Asia/Seoul"
tags: ["Rust", "Axum", "SQLx", "Wiki", "Copilot", "Axum", "Tokio", "OpenTelemetry"]

이 문서는 GitHub Copilot/Chat과 함께 Ferriki(러스트 기반 위키 엔진) 개발을 빠르게 시작하기 위한 핸드오프 문서입니다. 아래 체크리스트와 프롬프트를 그대로 사용하면 초기 스캐폴딩부터 기능 개발, 테스트, 배포까지 일관된 기준으로 진행할 수 있습니다.

1) 프로젝트 개요

목표: 빠르고 안전한 Rust 기반 위키 엔진(Ferriki)의 MVP 구현 → 페이지 작성/조회/수정/이력 관리, 사용자 인증, 관측/배포 자동화.

비전: 낮은 자원 점유, 높은 동시성, 온프레미스/클라우드/엣지에서 모두 가볍게 동작.

핵심 가치: 메모리 안전성·성능, 단순한 아키텍처, 관측 가능성, 테스트 용이성.

1.1 범위(MVP)

사용자: 가입/로그인/토큰 갱신(JWT), 자기 프로필 조회.

페이지: 생성/조회/수정/삭제, 리비전 저장(수정 이력), 간단한 태깅.

검색(초기): 제목/태그 기반 단순 LIKE 검색 → 이후 Full-text(트리그람/Meilisearch)로 확장.

관측: 구조화 로그(tracing), OpenTelemetry(Trace/Metric) → Grafana/Tempo/Loki 파이프.

배포: Docker 이미지, docker-compose 로컬, GitHub Actions CI.

1.2 비범위(차후)

첨부파일/이미지 업로드, 권한 롤(팀/조직), WYSIWYG 편집기, 실시간 협업, GraphQL, SSO, 다국어 i18n.

2) 기술 스택 & 기준

언어/런타임: Rust 1.79+ / Tokio

웹 프레임워크: Axum (+ Tower)

DB & ORM: PostgreSQL 16, SQLx(compile-time 쿼리 체크), 마이그레이션 sqlx migrate

구성: dotenvy 기반 env, 12-factor 원칙

에러/로그: thiserror, tracing, tracing-subscriber

관측: OpenTelemetry(OTLP exporter)

테스트: tokio::test, insta(스냅샷 선택), Testcontainers로 통합 테스트

품질: rustfmt, clippy -D warnings

커밋 규약: Conventional Commits (feat:, fix:, chore:…)

3) 리포지토리 레이아웃 (제안)

ferriki/
├─ Cargo.toml
├─ .editorconfig
├─ .gitignore
├─ .env.example
├─ Dockerfile
├─ docker-compose.yaml
├─ migrations/
│  └─ 2025xxxxxx_init.sql
├─ scripts/
│  ├─ dev-up.sh
│  ├─ lint.sh
│  └─ bench.sh
├─ .github/
│  ├─ ISSUE_TEMPLATE/bug_report.md
│  ├─ ISSUE_TEMPLATE/feature_request.md
│  └─ workflows/ci.yml
└─ src/
├─ main.rs
├─ config.rs
├─ telemetry.rs
├─ db.rs
├─ error.rs
├─ domain/
│  ├─ users.rs
│  └─ pages.rs
├─ routes/
│  ├─ mod.rs
│  ├─ users.rs
│  └─ pages.rs
└─ security/
├─ auth.rs
└─ jwt.rs

4) 환경 변수 표준(.env)

APP_HOST=0.0.0.0
APP_PORT=3000
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ferriki
RUST_LOG=info,tower_http=info,sqlx=warn
# OpenTelemetry (선택)
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
OTEL_SERVICE_NAME=ferriki

5) 초기 스캐폴딩(참고 코드 스니펫)

Copilot에게 아래 프롬프트로 파일을 생성시키세요.

Copilot Chat Prompt

Create an Axum app in Rust with modules: config, telemetry, db, error, routes.
Add /healthz and /readyz; use SQLx(Postgres) and tracing. Provide main.rs and minimal scaffolding.

main.rs(요지)

#[tokio::main]
async fn main() -> anyhow::Result<()> {
telemetry::init();
let cfg = config::Config::from_env()?;
let pool = db::connect(&cfg.database_url).await?;
let state = AppState { pool };
let app = routes::router(state.clone());
let listener = TcpListener::bind(cfg.addr()).await?;
axum::serve(listener, app).await?;
Ok(())
}

6) 데이터 모델 & API 스펙(MVP)

6.1 Users

fields: id(uuid), email(text, unique), password_hash(text), created_at(timestamptz)

endpoints

POST /api/v1/auth/register → {email, password}

POST /api/v1/auth/login → {email, password} → {access_token, refresh_token}

POST /api/v1/auth/refresh → {refresh_token}

GET /api/v1/me (Bearer) → 현재 사용자 정보

6.2 Pages

fields: id(uuid), title(text, index), slug(text, unique), body(text), tags(text[]), rev(int), updated_at, created_at

revisions: page_revisions(page_id, rev, body, updated_at, editor_id)

endpoints

POST /api/v1/pages

GET /api/v1/pages?query=&tag=&page=&limit=

GET /api/v1/pages/{slug}

PUT /api/v1/pages/{slug} (rev 충돌 시 409)

DELETE /api/v1/pages/{slug}

GET /api/v1/pages/{slug}/revisions

6.3 오류 모델

{ "error": "message", "code": "optional_code" }

7) 보안/인증

비밀번호: Argon2id(argon2 crate), 고정된 파라미터를 설정 파일화.

토큰: JWT(HS256/RS256). Access(15m) / Refresh(7d).

미들웨어: 요청 컨텍스트에 UserId 주입, 라우트 가드.

Copilot Prompt(보안)

Add auth: register/login with Argon2 password hashing and JWT tokens (access/refresh).
Implement a middleware that extracts Bearer token and sets UserId in request extensions.

8) 테스트 전략

유닛: 도메인 로직(pure functions) 집중.

통합: testcontainers로 Postgres 기동 후 실제 쿼리/라우트 테스트.

스냅샷: API 응답 JSON은 insta 스냅샷으로 회귀 방지.

품질 게이트: CI에서 clippy -D warnings + cargo test --all.

Copilot Prompt(테스트)

Write integration tests for /api/v1/pages using testcontainers (postgres).
Cover CRUD and revision behaviors; assert status codes and JSON schema.

9) 관측/로깅

tracing으로 구조화 로그, 요청 단위 request_id 필드 포함.

OpenTelemetry OTLP Exporter를 통해 Trace/Metric 전송(환경 변수로 On/Off).

에러 레벨 기준: warn(recoverable), error(user-visible or data corruption risk).

Copilot Prompt(관측)

Add tracing spans for each handler and DB query; propagate request_id through extensions.
Add optional OpenTelemetry exporter controlled by env variables.

10) 빌드/배포

도커: 멀티스테이지 빌드(정적 바이너리 선호), distroless 대체 가능.

컴포즈: 앱+Postgres 로컬 실행.

런타임 튜닝: RUST_LOG, worker threads(TOKIO_WORKER_THREADS 선택), ulimit 메모.

GitHub Actions(ci.yml 초안)

name: ci
on: [push, pull_request]
jobs:
build:
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- run: rustup component add clippy rustfmt
- run: cargo fmt --all -- --check
- run: cargo clippy --all-targets --all-features -- -D warnings
- run: cargo test --all --locked

11) 브랜치/릴리스 전략

main: 항상 배포 가능한 상태.

develop(선택): 기능 통합용.

feature/*: 이슈 기반 단위 브랜치.

태깅: SemVer(v0.1.0).

PR 체크리스트: 테스트 통과, 변경 로그(CHANGELOG), API 변경 시 문서 업데이트.

12) 커밋/이슈 템플릿

Conventional Commits 예시

feat(auth): add refresh token rotation

fix(pages): resolve stale revision conflict

이슈 템플릿: 버그/기능/문서(간단 양식 포함).

bug_report.md

## 버그 설명
## 재현 절차
## 기대 동작
## 스크린샷/로그
## 환경정보

13) SLO/성능 기준(초기)

레이지 요청 p95: 15ms(캐시 히트 시), 80ms(캐시 미스 시)

초기 QPS(개발 환경): 500rps 목표(로컬/도커, 단순 핸들러 기준)

자원 가이드: 컨테이너 RSS < 120MB(MVP), CPU 0.2~0.5 core 목표

부하 테스트: hey 또는 vegeta 스크립트 제공

14) 보안/규정 주의

비밀번호, 토큰, DB 시크릿은 .env.local / GitHub Secrets로만 관리.

PII(이메일 등) 로깅 금지; 필요 시 마스킹.

의존성 감사: cargo audit 주기 실행.

15) 작업 백로그(우선순위)

스캐폴딩 & 헬스체크(완료 기준: /healthz, /readyz 동작, DB 연결 확인)

Users 도메인 + 인증(JWT) — register/login/refresh + 미들웨어

Pages 도메인 + 리비전 — CRUD, 버전 충돌(409)

검색(제목/태그) — 간단 쿼리 파라미터

통합 테스트(Testcontainers) — Users/Pages 경로 커버

관측(OTel) — Trace/Metric export 토글

CI 파이프라인 — fmt/clippy/test + Docker build

각 항목은 feature/* 브랜치로 진행, PR 시 linked issue와 체크리스트 포함.

16) Copilot 대화 프롬프트 모음(바로 사용)

라우팅 생성: Add routes for /api/v1/pages with handlers and JSON schemas. Use axum extractors and return proper status codes.

SQLx 쿼리: Write SQLx queries with compile-time macros for inserting and selecting pages by slug. Handle unique violation with 409.

JWT 미들웨어: Implement a tower layer that validates Bearer token and sets UserId in request extensions.

통합 테스트: Using testcontainers, spin up Postgres and run integration tests for pages CRUD and revision history.

관측: Instrument handlers and DB calls with tracing spans and add request_id propagation.

17) Definition of Done (DoD)

문서화된 API(엔드포인트/샘플 응답), 단위·통합 테스트 통과(>80% 핵심 경로), 로그/트레이스에서 요청 상관관계 확인 가능, Docker 이미지 빌드 OK, 기본 부하 테스트 기준 충족.

18) 빠른 시작(개발자)

cp .env.example .env
cargo install sqlx-cli --no-default-features --features rustls,postgres
sqlx database create && sqlx migrate run
cargo run  # http://localhost:3000/healthz

문의/결정 필요 사항은 GitHub Discussions 또는 Issues로 남겨 주세요. 이 문서는 릴리스에 따라 업데이트됩니다.