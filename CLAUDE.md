# Meal Tracker

## 프로젝트 개요
병원 직원 식대 관리 앱. 순수 HTML/React CDN. 서버 없음.
Electron으로 패키징하여 Mac(dmg) / Windows(exe) 배포.

## 파일 구조
```
meal-tracker/
├── index.html          # 앱 전체 (React CDN, 스타일, 로직)
├── main.js             # Electron 메인 프로세스
├── package.json        # npm 스크립트 + electron-builder 설정
├── .github/workflows/
│   └── build.yml       # GitHub Actions: Mac·Win 자동 빌드
└── .gitignore
```

## 실행 / 빌드
```bash
npm start           # 로컬 Electron 실행
npm run build:mac   # dist/*.dmg 생성
npm run build:win   # dist/*.exe 생성 (NSIS 설치형)
```

## Electron 설정
- BrowserWindow: 1280×800, min 900×600
- titleBarStyle: default
- appId: com.hospital.meal-tracker
- productName: 식대관리

## GitHub Actions (push to main)
- job build-mac (macos-latest): dmg 아티팩트 업로드
- job build-win (windows-latest): exe 아티팩트 업로드

## 앱 기능 현황
- 직원 5명 기본값 (createdMonth 필드 없음 → getBal에서 month 기준)
- 직원 추가 시 createdMonth 저장 → 이월 계산 기준월로 사용
- PIN 로그인 (4자리)
- 월별 이월 자동 계산
- 다크/라이트 모드
- 설정: 다크모드 토글, 잔액 경고 기준, 사용처 즐겨찾기, 비밀번호 초기화

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
- **Windows only** (NSIS exe)
- Mac 패키징 앱은 macOS 26.x에서 `v8::Context::FromSnapshot` 크래시 (EXC_BREAKPOINT) 발생
  - 원인 미확인 (바이너리 identical, 서명 동일, JIT entitlement 추가해도 재현)
  - `npm start`는 정상 작동 → Mac 개발/테스트는 개발 모드로만 사용
  - Mac DMG 배포 중단

## 알려진 버그
- 없음

## 과거 버그 (수정됨)
- 직원 삭제 후 흰 화면: `empCI()` 함수가 없는 직원 ID에 대해 `findIndex` → `-1` 반환, `-1 % COLORS.length = -1`, `COLORS[-1] = undefined` → `.bg` 접근 시 TypeError. `monthExps.map()`에서 삭제된 직원의 내역이 남아있을 때 발생. `empCI` 를 `i<0?0:i` 처리로 수정.

## 작업 규칙
- 파일 1개 (index.html) 유지
- 수정 전 반드시 원인 파악 먼저
- 수정 후 브라우저에서 직접 확인
