### Frank Tests
Mint works
- Token does not exist
- Person has no Token
- mint returns ok
- person has token

Mint fails
- Cannot create token if exists

Transfer works
- Person A transfer
- Person B has 1
- Person A has 0

Invalid Transfer
- Token does not exists
- Person B cannot Transfer because not owner

Approved Transfer
- Create Token works with Person A
- Approve works on Person B works
- Transfer works with Person B works
- Token is owned by Person C
- Person A has 0 token
- Person B has 0 token

Invalid Approve
- Create Token works
- Person A owns Token
- Person B can not transfer
- Person A owns Token

### Lis of all unit tests from Cardinal

- mint_works()
- mint_existing_should_fail()
- transfer_works()
- transfer_emits_event() {              
- invalid_transfer_should_fail()
- approved_transfer_works()
- approve_emits_event() {              
- approved_for_all_works()
- approved_for_all_revoke_single_approval_should_fail()
- not_approved_transfer_should_fail()
- burn_works()
- burn_fails_token_not_found()
- burn_fails_not_owner()
