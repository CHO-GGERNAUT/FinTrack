# 금융 관리 시스템 도메인 요소 (기능 단위 구성)

이 문서는 금융 정보 관리 시스템의 도메인 모델 요소를 주요 기능 단위로 그룹화하여 보여줍니다. 각 기능은 관련된 애그리거트, 엔티티, 주요 값 객체를 포함합니다.

## 0. 공통 값 객체 (Shared Kernel VOs)

여러 기능/애그리거트에서 공통적으로 사용되는 핵심 값 객체입니다.

* `Amount`: 금액과 통화를 포함하는 불변 객체.
* `Currency`: 통화 코드 (ISO 4217).
* `Country`: 국가 코드

## 1. 사용자 관리 (User Management)

사용자 관리

* **핵심 애그리거트:**
    * `User` (Root): 사용자 정보, 인증/보안, 기본 설정 관리

* **관련 엔티티 (User 애그리거트 내부):**
    * `CustomCategory`: 사용자가 직접 정의하는 카테고리 정보

* **주요 값 객체:**
    * `Email`: 이메일
    * `PhoneNumber`: 연락처

## 2. 계좌 관리 (Account Management)

다양한 금융 계좌(은행, 증권, 카드 등)의 연결, 정보 조회, 거래 내역 관리

* **핵심 애그리거트:**
    * `FinancialAccount` (Root): 계좌/카드 정보, 잔액(파생), 거래 내역 목록 관리.

* **관련 엔티티 (FinancialAccount 애그리거트 내부):**
    * `Transaction`: 개별 거래 내역 상세 정보
    * `Holding`: 보유 자산(주식, 코인 등) 정보

* **주요 값 객체:**
    * `Institution`: 금융 기관 정보
    * `AccountNumber`: 계좌/카드 번호 (마스킹)
    * `AccountType`: 계좌/카드 종류 Enum
    * `AssetName`: 자산 식별자 (주식 티커, 코인명 등)
    * `Quantity`: 보유 자산 수량

## 3. 예산 관리 (Budget Management)

월별 또는 특정 기간의 예산을 설정하고 관리

* **핵심 애그리거트:**
    * `Budget` (Root): 예산 기간, 총 예산, 카테고리별 한도 관리. `User`에 속함.

* **관련 엔티티 (Budget 애그리거트 내부):**
    * `BudgetCategoryLimit`: 특정 카테고리의 예산 한도액.

* **주요 값 객체:**
    * `DateRange`: 예산 적용 기간.
    * `Category`: 예산 설정의 기준이 되는 카테고리.
    * *(의존)*: `Money`, `Currency` 공통 VO 사용.

## 4. 재정 목표 관리 (Financial Goal Management)

사용자의 재정 목표 설정 및 달성률 추적 기능입니다.

* **핵심 애그리거트:**
    * `FinancialGoal` (Root): 목표 정보(이름, 금액, 기한), 달성 현황 관리. `User`에 속함.

* **관련 엔티티 (FinancialGoal 애그리거트 내부):**
    * `GoalContribution`: 목표 달성을 위한 개별 적립/기여 내역.

* **주요 값 객체:**
    * *(의존)*: `Money`, `Currency`, `Timestamp` 공통 VO 사용.

## 5. 거래 관리 및 분류 (Transaction Management & Categorization)

거래 내역을 기록하고 카테고리 및 태그를 통해 분류/분석하는 핵심 기능입니다. (별도 애그리거트보다는 `FinancialAccount` 내의 기능에 가까움)

* **관련 애그리거트:** `FinancialAccount` (거래 내역 포함)
* **관련 엔티티:** `Transaction`
* **주요 값 객체:**
    * `TransactionDescription`: 거래 내용 설명.
    * `Category`: 거래 분류.
    * `Tag`: 사용자 정의 태그.
    * *(의존)*: `Money`, `Currency`, `Timestamp` 공통 VO 사용.
* **연관 요소:** `User` 애그리거트의 `CustomCategory` 엔티티와 연관될 수 있음.

## 6. 반복 거래 관리 (Recurring Transaction Management)

정기적인 수입/지출을 규칙 기반으로 관리하는 기능입니다.

* **핵심 애그리거트:**
    * `RecurringTransactionRule` (Root): 반복 규칙(주기, 금액, 계좌 등) 관리. `User`에 속함.

* **주요 값 객체:**
    * `Frequency`: 반복 주기 Enum (매일, 매주, 매월 등).
    * `Category`: 자동 생성될 거래의 카테고리.
    * *(의존)*: `Money`, `Currency`, `Timestamp`, `AccountNumber` (연결 계좌) 등 사용.




금융 정보를 관리하는 프로그램을 만들거야.



먼저 서버 부터 만들거야.



필요한 기능은



1. 사용자가 있어.

2. 사용자는 은행 계좌, 증권사 계좌, 가상화폐 거래소 계좌, 카드(신용/체크 등)을 가져.

3. 각 계좌 카드 등의 거래 내역을 모두 DB에 저장해.

4. 월별로 지출에 대한 분석, 수입에 대한 분석, 손익에 대한 분석 등을 제공할거야


