#Access Management
Access Management is a Scrypto package for Access Manager Blueprint, which allows a Radix user who has an NFT in his possession that has some kind of permission to delegate the permission of that NFT to several other people

#Terminology
**Auth Badge**: the original NFT that has the permission the owner wants to delegate
**Access Manager Owner Badge**: a badge issued to the component instantiator
**Access Key Badge**: a badge issued by the persom holding the "Component Owner Badge" to be given to the delegates
**Delegate**: The delegate is the user that the owner of the "Auth Badge" NFT desires to **delegate** the permission of the "Auth Badge" to, so that he can take actions that requires the authority of the "Auth Badge" on behalf of the owner of the "Auth Badge"

#Delegation Methodology Explained
By simply creating a component of this access manager blueprint, a "Access Manager Owner Badge" is given in return, which in turn allows the instantiator to issue and send "Access Key NFTs" to selected people, and recall those "Access Key NFTs" when/if necessary.
After depositing the "Auth Badge" NFT inside the created component, the holder of an "Access Key NFT" can call a specific method in the component that creates a "Proof" of the "Auth NFT" and return it to the caller, thus allowing the caller to take actions that require the authority of the "Auth NFT".

#Usage
##Create Access Manager Component
To create an access manager component, use the following transaction manifest syntax
```
CALL_FUNCTION
    Address("${package}")
    "AccessManager"
    "new"
    Address("${auth_badge}")
    Address("${account}");

CALL_METHOD Address("${account}") "deposit_batch" Expression("ENTIRE_WORKTOP");
```
After creating the access manager component, an "Access Manager Owner Badge" is minted and returned to the caller, we will use this badge to perform privileged actions later
##Create Access Manager Component with address reservation
Sometimes you need to create the component with address reservation on the transaction manifest level, to do so, use the following transaction manifest syntax
```
ALLOCATE_GLOBAL_ADDRESS
    Address("${package}")
    "AccessManager"
    AddressReservation("address_reservation")
    NamedAddress("component_address");

CALL_FUNCTION
    Address("${package}")
    "AccessManager"
    "new_with_address_reservation"
    Address("${auth_badge}")
    Address("${account}")
    AddressReservation("address_reservation");

CALL_METHOD Address("${account}") "deposit_batch" Expression("ENTIRE_WORKTOP");
```
##Depositing The Auth Badge
After creating the access manager component, you need to deposit the auth badge into it for the component to be able to create proof of that Auth Badge, to do so, use the following transaction manifest syntax
```
CALL_METHOD Address("${account}") "withdraw_non_fungibles" Address("${auth_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${auth_badge_id}"));
TAKE_NON_FUNGIBLES_FROM_WORKTOP Address("${auth_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${auth_badge_id}")) Bucket("auth_badge_bucket");

CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "deposit_auth_badge"
    Bucket("auth_badge_bucket");
```
##Creating (Minting) an access key badge
The access manager owner can create an "Access Key Badge" and give it to the delegate person, using the following transaction manifest syntax
```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "create_access_key_badge";

CALL_METHOD Address("${delegate_account}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;
```
**Note**: You can use the direct manifest mint instructions directly without calling the component as the "mint" permission is given to both the "Access Manager Owner Badge" and the component itself, the component "create_access_key_badge" method is provided for completion
**Note**: the created key only be moved between accounts after it is given to the delegate by the owner of the NFT, by creating a proof of the "Access Manager Owner Badge" in the transaction manifest, after that, if the "Access Key Badge" exists in his own account, he can withdraw it normally, if not, he can recall the "Access Key Badge" from the vault it is in, and then deposit it normally to anyne else (given that he passes other deposit restrictions the receiver has in place)
##Recall an Access Key Badge
to recall a previously issued Access Key Badge, use the following transaction manifest syntax
```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "recall_key_badge"
    Address("${access_key_badge_vault_address}");

CALL_METHOD Address("${account}") "deposit_batch" Expression("ENTIRE_WORKTOP");
```
**Note**: You can use the direct manifest recall instructions directly without calling the component as the "recall" permission is given to both the "Access Manager Owner Badge" and the component itself, the component "recall_key_badge" method is provided for completion
##Burn an Access Key Badge
To burn an access key badge, you can use the following transaction manifest syntax
```
CALL_METHOD
    Address("${component}")
    "burn_key_badge"
    Bucket("access_key_badge_bucket");
```
**Note**: You can use the direct manifest burn instructions directly without calling the component as the "burn" permission is set to "allow_all" so that anyone can burn the access key badge in his custody, the component "recall_key_badge" method is provided for completion
**Note**: By allowing any access key badge holder to burn the key in his custody this simply means that the delegate can give up the delegated authority/permission whenever he desires, but in order for him to "re-gain" the permission, the access manager owner needs to mint a new access key badge and give it to him
##Create Auth Badge Proof
This method allows both the "Access Manager Owner Badge" holder and the "Access Key Badge" holder to create a proof of the "Auh Badge" to be used in privileged actions in the same transaction manifest
To create a proof of the "Auth Badge" held inside the "Access Manager" component, use the following syntax
```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_key_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_key_badge_id}"));

CALL_METHOD
    Address("${component}")
    "create_auth_badge_proof";
```
**Note**: it's assumed in the above manifest that the holder of the "Access Key Badge" is the one requesting the "Auth Badge" proof, but the permission is given to both "Access Manager Owner Badge" and the "Access Key Badge", so the access manager owner can also create a proof from the "Auth Bagde" without the need to create a separate "Access Key Badge"
##Withdraw Auth Badge
At any time, the owner of the access manager component can withdraw the "Auth Badge" from the component, after this action, the access manager component will no longer be able to create a proof for the "Auth Badge"
To withdraw the "Auth Badge" from the access manager component, use the following syntax
```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "withdraw_auth_badge";

CALL_METHOD Address("${account}") "deposit_batch" Expression("ENTIRE_WORKTOP");
```