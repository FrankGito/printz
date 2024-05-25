# PSP34 token sale auction contract

This contract will implement a safe public sale of a PSP34 nft. The implementation will be generic,
so that it can be used for any PSP34 token sale, but the process is more tailored to an nft sale
that supports negotiation of the price and even allowing trading other PSP34 nfts and the
combination of Balance + nfts.

## Use cases

### Initiate a sell

- Who: the seller can start a new sale.
- How: the contract needs to be deployed, while also sending the PSP34 nfts to it.

### Bidding

- Who: any buyer can bid for the nft(s).
- How:
  1. (optional) If the bidder want to offer other PSP34 nfts, first he needs to set the allowance of
     them to the sell contract by invoking the PSP34 `allow` calls on each of them.
  2. The bidder then executes the `bid` call, sending the offered balance and passing in a list that
     contains each of the offered nfts along with their description (if provided).
  3. The contract will accept the balance, transfer the nft(s) to itself, updating the Mapping that
     represents the live bids and emits an event so that the seller is notified of a new bid.

### Updating the bid

- Who: any buyer who already did a valid bid.
- How:
  1. (optional) If the user wants to add more nfts to the bid, he needs to execute the `allow` call
     on the relevant PSP34 nfts.
  2. The bidder executes the `modify_bid` call with the following parameters:
     - `add_assets`: the nft(s) to add to the bid
     - `remove_assets`: the nft(s) the user wants to revoke from the bid
     - `remove_value`: the balance the user would like to receive back
     - `message`: an optional message that will be included in the generated event
     - transfer value: if the user wants to add more balance to the bid, he just needs to transfer
       the value with the call
  
### Accepting a bid

- Who: the seller.
- How: the seller can accept an incoming bid from the specified account with the `accept_bid`
  call. The smart contract sends the items to the bidder and the bid to the seller.
  
### Rejecting a bid

- Who: the seller.
- How: the seller can reject an incoming bid from the specified account with the `accept_bid` call.
  In this case the seller should include a message to indicate why he rejected the bid, so the
  bidder can modify his bid to become more acceptable.

### Revoking a bid

- WHo: any buyer who already did a valid bid.
- How: by executing the `reject_bid` call with an optional message, the bidder gets all his bids
  (nft(s) plus balance) back from the smart contract. In case another bid is accepted, unsuccessful
  bidders can use this call to retrieve their funds.

#### Security considerations

Careful consideration and implementation around the foreign PSP34 nft `transfer` calls are needed.
While it is expected that the contracts will actually be PSP34 tokens, an attacker can inject a
harmful smart contract with the PSP34 interface implemented. If the security of the smart contract
and the safety of the funds cannot be guaranteed, it is more prudent to remove the option to enter
other PSP34 tokens in the bidding process.