5. 자동 거래 내역 연동: 사용자가 직접 모든 내역을 입력하는 것은 매우 번거롭습니다. Open Banking API, 증권사 API, 카드사 API 등을 활용하거나, 스크래핑 기술(법적/기술적 제약 확인 필요) 또는 금융 데이터 통합 서비스(예: Plaid, 국내 유사 서비스)를 연동하여 거래 내역을 자동으로 가져오는 기능을 구현하면 사용자 편의성이 크게 향상됩니다.
수동 입력 편의 기능: 자동 연동이 어려운 자산(현금, 부동산 등)이나 거래 내역을 위한 수동 입력 기능을 개선합니다.
반복 거래 설정: 월급, 월세, 구독료 등 정기적인 수입/지출을 설정하여 자동으로 기록되게 합니다.
거래 분할: 하나의 거래 내역을 여러 카테고리로 나누어 기록하는 기능 (예: 마트에서 장보기 + 생필품 구매).
데이터 가져오기/내보내기: 다른 가계부 앱이나 은행/증권사에서 제공하는 CSV, Excel 파일 등을 가져와 초기 데이터를 쉽게 구축하거나, 필요시 데이터를 내보낼 수 있는 기능을 제공합니다.
거래 내역 자동 분류 (Categorization): 거래 내역의 내용(가맹점명, 입금자명 등)을 기반으로 자동으로 카테고리(식비, 교통비, 월급 등)를 분류해주는 엔진을 구현합니다. 사용자가 규칙을 직접 설정하거나 수정할 수도 있습니다.
태그(Tag) 기능: 카테고리 외에 사용자가 자유롭게 태그를 달아 거래 내역을 관리할 수 있게 합니다. (예: #여름휴가, #프로젝트X 관련 지출)
6. 분석 및 리포팅 기능 심화:

예산 설정 및 관리: 월별, 카테고리별 예산을 설정하고 지출 현황을 예산과 비교하여 시각적으로 보여줍니다. 예산 초과 시 알림 기능도 유용합니다.
자산/부채 현황 및 순자산 추이: 모든 계좌의 잔액과 부채(대출 등)를 합산하여 현재 총 자산, 부채, 순자산 현황을 보여주고 시간 경과에 따른 변화 추이를 그래프로 제공합니다.
투자 포트폴리오 분석: 증권 계좌 및 가상화폐 계좌의 자산을 통합하여 현재 포트폴리오 구성(자산군별 비중), 투자 수익률(기간별, 자산별), 평가 손익 등을 상세히 분석해줍니다.
현금 흐름 예측: 고정적인 수입/지출, 예산 등을 바탕으로 미래의 예상 현금 흐름을 예측해주는 기능을 제공할 수 있습니다.
커스텀 리포트 생성: 사용자가 원하는 기간, 계좌, 카테고리, 태그 등을 조합하여 맞춤형 보고서를 생성하고 조회할 수 있는 기능을 제공합니다.
7. 사용자 경험 및 부가 기능:

목표 관리: 특정 목표(주택 구매 자금 마련, 자동차 구매, 여행 자금 등)를 설정하고 관련 저축 현황을 추적하는 기능을 제공합니다.
다중 통화 지원: 해외 자산이나 외화 거래가 있는 사용자를 위해 여러 통화를 지원하고 환율을 적용하여 원화 기준으로 통합 관리할 수 있게 합니다.
보안 강화: 금융 정보는 민감 정보이므로 2단계 인증(MFA), 데이터 암호화(전송 중, 저장 시), 정기적인 보안 감사 등 강력한 보안 조치를 적용해야 합니다.
DDD/CQRS 관점에서의 고려사항:

이벤트 소싱 (Event Sourcing): 상태를 직접 저장하는 대신 상태 변경 '이벤트'들을 순차적으로 저장하는 방식을 고려해볼 수 있습니다. 모든 변경 이력을 추적할 수 있고, 특정 시점의 상태를 재구성하거나 다양한 읽기 모델(Read Model)을 유연하게 구축하는 데 장점이 있습니다. 다만 구현 복잡도가 증가합니다.
읽기 모델 최적화: 월별 분석, 자산 추이 등 다양한 조회 요구사항에 맞춰 최적화된 읽기 모델(별도의 테이블이나 View 등)을 설계하는 것이 중요합니다. CQRS의 핵심적인 부분입니다.
명령(Command)과 쿼리(Query)의 명확한 분리: 기능 추가 시 해당 기능이 상태를 변경하는 명령인지, 데이터를 조회하는 쿼리인지 명확히 구분하여 설계합니다.
Bounded Context 정의: 사용자, 계좌, 거래, 분석 등 주요 도메인 영역을 Bounded Context로 명확히 나누고 각 Context 간의 관계를 정의하는 것이 중요합니다.


너가 생각하기에 더 필요한 기능이 있을까?



구체적으로는 DDD + CQRS 가빈이고,

Rust, Postgres, Rest API 사용할거야 ( 추후에 gRPC 도 지원할거야 )




생각하는 과정 표시


```rust
use async_trait::async_trait;

use crate::domain::repositories::{
    AccountRepository, CardTransactionRepository, MerchantRepository,
};

use super::UnitOfWork;

#[async_trait]
pub trait CardTransactionUnitOfWork: UnitOfWork {
    type AccountRepo<'a>: AccountRepository + Send
    where
        Self: 'a;
    type CardTransactionRepo<'a>: CardTransactionRepository + Send
    where
        Self: 'a;
    type MerchantRepo<'a>: MerchantRepository + Send
    where
        Self: 'a;

    fn transaction_repo(&mut self) -> Self::CardTransactionRepo<'_>;
    fn account_repo(&mut self) -> Self::AccountRepo<'_>;
    fn merchant_repo(&mut self) -> Self::MerchantRepo<'_>;
}

```