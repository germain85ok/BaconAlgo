# BaconAlgo Stream Overlays

OBS-ready stream overlays for BaconAlgo trading signals and market data.

## Quick Start

1. Start the station server:
   ```bash
   cd station
   npm run dev
   ```

2. Open OBS Studio

3. Add a Browser Source with one of the widget URLs below

4. Configure the browser source:
   - Width/Height: Depends on widget (Full Dashboard: 1920x1080)
   - Enable "Shutdown source when not visible" for performance
   - Optional: Add Chroma Key filter for transparency

## Available Widgets

### Full Dashboard
- **URL**: `http://localhost:5173/stream?widget=dashboard`
- **Size**: 1920x1080
- **Description**: Complete stream overlay with all widgets

### Market Data Widgets
- **Market Countdown**: `?widget=countdown` - Time until market open/close
- **Fear & Greed Index**: `?widget=feargreed` - Market sentiment meter
- **VIX Meter**: `?widget=vix` - Volatility index display
- **Market Heatmap**: `?widget=heatmap` - Sector performance heatmap
- **Crypto Ticker**: `?widget=crypto` - Cryptocurrency prices
- **News Headlines**: `?widget=news` - Latest market news
- **Top Movers**: `?widget=movers` - Biggest gainers/losers

### Signal Widgets
- **Rocket Signals**: `?widget=signals` - Live trading signals

### Donation Widgets
- **Donation Alert**: `?widget=donations` - Donation notifications (animated)
- **Donation Feed**: `?widget=donation-feed` - Recent donations list
- **Donation Tiers**: `?widget=donation-tiers` - Donation tier information
- **Donation Goal**: `?widget=donation-goal` - Goal progress tracker
- **Donor Leaderboard**: `?widget=leaderboard` - Top donors ranking
- **Donation Links**: `?widget=donation-links` - Ways to donate

### Stream Widgets
- **Live Clock**: `?widget=clock` - Current time display
- **Viewer Count**: `?widget=viewers` - Live viewer counter
- **Music Visualizer**: `?widget=visualizer` - Audio visualization
- **Chat Overlay**: `?widget=chat` - Live chat messages
- **Stream Schedule**: `?widget=schedule` - Upcoming streams
- **Song Queue**: `?widget=songs` - Music requests

### Branding Widgets
- **Sponsor Banner**: `?widget=sponsor` - Sponsor advertisements
- **Social Links**: `?widget=social` - Social media links
- **Bacon Logo**: `?widget=logo` - BaconAlgo logo

## OBS Setup Guide

### Adding a Widget to OBS

1. **Add Browser Source**:
   - Right-click in Sources → Add → Browser
   - Name it (e.g., "Market Countdown")

2. **Configure URL**:
   - Copy widget URL from above
   - Paste into URL field
   - Adjust width/height as needed

3. **Performance Settings**:
   - ✅ Enable "Shutdown source when not visible"
   - ✅ Enable "Refresh browser when scene becomes active"
   - Set FPS to 30 for smooth updates

4. **Optional - Chroma Key**:
   - Right-click source → Filters → Add → Chroma Key
   - Select green/blue color if widget has colored background
   - Most widgets have transparent backgrounds already

### Layout Tips

- **Full Dashboard**: Use for "Starting Soon" or "BRB" scenes
- **Individual Widgets**: Layer them for custom layouts
- **Positioning**: Widgets have minimal padding - position freely
- **Layering**: Stack widgets with different z-indices in OBS
- **Scenes**: Create different scenes for different stream segments

### Performance Optimization

1. **Use Individual Widgets**: Only load what you need
2. **Scene-Specific Sources**: Enable/disable sources per scene
3. **FPS Limits**: Set browser source FPS to 30
4. **Auto-Shutdown**: Always enable "Shutdown when not visible"
5. **Hardware Acceleration**: Enable in OBS browser source settings

## Development

### File Structure

```
station/src/routes/stream/
├── +page.svelte    # Stream page component (widget router)
├── +page.ts        # Load function (URL params)
└── README.md       # This file
```

### Adding New Widgets

1. Create widget component in `src/lib/stream/widgets/`
2. Import in `stream/+page.svelte`
3. Add to `validWidgets` array
4. Add conditional render block
5. Update widget examples in main page

### Widget Guidelines

- **Transparent Background**: Use `background: transparent` for OBS
- **No Padding**: Individual widgets should have minimal padding
- **Real-time Updates**: Subscribe to stores for live data
- **Cleanup**: Unsubscribe in `onDestroy`
- **Error Handling**: Handle API failures gracefully
- **Performance**: Optimize re-renders and animations

## Troubleshooting

### Widget Not Loading
- Check browser console for errors
- Verify server is running (`npm run dev`)
- Check URL parameters are correct
- Try refreshing the browser source

### Performance Issues
- Enable "Shutdown when not visible"
- Reduce FPS to 30
- Use individual widgets instead of full dashboard
- Check CPU/GPU usage in OBS

### Transparency Issues
- Most widgets use transparent backgrounds
- Some may need Chroma Key filter
- Check widget CSS for background colors

### Data Not Updating
- Check Supabase connection
- Verify stores are initialized
- Check browser console for errors
- Refresh the browser source

## Production Deployment

For production streams, use the production URL:

```
https://your-domain.com/stream?widget=dashboard
```

Replace `localhost:5173` with your actual domain.

## Support

For issues or questions:
- Check the main page (`/`) for quick start guide
- Review component source code
- Check store initialization
- Verify Supabase configuration

---

**Made with 🥓 by BaconAlgo**
