# soil-query web

[Interactive map interface](https://soilmap.josuekpodo.com) for the Global Soil Profile Explorer.


---

## What it does

- Click anywhere on the map to query soil data for that location
- Search by address or place name
- View soil profile data in a side panel (layers, properties, depth)
- Download results as a DSSAT-compatible `.SOL` file
- Toggle between globe and flat map projections
- Switch between Standard, Satellite, and OSM base maps

---

## Local Development

No build step required. The frontend is plain HTML + JavaScript.

```bash
# Option 1: open directly in browser
open web/index.html

# Option 2: serve with any static server (avoids some CORS quirks)
npx serve web/
# or
python3 -m http.server 8080 --directory web/
```

By default the frontend points to the production API. To use a local API instead, edit `web/js/api.js`:

```js
// Change this:
const API_BASE_URL = "https://dsiweb.cse.msu.edu/soil-query-api";

// To this:
const API_BASE_URL = "http://127.0.0.1:3000";
```

---

## File Structure

```
web/
├── index.html          # Main page: layout, modals, header
└── js/
    ├── api.js          # API_BASE_URL and all fetch calls to soil-query-api
    ├── app.js          # Core app logic: click handler, result display, .SOL download
    ├── map.js          # MapLibre GL JS setup: globe/flat toggle, base map switching
    └── geocoding.js    # Address search via Nominatim (OpenStreetMap)
```

---

## Tech Stack

| Tool | Purpose |
|------|---------|
| [MapLibre GL JS](https://maplibre.org/) | Interactive map rendering |
| [Tailwind CSS](https://tailwindcss.com/) (CDN) | Utility-first styling |
| Vanilla JavaScript | No framework |
| [Nominatim](https://nominatim.org/) | Free geocoding (from address to coordinates) |

---

## Deployment

[TODO]

---

## API Connection

All data comes from `soil-query-api`. The base URL is set in `web/js/api.js`:

```js
const API_BASE_URL = "https://dsiweb.cse.msu.edu/soil-query-api";
```

The frontend calls:
- `GET /soil?lat=...&lon=...&format=json`: on map click or address search
- `GET /soil?lat=...&lon=...&format=sol`: when user downloads `.SOL` file
- `GET /definitions`: to populate abbreviation tooltips

See [`crates/soil-query-api/README.md`](../crates/soil-query-api/README.md) for full API documentation.