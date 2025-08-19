# Tests runner for accessmanager manifests
# This script performs setup, runs each manifest, and prints simple pass/fail checks.

# Helper: assert that output contains a pattern
function Assert-Contains($outputLines, $pattern, $testName) {
    if ($outputLines -match $pattern) {
        Write-Host "[PASS] $testName" -ForegroundColor Green
        return $true
    } else {
        Write-Host "[FAIL] $testName - pattern not found: $pattern" -ForegroundColor Red
        return $false
    }
}

Write-Host "=== Access Manager manifests test runner ==="

# Environment reset and accounts
Write-Host "-- resim reset and create test accounts --"
resim reset | Out-Null
$command_output = resim new-account
$env:account = $command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:private_key = $command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:public_key = $command_output | Where-Object { $_ -match "Public key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge = $command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge_id = $command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }

$command_output = resim new-account
$env:account2 = $command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:private_key2 = $command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:public_key2 = $command_output | Where-Object { $_ -match "Public key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge2 = $command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge_id2 = $command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }

$command_output = resim new-account
$env:account3 = $command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:private_key3 = $command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:public_key3 = $command_output | Where-Object { $_ -match "Public key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge3 = $command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$env:owner_badge_id3 = $command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }
Write-Host "Accounts created: $env:account , $env:account2 , $env:account3"

# Create Validator
Write-Host "-- Creating validator --"
$command_output = resim run .\manifests\create_validator.rtm
if ($command_output -match "Success") {
    Write-Host "[PASS] Validator created successfully" -ForegroundColor Green
} else {
    Write-Host "[FAIL] Validator creation failed" -ForegroundColor Red
}
$env:validator = $command_output | Where-Object { $_ -match "Component:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
$env:auth_badge_id = $non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{[", "[").Replace("]},-{}", "]")
$index = $command_output.IndexOf($non_fungible_ids)
$previous_line = $command_output[$index - 1]
$env:auth_badge = ($previous_line -split "ResAddr:\s+")[1].Trim()

# # Create simple auth badge used in some tests
# $command_output = resim new-simple-badge --name \"Auth Badge\"
# $env:auth_badge = $command_output | Where-Object { $_ -match "NonFungibleGlobalId" } | ForEach-Object { $_.Split(" ")[1].Split(":")[0] }
# $env:auth_badge_id = $command_output | Where-Object { $_ -match "NonFungibleGlobalId" } | ForEach-Object { $_.Split(" ")[1].Split(":")[1] }

# Write-Host "Auth badge: $env:auth_badge:$env:auth_badge_id"

# Publish package
$command_output = resim publish .
$env:package = $command_output | Where-Object { $_ -match "New Package:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
Assert-Contains $command_output "New Package:" "Publish package"

Write-Host "-- Running manifests and basic checks --"

# create_component.rtm: should emit component and resources
$command_output = resim run .\manifests\create_component.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "create_component - component created"
$env:component=$command_output | Where-Object { $_ -match "component:" -And $_ -NotMatch "Address" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$resources = $command_output | Where-Object { $_ -match "resource:" -And $_ -NotMatch "Address" }
$env:component_manager_badge=$resources.Split(":")[1].Replace(" ", "")
if ($resources) {
    Write-Host "[PASS] create_component - resources line found" -ForegroundColor Green
} else {
    Write-Host "[FAIL] create_component - resources line not found" -ForegroundColor Red
}
$env:component_manager_badge=$resources.Split(":")[1].Replace(" ", "")
$env:access_key_badge=$resources.Split(":")[3].Replace(" ", "")
$non_fungible_ids=$command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
$env:component_manager_badge_id=$non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")

# deposit_auth_badge.rtm: should accept deposit action
$command_output = resim run .\manifests\owner\deposit_auth_badge.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "deposit_auth_badge - success indicator"

# create_super_key_badge.rtm: should create a non-fungible id
$command_output = resim run .\manifests\owner\create_super_key_badge.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "create_super_key_badge - issued NF id"
$non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
if ($non_fungible_ids) {
    $env:access_key_badge_id = $non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")
    Write-Host "Access key badge id: $env:access_key_badge_id"
    $non_fungible_ids_index=$command_output.IndexOf($non_fungible_ids)
    $env:access_key_badge_vault_address=$command_output[$non_fungible_ids_index-2].Split(":")[1].Replace(" ", "")
}


# key_holder/validator_update_key.rtm
resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2`:$env:owner_badge_id2 | Out-Null
$command_output = resim run .\manifests\key_holder\validator_update_key.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "key_holder\validator_update_key - key updated"

# key_holder/create_basic_key_badge.rtm: should create a non-fungible id
resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2`:$env:owner_badge_id2 | Out-Null
$command_output = resim run .\manifests\key_holder\create_basic_key_badge.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "create_basic_key_badge - issued NF id"
$non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
if ($non_fungible_ids) {
    $env:access_key_badge_id2 = $non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")
    Write-Host "Access key badge id: $env:access_key_badge_id"
    $non_fungible_ids_index=$command_output.IndexOf($non_fungible_ids)
    $env:access_key_badge_vault_address2=$command_output[$non_fungible_ids_index-2].Split(":")[1].Replace(" ", "")
}

# key_holder/create_custom_key_badge.rtm
$command_output = resim run .\manifests\key_holder\create_custom_key_badge.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "create_custom_key_badge - issued NF id"
$non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
if ($non_fungible_ids) {
    $env:access_key_badge_id3 = $non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")
    Write-Host "Access key badge id: $env:access_key_badge_id"
    $non_fungible_ids_index=$command_output.IndexOf($non_fungible_ids)
    $env:access_key_badge_vault_address3=$command_output[$non_fungible_ids_index-2].Split(":")[1].Replace(" ", "")
}

# recall_and_burn_access_key_badge.rtm
$command_output = resim run .\manifests\recall_and_burn_access_key_badge.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "recall_and_burn_access_key_badge - recall indicator"

# create_native_proof.rtm
$command_output = resim run .\manifests\create_native_proof.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "create_native_proof - proof created"

# key_holder/create_native_proof.rtm
resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2`:$env:owner_badge_id2 | Out-Null
$command_output = resim run .\manifests\key_holder\create_native_proof.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "key_holder\create_native_proof - proof created"

# key_holder/validator_update_fee.rtm
resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2`:$env:owner_badge_id2 | Out-Null
$command_output = resim run .\manifests\key_holder\validator_update_fee.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "key_holder\validator_update_fee - fee updated"

# validator extension tests
$command_output = resim run .\\manifests\\validator_extension_tests.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "validator_extension_tests - ran validator flow (check output for indicators)"

# test_access_key_badge_withdraw.rtm - run as second account
resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2`:$env:owner_badge_id2 | Out-Null
$command_output = resim run .\manifests\test_access_key_badge_withdraw.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "test_access_key_badge_withdraw - success"

# holder_burn_access_key.rtm
$command_output = resim run .\manifests\holder_burn_access_key.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "holder_burn_access_key - burned"

# withdraw_auth_badge.rtm
$command_output = resim run .\manifests\owner\withdraw_auth_badge.rtm
Assert-Contains $command_output "COMMITTED SUCCESS" "withdraw_auth_badge - withdrawn badge"

# Finish: restore default account
resim set-default-account $env:account $env:private_key $env:owner_badge`:$env:owner_badge_id | Out-Null

Write-Host "=== Test run complete ==="



# # reset, publish, create test tokens
# resim reset
# $command_output=resim new-account
# $env:account=$command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $env:private_key=$command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $env:owner_badge=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $env:owner_badge_id=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }
# $command_output=resim new-account
# $env:account2=$command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $env:private_key2=$command_output | Where-Object { $_ -match "Private key:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $env:owner_badge2=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $env:owner_badge_id2=$command_output | Where-Object { $_ -match "Owner badge:" } | ForEach-Object { $_.Split(":")[2] }
# $command_output=resim new-simple-badge --name "Auth Badge"
# $env:auth_badge=$command_output | Where-Object { $_ -match "NonFungibleGlobalId" } | ForEach-Object { $_.Split(" ")[1].Split(":")[0] }
# $env:auth_badge_id=$command_output | Where-Object { $_ -match "NonFungibleGlobalId" } | ForEach-Object { $_.Split(" ")[1].Split(":")[1] }

# $command_output=resim publish .
# $env:package=$command_output | Where-Object { $_ -match "New Package:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }

# $command_output=resim run .\manifests\create_component.rtm
# $env:component=$command_output | Where-Object { $_ -match "component:" -And $_ -NotMatch "Address" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
# $resources=$command_output | Where-Object { $_ -match "resource:" -And $_ -NotMatch "Address" }
# $env:component_manager_badge=$resources.Split(":")[1].Replace(" ", "")
# $env:access_key_badge=$resources.Split(":")[3].Replace(" ", "")
# $non_fungible_ids=$command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
# $env:component_manager_badge_id=$non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")

# $command_output=resim run .\manifests\deposit_auth_badge.rtm

# # Create access key badge (basic)
# $command_output=resim run .\manifests\create_basic_key_badge.rtm
# $non_fungible_ids=$command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
# $env:access_key_badge_id=$non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")
# $non_fungible_ids_index=$command_output.IndexOf($non_fungible_ids)
# $env:access_key_badge_vault_address=$command_output[$non_fungible_ids_index-2].Split(":")[1].Replace(" ", "")

# # Recall and burn access key badge
# $command_output=resim run .\manifests\recall_and_burn_access_key_badge.rtm

# # Create auth badge proof
# $command_output=resim run .\manifests\create_auth_badge_proof.rtm

# # Withdraw auth badge
# $command_output=resim run .\manifests\withdraw_auth_badge.rtm








# $command_output=resim set-default-account $env:account2 $env:private_key2 $env:owner_badge2":"$env:owner_badge_id2
# $command_output=resim run .\manifests\test_access_key_badge_withdraw.rtm

# # Burn the access key badge
# $command_output=resim run .\manifests\holder_burn_access_key.rtm

# $command_output=resim set-default-account $env:account $env:private_key $env:owner_badge":"$env:owner_badge_id
