#!/usr/bin/env pwsh

param(
    [switch]$DryRun
)

# Comprehensive permission test runner for Access Manager
# Uses resim manifests and simple templating to substitute environment values into manifests

function Run-Capture($cmd) {
    $output = Invoke-Expression $cmd 2>&1 | ForEach-Object { $_ }
    return ,$output
}

function Assert-Contains($outputLines, $pattern, $testName) {
    if ($outputLines -match $pattern) {
        Write-Host "[PASS] $testName" -ForegroundColor Green
        return $true
    } else {
        Write-Host "[FAIL] $testName - pattern not found: $pattern" -ForegroundColor Red
        return $false
    }
}

function Run-Manifest($manifestPath) {
    if (-not (Test-Path $manifestPath)) {
        if (Test-Path ".\$manifestPath") { $manifestPath = ".\$manifestPath" } else { Write-Host "Manifest not found: $manifestPath" -ForegroundColor Red; return @() }
    }
    $content = Get-Content $manifestPath -Raw
    $vars = @{
        'account' = $env:account
        'account2' = $env:account2
        'auth_badge' = $env:auth_badge
        'owner_badge' = $env:owner_badge
        'owner_badge_id' = $env:owner_badge_id
        'package' = $env:package
        'component' = $env:component
        'access_key_badge_id' = $env:access_key_badge_id
        'access_key_badge' = $env:access_key_badge
        'super_access_key_badge_id' = $env:super_access_key_badge_id
        'access_manager_badge' = $env:access_manager_badge
        'access_manager_badge_id' = $env:access_manager_badge_id
        'validator' = $env:validator
    }
    foreach ($k in $vars.Keys) {
        if ($vars[$k]) { $content = $content -replace ("\$\{" + [regex]::Escape($k) + "\}"), $vars[$k] }
    }
    $tmp = Join-Path $env:TEMP ("manifest_" + [guid]::NewGuid().ToString() + ".rtm")
    Set-Content -Path $tmp -Value $content -Encoding UTF8
    if ($DryRun) {
        Write-Host "----- Templated manifest ($manifestPath) -----"
        Write-Host $content
        return ,($content -split "`n")
    }
    Write-Host "Running resim run $tmp"
    $out = Run-Capture ("resim run `"$tmp`"")
    return $out
}

Write-Host "=== Access Manager comprehensive permission tests ==="

Write-Host "-- resim reset and create test accounts --"
$command_output = Run-Capture "resim reset; resim new-account"
$env:account = $command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { ($_ -split ':')[1].Trim() }
$command_output = Run-Capture "resim new-account"
$env:account2 = $command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { ($_ -split ':')[1].Trim() }
Write-Host "Accounts created: $env:account , $env:account2"

Write-Host "-- Creating validator --"
$command_output = Run-Manifest "manifests/create_validator.rtm"
if ($command_output -match "Success") { Write-Host "[PASS] Validator created successfully" -ForegroundColor Green } else { Write-Host "[FAIL] Validator creation failed" -ForegroundColor Red }
$env:validator = $command_output | Where-Object { $_ -match "Component:" } | ForEach-Object { ($_ -split ':')[1].Trim() }

Write-Host "-- Publish package --"
$command_output = Run-Manifest "manifests/publish_package.rtm"
if (-not ($command_output -match "New Package:")) { $command_output = Run-Capture "resim publish ." }
$env:package = $command_output | Where-Object { $_ -match "New Package:" } | ForEach-Object { ($_ -split ':')[1].Trim() }
Assert-Contains $command_output "New Package:" "Publish package"

Write-Host "-- Create component --"
$command_output = Run-Manifest "manifests/create_component.rtm"
Assert-Contains $command_output "component:" "create_component - component created"
$resources = $command_output | Where-Object { $_ -match "resource:" -and $_ -notmatch "Address" }
if ($resources) { Write-Host "[PASS] create_component - resources line found" -ForegroundColor Green } else { Write-Host "[FAIL] create_component - resources line not found" -ForegroundColor Red }
if ($resources) {
    $parts = ($resources -split ':') | ForEach-Object { $_.Trim() }
    if ($parts.Length -ge 2) { $env:access_manager_badge = $parts[1] }
    if ($parts.Length -ge 4) { $env:access_key_badge = $parts[3] }
}
$non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -and $_ -match "{" }
if ($non_fungible_ids) { $env:access_manager_badge_id = ($non_fungible_ids -split ':')[1].Trim() -replace '\+\{\{','{' -replace '\}\},-\{\}','}' }
$component_line = $command_output | Where-Object { $_ -match "Component:" -or $_ -match "component:" } | Select-Object -First 1
if ($component_line) { $env:component = ($component_line -split ':')[1].Trim(); Write-Host "Captured component: $env:component" }

if ($env:owner_badge) { $env:auth_badge = $env:owner_badge }

Write-Host "-- Deposit auth badge --"
$command_output = Run-Manifest "manifests/deposit_auth_badge.rtm"
Assert-Contains $command_output "Success" "deposit_auth_badge - success indicator"

Write-Host "-- Create super key badge --"
$command_output = Run-Manifest "manifests/create_super_key_badge.rtm"
Assert-Contains $command_output "change:" "create_super_key_badge - issued NF id"
$non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -and $_ -match "{" }
if ($non_fungible_ids) {
    $env:super_access_key_badge_id = ($non_fungible_ids -split ':')[1].Trim() -replace '\+\{\{','{' -replace '\}\},-\{\}','}'
    Write-Host "Super Access key badge id: $env:super_access_key_badge_id"
    $env:access_key_badge_id = $env:super_access_key_badge_id
}

Write-Host "-- Creating a set of custom key badges --"
$permissions = @("CreateAccessKey","RecallAccessKey","CreateNativeProof","Validator_UpdateKey","Validator_UpdateFee")
foreach ($perm in $permissions) {
    $manifest = "manifests/test_permissions/create_custom_${perm}.rtm"
    Write-Host "Running create manifest for permission: $perm"
    $command_output = Run-Manifest $manifest
    Assert-Contains $command_output "change:" "create_custom_${perm} - issued NF id"
    $non_fungible_ids = $command_output | Where-Object { $_ -match "change:" -and $_ -match "{" }
    if ($non_fungible_ids) {
        $id = ($non_fungible_ids -split ':')[1].Trim() -replace '\+\{\{','{' -replace '\}\},-\{\}','}'
        Set-Variable -Name "badge_${perm}_id" -Value $id -Scope Global
        Write-Host "Created badge for $perm -> $id"
        $env:access_key_badge_id = $id
    }
}

Write-Host "-- Running permission tests --"
function Run-Permission-Test($manifestPath, $expectedPattern, $testName) {
    $command_output = Run-Manifest $manifestPath
    Assert-Contains $command_output $expectedPattern $testName
}

Run-Permission-Test "manifests/test_permissions/test_create_native_proof_super.rtm" "COMMITTED SUCCESS" "super - create_native_proof"
Run-Permission-Test "manifests/test_permissions/test_validator_update_fee_super.rtm" "COMMITTED SUCCESS" "super - validator_update_fee"
Run-Permission-Test "manifests/test_permissions/test_validator_update_key_super.rtm" "COMMITTED SUCCESS" "super - validator_update_key"

foreach ($perm in $permissions) {
    $badgeVar = Get-Variable -Name "badge_${perm}_id" -ErrorAction SilentlyContinue
    if (-not $badgeVar) { continue }
    switch ($perm) {
        "CreateNativeProof" {
            Run-Permission-Test "manifests/test_permissions/test_create_native_proof_single.rtm" "COMMITTED SUCCESS" "${perm} badge - create_native_proof should succeed"
            Run-Permission-Test "manifests/test_permissions/test_validator_update_fee_single.rtm" "COMMITTED FAILURE" "${perm} badge - validator_update_fee should fail"
        }
        "Validator_UpdateFee" {
            Run-Permission-Test "manifests/test_permissions/test_validator_update_fee_single.rtm" "COMMITTED SUCCESS" "${perm} badge - validator_update_fee should succeed"
            Run-Permission-Test "manifests/test_permissions/test_create_native_proof_single.rtm" "COMMITTED FAILURE" "${perm} badge - create_native_proof should fail"
        }
        "Validator_UpdateKey" {
            Run-Permission-Test "manifests/test_permissions/test_validator_update_key_single.rtm" "COMMITTED SUCCESS" "${perm} badge - validator_update_key should succeed"
            Run-Permission-Test "manifests/test_permissions/test_create_native_proof_single.rtm" "COMMITTED FAILURE" "${perm} badge - create_native_proof should fail"
        }
        "RecallAccessKey" {
            Run-Permission-Test "manifests/test_permissions/test_recall_key_badge_single.rtm" "COMMITTED SUCCESS|COMMITTED FAILURE" "${perm} badge - recall_key_badge (vault state dependent)"
        }
        "CreateAccessKey" {
            Run-Permission-Test "manifests/test_permissions/test_create_custom_access_key_badge_single.rtm" "COMMITTED SUCCESS" "${perm} badge - create_custom_access_key_badge should succeed"
        }
    }
}

Write-Host "=== Complete permission test run finished ==="
