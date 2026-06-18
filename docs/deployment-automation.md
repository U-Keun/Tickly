# Deployment Automation Guide

> Legacy note: this guide describes the pre-v2 release automation, including Supabase secrets and older cloud-sync assumptions. Tickly is now a v2-only local-first runtime; refresh this guide before using it for a production release.

Tickly 배포 자동화는 GitHub Actions의 CI/CD 워크플로우로 구성됩니다.

## Workflows

- CI: `.github/workflows/ci.yml`
  - 실행 시점: `main` 브랜치 push, `main` 대상 PR, 수동 실행
  - 수행 작업:
    - `package.json`과 `src-tauri/tauri.conf.json` 버전 동기화 검증
    - 프론트엔드 타입체크 (`yarn run check`)
    - Rust 체크/테스트 (`cargo check`, `cargo test`)
    - `main` push 시 macOS 번들 빌드 스모크 체크

- CD: `.github/workflows/cd.yml`
  - 실행 시점: `v*.*.*` 태그 push, 수동 실행
  - 수행 작업:
    - 릴리즈 버전/태그 검증
    - 시크릿 사전 검증 (Desktop/iOS 분리)
    - Desktop( macOS / Windows / Linux ) 아티팩트 빌드
    - iOS IPA 빌드 + TestFlight 업로드
    - iOS 위젯 템플릿 동기화 (`yarn ios:widget:setup`)
    - 태그 기준 GitHub Draft Release 생성

## Required GitHub Secrets

### Common

- `SUPABASE_URL`
- `SUPABASE_ANON_KEY`

### iOS / TestFlight

- `IOS_CERTIFICATE_P12` (base64)
- `IOS_CERTIFICATE_PASSWORD`
- `IOS_PROVISIONING_PROFILE` (base64, `com.u-keunsong.tickly`)
- `IOS_WIDGET_PROVISIONING_PROFILE` (base64, `com.u-keunsong.tickly.widget`)
- `APP_STORE_CONNECT_API_ISSUER_ID`
- `APP_STORE_CONNECT_API_KEY_ID`
- `APP_STORE_CONNECT_API_KEY_CONTENT`

두 provisioning profile 모두 아래 조건이 필요합니다.
- Distribution(App Store) 프로파일 (`get-task-allow = false`)
- `group.com.u-keunsong.tickly` App Group entitlement 포함

## Standard Release Flow

1. 앱 버전 동기화
   - `package.json`의 `version`
   - `src-tauri/tauri.conf.json`의 `version`
   - `src-tauri/Cargo.toml`의 `version`
   - `src-tauri/ios-widget/project.yml`의 `CFBundleShortVersionString` / `CFBundleVersion`
2. 로컬 검증
   - `yarn run check`
   - `cargo test --manifest-path src-tauri/Cargo.toml`
3. 태그 생성/푸시
   - `git tag vX.Y.Z`
   - `git push origin vX.Y.Z`
4. GitHub Actions CD 실행 확인
   - Desktop 아티팩트 업로드 완료
   - iOS TestFlight 업로드 완료
5. Draft Release 검토 후 Publish

## Manual Run (workflow_dispatch)

CD 워크플로우는 수동 실행 시 아래 옵션을 제공합니다.

- `build_ios`: iOS 빌드/TestFlight 업로드 실행 여부
- `build_desktop`: 데스크톱 빌드 실행 여부
- `create_release`: 태그 기준 Draft Release 생성 여부

주의: Release 생성은 태그(`refs/tags/v*.*.*`) 기준에서만 동작합니다.

## Failure Checklist

- 버전 불일치
  - 태그와 앱 버전이 일치하는지 확인
- 시크릿 누락
  - Preflight job 로그에서 누락된 키 이름 확인
- iOS 서명 실패
  - 인증서/프로비저닝 프로파일 base64 인코딩 값 재확인
  - `IOS_PROVISIONING_PROFILE`(앱)과 `IOS_WIDGET_PROVISIONING_PROFILE`(위젯) 둘 다 등록되어 있는지 확인
  - 두 프로파일 모두 `group.com.u-keunsong.tickly` App Group entitlement 포함 여부 확인
- iOS 위젯 변경 미반영
  - CD 로그에서 `Sync iOS widget project files` 단계 성공 여부 확인
  - 로컬에서도 `yarn ios:widget:setup` 실행 후 빌드
