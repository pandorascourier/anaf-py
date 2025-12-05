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
    
    echo "Response status: " . $response->getStatus() . "\n";
    echo "Response message: " . $response->getMessage() . "\n";
    echo "Found " . $response->count() . " companies\n\n";
    
    if ($response->count() > 0) {
        $company = $response->first();
        $data = $company->getGeneralData();
        
        echo "Company Information:\n";
        echo "  Name: " . $data->getName() . "\n";
        echo "  CUI: " . $data->getCui() . "\n";
        echo "  Address: " . $data->getAddress() . "\n";
        echo "  CAEN Code: " . $data->getCaenCode() . "\n";
        echo "  Trade Register: " . $data->getTradeRegisterNumber() . "\n";
        echo "  RO e-Factura: " . ($data->getRoEfacturaStatus() ? 'Yes' : 'No') . "\n";
        
        echo "\nVAT Registration:\n";
        $vat = $company->getVatRegistration();
        echo "  Is VAT Payer: " . ($vat->getIsRegistered() ? 'Yes' : 'No') . "\n";
        
        echo "\nInactivity Status:\n";
        $inactive = $company->getInactivityStatus();
        echo "  Is Inactive: " . ($inactive->getIsInactive() ? 'Yes' : 'No') . "\n";
        
        echo "\nSplit VAT:\n";
        $split = $company->getSplitVat();
        echo "  Uses Split VAT: " . ($split->getIsApplied() ? 'Yes' : 'No') . "\n";
    }
    
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}

echo "\n=== Test Complete ===\n";
