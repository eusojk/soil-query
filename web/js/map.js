// Map initialization and interaction

let map;
let currentMarker = null;

/**
 * Initialize the map
 */
function initMap() {
    map = new maplibregl.Map({
        container: 'map',
        style: 'https://demotiles.maplibre.org/style.json', // Free base map
        center: [0, 20], // World view
        zoom: 2,
        attributionControl: true
    });

    // Add navigation controls
    map.addControl(new maplibregl.NavigationControl(), 'top-right');

    // Add click handler
    map.on('click', handleMapClick);

    // Add load handler
    map.on('load', () => {
        console.log('Map loaded successfully');
    });
}

/**
 * Handle map click events
 * @param {Object} e - Click event
 */
function handleMapClick(e) {
    const { lng, lat } = e.lngLat;
    console.log(`Map clicked at: ${lat.toFixed(3)}, ${lng.toFixed(3)}`);
    
    // Query soil data
    searchSoilData(lat, lng);
    
    // Add marker
    addMarker(lat, lng);
}

/**
 * Add marker to map
 * @param {number} lat - Latitude
 * @param {number} lon - Longitude
 */
function addMarker(lat, lon) {
    // Remove existing marker
    if (currentMarker) {
        currentMarker.remove();
    }

    // Create new marker
    currentMarker = new maplibregl.Marker({
        color: '#3B82F6'
    })
        .setLngLat([lon, lat])
        .addTo(map);
}

/**
 * Fly to location on map
 * @param {number} lat - Latitude
 * @param {number} lon - Longitude
 */
function flyToLocation(lat, lon) {
    map.flyTo({
        center: [lon, lat],
        zoom: 10,
        essential: true
    });
}

/**
 * Add result marker at profile location
 * @param {number} lat - Latitude
 * @param {number} lon - Longitude
 * @param {string} profileId - Profile ID
 */
function addResultMarker(lat, lon, profileId) {
    // Remove existing marker
    if (currentMarker) {
        currentMarker.remove();
    }

    // Create popup
    const popup = new maplibregl.Popup({ offset: 25 }).setHTML(
        `<div class="text-sm">
            <p class="font-semibold">${profileId}</p>
            <p class="text-gray-600">${lat.toFixed(3)}°, ${lon.toFixed(3)}°</p>
        </div>`
    );

    // Create marker with popup
    currentMarker = new maplibregl.Marker({
        color: '#10B981'
    })
        .setLngLat([lon, lat])
        .setPopup(popup)
        .addTo(map);

    // Show popup
    currentMarker.togglePopup();
}
