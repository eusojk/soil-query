// Main application logic

let currentProfileData = null;
let currentQueryCoords = null;

/**
 * Initialize the application
 */
document.addEventListener('DOMContentLoaded', async () => {
    console.log('Soil Query Web Interface starting...');

    // Check API health
    const apiHealthy = await checkAPIHealth();
    if (!apiHealthy) {
        showError('API server is not running. Please start the API server at http://127.0.0.1:3000');
        return;
    }

    // Initialize map
    initMap();

    // Setup event listeners
    setupEventListeners();

    console.log('Application initialized');
});

/**
 * Setup all event listeners
 */
function setupEventListeners() {
    // Home button
    document.getElementById('home-btn').addEventListener('click', resetToInstructions);

    // Info modal
    document.getElementById('info-btn').addEventListener('click', () => {
        document.getElementById('info-modal').classList.remove('hidden');
    });
    document.getElementById('modal-close').addEventListener('click', () => {
        document.getElementById('info-modal').classList.add('hidden');
    });
    document.getElementById('modal-backdrop').addEventListener('click', () => {
        document.getElementById('info-modal').classList.add('hidden');
    });

    // Search button
    document.getElementById('search-btn').addEventListener('click', handleSearchButton);

    // Enter key in inputs
    document.getElementById('lat-input').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleSearchButton();
    });
    document.getElementById('lon-input').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleSearchButton();
    });

    // Close results button
    document.getElementById('close-results').addEventListener('click', resetToInstructions);

    // Download buttons
    document.getElementById('download-sol').addEventListener('click', handleDownloadSOL);
    document.getElementById('download-json').addEventListener('click', handleDownloadJSON);

    // New search button
    document.getElementById('new-search').addEventListener('click', resetToInstructions);

    // Setup address search
    setupAddressSearch();
}

/**
 * Handle search button click
 */
function handleSearchButton() {
    const lat = parseFloat(document.getElementById('lat-input').value);
    const lon = parseFloat(document.getElementById('lon-input').value);

    // Validate inputs
    if (isNaN(lat) || isNaN(lon)) {
        showError('Please enter valid coordinates');
        return;
    }

    if (lat < -90 || lat > 90) {
        showError('Latitude must be between -90 and 90');
        return;
    }

    if (lon < -180 || lon > 180) {
        showError('Longitude must be between -180 and 180');
        return;
    }

    // Search (flyToLocation is called inside searchSoilData after API responds)
    searchSoilData(lat, lon);
    addMarker(lat, lon);
}

/**
 * Search for soil data
 * @param {number} lat - Latitude
 * @param {number} lon - Longitude
 */
async function searchSoilData(lat, lon) {
    try {
        // Show loading state with coordinates
        showLoading(lat, lon);

        // Store query coordinates
        currentQueryCoords = { lat, lon };

        // Query API
        const data = await querySoilData(lat, lon);

        // Store data
        currentProfileData = data;

        // Display results in sidebar
        displayResults(data);

        // Add marker and popup (simple delay to let any map movement settle)
        const profileLat = data.profile.location.lat;
        const profileLon = data.profile.location.lon;
        addResultMarker(profileLat, profileLon, data.profile.id);

    } catch (error) {
        showError(error.message);
    }
}

/**
 * Display soil profile results
 * @param {Object} data - API response data
 */
function displayResults(data) {
    const { profile, distance_km } = data;

    // Update profile header
    document.getElementById('profile-id').textContent = profile.id;
    document.getElementById('profile-location').textContent = 
        `${profile.location.lat.toFixed(3)}°, ${profile.location.lon.toFixed(3)}°`;
    document.getElementById('profile-distance').textContent = `${distance_km.toFixed(2)} km from query point`;

    // Update profile info
    document.getElementById('profile-country').textContent = profile.location.country_code;
    document.getElementById('profile-texture').textContent = profile.site.texture;
    document.getElementById('profile-depth').textContent = `${profile.site.max_depth_cm} cm`;
    document.getElementById('profile-layers').textContent = profile.layers.length;

    // Update layers table
    const tbody = document.getElementById('layers-table');
    tbody.innerHTML = '';

    profile.layers.forEach(layer => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td class="px-2 py-2">${layer.slb}</td>
            <td class="px-2 py-2">${layer.slmh}</td>
            <td class="px-2 py-2 text-right">${formatValue(layer.slll)}</td>
            <td class="px-2 py-2 text-right">${formatValue(layer.sdul)}</td>
            <td class="px-2 py-2 text-right">${formatValue(layer.slhw)}</td>
        `;
        tbody.appendChild(row);
    });

    // Show results, hide other states
    showSection('results');
}

/**
 * Format optional value for display
 * @param {number|null} value
 * @returns {string}
 */
function formatValue(value) {
    return value !== null ? value.toFixed(3) : '-';
}

/**
 * Handle download .SOL button
 */
async function handleDownloadSOL() {
    if (!currentProfileData || !currentQueryCoords) return;

    try {
        await downloadSOL(
            currentQueryCoords.lat,
            currentQueryCoords.lon,
            currentProfileData.profile.id
        );
    } catch (error) {
        showError('Failed to download .SOL file: ' + error.message);
    }
}

/**
 * Handle download JSON button
 */
function handleDownloadJSON() {
    if (!currentProfileData) return;

    try {
        downloadJSON(currentProfileData, currentProfileData.profile.id);
    } catch (error) {
        showError('Failed to download JSON: ' + error.message);
    }
}

/**
 * Show loading state with coordinates
 */
function showLoading(lat, lon) {
    const coordEl = document.getElementById('loading-coords');
    if (lat !== undefined && lon !== undefined) {
        coordEl.textContent = `near ${lat.toFixed(3)}°, ${lon.toFixed(3)}°`;
    } else {
        coordEl.textContent = '';
    }
    showSection('loading');
}

/**
 * Show error message
 * @param {string} message - Error message
 */
function showError(message) {
    document.getElementById('error-message').textContent = message;
    showSection('error');

    // Auto-hide after 5 seconds
    setTimeout(() => {
        if (!currentProfileData) {
            showSection('instructions');
        }
    }, 5000);
}

/**
 * Show a specific section
 * @param {string} section - Section ID to show
 */
function showSection(section) {
    // Hide all sections
    document.getElementById('instructions').classList.add('hidden');
    document.getElementById('loading').classList.add('hidden');
    document.getElementById('error').classList.add('hidden');
    document.getElementById('results').classList.add('hidden');

    // Show requested section
    document.getElementById(section).classList.remove('hidden');
}

/**
 * Reset to instructions view
 */
function resetToInstructions() {
    currentProfileData = null;
    currentQueryCoords = null;
    
    // Clear inputs
    document.getElementById('lat-input').value = '';
    document.getElementById('lon-input').value = '';
    
    // Remove marker and popup
    if (currentMarker) {
        currentMarker.remove();
        currentMarker = null;
    }
    if (typeof currentPopup !== 'undefined' && currentPopup) {
        currentPopup.remove();
        currentPopup = null;
    }
    
    // Reset map view
    map.flyTo({
        center: [0, 20],
        zoom: 2
    });
    
    showSection('instructions');
}