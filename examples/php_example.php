<?php
/**
 * ANAF API PHP Extension Example
 * 
 * This example demonstrates how to use the ANAF API PHP extension
 * to query company information from the Romanian tax authority.
 */

echo "=== ANAF API PHP Extension Test ===\n\n";

// Check if extension is loaded
if (!extension_loaded('anaf-api')) {
    echo "Error: anaf-api extension is not loaded!\n";
    echo "Run with: php -d extension=/path/to/libanaf_api.so test.php\n";
    exit(1);
}

// List available classes
echo "Available classes:\n";
$classes = get_declared_classes();
foreach ($classes as $class) {
    if (strpos($class, 'Anaf\\') === 0) {
        echo "  - $class\n";
    }
}
echo "\n";

// Create a client
echo "Creating AnafClient...\n";
$client = new Anaf\AnafClient();
echo "Client created successfully!\n\n";

// Get today's date
$today = Anaf\AnafClient::today();
echo "Today's date: $today\n\n";

// Query VAT payer information
echo "Querying VAT payer info for CUI 18158683 (Dedeman)...\n";
try {
    $response = $client->getVatPayer([
        [18158683, $today]
    ]);
    
    echo "Response status: {$response->status}\n";
    echo "Response message: {$response->message}\n";
    echo "Found {$response->count()} companies\n\n";
    
    if ($response->count() > 0) {
        $company = $response->first();
        $data = $company->generalData;
        
        echo "Company Information:\n";
        echo "  Name: {$data->name}\n";
        echo "  CUI: {$data->cui}\n";
        echo "  Address: {$data->address}\n";
        echo "  CAEN Code: {$data->caenCode}\n";
        echo "  Trade Register: {$data->tradeRegisterNumber}\n";
        echo "  RO e-Factura: " . ($data->roEfacturaStatus ? 'Yes' : 'No') . "\n";
        
        echo "\nVAT Registration:\n";
        $vat = $company->vatRegistration;
        echo "  Is VAT Payer: " . ($vat->isRegistered ? 'Yes' : 'No') . "\n";
        
        echo "\nInactivity Status:\n";
        $inactive = $company->inactivityStatus;
        echo "  Is Inactive: " . ($inactive->isInactive ? 'Yes' : 'No') . "\n";
        
        echo "\nSplit VAT:\n";
        $split = $company->splitVat;
        echo "  Uses Split VAT: " . ($split->isApplied ? 'Yes' : 'No') . "\n";
    }
    
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}

echo "\n=== Test Complete ===\n";
