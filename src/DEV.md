

TokenInfo is only a response, no need to store it 


Still to implement:  
PAYMENT
WITHDRAW

execute:  
    Mint         --- DONE
    TransferNft  --- DONE
    SendNft      --- DONE
    Approve      --- DONE
    Revoke       --- DONE
    ApproveAll   --- DONE
    RevokeAll    --- DONE
    AddClass

query: 
    OwnerOf         --- DONE
    Approved        --- DONE
    ApprovedForAll  --- DONE?
    NumTokens       --- DONE
    ContractInfo    --- DONE
    NftInfo         --- DONE
    AllNftInfo      --- DONE??? Vérifier les infos renvoyés, je crois que j'ai pas le bon struct en sortie
    Tokens          --- DONE
    AllTokens       --- DONE


tests: 
    Mint                                    --- DONE
    Transfer                                --- DONE
    Allow + transfer                        --- DONE
    Allow + revoke + transfer               --- DONE
    ApproveAll + transfer                   --- DONE
    ApproveAll + RevokeAll + transfer       --- DONE

