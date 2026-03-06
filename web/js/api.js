// API communication module

const API_BASE_URL = 'https://soil-query-production.up.railway.app';

/**
 * Query soil data by coordinates
 * @param {number} lat - Latitude
 * @param {number} lon - Longitude
 * @returns {Promise<Object>} Soil profile data
 */
async function querySoilData(lat, lon) {
    try {
        const response = await fetch(
            `${API_BASE_URL}/soil?lat=${lat}&lon=${lon}&format=json`
        );

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.error || 'Failed to fetch soil data');
        }

        return await response.json();
    } catch (error) {
        console.error('API Error:', error);
        throw error;
    }
}

/**
 * Download soil data as .SOL file
 * @param {number} lat - Latitude
 * @param {number} lon - Longitude
 * @param {string} profileId - Profile ID for filename
 */
async function downloadSOL(lat, lon, profileId) {
    try {
        const response = await fetch(
            `${API_BASE_URL}/soil?lat=${lat}&lon=${lon}&format=sol`
        );

        if (!response.ok) {
            throw new Error('Failed to fetch .SOL file');
        }

        const solContent = await response.text();
        
        // Create blob and download
        const blob = new Blob([solContent], { type: 'text/plain' });
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${profileId}.SOL`;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        document.body.removeChild(a);
    } catch (error) {
        console.error('Download Error:', error);
        throw error;
    }
}

/**
 * Download soil data as JSON file
 * @param {Object} data - Soil profile data
 * @param {string} profileId - Profile ID for filename
 */
function downloadJSON(data, profileId) {
    const jsonContent = JSON.stringify(data, null, 2);
    const blob = new Blob([jsonContent], { type: 'application/json' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${profileId}.json`;
    document.body.appendChild(a);
    a.click();
    window.URL.revokeObjectURL(url);
    document.body.removeChild(a);
}

/**
 * Check if API is available
 * @returns {Promise<boolean>}
 */
async function checkAPIHealth() {
    try {
        const response = await fetch(`${API_BASE_URL}/health`);
        return response.ok;
    } catch (error) {
        return false;
    }
}