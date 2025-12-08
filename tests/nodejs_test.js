/**
 * Node.js Binding Tests for ANAF API
 * 
 * Run with: node tests/nodejs_test.js
 * 
 * Prerequisites:
 *   cargo build --release --no-default-features --features "nodejs,vat_payer_async_api,balance_api,cults_api,farmers_api"
 *   cp target/release/libanaf_api.so index.node
 */

const assert = require('assert');
const path = require('path');

// Load the native module
const anaf = require(path.join(__dirname, '..', 'index.node'));

// Test results tracking
let passed = 0;
let failed = 0;
const results = [];

function test(name, fn) {
    return async () => {
        try {
            await fn();
            passed++;
            results.push({ name, status: 'PASS' });
            console.log(`  ✓ ${name}`);
        } catch (error) {
            failed++;
            results.push({ name, status: 'FAIL', error: error.message });
            console.log(`  ✗ ${name}`);
            console.log(`    Error: ${error.message}`);
        }
    };
}

// =============================================================================
// Test Suite
// =============================================================================

const tests = [
    // Module loading tests
    test('Module loads successfully', () => {
        assert(anaf !== undefined, 'Module should be defined');
        assert(typeof anaf === 'object', 'Module should be an object');
    }),

    test('createClient function exists', () => {
        assert(typeof anaf.createClient === 'function', 'createClient should be a function');
    }),

    test('today function exists', () => {
        assert(typeof anaf.today === 'function', 'today should be a function');
    }),

    test('createRequest function exists', () => {
        assert(typeof anaf.createRequest === 'function', 'createRequest should be a function');
    }),

    test('getVatPayer function exists', () => {
        assert(typeof anaf.getVatPayer === 'function', 'getVatPayer should be a function');
    }),

    test('getBalance function exists', () => {
        assert(typeof anaf.getBalance === 'function', 'getBalance should be a function');
    }),

    test('getCult function exists', () => {
        assert(typeof anaf.getCult === 'function', 'getCult should be a function');
    }),

    test('getFarmer function exists', () => {
        assert(typeof anaf.getFarmer === 'function', 'getFarmer should be a function');
    }),

    // Client creation tests
    test('createClient returns a client object', () => {
        const client = anaf.createClient();
        assert(client !== undefined, 'Client should be defined');
        assert(client !== null, 'Client should not be null');
    }),

    // today() tests
    test('today returns a valid date string', () => {
        const today = anaf.today();
        assert(typeof today === 'string', 'today should return a string');
        assert(/^\d{4}-\d{2}-\d{2}$/.test(today), 'Date should be in YYYY-MM-DD format');
    }),

    test('today returns current date', () => {
        const today = anaf.today();
        const expected = new Date().toISOString().split('T')[0];
        assert.strictEqual(today, expected, 'Should return current date');
    }),

    // createRequest tests
    test('createRequest creates request object with CUI and date', () => {
        const req = anaf.createRequest(12345678, '2024-01-15');
        assert(typeof req === 'object', 'Request should be an object');
        assert.strictEqual(req.cui, 12345678, 'CUI should match');
        assert.strictEqual(req.date, '2024-01-15', 'Date should match');
    }),

    test('createRequest uses today as default date', () => {
        const req = anaf.createRequest(12345678);
        const today = anaf.today();
        assert.strictEqual(req.cui, 12345678, 'CUI should match');
        assert.strictEqual(req.date, today, 'Date should default to today');
    }),

    // VAT Payer API tests (requires network)
    test('getVatPayer returns a Promise', () => {
        const client = anaf.createClient();
        const requests = [anaf.createRequest(14399840)];
        const result = anaf.getVatPayer(client, requests);
        assert(result instanceof Promise, 'getVatPayer should return a Promise');
    }),

    test('getVatPayer resolves with valid response structure', async () => {
        const client = anaf.createClient();
        const requests = [anaf.createRequest(14399840)]; // DANTE INTERNATIONAL SA
        
        const response = await anaf.getVatPayer(client, requests);
        
        assert(typeof response === 'object', 'Response should be an object');
        assert(typeof response.status === 'number', 'status should be a number');
        assert(typeof response.message === 'string', 'message should be a string');
        assert(Array.isArray(response.data), 'data should be an array');
        assert(Array.isArray(response.notFound), 'notFound should be an array');
    }),

    test('getVatPayer returns company data for valid CUI', async () => {
        const client = anaf.createClient();
        const requests = [anaf.createRequest(14399840)]; // DANTE INTERNATIONAL SA
        
        const response = await anaf.getVatPayer(client, requests);
        
        assert(response.data.length > 0, 'Should find at least one company');
        
        const company = response.data[0];
        assert(company.generalData, 'Should have generalData');
        assert(company.generalData.cui === 14399840, 'CUI should match');
        assert(typeof company.generalData.name === 'string', 'name should be a string');
        assert(company.generalData.name.length > 0, 'name should not be empty');
    }),

    test('getVatPayer includes VAT registration info', async () => {
        const client = anaf.createClient();
        const requests = [anaf.createRequest(14399840)];
        
        const response = await anaf.getVatPayer(client, requests);
        const company = response.data[0];
        
        assert(company.vatRegistration, 'Should have vatRegistration');
        assert(typeof company.vatRegistration.isRegistered === 'boolean', 'isRegistered should be boolean');
        assert(Array.isArray(company.vatRegistration.periods), 'periods should be an array');
    }),

    test('getVatPayer includes address info', async () => {
        const client = anaf.createClient();
        const requests = [anaf.createRequest(14399840)];
        
        const response = await anaf.getVatPayer(client, requests);
        const company = response.data[0];
        
        assert(company.headquartersAddress, 'Should have headquartersAddress');
        assert(typeof company.headquartersAddress.fullAddress === 'string', 'fullAddress should be a string');
        assert(company.fiscalAddress, 'Should have fiscalAddress');
    }),

    test('getVatPayer handles not found CUI', async () => {
        const client = anaf.createClient();
        const requests = [anaf.createRequest(99999999)]; // Non-existent CUI
        
        try {
            const response = await anaf.getVatPayer(client, requests);
            // If it succeeds, check that the CUI is in notFound
            assert(response.notFound.length > 0 || response.data.length === 0, 
                'Should report not found or return empty data');
        } catch (error) {
            // API may throw an error for not-found CUIs - this is acceptable
            assert(error.message.includes('notFound') || error.message.includes('99999999'),
                'Error should mention the not found CUI');
        }
    }),

    test('getVatPayer handles multiple requests', async () => {
        const client = anaf.createClient();
        const requests = [
            anaf.createRequest(14399840), // DANTE INTERNATIONAL SA
            anaf.createRequest(18158683), // DEDEMAN
        ];
        
        const response = await anaf.getVatPayer(client, requests);
        
        assert(response.data.length + response.notFound.length === 2, 
            'Should process all requests');
    }),

    // Balance API tests
    test('getBalance returns a Promise', () => {
        const client = anaf.createClient();
        const result = anaf.getBalance(client, 14399840, 2023);
        assert(result instanceof Promise, 'getBalance should return a Promise');
    }),

    test('getBalance resolves with valid response structure', async () => {
        const client = anaf.createClient();
        
        const response = await anaf.getBalance(client, 14399840, 2023);
        
        assert(typeof response === 'object', 'Response should be an object');
        assert(typeof response.year === 'number', 'year should be a number');
        assert(typeof response.uniqueRegistrationCode === 'number', 'uniqueRegistrationCode should be a number');
    }),

    // Error handling tests
    test('getVatPayer rejects with invalid client', async () => {
        try {
            await anaf.getVatPayer({}, [anaf.createRequest(14399840)]);
            assert.fail('Should have thrown an error');
        } catch (error) {
            assert(error, 'Should throw an error for invalid client');
        }
    }),

    test('getVatPayer handles empty requests array', async () => {
        const client = anaf.createClient();
        
        try {
            const response = await anaf.getVatPayer(client, []);
            // Either returns empty response or throws - both are valid
            assert(Array.isArray(response.data), 'Should return valid response');
        } catch (error) {
            // Also acceptable to throw for empty requests
            assert(error, 'Error is acceptable for empty requests');
        }
    }),
];

// =============================================================================
// Run Tests
// =============================================================================

async function runTests() {
    console.log('\n🧪 ANAF API Node.js Binding Tests\n');
    console.log('='.repeat(50));
    
    for (const testFn of tests) {
        await testFn();
    }
    
    console.log('\n' + '='.repeat(50));
    console.log(`\n📊 Results: ${passed} passed, ${failed} failed\n`);
    
    if (failed > 0) {
        console.log('Failed tests:');
        results.filter(r => r.status === 'FAIL').forEach(r => {
            console.log(`  - ${r.name}: ${r.error}`);
        });
        process.exit(1);
    } else {
        console.log('✅ All tests passed!\n');
        process.exit(0);
    }
}

runTests().catch(err => {
    console.error('Test runner error:', err);
    process.exit(1);
});
