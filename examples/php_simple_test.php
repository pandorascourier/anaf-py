<?php
/**
 * ANAF API PHP Extension - Simple Test
 */

echo "=== ANAF API PHP Extension Test ===\n\n";

// Check if extension is loaded
if (!extension_loaded('anaf-api')) {
    echo "Error: anaf-api extension is not loaded!\n";
    exit(1);
}

echo "Extension loaded successfully!\n\n";

// List available classes
echo "Available ANAF classes:\n";
$classes = get_declared_classes();
$anafClasses = array_filter($classes, fn($c) => str_starts_with($c, 'Anaf\\'));
foreach ($anafClasses as $class) {
    echo "  - $class\n";
}
echo "\nTotal: " . count($anafClasses) . " classes\n\n";

// Create a client  
echo "Creating AnafClient...\n";
$client = new Anaf\AnafClient();
echo "Client created successfully!\n\n";

// Get today's date
$today = Anaf\AnafClient::today();
echo "Today's date helper: $today\n\n";

// List available methods on AnafClient
echo "AnafClient methods:\n";
$reflection = new ReflectionClass('Anaf\AnafClient');
foreach ($reflection->getMethods() as $method) {
    echo "  - {$method->getName()}()\n";
}

echo "\n=== Test Complete ===\n";
