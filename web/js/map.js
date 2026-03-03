// Map initialization and interaction

let map;
let currentMarker = null;
let currentPopup = null;

// Available base map styles
const BASE_MAPS = {
    default: {
        name: 'Standard',
        style: 'https://demotiles.maplibre.org/style.json'
    },
    satellite: {
        name: 'Standard Satellite',
        style: {
            version: 8,
            sources: {
                'satellite': {
                    type: 'raster',
                    tiles: [
                        'https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}'
                    ],
                    tileSize: 256,
                    attribution: '© Esri'
                },
                'labels': {
                    type: 'raster',
                    tiles: [
                        'https://server.arcgisonline.com/ArcGIS/rest/services/Reference/World_Boundaries_and_Places/MapServer/tile/{z}/{y}/{x}'
                    ],
                    tileSize: 256
                }
            },
            layers: [
                {
                    id: 'satellite',
                    type: 'raster',
                    source: 'satellite'
                },
                {
                    id: 'labels',
                    type: 'raster',
                    source: 'labels'
                }
            ]
        }
    },
    osm: {
        name: 'OSM',
        style: {
            version: 8,
            sources: {
                'osm': {
                    type: 'raster',
                    tiles: ['https://tile.openstreetmap.org/{z}/{x}/{y}.png'],
                    tileSize: 256,
                    attribution: '© OpenStreetMap contributors'
                }
            },
            layers: [{
                id: 'osm',
                type: 'raster',
                source: 'osm'
            }]
        }
    }
};

let currentBaseMap = 'default';
let isGlobeMode = true;

/**
 * Initialize the map
 */
function initMap() {
    map = new maplibregl.Map({
        container: 'map',
        style: BASE_MAPS.default.style,
        center: [0, 20],
        zoom: 2.5, // Better zoom for globe view
        attributionControl: true
    });

    // Set projection on style load (required for globe)
    map.on('style.load', () => {
        // Start with globe by default
        map.setProjection({
            type: 'globe' // Changed from 'mercator' to 'globe'
        });
        console.log('Map style loaded with globe projection');
    });

    // Add navigation controls
    map.addControl(new maplibregl.NavigationControl(), 'top-right');

    // Add scale control
    map.addControl(new maplibregl.ScaleControl(), 'bottom-right');

    // Add custom projection and style controls
    map.on('load', () => {
        addMapControls();
        console.log('Map loaded successfully');
    });

    // Add click handler
    map.on('click', handleMapClick);
}

/**
 * Add map controls (projection toggle and base map switcher)
 */
function addMapControls() {
    const controlContainer = document.createElement('div');
    controlContainer.style.cssText = 'position: absolute; bottom: 30px; left: 10px; z-index: 1; background: white; border-radius: 4px; padding: 8px; box-shadow: 0 0 0 2px rgba(0,0,0,.1);';
    
    // Projection toggle (Globe | Mercator) - Globe selected by default
    const projectionDiv = document.createElement('div');
    projectionDiv.className = 'flex items-center gap-2 mb-2 pb-2 border-b border-gray-200';
    projectionDiv.innerHTML = `
        <button id="globe-btn" class="px-3 py-1 text-sm font-medium bg-blue-100 rounded transition border border-blue-300">
            🌍 Globe
        </button>
        <span class="text-gray-400"></span>
        <button id="mercator-btn" class="px-3 py-1 text-sm font-medium bg-white hover:bg-gray-100 rounded transition border border-gray-300">
            🗺️ Mercator
        </button>
    `;
    
    // Base map selector
    const styleDiv = document.createElement('div');
    styleDiv.className = 'flex items-center gap-2';
    styleDiv.innerHTML = `
        <button data-style="default" class="px-3 py-1 text-sm font-medium bg-blue-100 hover:bg-gray-100 rounded transition border border-blue-300">
            Standard
        </button>
        <span class="text-gray-400"></span>
        <button data-style="satellite" class="px-3 py-1 text-sm font-medium bg-white hover:bg-gray-100 rounded transition border border-gray-300">
            Satellite
        </button>
        <span class="text-gray-400"></span>
        <button data-style="osm" class="px-3 py-1 text-sm font-medium bg-white hover:bg-gray-100 rounded transition border border-gray-300">
            OSM
        </button>
    `;
    
    controlContainer.appendChild(projectionDiv);
    controlContainer.appendChild(styleDiv);
    document.getElementById('map').appendChild(controlContainer);
    
    // Event listeners for projection toggle
    document.getElementById('globe-btn').addEventListener('click', () => {
        setProjection('globe');
        updateProjectionButtons();
    });
    
    document.getElementById('mercator-btn').addEventListener('click', () => {
        setProjection('mercator');
        updateProjectionButtons();
    });
    
    // Event listeners for style buttons
    styleDiv.querySelectorAll('button[data-style]').forEach(btn => {
        btn.addEventListener('click', (e) => {
            const style = e.target.getAttribute('data-style');
            switchBaseMap(style);
            updateStyleButtons();
        });
    });
}

