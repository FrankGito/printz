# PSP34 item contract

The item contract is the implementation of the PSP34 standard. The individual nfts are used to
declare ownership. Furthermore, the PSP34 metadata extension can be used in a way so that each item
in the collection is representing a 3d model that can be printed.

## Use cases

### Minting

A user can mint (create) a 3d model by executing the `set_attribute` call on the contract with a new
id value for each of the metadate properties to be set. After that, the user can execute the `mint`
call with the given id. This way the caller is marked as the owner of the nft.

### Transfer/approve

These calls are also implemented as per the PSP34 specification, allowing the transfer of specific
nfts.
