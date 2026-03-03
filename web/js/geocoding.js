// Geocoding functionality using Nominatim (OpenStreetMap)

const NOMINATIM_URL = 'https://nominatim.openstreetmap.org/search';

/**
 * Search for addresses using Nominatim
 * @param {string} query - Address search query
 * @returns {Promise<Array>} Array of results
 */
async function searchAddress(query) {
    if (!query || query.trim().length < 3) {
        return [];
    }

    try {
        const response = await fetch(
            `${NOMINATIM_URL}?format=json&q=${encodeURIComponent(query)}&limit=5`,
            {
                headers: {
                    'Accept': 'application/json'
                }
            }
        );

        if (!response.ok) {
            throw new Error('Geocoding request failed');
        }

        return await response.json();
    } catch (error) {
        console.error('Geocoding error:', error);
        return [];
    }
}

/**
 * Setup address search functionality
 */
function setupAddressSearch() {
    const addressInput = document.getElementById('address-input');
    const addressSearchBtn = document.getElementById('address-search-btn');
    const addressResults = document.getElementById('address-results');

    // Search on button click
    addressSearchBtn.addEventListener('click', async () => {
        const query = addressInput.value.trim();
        if (query) {
            await performAddressSearch(query);
        }
    });

    // Search on Enter key
    addressInput.addEventListener('keypress', async (e) => {
        if (e.key === 'Enter') {
            const query = addressInput.value.trim();
            if (query) {
                await performAddressSearch(query);
            }
        }
    });

    // Hide results when clicking outside
    document.addEventListener('click', (e) => {
        if (!addressInput.contains(e.target) && 
            !addressResults.contains(e.target) &&
            !addressSearchBtn.contains(e.target)) {
            addressResults.classList.add('hidden');
        }
    });
}

/**
 * Perform address search and show results
 * @param {string} query - Search query
 */
async function performAddressSearch(query) {
    const addressResults = document.getElementById('address-results');
    
    // Show loading
    addressResults.innerHTML = '<div class="p-3 text-sm text-gray-500">Searching...</div>';
    addressResults.classList.remove('hidden');

    try {
        const results = await searchAddress(query);

        if (results.length === 0) {
            addressResults.innerHTML = '<div class="p-3 text-sm text-gray-500">No results found</div>';
            return;
        }

        // Display results
        addressResults.innerHTML = results.map(result => `
            <button class="address-result w-full text-left px-3 py-2 hover:bg-gray-100 border-b border-gray-100 last:border-b-0"
                    data-lat="${result.lat}"
                    data-lon="${result.lon}">
                <div class="text-sm font-medium text-gray-900">${result.display_name.split(',')[0]}</div>
                <div class="text-xs text-gray-500">${result.display_name}</div>
            </button>
        `).join('');

        // Add click handlers to results
        addressResults.querySelectorAll('.address-result').forEach(btn => {
            btn.addEventListener('click', () => {
                const lat = parseFloat(btn.getAttribute('data-lat'));
                const lon = parseFloat(btn.getAttribute('data-lon'));
                
                // Fill in coordinate inputs
                document.getElementById('lat-input').value = lat.toFixed(3);
                document.getElementById('lon-input').value = lon.toFixed(3);
                
                // Hide results
                addressResults.classList.add('hidden');
                
                // Clear address input
                document.getElementById('address-input').value = btn.querySelector('.text-sm').textContent;
                
                // Search soil data (flyToLocation is handled inside searchSoilData)
                searchSoilData(lat, lon);
            });
        });

    } catch (error) {
        addressResults.innerHTML = '<div class="p-3 text-sm text-red-500">Search failed. Please try again.</div>';
    }
}
