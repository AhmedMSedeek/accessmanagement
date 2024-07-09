**Access Management**

Access Management is a Scrypto package for Access Manager Blueprint, which allows a Radix user who has an NFT in his possession that has some kind of permission to delegate the permission of that NFT to several other people


**Terminology**

Auth Badge: the original NFT that has the permission the owner wants to delegate
Component Owner Badge: a badge issued to the component instantiator
Access Key Badge: a badge issued by the persom holding the "Component Owner Badge" to be given to the delegates

**Delegation Methodology Explained**

By simply creating a component of this access manager blueprint, a "Component Owner Badge" is given in return, which in turn allows the instantiator to issue and send "Access Key NFTs" to selected people, and recall those "Access Key NFTs" when/if necessary.
After depositing the "Auth Badge" NFT inside the created component, the holder of an "Access Key NFT" can call a specific method in the component that creates a "Proof" of the "Auth NFT" and return it to the caller, thus allowing the caller to take actions that require the authority of the "Auth NFT".

**Usage**


