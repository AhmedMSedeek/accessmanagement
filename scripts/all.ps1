# reset, publish, create test tokens
resim reset
$command_output=resim new-account
$env:account=$command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:private_key=$command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge_id=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }
$command_output=resim new-account
$env:account2=$command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:private_key2=$command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge2=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge_id2=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }
$command_output=resim new-simple-badge --name "Auth Badge"
$env:auth_badge=$command_output | Where-Object { $_ -match "NonFungibleGlobalId" } | ForEach-Object { $_.Split(" ")[1].Split(":")[0] }
$env:auth_badge_id=$command_output | Where-Object { $_ -match "NonFungibleGlobalId" } | ForEach-Object { $_.Split(" ")[1].Split(":")[1] }

$command_output=resim publish .
$env:package=$command_output | Where-Object { $_ -match "New Package:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }

$command_output=resim run .\manifests\create_component.rtm
$env:component=$command_output | Where-Object { $_ -match "component:" -And $_ -NotMatch "Address" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$resources=$command_output | Where-Object { $_ -match "resource:" -And $_ -NotMatch "Address" }
$env:component_manager_badge=$resources.Split(":")[1].Replace(" ", "")
$env:access_key_badge=$resources.Split(":")[3].Replace(" ", "")
$non_fungible_ids=$command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
$env:component_manager_badge_id=$non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")

$command_output=resim run .\manifests\deposit_auth_badge.rtm

# Create access key badge
$command_output=resim run .\manifests\create_access_key_badge.rtm
$non_fungible_ids=$command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
$env:access_key_badge_id=$non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")
$non_fungible_ids_index=$command_output.IndexOf($non_fungible_ids)
$env:access_key_badge_vault_address=$command_output[$non_fungible_ids_index-2].Split(":")[1].Replace(" ", "")

# Recall and burn access key badge
$command_output=resim run .\manifests\recall_and_burn_access_key_badge.rtm

# Create auth badge proof
$command_output=resim run .\manifests\create_auth_badge_proof.rtm








$command_output=resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2":"$env:owner_badge_id2
# This transaction must fail
$command_output=resim run .\manifests\test_access_key_badge_withdraw.rtm

# Burn the access key badge
$command_output=resim run .\manifests\holder_burn_access_key.rtm

$command_output=resim set-default-account $env:account $env:private_key $env:owner_badge":"$env:owner_badge_id
