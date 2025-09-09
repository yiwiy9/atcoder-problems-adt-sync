# DynamoDB Client Architecture

DynamoDB client library for AtCoder Problems ADT Sync backend services.
Provides data models and operations for storing user AC problems and contest information.

## Data Model Design

### Single Table Design

This system uses a single DynamoDB table with composite primary keys (`PK` + `SK`) to store both user data and contest metadata efficiently.

### Record Types

#### 1. UserAcProblemRecord

Stores a user's AC (Accepted) problems list.

**Partition Key (PK)**: `USER_AC#{user_id}`  
**Sort Key (SK)**: `AC`  
**Attributes**: `ac_problems` (List of problem IDs)

```rust
UserAcProblemRecord {
    pk: "USER_AC#username123",
    sk: "AC", 
    ac_problems: ["abc001_a", "abc002_b", "abc400_c"]
}
```

#### 2. AdtContestRecord

Stores AtCoder Daily Training contest metadata for batch processing.

**Partition Key (PK)**: `CONTEST#{YYYYMM}` (year-month partitioning to avoid hot partitions)  
**Sort Key (SK)**: `{start_epoch_second}-{difficulty_order}`  
**Attributes**: `contest_id`, `last_fetched_submission_id`

```rust  
AdtContestRecord {
    pk: "CONTEST#202505",
    sk: "1746688000-02",  // epoch-difficulty_order
    contest_id: "adt_all_20250522_3", 
    last_fetched_submission_id: 66203973
}
```

**Difficulty Order**: Easy=1, Medium=2, Hard=3, All=4

## Design Decision: User AC List vs Individual Submissions

### Cost & Performance Analysis

The current USER_AC design was chosen over individual submission records based on DynamoDB free tier constraints:

#### 1. ADT Scale Estimation

```bash
Problem Pool (Past AtCoder contests):
- Historical contests: ~400 contests × 6 problems = 2,400 problems

ADT Practice Contest Schedule:
- ADT contests per year: 12 months × 4 weeks × 3 days × 4 levels × 3 contests = 1,728 ADT contests/year
- Problems per ADT: 5 problems (average across difficulty levels)
- Note: 4 levels = Easy, Medium, Hard, All; 3 contests = multiple time slots per day

User Activity:
- Average submissions per ADT problem: 15 users × 2 attempts = 30 submissions/problem  
- Annual submissions: 1,728 ADT contests × 5 problems × 30 submissions = 259,200/year
- Current total (1.5 years): 259,200 × 1.5 = 388,800 submissions

Per-user submissions: 388,800 ÷ 100 active users = 3,888 submissions/user
```

#### 2. Read Capacity Analysis (DynamoDB Free Tier: 25 RCU = 50KB/sec)

```bash
Individual Submissions Approach:
- Data per user query: 3,888 submissions × 100B = 388,800B ≈ 389KB
  (100B = estimated record size per submission)
- RCU required: 389KB ÷ 4KB = 97.25 RCU
- Result: EXCEEDS free tier (25 RCU) by 4×

USER_AC Approach:
- Data per user query: 2,400 problem IDs × 10B = 24,000B ≈ 24KB
  (10B = estimated record size per problem ID)
- RCU required: 24KB ÷ 4KB = 6 RCU
- Result: Well within free tier (25 RCU)
```

#### 3. Storage Analysis (DynamoDB Free Tier: 25GB)

```bash
Current Storage Requirements (USER_AC approach only):
- USER_AC records: 10,000 users × 24KB × 50% AC rate = 120MB
- ADT contest metadata: 1,728 contests/year × 1.5 years × 80B = 0.21MB
  (80B = estimated record size per contest: pk + sk + contest_id + submission_id)
- Total current: 120MB + 0.21MB ≈ 120MB

Annual Growth:
- New USER_AC data: 10,000 users × 1,296 new problems/year × 10B × 50% AC rate = 6.5MB/year
- New contest metadata: 1,728 contests/year × 80B = 0.14MB/year
- Total growth: 6.5MB + 0.14MB ≈ 6.6MB/year
- Available capacity: 25,000MB - 120MB = 24,880MB
- Estimated lifespan: 24,880MB ÷ 6.6MB = 3,770+ years
```

**Conclusion**: USER_AC aggregation approach reduces read costs by 75% and enables 3,770+ years of operation within free tier constraints. Additionally, USER_AC storage is naturally bounded by AtCoder's problem count growth rate, while individual submissions grow unbounded annually.

## Access Patterns

### Primary Operations

1. **Get User AC Problems**
   - Query: `PK = USER_AC#{user_id} AND SK = AC`
   - Used by: API Lambda for Chrome extension requests

2. **Batch Get Multiple Users**
   - BatchGetItem: Multiple `USER_AC#` keys  
   - Used by: Loading existing user AC records for merging with new submission data

3. **Get Contests by Month**
   - Query: `PK = CONTEST#{YYYYMM}`
   - Sort by SK (chronological order)
   - Used by: Finding contests that need submission processing
   - Note: Year-month partitioning distributes load across multiple partitions, avoiding hot partition issues

4. **Batch Write Operations**
   - BatchWriteItem: Up to 25 items per request
   - Used by:
     - Storing newly discovered contest metadata
     - Updating contest records with latest processed submission IDs
     - Updating user AC records with merged submission data
