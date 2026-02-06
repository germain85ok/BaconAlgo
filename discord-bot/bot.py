import discord
from discord.ext import commands, tasks
import aiohttp
import os
from datetime import datetime, time
import asyncio
from dotenv import load_dotenv

load_dotenv()

# Bot setup
intents = discord.Intents.default()
intents.message_content = True
bot = commands.Bot(command_prefix='!bacon ', intents=intents)

# Configuration
DISCORD_TOKEN = os.getenv('DISCORD_TOKEN')
SIGNALS_CHANNEL_ID = int(os.getenv('SIGNALS_CHANNEL_ID', '0'))
API_URL = os.getenv('API_URL', 'http://localhost:8000')

@bot.event
async def on_ready():
    print(f'{bot.user} is now running!')
    print(f'Bot is in {len(bot.guilds)} guilds')
    
    # Start scheduled tasks
    market_open_alert.start()
    market_close_summary.start()
    auto_post_signals.start()

@bot.command(name='signal', help='Get latest trading signals')
async def get_signal(ctx, symbol: str = None):
    """Get latest trading signal for a symbol"""
    try:
        async with aiohttp.ClientSession() as session:
            url = f'{API_URL}/api/signals'
            if symbol:
                url += f'?symbol={symbol}'
            
            async with session.get(url) as response:
                data = await response.json()
                signals = data.get('signals', [])
                
                if not signals:
                    await ctx.send(f'❌ No signals found{" for " + symbol if symbol else ""}')
                    return
                
                # Display first signal
                signal = signals[0]
                embed = create_signal_embed(signal)
                await ctx.send(embed=embed)
    
    except Exception as e:
        await ctx.send(f'❌ Error fetching signals: {str(e)}')

@bot.command(name='market', help='Get market summary')
async def market_summary(ctx):
    """Get current market summary"""
    try:
        async with aiohttp.ClientSession() as session:
            async with session.get(f'{API_URL}/api/market/summary') as response:
                data = await response.json()
                
                embed = discord.Embed(
                    title='📊 Market Summary',
                    color=discord.Color.gold(),
                    timestamp=datetime.now()
                )
                
                # Indices
                indices_text = '\n'.join([
                    f"{idx['symbol']}: ${idx['price']:.2f} ({idx['changePercent']:+.2f}%)"
                    for idx in data.get('indices', [])[:5]
                ])
                embed.add_field(name='📈 Major Indices', value=indices_text or 'N/A', inline=False)
                
                # Crypto
                crypto_text = '\n'.join([
                    f"{c['symbol']}: ${c['price']:,.2f} ({c.get('changePercent24h', 0):+.2f}%)"
                    for c in data.get('crypto', [])[:5]
                ])
                embed.add_field(name='₿ Top Crypto', value=crypto_text or 'N/A', inline=False)
                
                await ctx.send(embed=embed)
    
    except Exception as e:
        await ctx.send(f'❌ Error fetching market data: {str(e)}')

@bot.command(name='stats', help='Get trading statistics')
async def get_stats(ctx):
    """Get trading statistics"""
    embed = discord.Embed(
        title='📊 BaconAlgo Statistics',
        color=discord.Color.green(),
        timestamp=datetime.now()
    )
    
    embed.add_field(name='Win Rate', value='68.5%', inline=True)
    embed.add_field(name='Total Signals', value='247', inline=True)
    embed.add_field(name='Active Users', value='1,234', inline=True)
    embed.add_field(name='Avg R:R', value='2.8:1', inline=True)
    embed.add_field(name='Today\'s Signals', value='12', inline=True)
    embed.add_field(name='Status', value='🟢 Online', inline=True)
    
    await ctx.send(embed=embed)

# Scheduled Tasks
@tasks.loop(time=time(hour=9, minute=25))  # 9:25 AM EST
async def market_open_alert():
    """Alert before market open"""
    if SIGNALS_CHANNEL_ID == 0:
        return
    
    channel = bot.get_channel(SIGNALS_CHANNEL_ID)
    if channel:
        embed = discord.Embed(
            title='🔔 Market Opening Soon!',
            description='Markets open in 5 minutes (9:30 AM EST)\nGood luck trading! 🥓',
            color=discord.Color.orange(),
            timestamp=datetime.now()
        )
        await channel.send(embed=embed)

