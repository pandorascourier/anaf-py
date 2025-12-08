/**
 * ANAF API Node.js Example
 *
 * This example demonstrates how to use the ANAF API native Node.js addon
 * to query Romanian company fiscal information.
 *
 * Build the native module first:
 *   cargo build --release --no-default-features --features "nodejs,vat_payer_async_api,balance_api,cults_api,farmers_api"
 *
 * Then copy the library:
 *   cp target/release/libanaf_api.so index.node
 *
 * Run this example:
 *   node examples/nodejs_example.js
 */

// Load the native module
const anaf = require('../index.node');

async function main() {
    console.log('ANAF API Node.js Example');
    console.log('========================\n');

    // Create an ANAF API client
    const client = anaf.createClient();
    console.log('✓ Client created successfully\n');

    // Get today's date
    const today = anaf.today();
    console.log(`Today's date: ${today}\n`);

    // Create a request to query a company
    // Example: Romanian National Television (TVR) - CUI: 8468930
    const requests = [
        anaf.createRequest(8468930, today),      // TVR
        anaf.createRequest(14399840, today),     // A well-known Romanian company
    ];

    console.log('Querying VAT payer information...\n');

    try {
        // Query VAT payer information
        const result = await anaf.getVatPayer(client, requests);

        console.log(`Status: ${result.status}`);
        console.log(`Message: ${result.message}`);
        console.log(`Found: ${result.data.length} companies`);
        console.log(`Not found: ${result.notFound.length} CUIs\n`);

        // Display found companies
        for (const entity of result.data) {
            console.log('-----------------------------------');
            console.log(`Company: ${entity.generalData.name}`);
            console.log(`CUI: ${entity.generalData.cui}`);
            console.log(`Address: ${entity.generalData.address}`);
            console.log(`Registration Status: ${entity.generalData.registrationStatus}`);
            console.log(`CAEN Code: ${entity.generalData.caenCode}`);
            console.log(`VAT Registered: ${entity.vatRegistration.isRegistered}`);
            console.log(`RO e-Factura: ${entity.generalData.roEfacturaStatus}`);
            
            if (entity.vatRegistration.periods.length > 0) {
                const latestPeriod = entity.vatRegistration.periods[0];
                console.log(`VAT Start Date: ${latestPeriod.startDate}`);
            }
            
            console.log(`Headquarters: ${entity.headquartersAddress.fullAddress}`);
            console.log('');
        }

        // Display not found CUIs
        if (result.notFound.length > 0) {
            console.log('Not found CUIs:', result.notFound.join(', '));
        }

    } catch (error) {
        console.error('Error querying ANAF API:', error.message);
    }
}

// Example of using the Balance API (if enabled)
async function queryBalance() {
    console.log('\n\nBalance API Example');
    console.log('===================\n');

    const client = anaf.createClient();

    try {
        // Query balance for a company (CUI, year)
        const balance = await anaf.getBalance(client, 8468930, 2023);
        
        console.log('Balance Information:');
        console.log(`  Company: ${balance.name}`);
        console.log(`  Year: ${balance.year}`);
        console.log(`  Type: ${balance.kind}`);
        console.log(`  Activity Code: ${balance.activityCode}`);
        console.log(`  Activity Name: ${balance.activityName}`);
    } catch (error) {
        console.error('Error querying balance:', error.message);
    }
}

// Run the examples
main()
    .then(() => queryBalance())
    .then(() => console.log('\nDone!'))
    .catch(console.error);