/**
 * Set map projection
 * @param {string} projection - 'globe' or 'mercator'
 */
function setProjection(projection) {
    isGlobeMode = projection === 'globe';
    
    map.setProjection({
        type: projection
    });
    
    if (projection === 'globe') {
        // Zoom out for better globe view
        map.easeTo({ 
            center: [0, 20],
            zoom: 1.5,
            duration: 1000
        });
        console.log('Globe projection enabled - drag to rotate the Earth!');
    } else {
        console.log('Mercator projection enabled');
    }
}

/**
 * Update projection button styles
 */
function updateProjectionButtons() {
    const globeBtn = document.getElementById('globe-btn');
    const mercatorBtn = document.getElementById('mercator-btn');
    
    if (isGlobeMode) {
        globeBtn.className = 'px-3 py-1 text-sm font-medium bg-blue-100 rounded transition border border-blue-300';
        mercatorBtn.className = 'px-3 py-1 text-sm font-medium bg-white hover:bg-gray-100 rounded transition border border-gray-300';
    } else {
        globeBtn.className = 'px-3 py-1 text-sm font-medium bg-white hover:bg-gray-100 rounded transition border border-gray-300';
        mercatorBtn.className = 'px-3 py-1 text-sm font-medium bg-blue-100 rounded transition border border-blue-300';
    }
}

/**
 * Update style button states
 */
function updateStyleButtons() {
    document.querySelectorAll('button[data-style]').forEach(btn => {
        const style = btn.getAttribute('data-style');
        if (style === currentBaseMap) {
            btn.className = 'px-3 py-1 text-sm font-medium bg-blue-100 rounded transition border border-blue-300';
        } else {
            btn.className = 'px-3 py-1 text-sm font-medium bg-white hover:bg-gray-100 rounded transition border border-gray-300';
        }
    });
}

/**
 * Switch base map style
 * @param {string} mapType - Map type (default, satellite, osm)
 */
function switchBaseMap(mapType) {
    if (!BASE_MAPS[mapType]) return;
    
    currentBaseMap = mapType;
    const currentCenter = map.getCenter();
    const currentZoom = map.getZoom();
    const currentProjectionType = isGlobeMode ? 'globe' : 'mercator';

    // Handle both URL string and style object
    const newStyle = typeof BASE_MAPS[mapType].style === 'string' 
        ? BASE_MAPS[mapType].style 
        : BASE_MAPS[mapType].style;

    map.setStyle(newStyle);

    // Restore view after style change
    map.once('style.load', () => {
        // Re-apply projection after style change
        map.setProjection({
            type: currentProjectionType
        });
        
        map.setCenter(currentCenter);
        map.setZoom(currentZoom);
        
        // Re-add marker if exists (preserve popup if it was attached)
        if (currentMarker) {
            const lngLat = currentMarker.getLngLat();
            const hadPopup = currentMarker.getPopup();
            const popup = hadPopup ? currentMarker.getPopup() : null;
            currentMarker.remove();
            currentMarker = new maplibregl.Marker({ color: '#10B981' })
                .setLngLat(lngLat)
                .addTo(map);
            if (popup) {
                currentMarker.setPopup(popup);
            }
        }
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
    // Remove existing marker and popup
    if (currentMarker) {
        currentMarker.remove();
        currentMarker = null;
    }
    if (currentPopup) {
        currentPopup.remove();
        currentPopup = null;
    }

    // Add green marker at result location
    currentMarker = new maplibregl.Marker({ color: '#10B981' })
        .setLngLat([lon, lat])
        .addTo(map);

    // Add popup directly to map (most reliable approach)
    currentPopup = new maplibregl.Popup({
        offset: 25,
        closeButton: true,
        closeOnClick: false,
        anchor: 'bottom'
    })
        .setLngLat([lon, lat])
        .setHTML(
            `<div style="font-size:13px; padding:4px 2px; line-height:1.4;">
                <p style="font-weight:600; margin:0 0 2px 0;">${profileId}</p>
                <p style="color:#555; margin:0;">${lat.toFixed(3)}°, ${lon.toFixed(3)}°</p>
            </div>`
        )
        .addTo(map);
}