@tasks.loop(time=time(hour=16, minute=30))  # 4:30 PM EST
async def market_close_summary():
    """Post market close summary"""
    if SIGNALS_CHANNEL_ID == 0:
        return
    
    channel = bot.get_channel(SIGNALS_CHANNEL_ID)
    if channel:
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(f'{API_URL}/api/market/summary') as response:
                    data = await response.json()
                    
                    embed = discord.Embed(
                        title='📊 Market Close Summary',
                        description='Markets are now closed. Here\'s today\'s summary:',
                        color=discord.Color.blue(),
                        timestamp=datetime.now()
                    )
                    
                    # Top movers
                    movers_response = await session.get(f'{API_URL}/api/market/movers')
                    movers_data = await movers_response.json()
                    
                    gainers_text = '\n'.join([
                        f"🚀 {g['symbol']}: {g['changePercent']:+.2f}%"
                        for g in movers_data.get('gainers', [])[:3]
                    ])
                    
                    losers_text = '\n'.join([
                        f"📉 {l['symbol']}: {l['changePercent']:+.2f}%"
                        for l in movers_data.get('losers', [])[:3]
                    ])
                    
                    embed.add_field(name='Top Gainers', value=gainers_text or 'N/A', inline=True)
                    embed.add_field(name='Top Losers', value=losers_text or 'N/A', inline=True)
                    
                    await channel.send(embed=embed)
        
        except Exception as e:
            print(f'Error posting market summary: {e}')

@tasks.loop(minutes=30)
async def auto_post_signals():
    """Auto-post high-quality signals"""
    if SIGNALS_CHANNEL_ID == 0:
        return
    
    channel = bot.get_channel(SIGNALS_CHANNEL_ID)
    if channel:
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(f'{API_URL}/api/signals?minScore=80') as response:
                    data = await response.json()
                    signals = data.get('signals', [])
                    
                    # Post only new signals (in production, track posted signals)
                    for signal in signals[:1]:  # Post one at a time
                        embed = create_signal_embed(signal)
                        await channel.send(embed=embed)
        
        except Exception as e:
            print(f'Error auto-posting signals: {e}')

def create_signal_embed(signal):
    """Create Discord embed for a trading signal"""
    color = discord.Color.green() if signal['direction'] == 'LONG' else discord.Color.red()
    
    embed = discord.Embed(
        title=f"🥓 {signal['symbol']} - {signal['direction']}",
        description=f"Score: **{signal['score']}/100** | {signal['timeframe']}",
        color=color,
        timestamp=datetime.now()
    )
    
    embed.add_field(name='Entry', value=f"${signal['entry']:.2f}", inline=True)
    embed.add_field(name='Stop Loss', value=f"${signal['stopLoss']:.2f}", inline=True)
    embed.add_field(name='Take Profit', value=f"${signal['takeProfit']:.2f}", inline=True)
    
    targets_text = ' → '.join([f"${t:.2f}" for t in signal['targets']])
    embed.add_field(name='Targets', value=targets_text, inline=False)
    
    embed.add_field(name='R:R Ratio', value=f"{signal['riskRewardRatio']:.1f}:1", inline=True)
    embed.add_field(name='Confidence', value=f"{signal['confidence']*100:.0f}%", inline=True)
    
    reasons_text = '\n'.join([f"• {r}" for r in signal['reasons']])
    embed.add_field(name='Reasons', value=reasons_text, inline=False)
    
    embed.set_footer(text='BaconAlgo 2030')
    
    return embed

# Run bot
if __name__ == '__main__':
    if not DISCORD_TOKEN:
        print('❌ DISCORD_TOKEN not found in environment variables')
    else:
        bot.run(DISCORD_TOKEN)
