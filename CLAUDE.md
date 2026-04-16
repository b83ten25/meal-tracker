# Meal Tracker

## 프로젝트 개요
병원 직원 식대 관리 앱. 순수 HTML/React CDN. 서버 없음.
Tauri로 패키징하여 Mac(dmg) / Windows(exe) 배포.

## 파일 구조
```
meal-tracker/
├── index.html              # 앱 전체 (React CDN, 스타일, 로직)
├── icon.png                # 앱 아이콘
├── package.json            # npm 스크립트 + @tauri-apps/cli
├── src-tauri/
│   ├── tauri.conf.json     # Tauri 설정 (윈도우, 아이콘, 업데이트)
│   ├── Cargo.toml          # Rust 의존성
│   ├── build.rs
│   ├── icons/icon.png      # Tauri용 아이콘
│   └── src/
│       ├── main.rs         # 진입점
│       └── lib.rs          # 메뉴, 업데이트, 이벤트
├── .github/workflows/
│   └── build.yml           # GitHub Actions: Mac·Win 자동 빌드
└── .gitignore
```

## 실행 / 빌드
```bash
npm run dev         # 로컬 Tauri 개발 실행
npm run build       # 로컬 빌드 (현재 플랫폼)
npm run build:mac   # aarch64-apple-darwin dmg
npm run build:win   # x86_64-pc-windows-msvc exe (NSIS)
```

## Tauri 설정 (src-tauri/tauri.conf.json)
- productName: MealTracker (WiX 한글 미지원으로 영문 사용)
- identifier: com.hospital.meal-tracker
- 윈도우: 1280×800, min 900×600, title "병원 식대 관리"
- bundle.targets: ["app", "dmg", "nsis"] — 플랫폼별 자동 필터링
- 자동 업데이트: GitHub Releases latest.json (단일 endpoint)
- withGlobalTauri: true (window.__TAURI__ 노출)

## Tauri 메뉴바 기능 (src-tauri/src/lib.rs)
- 항상 위에 표시 (토글)
- 다크 모드 / 라이트 모드 (theme-change 이벤트 → index.html 수신)
- 새로고침 (CmdOrCtrl+R)
- 개발자 도구 (CmdOrCtrl+Alt+I)
- 업데이트 확인 (에러 시 다이얼로그 표시)
- 버전 표시

## GitHub Actions (v* 태그 push 시)
- job publish-tauri: matrix (macos-latest / windows-latest) — tauri-action@v0
  - Mac: app + dmg 번들, 서명 포함
  - Win: nsis 번들, 서명 포함
  - 환경변수: TAURI_SIGNING_PRIVATE_KEY, TAURI_SIGNING_PRIVATE_KEY_PASSWORD
- job create-updater-json: publish-tauri 완료 후 ubuntu-latest에서 실행
  - .sig 파일 다운로드 → latest.json 생성 → Release에 업로드
  - latest.json: darwin-aarch64 + windows-x86_64 플랫폼 포함

## 앱 기능 현황
- 직원 5명 기본값 (createdMonth 필드 없음 → getBal에서 month 기준)
- 직원 추가 시 createdMonth 저장 → 이월 계산 기준월로 사용
- PIN 로그인 (4자리)
- 월별 이월 자동 계산
- 다크/라이트 모드 (메뉴바에서만 조작)
- 설정: 잔액 경고 기준, 사용처 즐겨찾기, 비밀번호 초기화

## getBal() 로직
- first = [createdMonth, 첫내역달] 중 더 이른 달 (둘다 없으면 현재달)
- first 달부터 현재달까지 순차 루프하며 carry 누적

## 직원 관리 UX
- 직원 추가: 사이드바 "+ 직원 추가" → AddEmployeeModal
- 직원 삭제: 사이드바 카드 호버 시 × 버튼

## 직원 상세 패널
- 6개월 숫자 테이블: 이번달 파란색★, 미사용 "-"
- 월 한도: detail-hd 인라인 스테퍼 (1,000원 단위, ArrowUp/Down 지원)

## 내역 관리
- 행 클릭 → EditExpenseModal
- 삭제: hover 시 × 아이콘

## UI 스펙
- 요약 카드 4개: 숫자 색상으로 구분, 총 사용 카드에 진행바
- 모달 내 input/select: height 52px, font-size 16px
- fmtS: 10000원 미만 "X,XXX원", 이상 "X만원"/"X.X만원"

## 배포 플랫폼
- Windows (NSIS exe) + macOS (dmg) 동시 빌드
- Electron 시절 macOS 26.x 크래시 이슈는 Tauri 전환으로 해소 여부 확인 필요

## 릴리즈 현황
- 현재 GitHub Releases: v1.4.17 (Latest) 만 유지, 이전 릴리즈 및 태그 전체 삭제됨

## 업데이터 동작 (v1.4.17~)
- 업데이트 감지 → 프론트엔드에 `update-status` 이벤트 emit → 토스트 "새 버전 다운로드 중..." 표시
- 다운로드 완료 → `app.restart()` 자동 호출 (다이얼로그 없음)
- 메뉴 수동 확인: 최신 버전이면 다이얼로그 표시, 아니면 자동 다운로드+재시작
- 자동 확인(앱 시작 시): 최신이어도 다이얼로그 없음 (silent)

## 알려진 버그
- 없음

## 과거 버그 (수정됨)
- 직원 삭제 후 흰 화면: `empCI()` 함수가 없는 직원 ID에 대해 `findIndex` → `-1` 반환, `-1 % COLORS.length = -1`, `COLORS[-1] = undefined` → `.bg` 접근 시 TypeError. `monthExps.map()`에서 삭제된 직원의 내역이 남아있을 때 발생. `empCI` 를 `i<0?0:i` 처리로 수정.

## 작업 규칙
- 프론트엔드 파일 1개 (index.html) 유지
- 수정 전 반드시 원인 파악 먼저
- 수정 후 브라우저에서 직접 확인
