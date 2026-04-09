# Meal Tracker

## 프로젝트 개요
병원 직원 식대 관리 앱. 순수 HTML/React CDN. 서버 없음.

## 현재 상태
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
- 직원 추가: 사이드바 "+ 직원 추가" → AddEmployeeModal (이름 + 한도 스테퍼)
- 직원 삭제: 사이드바 직원 카드 호버 시 × 버튼 → confirm 후 즉시 삭제

## 직원 상세 패널
- 6개월 숫자 테이블 (MonthlyTable): 이번달 파란색★, 미사용 "-"
- 월 한도: detail-hd 우측 인라인 스테퍼 (30px 버튼, 1,000원 단위, 최소 10,000원)
  - ArrowUp/Down 키보드 지원

## 내역 관리
- 행 클릭 → EditExpenseModal (직원/날짜/금액/사용처 pre-fill)
- 삭제: hover 시 우측 × 아이콘만 표시 (텍스트 없음, 클릭해도 행 선택 안 됨)
- compact 레이아웃: padding 7px, 사용처+날짜 한 줄

## UI 스펙
- 요약 카드 4개: 숫자 색상으로 구분, 총 사용 카드에 진행바
  - 레이블 11px #999 bold, 숫자 20px bold, 서브 12px
- 모달 내 input/select: height 52px, font-size 16px
- 모든 버튼/select/input 최소 높이 48px (단, limit-inline-btn·txn-del 등 작은 버튼은 min-height:unset)
- fmtS: 10000원 미만 "X,XXX원", 이상 "X만원"/"X.X만원"
- 직원 상세 통계 카드: fmtS 만원 단위 표시

## 알려진 버그
- 없음

## 작업 규칙
- 파일 1개 (index.html) 유지
- 수정 전 반드시 원인 파악 먼저
- 수정 후 브라우저에서 직접 확인
